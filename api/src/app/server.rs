use super::{config::Config, routes::config_routes};
use crate::app::config::ConfigError;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub struct Server {
    pool: PgPool,
    config: Config,
}

impl Server {
    pub async fn new() -> Result<Self, ConfigError> {
        let config = Config::load().await?;
        let pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(10))
            .connect(&config.db_url())
            .await
            .expect("Failed to connect to database");

        Ok(Self { pool, config })
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let Self { pool, config } = self;
        let app = config_routes(&pool);
        let addr: SocketAddr = format!("{}:{}", config.srv_host, config.srv_port)
            .parse()
            .expect("Invalid address format");
        let listener = TcpListener::bind(addr).await?;

        println!("Server running at http://{}", addr);
        axum::serve(listener, app.into_make_service()).await?;

        Ok(())
    }
}
