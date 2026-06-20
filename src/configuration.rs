use serde_aux::field_attributes::deserialize_number_from_string;
use secrecy::{ExposeSecret, SecretString};
use sqlx::{postgres::PgConnectOptions};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: Application,
    pub database: Database
}

#[derive(serde::Deserialize)]
pub struct Application {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String

}

#[derive(serde::Deserialize, Clone)]
pub struct Database {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub username: String,
    pub password: SecretString,
    pub database_name: String
}

impl Database {
    pub fn connection_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .database(&self.database_name)
            
    }

}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir()
        .expect("could not get base path");
    let configuration_dir = base_path.join("configuration");

    let env: Environment = std::env::var("APP_ENVIRONMENT")
    .unwrap_or_else(|_| "local".into())
    .try_into()
    .expect("failed to parse");

    let environment_file_name = configuration_dir
    .join(format!("{}.yaml", env.as_str()));

    let settings = config::Config::builder()
    .add_source(
        config::File::from(configuration_dir.join("base.yaml"))
    )
    .add_source(
        config::File::from(environment_file_name)
    )
    .build()?;

    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production
}

impl Environment {
    // exists static throughout entire lifetime of entire application
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production"
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "Envrionment provided not support: {}",
                other
            ))
        }
    }
}