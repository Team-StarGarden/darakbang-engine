pub use context::Context;
pub use mutation::Mutation;
pub use query::Query;

mod context;
mod mutation;
mod query;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
