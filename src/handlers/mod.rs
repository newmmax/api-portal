// src/handlers/mod.rs
// Módulo principal de handlers - organiza todos os manipuladores de requisições

pub mod auth_handlers;
pub mod data_handlers;  
pub mod query_handlers;
pub mod dynamic_query_handler;  // 🚀 NOVO: Query dinâmica inspirada na Rapido-SQL
pub mod portal_handlers;
pub mod protheus_handlers;
pub mod debug_handlers;

// 🎯 ESTRUTURA MODULAR - Arquivos < 500 linhas
pub mod analytics;     // Novo: analytics modularizado
pub mod pedidos;       // Novo: pedidos modularizado

// 🌐 NOVOS ENDPOINTS CRÍTICOS
pub mod portal_endpoints;  // Endpoints básicos do portal

use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use crate::database::DatabasePools;

/// Handler para verificação de saúde da API
/// Retorna status do servidor e conexão com todos os bancos de dados
pub async fn health_check(
    pools: web::Data<DatabasePools>,
) -> Result<HttpResponse> {
    let mut status = json!({
        "status": "healthy",
        "message": "FC Data API Unificada está operacional",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "databases": {}
    });
    
    // Verificar PostgreSQL FC
    match pools.postgres_fc.get().await {
        Ok(_) => {
            status["databases"]["postgres_fc"] = json!({
                "status": "conectado",
                "database": "fc_data"
            });
        }
        Err(e) => {
            status["status"] = json!("degraded");
            status["databases"]["postgres_fc"] = json!({
                "status": "erro",
                "error": e.to_string()
            });
        }
    }
    
    // Verificar SQL Server Portal
    match pools.sqlserver_portal.get().await {
        Ok(_) => {
            status["databases"]["portal_pedidos"] = json!({
                "status": "conectado",
                "database": "sys_pedidos"
            });
        }
        Err(e) => {
            status["status"] = json!("degraded");
            status["databases"]["portal_pedidos"] = json!({
                "status": "erro",
                "error": e.to_string()
            });
        }
    }
    
    // Verificar SQL Server Protheus
    match pools.sqlserver_protheus.get().await {
        Ok(_) => {
            status["databases"]["protheus"] = json!({
                "status": "conectado",
                "database": "SIGAOFC"
            });
        }
        Err(e) => {
            status["status"] = json!("degraded");
            status["databases"]["protheus"] = json!({
                "status": "erro",
                "error": e.to_string()
            });
        }
    }
    
    let mut response_status = if status["status"] == "healthy" {
        HttpResponse::Ok()
    } else {
        HttpResponse::ServiceUnavailable()
    };
    
    Ok(response_status.json(status))
}
