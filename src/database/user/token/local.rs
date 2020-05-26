use diesel::MysqlConnection;
use crate::database::user::token::TokenClaims;
use crate::config::Config;
use crate::config;
use crate::database::model::User;
use crate::database::user::find_local_user;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::ops::Add;
use crate::database::user::token::error::UserAuthError;

pub fn auth(
    conn: &MysqlConnection,
    user_name: &str,
    password: &str,
) -> Result<String, UserAuthError> {
    let config: Config = config::Config::load().expect("Invalid configuration detected");
    let user: User = match find_local_user(conn, user_name, password) {
        Ok(Some(user)) => user,
        _ => Err(UserAuthError::NotFound)?,
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
    ).map_err(|e| UserAuthError::Authentication)
}