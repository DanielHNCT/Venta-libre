use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMetric {
    pub method: String,
    pub path: String,
    pub status: u16,
    pub duration_ms: u64,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointStats {
    pub path: String,
    pub method: String,
    pub total_requests: u64,
    pub success_requests: u64,
    pub error_requests: u64,
    pub avg_response_time_ms: f64,
    pub min_response_time_ms: u64,
    pub max_response_time_ms: u64,
    pub last_accessed: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub timestamp: DateTime<Utc>,
    pub uptime_seconds: u64,
    pub total_requests: u64,
    pub requests_per_minute: f64,
    pub avg_response_time_ms: f64,
    pub error_rate_percent: f64,
    pub active_users: u64,
    pub most_used_endpoints: Vec<EndpointStats>,
    pub slowest_endpoints: Vec<EndpointStats>,
    pub error_endpoints: Vec<EndpointStats>,
    pub status_code_distribution: HashMap<u16, u64>,
    pub hourly_stats: Vec<HourlyStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyStats {
    pub hour: DateTime<Utc>,
    pub requests: u64,
    pub avg_response_time_ms: f64,
    pub error_rate_percent: f64,
}

pub struct MetricsCollector {
    start_time: Instant,
    metrics: Arc<RwLock<Vec<RequestMetric>>>,
    endpoint_stats: Arc<RwLock<HashMap<String, EndpointStats>>>,
    max_metrics: usize,
}

impl MetricsCollector {
    pub fn new(max_metrics: usize) -> Self {
        Self {
            start_time: Instant::now(),
            metrics: Arc::new(RwLock::new(Vec::new())),
            endpoint_stats: Arc::new(RwLock::new(HashMap::new())),
            max_metrics,
        }
    }

    // Registrar una nueva métrica de request
    pub fn record_request(
        &self,
        method: String,
        path: String,
        status: u16,
        duration_ms: u64,
        user_id: Option<i32>,
    ) {
        let metric = RequestMetric {
            method: method.clone(),
            path: path.clone(),
            status,
            duration_ms,
            timestamp: Utc::now(),
            user_id,
        };

        // Actualizar métricas globales
        {
            let mut metrics = self.metrics.write().unwrap();
            metrics.push(metric);
            
            // Limitar el número de métricas en memoria
            if metrics.len() > self.max_metrics {
                let drain_count = metrics.len() - self.max_metrics;
                metrics.drain(0..drain_count);
            }
        }

        // Actualizar estadísticas por endpoint
        self.update_endpoint_stats(method, path, status, duration_ms);
    }

    // Actualizar estadísticas por endpoint
    fn update_endpoint_stats(&self, method: String, path: String, status: u16, duration_ms: u64) {
        let key = format!("{} {}", method, path);
        let mut stats = self.endpoint_stats.write().unwrap();
        
        let endpoint_stat = stats.entry(key).or_insert(EndpointStats {
            path: path.clone(),
            method: method.clone(),
            total_requests: 0,
            success_requests: 0,
            error_requests: 0,
            avg_response_time_ms: 0.0,
            min_response_time_ms: u64::MAX,
            max_response_time_ms: 0,
            last_accessed: Utc::now(),
        });

        // Actualizar contadores
        endpoint_stat.total_requests += 1;
        endpoint_stat.last_accessed = Utc::now();

        if status >= 200 && status < 400 {
            endpoint_stat.success_requests += 1;
        } else {
            endpoint_stat.error_requests += 1;
        }

        // Actualizar tiempos de respuesta
        endpoint_stat.min_response_time_ms = endpoint_stat.min_response_time_ms.min(duration_ms);
        endpoint_stat.max_response_time_ms = endpoint_stat.max_response_time_ms.max(duration_ms);
        
        // Calcular promedio móvil simple
        let total_time = endpoint_stat.avg_response_time_ms * (endpoint_stat.total_requests - 1) as f64;
        endpoint_stat.avg_response_time_ms = (total_time + duration_ms as f64) / endpoint_stat.total_requests as f64;
    }

    // Obtener snapshot completo de métricas
    pub fn get_metrics_snapshot(&self) -> MetricsSnapshot {
        let metrics = self.metrics.read().unwrap();
        let endpoint_stats = self.endpoint_stats.read().unwrap();
        
        let uptime_seconds = self.start_time.elapsed().as_secs();
        let total_requests = metrics.len() as u64;
        
        // Calcular requests por minuto (últimos 60 segundos)
        let one_minute_ago = Utc::now() - chrono::Duration::minutes(1);
        let recent_requests = metrics
            .iter()
            .filter(|m| m.timestamp > one_minute_ago)
            .count() as f64;
        
        // Calcular tiempo de respuesta promedio
        let avg_response_time_ms = if !metrics.is_empty() {
            metrics.iter().map(|m| m.duration_ms as f64).sum::<f64>() / metrics.len() as f64
        } else {
            0.0
        };
        
        // Calcular tasa de error
        let error_requests = metrics.iter().filter(|m| m.status >= 400).count();
        let error_rate_percent = if total_requests > 0 {
            (error_requests as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };
        
        // Contar usuarios activos (últimos 5 minutos)
        let five_minutes_ago = Utc::now() - chrono::Duration::minutes(5);
        let active_users = metrics
            .iter()
            .filter(|m| m.timestamp > five_minutes_ago && m.user_id.is_some())
            .map(|m| m.user_id.unwrap())
            .collect::<std::collections::HashSet<_>>()
            .len() as u64;
        
        // Top endpoints más usados
        let mut most_used: Vec<EndpointStats> = endpoint_stats.values().cloned().collect();
        most_used.sort_by(|a, b| b.total_requests.cmp(&a.total_requests));
        most_used.truncate(10);
        
        // Endpoints más lentos
        let mut slowest: Vec<EndpointStats> = endpoint_stats.values().cloned().collect();
        slowest.sort_by(|a, b| b.avg_response_time_ms.partial_cmp(&a.avg_response_time_ms).unwrap());
        slowest.truncate(10);
        
        // Endpoints con más errores
        let mut error_endpoints: Vec<EndpointStats> = endpoint_stats
            .values()
            .filter(|stat| stat.error_requests > 0)
            .cloned()
            .collect();
        error_endpoints.sort_by(|a, b| b.error_requests.cmp(&a.error_requests));
        error_endpoints.truncate(10);
        
        // Distribución de códigos de estado
        let mut status_distribution = HashMap::new();
        for metric in metrics.iter() {
            *status_distribution.entry(metric.status).or_insert(0) += 1;
        }
        
        // Estadísticas por hora (últimas 24 horas)
        let hourly_stats = self.calculate_hourly_stats(&metrics);
        
        MetricsSnapshot {
            timestamp: Utc::now(),
            uptime_seconds,
            total_requests,
            requests_per_minute: recent_requests,
            avg_response_time_ms,
            error_rate_percent,
            active_users,
            most_used_endpoints: most_used,
            slowest_endpoints: slowest,
            error_endpoints,
            status_code_distribution: status_distribution,
            hourly_stats,
        }
    }

    // Calcular estadísticas por hora
    fn calculate_hourly_stats(&self, metrics: &[RequestMetric]) -> Vec<HourlyStats> {
        let mut hourly_map: HashMap<i64, Vec<&RequestMetric>> = HashMap::new();
        
        // Agrupar métricas por hora
        for metric in metrics.iter() {
            let hour_timestamp = metric.timestamp.timestamp() / 3600 * 3600;
            hourly_map.entry(hour_timestamp).or_default().push(metric);
        }
        
        // Calcular estadísticas para cada hora
        let mut hourly_stats: Vec<HourlyStats> = hourly_map
            .into_iter()
            .map(|(hour_timestamp, hour_metrics)| {
                let requests = hour_metrics.len() as u64;
                let avg_response_time_ms = if !hour_metrics.is_empty() {
                    hour_metrics.iter().map(|m| m.duration_ms as f64).sum::<f64>() / hour_metrics.len() as f64
                } else {
                    0.0
                };
                
                let error_count = hour_metrics.iter().filter(|m| m.status >= 400).count();
                let error_rate_percent = if requests > 0 {
                    (error_count as f64 / requests as f64) * 100.0
                } else {
                    0.0
                };
                
                HourlyStats {
                    hour: DateTime::from_timestamp(hour_timestamp, 0).unwrap_or(Utc::now()),
                    requests,
                    avg_response_time_ms,
                    error_rate_percent,
                }
            })
            .collect();
        
        // Ordenar por hora y tomar solo las últimas 24 horas
        hourly_stats.sort_by_key(|stat| stat.hour);
        hourly_stats.into_iter().rev().take(24).collect()
    }

    // Obtener métricas de un endpoint específico
    pub fn get_endpoint_metrics(&self, method: &str, path: &str) -> Option<EndpointStats> {
        let key = format!("{} {}", method, path);
        self.endpoint_stats.read().unwrap().get(&key).cloned()
    }

    // Limpiar métricas antiguas (para ser llamado periódicamente)
    pub fn cleanup_old_metrics(&self, older_than: Duration) {
        let cutoff_time = Utc::now() - chrono::Duration::from_std(older_than).unwrap();
        
        let mut metrics = self.metrics.write().unwrap();
        metrics.retain(|metric| metric.timestamp > cutoff_time);
        
        tracing::info!(
            event = "metrics_cleanup",
            metrics_retained = metrics.len(),
            cutoff_time = %cutoff_time,
            "🧹 Limpieza de métricas antiguas"
        );
    }
}