use config::{Config, ConfigError, File};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            _ => Err("Unknown environment".into()),
        }
    }
}
/// Struct to hold the configuration settings
#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

impl Settings {
    pub fn from_file() -> Result<Self, ConfigError> {
        let config_directory =
            std::env::current_dir().expect("Failed to determine current directory1");
        let config_directory = &config_directory.join("config");

        let config_builder =
            Config::builder().add_source(File::from(config_directory.join("base")));

        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT");

        let config = config_builder
            .add_source(File::from(config_directory.join(environment.as_str())))
            .build()?;

        config.try_deserialize()
    }
}
#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub port: u16,
    pub host: String,
    pub database_user: String,
    pub database_password: Secret<String>,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database_user,
            self.database_password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/postgres",
            self.database_user,
            self.database_password.expose_secret(),
            self.host,
            self.port
        ))
    }
}
