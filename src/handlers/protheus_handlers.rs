use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use crate::database::DatabasePools;
use crate::errors::ApiError;
use crate::auth::Claims;

#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    #[serde(alias = "query")]  // Aceita tanto "sql" quanto "query"
    pub sql: String,
    pub params: Option<Vec<serde_json::Value>>,
}

/// Executa query customizada no banco do Protheus
pub async fn query_protheus(
    query: web::Json<QueryRequest>,
    pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    log::info!("Executando query no Protheus: {}", query.sql);
    
    let _conn = pools.sqlserver_protheus.get().await
        .map_err(|e| ApiError::Database(format!("Erro ao conectar no Protheus: {}", e)))?;
    
    // Por enquanto apenas retornamos uma resposta de placeholder
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Handler Protheus em desenvolvimento",
        "database": "SIGAOFC"
    })))
}

/// Busca status de pedido no Protheus
pub async fn status_pedido_protheus(
    numero: web::Path<String>,
    _pools: web::Data<DatabasePools>,
    _claims: Claims,
) -> Result<HttpResponse, ApiError> {
    log::info!("Buscando status do pedido {} no Protheus", numero);
    
    // TODO: Implementar query real nas tabelas ZC7010/ZC8010
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "numero_pedido": numero.into_inner(),
        "status": "em desenvolvimento"
    })))
}
