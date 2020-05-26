use std::error::Error;
use std::ops::Add;

use chrono::{DateTime, Duration, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel::mysql::MysqlConnection;
use diesel::result::QueryResult;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use juniper::parser::Token;

use crate::database::model::{NewUser, User};
use crate::diesel::RunQueryDsl;

pub mod local;

pub fn get_all_users(
    conn: &MysqlConnection,
) -> QueryResult<Vec<User>> {
    use crate::database::schema::user::dsl::*;

    user.select((uid, service_name, user_name, point)).load(conn)
}

pub fn create_user(
    conn: &MysqlConnection,
    uid: String,
    service_name: String,
    user_name: String,
) -> QueryResult<User> {
    let new_user = NewUser::new_oauth(uid, service_name, user_name);
    {
        use crate::database::schema::user::dsl::*;

        diesel::insert_into(user).values(&new_user).execute(conn)?;

        user.find(uid).select((uid, service_name, user_name, point)).get_result(conn)
    }
}

pub mod auth;
