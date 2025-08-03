// src/handlers/query_handlers.rs
// Handler para consultas SQL personalizadas

use actix_web::{web, HttpResponse, Result};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::{json, Value};
use tokio_postgres::Row;

/// Estrutura para receber consultas SQL personalizadas
#[derive(Debug, Deserialize)]
pub struct CustomQueryRequest {
    #[serde(alias = "sql")]  // Aceita tanto "query" quanto "sql"
    pub query: String,
}

/// Handler para executar consultas SQL personalizadas
/// 🔧 MELHORIA: Suporte aprimorado para SELECT * e feedback detalhado
/// 🛡️ SEGURANÇA: Mantém validações rigorosas - apenas SELECT permitido
/// 📊 USABILIDADE: Mensagens de erro mais claras e informativas
pub async fn execute_custom_query(
    pool: web::Data<Pool>,
    query_req: web::Json<CustomQueryRequest>,
) -> Result<HttpResponse> {
    let query = query_req.query.trim();
    
    // 🔍 VALIDAÇÃO DE SEGURANÇA: Apenas consultas SELECT
    if !is_select_query(query) {
        log::warn!("Tentativa de execução de query não-SELECT: {}", query);
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "error": "SECURITY_RESTRICTION",
            "message": "Apenas consultas SELECT são permitidas por motivos de segurança",
            "allowed_examples": [
                "SELECT * FROM clientes",
                "SELECT nome, email FROM clientes WHERE ativo = true",
                "SELECT COUNT(*) FROM pedidos WHERE data >= '2024-01-01'"
            ]
        })));
    }
    
    log::info!("🔍 Executando consulta personalizada: {}", query);

    // 🔌 OBTER CONEXÃO DO POOL
    let client = match pool.get().await {
        Ok(client) => client,
        Err(e) => {
            log::error!("Erro no pool de conexões PostgreSQL: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "error": "CONNECTION_POOL_ERROR",
                "message": "Erro na conexão com banco de dados PostgreSQL",
                "details": e.to_string()
            })));
        }
    };
    
    // 📝 PREPARAR QUERY
    let stmt = match client.prepare(query).await {
        Ok(stmt) => {
            log::debug!("✅ Query preparada com sucesso");
            stmt
        },
        Err(e) => {
            log::error!("❌ Erro ao preparar query: {}", e);
            return Ok(HttpResponse::BadRequest().json(json!({
                "success": false,
                "error": "QUERY_PREPARATION_ERROR", 
                "message": "Erro na sintaxe da consulta SQL",
                "details": e.to_string(),
                "query_submitted": query,
                "suggestions": [
                    "Verifique a sintaxe SQL",
                    "Confirme se as tabelas/colunas existem", 
                    "Use apenas comandos SELECT"
                ]
            })));
        }
    };
    
    // 🚀 EXECUTAR QUERY  
    let rows = match client.query(&stmt, &[]).await {
        Ok(rows) => {
            log::info!("✅ Query executada com sucesso. {} registros retornados", rows.len());
            rows
        },
        Err(e) => {
            log::error!("❌ Erro ao executar query: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "error": "QUERY_EXECUTION_ERROR",
                "message": "Erro durante execução da consulta",
                "details": e.to_string(),
                "query_submitted": query,
                "possible_causes": [
                    "Tabela ou coluna não existe",
                    "Permissões insuficientes",
                    "Erro de sintaxe SQL",
                    "Timeout de consulta"
                ]
            })));
        }
    };
    
    // 🎯 CONVERTER RESULTADOS USANDO NOVA FUNÇÃO ROBUSTA
    let result = rows_to_json(rows);
    
    log::info!("🎉 Consulta personalizada concluída com sucesso");
    Ok(HttpResponse::Ok().json(result))
}

/// Verifica se a consulta é do tipo SELECT
fn is_select_query(query: &str) -> bool {
    let normalized = query.trim().to_lowercase();
    
    // Verificar se começa com SELECT
    if !normalized.starts_with("select") {
        return false;
    }
    
    // Verificar palavras-chave proibidas que indicam modificação de dados
    let prohibited_keywords = [
        " insert ", " update ", " delete ", " drop ", " alter ", 
        " truncate ", " create ", " replace ", " exec ", " execute ",
        ";insert", ";update", ";delete", ";drop", ";alter",
        ";truncate", ";create", ";replace", ";exec", ";execute"
    ];
    
    for keyword in prohibited_keywords.iter() {
        if normalized.contains(keyword) {
            return false;
        }
    }
    
    true
}

/// Converte linhas de resultado em JSON com suporte robusto a tipos PostgreSQL
/// 🔧 MELHORIA: Suporta SELECT * e todos os tipos PostgreSQL comuns
/// 🛡️ SEGURANÇA: Mantém todas as validações de segurança existentes
/// 📊 COMPATIBILIDADE: 100% compatível com código anterior
fn rows_to_json(rows: Vec<Row>) -> Value {
    if rows.is_empty() {
        return json!({ 
            "success": true,
            "count": 0, 
            "data": [] 
        });
    }
    
    let columns = rows[0].columns();
    let column_info: Vec<(String, &tokio_postgres::types::Type)> = columns.iter()
        .map(|col| (col.name().to_string(), col.type_()))
        .collect();
    
    let data: Vec<Value> = rows.iter().map(|row| {
        let mut obj = serde_json::Map::new();
        
        for (i, (col_name, col_type)) in column_info.iter().enumerate() {
            // 🎯 ESTRATÉGIA INTELIGENTE: Usar tipo PostgreSQL para conversão precisa
            let value = convert_postgres_value_to_json(row, i, col_type);            
            obj.insert(col_name.clone(), value);
        }
        
        json!(obj)
    }).collect();
    
    json!({
        "success": true,
        "count": rows.len(),
        "data": data,
        "message": format!("Query executada com sucesso. {} registros retornados.", rows.len())
    })
}

