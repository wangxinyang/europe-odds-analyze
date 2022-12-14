use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::OddsError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub db: DbConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

fn default_max_connections() -> u32 {
    5
}

impl Config {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, OddsError> {
        let content = fs::read_to_string(path).map_err(|_| OddsError::ConfigReadError)?;
        let config = serde_yaml::from_str(&content).map_err(|_| OddsError::ConfigParseError)?;
        Ok(config)
    }
}

impl DbConfig {
    pub fn server_url(&self) -> String {
        if self.password.is_empty() {
            format!("postgres://{}@{}:{}", self.user, self.host, self.port)
        } else {
            format!(
                "postgres://{}:{}@{}:{}",
                self.user, self.password, self.host, self.port
            )
        }
    }

    pub fn url(&self) -> String {
        format!("{}/{}", self.server_url(), self.dbname)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_should_be_loaded() {
        let config = Config::from_file("../presentation/fixtures/config.yml").unwrap();
        assert_eq!(
            config,
            Config {
                db: DbConfig {
                    host: "localhost".to_string(),
                    port: 5432,
                    user: "postgres".to_string(),
                    password: "postgres".to_string(),
                    dbname: "european_odds".to_string(),
                    max_connections: 5,
                },
            }
        );
    }
}
