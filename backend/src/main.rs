mod database;
mod handlers;
mod models;
mod routes;

use axum::{
    http::Method,
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;
use crate::database::create_pool;

#[tokio::main]
async fn main() {
    // Inicializar logging
    tracing_subscriber::fmt::init();

    // Crear pool de conexiones a DB
    let pool = create_pool().await
        .expect("Failed to create database pool");

    // Configurar CORS para tu frontend
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap());

    // Crear el router principal
    let app = Router::new()
        .nest("/api/v1", routes::create_routes())
        .layer(cors)
        .with_state(pool);

    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();
}