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
    
    // 🎯 CARD 01: RECOMPRA INTELIGENTE - ALGORITMO DE IA PARA SUGESTÕES
    // Query otimizada baseada no documento oficial do Card 01
    let sql_recompra = r#"
        -- 🎯 CARD 01: RECOMPRA INTELIGENTE - ALGORITMO DE IA PARA SUGESTÕES
        -- Analisa padrão histórico de compras e sugere produtos para recompra
        -- Baseado em frequência de compra vs tempo desde última compra

        -- 🔍 CTE PRINCIPAL: ANÁLISE DE PADRÕES DE RECOMPRA
        WITH produtos_recompra AS (
            SELECT 
                -- 📦 IDENTIFICAÇÃO DO PRODUTO
                pr.codigo as codigo_produto,           -- Código único do produto
                pr.descricao as descricao_produto,     -- Nome completo
                cat.nome as categoria,                 -- Categoria
                
                -- 📊 MÉTRICAS DE COMPORTAMENTO DE COMPRA
                COUNT(DISTINCT p.id) as frequencia_compra,  -- Quantos pedidos diferentes compraram
                AVG(CAST(i.quantidade AS FLOAT)) as quantidade_media,  -- Quantidade média por pedido
                AVG(i.preco_unitario * i.quantidade) as valor_medio,  -- Valor médio gasto por pedido
                
                -- ⏰ ANÁLISE TEMPORAL
                DATEDIFF(day, MAX(p.created_at), GETDATE()) as dias_ultima_compra,  -- Há quantos dias foi a última compra
                
                -- 🎯 SCORE INTELIGENTE DE RECOMPRA (ALGORITMO PRINCIPAL)
                -- Fórmula: Quanto mais frequente E mais recente, maior o score
                (COUNT(DISTINCT p.id) * 10.0 / 
                 CASE WHEN DATEDIFF(day, MAX(p.created_at), GETDATE()) > 0 
                      THEN DATEDIFF(day, MAX(p.created_at), GETDATE()) 
                      ELSE 1 END) as score_recompra
                      
            FROM pedidos p
            INNER JOIN items i ON p.id = i.pedido_id      -- Itens do pedido
            INNER JOIN clientes c ON p.cliente_id = c.id  -- Dados do franqueado
            INNER JOIN produtos pr ON i.produto_id = pr.id -- Dados do produto
            LEFT JOIN categorias cat ON pr.categoria_id = cat.id -- Categoria (opcional)
            
            WHERE 
                -- 🎯 FILTROS DE QUALIDADE DOS DADOS
                c.cnpj = @P1                                           -- Apenas este franqueado
                AND p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')  -- Apenas pedidos reais/válidos
                AND p.created_at >= DATEADD(day, -@P2, GETDATE())         -- Período de análise
                AND c.deleted_at IS NULL                                 -- Cliente ativo
                AND pr.status = 1                                        -- Produto ativo/disponível
                
            GROUP BY pr.codigo, pr.descricao, cat.nome
            
            -- 🔍 FILTRO DE RELEVÂNCIA: Apenas produtos com histórico mínimo
            HAVING COUNT(DISTINCT p.id) >= 1                            -- Pelo menos 1 compra
        )

        -- 📋 RESULTADO FINAL: LISTA PRIORIZADA DE SUGESTÕES
        SELECT TOP (@P3)
            codigo_produto,
            descricao_produto,
            categoria,
            frequencia_compra,      -- Para mostrar: "Você já comprou 3 vezes"
            quantidade_media,       -- Para mostrar: "Média 12 unidades por pedido"
            valor_medio,           -- Para mostrar: "Investimento médio R$ 240"
            dias_ultima_compra,    -- Para mostrar: "Última compra há 45 dias"
            CAST(score_recompra AS DECIMAL(10,2)) as score_recompra,  -- Score formatado
            
            -- 📈 CLASSIFICAÇÃO DE PRIORIDADE (PARA UX/UI)
            CASE 
                WHEN score_recompra >= 3.0 THEN 'ALTA'      -- Compra frequente + recente
                WHEN score_recompra >= 1.0 THEN 'MÉDIA'     -- Padrão moderado
                ELSE 'BAIXA'                                 -- Compra esporádica
            END as nivel_prioridade,
            
            -- 💡 MENSAGEM PERSONALIZADA (PARA UX/UI)
            CASE 
                WHEN score_recompra >= 3.0 THEN 'Produto em reposição! Sugerimos incluir no próximo pedido.'
                WHEN score_recompra >= 1.0 THEN 'Padrão de recompra identificado. Considere reabastecer.'
                ELSE 'Produto já comprado anteriormente. Avalie necessidade atual.'
            END as sugestao_inteligente

        FROM produtos_recompra
        ORDER BY score_recompra DESC
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
                    nivel_prioridade: row.get::<&str, _>(8).unwrap_or("BAIXA").to_string(),
                    sugestao_inteligente: row.get::<&str, _>(9).unwrap_or("Produto disponível").to_string(),
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
    
    // 🏆 CARD 02: OPORTUNIDADES NA REDE 
    // Query otimizada baseada no documento oficial do Card 02
    // CORREÇÃO: Separadas as agregações aninhadas em CTEs independentes
    let sql_oportunidades = r#"
        -- 🏆 CARD 02: OPORTUNIDADES NA REDE 
        -- Compara performance do franqueado vs média do seu grupo ABC
        -- CORREÇÃO: Separadas as agregações aninhadas em CTEs independentes

        -- 🏅 ETAPA 1: CLASSIFICAÇÃO ABC DE TODOS OS FRANQUEADOS
        WITH volume_por_franqueado AS (
            -- Calcula volume total de cada franqueado no período
            SELECT 
                c.cnpj,
                c.nome as nome_franqueado,
                SUM(i.quantidade) as total_quantidade_periodo
                
            FROM pedidos p
            INNER JOIN items i ON p.id = i.pedido_id
            INNER JOIN clientes c ON p.cliente_id = c.id
            WHERE 
                p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
                AND p.created_at >= DATEADD(day, -@P2, GETDATE())
                AND c.deleted_at IS NULL
            GROUP BY c.cnpj, c.nome
        ),

        classificacao_abc AS (
            -- Classifica os franqueados em grupos A, B, C baseado no volume
            SELECT 
                cnpj,
                nome_franqueado,
                total_quantidade_periodo,
                
                -- 📊 ALGORITMO DE SEGMENTAÇÃO NTILE(3)
                NTILE(3) OVER (ORDER BY total_quantidade_periodo DESC) as percentil,
                
                -- 🎯 CLASSIFICAÇÃO ABC
                CASE 
                    WHEN NTILE(3) OVER (ORDER BY total_quantidade_periodo DESC) = 1 THEN 'A'  -- Top 33%
                    WHEN NTILE(3) OVER (ORDER BY total_quantidade_periodo DESC) = 2 THEN 'B'  -- Meio 33%
                    ELSE 'C'                                                                   -- Últimos 33%
                END as grupo_abc
                
            FROM volume_por_franqueado
        ),

        -- 🎯 PERFORMANCE DO FRANQUEADO ESPECÍFICO POR PRODUTO  
        performance_franqueado AS (
            SELECT 
                pr.codigo as codigo_produto,
                pr.descricao as descricao_produto,
                cat.nome as categoria,
                abc.grupo_abc,                                    -- Grupo ABC do franqueado
                SUM(i.quantidade) as quantidade_franqueado,       -- Total comprado pelo franqueado
                AVG(i.preco_unitario) as preco_medio_franqueado, -- Preço médio que ele paga
                COUNT(DISTINCT p.id) as pedidos_franqueado        -- Quantos pedidos ele fez
                
            FROM pedidos p
            INNER JOIN items i ON p.id = i.pedido_id
            INNER JOIN clientes c ON p.cliente_id = c.id
            INNER JOIN produtos pr ON i.produto_id = pr.id
            LEFT JOIN categorias cat ON pr.categoria_id = cat.id
            INNER JOIN classificacao_abc abc ON c.cnpj = abc.cnpj
            
            WHERE 
                c.cnpj = @P1                                    -- Apenas este franqueado
                AND p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
                AND p.created_at >= DATEADD(day, -@P2, GETDATE())
                AND c.deleted_at IS NULL
                AND pr.status = 1                                 -- Produtos ativos
                
            GROUP BY pr.codigo, pr.descricao, cat.nome, abc.grupo_abc
        ),

        -- 📊 ETAPA 2A: VOLUME POR FRANQUEADO E PRODUTO (para calcular média depois)
        volume_por_franqueado_produto AS (
            SELECT 
                pr.codigo as codigo_produto,
                c.cnpj,
                abc.grupo_abc,
                SUM(i.quantidade) as quantidade_franqueado_produto  -- Volume deste franqueado neste produto
            
            FROM pedidos p
            INNER JOIN items i ON p.id = i.pedido_id
            INNER JOIN clientes c ON p.cliente_id = c.id
            INNER JOIN produtos pr ON i.produto_id = pr.id
            INNER JOIN classificacao_abc abc ON c.cnpj = abc.cnpj
            
            WHERE 
                p.status_pedido IN ('integrado', 'Confirmado ERP', 'Faturado')
                AND p.created_at >= DATEADD(day, -@P2, GETDATE())
                AND c.deleted_at IS NULL
                AND pr.status = 1
                AND c.cnpj != @P1  -- 🎯 EXCLUIR o próprio franqueado da média
                
            GROUP BY pr.codigo, c.cnpj, abc.grupo_abc
        ),

        -- 📊 ETAPA 2B: MÉDIA DA REDE POR PRODUTO E GRUPO (sem agregação aninhada)
        media_rede_por_grupo AS (
            SELECT 
                codigo_produto,
                grupo_abc,
                
                -- ✅ CORREÇÃO: Média calculada diretamente, sem agregação aninhada
                AVG(CAST(quantidade_franqueado_produto AS FLOAT)) as media_quantidade_grupo,
                COUNT(DISTINCT cnpj) as franqueados_compraram,
                SUM(quantidade_franqueado_produto) as total_rede_produto
                
            FROM volume_por_franqueado_produto
            GROUP BY codigo_produto, grupo_abc
            HAVING COUNT(DISTINCT cnpj) >= 2  -- Pelo menos 2 franqueados compraram
        ),

        -- 🔍 IDENTIFICAÇÃO DE OPORTUNIDADES
        oportunidades_identificadas AS (
            SELECT 
                pf.codigo_produto,
                pf.descricao_produto,
                pf.categoria,
                pf.grupo_abc,
                
                -- 📊 DADOS DO FRANQUEADO
                pf.quantidade_franqueado,
                pf.preco_medio_franqueado,
                pf.pedidos_franqueado,
                
                -- 📊 DADOS DA REDE (BENCHMARK)
                mr.media_quantidade_grupo,
                mr.franqueados_compraram,
                
                -- 🎯 CÁLCULO DA DIFERENÇA PERCENTUAL
                -- Negativo = OPORTUNIDADE (está abaixo da média)
                CASE 
                    WHEN mr.media_quantidade_grupo > 0 
                    THEN ((pf.quantidade_franqueado - mr.media_quantidade_grupo) / mr.media_quantidade_grupo) * 100.0
                    ELSE 0
                END as diferenca_percentual,
                
                -- 💰 POTENCIAL ADICIONAL (Quantas unidades a mais poderia vender)
                CASE 
                    WHEN mr.media_quantidade_grupo > pf.quantidade_franqueado 
                    THEN mr.media_quantidade_grupo - pf.quantidade_franqueado
                    ELSE 0
                END as potencial_adicional_unidades,
                
                -- 💸 IMPACTO FINANCEIRO (Valor da oportunidade)
                CASE 
                    WHEN mr.media_quantidade_grupo > pf.quantidade_franqueado 
                    THEN (mr.media_quantidade_grupo - pf.quantidade_franqueado) * pf.preco_medio_franqueado
                    ELSE 0
                END as impacto_financeiro_estimado
                
            FROM performance_franqueado pf
            INNER JOIN media_rede_por_grupo mr ON pf.codigo_produto = mr.codigo_produto 
                                              AND pf.grupo_abc = mr.grupo_abc
            
            -- 🎯 FILTRO: Apenas onde há OPORTUNIDADE real (está abaixo da média)
            WHERE mr.media_quantidade_grupo > pf.quantidade_franqueado
        ),

        -- 🏆 CLASSIFICAÇÃO POR PRIORIDADE
        oportunidades_priorizadas AS (
            SELECT *,
                -- 🎯 ALGORITMO DE PRIORIZAÇÃO
                -- Combina: diferença percentual + impacto financeiro + popularidade
                (ABS(diferenca_percentual) * 0.5 +                              -- 50% peso: quão abaixo está
                 (impacto_financeiro_estimado / 100.0) * 0.3 +                  -- 30% peso: impacto financeiro
                 (CAST(franqueados_compraram AS FLOAT) / 81.0 * 100.0) * 0.2    -- 20% peso: popularidade na rede
                ) as score_prioridade,
                
                -- 📈 CLASSIFICAÇÃO DE PRIORIDADE  
                CASE 
                    WHEN ABS(diferenca_percentual) >= 50 AND impacto_financeiro_estimado >= 500 THEN 'ALTA'
                    WHEN ABS(diferenca_percentual) >= 30 OR impacto_financeiro_estimado >= 300 THEN 'MÉDIA'
                    ELSE 'BAIXA'
                END as nivel_prioridade
                
            FROM oportunidades_identificadas
        )

        -- 📋 RESULTADO FINAL: RELATÓRIO DE OPORTUNIDADES PRIORIZADAS
        SELECT TOP (@P3)
            -- 📦 IDENTIFICAÇÃO DO PRODUTO
            codigo_produto,
            descricao_produto,
            categoria,
            grupo_abc as seu_grupo,
            
            -- 📊 PERFORMANCE ATUAL vs BENCHMARK
            quantidade_franqueado as sua_quantidade,
            CAST(media_quantidade_grupo AS DECIMAL(10,1)) as media_do_grupo,
            CAST(diferenca_percentual AS DECIMAL(5,1)) as diferenca_percentual,
            
            -- 💰 OPORTUNIDADE FINANCEIRA
            CAST(potencial_adicional_unidades AS DECIMAL(10,1)) as unidades_adicionais,
            CAST(impacto_financeiro_estimado AS DECIMAL(10,2)) as oportunidade_reais,
            
            -- 🎯 ANÁLISE ESTRATÉGICA
            franqueados_compraram as outros_franqueados_compram,
            nivel_prioridade,
            CAST(score_prioridade AS DECIMAL(8,2)) as score_prioridade,
            
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
                WHEN nivel_prioridade = 'ALTA' THEN 'INCLUIR NO PRÓXIMO PEDIDO'
                WHEN nivel_prioridade = 'MÉDIA' THEN 'AVALIAR DEMANDA LOCAL'
                ELSE 'MONITORAR TENDÊNCIA'
            END as recomendacao

        FROM oportunidades_priorizadas
        WHERE potencial_adicional_unidades > 0  -- Apenas oportunidades reais
        ORDER BY score_prioridade DESC, impacto_financeiro_estimado DESC
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
        "cnpj": params.cnpj,
        "periodo_dias": periodo_dias,
        "oportunidades": oportunidades,
        "total_oportunidades": oportunidades.len(),
        "algoritmo": "comparacao_vs_media_grupo_abc_corrigido",
        "versao": "card_02_oficial"
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
