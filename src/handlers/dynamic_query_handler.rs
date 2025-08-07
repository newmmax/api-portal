// src/handlers/dynamic_query_handler.rs
// 🚀 Dynamic Query Handler - Inspirado na arquitetura Rapido-SQL
// 
// SOLUÇÃO PARA: SELECT *, queries complexas, WITH (CTEs), CASE statements
// ARQUITETURA: Conversão dinâmica PostgreSQL → JSON sem structs fixas
// COMPATIBILIDADE: 100% compatível com sistema existente (endpoint paralelo)

use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use serde_json::{json, Value, Map};
use tokio_postgres::Row;
use crate::database::DatabasePools;

/// 📋 Request para query dinâmica (compatível com endpoint existente)
#[derive(Debug, Deserialize)]
pub struct DynamicQueryRequest {
    #[serde(alias = "sql")]
    pub query: String,
    pub database: Option<String>,
}

/// 🚀 ENDPOINT PRINCIPAL: Executa queries PostgreSQL com conversão dinâmica
/// 
/// ✅ FUNCIONA COM:
/// - SELECT * FROM qualquer_tabela
/// - WITH (CTEs) complexas  
/// - CASE statements aninhados
/// - JOINs dinâmicos
/// - Qualquer query SELECT válida
/// 
/// 🛡️ SEGURANÇA: Mantém todas as validações do sistema original
pub async fn execute_dynamic_query(
    pools: web::Data<DatabasePools>,
    query_req: web::Json<DynamicQueryRequest>,
) -> Result<HttpResponse> {
    let query = query_req.query.trim();
    
    // 🔒 VALIDAÇÃO DE SEGURANÇA: Apenas SELECT (mesma do sistema original)
    if !is_select_query(query) {
        log::warn!("🚨 Tentativa de query não-SELECT no endpoint dinâmico: {}", query);
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "error": "SECURITY_RESTRICTION",
            "message": "Apenas consultas SELECT são permitidas por motivos de segurança",
            "endpoint": "dynamic",
            "allowed_examples": [
                "SELECT * FROM fc14000 LIMIT 10",
                "SELECT companygroupname, cnpj FROM fc14000",
                "WITH cte AS (SELECT * FROM fc03000) SELECT * FROM cte",
                "SELECT CASE WHEN id > 100 THEN 'Alto' ELSE 'Baixo' END FROM fc14000"
            ]
        })));
    }
    
    log::info!("🔍 Executando query dinâmica PostgreSQL: {}", query);
    
    // 🔌 CONEXÃO POSTGRESQL (mesmo pool do sistema original)
    let client = match pools.postgres_fc.get().await {
        Ok(client) => client,
        Err(e) => {
            log::error!("❌ Erro no pool PostgreSQL (dinâmico): {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "error": "CONNECTION_POOL_ERROR",
                "message": "Erro na conexão com banco PostgreSQL",
                "details": e.to_string(),
                "endpoint": "dynamic"
            })));
        }
    };
    
    // 🚀 EXECUÇÃO DA QUERY (sem preparação prévia)
    let rows = match client.query(query, &[]).await {
        Ok(rows) => rows,
        Err(e) => {
            log::error!("❌ Erro ao executar query dinâmica: {}", e);
            return Ok(HttpResponse::BadRequest().json(json!({
                "success": false,
                "error": "QUERY_EXECUTION_ERROR",
                "message": "Erro durante execução da consulta dinâmica",
                "details": e.to_string(),
                "query_submitted": query,
                "suggestions": [
                    "Verifique se as tabelas existem no banco FC",
                    "Confirme a sintaxe PostgreSQL",
                    "Para SELECT *, certifique-se que a tabela tem dados",
                    "Use LIMIT para queries grandes"
                ]
            })));
        }
    };
    
    // ⭐ CONVERSÃO DINÂMICA (inspirada na Rapido-SQL)
    let result = dynamic_postgres_to_json(rows);
    
    log::info!("✅ Query dinâmica executada com sucesso. {} registros retornados", 
        result.get("count").and_then(|v| v.as_u64()).unwrap_or(0));
    
    Ok(HttpResponse::Ok().json(result))
}

