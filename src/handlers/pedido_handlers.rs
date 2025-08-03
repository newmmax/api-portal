use actix_web::{web, HttpResponse};
use rust_decimal::Decimal;
use serde_json::json;
use tiberius::Query;
use crate::database::DatabasePools;
use crate::errors::ApiError;
use crate::auth::Claims;
use crate::models::{
    CriarPedidoRequest, CriarPedidoResponse
};

/// Criar novo pedido no Portal
pub async fn criar_pedido(
    pedido_req: web::Json<CriarPedidoRequest>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    log::info!("Criando novo pedido para cliente: {}", pedido_req.codigo_cliente);
    
    let mut conn = pools.sqlserver_portal.get().await
        .map_err(|e| ApiError::Database(format!("Erro ao conectar no Portal: {}", e)))?;
    
    // 1. Validar cliente existe e está ativo
    let cliente_query = r#"
        SELECT id, grupo_venda, ativo 
        FROM clientes 
        WHERE codigo = @P1 AND loja = @P2
    "#;
    
    let mut query = Query::new(cliente_query);
    query.bind(&pedido_req.codigo_cliente as &str);
    query.bind(&pedido_req.loja_cliente as &str);
    
    let cliente_row = query.query(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro ao buscar cliente: {}", e)))?
        .into_row().await
        .map_err(|e| ApiError::Database(format!("Erro ao processar cliente: {}", e)))?;
    
    if cliente_row.is_none() {
        return Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": "Cliente não encontrado"
        })));
    }
    
    let cliente_row = cliente_row.unwrap();
    let cliente_id: i32 = cliente_row.get(0).unwrap_or(0);
    let grupo_venda: &str = cliente_row.get(1).unwrap_or("");
    let ativo: bool = cliente_row.get(2).unwrap_or(false);
    
    if !ativo {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Cliente inativo não pode criar pedidos"
        })));
    }
    
    // 2. Validar regras existem e calcular total
    let mut valor_total = Decimal::ZERO;
    let mut items_validados = Vec::new();
    
    for item in &pedido_req.items {
        // Buscar produto e preço
        let produto_query = r#"
            SELECT p.id, p.codigo, p.descricao, p.saldo, p.status, 
                   p.quantidade_minima_embalagem, pp.preco
            FROM produtos p
            LEFT JOIN precos_produtos pp ON p.codigo = pp.codigo_produto
            WHERE p.id = @P1 AND pp.grupo_venda = @P2
        "#;
        
        let mut prod_query = Query::new(produto_query);
        prod_query.bind(item.produto_id);
        prod_query.bind(grupo_venda);
        
        let prod_row = prod_query.query(&mut conn).await
            .map_err(|e| ApiError::Database(format!("Erro ao buscar produto: {}", e)))?
            .into_row().await
            .map_err(|e| ApiError::Database(format!("Erro: {}", e)))?;
        
        if prod_row.is_none() {
            return Ok(HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": format!("Produto {} não encontrado ou sem preço para o grupo", item.produto_id)
            })));
        }
        
        let prod = prod_row.unwrap();
        let prod_id: i32 = prod.get(0).unwrap_or(0);
        let codigo: &str = prod.get(1).unwrap_or("");
        let saldo: i32 = prod.get(3).unwrap_or(0);
        let status: bool = prod.get(4).unwrap_or(false);
        let preco: f64 = prod.get(6).unwrap_or(0.0);
        
        if !status {
            return Ok(HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": format!("Produto {} está inativo", codigo)
            })));
        }
        
        if saldo < item.quantidade {
            return Ok(HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": format!("Produto {} sem saldo suficiente", codigo)
            })));
        }
        
        let preco_decimal = Decimal::try_from(preco).unwrap_or(Decimal::ZERO);
        let subtotal = preco_decimal * Decimal::from(item.quantidade);
        valor_total += subtotal;
        
        items_validados.push((prod_id, item.quantidade, preco));
    }
    
    if valor_total <= Decimal::ZERO {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Pedido deve ter valor maior que zero"
        })));
    }
    
    // 3. Criar pedido com transação
    Query::new("BEGIN TRANSACTION").execute(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro ao iniciar transação: {}", e)))?;
    
    let natureza = pedido_req.natureza.as_deref().unwrap_or("10212");
    
    // Inserir pedido
    let insert_pedido = r#"
        INSERT INTO pedidos (
            cliente_id, codigo_cliente, loja_cliente, emissao, mensagem, 
            natureza, status_pedido, regra_condicao_pagamento_id, 
            regra_frete_id, tabela_precos, integrado, confirmado,
            created_at, updated_at
        ) VALUES (
            @P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9, @P10, 0, 0,
            GETDATE(), GETDATE()
        );
        SELECT SCOPE_IDENTITY() AS id;
    "#;
    
    let mut insert_query = Query::new(insert_pedido);
    insert_query.bind(cliente_id);
    insert_query.bind(&pedido_req.codigo_cliente as &str);
    insert_query.bind(&pedido_req.loja_cliente as &str);
    insert_query.bind(&pedido_req.emissao as &str);
    insert_query.bind(pedido_req.mensagem.as_deref().unwrap_or(""));
    insert_query.bind(natureza);
    insert_query.bind("a confirmar");
    insert_query.bind(pedido_req.regra_condicao_pagamento_id);
    insert_query.bind(pedido_req.regra_frete_id);
    insert_query.bind(grupo_venda);
    
    let pedido_result = insert_query.query(&mut conn).await
        .map_err(|e| {
            ApiError::Database(format!("Erro ao criar pedido: {}", e))
        })?
        .into_row().await
        .map_err(|e| {
            ApiError::Database(format!("Erro: {}", e))
        })?;
    
    if pedido_result.is_none() {
        Query::new("ROLLBACK").execute(&mut conn).await.ok();
        return Err(ApiError::Database("Erro ao obter ID do pedido".to_string()));
    }
    
    let pedido_id: f64 = pedido_result.unwrap().get(0).unwrap_or(0.0);
    let pedido_id = pedido_id as i32;
    
    // Inserir items
    for (produto_id, quantidade, preco) in items_validados {
        let insert_item = r#"
            INSERT INTO items (
                pedido_id, produto_id, quantidade, preco_unitario,
                created_at, updated_at
            ) VALUES (
                @P1, @P2, @P3, @P4, GETDATE(), GETDATE()
            )
        "#;
        
        let mut item_query = Query::new(insert_item);
        item_query.bind(pedido_id);
        item_query.bind(produto_id);
        item_query.bind(quantidade);
        item_query.bind(preco);
        
        item_query.execute(&mut conn).await
            .map_err(|e| {
                ApiError::Database(format!("Erro ao criar item: {}", e))
            })?;
    }
    
    // Commit
    Query::new("COMMIT").execute(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro ao confirmar: {}", e)))?;
    
    log::info!("Pedido {} criado com sucesso", pedido_id);
    
    Ok(HttpResponse::Ok().json(CriarPedidoResponse {
        success: true,
        pedido_id: Some(pedido_id),
        numero_pedido: None,
        total: Some(valor_total),
        message: "Pedido criado com sucesso! Confirme para finalizar.".to_string(),
        errors: None,
    }))
}

/// Confirmar pedido existente
pub async fn confirmar_pedido(
    pedido_id: web::Path<i32>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    log::info!("Confirmando pedido ID: {}", pedido_id);
    
    let mut conn = pools.sqlserver_portal.get().await
        .map_err(|e| ApiError::Database(format!("Erro ao conectar: {}", e)))?;
    
    // Verificar status
    let pedido_id_value = pedido_id.into_inner();
    
    let mut query = Query::new("SELECT status_pedido FROM pedidos WHERE id = @P1");
    query.bind(pedido_id_value);
    
    let result = query.query(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro: {}", e)))?
        .into_row().await
        .map_err(|e| ApiError::Database(format!("Erro: {}", e)))?;
    
    if result.is_none() {
        return Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": "Pedido não encontrado"
        })));
    }
    
    let row = result.unwrap();
    let status: &str = row.get(0).unwrap_or("");
    
    if status == "confirmado" {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Pedido já está confirmado"
        })));
    }
    
    if status != "a confirmar" {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Pedido não pode ser confirmado neste status"
        })));
    }
    
    // Atualizar para confirmado
    let update_query = r#"
        UPDATE pedidos 
        SET status_pedido = 'confirmado',
            emissao = CONVERT(date, GETDATE()),
            updated_at = GETDATE()
        WHERE id = @P1
    "#;
    
    let mut update = Query::new(update_query);
    update.bind(pedido_id_value);
    
    update.execute(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro ao confirmar: {}", e)))?;
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Pedido confirmado com sucesso!"
    })))
}
/// Buscar pedido específico
pub async fn buscar_pedido(
    pedido_id: web::Path<i32>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    log::info!("Buscando pedido ID: {}", pedido_id);
    
    let mut conn = pools.sqlserver_portal.get().await
        .map_err(|e| ApiError::Database(format!("Erro ao conectar: {}", e)))?;
    
    // Buscar pedido
    let query_pedido = r#"
        SELECT p.*, c.razao_social, c.grupo_venda
        FROM pedidos p
        INNER JOIN clientes c ON p.cliente_id = c.id
        WHERE p.id = @P1
    "#;
    
    let pedido_id_val = pedido_id.into_inner();
    
    let mut query = Query::new(query_pedido);
    query.bind(pedido_id_val);
    
    let pedido_row = query.query(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro ao buscar pedido: {}", e)))?
        .into_row().await
        .map_err(|e| ApiError::Database(format!("Erro: {}", e)))?;
    
    if pedido_row.is_none() {
        return Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": "Pedido não encontrado"
        })));
    }
    
    let row = pedido_row.unwrap();
    
    // Buscar items do pedido
    let query_items = r#"
        SELECT i.*, p.codigo, p.descricao, p.unidade_medida
        FROM items i
        INNER JOIN produtos p ON i.produto_id = p.id
        WHERE i.pedido_id = @P1
    "#;
    
    let mut items_query = Query::new(query_items);
    items_query.bind(pedido_id_val);
    
    let items_result = items_query.query(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro ao buscar items: {}", e)))?
        .into_first_result().await
        .map_err(|e| ApiError::Database(format!("Erro: {}", e)))?;
    
    let mut items = Vec::new();
    for item_row in items_result {
        items.push(json!({
            "id": item_row.get::<i32, _>(0).unwrap_or(0),
            "produto_id": item_row.get::<i32, _>(2).unwrap_or(0),
            "quantidade": item_row.get::<i32, _>(3).unwrap_or(0),
            "preco_unitario": item_row.get::<f64, _>(4).unwrap_or(0.0),
            "codigo_produto": item_row.get::<&str, _>(7).unwrap_or(""),
            "descricao_produto": item_row.get::<&str, _>(8).unwrap_or(""),
            "unidade_medida": item_row.get::<&str, _>(9).unwrap_or("")
        }));
    }
    
    // Montar resposta
    let pedido_json = json!({
        "id": row.get::<i32, _>(0).unwrap_or(0),
        "cliente_id": row.get::<i32, _>(1).unwrap_or(0),
        "codigo_cliente": row.get::<&str, _>(2).unwrap_or(""),
        "numero_pedido": row.get::<&str, _>(3),
        "loja_cliente": row.get::<&str, _>(4).unwrap_or(""),
        "emissao": row.get::<&str, _>(5).unwrap_or(""),
        "mensagem": row.get::<&str, _>(11),
        "natureza": row.get::<&str, _>(12).unwrap_or(""),
        "status_pedido": row.get::<&str, _>(13).unwrap_or(""),
        "regra_condicao_pagamento_id": row.get::<i32, _>(26).unwrap_or(0),
        "regra_frete_id": row.get::<i32, _>(27).unwrap_or(0),
        "razao_social": row.get::<&str, _>(29).unwrap_or(""),
        "grupo_venda": row.get::<&str, _>(30).unwrap_or(""),
        "items": items
    });
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": pedido_json
    })))
}

