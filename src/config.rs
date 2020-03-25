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

macro_rules! try_get_var {
    ($key:literal) => {
        var($key).map_err(|e| ConfigError::from_var_error($key, e))?
    };
}

pub struct Database {
    pub url: String,
}

impl Database {
    fn from_env() -> Result<Self, ConfigError> {
        let url = try_get_var!("DATABASE_URL");
        
        Ok(Self {
            url,
        })
    }
}

pub struct Config {
    pub database: Database,
    pub bind_address: String,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();

        let database = Database::from_env()?;

        let bind_address = var("BIND_ADDRESS").unwrap_or("0.0.0.0:8080".to_owned());

        Ok(Self {
            database,
            bind_address,
        })
    }
}