/// 🎯 CORAÇÃO DA SOLUÇÃO: Conversão dinâmica PostgreSQL → JSON
/// 
/// INSPIRAÇÃO RAPIDO-SQL: Não depende de structs fixas, detecta tipos automaticamente
/// INOVAÇÃO: Adaptado para PostgreSQL (original era SQL Server)
fn dynamic_postgres_to_json(rows: Vec<Row>) -> Value {
    if rows.is_empty() {
        return json!({
            "success": true,
            "count": 0,
            "data": [],
            "message": "Query executada com sucesso, mas não retornou registros",
            "query_type": "dynamic",
            "columns": []
        });
    }
    
    // 📊 EXTRAIR METADADOS DAS COLUNAS (primeira linha)
    let first_row = &rows[0];
    let columns: Vec<ColumnMetadata> = first_row.columns().iter()
        .enumerate()
        .map(|(index, col)| ColumnMetadata {
            index,
            name: col.name().to_string(),
            pg_type: col.type_().name().to_string(),
            is_nullable: true, // PostgreSQL não expõe facilmente
        })
        .collect();
    
    // 🔄 CONVERTER TODAS AS LINHAS
    let data: Vec<Value> = rows.iter()
        .map(|row| dynamic_row_to_json(row, &columns))
        .collect();
    
    // 📈 ESTATÍSTICAS AVANÇADAS (inspirado na Rapido-SQL)
    let stats = generate_dynamic_stats(&data, &columns);
    
    json!({
        "success": true,
        "count": rows.len(),
        "data": data,
        "message": format!("Query dinâmica executada com sucesso. {} registros retornados.", rows.len()),
        "query_type": "dynamic",
        "columns": columns.iter().map(|c| json!({
            "name": c.name,
            "type": c.pg_type,
            "index": c.index
        })).collect::<Vec<_>>(),
        "stats": stats
    })
}

/// 🔧 CONVERSÃO DE LINHA: PostgreSQL Row → JSON Object
/// 
/// ESTRATÉGIA CASCATA: Inspirada na Rapido-SQL, tenta múltiplos tipos automaticamente
fn dynamic_row_to_json(row: &Row, columns: &[ColumnMetadata]) -> Value {
    let mut obj = Map::new();
    
    for column in columns {
        // ⭐ CONVERSÃO INTELIGENTE por coluna
        let value = dynamic_value_converter(row, column.index, &column.name);
        obj.insert(column.name.clone(), value);
    }
    
    Value::Object(obj)
}

/// 🎯 NÚCLEO DA CONVERSÃO: Estratégia cascata para tipos PostgreSQL
/// 
/// INSPIRAÇÃO RAPIDO-SQL: Tenta múltiplos tipos até encontrar um que funcione
/// MELHORIA: Adaptado para peculiaridades do PostgreSQL vs SQL Server
fn dynamic_value_converter(row: &Row, col_index: usize, col_name: &str) -> Value {
    // 🔄 ESTRATÉGIA CASCATA: Ordem de probabilidade para PostgreSQL
    
    // 1️⃣ PRIMEIRA TENTATIVA: String/Text (mais comum em PostgreSQL)
    if let Ok(v) = row.try_get::<_, Option<String>>(col_index) {
        return v.map_or(json!(null), |val| json!(val.trim()));
    }
    if let Ok(v) = row.try_get::<_, String>(col_index) {
        return json!(v.trim());
    }
    
    // 2️⃣ SEGUNDA TENTATIVA: Inteiros (int4, int8, int2)
    if let Ok(v) = row.try_get::<_, Option<i32>>(col_index) {
        return v.map_or(json!(null), |val| json!(val));
    }
    if let Ok(v) = row.try_get::<_, i32>(col_index) {
        return json!(v);
    }
    if let Ok(v) = row.try_get::<_, Option<i64>>(col_index) {
        return v.map_or(json!(null), |val| json!(val));
    }
    if let Ok(v) = row.try_get::<_, i64>(col_index) {
        return json!(v);
    }
    if let Ok(v) = row.try_get::<_, Option<i16>>(col_index) {
        return v.map_or(json!(null), |val| json!(val));
    }
    if let Ok(v) = row.try_get::<_, i16>(col_index) {
        return json!(v);
    }
    
    // 3️⃣ TERCEIRA TENTATIVA: Decimais/Float (float4, float8, numeric)
    if let Ok(v) = row.try_get::<_, Option<f64>>(col_index) {
        return v.map_or(json!(null), |val| json!(val));
    }
    if let Ok(v) = row.try_get::<_, f64>(col_index) {
        return json!(v);
    }
    if let Ok(v) = row.try_get::<_, Option<f32>>(col_index) {
        return v.map_or(json!(null), |val| json!(val));
    }
    if let Ok(v) = row.try_get::<_, f32>(col_index) {
        return json!(v);
    }
    
    // 4️⃣ QUARTA TENTATIVA: Booleanos
    if let Ok(v) = row.try_get::<_, Option<bool>>(col_index) {
        return v.map_or(json!(null), |val| json!(val));
    }
    if let Ok(v) = row.try_get::<_, bool>(col_index) {
        return json!(v);
    }
    
    // 5️⃣ QUINTA TENTATIVA: Datas e timestamps
    if let Ok(v) = row.try_get::<_, Option<chrono::NaiveDateTime>>(col_index) {
        return v.map_or(json!(null), |val| json!(val.format("%Y-%m-%d %H:%M:%S").to_string()));
    }
    if let Ok(v) = row.try_get::<_, chrono::NaiveDateTime>(col_index) {
        return json!(v.format("%Y-%m-%d %H:%M:%S").to_string());
    }
    if let Ok(v) = row.try_get::<_, Option<chrono::NaiveDate>>(col_index) {
        return v.map_or(json!(null), |val| json!(val.to_string()));
    }
    if let Ok(v) = row.try_get::<_, chrono::NaiveDate>(col_index) {
        return json!(v.to_string());
    }
    if let Ok(v) = row.try_get::<_, Option<chrono::NaiveTime>>(col_index) {
        return v.map_or(json!(null), |val| json!(val.to_string()));
    }
    if let Ok(v) = row.try_get::<_, chrono::NaiveTime>(col_index) {
        return json!(v.to_string());
    }
    
    // 6️⃣ SEXTA TENTATIVA: UUIDs
    if let Ok(v) = row.try_get::<_, Option<uuid::Uuid>>(col_index) {
        return v.map_or(json!(null), |val| json!(val.to_string()));
    }
    if let Ok(v) = row.try_get::<_, uuid::Uuid>(col_index) {
        return json!(v.to_string());
    }
    
    // 7️⃣ SÉTIMA TENTATIVA: Dados binários
    if let Ok(v) = row.try_get::<_, Option<Vec<u8>>>(col_index) {
        return v.map_or(json!(null), |val| {
            use base64::Engine;
            json!(base64::engine::general_purpose::STANDARD.encode(val))
        });
    }
    if let Ok(v) = row.try_get::<_, Vec<u8>>(col_index) {
        use base64::Engine;
        return json!(base64::engine::general_purpose::STANDARD.encode(v));
    }
    
    // 🆘 ÚLTIMO RECURSO: Tipo não suportado
    log::warn!("🟡 Tipo PostgreSQL não mapeado na coluna '{}' (índice {})", col_name, col_index);
    json!(format!("UNMAPPED_PG_TYPE:{}", col_name))
}

