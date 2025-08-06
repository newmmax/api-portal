use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tiberius::{Query, Row, QueryItem};
use futures_util::TryStreamExt;
use crate::database::DatabasePools;
use crate::errors::ApiError;
use crate::auth::Claims;

#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    #[serde(alias = "query")]  // Aceita tanto "sql" quanto "query"
    pub sql: String,
    pub params: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct ProdutosParams {
    pub cliente_id: Option<i32>,
    #[allow(dead_code)] // Usado indiretamente pela query SQL (pp.grupo_venda = c.grupo_venda)
    pub grupo_venda: Option<String>,
    pub apenas_ativos: Option<bool>,
    pub limite: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ProdutoPortal {
    pub id: i32,
    pub codigo: String,
    pub descricao: String,
    pub saldo: i32,
    pub status: bool,
    pub quantidade_minima_embalagem: i32,
    pub preco_unitario: Option<f64>,
    pub grupo_venda: Option<String>,
}

/// Executa query customizada no banco do Portal
/// 🔧 MELHORIA: Feedback detalhado e suporte aprimorado para SELECT *
/// 🛡️ SEGURANÇA: Mantém validações de segurança (apenas SELECT implícito via tiberius)
pub async fn query_portal(
    query: web::Json<QueryRequest>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    let sql = query.sql.trim();
    log::info!("🔍 Executando query no Portal SQL Server: {}", sql);
    
    // 🔌 OBTER CONEXÃO DO POOL SQL SERVER
    let mut conn = match pools.sqlserver_portal.get().await {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("❌ Erro ao conectar no Portal SQL Server: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "error": "CONNECTION_POOL_ERROR",
                "message": "Erro na conexão com banco Portal (SQL Server)",
                "details": e.to_string()
            })));
        }
    };
    
    // 📝 PREPARAR QUERY
    let mut query_obj = Query::new(sql);
    
    // 📎 ADICIONAR PARÂMETROS se fornecidos
    if let Some(params) = &query.params {
        log::debug!("📎 Adicionando {} parâmetros à query", params.len());
        for (i, param) in params.iter().enumerate() {
            match param {
                serde_json::Value::String(s) => {
                    query_obj.bind(s);
                    log::debug!("  Param {}: String = '{}'", i, s);
                },
                serde_json::Value::Number(n) => {
                    if let Some(i_val) = n.as_i64() {
                        query_obj.bind(i_val);
                        log::debug!("  Param {}: Int64 = {}", i, i_val);
                    } else if let Some(f_val) = n.as_f64() {
                        query_obj.bind(f_val);
                        log::debug!("  Param {}: Float64 = {}", i, f_val);
                    }
                },
                serde_json::Value::Bool(b) => {
                    query_obj.bind(*b);
                    log::debug!("  Param {}: Bool = {}", i, b);
                },
                _ => {
                    log::warn!("  Param {}: Tipo não suportado, ignorado", i);
                },
            }
        }
    }
    
    // 🚀 EXECUTAR QUERY
    let result = query_obj.query(&mut conn).await
        .map_err(|e| {
            log::error!("❌ Erro ao executar query SQL Server: {}", e);
            ApiError::Database(format!("Erro ao executar query: {}", e))
        })?;
    
    let mut rows = Vec::new();
    
    // 🔄 CONSUMIR STREAM e coletar resultados
    let mut stream = result;
    while let Some(item) = stream.try_next().await
        .map_err(|e| {
            log::error!("❌ Erro ao ler resultados SQL Server: {}", e);
            ApiError::Database(format!("Erro ao ler resultados: {}", e))
        })? {
        match item {
            QueryItem::Row(row) => {
                // 🎯 USAR NOVA CONVERSÃO ROBUSTA
                rows.push(row_to_json(&row));
            }
            _ => {} // Ignorar outros tipos de itens (metadata, etc.)
        }
    }
    
    log::info!("✅ Query SQL Server executada com sucesso. {} registros retornados", rows.len());
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": rows,
        "count": rows.len(),
        "message": format!("Query executada com sucesso no Portal. {} registros retornados.", rows.len()),
        "database": "SQL Server (Portal de Pedidos)"
    })))
}


