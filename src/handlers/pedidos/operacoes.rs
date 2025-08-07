//! 🔍 Operações Avançadas de Pedidos
//!
//! Implementa operações de busca, atualização e alteração de status
//! com validações rigorosas e controle de workflow.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tiberius::{Query, QueryItem};
use futures_util::TryStreamExt;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use crate::database::DatabasePools;
use crate::errors::ApiError;
use crate::auth::Claims;
use crate::models::{CriarPedidoRequest, Pedido};
use super::helpers::{
    get_optional_string, get_string_with_default, 
    begin_transaction, commit_transaction, 
    safe_rollback, map_tiberius_error
};

#[derive(Debug, Deserialize)]
pub struct AlterarStatusRequest {
    pub novo_status: String,
    pub observacoes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PedidoCompleto {
    pub pedido: Pedido,
    pub items: Vec<ItemCompleto>,
    pub cliente: ClienteResumo,
    pub totais: TotaisPedido,
    pub regras: RegrasPedido,
}

#[derive(Debug, Serialize)]
pub struct ItemCompleto {
    pub id: i32,
    pub produto_id: i32,
    pub codigo_produto: String,
    pub descricao_produto: String,
    pub quantidade: i32,
    pub preco_unitario: Decimal,
    pub valor_total: Decimal,
    pub percentual_desconto: Decimal,
}

#[derive(Debug, Serialize)]
pub struct ClienteResumo {
    pub id: i32,
    pub nome: String,
    pub cnpj: String,
    pub grupo_venda: String,
}

#[derive(Debug, Serialize)]
pub struct TotaisPedido {
    pub subtotal: Decimal,
    pub desconto_total: Decimal,
    pub valor_final: Decimal,
    pub quantidade_total: i32,
}

#[derive(Debug, Serialize)]
pub struct RegrasPedido {
    pub condicao_pagamento: String,
    pub tipo_frete: String,
    pub natureza: String,
}

/// GET /pedidos/{id}
/// Buscar pedido completo com todos os relacionamentos
pub async fn buscar_pedido_completo(
    pedido_id: web::Path<i32>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    let pedido_id_val = pedido_id.into_inner();
    log::info!("Buscando pedido completo ID: {}", pedido_id_val);
    
    let mut conn = pools.sqlserver_portal.get().await
        .map_err(|e| ApiError::Database(format!("Erro ao conectar: {}", e)))?;
    
    // 1. Buscar dados principais do pedido
    let mut query_pedido = Query::new(r#"
        SELECT 
            p.id, p.cliente_id, p.codigo_cliente, p.numero_pedido,
            p.loja_cliente, p.emissao, p.dt_envio, p.condicao_pagamento,
            p.tipo_pedido, p.tabela_precos, p.tipo_frete, p.mensagem,
            p.natureza, p.status_pedido, p.numero_nota_fiscal,
            p.transportadora, p.rastreio_carga, p.vendedor,
            p.integrado, p.confirmado, p.status_liberacao,
            p.tentativas_integracao, p.regra_condicao_pagamento_id,
            p.regra_frete_id, p.nota, p.created_at, p.updated_at,
            c.nome as cliente_nome, c.cnpj, c.grupo_venda,
            rf.descricao as frete_descricao,
            rp.descricao as parcelamento_descricao
        FROM pedidos p
        INNER JOIN clientes c ON p.cliente_id = c.id
        LEFT JOIN regra_frete rf ON p.regra_frete_id = rf.id  
        LEFT JOIN regras_parcelamento rp ON p.regra_condicao_pagamento_id = rp.id
        WHERE p.id = @P1
    "#);
    query_pedido.bind(pedido_id_val);
    
    let pedido_row = match map_tiberius_error!(
        query_pedido.query(&mut conn).await,
        "Erro ao buscar dados do pedido"
    )?.into_row().await
        .map_err(|e| ApiError::Database(format!("Erro ao processar dados do pedido: {}", e)))? {
        Some(row) => row,
        None => return Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": "Pedido não encontrado"
        })))
    };
    
    // 2. Construir objeto Pedido
    let pedido = Pedido {
        id: pedido_row.get::<i32, _>(0).unwrap_or(0),
        cliente_id: pedido_row.get::<i32, _>(1).unwrap_or(0),
        codigo_cliente: pedido_row.get::<&str, _>(2).unwrap_or("").to_string(),
        numero_pedido: get_optional_string(&pedido_row, 3),
        loja_cliente: pedido_row.get::<&str, _>(4).unwrap_or("").to_string(),
        emissao: pedido_row.get::<&str, _>(5).unwrap_or("").to_string(),
        dt_envio: get_optional_string(&pedido_row, 6),
        condicao_pagamento: get_optional_string(&pedido_row, 7),
        tipo_pedido: get_optional_string(&pedido_row, 8),
        tabela_precos: get_optional_string(&pedido_row, 9),
        tipo_frete: get_optional_string(&pedido_row, 10),
        mensagem: get_optional_string(&pedido_row, 11),
        natureza: pedido_row.get::<&str, _>(12).unwrap_or("10212").to_string(),
        status_pedido: pedido_row.get::<&str, _>(13).unwrap_or("a confirmar").to_string(),
        numero_nota_fiscal: get_optional_string(&pedido_row, 14),
        transportadora: get_optional_string(&pedido_row, 15),
        rastreio_carga: get_optional_string(&pedido_row, 16),
        vendedor: get_optional_string(&pedido_row, 17),
        integrado: pedido_row.get::<bool, _>(18).unwrap_or(false),
        confirmado: pedido_row.get::<bool, _>(19).unwrap_or(false),
        status_liberacao: get_optional_string(&pedido_row, 20),
        tentativas_integracao: pedido_row.get::<i32, _>(21),
        regra_condicao_pagamento_id: pedido_row.get::<i32, _>(22).unwrap_or(0),
        regra_frete_id: pedido_row.get::<i32, _>(23).unwrap_or(0),
        nota: get_optional_string(&pedido_row, 24),
        created_at: get_optional_string(&pedido_row, 25),
        updated_at: get_optional_string(&pedido_row, 26),
    };
    
    // 3. Buscar itens do pedido
    let mut query_items = Query::new(r#"
        SELECT 
            i.id, i.produto_id, i.codigo_produto, i.descricao_produto,
            i.quantidade, i.preco_unitario, i.percentual_desconto,
            i.valor_com_desconto
        FROM items i
        WHERE i.pedido_id = @P1
        ORDER BY i.id
    "#);
    query_items.bind(pedido_id_val);
    
    let mut items = Vec::new();
    let mut quantidade_total = 0;
    let mut subtotal = Decimal::ZERO;
    let mut desconto_total = Decimal::ZERO;
    
    let mut item_stream = map_tiberius_error!(
        query_items.query(&mut conn).await,
        "Erro ao buscar itens do pedido"
    )?;
    
    while let Some(item) = map_tiberius_error!(
        item_stream.try_next().await,
        "Erro ao processar itens do pedido"
    )? {
        match item {
            QueryItem::Row(item_row) => {
                let quantidade: i32 = item_row.get::<i32, _>(4).unwrap_or(0);
                let preco_unitario: Decimal = item_row.get::<Decimal, _>(5).unwrap_or(Decimal::ZERO);
                let percentual_desconto: Decimal = item_row.get::<Decimal, _>(6).unwrap_or(Decimal::ZERO);
                let valor_com_desconto: Decimal = item_row.get::<Decimal, _>(7).unwrap_or(Decimal::ZERO);
                
                let valor_total = if valor_com_desconto > Decimal::ZERO {
                    valor_com_desconto
                } else {
                    Decimal::from(quantidade) * preco_unitario
                };
                
                quantidade_total += quantidade;
                subtotal += Decimal::from(quantidade) * preco_unitario;
                
                if percentual_desconto > Decimal::ZERO {
                    let desconto_item = (Decimal::from(quantidade) * preco_unitario) * 
                                       (percentual_desconto / Decimal::from(100));
                    desconto_total += desconto_item;
                }
                
                items.push(ItemCompleto {
                    id: item_row.get::<i32, _>(0).unwrap_or(0),
                    produto_id: item_row.get::<i32, _>(1).unwrap_or(0),
                    codigo_produto: item_row.get::<&str, _>(2).unwrap_or("").to_string(),
                    descricao_produto: item_row.get::<&str, _>(3).unwrap_or("").to_string(),
                    quantidade,
                    preco_unitario,
                    valor_total,
                    percentual_desconto,
                });
            }
            _ => {} // Ignorar outros tipos de QueryItem
        }
    }
    
    // 4. Construir objetos auxiliares
    let cliente = ClienteResumo {
        id: pedido.cliente_id,
        nome: pedido_row.get::<&str, _>(27).unwrap_or("").to_string(),
        cnpj: pedido_row.get::<&str, _>(28).unwrap_or("").to_string(),
        grupo_venda: pedido_row.get::<&str, _>(29).unwrap_or("").to_string(),
    };
    
    let totais = TotaisPedido {
        subtotal,
        desconto_total,
        valor_final: subtotal - desconto_total,
        quantidade_total,
    };
    
    let regras = RegrasPedido {
        condicao_pagamento: get_string_with_default(&pedido_row, 31, "Não definida"),
        tipo_frete: get_string_with_default(&pedido_row, 30, "Não definida"),
        natureza: pedido.natureza.clone(),
    };
    
    let pedido_completo = PedidoCompleto {
        pedido,
        items,
        cliente,
        totais,
        regras,
    };
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": pedido_completo
    })))
}

