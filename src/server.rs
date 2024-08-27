use std::io::Error;
use actix_web::{get, web, App, HttpServer, Responder};
use config::ConfigError;
use crate::config::AllConfig;
use thiserror::Error;
use crate::db::{DBClient, DBClientError};

#[derive(Error, Debug)]
pub enum QueryServerError {
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

impl From<ConfigError> for QueryServerError {
    fn from(value: ConfigError) -> Self {
        QueryServerError::DBError(value.to_string())
    }
}

impl From<Error> for QueryServerError {
    fn from(value: Error) -> Self {
        QueryServerError::DBError(value.to_string())
    }
}

pub struct QueryServer {
    cfg: AllConfig,
    db: DBClient,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

impl QueryServer {
    pub fn new(cfg: AllConfig) -> Result<Self, QueryServerError> {
        let db_client = DBClient::new(cfg.database().clone().to_owned())?;
        Ok(QueryServer{cfg, db: db_client})
    }

    pub async fn run(&self) -> Result<(), QueryServerError> {
        let addr = format!("{}:{}", self.cfg.http_server().host(), self.cfg.http_server().port());
        let routes = || {
            App::new()
                .service(greet)
        };
        let server = HttpServer::new(routes)
            .bind(addr.clone());
        println!("running server at http://{}", addr.clone());
        server?.run().await?;
        Ok(())
    }
}