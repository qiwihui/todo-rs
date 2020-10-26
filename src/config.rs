use crate::errors::Error;
pub use config::ConfigError;
use deadpool_postgres;
use deadpool_postgres::Pool;
use serde::Deserialize;
use tokio_postgres::NoTls;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}

pub fn init_pool(config: &Config) -> Result<Pool, Error> {
    match config.pg.create_pool(NoTls) {
        Ok(pool) => Ok(pool),
        Err(_) => Err(Error::ConfigError("config error".into())),
    }
}