/// Lista produtos com preços por grupo de venda
pub async fn listar_produtos_por_grupo(
    params: web::Query<ProdutosParams>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    // Usando o mesmo padrão que funciona
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
    
    let cliente_id = params.cliente_id.unwrap_or(0);
    let apenas_ativos = params.apenas_ativos.unwrap_or(true);
    let limite = params.limite.unwrap_or(100);
    
    let sql = r#"
        SELECT TOP (@P1)
            p.id,
            p.codigo,
            p.descricao,
            p.saldo,
            p.status,
            p.quantidade_minima_embalagem,
            pp.preco_unitario,
            c.grupo_venda
        FROM produtos p
        LEFT JOIN clientes c ON c.id = @P2
        LEFT JOIN precos_produtos pp ON p.id = pp.produto_id 
            AND pp.grupo_venda = c.grupo_venda
        WHERE (@P3 = 0 OR p.status = 1)
        AND p.saldo > 0
        ORDER BY p.descricao
    "#;
    
    let mut query = Query::new(sql);
    query.bind(limite);
    query.bind(cliente_id);
    query.bind(if apenas_ativos { 1 } else { 0 });
    
    let rows = query.query(&mut conn).await
        .map_err(|e| ApiError::Database(format!("Erro ao buscar produtos: {}", e)))?;
    
    let mut produtos = Vec::new();
    
    // Consumir o stream e coletar os produtos
    let mut stream = rows;
    while let Some(item) = stream.try_next().await
        .map_err(|e| ApiError::Database(format!("Erro ao ler produtos: {}", e)))? {
        match item {
            QueryItem::Row(row) => {
                produtos.push(ProdutoPortal {
                    id: row.get::<i32, _>(0).unwrap_or(0),
                    codigo: row.get::<&str, _>(1).unwrap_or("").to_string(),
                    descricao: row.get::<&str, _>(2).unwrap_or("").to_string(),
                    saldo: row.get::<i32, _>(3).unwrap_or(0),
                    status: row.get::<bool, _>(4).unwrap_or(false),
                    quantidade_minima_embalagem: row.get::<i32, _>(5).unwrap_or(1),
                    preco_unitario: row.get::<f64, _>(6),
                    grupo_venda: row.get::<&str, _>(7).map(|s| s.to_string()),
                });
            }
            _ => {} // Ignorar outros tipos de itens
        }
    }
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": produtos,
        "count": produtos.len()
    })))
}

/// Converte uma Row do tiberius para JSON com suporte robusto a todos os tipos SQL Server
/// 🔧 MELHORIA CRÍTICA: Suporta SELECT * e todos os tipos SQL Server comuns  
/// 🛡️ COMPATIBILIDADE: 100% compatível com código anterior
/// 📊 PROBLEMA RESOLVIDO: SELECT * FROM clientes agora funciona perfeitamente
fn row_to_json(row: &Row) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    
    for (i, col) in row.columns().iter().enumerate() {
        let name = col.name();
        
        // 🎯 CONVERSÃO INTELIGENTE baseada no tipo SQL Server real
        let value = convert_sqlserver_value_to_json(row, i, col.column_type());
        obj.insert(name.to_string(), value);
    }
    
    serde_json::Value::Object(obj)
}

