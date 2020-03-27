use crate::database::model::User as DatabaseUser;

#[derive(juniper::GraphQLObject)]
#[graphql(description = "user object")]
pub struct User {
    uid: String,
    user_name: String,
    point: i32,
}

impl User {
    pub fn from_database(user: &DatabaseUser) -> Self {
        Self {
            uid: user.uid.clone(),
            user_name: user.user_name.clone(),
            point: user.point,
        }
    }
}
