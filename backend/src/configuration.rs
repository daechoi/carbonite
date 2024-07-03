use config::{Config, ConfigError, File};
use serde::Deserialize;

/// Struct to hold the configuration settings
#[derive(Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn from_file(file_name: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(file_name))
            .build()?;

        config.try_deserialize()
    }
}
#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub port: u16,
    pub host: String,
    pub database_user: String,
    pub database_password: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database_user, self.database_password, self.host, self.port, self.database_name
        )
    }
}
