use crate::users::schema::{MutationRoot, QueryRoot, UserSchema};
use async_graphql::{EmptySubscription, Schema};
use sqlx::PgPool;

pub struct AppSchema {
    pub user_schema: UserSchema,
}

impl AppSchema {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            user_schema: crate::users::schema::create_schema(pool.clone()),
        }
    }

    pub fn build(pool: &PgPool) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
        Schema::build(
            crate::users::schema::QueryRoot,
            crate::users::schema::MutationRoot,
            EmptySubscription,
        )
        .data(pool.clone())
        .finish()
    }
}
