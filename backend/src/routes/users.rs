use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use crate::handlers::users;

pub fn create_user_routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(users::get_all_users))
        .route("/", post(users::create_user))
        .route("/:id", get(users::get_user_by_id))
}