use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::ConnectionError;

use crate::config;

pub fn establish_connection(config: &config::Database) -> Result<MysqlConnection, ConnectionError> {
    let database_url = &config.url;
    if cfg!(feature = "debug") {
        dbg!(database_url);
    }
    MysqlConnection::establish(database_url)
}
