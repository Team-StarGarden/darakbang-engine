use std::ops::Add;

use chrono::{Duration, Utc};
use diesel::MysqlConnection;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::config;
use crate::database::model::User;
use crate::database::user::{auth, local};

pub fn auth(
    conn: &MysqlConnection,
    user_name: &str,
    password: &str,
) -> Result<String, auth::error::UserAuthError> {
    let config: config::Config = config::Config::load().expect("Invalid configuration detected");
    let user: User = match local::find(conn, user_name, password) {
        Ok(Some(user)) => user,
        _ => Err(auth::error::UserAuthError::NotFound)?,
    };
    let claim = auth::TokenClaims {
        exp: Utc::now().add(Duration::weeks(1)).timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
        iss: config.jwt_issuer.clone(),
        sub: user.uid,
    };
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    ).map_err(|e| auth::error::UserAuthError::Authentication)
}