use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::ConnectionError;

use crate::config;

pub fn establish_connection(config: &config::Database) -> Result<MysqlConnection, ConnectionError> {
    let database_url = &config.url;
    MysqlConnection::establish(dbg!(database_url))
}