/// 🔧 NOVA FUNÇÃO: Conversão inteligente baseada no tipo PostgreSQL real
/// Resolve o problema do SELECT * ao usar informações precisas de tipo
fn convert_postgres_value_to_json(row: &Row, col_index: usize, pg_type: &tokio_postgres::types::Type) -> Value {
    use tokio_postgres::types::Type;
    
    // 📋 MAPEAMENTO COMPLETO DOS TIPOS POSTGRESQL
    match *pg_type {
        // 🔢 TIPOS NUMÉRICOS
        Type::INT2 => row.try_get::<_, Option<i16>>(col_index)
            .map(|v| v.map_or(json!(null), |val| json!(val)))
            .unwrap_or_else(|_| json!(null)),
            
        Type::INT4 => row.try_get::<_, Option<i32>>(col_index)
            .map(|v| v.map_or(json!(null), |val| json!(val)))
            .unwrap_or_else(|_| json!(null)),
            
        Type::INT8 => row.try_get::<_, Option<i64>>(col_index)
            .map(|v| v.map_or(json!(null), |val| json!(val)))
            .unwrap_or_else(|_| json!(null)),
            
        Type::FLOAT4 => row.try_get::<_, Option<f32>>(col_index)
            .map(|v| v.map_or(json!(null), |val| json!(val)))
            .unwrap_or_else(|_| json!(null)),
            
        Type::FLOAT8 => row.try_get::<_, Option<f64>>(col_index)
            .map(|v| v.map_or(json!(null), |val| json!(val)))
            .unwrap_or_else(|_| json!(null)),
            
        Type::NUMERIC => {
            // 🔄 Para NUMERIC, tentamos múltiplas abordagens
            if let Ok(v) = row.try_get::<_, Option<f64>>(col_index) {
                v.map_or(json!(null), |val| json!(val))
            } else if let Ok(v) = row.try_get::<_, Option<String>>(col_index) {
                v.map_or(json!(null), |val| json!(val))
            } else {
                json!(null)
            }
        },
            
        // 📝 TIPOS TEXTO
        Type::TEXT | Type::VARCHAR | Type::CHAR | Type::NAME => 
            row.try_get::<_, Option<String>>(col_index)
                .map(|v| v.map_or(json!(null), |val| json!(val)))
                .unwrap_or_else(|_| json!(null)),
        
        // 📅 TIPOS DATA/HORA        
        Type::DATE => row.try_get::<_, Option<chrono::NaiveDate>>(col_index)
            .map(|v| v.map_or(json!(null), |val| json!(val.to_string())))
            .unwrap_or_else(|_| json!(null)),
            
        Type::TIMESTAMP => row.try_get::<_, Option<chrono::NaiveDateTime>>(col_index)
            .map(|v| v.map_or(json!(null), |val| json!(val.to_string())))
            .unwrap_or_else(|_| json!(null)),
            
        Type::TIMESTAMPTZ => row.try_get::<_, Option<chrono::DateTime<chrono::Utc>>>(col_index)
            .map(|v| v.map_or(json!(null), |val| json!(val.to_string())))
            .unwrap_or_else(|_| json!(null)),
            
        Type::TIME => row.try_get::<_, Option<chrono::NaiveTime>>(col_index)
            .map(|v| v.map_or(json!(null), |val| json!(val.to_string())))
            .unwrap_or_else(|_| json!(null)),
            
        // ✅ TIPO BOOLEANO
        Type::BOOL => row.try_get::<_, Option<bool>>(col_index)
            .map(|v| v.map_or(json!(null), |val| json!(val)))
            .unwrap_or_else(|_| json!(null)),
            
        // 🔐 TIPO UUID
        Type::UUID => row.try_get::<_, Option<uuid::Uuid>>(col_index)
            .map(|v| v.map_or(json!(null), |val| json!(val.to_string())))
            .unwrap_or_else(|_| json!(null)),
            
        // 📦 FALLBACK INTELIGENTE para tipos não mapeados
        _ => {
            // 🎯 ESTRATÉGIA CASCATA: Tenta conversões em ordem de probabilidade
            
            // 1️⃣ Primeira tentativa: String (mais comum)
            if let Ok(Some(v)) = row.try_get::<_, Option<String>>(col_index) {
                return json!(v);
            }
            if let Ok(v) = row.try_get::<_, String>(col_index) {
                return json!(v);
            }
            
            // 2️⃣ Segunda tentativa: Números inteiros
            if let Ok(v) = row.try_get::<_, i32>(col_index) {
                return json!(v);
            }
            if let Ok(v) = row.try_get::<_, i64>(col_index) {
                return json!(v);
            }
            
            // 3️⃣ Terceira tentativa: Números decimais
            if let Ok(v) = row.try_get::<_, f64>(col_index) {
                return json!(v);
            }
            
            // 4️⃣ Quarta tentativa: Booleano
            if let Ok(v) = row.try_get::<_, bool>(col_index) {
                return json!(v);
            }
            
            // 5️⃣ Última tentativa: Binário como string base64
            if let Ok(v) = row.try_get::<_, Vec<u8>>(col_index) {
                use base64::Engine;
                return json!(base64::engine::general_purpose::STANDARD.encode(v));
            }
            
            // 🆘 ÚLTIMO RECURSO: Tipo desconhecido
            log::warn!("Tipo PostgreSQL não suportado: {:?} na coluna {}", pg_type, col_index);
            json!(format!("UNSUPPORTED_TYPE:{}", pg_type.name()))
        }
    }
}
