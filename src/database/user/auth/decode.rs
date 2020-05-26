use diesel::MysqlConnection;
use jsonwebtoken::{DecodingKey, Validation};

use crate::config;
use crate::database::user::auth::{error::UserAuthError, TokenClaims};

pub fn decode(
    conn: &MysqlConnection,
    token: &str,
) -> Result<TokenClaims, UserAuthError> {
    let config: config::Config = config::Config::load().expect("Invalid configuration detected");
    let token_data = jsonwebtoken::decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::default(),
    ).map_err(|e| UserAuthError::Authentication)?;
    Ok(token_data.claims)
}