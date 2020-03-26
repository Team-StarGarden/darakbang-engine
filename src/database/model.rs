use crate::database::schema::user;

#[derive(Queryable)]
pub struct User {
    pub uid: String,
    pub service_name: String,
    pub user_name: String,
    pub point: i32,
    #[diesel(deserialize_as = "HiddenColumn<Option<Vec<u8>>>")]
    pub password: (),
    #[diesel(deserialize_as = "HiddenColumn<Option<String>>")]
    pub salt: (),
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

use diesel::backend::Backend;
use diesel::deserialize::Queryable;

/// Represents a column which should not be serialized
pub struct HiddenColumn<T> {
    _data: std::marker::PhantomData<T>,
}

impl<DB, ST, T> Queryable<ST, DB> for HiddenColumn<T>
where
    DB: Backend,
    T: Queryable<ST, DB>,
{
    type Row = T::Row;

    fn build(_: Self::Row) -> Self {
        Self {
            _data: Default::default()
        }
    }
}

impl<T> Into<()> for HiddenColumn<T> {
    fn into(self) -> () {}
}
