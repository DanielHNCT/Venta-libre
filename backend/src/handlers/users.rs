use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use sqlx::PgPool;

// GET /api/v1/users
pub async fn get_all_users(State(pool): State<PgPool>) -> Result<Json<Value>, StatusCode> {
    let users = sqlx::query!("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users_json: Vec<Value> = users
        .iter()
        .map(|user| {
            json!({
                "id": user.id,
                "name": user.name,
                "email": user.email
            })
        })
        .collect();

    Ok(Json(json!(users_json)))
}

// POST /api/v1/users (mantenemos simple por ahora)
pub async fn create_user() -> Result<Json<Value>, StatusCode> {
    let response = json!({"message": "Usuario creado", "id": 3});
    Ok(Json(response))
}

// GET /api/v1/users/:id
pub async fn get_user_by_id(
    Path(id): Path<i32>,
    State(pool): State<PgPool>
) -> Result<Json<Value>, StatusCode> {
    let user = sqlx::query!("SELECT id, name, email FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(user) => Ok(Json(json!({
            "id": user.id,
            "name": user.name,
            "email": user.email
        }))),
        None => Err(StatusCode::NOT_FOUND)
    }
}