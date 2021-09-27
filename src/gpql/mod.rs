mod context;
pub mod handler;
mod model;
mod root;

pub use context::GraphContext as Context;
use juniper::{EmptySubscription, RootNode};
use root::{MutationRoot, QueryRoot};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
