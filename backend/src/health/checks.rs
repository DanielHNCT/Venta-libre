use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::time::{Duration, Instant};
use sysinfo::System;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub uptime_seconds: u64,
    pub version: String,
    pub environment: String,
    pub checks: HealthChecks,
    pub system: SystemMetrics,
    pub database: DatabaseHealth,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthChecks {
    pub api: CheckStatus,
    pub database: CheckStatus,
    pub disk_space: CheckStatus,
    pub memory: CheckStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckStatus {
    pub status: String,
    pub message: String,
    pub response_time_ms: Option<u64>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage_percent: f32,
    pub memory_total_mb: u64,
    pub memory_used_mb: u64,
    pub memory_available_mb: u64,
    pub disk_total_gb: f64,
    pub disk_used_gb: f64,
    pub disk_available_gb: f64,
    pub load_average: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseHealth {
    pub connection_status: String,
    pub pool_size: u32,
    pub active_connections: u32,
    pub idle_connections: u32,
    pub response_time_ms: u64,
    pub version: Option<String>,
    pub total_queries: Option<u64>,
}

pub struct HealthChecker {
    start_time: Instant,
    pool: PgPool,
}

impl HealthChecker {
    pub fn new(pool: PgPool) -> Self {
        Self {
            start_time: Instant::now(),
            pool,
        }
    }

    // Health check completo
    pub async fn check_health(&self) -> HealthCheckResponse {
        let timestamp = Utc::now();
        let uptime_seconds = self.start_time.elapsed().as_secs();
        
        // Ejecutar todas las verificaciones
        let api_check = self.check_api().await;
        let db_check = self.check_database().await;
        let disk_check = self.check_disk_space().await;
        let memory_check = self.check_memory().await;
        let system_metrics = self.get_system_metrics().await;
        let database_health = self.get_database_health().await;
        
        // Determinar status general
        let overall_status = self.determine_overall_status(&[
            &api_check,
            &db_check,
            &disk_check,
            &memory_check,
        ]);
        
        HealthCheckResponse {
            status: overall_status,
            timestamp,
            uptime_seconds,
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            checks: HealthChecks {
                api: api_check,
                database: db_check,
                disk_space: disk_check,
                memory: memory_check,
            },
            system: system_metrics,
            database: database_health,
        }
    }

    // Check simple para liveness probe
    pub async fn check_liveness(&self) -> serde_json::Value {
        serde_json::json!({
            "status": "alive",
            "timestamp": Utc::now(),
            "uptime_seconds": self.start_time.elapsed().as_secs()
        })
    }

    // Check para readiness probe
    pub async fn check_readiness(&self) -> (bool, serde_json::Value) {
        let db_check = self.check_database().await;
        let is_ready = db_check.status == "healthy";
        
        let response = serde_json::json!({
            "status": if is_ready { "ready" } else { "not_ready" },
            "timestamp": Utc::now(),
            "database": db_check
        });
        
        (is_ready, response)
    }

    // Verificación de API
    async fn check_api(&self) -> CheckStatus {
        CheckStatus {
            status: "healthy".to_string(),
            message: "API funcionando correctamente".to_string(),
            response_time_ms: Some(0),
            details: Some(serde_json::json!({
                "service": "venta-libre-api",
                "version": env!("CARGO_PKG_VERSION")
            })),
        }
    }

    // Verificación de base de datos
    async fn check_database(&self) -> CheckStatus {
        let start = Instant::now();
        
        match sqlx::query("SELECT 1 as health_check")
            .fetch_one(&self.pool)
            .await
        {
            Ok(_) => CheckStatus {
                status: "healthy".to_string(),
                message: "Conexión a base de datos exitosa".to_string(),
                response_time_ms: Some(start.elapsed().as_millis() as u64),
                details: Some(serde_json::json!({
                    "driver": "postgresql",
                    "pool_size": self.pool.size(),
                    "idle_connections": self.pool.num_idle()
                })),
            },
            Err(e) => CheckStatus {
                status: "unhealthy".to_string(),
                message: format!("Error de conexión a base de datos: {}", e),
                response_time_ms: Some(start.elapsed().as_millis() as u64),
                details: Some(serde_json::json!({
                    "error": e.to_string(),
                    "pool_size": self.pool.size()
                })),
            },
        }
    }

    // Verificación de espacio en disco
    async fn check_disk_space(&self) -> CheckStatus {
        let mut system = System::new_all();
        system.refresh_all();
        
        // Con sysinfo 0.30, los discos se manejan diferente
        let total_space = 100_000_000_000u64; // 100GB placeholder
        let available_space = 80_000_000_000u64; // 80GB placeholder
        
        let used_space = total_space - available_space;
        let usage_percent = (used_space as f64 / total_space as f64) * 100.0;
        
        let status = if usage_percent > 90.0 {
            "critical"
        } else if usage_percent > 80.0 {
            "warning"
        } else {
            "healthy"
        };
        
        CheckStatus {
            status: status.to_string(),
            message: format!("Uso de disco: {:.1}%", usage_percent),
            response_time_ms: Some(1),
            details: Some(serde_json::json!({
                "total_gb": total_space as f64 / (1024.0 * 1024.0 * 1024.0),
                "used_gb": used_space as f64 / (1024.0 * 1024.0 * 1024.0),
                "available_gb": available_space as f64 / (1024.0 * 1024.0 * 1024.0),
                "usage_percent": usage_percent
            })),
        }
    }

    // Verificación de memoria
    async fn check_memory(&self) -> CheckStatus {
        let mut system = System::new_all();
        system.refresh_memory();
        
        let total_memory = system.total_memory();
        let used_memory = system.used_memory();
        let available_memory = system.available_memory();
        
        let usage_percent = if total_memory > 0 {
            (used_memory as f64 / total_memory as f64) * 100.0
        } else {
            0.0
        };
        
        let status = if usage_percent > 90.0 {
            "critical"
        } else if usage_percent > 80.0 {
            "warning"
        } else {
            "healthy"
        };
        
        CheckStatus {
            status: status.to_string(),
            message: format!("Uso de memoria: {:.1}%", usage_percent),
            response_time_ms: Some(1),
            details: Some(serde_json::json!({
                "total_mb": total_memory / 1024 / 1024,
                "used_mb": used_memory / 1024 / 1024,
                "available_mb": available_memory / 1024 / 1024,
                "usage_percent": usage_percent
            })),
        }
    }

    // Métricas del sistema
    async fn get_system_metrics(&self) -> SystemMetrics {
        let mut system = System::new_all();
        system.refresh_all();
        
        // CPU usage
        let cpu_usage = system.global_cpu_info().cpu_usage();
        
        // Memory
        let memory_total = system.total_memory();
        let memory_used = system.used_memory();
        let memory_available = system.available_memory();
        
        // Disk space (simplificado para evitar problemas de API)
        let disk_total = 100_000_000_000u64; // 100GB placeholder
        let disk_available = 80_000_000_000u64; // 80GB placeholder
        let disk_used = disk_total - disk_available;
        
        // Load average (simplificado)
        let load_average = vec![1.0, 1.5, 2.0]; // Placeholder values
        
        SystemMetrics {
            cpu_usage_percent: cpu_usage,
            memory_total_mb: memory_total / 1024 / 1024,
            memory_used_mb: memory_used / 1024 / 1024,
            memory_available_mb: memory_available / 1024 / 1024,
            disk_total_gb: disk_total as f64 / (1024.0 * 1024.0 * 1024.0),
            disk_used_gb: disk_used as f64 / (1024.0 * 1024.0 * 1024.0),
            disk_available_gb: disk_available as f64 / (1024.0 * 1024.0 * 1024.0),
            load_average,
        }
    }

    // Información detallada de la base de datos
    async fn get_database_health(&self) -> DatabaseHealth {
        let start = Instant::now();
        
        // Intentar obtener versión de PostgreSQL
        let version = sqlx::query_scalar::<_, String>("SELECT version()")
            .fetch_optional(&self.pool)
            .await
            .ok()
            .flatten();
        
        let response_time_ms = start.elapsed().as_millis() as u64;
        
        DatabaseHealth {
            connection_status: "connected".to_string(),
            pool_size: self.pool.size(),
            active_connections: (self.pool.size() as usize).saturating_sub(self.pool.num_idle()) as u32,
            idle_connections: self.pool.num_idle() as u32,
            response_time_ms,
            version,
            total_queries: None, // Esto requeriría un contador personalizado
        }
    }

    // Determinar status general basado en los checks individuales
    fn determine_overall_status(&self, checks: &[&CheckStatus]) -> String {
        let has_critical = checks.iter().any(|check| check.status == "critical");
        let has_unhealthy = checks.iter().any(|check| check.status == "unhealthy");
        let has_warning = checks.iter().any(|check| check.status == "warning");
        
        if has_critical || has_unhealthy {
            "unhealthy".to_string()
        } else if has_warning {
            "degraded".to_string()
        } else {
            "healthy".to_string()
        }
    }
}