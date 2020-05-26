use std::error::Error;
use std::ops::Add;

use chrono::{DateTime, Duration, Utc};
use diesel::{MysqlConnection, QueryResult, RunQueryDsl};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use juniper::parser::Token;

pub use auth::auth;
pub use create::create;

use crate::config;
use crate::config::Config;
use crate::database::model::{NewUser, User};

mod auth;
mod create;

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