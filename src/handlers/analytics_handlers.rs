use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tiberius::{Query, QueryItem};
use futures_util::TryStreamExt;
use crate::database::DatabasePools;
use crate::errors::ApiError;
use crate::auth::Claims;

#[derive(Debug, Deserialize)]
pub struct AnalyticsParams {
    pub periodo: Option<String>, // 30d, 90d, 180d, 365d
    pub tipo: Option<String>,    // vendas, compras, completo
}

#[derive(Debug, Deserialize)]
pub struct RecompraParams {
    pub cnpj: String,
    pub periodo_dias: Option<i32>, // Padrão: 90 dias
    pub limite: Option<i32>,       // Padrão: 50 produtos
}

#[derive(Debug, Deserialize)]
pub struct OportunidadesRedeParams {
    pub cnpj: String,
    pub periodo_dias: Option<i32>, // Padrão: 90 dias
    pub limite: Option<i32>,       // Padrão: 50 oportunidades
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
    pub produtos_relacionados: Vec<ProdutoRelacionado>,
}

#[derive(Debug, Serialize)]
pub struct ProdutoRelacionado {
    pub codigo_produto: String,
    pub descricao_produto: String,
    pub correlacao_percentual: f64,
    pub vendas_conjuntas: i32,
}

#[derive(Debug, Serialize)]
pub struct OportunidadeRede {
    pub codigo_produto: String,
    pub descricao_produto: String,
    pub categoria: Option<String>,
    pub media_franqueado: f64,
    pub media_rede: f64,
    pub diferenca_percentual: f64,
    pub potencial_adicional: f64,
    pub grupo_abc: String,
    pub prioridade: String,
}

/// Card 01: Análise de Recompra Inteligente
/// Analisa histórico de pedidos do franqueado para sugerir recompras
pub async fn recompra_inteligente(
    params: web::Query<RecompraParams>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    log::info!("Análise de recompra inteligente para CNPJ: {}", params.cnpj);
    
    let periodo_dias = params.periodo_dias.unwrap_or(90);
    let limite = params.limite.unwrap_or(50);
    
    // 1. Buscar padrões de recompra do franqueado específico
    let mut conn = match pools.sqlserver_portal.get().await {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Erro ao conectar no Portal: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Erro de conexão com Portal",
                "error": e.to_string()
            })));
        }
    };
    
    // Query corrigida para SQL Server Portal (SELL-IN)
    let sql_recompra = r#"
        SELECT 
            i.codigo_produto,
            i.descricao_produto,
            cat.nome as categoria,
            COUNT(DISTINCT p.id) as frequencia_compra,
            AVG(CAST(i.quantidade AS FLOAT)) as quantidade_media,
            AVG(i.preco_unitario * i.quantidade) as valor_medio,
            DATEDIFF(day, MAX(p.created_at), GETDATE()) as dias_ultima_compra,
            -- Score de recompra baseado em frequência e recência
            (COUNT(DISTINCT p.id) * 10.0 / DATEDIFF(day, MAX(p.created_at), GETDATE())) as score_recompra
        FROM pedidos p
        INNER JOIN items i ON p.id = i.pedido_id
        INNER JOIN clientes c ON p.cliente_id = c.id
        INNER JOIN produtos pr ON i.produto_id = pr.id
        LEFT JOIN categorias cat ON pr.categoria_id = cat.id
        WHERE c.cnpj = @P1
          AND p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
          AND p.created_at >= DATEADD(day, -@P2, GETDATE())
          AND c.deleted_at IS NULL
        GROUP BY i.codigo_produto, i.descricao_produto, cat.nome
        HAVING COUNT(DISTINCT p.id) >= 2  -- Pelo menos 2 compras
        ORDER BY score_recompra DESC
        OFFSET 0 ROWS FETCH NEXT @P3 ROWS ONLY
    "#;
    
    let mut query = Query::new(sql_recompra);
    query.bind(&params.cnpj);
    query.bind(periodo_dias);
    query.bind(limite);
    
    let result = query.query(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro ao buscar dados recompra: {}", e)))?;
    
    let mut produtos_recompra = Vec::new();
    let mut stream = result;
    
    while let Some(item) = stream.try_next().await
        .map_err(|e| ApiError::Database(format!("Erro ao ler dados recompra: {}", e)))? {
        match item {
            QueryItem::Row(row) => {
                let codigo_produto = row.get::<&str, _>(0).unwrap_or("").to_string();
                
                produtos_recompra.push(ProdutoRecompra {
                    codigo_produto: codigo_produto.clone(),
                    descricao_produto: row.get::<&str, _>(1).unwrap_or("").to_string(),
                    categoria: row.get::<&str, _>(2).map(|s| s.to_string()),
                    frequencia_compra: row.get::<i32, _>(3).unwrap_or(0),
                    quantidade_media: row.get::<f64, _>(4).unwrap_or(0.0),
                    valor_medio: row.get::<f64, _>(5).unwrap_or(0.0),
                    dias_ultima_compra: row.get::<i32, _>(6).unwrap_or(0),
                    score_recompra: row.get::<f64, _>(7).unwrap_or(0.0),
                    produtos_relacionados: Vec::new(), // Será preenchido na próxima query
                });
            }
            _ => {}
        }
    }
    
    // 2. Para cada produto, buscar produtos frequentemente comprados juntos
    // CORREÇÃO: Criar nova conexão para evitar conflito de borrow
    let mut conn2 = match pools.sqlserver_portal.get().await {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Erro ao conectar no Portal para produtos relacionados: {}", e);
            // Continuar sem produtos relacionados se houver erro
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
            &params.cnpj, 
            &produto.codigo_produto,
            periodo_dias
        ).await {
            Ok(relacionados) => produto.produtos_relacionados = relacionados,
            Err(e) => {
                log::warn!("Erro ao buscar produtos relacionados para {}: {}", produto.codigo_produto, e);
                // Continuar com lista vazia de produtos relacionados
            }
        }
    }
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "cnpj": params.cnpj,
        "periodo_dias": periodo_dias,
        "produtos_recompra": produtos_recompra,
        "total_produtos": produtos_recompra.len(),
        "algoritmo": "score_baseado_em_frequencia_e_recencia"
    })))
}

