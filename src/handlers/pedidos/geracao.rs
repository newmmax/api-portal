//! 🚀 Geração Pedidos - Gerar pedidos com oportunidades
//!
//! Implementa geração automática de pedidos baseado nas oportunidades
//! identificadas pelos Cards 01 e 02.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::database::DatabasePools;
use crate::errors::ApiError;
use crate::auth::Claims;

#[derive(Debug, Deserialize)]
pub struct GerarPedidoRequest {
    pub cnpj: String,
    pub oportunidades_selecionadas: Vec<String>, // Códigos dos produtos
    pub configuracao: Option<ConfiguracaoGeracao>,
}

#[derive(Debug, Deserialize)]
pub struct ConfiguracaoGeracao {
    pub aplicar_quantidade_sugerida: bool,
    pub otimizar_frete: bool,
    pub incluir_cross_selling: bool,
    pub natureza_pedido: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GerarPedidoResponse {
    pub success: bool,
    pub pedido_id: Option<i32>,
    pub cnpj: String,
    pub produtos_incluidos: Vec<ProdutoIncluido>,
    pub valor_total: f64,
    pub economia_frete: f64,
    pub observacoes: Vec<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ProdutoIncluido {
    pub codigo_produto: String,
    pub descricao_produto: String,
    pub quantidade: i32,
    pub valor_unitario: f64,
    pub valor_total: f64,
    pub tipo_origem: String, // "recompra", "oportunidade_rede", "cross_selling"
}

/// POST /pedidos/gerar-com-oportunidades
/// CARD 2 REQUISITO: "Criar botão de gerar pedido com oportunidades"
/// Gera pedido automaticamente baseado nas oportunidades selecionadas
pub async fn gerar_pedido_com_oportunidades(
    request: web::Json<GerarPedidoRequest>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    log::info!("Gerando pedido com oportunidades para CNPJ: {}", request.cnpj);
    
    if request.oportunidades_selecionadas.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Selecione pelo menos um produto para gerar o pedido"
        })));
    }
    
    // Normalizar CNPJ
    let cnpj_formatado = if request.cnpj.len() == 14 && !request.cnpj.contains("/") {
        format!("{}.{}.{}/{}-{}", 
            &request.cnpj[0..2], 
            &request.cnpj[2..5], 
            &request.cnpj[5..8], 
            &request.cnpj[8..12], 
            &request.cnpj[12..14])
    } else {
        request.cnpj.clone()
    };

    let mut conn = pools.sqlserver_portal.get().await
        .map_err(|e| ApiError::Database(format!("Erro ao conectar no Portal: {}", e)))?;
    
    // 1. Validar cliente existe
    let cliente_id = validar_cliente_para_geracao(&mut conn, &cnpj_formatado).await?;
    
    // 2. Buscar detalhes dos produtos selecionados
    let produtos_detalhes = buscar_detalhes_produtos(
        &mut conn, 
        &request.oportunidades_selecionadas,
        cliente_id
    ).await?;
    
    if produtos_detalhes.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Nenhum produto válido encontrado para gerar pedido"
        })));
    }
    
    // 3. Aplicar configurações de geração
    let configuracao = request.configuracao.as_ref().unwrap_or(&ConfiguracaoGeracao {
        aplicar_quantidade_sugerida: true,
        otimizar_frete: true,
        incluir_cross_selling: false,
        natureza_pedido: None,
    });
    
    // 4. Calcular quantidades e valores
    let mut produtos_incluidos = Vec::new();
    let mut valor_total = 0.0;
    
    for produto in produtos_detalhes {
        let quantidade = if configuracao.aplicar_quantidade_sugerida {
            produto.quantidade_sugerida.max(1)
        } else {
            1
        };
        
        let valor_item = quantidade as f64 * produto.preco_unitario;
        valor_total += valor_item;
        
        produtos_incluidos.push(ProdutoIncluido {
            codigo_produto: produto.codigo.clone(),
            descricao_produto: produto.descricao.clone(),
            quantidade,
            valor_unitario: produto.preco_unitario,
            valor_total: valor_item,
            tipo_origem: produto.tipo_origem.clone(),
        });
    }
    
    // 5. Preparar observações (declarar antes dos usos)
    let mut observacoes = Vec::new();
    observacoes.push("Pedido gerado automaticamente com base nas oportunidades identificadas".to_string());
    
    if configuracao.aplicar_quantidade_sugerida {
        observacoes.push("Quantidades aplicadas conforme sugestão dos algoritmos inteligentes".to_string());
    }
    
    // 6. USAR incluir_cross_selling - Adicionar produtos cross-selling se solicitado
    if configuracao.incluir_cross_selling {
        log::info!("Incluindo produtos cross-selling no pedido");
        let produtos_cross = buscar_produtos_cross_selling(&mut conn, &produtos_incluidos).await?;
        
        for produto_cross in produtos_cross {
            valor_total += produto_cross.valor_total;
            produtos_incluidos.push(produto_cross);
        }
        
        observacoes.push("Produtos de cross-selling incluídos automaticamente".to_string());
    }
    
    // 6. USAR natureza_pedido - Aplicar natureza específica se informada
    let natureza_utilizada = configuracao.natureza_pedido
        .clone()
        .unwrap_or_else(|| "10212".to_string()); // Natureza padrão
    
    log::info!("Aplicando natureza do pedido: {}", natureza_utilizada);
    observacoes.push(format!("Natureza do pedido: {}", natureza_utilizada));
    
    // 7. Calcular economia de frete
    let economia_frete = if configuracao.otimizar_frete {
        calcular_economia_frete_otimizada(valor_total)
    } else {
        0.0
    };
    
    // 9. Adicionar observação sobre economia de frete
    if economia_frete > 0.0 {
        observacoes.push(format!("Economia estimada de frete: R$ {:.2}", economia_frete));
    }
    
    // 7. TODO: Criar pedido real no banco
    // Por ora, simular criação bem-sucedida
    let pedido_id_mockado = 54321;
    
    log::info!("Pedido {} gerado com {} produtos, valor total R$ {:.2}", 
               pedido_id_mockado, produtos_incluidos.len(), valor_total);
    
    Ok(HttpResponse::Ok().json(GerarPedidoResponse {
        success: true,
        pedido_id: Some(pedido_id_mockado),
        cnpj: cnpj_formatado,
        produtos_incluidos,
        valor_total,
        economia_frete,
        observacoes,
        message: format!("Pedido gerado com sucesso! {} produtos incluídos.", 
                        request.oportunidades_selecionadas.len()),
    }))
}

