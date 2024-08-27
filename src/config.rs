use clap::{Parser, Subcommand, Args};
use config::{Config, ConfigError, File};
use getset::Getters;
use serde::Deserialize;

#[derive(Getters)]
#[get = "pub"]
#[derive(Debug, Deserialize, Clone)]
pub struct AllConfig {
    http_server: HttpServerConfig,
    database: DatabaseConfig,
}

#[derive(Getters)]
#[get = "pub"]
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    db_type: String,
    host: String,
    port: u32,
    db_name: String,
    username: String,
    password: String,
}

#[derive(Getters)]
#[get = "pub"]
#[derive(Debug, Deserialize, Clone)]
pub struct HttpServerConfig {
    host: String,
    port: u32,
}

impl AllConfig {
    pub fn new(path: String) -> Result<AllConfig, ConfigError> {
        let settings = Config::builder()
            // File::with_name(..) is shorthand for File::from(Path::new(..))
            .add_source(File::with_name(path.as_str()))
            .build();

        settings?.try_deserialize()
    }
}

impl DatabaseConfig {
    pub fn dsn(&self) -> String {
        format!("{}://{}:{}@{}:{}/{}", self.db_type, self.username, self.password, self.host, self.port, self.db_name)
    }
}

#[derive(Getters)]
#[get = "pub"]
#[derive(Debug, Parser)] // requires `derive` feature
#[command(about = "start a http query server", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "start")]
    /// start the http query server
    Start(StartServerArgs)
}


#[derive(Getters)]
#[get = "pub"]
#[derive(Args, Debug)]
pub struct StartServerArgs {
    #[arg(short, long, default_value = "/etc/http_server.yaml")]
    path: String
}