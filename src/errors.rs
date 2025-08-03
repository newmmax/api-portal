// src/errors.rs
// Definição de erros customizados para a API

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde_json::json;

/// Tipo de resultado padrão da aplicação
pub type ApiResult<T> = Result<T, ApiError>;

/// Enum principal de erros da API
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum ApiError {
    #[error("Erro de banco de dados: {0}")]
    Database(String),
    
    #[error("Erro de autenticação: {0}")]
    Auth(String),
    
    #[error("Token JWT inválido")]
    InvalidToken,
    
    #[error("Token JWT expirado")]
    ExpiredToken,
    
    #[error("Credenciais inválidas")]
    InvalidCredentials,
    
    #[error("Acesso não autorizado")]
    Unauthorized,
    
    #[error("Recurso não encontrado")]
    NotFound,
    
    #[error("Requisição inválida: {0}")]
    BadRequest(String),
    
    #[error("Erro interno do servidor: {0}")]
    InternalError(String),
    
    #[error("Erro de configuração: {0}")]
    Configuration(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let message = self.to_string();
        
        HttpResponse::build(status).json(json!({
            "error": true,
            "message": message,
            "code": status.as_u16()
        }))
    }
    
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Auth(_) => StatusCode::UNAUTHORIZED,
            ApiError::InvalidToken => StatusCode::UNAUTHORIZED,
            ApiError::ExpiredToken => StatusCode::UNAUTHORIZED,
            ApiError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            ApiError::Unauthorized => StatusCode::FORBIDDEN,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Configuration(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// Conversões de outros erros para ApiError
impl From<deadpool_postgres::PoolError> for ApiError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        ApiError::Database(format!("Erro no pool de conexões: {}", err))
    }
}

impl From<tokio_postgres::Error> for ApiError {
    fn from(err: tokio_postgres::Error) -> Self {
        ApiError::Database(format!("Erro PostgreSQL: {}", err))
    }
}

impl From<std::env::VarError> for ApiError {
    fn from(err: std::env::VarError) -> Self {
        ApiError::Configuration(format!("Erro de variável de ambiente: {}", err))
    }
}

// Conversões não são mais necessárias - removidas para evitar conflito
