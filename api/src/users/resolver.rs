use super::entity::{UpdateUser, User, UserInput};
use async_graphql::Context;
use sqlx::PgPool;
use uuid::Uuid;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn list_users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let pool = ctx.data::<PgPool>()?;
        let users = User::list(pool).await?;
        Ok(users)
    }

    async fn get_user(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<User> {
        let pool = ctx.data::<PgPool>()?;
        let user = User::get(pool, id).await?;
        Ok(user)
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_user(&self, ctx: &Context<'_>, data: UserInput) -> async_graphql::Result<User> {
        let pool = ctx.data::<PgPool>()?;
        let user = UserInput::create(pool, data).await?;
        Ok(user)
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        data: UpdateUser,
    ) -> async_graphql::Result<User> {
        let pool = ctx.data::<PgPool>()?;
        let user = UpdateUser::update(pool, id, data).await?;
        Ok(user)
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<User> {
        let pool = ctx.data::<PgPool>()?;
        let user = User::delete(pool, id).await?;
        Ok(user)
    }

    async fn restore_user(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<User> {
        let pool = ctx.data::<PgPool>()?;
        let user = User::restore(pool, id).await?;
        Ok(user)
    }

    async fn destroy_user(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<bool> {
        let pool = ctx.data::<PgPool>()?;
        User::destroy(pool, id).await?;
        Ok(true)
    }
}
