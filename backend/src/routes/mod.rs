pub mod users;
pub mod auth;

use axum::Router;
use sqlx::PgPool;

pub fn create_routes() -> Router<PgPool> {
    Router::new()
        .nest("/users", users::create_user_routes())
        .nest("/auth", auth::create_auth_routes())
}