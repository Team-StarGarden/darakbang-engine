use std::ptr::null;

use crate::database::user::get_all_users;
use crate::gql::Context;
use crate::gql::schema::User;

pub struct Query;

#[juniper::object(
Context = Context,
)]
impl Query {
    fn api_version() -> &str {
        "1.0"
    }

    fn all_users(context: &Context) -> User {
        let conn = context.database_pool.get().unwrap();
        let users = get_all_users(&conn);
        match users {
            Ok(result) => {
                User::from_database(&result)
            }
            Err(error) => {
                User {
                    uid: "".parse().unwrap(),
                    user_name: "".parse().unwrap(),
                    point: 0,
                }
            }
        }
    }
}
