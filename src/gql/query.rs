use crate::gql::Context;

pub struct Query;

#[juniper::object(
Context = Context,
)]
impl Query {
    fn api_version() -> &str {
        "1.0"
    }
}
