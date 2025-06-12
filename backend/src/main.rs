mod auth;
mod database;
mod handlers;
mod health;
mod logging;
mod metrics;
mod models;
mod routes;

use axum::{
    http::{HeaderValue, Method},
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    cors::CorsLayer,
    request_id::{MakeRequestId, RequestId, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use uuid::Uuid;

use crate::database::create_pool;
use crate::health::HealthChecker;
use crate::logging::{logging_middleware, slow_request_middleware, Logger};
use crate::metrics::MetricsCollector;

// Generador de Request ID personalizado
#[derive(Clone, Default)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _request: &axum::http::Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string().parse().ok()?;
        Some(RequestId::new(request_id))
    }
}

// Handler para ruta raíz
async fn root_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "service": "venta-libre-api",
        "version": env!("CARGO_PKG_VERSION"),
        "status": "running",
        "environment": std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
        "timestamp": chrono::Utc::now(),
        "endpoints": {
            "health": "/health",
            "metrics": "/metrics/public",
            "api": "/api/v1",
            "docs": "https://github.com/tu-usuario/venta-libre"
        }
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar sistema de logging profesional
    Logger::init()?;
    
    tracing::info!(
        service = "venta-libre-api",
        version = env!("CARGO_PKG_VERSION"),
        "🚀 Iniciando Venta Libre Bolivia API"
    );

    // Cargar variables de entorno
    dotenv::dotenv().ok();
    let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    
    // Crear pool de conexiones a DB
    tracing::info!("📊 Conectando a base de datos...");
    let pool = create_pool().await
        .map_err(|e| {
            tracing::error!(error = %e, "🚨 Error conectando a base de datos");
            e
        })?;
    tracing::info!("✅ Conexión a base de datos establecida");

    // Inicializar sistemas de monitoreo
    let health_checker = Arc::new(HealthChecker::new(pool.clone()));
    let metrics_collector = Arc::new(MetricsCollector::new(10000)); // Máximo 10k métricas en memoria
    
    tracing::info!("📈 Sistemas de monitoreo inicializados");

    // Configurar CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
        ])
        .allow_origin("http://localhost:5173".parse::<HeaderValue>()?);

    // Crear middleware stack profesional - ORDEN CORREGIDO
    let middleware_stack = ServiceBuilder::new()
        // Timeout global para prevenir requests colgados
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        // Manejo de panics sin derribar el servidor
        .layer(CatchPanicLayer::custom(|_| {
            tracing::error!("💥 Panic capturado, servidor sigue funcionando");
            axum::http::Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .body(axum::body::Body::from("🚨 Error interno del servidor"))
                .unwrap()
        }))
        // Request ID único para trazabilidad
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        // CORS
        .layer(cors)
        // Middleware de logging personalizado PRIMERO
        .layer(middleware::from_fn(logging_middleware))
        // Middleware para detectar requests lentos
        .layer(middleware::from_fn(slow_request_middleware))
        // Tracing automático de requests DESPUÉS
        .layer(TraceLayer::new_for_http());

    // Crear rutas principales de la API
    let api_routes = routes::create_routes();

    // Crear rutas de health y métricas (sin auth)
    let health_routes = Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/health/live", get(handlers::health::liveness_check))
        .route("/health/ready", get(handlers::health::readiness_check))
        .route("/status", get(handlers::health::status_check))
        .route("/info", get(handlers::health::server_info))
        .with_state(health_checker.clone());

    let metrics_routes = Router::new()
        .route("/metrics", get(handlers::metrics::get_metrics))
        .route("/metrics/public", get(handlers::metrics::get_public_metrics))
        .route("/metrics/endpoints/top", get(handlers::metrics::get_top_endpoints))
        .route("/metrics/endpoints/slow", get(handlers::metrics::get_slowest_endpoints))
        .route("/metrics/status-distribution", get(handlers::metrics::get_status_distribution))
        .route("/metrics/hourly", get(handlers::metrics::get_hourly_stats))
        .route("/metrics/endpoint/:method/:path", get(handlers::metrics::get_endpoint_metrics))
        .with_state(metrics_collector.clone());

    // Configurar middleware para registrar métricas
    let metrics_middleware = {
        let collector = metrics_collector.clone();
        middleware::from_fn(move |req: axum::extract::Request, next: axum::middleware::Next| {
            let collector = collector.clone();
            async move {
                let start = std::time::Instant::now();
                let method = req.method().to_string();
                let path = req.uri().path().to_string();
                
                let response = next.run(req).await;
                
                let duration_ms = start.elapsed().as_millis() as u64;
                let status = response.status().as_u16();
                
                // Extraer user_id si existe
                let user_id = response.extensions().get::<crate::auth::middleware::AuthUser>()
                    .map(|auth| auth.user.id);
                
                // Registrar métrica
                collector.record_request(method, path, status, duration_ms, user_id);
                
                response
            }
        })
    };

    // Construir aplicación completa
let app = Router::new()
    // Rutas principales de la API
    .nest("/api/v1", api_routes)
    // Rutas de monitoreo y salud
    .merge(health_routes)
    .merge(metrics_routes)
    // Ruta raíz para verificación básica
    .route("/", get(root_handler))
    // Aplicar middleware de métricas a toda la app
    .layer(metrics_middleware)
    // AGREGAR ESTA LÍNEA: Aplicar logging a toda la app
    .layer(middleware_stack)
    // State compartido
    .with_state(pool);

    // Configurar dirección y puerto
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let bind_address = format!("{}:{}", host, port);

    // Inicializar servidor
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    let local_addr = listener.local_addr()?;

    // Configurar tarea de limpieza de métricas (cada 1 hora)
    let cleanup_collector = metrics_collector.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(3600)); // 1 hora
        loop {
            interval.tick().await;
            cleanup_collector.cleanup_old_metrics(Duration::from_secs(86400)); // 24 horas
        }
    });

    // Configurar task de logging de métricas del sistema (cada 5 minutos)
    let system_metrics_checker = health_checker.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(300)); // 5 minutos
        loop {
            interval.tick().await;
            let health = system_metrics_checker.check_health().await;
            Logger::log_system_metrics(
                health.system.cpu_usage_percent,
                health.system.memory_used_mb * 1024 * 1024, // Convertir a bytes
                0, // active_connections - podríamos implementar esto
                health.database.pool_size,
            );
        }
    });

    // Logs de inicio
    tracing::info!(
        bind_address = %bind_address,
        local_address = %local_addr,
        environment = %environment,
        version = env!("CARGO_PKG_VERSION"),
        "🚀 Servidor iniciado exitosamente"
    );

    tracing::info!("📋 Endpoints disponibles:");
    tracing::info!("   🏥 Health Check: http://{}/health", local_addr);
    tracing::info!("   📊 Métricas Públicas: http://{}/metrics/public", local_addr);
    tracing::info!("   🔐 API Auth: http://{}/api/v1/auth/*", local_addr);
    tracing::info!("   👥 API Users: http://{}/api/v1/users/*", local_addr);
    tracing::info!("   ℹ️  Info del Servidor: http://{}/info", local_addr);

    if environment == "development" {
        tracing::info!("🔧 Modo desarrollo - Logs detallados habilitados");
        tracing::info!("📈 Dashboard de métricas (admin): http://{}/metrics", local_addr);
    }

    // Iniciar servidor
    tracing::info!("🎯 Servidor listo para recibir conexiones");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "🚨 Error fatal del servidor");
        e
    })?;

    Ok(())
}