/// PUT /pedidos/{id}
/// Atualizar pedido (apenas se status permitir)
pub async fn atualizar_pedido_controlado(
    pedido_id: web::Path<i32>,
    request: web::Json<CriarPedidoRequest>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    let pedido_id_val = pedido_id.into_inner();
    log::info!("Atualizando pedido ID: {}", pedido_id_val);
    
    let mut conn = pools.sqlserver_portal.get().await
        .map_err(|e| ApiError::Database(format!("Erro ao conectar: {}", e)))?;
    
    // 1. Verificar se pedido pode ser editado
    let mut query_status = Query::new(r#"
        SELECT status_pedido, confirmado, integrado
        FROM pedidos 
        WHERE id = @P1
    "#);
    query_status.bind(pedido_id_val);
    
    let (status, confirmado, integrado) = match map_tiberius_error!(
        query_status.query(&mut conn).await,
        "Erro ao verificar status do pedido"
    )?.into_row().await
        .map_err(|e| ApiError::Database(format!("Erro ao processar status: {}", e)))? {
        Some(row) => {
            let status: String = row.get::<&str, _>(0).unwrap_or("").to_string();
            let confirmado: bool = row.get::<bool, _>(1).unwrap_or(false);
            let integrado: bool = row.get::<bool, _>(2).unwrap_or(false);
            (status, confirmado, integrado)
        },
        None => return Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": "Pedido não encontrado"
        })))
    };
    
    // 2. Validar se pode ser editado
    if confirmado || integrado || status == "integrado" {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": match status.as_str() {
                "confirmado" => "Pedido confirmado não pode ser editado. Cancele primeiro.",
                "integrado" => "Pedido integrado não pode ser editado",
                _ => "Pedido não pode ser editado no status atual"
            }
        })));
    }
    
    // 3. Validar dados da atualização (reutilizar validações existentes)
    let cliente_id = validar_cliente_existe(&mut conn, &request.codigo_cliente).await?;
    let (items_validados, _valor_total) = validar_items_atualizacao(
        &mut conn, &request, cliente_id
    ).await?;
    
    // 4. Atualizar em transação
    begin_transaction(&mut conn).await?;
    
    // Atualizar cabeçalho
    let mut update_pedido = Query::new(r#"
        UPDATE pedidos SET
            mensagem = @P1,
            regra_condicao_pagamento_id = @P2,
            regra_frete_id = @P3,
            updated_at = GETDATE()
        WHERE id = @P4
    "#);
    update_pedido.bind(request.mensagem.as_deref().unwrap_or(""));
    update_pedido.bind(request.regra_condicao_pagamento_id);
    update_pedido.bind(request.regra_frete_id);
    update_pedido.bind(pedido_id_val);
    
    if let Err(e) = update_pedido.execute(&mut conn).await {
        safe_rollback(&mut conn).await;
        return Err(ApiError::Database(format!("Erro atualizar pedido: {}", e)));
    }
    
    // Remover itens antigos
    let mut delete_items = Query::new("DELETE FROM items WHERE pedido_id = @P1");
    delete_items.bind(pedido_id_val);
    if let Err(e) = delete_items.execute(&mut conn).await {
        safe_rollback(&mut conn).await;
        return Err(ApiError::Database(format!("Erro remover itens: {}", e)));
    }
    
    // Inserir novos itens
    for (produto_id, quantidade, preco_unitario) in items_validados {
        let mut insert_item = Query::new(r#"
            INSERT INTO items (
                pedido_id, produto_id, codigo_produto, descricao_produto,
                quantidade, preco_unitario, percentual_desconto, valor_com_desconto,
                created_at, updated_at
            ) 
            SELECT @P1, @P2, p.codigo, p.descricao, @P3, @P4, 0, @P5, GETDATE(), GETDATE()
            FROM produtos p WHERE p.id = @P6
        "#);
        
        let valor_item = quantidade as f64 * preco_unitario;
        
        insert_item.bind(pedido_id_val);
        insert_item.bind(produto_id);
        insert_item.bind(quantidade);
        insert_item.bind(preco_unitario);
        insert_item.bind(valor_item);
        insert_item.bind(produto_id);
        
        if let Err(e) = insert_item.execute(&mut conn).await {
            safe_rollback(&mut conn).await;
            return Err(ApiError::Database(format!("Erro inserir item: {}", e)));
        }
    }
    
    commit_transaction(&mut conn).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Pedido atualizado com sucesso"
    })))
}