/// Atualizar pedido existente
pub async fn atualizar_pedido(
    pedido_id: web::Path<i32>,
    pedido_req: web::Json<CriarPedidoRequest>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    log::info!("Atualizando pedido ID: {}", pedido_id);
    
    let mut conn = pools.sqlserver_portal.get().await
        .map_err(|e| ApiError::Database(format!("Erro ao conectar: {}", e)))?;
    
    let pedido_id_val = pedido_id.into_inner();
    
    // Verificar se pedido existe e pode ser editado
    let mut query = Query::new("SELECT status_pedido, cliente_id FROM pedidos WHERE id = @P1");
    query.bind(pedido_id_val);
    
    let result = query.query(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro: {}", e)))?
        .into_row().await
        .map_err(|e| ApiError::Database(format!("Erro: {}", e)))?;
    
    if result.is_none() {
        return Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": "Pedido não encontrado"
        })));
    }
    
    let row = result.unwrap();
    let status: &str = row.get(0).unwrap_or("");
    let _cliente_id_pedido: i32 = row.get(1).unwrap_or(0);
    
    // Só permite editar pedidos com status "a confirmar"
    if status != "a confirmar" {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Apenas pedidos com status 'a confirmar' podem ser editados"
        })));
    }
    
    // Buscar dados do cliente para validações
    let cliente_query = r#"
        SELECT id, grupo_venda, ativo 
        FROM clientes 
        WHERE codigo = @P1 AND loja = @P2
    "#;
    
    let mut query = Query::new(cliente_query);
    query.bind(&pedido_req.codigo_cliente as &str);
    query.bind(&pedido_req.loja_cliente as &str);
    
    let cliente_row = query.query(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro: {}", e)))?
        .into_row().await
        .map_err(|e| ApiError::Database(format!("Erro: {}", e)))?;
    
    if cliente_row.is_none() {
        return Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": "Cliente não encontrado"
        })));
    }
    
    let cliente_row = cliente_row.unwrap();
    let _cliente_id: i32 = cliente_row.get(0).unwrap_or(0);
    let grupo_venda: &str = cliente_row.get(1).unwrap_or("");
    
    // Validar items e calcular total
    let mut valor_total = Decimal::ZERO;
    let mut items_validados = Vec::new();
    
    for item in &pedido_req.items {
        // Validar produto (mesmo código da criação)
        let produto_query = r#"
            SELECT p.id, p.codigo, p.saldo, p.status, pp.preco
            FROM produtos p
            LEFT JOIN precos_produtos pp ON p.codigo = pp.codigo_produto
            WHERE p.id = @P1 AND pp.grupo_venda = @P2
        "#;
        
        let mut prod_query = Query::new(produto_query);
        prod_query.bind(item.produto_id);
        prod_query.bind(grupo_venda);
        
        let prod_row = prod_query.query(&mut conn).await
            .map_err(|e| ApiError::Database(format!("Erro: {}", e)))?
            .into_row().await
            .map_err(|e| ApiError::Database(format!("Erro: {}", e)))?;
        
        if prod_row.is_none() {
            return Ok(HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": format!("Produto {} não encontrado ou sem preço", item.produto_id)
            })));
        }
        
        let prod = prod_row.unwrap();
        let prod_id: i32 = prod.get(0).unwrap_or(0);
        let codigo: &str = prod.get(1).unwrap_or("");
        let saldo: i32 = prod.get(2).unwrap_or(0);
        let status: bool = prod.get(3).unwrap_or(false);
        let preco: f64 = prod.get(4).unwrap_or(0.0);
        
        if !status {
            return Ok(HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": format!("Produto {} está inativo", codigo)
            })));
        }
        
        if saldo < item.quantidade {
            return Ok(HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": format!("Produto {} sem saldo suficiente", codigo)
            })));
        }
        
        let preco_decimal = Decimal::try_from(preco).unwrap_or(Decimal::ZERO);
        let subtotal = preco_decimal * Decimal::from(item.quantidade);
        valor_total += subtotal;
        
        items_validados.push((prod_id, item.quantidade, preco));
    }
    
    // Iniciar transação
    Query::new("BEGIN TRANSACTION").execute(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro: {}", e)))?;
    
    // Atualizar pedido
    let update_pedido = r#"
        UPDATE pedidos SET
            codigo_cliente = @P1,
            loja_cliente = @P2,
            mensagem = @P3,
            regra_condicao_pagamento_id = @P4,
            regra_frete_id = @P5,
            updated_at = GETDATE()
        WHERE id = @P6
    "#;
    
    let mut update_query = Query::new(update_pedido);
    update_query.bind(&pedido_req.codigo_cliente as &str);
    update_query.bind(&pedido_req.loja_cliente as &str);
    update_query.bind(pedido_req.mensagem.as_deref().unwrap_or(""));
    update_query.bind(pedido_req.regra_condicao_pagamento_id);
    update_query.bind(pedido_req.regra_frete_id);
    update_query.bind(pedido_id_val);
    
    update_query.execute(&mut conn).await
        .map_err(|e| {
            let _ = Query::new("ROLLBACK").execute(&mut conn);
            ApiError::Database(format!("Erro ao atualizar pedido: {}", e))
        })?;
    
    // Deletar items antigos
    let mut delete_query = Query::new("DELETE FROM items WHERE pedido_id = @P1");
    delete_query.bind(pedido_id_val);
    
    delete_query.execute(&mut conn).await
        .map_err(|e| {
            let _ = Query::new("ROLLBACK").execute(&mut conn);
            ApiError::Database(format!("Erro ao deletar items: {}", e))
        })?;
    
    // Inserir novos items
    for (produto_id, quantidade, preco) in items_validados {
        let insert_item = r#"
            INSERT INTO items (
                pedido_id, produto_id, quantidade, preco_unitario,
                created_at, updated_at
            ) VALUES (
                @P1, @P2, @P3, @P4, GETDATE(), GETDATE()
            )
        "#;
        
        let mut item_query = Query::new(insert_item);
        item_query.bind(pedido_id_val);
        item_query.bind(produto_id);
        item_query.bind(quantidade);
        item_query.bind(preco);
        
        item_query.execute(&mut conn).await
            .map_err(|e| {
                let _ = Query::new("ROLLBACK").execute(&mut conn);
                ApiError::Database(format!("Erro ao criar item: {}", e))
            })?;
    }
    
    // Commit
    Query::new("COMMIT").execute(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro ao confirmar: {}", e)))?;
    
    log::info!("Pedido {} atualizado com sucesso", pedido_id_val);
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Pedido atualizado com sucesso!",
        "pedido_id": pedido_id_val,
        "total": valor_total.to_string()
    })))
}

