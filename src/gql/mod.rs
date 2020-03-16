
    mod context;
    mod mutation;
    mod query;

    pub use context::Context;
    pub use mutation::Mutation;
    pub use query::Query;

    pub type Schema = juniper::RootNode<'static, Query, Mutation>;