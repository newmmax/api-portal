//! 🎯 Card 01: Recompra Inteligente
//! 
//! Algoritmo de IA que analisa histórico de compras para sugerir recompras
//! baseado em frequência vs recência de compra.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tiberius::{Query, QueryItem};
use futures_util::TryStreamExt;
use crate::database::DatabasePools;
use crate::errors::ApiError;
use crate::auth::Claims;
use std::time::Instant;
use crate::cards_log;

#[derive(Debug, Deserialize)]
pub struct RecompraParams {
    pub cnpj: String,
    pub periodo_dias: Option<i32>, // Padrão: 90 dias
    pub limite: Option<i32>,       // Padrão: 50 produtos
}

#[derive(Debug, Serialize)]
pub struct ProdutoRecompra {
    pub codigo_produto: String,
    pub descricao_produto: String,
    pub categoria: Option<String>,
    pub frequencia_compra: i32,
    pub quantidade_media: f64,
    pub valor_medio: f64,
    pub dias_ultima_compra: i32,
    pub score_recompra: f64,
    pub nivel_prioridade: String,      // ALTA, MÉDIA, BAIXA
    pub sugestao_inteligente: String,  // Mensagem personalizada
    pub produtos_relacionados: Vec<ProdutoRelacionado>,
}

#[derive(Debug, Serialize)]
pub struct ProdutoRelacionado {
    pub codigo_produto: String,
    pub descricao_produto: String,
    pub correlacao_percentual: f64,
    pub vendas_conjuntas: i32,
}

