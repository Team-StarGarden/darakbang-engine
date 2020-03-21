use crate::database::schema::user;

#[derive(Queryable)]
pub struct User {
    pub uid: String,
    pub service_name: String,
    pub user_name: String,
    pub point: i32,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub uid: String,
    pub service_name: String,
    pub user_name: String,
}

#[derive(Queryable)]
pub struct Word {
    pub id: i32,
    pub service_name: String,
    pub user_name: String,
    pub point: u32,
}
