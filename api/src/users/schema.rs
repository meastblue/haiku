pub(crate) use super::resolver::{MutationRoot, QueryRoot};
use async_graphql::{EmptySubscription, Schema};
use sqlx::PgPool;

pub type UserSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(pool: PgPool) -> UserSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish()
}
