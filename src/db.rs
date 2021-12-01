#[cfg(feature = "sqlite")]
pub use sqlx::sqlite::{SqlitePool as Pool, SqlitePoolOptions as PoolOptions};

#[cfg(feature = "postgres")]
pub use sqlx::postgres::{PgPool as Pool, PgPoolOptions as PoolOptions};

#[cfg(feature = "mysql")]
pub use sqlx::mysql::{MySqlPool as Pool, MySqlPoolOptions as PoolOptions};

pub async fn connect(max_connections: Option<u32>, db_uri: &str) -> sqlx::Result<Pool> {
    let mut pool_opts = PoolOptions::new();
    if let Some(c) = max_connections {
        pool_opts = pool_opts.max_connections(c);
    }

    pool_opts.connect(db_uri).await
}