/// Card 02: Oportunidades na Rede (Ranking Comparativo)
/// Compara performance do franqueado vs média da rede
pub async fn oportunidades_rede(
    params: web::Query<OportunidadesRedeParams>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    log::info!("Análise de oportunidades na rede para CNPJ: {}", params.cnpj);
    
    let periodo_dias = params.periodo_dias.unwrap_or(90);
    let limite = params.limite.unwrap_or(50);
    
    // 1. Primeira conexão para classificação ABC
    let mut conn1 = match pools.sqlserver_portal.get().await {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Erro ao conectar no Portal: {}", e);
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
    
    let mut query_class = Query::new(sql_classificacao);
    query_class.bind(periodo_dias);
    query_class.bind(&params.cnpj);
    
    let result_class = query_class.query(&mut conn1).await
        .map_err(|e| ApiError::Database(format!("Erro ao classificar franqueado: {}", e)))?;
    
    let mut grupo_abc = "C".to_string();
    let mut stream_class = result_class;
    
    // Consumir stream de classificação
    while let Some(item) = stream_class.try_next().await
        .map_err(|e| ApiError::Database(format!("Erro ao ler classificação: {}", e)))? {
        match item {
            QueryItem::Row(row) => {
                grupo_abc = row.get::<&str, _>(2).unwrap_or("C").to_string();
                break;
            }
            _ => {}
        }
    }
    
    // 2. Segunda conexão para buscar oportunidades
    let mut conn2 = match pools.sqlserver_portal.get().await {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Erro ao conectar no Portal para oportunidades: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Erro de conexão com Portal para oportunidades", 
                "error": e.to_string()
            })));
        }
    };
    
    // Query para buscar oportunidades comparando com média do grupo ABC
    let sql_oportunidades = r#"
        WITH media_franqueado AS (
            SELECT 
                i.codigo_produto,
                i.descricao_produto,
                cat.nome as categoria,
                AVG(CAST(i.quantidade AS FLOAT)) as media_franqueado,
                SUM(i.quantidade) as total_franqueado
            FROM pedidos p
            INNER JOIN items i ON p.id = i.pedido_id
            INNER JOIN clientes c ON p.cliente_id = c.id
            INNER JOIN produtos pr ON i.produto_id = pr.id
            LEFT JOIN categorias cat ON pr.categoria_id = cat.id
            WHERE c.cnpj = @P1
              AND p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
              AND p.created_at >= DATEADD(day, -@P2, GETDATE())
              AND c.deleted_at IS NULL
            GROUP BY i.codigo_produto, i.descricao_produto, cat.nome
        ),
        classificacao_abc AS (
            SELECT 
                c.cnpj,
                CASE 
                    WHEN NTILE(3) OVER (ORDER BY SUM(i.quantidade) DESC) = 1 THEN 'A'
                    WHEN NTILE(3) OVER (ORDER BY SUM(i.quantidade) DESC) = 2 THEN 'B'
                    ELSE 'C'
                END as grupo_abc
            FROM pedidos p
            INNER JOIN items i ON p.id = i.pedido_id
            INNER JOIN clientes c ON p.cliente_id = c.id
            WHERE p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
              AND p.created_at >= DATEADD(day, -@P2, GETDATE())
              AND c.deleted_at IS NULL
            GROUP BY c.cnpj
        ),
        media_rede AS (
            SELECT 
                i.codigo_produto,
                AVG(CAST(i.quantidade AS FLOAT)) as media_rede,
                COUNT(DISTINCT c.cnpj) as franqueados_grupo
            FROM pedidos p
            INNER JOIN items i ON p.id = i.pedido_id
            INNER JOIN clientes c ON p.cliente_id = c.id
            INNER JOIN classificacao_abc abc ON c.cnpj = abc.cnpj
            WHERE abc.grupo_abc = @P3
              AND p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
              AND p.created_at >= DATEADD(day, -@P2, GETDATE())
              AND c.deleted_at IS NULL
            GROUP BY i.codigo_produto
            HAVING COUNT(DISTINCT c.cnpj) >= 3  -- Pelo menos 3 franqueados no grupo
        )
        SELECT 
            mf.codigo_produto,
            mf.descricao_produto,
            mf.categoria,
            mf.media_franqueado,
            ISNULL(mr.media_rede, 0) as media_rede,
            CASE 
                WHEN mr.media_rede > 0 THEN 
                    ((mf.media_franqueado - mr.media_rede) / mr.media_rede) * 100
                ELSE 0
            END as diferenca_percentual,
            CASE 
                WHEN mr.media_rede > mf.media_franqueado THEN 
                    mr.media_rede - mf.media_franqueado
                ELSE 0
            END as potencial_adicional
        FROM media_franqueado mf
        LEFT JOIN media_rede mr ON mf.codigo_produto = mr.codigo_produto
        WHERE mr.media_rede > mf.media_franqueado  -- Apenas oportunidades
        ORDER BY potencial_adicional DESC
        OFFSET 0 ROWS FETCH NEXT @P4 ROWS ONLY
    "#;
    
    let mut query_oport = Query::new(sql_oportunidades);
    query_oport.bind(&params.cnpj);
    query_oport.bind(periodo_dias);
    query_oport.bind(&grupo_abc);
    query_oport.bind(limite);
    
    let result_oport = query_oport.query(&mut conn2).await
        .map_err(|e| ApiError::Database(format!("Erro ao buscar oportunidades: {}", e)))?;
    
    let mut oportunidades = Vec::new();
    let mut stream_oport = result_oport;
    
    while let Some(item) = stream_oport.try_next().await
        .map_err(|e| ApiError::Database(format!("Erro ao ler oportunidades: {}", e)))? {
        match item {
            QueryItem::Row(row) => {
                let diferenca_percentual = row.get::<f64, _>(5).unwrap_or(0.0);
                let potencial = row.get::<f64, _>(6).unwrap_or(0.0);
                
                let prioridade = if diferenca_percentual <= -50.0 || potencial >= 100.0 {
                    "alta"
                } else if diferenca_percentual <= -30.0 || potencial >= 50.0 {
                    "media"
                } else {
                    "baixa"
                };
                
                oportunidades.push(OportunidadeRede {
                    codigo_produto: row.get::<&str, _>(0).unwrap_or("").to_string(),
                    descricao_produto: row.get::<&str, _>(1).unwrap_or("").to_string(),
                    categoria: row.get::<&str, _>(2).map(|s| s.to_string()),
                    media_franqueado: row.get::<f64, _>(3).unwrap_or(0.0),
                    media_rede: row.get::<f64, _>(4).unwrap_or(0.0),
                    diferenca_percentual,
                    potencial_adicional: potencial,
                    grupo_abc: grupo_abc.clone(),
                    prioridade: prioridade.to_string(),
                });
            }
            _ => {}
        }
    }
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "cnpj": params.cnpj,
        "grupo_abc": grupo_abc,
        "periodo_dias": periodo_dias,
        "oportunidades": oportunidades,
        "total_oportunidades": oportunidades.len(),
        "algoritmo": "comparacao_vs_media_grupo_abc"
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
        WHERE i.codigo_produto != @P2  -- Excluir o produto principal
        GROUP BY i.codigo_produto, i.descricao_produto
        HAVING COUNT(*) >= 2  -- Pelo menos 2 ocorrências
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

