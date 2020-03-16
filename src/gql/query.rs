use crate::gql::Context;
pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }
}
