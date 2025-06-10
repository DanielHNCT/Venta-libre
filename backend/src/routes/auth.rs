use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use crate::handlers::auth;

pub fn create_auth_routes() -> Router<PgPool> {
    Router::new()
        // Rutas públicas (sin autenticación)
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        // Rutas que manejan autenticación internamente
        .route("/me", get(auth::get_current_user))
        .route("/logout", post(auth::logout))
}