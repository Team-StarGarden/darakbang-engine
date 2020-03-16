use crate::gql::Context;
use juniper::FieldResult;
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
    fn createUser(context: &Context, new_human: CreateUser) -> FieldResult<String> {
        Ok("Example Response".into())
    }
}
