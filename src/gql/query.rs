use crate::gql::Context;
use crate::gql::schema::User;

use log::error;
use chrono::{DateTime, Utc};
use crate::database::user::{auth_local_user, auth_token};

pub struct Query;

#[juniper::object(
Context = Context,
)]
impl Query {
    fn api_version() -> &str {
        "1.0"
    }

    fn all_users(context: &Context) -> Option<Vec<User>> {
        use crate::database::user::get_all_users;

        let conn = context.database_pool
            .get()
            .map_err(|e| error!("Database connection failed: {}", e))
            .ok()?;
        get_all_users(&conn)
            .map(|vec| vec.into_iter().map(User::from_database).collect())
            .map_err(|e| error!("Failed to get all users: {}", e))
            .ok()
    }

    fn local_user(context: &Context, user_name: String, password: String) -> Option<User> {
        use crate::database::user::find_local_user;
        use zeroize::Zeroizing;

        let conn = context.database_pool
            .get()
            .map_err(|e| error!("Database connection failed: {}", e))
            .ok()?;
        let password = Zeroizing::new(password);
        find_local_user(&conn, &user_name, &password)
            .map_err(|e| error!("Failed to find local user: {}", e))
            .ok()
            .flatten()
            .map(User::from_database)
    }

    fn token_local_user(context: &Context, user_name: String, password: String) -> Option<String> {
        let conn = context.database_pool
            .get()
            .map_err(|e| error!("Database connection failed: {}", e))
            .ok()?;
        auth_local_user(&conn, &user_name, &password).ok()
    }

    fn token_valid(context: &Context, token: String) -> Option<bool> {
        let conn = context.database_pool
            .get()
            .map_err(|e| error!("Database connection failed: {}", e))
            .ok()?;
        match auth_token(&conn, &token).ok() {
            Some(_) => Some(true),
            _ => Some(false),
        }
    }
}
