use crate::database::model::User as DatabaseUser;

#[derive(juniper::GraphQLObject)]
#[graphql(description = "User object")]
pub struct User {
    uid: String,
    service_name: String,
    user_name: String,
    point: i32,
}

impl User {
    pub fn from_database(user: DatabaseUser) -> Self {
        Self {
            uid: user.uid,
            service_name: user.service_name,
            user_name: user.user_name,
            point: user.point,
        }
    }
}