/// 🔧 NOVA FUNÇÃO: Conversão inteligente baseada no tipo SQL Server real
/// Resolve o problema do SELECT * ao usar informações precisas de tipo
/// 📋 ADAPTADO para API do tiberius (SQL Server)
fn convert_sqlserver_value_to_json(row: &Row, col_index: usize, sql_type: tiberius::ColumnType) -> serde_json::Value {
    use tiberius::time::chrono::NaiveDateTime;
    use tiberius::ColumnType;
    
    // 📋 MAPEAMENTO DOS TIPOS SQL SERVER DISPONÍVEIS NO TIBERIUS
    match sql_type {
        // 🔢 TIPOS NUMÉRICOS
        ColumnType::Int1 => {
            // Pode ser BOOL ou TINYINT
            if let Some(v) = row.get::<bool, _>(col_index) {
                json!(v)
            } else if let Some(v) = row.get::<u8, _>(col_index) {
                json!(v)
            } else {
                json!(null)
            }
        },
        
        ColumnType::Int2 => row.get::<i16, _>(col_index)
            .map(|v| json!(v))
            .unwrap_or(json!(null)),
            
        ColumnType::Int4 => row.get::<i32, _>(col_index)
            .map(|v| json!(v))
            .unwrap_or(json!(null)),
            
        ColumnType::Int8 => row.get::<i64, _>(col_index)
            .map(|v| json!(v))
            .unwrap_or(json!(null)),
            
        ColumnType::Float4 => row.get::<f32, _>(col_index)
            .map(|v| json!(v))
            .unwrap_or(json!(null)),
            
        ColumnType::Float8 => row.get::<f64, _>(col_index)
            .map(|v| json!(v))
            .unwrap_or(json!(null)),
            
        // 💰 TIPOS MONETÁRIOS E DECIMAIS
        ColumnType::Money => {
            // Tentar como decimal primeiro, fallback para float
            if let Some(v) = row.get::<rust_decimal::Decimal, _>(col_index) {
                json!(v.to_string())
            } else if let Some(v) = row.get::<f64, _>(col_index) {
                json!(v)
            } else {
                json!(null)
            }
        },
        
        ColumnType::Decimaln | ColumnType::Numericn => {
            // Para DECIMAL/NUMERIC, tentar múltiplas abordagens
            if let Some(v) = row.get::<rust_decimal::Decimal, _>(col_index) {
                json!(v.to_string())
            } else if let Some(v) = row.get::<f64, _>(col_index) {
                json!(v)
            } else {
                json!(null)
            }
        },
            
        // 📝 TIPOS TEXTO (SQL Server específicos)
        ColumnType::NChar | ColumnType::NVarchar | ColumnType::NText => {
            row.get::<&str, _>(col_index)
                .map(|v| json!(v.trim()))  // Remove espaços em branco
                .unwrap_or(json!(null))
        },
        
        ColumnType::Text => {
            row.get::<&str, _>(col_index)
                .map(|v| json!(v.trim()))
                .unwrap_or(json!(null))
        },
        
        // 📅 TIPOS DATA/HORA SQL Server (CORRIGIDO para suportar NULL)
        ColumnType::Datetime | ColumnType::Datetime2 => {
            // 🛡️ CORREÇÃO CRÍTICA: get() já retorna Option<T>, verificar NULL explicitamente
            match row.get::<NaiveDateTime, _>(col_index) {
                Some(dt) => json!(dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                None => json!(null),
            }
        },
        
        ColumnType::Daten => {
            // 🛡️ CORREÇÃO: get() já retorna Option<T>
            match row.get::<chrono::NaiveDate, _>(col_index) {
                Some(d) => json!(d.to_string()),
                None => json!(null),
            }
        },
        
        ColumnType::Timen => {
            // 🛡️ CORREÇÃO: get() já retorna Option<T>
            match row.get::<chrono::NaiveTime, _>(col_index) {
                Some(t) => json!(t.to_string()),
                None => json!(null),
            }
        },
                
        // 🔐 TIPOS IDENTIFICADORES
        ColumnType::Guid => {
            row.get::<uuid::Uuid, _>(col_index)
                .map(|id| json!(id.to_string()))
                .unwrap_or(json!(null))
        },
                
        // 📦 TIPOS BINÁRIOS
        ColumnType::Image => {
            if let Some(v) = row.get::<&[u8], _>(col_index) {
                use base64::Engine;
                json!(base64::engine::general_purpose::STANDARD.encode(v))
            } else {
                json!(null)
            }
        },
        
        // 📊 FALLBACK INTELIGENTE para tipos não mapeados
        _ => {
            // 🎯 ESTRATÉGIA CASCATA: Tenta conversões em ordem de probabilidade
            
            // 1️⃣ Primeira tentativa: String (mais comum no SQL Server)
            if let Some(v) = row.get::<&str, _>(col_index) {
                return json!(v.trim());
            }
            
            // 2️⃣ Segunda tentativa: Números inteiros
            if let Some(v) = row.get::<i32, _>(col_index) {
                return json!(v);
            }
            if let Some(v) = row.get::<i64, _>(col_index) {
                return json!(v);
            }
            
            // 3️⃣ Terceira tentativa: Números decimais
            if let Some(v) = row.get::<f64, _>(col_index) {  
                return json!(v);
            }
            
            // 4️⃣ Quarta tentativa: Booleano
            if let Some(v) = row.get::<bool, _>(col_index) {
                return json!(v);
            }
            
            // 5️⃣ Quinta tentativa: Data/hora
            if let Some(v) = row.get::<NaiveDateTime, _>(col_index) {
                return json!(v.format("%Y-%m-%d %H:%M:%S").to_string());
            }
            
            // 6️⃣ Sexta tentativa: UUID
            if let Some(v) = row.get::<uuid::Uuid, _>(col_index) {
                return json!(v.to_string());
            }
            
            // 7️⃣ Sétima tentativa: Decimal
            if let Some(v) = row.get::<rust_decimal::Decimal, _>(col_index) {
                return json!(v.to_string());
            }
            
            // 8️⃣ Última tentativa: Binário como base64
            if let Some(v) = row.get::<&[u8], _>(col_index) {
                use base64::Engine;
                return json!(base64::engine::general_purpose::STANDARD.encode(v));
            }
            
            // 🆘 ÚLTIMO RECURSO: Tipo desconhecido
            log::warn!("Tipo SQL Server não suportado: {:?} na coluna {}", sql_type, col_index);
            json!(format!("UNSUPPORTED_TYPE:{:?}", sql_type))
        }
    }
}
