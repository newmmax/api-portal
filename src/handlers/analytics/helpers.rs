//! 🔧 Helper Functions - Compatibilidade e Utilitários
//!
//! Funções auxiliares e endpoints mantidos para compatibilidade
//! com versões anteriores da API.

use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use crate::database::DatabasePools;
use crate::errors::ApiError;
use crate::auth::Claims;

#[derive(Debug, Deserialize)]
pub struct AnalyticsParams {
    pub periodo: Option<String>, // 30d, 90d, 180d, 365d
    #[allow(dead_code)] // Funcionalidade futura - diferentes tipos de análise
    pub tipo: Option<String>,    // vendas, compras, completo
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