/// Deletar pedido
pub async fn deletar_pedido(
    pedido_id: web::Path<i32>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    log::info!("Deletando pedido ID: {}", pedido_id);
    
    let mut conn = pools.sqlserver_portal.get().await
        .map_err(|e| ApiError::Database(format!("Erro ao conectar: {}", e)))?;
    
    let pedido_id_val = pedido_id.into_inner();
    
    // Verificar se pedido existe e pode ser deletado
    let mut query = Query::new("SELECT status_pedido FROM pedidos WHERE id = @P1");
    query.bind(pedido_id_val);
    
    let result = query.query(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro: {}", e)))?
        .into_row().await
        .map_err(|e| ApiError::Database(format!("Erro: {}", e)))?;
    
    if result.is_none() {
        return Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": "Pedido não encontrado"
        })));
    }
    
    let row = result.unwrap();
    let status: &str = row.get(0).unwrap_or("");
    
    // Só permite deletar pedidos com status "a confirmar"
    if status != "a confirmar" {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Apenas pedidos com status 'a confirmar' podem ser excluídos"
        })));
    }
    
    // Deletar pedido (cascade deleta os items)
    let mut delete_query = Query::new("DELETE FROM pedidos WHERE id = @P1");
    delete_query.bind(pedido_id_val);
    
    let rows_affected = delete_query.execute(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro ao deletar: {}", e)))?
        .total();
    
    if rows_affected == 0 {
        return Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "Erro ao excluir pedido"
        })));
    }
    
    log::info!("Pedido {} deletado com sucesso", pedido_id_val);
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Pedido excluído com sucesso!"
    })))
}