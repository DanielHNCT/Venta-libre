use tracing_subscriber::{
    fmt,
    prelude::*,
    EnvFilter,
    layer::SubscriberExt,
};
use tracing_appender::{rolling, non_blocking};
use std::env;
use std::fs;
use serde_json::json;

pub struct Logger;

impl Logger {
    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let env = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    
    // Configuración super simple para debug
    tracing_subscriber::fmt()
        .with_env_filter(&log_level)
        .with_target(false)
        .init();
    
    tracing::info!(
        service = "venta-libre-api",
        version = env!("CARGO_PKG_VERSION"),
        environment = %env,
        log_level = %log_level,
        "🚀 Sistema de logging inicializado (versión simple)"
    );
    
    Ok(())
}
    
    // Función para logs estructurados de requests
    pub fn log_request(
        method: &str,
        path: &str,
        status: u16,
        duration_ms: u64,
        user_id: Option<i32>,
        request_id: &str,
    ) {
        let log_data = json!({
            "event": "http_request",
            "method": method,
            "path": path,
            "status": status,
            "duration_ms": duration_ms,
            "user_id": user_id,
            "request_id": request_id,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });
        
        match status {
            200..=299 => tracing::info!(
                method = %method,
                path = %path,
                status = %status,
                duration_ms = %duration_ms,
                user_id = ?user_id,
                request_id = %request_id,
                "✅ Request exitoso"
            ),
            400..=499 => tracing::warn!(
                method = %method,
                path = %path,
                status = %status,
                duration_ms = %duration_ms,
                user_id = ?user_id,
                request_id = %request_id,
                "⚠️ Error de cliente"
            ),
            500..=599 => tracing::error!(
                method = %method,
                path = %path,
                status = %status,
                duration_ms = %duration_ms,
                user_id = ?user_id,
                request_id = %request_id,
                "🚨 Error del servidor"
            ),
            _ => tracing::debug!(
                method = %method,
                path = %path,
                status = %status,
                duration_ms = %duration_ms,
                user_id = ?user_id,
                request_id = %request_id,
                "📝 Request"
            ),
        }
    }
    
    // Log de errores con contexto
    pub fn log_error(
        error: &dyn std::error::Error,
        context: &str,
        request_id: Option<&str>,
        user_id: Option<i32>,
    ) {
        tracing::error!(
            error = %error,
            context = %context,
            request_id = ?request_id,
            user_id = ?user_id,
            error_chain = ?Self::get_error_chain(error),
            "🚨 Error del sistema"
        );
    }
    
    // Función para obtener cadena de errores
    fn get_error_chain(error: &dyn std::error::Error) -> Vec<String> {
        let mut chain = vec![error.to_string()];
        let mut source = error.source();
        
        while let Some(err) = source {
            chain.push(err.to_string());
            source = err.source();
        }
        
        chain
    }
    
    // Log de métricas de sistema
    pub fn log_system_metrics(
        cpu_usage: f32,
        memory_usage: u64,
        active_connections: usize,
        db_pool_size: u32,
    ) {
        tracing::info!(
            event = "system_metrics",
            cpu_usage = %cpu_usage,
            memory_usage_mb = %(memory_usage / 1024 / 1024),
            active_connections = %active_connections,
            db_pool_size = %db_pool_size,
            timestamp = %chrono::Utc::now().to_rfc3339(),
            "📊 Métricas del sistema"
        );
    }
    
    // Log de eventos de autenticación
    pub fn log_auth_event(
        event_type: &str,
        user_id: Option<i32>,
        email: Option<&str>,
        ip_address: Option<&str>,
        success: bool,
        request_id: &str,
    ) {
        let level = if success { "info" } else { "warn" };
        
        match success {
            true => tracing::info!(
                event = "auth_event",
                event_type = %event_type,
                user_id = ?user_id,
                email = ?email,
                ip_address = ?ip_address,
                success = %success,
                request_id = %request_id,
                "🔐 Evento de autenticación exitoso"
            ),
            false => tracing::warn!(
                event = "auth_event",
                event_type = %event_type,
                user_id = ?user_id,
                email = ?email,
                ip_address = ?ip_address,
                success = %success,
                request_id = %request_id,
                "🚨 Evento de autenticación fallido"
            ),
        }
    }
    
    // Log de eventos de base de datos
    pub fn log_db_event(
        operation: &str,
        table: &str,
        duration_ms: u64,
        affected_rows: Option<u64>,
        success: bool,
        request_id: Option<&str>,
    ) {
        match success {
            true => tracing::debug!(
                event = "db_operation",
                operation = %operation,
                table = %table,
                duration_ms = %duration_ms,
                affected_rows = ?affected_rows,
                success = %success,
                request_id = ?request_id,
                "💾 Operación de BD exitosa"
            ),
            false => tracing::error!(
                event = "db_operation",
                operation = %operation,
                table = %table,
                duration_ms = %duration_ms,
                affected_rows = ?affected_rows,
                success = %success,
                request_id = ?request_id,
                "🚨 Error en operación de BD"
            ),
        }
    }
}