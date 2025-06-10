use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// Modelo completo del usuario (para base de datos)
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub is_admin: bool,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Usuario público (sin password_hash)
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
}

// DTO para crear usuario
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

// DTO para actualizar usuario
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl User {
    // Convertir a usuario público (sin datos sensibles)
    pub fn to_public(&self) -> PublicUser {
        PublicUser {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            is_admin: self.is_admin,
            is_active: self.is_active,
            created_at: self.created_at,
        }
    }
    
    // Verificar si el usuario es admin
    pub fn is_admin(&self) -> bool {
        self.is_admin && self.is_active
    }
    
    // Verificar si el usuario está activo
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}