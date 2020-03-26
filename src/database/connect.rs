use diesel::mysql::MysqlConnection;
use diesel::r2d2::{Pool, PooledConnection, ConnectionManager, PoolError};

use crate::config;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MysqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn establish_connection(config: &config::Database) -> Result<MysqlPool, PoolError> {
    let database_url = &config.url;
    let manager = ConnectionManager::new(database_url);
    Pool::builder().build(manager)
}
