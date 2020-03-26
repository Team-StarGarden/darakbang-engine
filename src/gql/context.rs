use crate::database::MysqlPool;

impl juniper::Context for Context {}

pub struct Context {
    pub database_pool: MysqlPool,
}

impl Context {
    pub fn new(database_pool: MysqlPool) -> Self {
        Context {
            database_pool,
        }
    }
}
