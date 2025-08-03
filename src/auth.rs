// src/auth.rs
// Módulo de autenticação JWT

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
    error::ErrorUnauthorized,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use crate::config::Settings;
use crate::errors::{ApiError, ApiResult};

/// Claims do token JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // subject (username)
    pub exp: i64,    // expiration timestamp
    pub iat: i64,    // issued at timestamp
}

/// Gera um novo token JWT
#[allow(dead_code)]
pub fn generate_token(username: &str, settings: &Settings) -> ApiResult<String> {
    let now = chrono::Utc::now();
    let expiration = now + chrono::Duration::hours(settings.jwt.expiration_hours);
    
    let claims = Claims {
        sub: username.to_string(),
        exp: expiration.timestamp(),
        iat: now.timestamp(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(settings.jwt.secret.as_bytes()),
    )
    .map_err(|e| ApiError::Auth(format!("Erro ao gerar token: {}", e)))}

/// Valida um token JWT
pub fn validate_token(token: &str, settings: &Settings) -> ApiResult<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(settings.jwt.secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => ApiError::ExpiredToken,
        _ => ApiError::InvalidToken,
    })?;
    
    Ok(token_data.claims)
}

/// Middleware para validação JWT
pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService {
            service: Rc::new(service),
        }))
    }
}
/// Serviço do middleware JWT
pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        
        Box::pin(async move {
            // Extrair token do header Authorization
            let auth_header = req.headers().get("Authorization");
            
            if let Some(auth_value) = auth_header {
                if let Ok(auth_str) = auth_value.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];
                        
                        // Obter configurações
                        if let Some(settings) = req.app_data::<actix_web::web::Data<Settings>>() {
                            match validate_token(token, &settings) {
                                Ok(claims) => {
                                    // Adicionar claims ao request
                                    req.extensions_mut().insert(claims);
                                    return service.call(req).await;
                                }
                                Err(e) => {
                                    return Err(ErrorUnauthorized(e.to_string()));
                                }
                            }
                        }
                    }
                }
            }
            
            Err(ErrorUnauthorized("Token não fornecido"))
        })
    }
}

/// Função auxiliar para criar JWT (alias para generate_token)
pub fn create_jwt(username: &str, secret: &str, expiration_hours: i64) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let expiration = now + chrono::Duration::hours(expiration_hours);
    
    let claims = Claims {
        sub: username.to_string(),
        exp: expiration.timestamp(),
        iat: now.timestamp(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

// Implementação de FromRequest para Claims
use actix_web::{FromRequest, HttpRequest, dev::Payload};
use std::future::Ready as StdReady;

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = StdReady<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // Tentar pegar os claims das extensions (colocados pelo middleware)
        if let Some(claims) = req.extensions().get::<Claims>() {
            ready(Ok(claims.clone()))
        } else {
            ready(Err(ErrorUnauthorized("Claims não encontrados no request")))
        }
    }
}
