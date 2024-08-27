use std::error::Error;
use crate::config::AllConfig;
use thiserror::Error;
use crate::db::{DBClient, DBClientError};

#[derive(Error, Debug)]
enum QueryServerError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Network error occurred")]
    NetworkError,
    #[error("database error occurred: {0}")]
    DBError(String),
}

impl From<DBClientError> for QueryServerError {
    fn from(value: DBClientError) -> Self {
       QueryServerError::DBError(value.to_string())
    }
}

pub struct QueryServer {
    cfg: AllConfig,
    db: DBClient,
}

impl QueryServer {
    pub fn new(cfg: AllConfig) -> Result<Self, QueryServerError> {
        let db_client = DBClient::new(cfg.database().clone().to_owned())?;
        Ok(QueryServer{cfg, db: db_client})
    }

    pub fn run(&self) -> Result<(), QueryServerError> {
        Ok(())
    }
}