use log::error;

use crate::gql::Context;
use crate::gql::schema::User;

pub struct Mutation;

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "Create a local user")]
struct CreateLocalUser {
    user_name: String,
    password: String,
}

#[juniper::object(
Context = Context,
)]
impl Mutation {
    fn create_local_user(context: &Context, new_user: CreateLocalUser) -> Option<User> {
        use crate::database::user::create_local_user;
        use zeroize::Zeroizing;

        let conn = context.database_pool
            .get()
            .map_err(|e| error!("Database connection failed: {}", e))
            .ok()?;
        let password = Zeroizing::new(new_user.password);
        create_local_user(&conn, new_user.user_name, &password)
            .map(User::from_database)
            .map_err(|e| error!("Database connection failed: {}", e))
            .ok()
    }
}