/// Card 01: Análise de Recompra Inteligente
/// Analisa histórico de pedidos do franqueado para sugerir recompras
pub async fn recompra_inteligente(
    params: web::Query<RecompraParams>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    let _start_time = Instant::now();
    let card_name = "recompra-inteligente";
    
    // 🔍 LOG: Início da requisição
    cards_log!(request, card_name, &params.cnpj, 
               &format!("periodo_dias={:?}, limite={:?}", params.periodo_dias, params.limite));
    
    log::info!("Análise de recompra inteligente para CNPJ: {}", params.cnpj);
    
    let periodo_dias = params.periodo_dias.unwrap_or(90);
    let limite = params.limite.unwrap_or(50);
    
    // Buscar padrões de recompra do franqueado específico
    let mut conn = match pools.sqlserver_portal.get().await {
        Ok(conn) => conn,
        Err(e) => {
            let error_msg = format!("Erro ao conectar no Portal: {}", e);
            cards_log!(error, card_name, &params.cnpj, &error_msg, "database_connection");
            log::error!("{}", error_msg);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Erro de conexão com Portal", 
                "error": e.to_string()
            })));
        }
    };
    
    // Query otimizada baseada no algoritmo de IA
    let sql_recompra = r#"
        -- 🎯 CARD 01: RECOMPRA INTELIGENTE - ALGORITMO DE IA PARA SUGESTÕES
        -- Analisa padrão histórico de compras e sugere produtos para recompra
        -- Baseado em frequência de compra vs tempo desde última compra

        WITH produtos_recompra AS (
            SELECT 
                pr.codigo as codigo_produto,
                pr.descricao as descricao_produto,
                cat.nome as categoria,
                
                -- 📊 MÉTRICAS DE COMPORTAMENTO DE COMPRA
                COUNT(DISTINCT p.id) as frequencia_compra,
                AVG(CAST(i.quantidade AS FLOAT)) as quantidade_media,
                AVG(i.preco_unitario * i.quantidade) as valor_medio,
                
                -- ⏰ ANÁLISE TEMPORAL
                DATEDIFF(day, MAX(p.created_at), GETDATE()) as dias_ultima_compra,
                
                -- 🎯 SCORE INTELIGENTE DE RECOMPRA (ALGORITMO PRINCIPAL)
                (COUNT(DISTINCT p.id) * 10.0 / 
                 CASE WHEN DATEDIFF(day, MAX(p.created_at), GETDATE()) > 0 
                      THEN DATEDIFF(day, MAX(p.created_at), GETDATE()) 
                      ELSE 1 END) as score_recompra
                      
            FROM pedidos p
            INNER JOIN items i ON p.id = i.pedido_id
            INNER JOIN clientes c ON p.cliente_id = c.id
            INNER JOIN produtos pr ON i.produto_id = pr.id
            LEFT JOIN categorias cat ON pr.categoria_id = cat.id
            
            WHERE 
                c.cnpj = @P1
                AND p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
                AND p.created_at >= DATEADD(day, -@P2, GETDATE())
                AND c.deleted_at IS NULL
                AND pr.status = 1
                
            GROUP BY pr.codigo, pr.descricao, cat.nome
            HAVING COUNT(DISTINCT p.id) >= 1
        )

        SELECT TOP (@P3)
            codigo_produto,
            descricao_produto,
            categoria,
            frequencia_compra,
            quantidade_media,
            valor_medio,
            dias_ultima_compra,
            CAST(score_recompra AS DECIMAL(10,2)) as score_recompra,
            
            -- 📈 CLASSIFICAÇÃO DE PRIORIDADE
            CASE 
                WHEN score_recompra >= 3.0 THEN 'ALTA'
                WHEN score_recompra >= 1.0 THEN 'MÉDIA'
                ELSE 'BAIXA'
            END as nivel_prioridade,
            
            -- 💡 MENSAGEM PERSONALIZADA
            CASE 
                WHEN score_recompra >= 3.0 THEN 'Produto em reposição! Sugerimos incluir no próximo pedido.'
                WHEN score_recompra >= 1.0 THEN 'Padrão de recompra identificado. Considere reabastecer.'
                ELSE 'Produto já comprado anteriormente. Avalie necessidade atual.'
            END as sugestao_inteligente

        FROM produtos_recompra
        ORDER BY score_recompra DESC
    "#;
    
    // Normalizar CNPJ para formatação padrão
    let cnpj_formatado = if params.cnpj.len() == 14 && !params.cnpj.contains("/") {
        format!("{}.{}.{}/{}-{}", 
            &params.cnpj[0..2], 
            &params.cnpj[2..5], 
            &params.cnpj[5..8], 
            &params.cnpj[8..12], 
            &params.cnpj[12..14])
    } else {
        params.cnpj.clone()
    };

    // 🔍 LOG: Normalização de CNPJ
    cards_log!(cnpj, &params.cnpj, &cnpj_formatado);
    log::info!("CNPJ original: {} | CNPJ formatado: {}", params.cnpj, cnpj_formatado);

    let mut query = Query::new(sql_recompra);
    query.bind(&cnpj_formatado);
    query.bind(periodo_dias);
    query.bind(limite);
    
    let sql_start = Instant::now();
    let result = query.query(&mut conn).await
        .map_err(|e| {
            let error_msg = format!("Erro ao buscar dados recompra: {}", e);
            cards_log!(error, card_name, &cnpj_formatado, &error_msg, "sql_execution");
            ApiError::Database(error_msg)
        })?;
    
    let _sql_duration = sql_start.elapsed();
    let mut produtos_recompra = Vec::new();
    let mut stream = result;
    
    while let Some(item) = stream.try_next().await
        .map_err(|e| ApiError::Database(format!("Erro ao ler dados recompra: {}", e)))? {
        match item {
            QueryItem::Row(row) => {
                produtos_recompra.push(ProdutoRecompra {
                    codigo_produto: row.get::<&str, _>(0).unwrap_or("").to_string(),
                    descricao_produto: row.get::<&str, _>(1).unwrap_or("").to_string(),
                    categoria: row.get::<&str, _>(2).map(|s| s.to_string()),
                    frequencia_compra: row.get::<i32, _>(3).unwrap_or(0),
                    quantidade_media: row.get::<f64, _>(4).unwrap_or(0.0),
                    valor_medio: row.get::<f64, _>(5).unwrap_or(0.0),
                    dias_ultima_compra: row.get::<i32, _>(6).unwrap_or(0),
                    score_recompra: row.get::<f64, _>(7).unwrap_or(0.0),
                    nivel_prioridade: row.get::<&str, _>(8).unwrap_or("BAIXA").to_string(),
                    sugestao_inteligente: row.get::<&str, _>(9).unwrap_or("Produto disponível").to_string(),
                    produtos_relacionados: Vec::new(), // Será preenchido pelo helper
                });
            }
            _ => {}
        }
    }
    
    // Buscar produtos relacionados para cada produto
    let mut conn2 = match pools.sqlserver_portal.get().await {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Erro ao conectar no Portal para produtos relacionados: {}", e);
            return Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "cnpj": params.cnpj,
                "periodo_dias": periodo_dias,
                "produtos_recompra": produtos_recompra,
                "total_produtos": produtos_recompra.len(),
                "algoritmo": "score_baseado_em_frequencia_e_recencia",
                "note": "Produtos relacionados não carregados devido a erro de conexão"
            })));
        }
    };
    
    for produto in &mut produtos_recompra {
        match buscar_produtos_relacionados(
            &mut conn2, 
            &cnpj_formatado,
            &produto.codigo_produto,
            periodo_dias
        ).await {
            Ok(relacionados) => produto.produtos_relacionados = relacionados,
            Err(e) => {
                log::warn!("Erro ao buscar produtos relacionados para {}: {}", produto.codigo_produto, e);
            }
        }
    }
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "cnpj": cnpj_formatado,
        "cnpj_original": params.cnpj,
        "periodo_dias": periodo_dias,
        "produtos_recompra": produtos_recompra,
        "total_produtos": produtos_recompra.len(),
        "algoritmo": "score_baseado_em_frequencia_e_recencia"
    })))
}

