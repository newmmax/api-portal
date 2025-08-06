//! 🏆 Card 02: Oportunidades na Rede  
//!
//! Compara performance do franqueado vs média da rede do mesmo grupo ABC
//! para identificar oportunidades de crescimento.

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
pub struct OportunidadesRedeParams {
    pub cnpj: String,
    pub periodo_dias: Option<i32>, // Padrão: 90 dias
    pub limite: Option<i32>,       // Padrão: 50 oportunidades
}

#[derive(Debug, Serialize)]
pub struct OportunidadeRede {
    pub codigo_produto: String,
    pub descricao_produto: String,
    pub categoria: Option<String>,
    pub seu_grupo: String,                    // Grupo ABC do franqueado
    pub sua_quantidade: f64,                  // Quantidade atual do franqueado
    pub media_do_grupo: f64,                  // Média do grupo ABC
    pub diferenca_percentual: f64,            // % diferença vs média
    pub unidades_adicionais: f64,             // Potencial em unidades
    pub oportunidade_reais: f64,              // Impacto financeiro estimado
    pub outros_franqueados_compram: i32,      // Quantos outros compram
    pub nivel_prioridade: String,             // ALTA, MÉDIA, BAIXA
    pub score_prioridade: f64,                // Score de priorização
    pub insight: String,                      // Insight personalizado
    pub recomendacao: String,                 // Recomendação de ação
}

