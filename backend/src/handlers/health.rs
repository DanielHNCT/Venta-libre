use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use crate::health::HealthChecker;
use crate::logging::get_request_id;
use std::sync::Arc;

// Health check completo - usado para monitoreo
pub async fn health_check(
    State(health_checker): State<Arc<HealthChecker>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let health_response = health_checker.check_health().await;
    
    // Determinar status code basado en el estado
    let status_code = match health_response.status.as_str() {
        "healthy" => StatusCode::OK,
        "degraded" => StatusCode::OK, // 200 pero con warnings
        "unhealthy" => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    
    // Log del health check
    tracing::info!(
        event = "health_check",
        status = %health_response.status,
        uptime_seconds = %health_response.uptime_seconds,
        cpu_usage = %health_response.system.cpu_usage_percent,
        memory_usage_percent = %(health_response.system.memory_used_mb as f64 / health_response.system.memory_total_mb as f64 * 100.0),
        db_status = %health_response.checks.database.status,
        "🏥 Health check ejecutado"
    );
    
    if status_code.is_success() {
        Ok(Json(serde_json::to_value(health_response).unwrap()))
    } else {
        Err((status_code, Json(serde_json::to_value(health_response).unwrap())))
    }
}

// Liveness probe - usado por Kubernetes/Docker
pub async fn liveness_check(
    State(health_checker): State<Arc<HealthChecker>>,
) -> Json<serde_json::Value> {
    let liveness_response = health_checker.check_liveness().await;
    
    tracing::debug!(
        event = "liveness_check",
        status = "alive",
        "💓 Liveness check"
    );
    
    Json(liveness_response)
}

// Readiness probe - usado por Kubernetes/Docker
pub async fn readiness_check(
    State(health_checker): State<Arc<HealthChecker>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let (is_ready, readiness_response) = health_checker.check_readiness().await;
    
    tracing::debug!(
        event = "readiness_check",
        is_ready = %is_ready,
        "🚦 Readiness check"
    );
    
    if is_ready {
        Ok(Json(readiness_response))
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, Json(readiness_response)))
    }
}

// Status simple para load balancers
pub async fn status_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "venta-libre-api",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now()
    }))
}

// Información del servidor
pub async fn server_info(
    State(health_checker): State<Arc<HealthChecker>>,
) -> Json<serde_json::Value> {
    let system_metrics = health_checker.check_health().await.system;
    
    Json(serde_json::json!({
        "service": "venta-libre-api",
        "version": env!("CARGO_PKG_VERSION"),
        "environment": std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
        "rust_version": env!("CARGO_PKG_RUST_VERSION"),
        "build_timestamp": "compiled",
        "uptime_seconds": health_checker.check_liveness().await["uptime_seconds"],
        "system": {
            "cpu_cores": num_cpus::get(),
            "memory_total_mb": system_metrics.memory_total_mb,
            "disk_total_gb": system_metrics.disk_total_gb,
            "platform": std::env::consts::OS,
            "architecture": std::env::consts::ARCH
        },
        "features": {
            "authentication": true,
            "logging": true,
            "metrics": true,
            "health_checks": true,
            "cors": true
        }
    }))
}