/// Função auxiliar para buscar produtos relacionados (cross-selling)
async fn buscar_produtos_relacionados(
    conn: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    cnpj: &str,
    codigo_produto: &str,
    periodo_dias: i32,
) -> Result<Vec<ProdutoRelacionado>, ApiError> {
    
    let sql_relacionados = r#"
        WITH pedidos_com_produto AS (
            SELECT DISTINCT p.id as pedido_id
            FROM pedidos p
            INNER JOIN items i ON p.id = i.pedido_id
            INNER JOIN clientes c ON p.cliente_id = c.id
            WHERE c.cnpj = @P1
              AND i.codigo_produto = @P2
              AND p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
              AND p.created_at >= DATEADD(day, -@P3, GETDATE())
              AND c.deleted_at IS NULL
        )
        SELECT 
            i.codigo_produto,
            i.descricao_produto,
            COUNT(*) as vendas_conjuntas,
            (COUNT(*) * 100.0 / (SELECT COUNT(*) FROM pedidos_com_produto)) as correlacao_percentual
        FROM pedidos_com_produto pcp
        INNER JOIN items i ON pcp.pedido_id = i.pedido_id
        WHERE i.codigo_produto != @P2
        GROUP BY i.codigo_produto, i.descricao_produto
        HAVING COUNT(*) >= 2
        ORDER BY correlacao_percentual DESC
        OFFSET 0 ROWS FETCH NEXT 10 ROWS ONLY
    "#;
    
    let mut query_rel = Query::new(sql_relacionados);
    query_rel.bind(cnpj);
    query_rel.bind(codigo_produto);
    query_rel.bind(periodo_dias);
    
    let result_rel = query_rel.query(conn).await
        .map_err(|e| ApiError::Database(format!("Erro ao buscar produtos relacionados: {}", e)))?;
    
    let mut relacionados = Vec::new();
    let mut stream_rel = result_rel;
    
    while let Some(item) = stream_rel.try_next().await
        .map_err(|e| ApiError::Database(format!("Erro ao ler produtos relacionados: {}", e)))? {
        match item {
            QueryItem::Row(row) => {
                relacionados.push(ProdutoRelacionado {
                    codigo_produto: row.get::<&str, _>(0).unwrap_or("").to_string(),
                    descricao_produto: row.get::<&str, _>(1).unwrap_or("").to_string(),
                    vendas_conjuntas: row.get::<i32, _>(2).unwrap_or(0),
                    correlacao_percentual: row.get::<f64, _>(3).unwrap_or(0.0),
                });
            }
            _ => {}
        }
    }
    
    Ok(relacionados)
}