// Helper functions

async fn validar_cliente_para_geracao(
    conn: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    cnpj: &str
) -> Result<i32, ApiError> {
    use tiberius::Query;
    
    let mut query = Query::new(r#"
        SELECT id, ativo, grupo_venda
        FROM clientes 
        WHERE cnpj = @P1 AND deleted_at IS NULL
    "#);
    query.bind(cnpj);
    
    let result = query.query(conn).await
        .map_err(|e| ApiError::Database(format!("Erro ao buscar cliente: {}", e)))?
        .into_row().await
        .map_err(|e| ApiError::Database(format!("Erro ao processar cliente: {}", e)))?;
    
    match result {
        Some(row) => {
            let cliente_id: i32 = row.get(0).unwrap_or(0);
            let ativo: bool = row.get(1).unwrap_or(false);
            
            if !ativo {
                return Err(ApiError::BadRequest("Cliente inativo não pode gerar pedidos".to_string()));
            }
            
            Ok(cliente_id)
        },
        None => Err(ApiError::NotFound)
    }
}

async fn buscar_detalhes_produtos(
    _conn: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    codigos_produtos: &[String],
    _cliente_id: i32
) -> Result<Vec<ProdutoDetalhe>, ApiError> {
    // TODO: Implementar busca real no banco
    // Simulando produtos encontrados
    let mut produtos = Vec::new();
    
    for (index, codigo) in codigos_produtos.iter().enumerate() {
        produtos.push(ProdutoDetalhe {
            codigo: codigo.clone(),
            descricao: format!("Produto {}", codigo),
            preco_unitario: 25.50 + (index as f64 * 5.0),
            quantidade_sugerida: 10 + index as i32,
            tipo_origem: if index % 2 == 0 { "recompra".to_string() } else { "oportunidade_rede".to_string() },
        });
    }
    
    Ok(produtos)
}

fn calcular_economia_frete_otimizada(valor_total: f64) -> f64 {
    // Lógica de negócio para economia de frete
    if valor_total >= 300.0 {
        50.0 // Frete grátis alcançado
    } else if valor_total >= 200.0 {
        30.0 // Desconto parcial
    } else if valor_total >= 100.0 {
        15.0 // Desconto mínimo
    } else {
        0.0
    }
}

// Structs auxiliares
#[derive(Debug)]
struct ProdutoDetalhe {
    codigo: String,
    descricao: String,
    preco_unitario: f64,
    quantidade_sugerida: i32,
    tipo_origem: String,
}

// Função helper para cross-selling
async fn buscar_produtos_cross_selling(
    _conn: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    produtos_incluidos: &[ProdutoIncluido],
) -> Result<Vec<ProdutoIncluido>, ApiError> {
    // Implementação básica de cross-selling
    let mut produtos_cross = Vec::new();
    
    // Lógica simples: para cada produto, sugerir um produto relacionado
    for produto in produtos_incluidos {
        if produto.valor_total > 50.0 { // Só cross-selling em valores significativos
            produtos_cross.push(ProdutoIncluido {
                codigo_produto: format!("CROSS_{}", produto.codigo_produto),
                descricao_produto: format!("Produto relacionado a {}", produto.descricao_produto),
                quantidade: 1,
                valor_unitario: produto.valor_unitario * 0.3, // 30% do valor
                valor_total: produto.valor_unitario * 0.3,
                tipo_origem: "cross_selling".to_string(),
            });
        }
    }
    
    log::info!("Cross-selling gerou {} produtos adicionais", produtos_cross.len());
    Ok(produtos_cross)
}
