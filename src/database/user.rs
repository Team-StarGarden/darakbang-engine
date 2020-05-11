use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel::mysql::MysqlConnection;
use diesel::result::QueryResult;

use crate::database::model::{NewUser, User};
use crate::diesel::RunQueryDsl;
use std::error::Error;
use jsonwebtoken::{encode, Header, EncodingKey, decode, Validation, Algorithm, DecodingKey, TokenData};
use chrono::{DateTime, Utc, Duration};
use juniper::parser::Token;
use std::ops::Add;

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

pub fn create_local_user(
    conn: &MysqlConnection,
    user_name: String,
    password: &str,
) -> QueryResult<User> {
    use rand::{distributions::Alphanumeric, Rng};

    let salt = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .collect::<String>();
    let password_hash = argon2rs::argon2i_simple(password, &salt);

    let new_user_name = user_name.clone();
    let new_user = NewUser::new_local(user_name, password_hash, salt);
    {
        use crate::database::schema::user::dsl::*;

        diesel::insert_into(user).values(&new_user).execute(conn)?;

        user.filter(user_name.eq(new_user_name)).select((uid, service_name, user_name, point)).get_result(conn)
    }
}

pub fn find_local_user(
    conn: &MysqlConnection,
    user_name: &str,
    password: &str,
) -> QueryResult<Option<User>> {
    use crate::database::schema::user::dsl::{self, user};
    use zeroize::Zeroizing;

    let salt: Zeroizing<_> = user
        .select(dsl::salt)
        .filter(dsl::user_name.eq(user_name))
        .first::<Option<String>>(conn)
        .optional()?
        .flatten()
        .into();

    if let Some(salt) = salt.as_ref() {
        let password_hash = Zeroizing::new(argon2rs::argon2i_simple(password, salt));
        let result = user
            .filter(dsl::user_name.eq(user_name))
            .filter(dsl::salt.eq(salt))
            .filter(dsl::password.eq(Some(password_hash.as_ref())))
            .select((dsl::uid, dsl::service_name, dsl::user_name, dsl::point))
            .first(conn)
            .optional()?;
        Ok(result)
    } else {
        Ok(None)
    }
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    exp: usize,
    iat: usize,
    iss: String,
    sub: String,
}

use thiserror::Error;
use crate::config;
use crate::config::Config;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    NotFound,
    #[error("Authentication error")]
    Authentication,
}

pub fn auth_local_user(
    conn: &MysqlConnection,
    user_name: &str,
    password: &str,
) -> Result<String, UserError> {
    let config: Config = config::Config::load().expect("Invalid configuration detected");
    let user: User = match find_local_user(conn, user_name, password) {
        Ok(Some(user)) => user,
        _ => Err(UserError::NotFound)?,
    };
    let claim = TokenClaims {
        exp: Utc::now().add(Duration::weeks(1)).timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
        iss: config.jwt_issuer.clone(),
        sub: user.uid,
    };
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    ).map_err(|e| UserError::Authentication)
}

pub fn auth_token(
    conn: &MysqlConnection,
    token: &str,
) -> Result<TokenClaims, UserError> {
    let config: Config = config::Config::load().expect("Invalid configuration detected");
    let token_data = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::default(),
    ).map_err(|e| UserError::Authentication)?;
    Ok(token_data.claims)
}
