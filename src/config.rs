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