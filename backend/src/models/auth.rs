use serde::{Deserialize, Serialize};

// Request de login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Request de registro
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

// Response de autenticación exitosa
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: crate::models::user::PublicUser,
    pub expires_at: i64, // timestamp
}

// Claims del JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,    // user_id
    pub email: String,
    pub name: String,
    pub is_admin: bool,
    pub exp: usize,     // expiration time
    pub iat: usize,     // issued at
}

// Response de error de autenticación
#[derive(Debug, Serialize)]
pub struct AuthError {
    pub error: String,
    pub message: String,
}

impl AuthError {
    pub fn new(error: &str, message: &str) -> Self {
        Self {
            error: error.to_string(),
            message: message.to_string(),
        }
    }
    
    pub fn invalid_credentials() -> Self {
        Self::new("invalid_credentials", "Email o contraseña incorrectos")
    }
    
    pub fn user_not_found() -> Self {
        Self::new("user_not_found", "Usuario no encontrado")
    }
    
    pub fn email_exists() -> Self {
        Self::new("email_exists", "Este email ya está registrado")
    }
    
    pub fn invalid_token() -> Self {
        Self::new("invalid_token", "Token inválido o expirado")
    }
    
    pub fn unauthorized() -> Self {
        Self::new("unauthorized", "No autorizado")
    }
    
    pub fn forbidden() -> Self {
        Self::new("forbidden", "No tienes permisos para esta acción")
    }
}