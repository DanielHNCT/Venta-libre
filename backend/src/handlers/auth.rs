use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use sqlx::PgPool;
use crate::auth::generate_token;
use crate::models::auth::{AuthError, AuthResponse, LoginRequest, RegisterRequest};
use crate::models::user::{CreateUserRequest, User};

// POST /api/v1/auth/register
pub async fn register(
    
    State(pool): State<PgPool>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<AuthError>)> {
    tracing::info!("🔄 Intento de registro: email={}", request.email);
    // Validar datos de entrada
    if request.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AuthError::new("invalid_name", "El nombre es requerido")),
        ));
    }

    if request.email.trim().is_empty() || !request.email.contains('@') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AuthError::new("invalid_email", "Email inválido")),
        ));
    }

    if request.password.len() < 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AuthError::new("weak_password", "La contraseña debe tener al menos 6 caracteres")),
        ));
    }

    // Verificar que el email no exista
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        request.email.trim().to_lowercase()
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthError::new("database_error", "Error de base de datos")),
        )
    })?;

    if existing_user.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(AuthError::email_exists()),
        ));
    }

    // Hash de la contraseña
    let password_hash = hash(&request.password, DEFAULT_COST).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthError::new("hash_error", "Error al procesar contraseña")),
        )
    })?;

    // Crear usuario
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, password_hash, is_admin, is_active, created_at, updated_at)
         VALUES ($1, $2, $3, false, true, $4, $4)
         RETURNING id, name, email, password_hash, is_admin, is_active, created_at, updated_at"
    )
    .bind(request.name.trim())
    .bind(request.email.trim().to_lowercase())
    .bind(password_hash)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
    tracing::error!(
        error = %e,
        email = %request.email,
        "🚨 Error al crear usuario en BD"
    );
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(AuthError::new("create_user_error", "Error al crear usuario")),
    )
})?;

    // Generar token JWT
    let token = generate_token(&user).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthError::new("token_error", "Error al generar token")),
        )
    })?;

    // Calcular expiración (24 horas)
    let expires_at = (Utc::now() + chrono::Duration::hours(24)).timestamp();

    Ok(Json(AuthResponse {
        token,
        user: user.to_public(),
        expires_at,
    }))
}

// POST /api/v1/auth/login
pub async fn login(
    State(pool): State<PgPool>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<AuthError>)> {
    // Buscar usuario por email
    let user = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash, is_admin, is_active, created_at, updated_at
         FROM users WHERE email = $1"
    )
    .bind(request.email.trim().to_lowercase())
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
            Json(AuthError::invalid_credentials()),
        )
    })?;

    // Verificar que el usuario esté activo
    if !user.is_active {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(AuthError::new("user_inactive", "Usuario inactivo")),
        ));
    }

    // Verificar contraseña
    let password_hash = user.password_hash.as_ref().ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthError::invalid_credentials()),
        )
    })?;

    let password_valid = verify(&request.password, password_hash).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthError::new("verification_error", "Error al verificar contraseña")),
        )
    })?;

    if !password_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(AuthError::invalid_credentials()),
        ));
    }

    // Generar token JWT
    let token = generate_token(&user).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthError::new("token_error", "Error al generar token")),
        )
    })?;

    // Calcular expiración
    let expires_at = (Utc::now() + chrono::Duration::hours(24)).timestamp();

    Ok(Json(AuthResponse {
        token,
        user: user.to_public(),
        expires_at,
    }))
}

// GET /api/v1/auth/me
pub async fn get_current_user(
    State(pool): State<PgPool>,
    headers: axum::http::HeaderMap,
) -> Result<Json<crate::models::user::PublicUser>, (StatusCode, Json<AuthError>)> {
    // Extraer token del header
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(AuthError::new("missing_token", "Token de autorización requerido")),
            )
        })?;

    let token = crate::auth::jwt::extract_token_from_header(auth_header).ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthError::new("invalid_format", "Formato de token inválido")),
        )
    })?;

    // Verificar token
    let claims = crate::auth::jwt::verify_token(token).map_err(|_| {
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

    Ok(Json(user.to_public()))
}

// POST /api/v1/auth/logout
pub async fn logout() -> Result<Json<serde_json::Value>, StatusCode> {
    // En JWT no hay logout real del lado del servidor
    // El frontend debe eliminar el token
    Ok(Json(serde_json::json!({
        "message": "Sesión cerrada exitosamente"
    })))
}