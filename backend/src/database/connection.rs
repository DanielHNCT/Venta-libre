use sqlx::{Pool, Postgres, PgPool};
use std::env;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    dotenv::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = PgPool::connect(&database_url).await?;
    
    Ok(pool)
}