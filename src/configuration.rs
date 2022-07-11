use std::convert::TryInto;

use config::{Config, File};
use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    ConnectOptions,
};

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

impl TryInto<Environment> for String {
    type Error = String;

    fn try_into(self) -> Result<Environment, Self::Error> {
        match self.as_str() {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            other => Err(format!(
                "Unsuported environment: {other} - Use either `local` or `production`."
            )),
        }
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    port: u16,
}

impl ApplicationSettings {
    pub fn host(&self) -> &str {
        self.host.as_str()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn assign_random_port(&mut self) {
        self.port = 0
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    port: u16,
    username: String,
    password: Secret<String>,
    pub database_name: String,
    require_ssl: bool,
}

impl DatabaseSettings {
    pub fn connect_options_without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .username(&self.username)
            .password(self.password.expose_secret())
            .host(&self.host)
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn connect_options_with_db(&self) -> PgConnectOptions {
        let mut connect_options = self
            .connect_options_without_db()
            .database(&self.database_name);
        connect_options.log_statements(tracing::log::LevelFilter::Trace);
        connect_options
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let configuration_directory = std::env::current_dir()
        .expect("Failed to determine the current directry")
        .join("configuration");

    // 実行環境を検出する
    // 指定がない場合のデフォルトは `local`
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse `APP_ENVIRONMENT`.");
    let environment_file = format!("{}.yaml", environment.as_str());
    let settings = Config::builder()
        .add_source(File::from(configuration_directory.join("base.yml")))
        .add_source(File::from(configuration_directory.join(environment_file)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}
