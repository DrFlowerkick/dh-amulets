//! src/configuration.rs

use anyhow::{anyhow, Context, Result};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

/// The possible runtime environment for our application.
pub enum Environment {
    Local,
    Github,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Github => "github",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "github" => Ok(Self::Github),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local`, `github` or `production`.",
                other
            )),
        }
    }
}

pub fn get_configuration() -> Result<Settings> {
    let base_path =
        std::env::current_dir().context("Failed to determine the current directory.")?;
    let configuration_directory = base_path.join("configuration");

    // Detect the running environment.
    // Default to `local` if unspecified.
    let environment: Environment = match std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
    {
        Ok(environment) => environment,
        Err(e) => return Err(anyhow!("Failed to parse APP_ENVIRONMENT: {}", e)),
    };
    let environment_filename = format!("{}.toml", environment.as_str());

    // Initialise our configuration reader
    let settings = config::Config::builder()
        // Add configuration values from a file named 'configuration.toml'
        .add_source(config::File::from(
            configuration_directory.join("base.toml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;
    // Try to convert the configuration values it read into our Settings type
    let config = settings.try_deserialize::<Settings>()?;
    Ok(config)
}