/// 📊 GERAR ESTATÍSTICAS: Inspirado no módulo de stats da Rapido-SQL
fn generate_dynamic_stats(data: &[Value], columns: &[ColumnMetadata]) -> Value {
    let mut stats = Map::new();
    
    stats.insert("row_count".to_string(), json!(data.len()));
    stats.insert("column_count".to_string(), json!(columns.len()));
    stats.insert("has_data".to_string(), json!(!data.is_empty()));
    
    if !data.is_empty() {
        let mut column_stats = Map::new();
        
        for column in columns {
            let null_count = data.iter()
                .filter(|row| {
                    row.get(&column.name)
                        .map_or(true, |v| v.is_null())
                })
                .count();
            
            let mut col_stat = Map::new();
            col_stat.insert("null_count".to_string(), json!(null_count));
            col_stat.insert("non_null_count".to_string(), json!(data.len() - null_count));
            col_stat.insert("type".to_string(), json!(column.pg_type));
            
            column_stats.insert(column.name.clone(), Value::Object(col_stat));
        }
        
        stats.insert("column_stats".to_string(), Value::Object(column_stats));
    }
    
    Value::Object(stats)
}

/// 📋 METADADOS DE COLUNA: Estrutura auxiliar para organização
#[derive(Debug, Clone)]
struct ColumnMetadata {
    pub index: usize,
    pub name: String,
    pub pg_type: String,
    pub is_nullable: bool,
}

/// 🔒 VALIDAÇÃO DE SEGURANÇA: Reutilizada do sistema original
fn is_select_query(query: &str) -> bool {
    let normalized = query.trim().to_lowercase();
    
    // Verificar se começa com SELECT ou WITH (para CTEs)
    if !normalized.starts_with("select") && !normalized.starts_with("with") {
        return false;
    }
    
    // Verificar palavras-chave proibidas
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_select_query() {
        // ✅ Queries válidas
        assert!(is_select_query("SELECT * FROM fc14000"));
        assert!(is_select_query("SELECT id, name FROM users"));
        assert!(is_select_query("WITH cte AS (SELECT * FROM fc03000) SELECT * FROM cte"));
        assert!(is_select_query("  select count(*) from fc14000  "));
        
        // ❌ Queries inválidas
        assert!(!is_select_query("INSERT INTO users VALUES (1)"));
        assert!(!is_select_query("UPDATE users SET name = 'test'"));
        assert!(!is_select_query("DELETE FROM users"));
        assert!(!is_select_query("DROP TABLE users"));
        assert!(!is_select_query("SELECT * FROM users; DROP TABLE logs"));
    }
    
    #[test]
    fn test_column_metadata_creation() {
        let metadata = ColumnMetadata {
            index: 0,
            name: "test_column".to_string(),
            pg_type: "text".to_string(),
            is_nullable: true,
        };
        
        assert_eq!(metadata.name, "test_column");
        assert_eq!(metadata.pg_type, "text");
        assert!(metadata.is_nullable);
    }
}
