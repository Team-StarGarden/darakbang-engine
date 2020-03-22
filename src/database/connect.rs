use std::env;
use std::env::VarError;

use std::fmt;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::ConnectionError;

use dotenv::dotenv;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseConnectionError {
    DatabaseUrlInvalid(VarError),
    MysqlConnectionError(ConnectionError),
}

impl fmt::Display for DatabaseConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseConnectionError::DatabaseUrlInvalid(e) => write!(f, "DotenvError({:?})", e),
            DatabaseConnectionError::MysqlConnectionError(e) => write!(f, "DotenvError({:?})", e),
        }
    }
}

pub fn establish_connection() -> Result<MysqlConnection, DatabaseConnectionError> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").map_err(DatabaseConnectionError::DatabaseUrlInvalid)?;
    MysqlConnection::establish(&database_url).map_err(DatabaseConnectionError::MysqlConnectionError)
}
