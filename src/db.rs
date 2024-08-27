use sqlx::postgres::PgPoolOptions;
use thiserror::Error;
use crate::config::DatabaseConfig;

#[derive(Error, Debug)]
pub enum DBClientError {
    #[error("Network error occurred. {0}")]
    NetworkError(String),
}


pub struct DBClient {
    cfg: DatabaseConfig
}

impl DBClient {
    pub fn new(cfg: DatabaseConfig) -> Result<Self, DBClientError> {
       Ok(DBClient {cfg})
    }

    pub async fn connect(&self) -> Result<&Self, sqlx::Error> {
        println!("{}", self.cfg.dsn());
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(self.cfg.dsn().as_str()).await?;
        Ok(self)
    }
}