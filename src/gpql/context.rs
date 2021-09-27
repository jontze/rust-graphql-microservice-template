use juniper::Context;
use sqlx::PgPool;

pub struct GraphContext {
    pub pool: PgPool,
}

impl Context for GraphContext {}
