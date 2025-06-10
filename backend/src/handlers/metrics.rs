use axum::{
    extract::{State, Path, Query},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;
use std::collections::HashMap;
use crate::metrics::MetricsCollector;
use crate::auth::middleware::AuthUser;

// Obtener métricas generales del sistema
pub async fn get_metrics(
    State(metrics_collector): State<Arc<MetricsCollector>>,
    auth_user: Option<AuthUser>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Solo admins pueden ver métricas completas
    if let Some(ref user) = auth_user {
        if !user.user.is_admin {
            return Err((
                StatusCode::FORBIDDEN,
                Json(serde_json::json!({
                    "error": "forbidden",
                    "message": "Solo administradores pueden acceder a las métricas"
                }))
            ));
        }
    } else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "unauthorized",
                "message": "Autenticación requerida"
            }))
        ));
    }

    let snapshot = metrics_collector.get_metrics_snapshot();
    
    tracing::info!(
        event = "metrics_accessed",
        user_id = auth_user.as_ref().map(|u| u.user.id),
        total_requests = snapshot.total_requests,
        uptime_seconds = snapshot.uptime_seconds,
        "📊 Métricas accedidas por admin"
    );

    Ok(Json(serde_json::to_value(snapshot).unwrap()))
}

// Métricas públicas básicas (sin autenticación)
pub async fn get_public_metrics(
    State(metrics_collector): State<Arc<MetricsCollector>>,
) -> Json<serde_json::Value> {
    let snapshot = metrics_collector.get_metrics_snapshot();
    
    // Solo información básica sin datos sensibles
    Json(serde_json::json!({
        "service": "venta-libre-api",
        "version": env!("CARGO_PKG_VERSION"),
        "uptime_seconds": snapshot.uptime_seconds,
        "total_requests": snapshot.total_requests,
        "requests_per_minute": snapshot.requests_per_minute,
        "avg_response_time_ms": snapshot.avg_response_time_ms,
        "timestamp": snapshot.timestamp
    }))
}

// Métricas de un endpoint específico
pub async fn get_endpoint_metrics(
    State(metrics_collector): State<Arc<MetricsCollector>>,
    Path((method, path)): Path<(String, String)>,
    auth_user: AuthUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Solo admins pueden ver métricas de endpoints
    if !auth_user.user.is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": "forbidden",
                "message": "Solo administradores pueden acceder a estas métricas"
            }))
        ));
    }

    match metrics_collector.get_endpoint_metrics(&method, &path) {
        Some(endpoint_metrics) => {
            tracing::debug!(
                event = "endpoint_metrics_accessed",
                user_id = auth_user.user.id,
                method = %method,
                path = %path,
                "📈 Métricas de endpoint accedidas"
            );
            
            Ok(Json(serde_json::to_value(endpoint_metrics).unwrap()))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "not_found",
                "message": "No se encontraron métricas para este endpoint"
            }))
        ))
    }
}

// Top endpoints más usados
pub async fn get_top_endpoints(
    State(metrics_collector): State<Arc<MetricsCollector>>,
    Query(params): Query<HashMap<String, String>>,
    auth_user: AuthUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Solo admins
    if !auth_user.user.is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": "forbidden",
                "message": "Solo administradores pueden acceder a esta información"
            }))
        ));
    }

    let snapshot = metrics_collector.get_metrics_snapshot();
    
    // Parámetro opcional para limitar resultados
    let limit: usize = params
        .get("limit")
        .and_then(|l| l.parse().ok())
        .unwrap_or(10)
        .min(50); // Máximo 50

    let top_endpoints: Vec<_> = snapshot
        .most_used_endpoints
        .into_iter()
        .take(limit)
        .collect();

    Ok(Json(serde_json::json!({
        "top_endpoints": top_endpoints,
        "limit": limit,
        "timestamp": chrono::Utc::now()
    })))
}

// Endpoints más lentos
pub async fn get_slowest_endpoints(
    State(metrics_collector): State<Arc<MetricsCollector>>,
    Query(params): Query<HashMap<String, String>>,
    auth_user: AuthUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if !auth_user.user.is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": "forbidden",
                "message": "Solo administradores pueden acceder a esta información"
            }))
        ));
    }

    let snapshot = metrics_collector.get_metrics_snapshot();
    
    let limit: usize = params
        .get("limit")
        .and_then(|l| l.parse().ok())
        .unwrap_or(10)
        .min(50);

    let slowest_endpoints: Vec<_> = snapshot
        .slowest_endpoints
        .into_iter()
        .take(limit)
        .collect();

    Ok(Json(serde_json::json!({
        "slowest_endpoints": slowest_endpoints,
        "limit": limit,
        "timestamp": chrono::Utc::now()
    })))
}

// Distribución de códigos de estado
pub async fn get_status_distribution(
    State(metrics_collector): State<Arc<MetricsCollector>>,
    auth_user: AuthUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if !auth_user.user.is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": "forbidden",
                "message": "Solo administradores pueden acceder a esta información"
            }))
        ));
    }

    let snapshot = metrics_collector.get_metrics_snapshot();

    // Agrupar por categorías de status
    let mut categories = HashMap::new();
    for (status, count) in &snapshot.status_code_distribution {
        let category = match status {
            200..=299 => "success",
            300..=399 => "redirect", 
            400..=499 => "client_error",
            500..=599 => "server_error",
            _ => "other",
        };
        
        *categories.entry(category).or_insert(0u64) += count;
    }

    Ok(Json(serde_json::json!({
        "status_distribution": snapshot.status_code_distribution,
        "categories": categories,
        "total_requests": snapshot.total_requests,
        "timestamp": chrono::Utc::now()
    })))
}

// Estadísticas por hora
pub async fn get_hourly_stats(
    State(metrics_collector): State<Arc<MetricsCollector>>,
    auth_user: AuthUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if !auth_user.user.is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": "forbidden",
                "message": "Solo administradores pueden acceder a esta información"
            }))
        ));
    }

    let snapshot = metrics_collector.get_metrics_snapshot();

    Ok(Json(serde_json::json!({
        "hourly_stats": snapshot.hourly_stats,
        "summary": {
            "total_hours": snapshot.hourly_stats.len(),
            "avg_requests_per_hour": snapshot.hourly_stats.iter()
                .map(|h| h.requests)
                .sum::<u64>() as f64 / snapshot.hourly_stats.len().max(1) as f64,
            "avg_response_time": snapshot.hourly_stats.iter()
                .map(|h| h.avg_response_time_ms)
                .sum::<f64>() / snapshot.hourly_stats.len().max(1) as f64,
        },
        "timestamp": chrono::Utc::now()
    })))
}