/// Card 02: Oportunidades na Rede (Ranking Comparativo)
/// Compara performance do franqueado vs média da rede
pub async fn oportunidades_rede(
    params: web::Query<OportunidadesRedeParams>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    let _start_time = Instant::now();
    let card_name = "oportunidades-rede";
    
    cards_log!(request, card_name, &params.cnpj, 
               &format!("periodo_dias={:?}, limite={:?}", params.periodo_dias, params.limite));

    log::info!("Análise de oportunidades na rede para CNPJ: {}", params.cnpj);
    
    let periodo_dias = params.periodo_dias.unwrap_or(90);
    let limite = params.limite.unwrap_or(50);
    
    // 1. Primeira conexão para classificação ABC
    let mut conn1 = match pools.sqlserver_portal.get().await {
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
    
    // Query para classificar o franqueado em grupo ABC
    let sql_classificacao = r#"
        WITH ranking_franqueados AS (
            SELECT 
                c.cnpj,
                SUM(i.quantidade) as total_quantidade,
                NTILE(3) OVER (ORDER BY SUM(i.quantidade) DESC) as grupo_percentil
            FROM pedidos p
            INNER JOIN items i ON p.id = i.pedido_id
            INNER JOIN clientes c ON p.cliente_id = c.id
            WHERE p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
              AND p.created_at >= DATEADD(day, -@P1, GETDATE())
              AND c.deleted_at IS NULL
            GROUP BY c.cnpj
        )
        SELECT 
            cnpj,
            total_quantidade,
            CASE 
                WHEN grupo_percentil = 1 THEN 'A'
                WHEN grupo_percentil = 2 THEN 'B'
                ELSE 'C'
            END as grupo_abc
        FROM ranking_franqueados
        WHERE cnpj = @P2
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

    cards_log!(cnpj, &params.cnpj, &cnpj_formatado);

    let mut query_class = Query::new(sql_classificacao);
    query_class.bind(periodo_dias);
    query_class.bind(&cnpj_formatado);
    
    let result_class = query_class.query(&mut conn1).await
        .map_err(|e| {
            let error_msg = format!("Erro ao classificar franqueado: {}", e);
            cards_log!(error, card_name, &cnpj_formatado, &error_msg, "sql_classificacao");
            ApiError::Database(error_msg)
        })?;
    
    let mut grupo_abc = "C".to_string();
    let mut stream_class = result_class;
    
    // Consumir stream de classificação
    while let Some(item) = stream_class.try_next().await
        .map_err(|e| {
            let error_msg = format!("Erro ao ler classificação: {}", e);
            cards_log!(error, card_name, &cnpj_formatado, &error_msg, "stream_classificacao");
            ApiError::Database(error_msg)
        })? {
        match item {
            QueryItem::Row(row) => {
                grupo_abc = row.get::<&str, _>(2).unwrap_or("C").to_string();
                log::info!("Card 02 - Franqueado {} classificado como grupo: {}", cnpj_formatado, grupo_abc);
                break;
            }
            _ => {}
        }
    }
    
    log::info!("Card 02 - Usando grupo ABC calculado: {} para CNPJ: {}", grupo_abc, cnpj_formatado);
    
    // 2. Segunda conexão para buscar oportunidades
    let mut conn2 = match pools.sqlserver_portal.get().await {
        Ok(conn) => conn,
        Err(e) => {
            let error_msg = format!("Erro ao conectar no Portal para oportunidades: {}", e);
            cards_log!(error, card_name, &cnpj_formatado, &error_msg, "database_connection_2");
            log::error!("{}", error_msg);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Erro de conexão com Portal para oportunidades", 
                "error": e.to_string()
            })));
        }
    };
    
    // Query complexa do Card 02
    let sql_oportunidades = format!(r#"
        -- 🏆 CARD 02: OPORTUNIDADES NA REDE - VERSÃO CORRIGIDA
        -- Usa grupo ABC pré-calculado: {}
        
        -- 🎯 PERFORMANCE DO FRANQUEADO ESPECÍFICO POR PRODUTO  
        WITH performance_franqueado AS (
            SELECT 
                pr.codigo as codigo_produto,
                pr.descricao as descricao_produto,
                cat.nome as categoria,
                '{}' as grupo_abc,
                SUM(i.quantidade) as quantidade_franqueado,
                AVG(i.preco_unitario) as preco_medio_franqueado,
                COUNT(DISTINCT p.id) as pedidos_franqueado
                
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
        ),

        -- 📊 MÉDIA DA REDE PARA O GRUPO ABC ESPECÍFICO
        media_rede_por_grupo AS (
            SELECT 
                pr.codigo as codigo_produto,
                AVG(CAST(pedido_totals.quantidade_produto AS FLOAT)) as media_quantidade_grupo,
                COUNT(DISTINCT c.cnpj) as franqueados_compraram
                
            FROM (
                SELECT 
                    c.cnpj,
                    pr.id as produto_id,
                    SUM(i.quantidade) as quantidade_produto,
                    SUM(i.quantidade * i.preco_unitario) as valor_total_produto
                FROM pedidos p
                INNER JOIN items i ON p.id = i.pedido_id
                INNER JOIN clientes c ON p.cliente_id = c.id
                INNER JOIN produtos pr ON i.produto_id = pr.id
                WHERE 
                    p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
                    AND p.created_at >= DATEADD(day, -@P2, GETDATE())
                    AND c.deleted_at IS NULL
                    AND pr.status = 1
                    AND c.cnpj != @P1
                GROUP BY c.cnpj, pr.id
                HAVING SUM(i.quantidade * i.preco_unitario) > 0
            ) as pedido_totals
            INNER JOIN produtos pr ON pedido_totals.produto_id = pr.id
            
            -- 🎯 FILTRO POR GRUPO ABC: Apenas franqueados do mesmo grupo
            INNER JOIN (
                SELECT c.cnpj
                FROM pedidos p
                INNER JOIN items i ON p.id = i.pedido_id
                INNER JOIN clientes c ON p.cliente_id = c.id
                WHERE p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
                  AND p.created_at >= DATEADD(day, -@P2, GETDATE())
                  AND c.deleted_at IS NULL
                GROUP BY c.cnpj
                HAVING CASE 
                    WHEN NTILE(3) OVER (ORDER BY SUM(i.quantidade) DESC) = 1 THEN 'A'
                    WHEN NTILE(3) OVER (ORDER BY SUM(i.quantidade) DESC) = 2 THEN 'B'
                    ELSE 'C'
                END = '{}'
            ) grupos_abc ON pedido_totals.cnpj = grupos_abc.cnpj
            
            GROUP BY pr.codigo
            HAVING COUNT(DISTINCT pedido_totals.cnpj) >= 2
        ),

        -- 🔍 IDENTIFICAÇÃO DE OPORTUNIDADES
        oportunidades_identificadas AS (
            SELECT 
                pf.codigo_produto,
                pf.descricao_produto,
                pf.categoria,
                pf.grupo_abc,
                pf.quantidade_franqueado,
                pf.preco_medio_franqueado,
                mr.media_quantidade_grupo,
                mr.franqueados_compraram,
                
                -- 🎯 CÁLCULO DA DIFERENÇA PERCENTUAL
                CASE 
                    WHEN mr.media_quantidade_grupo > 0 
                    THEN ((pf.quantidade_franqueado - mr.media_quantidade_grupo) / mr.media_quantidade_grupo) * 100.0
                    ELSE 0
                END as diferenca_percentual,
                
                -- 💰 POTENCIAL ADICIONAL
                CASE 
                    WHEN mr.media_quantidade_grupo > pf.quantidade_franqueado 
                    THEN mr.media_quantidade_grupo - pf.quantidade_franqueado
                    ELSE 0
                END as potencial_adicional_unidades,
                
                -- 💸 IMPACTO FINANCEIRO
                CASE 
                    WHEN mr.media_quantidade_grupo > pf.quantidade_franqueado 
                    THEN (mr.media_quantidade_grupo - pf.quantidade_franqueado) * pf.preco_medio_franqueado
                    ELSE 0
                END as impacto_financeiro_estimado
                
            FROM performance_franqueado pf
            INNER JOIN media_rede_por_grupo mr ON pf.codigo_produto = mr.codigo_produto
            WHERE mr.media_quantidade_grupo > pf.quantidade_franqueado
        )

        SELECT TOP (@P3)
            codigo_produto,
            descricao_produto,
            categoria,
            grupo_abc as seu_grupo,
            quantidade_franqueado as sua_quantidade,
            CAST(media_quantidade_grupo AS DECIMAL(10,1)) as media_do_grupo,
            CAST(diferenca_percentual AS DECIMAL(5,1)) as diferenca_percentual,
            CAST(potencial_adicional_unidades AS DECIMAL(10,1)) as unidades_adicionais,
            CAST(impacto_financeiro_estimado AS DECIMAL(10,2)) as oportunidade_reais,
            franqueados_compraram as outros_franqueados_compram,
            
            -- 📈 CLASSIFICAÇÃO DE PRIORIDADE  
            CASE 
                WHEN ABS(diferenca_percentual) >= 50 AND impacto_financeiro_estimado >= 500 THEN 'ALTA'
                WHEN ABS(diferenca_percentual) >= 30 OR impacto_financeiro_estimado >= 300 THEN 'MÉDIA'
                ELSE 'BAIXA'
            END as nivel_prioridade,
            
            -- 🎯 ALGORITMO DE PRIORIZAÇÃO
            (ABS(diferenca_percentual) * 0.5 + 
             (impacto_financeiro_estimado / 100.0) * 0.3 + 
             (CAST(franqueados_compraram AS FLOAT) / 81.0 * 100.0) * 0.2
            ) as score_prioridade,
            
            -- 💡 INSIGHT PERSONALIZADO
            CASE 
                WHEN ABS(diferenca_percentual) >= 50 THEN 
                    'GRANDE OPORTUNIDADE: Você está ' + CAST(CAST(ABS(diferenca_percentual) AS INT) AS NVARCHAR) + '% abaixo da média!'
                WHEN ABS(diferenca_percentual) >= 30 THEN 
                    'Oportunidade identificada: +' + CAST(CAST(potencial_adicional_unidades AS INT) AS NVARCHAR) + ' unidades por período'
                ELSE 
                    'Pequena oportunidade de otimização'
            END as insight,
            
            -- 📈 RECOMENDAÇÃO DE AÇÃO
            CASE 
                WHEN ABS(diferenca_percentual) >= 50 AND impacto_financeiro_estimado >= 500 THEN 'INCLUIR NO PRÓXIMO PEDIDO'
                WHEN ABS(diferenca_percentual) >= 30 OR impacto_financeiro_estimado >= 300 THEN 'AVALIAR DEMANDA LOCAL'
                ELSE 'MONITORAR TENDÊNCIA'
            END as recomendacao

        FROM oportunidades_identificadas
        WHERE potencial_adicional_unidades > 0
        ORDER BY (ABS(diferenca_percentual) * 0.5 + 
                 (impacto_financeiro_estimado / 100.0) * 0.3 + 
                 (CAST(franqueados_compraram AS FLOAT) / 81.0 * 100.0) * 0.2
                ) DESC, impacto_financeiro_estimado DESC
    "#, grupo_abc, grupo_abc, grupo_abc);
    
    let mut query_oport = Query::new(sql_oportunidades);
    query_oport.bind(&cnpj_formatado);
    query_oport.bind(periodo_dias);
    query_oport.bind(limite);
    
    let result_oport = query_oport.query(&mut conn2).await
        .map_err(|e| ApiError::Database(format!("Erro ao buscar oportunidades: {}", e)))?;
    
    let mut oportunidades = Vec::new();
    let mut stream_oport = result_oport;
    
    while let Some(item) = stream_oport.try_next().await
        .map_err(|e| ApiError::Database(format!("Erro ao ler oportunidades: {}", e)))? {
        match item {
            QueryItem::Row(row) => {
                oportunidades.push(OportunidadeRede {
                    codigo_produto: row.get::<&str, _>(0).unwrap_or("").to_string(),
                    descricao_produto: row.get::<&str, _>(1).unwrap_or("").to_string(),
                    categoria: row.get::<&str, _>(2).map(|s| s.to_string()),
                    seu_grupo: row.get::<&str, _>(3).unwrap_or("C").to_string(),
                    sua_quantidade: row.get::<f64, _>(4).unwrap_or(0.0),
                    media_do_grupo: row.get::<f64, _>(5).unwrap_or(0.0),
                    diferenca_percentual: row.get::<f64, _>(6).unwrap_or(0.0),
                    unidades_adicionais: row.get::<f64, _>(7).unwrap_or(0.0),
                    oportunidade_reais: row.get::<f64, _>(8).unwrap_or(0.0),
                    outros_franqueados_compram: row.get::<i32, _>(9).unwrap_or(0),
                    nivel_prioridade: row.get::<&str, _>(10).unwrap_or("BAIXA").to_string(),
                    score_prioridade: row.get::<f64, _>(11).unwrap_or(0.0),
                    insight: row.get::<&str, _>(12).unwrap_or("").to_string(),
                    recomendacao: row.get::<&str, _>(13).unwrap_or("").to_string(),
                });
            }
            _ => {}
        }
    }
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "cnpj": cnpj_formatado,
        "cnpj_original": params.cnpj,
        "periodo_dias": periodo_dias,
        "oportunidades": oportunidades,
        "total_oportunidades": oportunidades.len(),
        "algoritmo": "comparacao_vs_media_grupo_abc_corrigido",
        "versao": "card_02_oficial"
    })))
}
