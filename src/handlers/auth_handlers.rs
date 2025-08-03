// src/handlers/auth_handlers.rs
// Handlers de autenticação - Login e validação JWT

use actix_web::{web, HttpResponse, Result};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::auth::{create_jwt, Claims};
use crate::config::Settings;
use crate::errors::ApiError;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
    expires_in: i64,
    token_type: String,
}

/// Handler para login de usuário
/// Valida credenciais e retorna JWT token
pub async fn login(
    _pool: web::Data<Pool>,
    settings: web::Data<Settings>,
    credentials: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    log::info!("Tentativa de login para usuário: {}", credentials.username);
    
    // Validar credenciais admin configuradas no .env
    if credentials.username == settings.admin.username && credentials.password == settings.admin.password {
        log::info!("Login autorizado para usuário: {}", credentials.username);
        
        // Criar JWT token usando o mesmo padrão que funciona
        match create_jwt(&credentials.username, &settings.jwt.secret, settings.jwt.expiration_hours) {
            Ok(token) => {
                log::info!("Token JWT gerado com sucesso");
                
                Ok(HttpResponse::Ok().json(LoginResponse {
                    token,
                    expires_in: settings.jwt.expiration_hours * 3600,
                    token_type: "Bearer".to_string(),
                }))
            },
            Err(e) => {
                log::error!("Erro ao gerar token JWT: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "message": "Erro interno ao gerar token",
                    "error": e.to_string()
                })))
            }
        }
    } else {
        log::warn!("Credenciais inválidas para usuário: {}", credentials.username);
        Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Credenciais inválidas"
        })))
    }
}

/// Handler para validar token JWT
/// Retorna informações do token se válido
pub async fn validate_token(
    claims: web::ReqData<Claims>,
) -> Result<HttpResponse> {
    let claims = claims.into_inner();
    log::info!("Validando token para usuário: {}", claims.sub);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "valid": true,
        "username": claims.sub,
        "expires_at": claims.exp,
        "issued_at": claims.iat
    })))
}

// Estrutura para futura implementação de usuários no banco
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub active: bool,
}

// Função auxiliar para criar hash de senha
#[allow(dead_code)]
pub fn hash_password(password: &str) -> Result<String, ApiError> {
    hash(password, DEFAULT_COST)
        .map_err(|_| ApiError::InternalError("Erro ao gerar hash de senha".to_string()))
}

// Função auxiliar para verificar senha
#[allow(dead_code)]
pub fn verify_password(password: &str, hash: &str) -> Result<bool, ApiError> {
    verify(password, hash)
        .map_err(|_| ApiError::InternalError("Erro ao verificar senha".to_string()))
}