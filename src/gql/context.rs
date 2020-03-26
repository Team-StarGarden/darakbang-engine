use diesel::mysql::MysqlConnection;

impl juniper::Context for Context {}

pub struct Context {
    pub database_connection: MysqlConnection,
}

impl Context {
    pub fn new(database_connection: MysqlConnection) -> Self {
        Context {
            database_connection,
        }
    }
}