/// PATCH /pedidos/{id}/status  
/// Alterar status do pedido seguindo workflow
pub async fn alterar_status_pedido(
    pedido_id: web::Path<i32>,
    request: web::Json<AlterarStatusRequest>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    let pedido_id_val = pedido_id.into_inner();
    log::info!("Alterando status do pedido {} para: {}", pedido_id_val, request.novo_status);
    
    // Validar status válido
    let status_validos = vec![
        "a confirmar", "confirmado", "integrado", 
        "Confirmado ERP", "Em Separação", "Faturado", "Pronto pra Coleta"
    ];
    
    if !status_validos.contains(&request.novo_status.as_str()) {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": format!("Status '{}' inválido", request.novo_status)
        })));
    }
    
    let mut conn = pools.sqlserver_portal.get().await
        .map_err(|e| ApiError::Database(format!("Erro ao conectar: {}", e)))?;
    
    // Verificar status atual
    let mut query_current = Query::new("SELECT status_pedido FROM pedidos WHERE id = @P1");
    query_current.bind(pedido_id_val);
    
    let status_atual = match map_tiberius_error!(
        query_current.query(&mut conn).await,
        "Erro ao buscar status atual"
    )?.into_row().await
        .map_err(|e| ApiError::Database(format!("Erro ao processar status atual: {}", e)))? {
        Some(row) => row.get::<&str, _>(0).unwrap_or("").to_string(),
        None => return Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": "Pedido não encontrado"
        })))
    };
    
    // Validar transição de status
    let transicao_valida = validar_transicao_status(&status_atual, &request.novo_status);
    
    if !transicao_valida {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": format!("Transição de '{}' para '{}' não permitida", 
                              status_atual, request.novo_status)
        })));
    }
    
    // Atualizar status
    let mut update_status = Query::new(r#"
        UPDATE pedidos 
        SET status_pedido = @P1, updated_at = GETDATE()
        WHERE id = @P2
    "#);
    update_status.bind(&request.novo_status as &str);
    update_status.bind(pedido_id_val);
    
    map_tiberius_error!(
        update_status.execute(&mut conn).await,
        "Erro ao atualizar status do pedido"
    )?;
    
    // Log da alteração
    if let Some(obs) = &request.observacoes {
        log::info!("Status alterado para '{}' - Obs: {}", request.novo_status, obs);
    }
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "status_anterior": status_atual,
        "status_atual": request.novo_status,
        "message": "Status alterado com sucesso"
    })))
}

