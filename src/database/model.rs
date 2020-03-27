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
    uid: String,
    service_name: String,
    user_name: String,
    point: i32,
    password: Option<Vec<u8>>,
    salt: Option<String>,
}

impl NewUser {
    pub fn new_local(user_name: String, password: [u8; 32], salt: String) -> Self {
        Self {
            uid: user_name.clone(),
            service_name: "local".into(),
            user_name,
            point: 0,
            password: Some(password.to_vec()),
            salt: Some(salt),
        }
    }

    pub fn new_oauth(uid: String, service_name: String, user_name: String) -> Self {
        Self {
            uid,
            service_name,
            user_name,
            point: 0,
            password: None,
            salt: None,
        }
    }
}

#[derive(Queryable)]
pub struct Word {
    pub id: i32,
    pub service_name: String,
    pub user_name: String,
    pub point: u32,
}
