use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;
use chrono::{Duration, Utc};
use crate::models::auth::Claims;
use crate::models::user::User;

// Configuración JWT
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

impl JwtConfig {
    pub fn from_env() -> Self {
        Self {
            secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string()),
            expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
        }
    }
}

// Generar token JWT
pub fn generate_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let config = JwtConfig::from_env();
    let now = Utc::now();
    let expiration = now + Duration::hours(config.expiration_hours);
    
    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        name: user.name.clone(),
        is_admin: user.is_admin,
        exp: expiration.timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_ref()),
    )
}

// Verificar y decodificar token JWT
pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let config = JwtConfig::from_env();
    
    let validation = Validation::default();
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_ref()),
        &validation,
    )?;
    
    Ok(token_data.claims)
}

// Extraer token del header Authorization
pub fn extract_token_from_header(auth_header: &str) -> Option<&str> {
    if auth_header.starts_with("Bearer ") {
        Some(&auth_header[7..])
    } else {
        None
    }
}

// Validar que el token no haya expirado
pub fn is_token_expired(claims: &Claims) -> bool {
    let now = Utc::now().timestamp() as usize;
    claims.exp < now
}

// Generar token de larga duración para admins
pub fn generate_admin_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    if !user.is_admin {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken
        ));
    }
    
    let config = JwtConfig::from_env();
    let now = Utc::now();
    let expiration = now + Duration::days(7); // Token de admin dura 7 días
    
    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        name: user.name.clone(),
        is_admin: user.is_admin,
        exp: expiration.timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_ref()),
    )
}