// Helper functions

async fn validar_cliente_existe(
    conn: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    codigo_cliente: &str
) -> Result<i32, ApiError> {
    let mut query = Query::new("SELECT id FROM clientes WHERE codigo = @P1");
    query.bind(codigo_cliente);
    
    match map_tiberius_error!(
        query.query(conn).await,
        "Erro ao buscar cliente"
    )?.into_row().await
        .map_err(|e| ApiError::Database(format!("Erro ao processar dados do cliente: {}", e)))? {
        Some(row) => Ok(row.get::<i32, _>(0).unwrap_or(0)),
        None => Err(ApiError::NotFound)
    }
}

async fn validar_items_atualizacao(
    conn: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    request: &CriarPedidoRequest,
    cliente_id: i32
) -> Result<(Vec<(i32, i32, f64)>, Decimal), ApiError> {
    // ✅ REUTILIZAR LÓGICA COMPLETA DO crud.rs - DADOS REAIS OBRIGATÓRIOS
    
    // Buscar grupo_venda do cliente para preços
    let mut query_cliente = Query::new(r#"
        SELECT grupo_venda 
        FROM clientes 
        WHERE id = @P1
    "#);
    query_cliente.bind(cliente_id);
    
    let grupo_venda = match map_tiberius_error!(
        query_cliente.query(conn).await,
        "Erro ao buscar grupo do cliente"
    )?.into_row().await
        .map_err(|e| ApiError::Database(format!("Erro ao processar grupo do cliente: {}", e)))? {
        Some(row) => row.get::<&str, _>(0).unwrap_or("").to_string(),
        None => return Err(ApiError::NotFound)
    };
    
    let mut items_validados = Vec::new();
    let mut valor_total = Decimal::ZERO;
    
    for item in &request.items {
        // 1. Validar produto existe e está ativo
        let mut query_produto = Query::new(r#"
            SELECT codigo, descricao, status, saldo, quantidade_minima_embalagem
            FROM produtos 
            WHERE id = @P1
        "#);
        query_produto.bind(item.produto_id);
        
        let produto = match map_tiberius_error!(
            query_produto.query(conn).await,
            "Erro ao buscar dados do produto"
        )?.into_row().await
            .map_err(|e| ApiError::Database(format!("Erro ao processar dados do produto: {}", e)))? {
            Some(row) => {
                let status: bool = row.get(2).unwrap_or(false);
                let saldo: i32 = row.get(3).unwrap_or(0);
                
                if !status {
                    return Err(ApiError::BadRequest(
                        format!("Produto {} está inativo", item.produto_id)
                    ));
                }
                
                if saldo <= 0 {
                    return Err(ApiError::BadRequest(
                        format!("Produto {} sem saldo em estoque", item.produto_id)
                    ));
                }
                
                row
            },
            None => return Err(ApiError::BadRequest(
                format!("Produto {} não encontrado", item.produto_id)
            ))
        };
        
        let codigo_produto: String = produto.get::<&str, _>(0).unwrap_or("").to_string();
        let qtd_min_embalagem: i32 = produto.get(4).unwrap_or(1);
        
        // 2. Validar quantidade mínima
        if item.quantidade < qtd_min_embalagem {
            return Err(ApiError::BadRequest(
                format!("Quantidade {} abaixo do mínimo {} para produto {}", 
                       item.quantidade, qtd_min_embalagem, codigo_produto)
            ));
        }
        
        // 3. Buscar preço por grupo_venda (DADOS REAIS)
        let mut query_preco = Query::new(r#"
            SELECT preco 
            FROM precos_produtos 
            WHERE codigo_produto = @P1 AND grupo_venda = @P2
        "#);
        query_preco.bind(&codigo_produto as &str);
        query_preco.bind(&grupo_venda as &str);
        
        // 3.1. Primeiro tentar buscar preço específico por grupo
        let preco_grupo_result = map_tiberius_error!(
            query_preco.query(conn).await,
            "Erro ao buscar preço por grupo"
        )?.into_row().await
            .map_err(|e| ApiError::Database(format!("Erro ao processar preço por grupo: {}", e)))?;
        
        let preco_unitario = if let Some(row) = preco_grupo_result {
            // Preço específico encontrado
            let preco: Decimal = row.get(0).unwrap_or(Decimal::ZERO);
            preco.to_f64().unwrap_or(0.0)
        } else {
            // Se não há preço específico, buscar preço padrão do produto
            let mut query_padrao = Query::new(r#"
                SELECT preco_unitario 
                FROM produtos 
                WHERE id = @P1
            "#);
            query_padrao.bind(item.produto_id);
            
            let preco_resultado = map_tiberius_error!(
                query_padrao.query(conn).await,
                "Erro ao buscar preço padrão"
            )?.into_row().await
                .map_err(|e| ApiError::Database(format!("Erro ao processar preço padrão: {}", e)))?;
            
            match preco_resultado {
                Some(row) => {
                    let preco: Option<Decimal> = row.get(0);
                    preco.unwrap_or(Decimal::ZERO).to_f64().unwrap_or(0.0)
                },
                None => return Err(ApiError::BadRequest(
                    format!("Preço não definido para produto {} no grupo {}", 
                           codigo_produto, grupo_venda)
                ))
            }
        };
        
        if preco_unitario <= 0.0 {
            return Err(ApiError::BadRequest(
                format!("Preço inválido para produto {}", codigo_produto)
            ));
        }
        
        // 4. Calcular valor do item (DADOS REAIS)
        let valor_item = Decimal::from(item.quantidade) * 
                        Decimal::try_from(preco_unitario).unwrap();
        valor_total += valor_item;
        
        items_validados.push((item.produto_id, item.quantidade, preco_unitario));
    }
    
    if valor_total <= Decimal::ZERO {
        return Err(ApiError::BadRequest("Total do pedido deve ser maior que zero".to_string()));
    }
    
    Ok((items_validados, valor_total))
}

fn validar_transicao_status(status_atual: &str, novo_status: &str) -> bool {
    match (status_atual, novo_status) {
        // Fluxo normal
        ("a confirmar", "confirmado") => true,
        ("confirmado", "integrado") => true,
        ("integrado", "Confirmado ERP") => true,
        ("Confirmado ERP", "Em Separação") => true,
        ("Em Separação", "Faturado") => true,
        ("Faturado", "Pronto pra Coleta") => true,
        
        // Retrocessos permitidos (exceções)
        ("confirmado", "a confirmar") => true, // Cancelar confirmação
        ("integrado", "confirmado") => true,   // Problema na integração
        
        // Mesmo status (idempotente)
        (atual, novo) if atual == novo => true,
        
        _ => false
    }
}
