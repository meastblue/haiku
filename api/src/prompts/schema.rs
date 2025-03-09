pub(crate) use super::resolver::{MutationRoot, QueryRoot};
use async_graphql::{EmptySubscription, Schema};
use sqlx::PgPool;

pub type PromptSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(pool: PgPool) -> PromptSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish()
}
