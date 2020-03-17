use juniper::FieldResult;

use crate::gql::Context;

pub struct Mutation;

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "Example object")]
struct CreateUser {
    name: String,
}

#[juniper::object(
Context = Context,
)]
impl Mutation {
    fn create_user(context: &Context, new_human: CreateUser) -> FieldResult<String> {
        Ok("Example Response".into())
    }
}
