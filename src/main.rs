use clap::Parser;
use crate::config::{AllConfig, Cli, Commands};
use crate::server::{QueryServer, QueryServerError};

mod config;
mod server;
mod db;

async fn start_server(path: &String) -> Result<(), QueryServerError> {
    let all_config = AllConfig::new(path.to_string())?;
    let server = QueryServer::new(all_config)?;
    server.run().await
}

#[actix_web::main]
async fn main() {
    let args = Cli::parse();

    match args.command() {
        Commands::Start(args) => {
            let result = start_server(args.path()).await;
            println!("{:?}", result)
        }
    }
}
