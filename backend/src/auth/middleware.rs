use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use sqlx::PgPool;
use crate::auth::jwt::{verify_token, extract_token_from_header};
use crate::models::auth::{AuthError, Claims};
use crate::models::user::User;

// Extension para agregar el usuario autenticado al request
#[derive(Clone)]
pub struct AuthUser {
    pub user: User,
    pub claims: Claims,
}

// Middleware para verificar autenticación
pub async fn auth_middleware(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<AuthError>)> {
    // Extraer token del header Authorization
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(AuthError::new("missing_token", "Token de autorización requerido")),
            )
        })?;

    let token = extract_token_from_header(auth_header).ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthError::new("invalid_format", "Formato de token inválido")),
        )
    })?;

    // Verificar token
    let claims = verify_token(token).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthError::invalid_token()),
        )
    })?;

    // Buscar usuario en base de datos
    let user_id: i32 = claims.sub.parse().map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthError::invalid_token()),
        )
    })?;

    let user = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash, is_admin, is_active, created_at, updated_at 
         FROM users WHERE id = $1 AND is_active = true"
    )
    .bind(user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthError::new("database_error", "Error de base de datos")),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthError::user_not_found()),
        )
    })?;

    // Verificar que el usuario esté activo
    if !user.is_active {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(AuthError::new("user_inactive", "Usuario inactivo")),
        ));
    }

    // Agregar usuario autenticado al request
    request.extensions_mut().insert(AuthUser { user, claims });

    Ok(next.run(request).await)
}

// Middleware para verificar que el usuario sea admin
pub async fn admin_middleware(
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<AuthError>)> {
    // Obtener usuario del middleware anterior
    let auth_user = request
        .extensions()
        .get::<AuthUser>()
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(AuthError::unauthorized()),
            )
        })?;

    // Verificar que sea admin
    if !auth_user.user.is_admin() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(AuthError::forbidden()),
        ));
    }

    Ok(next.run(request).await)
}

// Extractor para obtener el usuario autenticado fácilmente
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<AuthError>);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthUser>()
            .cloned()
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(AuthError::unauthorized()),
                )
            })
    }
}