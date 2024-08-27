use thiserror::Error;
use crate::config::DatabaseConfig;

#[derive(Error, Debug)]
pub enum DBClientError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Network error occurred")]
    NetworkError,
}


pub struct DBClient {
    cfg: DatabaseConfig
}

impl DBClient {
    pub fn new(cfg: DatabaseConfig) -> Result<Self, DBClientError> {
       Ok(DBClient {cfg})
    }
}