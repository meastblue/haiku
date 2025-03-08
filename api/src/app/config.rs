use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to laod .env file")]
    DotenvError,
    #[error("Environement variable `{0}` is missing")]
    MissingEnvVar(String),
    #[error("Invalid port number")]
    InvalidPort(#[from] std::num::ParseIntError),
}

pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub srv_host: String,
    pub srv_port: u16,
}

impl Config {
    pub async fn load() -> Result<Self, ConfigError> {
        dotenvy::dotenv().map_err(|_| ConfigError::DotenvError)?;

        fn get_env_var(key: &str) -> Result<String, ConfigError> {
            env::var(key).map_err(|_| ConfigError::MissingEnvVar(key.to_string()))
        }

        let db_host = get_env_var("DB_HOST")?;
        let db_port = get_env_var("DB_PORT")?.parse::<u16>()?;
        let db_name = get_env_var("DB_NAME")?;
        let db_user = get_env_var("DB_USER")?;
        let db_password = get_env_var("DB_PASSWORD")?;
        let srv_host = get_env_var("SRV_HOST")?;
        let srv_port = get_env_var("SRV_PORT")?.parse::<u16>()?;

        Ok(Self {
            db_host,
            db_port,
            db_name,
            db_user,
            db_password,
            srv_host,
            srv_port,
        })
    }

    pub fn db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user, self.db_password, self.db_host, self.db_port, self.db_name
        )
    }
}
