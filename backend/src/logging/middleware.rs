use axum::{
    extract::{Request, ConnectInfo},
    middleware::Next,
    response::Response,
    http::{HeaderMap, StatusCode},
};
use std::time::Instant;
use uuid::Uuid;
use std::net::SocketAddr;
use crate::logging::logger::Logger;
use crate::auth::middleware::AuthUser;

// Extension para request ID único
#[derive(Clone)]
pub struct RequestId(pub String);

// Extension para métricas de request
#[derive(Clone)]
pub struct RequestMetrics {
    pub start_time: Instant,
    pub method: String,
    pub path: String,
}

// Middleware principal de logging
pub async fn logging_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    let start_time = Instant::now();
    
    // Generar ID único para el request
    let request_id = Uuid::new_v4().to_string();
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let query = request.uri().query().map(|q| q.to_string());
    
    // Obtener IP del cliente
    let client_ip = get_client_ip(&headers, &addr);
    
    // Obtener User-Agent
    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown");
    
    // Agregar request ID y métricas al request
    request.extensions_mut().insert(RequestId(request_id.clone()));
    request.extensions_mut().insert(RequestMetrics {
        start_time,
        method: method.clone(),
        path: path.clone(),
    });
    
    // Log del inicio del request
    tracing::info!(
        event = "request_start",
        request_id = %request_id,
        method = %method,
        path = %path,
        query = ?query,
        client_ip = %client_ip,
        user_agent = %user_agent,
        "🌐 Request iniciado"
    );
    
    // Ejecutar el request
    let response = next.run(request).await;
    
    // Calcular duración
    let duration = start_time.elapsed();
    let duration_ms = duration.as_millis() as u64;
    
    // Obtener status code
    let status = response.status().as_u16();
    
    // Intentar obtener user_id si existe autenticación
    let user_id = response
        .extensions()
        .get::<AuthUser>()
        .map(|auth| auth.user.id);
    
    // Log estructurado del request completo
    Logger::log_request(
        &method,
        &path,
        status,
        duration_ms,
        user_id,
        &request_id,
    );
    
    // Log adicional con más contexto
    tracing::info!(
        event = "request_complete",
        request_id = %request_id,
        method = %method,
        path = %path,
        status = %status,
        duration_ms = %duration_ms,
        client_ip = %client_ip,
        user_agent = %user_agent,
        user_id = ?user_id,
        response_size = ?get_response_size(&response),
        "✅ Request completado"
    );
    
    // Agregar headers de respuesta útiles
    let mut response = response;
    let headers = response.headers_mut();
    
    // Agregar request ID a la respuesta
    if let Ok(header_value) = request_id.parse() {
        headers.insert("x-request-id", header_value);
    }
    
    // Agregar tiempo de procesamiento
    if let Ok(header_value) = duration_ms.to_string().parse() {
        headers.insert("x-response-time-ms", header_value);
    }
    
    response
}

// Middleware para requests lentos
pub async fn slow_request_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start_time = Instant::now();
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    
    let response = next.run(request).await;
    
    let duration = start_time.elapsed();
    let duration_ms = duration.as_millis() as u64;
    
    // Log warning para requests lentos (> 1 segundo)
    if duration_ms > 1000 {
        let request_id = response
            .extensions()
            .get::<RequestId>()
            .map(|r| r.0.clone())
            .unwrap_or_else(|| "unknown".to_string());
        
        tracing::warn!(
            event = "slow_request",
            request_id = %request_id,
            method = %method,
            path = %path,
            duration_ms = %duration_ms,
            "🐌 Request lento detectado"
        );
    }
    
    response
}

// Middleware para errores y panics
pub async fn error_handling_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let request_id = request
        .extensions()
        .get::<RequestId>()
        .map(|r| r.0.clone())
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    
    // Ejecutar el request con manejo de errores
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // Esta parte necesitaría ser async, pero catch_unwind no es async-friendly
        // En su lugar, usaremos tower-http's CatchPanicLayer en main.rs
        ()
    })) {
        Ok(_) => {
            let response = next.run(request).await;
            
            // Log de errores HTTP
            let status = response.status();
            if status.is_server_error() {
                tracing::error!(
                    event = "server_error",
                    request_id = %request_id,
                    method = %method,
                    path = %path,
                    status = %status.as_u16(),
                    "🚨 Error del servidor"
                );
            } else if status.is_client_error() {
                tracing::warn!(
                    event = "client_error",
                    request_id = %request_id,
                    method = %method,
                    path = %path,
                    status = %status.as_u16(),
                    "⚠️ Error del cliente"
                );
            }
            
            Ok(response)
        }
        Err(panic_info) => {
            // Log del panic
            tracing::error!(
                event = "panic",
                request_id = %request_id,
                method = %method,
                path = %path,
                panic_info = ?panic_info,
                "💥 Panic del servidor"
            );
            
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Función auxiliar para obtener IP del cliente
fn get_client_ip(headers: &HeaderMap, addr: &SocketAddr) -> String {
    // Intentar obtener IP de headers de proxy
    if let Some(forwarded_for) = headers.get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            if let Some(first_ip) = forwarded_str.split(',').next() {
                return first_ip.trim().to_string();
            }
        }
    }
    
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(real_ip_str) = real_ip.to_str() {
            return real_ip_str.to_string();
        }
    }
    
    // Fallback a la IP de la conexión directa
    addr.ip().to_string()
}

// Función auxiliar para obtener tamaño de respuesta
fn get_response_size(response: &Response) -> Option<usize> {
    response
        .headers()
        .get("content-length")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse().ok())
}

// Función para obtener request ID de extensions
pub fn get_request_id(request: &Request) -> String {
    request
        .extensions()
        .get::<RequestId>()
        .map(|r| r.0.clone())
        .unwrap_or_else(|| "unknown".to_string())
}