/// Analytics 360° do cliente (mantido para compatibilidade)
pub async fn analytics_cliente_360(
    cnpj: web::Path<String>,
    params: web::Query<AnalyticsParams>,
    _pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    log::info!("Gerando analytics 360° para CNPJ: {}", cnpj);
    
    let periodo = params.periodo.as_deref().unwrap_or("30d");
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "cnpj": cnpj.into_inner(),
        "periodo": periodo,
        "analytics": {
            "vendas_fc": {
                "total": 0,
                "quantidade_vendas": 0,
                "ticket_medio": 0
            },
            "compras_portal": {
                "total": 0,
                "quantidade_pedidos": 0,
                "ticket_medio": 0
            },
            "estoque_protheus": {
                "valor_total": 0,
                "giro_estoque": 0
            },
            "oportunidades": [],
            "insights": {
                "status": "Usando novos endpoints: /recompra-inteligente e /oportunidades-rede",
                "mensagem": "Analytics específicos implementados"
            }
        }
    })))
}

/// Correlações de produtos (mantido para compatibilidade)
pub async fn correlacoes_produto(
    produto_id: web::Path<i32>,
    _pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    log::info!("Buscando correlações para produto ID: {}", produto_id);
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "produto_id": produto_id.into_inner(),
        "correlacoes": [],
        "mensagem": "Use o endpoint /recompra-inteligente para análises de correlação"
    })))
}
