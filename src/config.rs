use std::env::{var, VarError};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum ConfigError {
    MissingKey(String),
    InvalidFormat(String),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingKey(key) => write!(f, "{} does not exist", key),
            Self::InvalidFormat(key) => write!(f, "{} is in invalid format", key),
        }
    }
}

impl ConfigError {
    fn from_var_error(key: &str, var_error: VarError) -> Self {
        match var_error {
            VarError::NotPresent => Self::MissingKey(key.to_owned()),
            VarError::NotUnicode(_) => Self::InvalidFormat(key.to_owned()),
        }
    }
}

pub struct Database {
    pub url: String,
}

impl Database {
    fn from_env() -> Result<Self, ConfigError> {
        let url = var("DATABASE_URL").map_err(|e| ConfigError::from_var_error("DATABASE_URL", e))?;
        
        Ok(Self {
            url,
        })
    }
}

pub struct Config {
    pub database: Database
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();

        let database = Database::from_env()?;

        Ok(Self {
            database,
        })
    }
}