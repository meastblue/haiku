use super::entity::{Prompt, PromptInput, UpdatePrompt};
use async_graphql::Context;
use sqlx::PgPool;
use uuid::Uuid;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn list_prompts(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Prompt>> {
        let pool = ctx.data::<PgPool>()?;
        let prompts = Prompt::list(pool).await?;
        Ok(prompts)
    }

    async fn get_prompt(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Prompt> {
        let pool = ctx.data::<PgPool>()?;
        let prompt = Prompt::get(pool, id).await?;
        Ok(prompt)
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_prompt(
        &self,
        ctx: &Context<'_>,
        data: PromptInput,
    ) -> async_graphql::Result<Prompt> {
        let pool = ctx.data::<PgPool>()?;
        let prompt = PromptInput::create(pool, data).await?;
        Ok(prompt)
    }

    async fn update_prompt(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        data: UpdatePrompt,
    ) -> async_graphql::Result<Prompt> {
        let pool = ctx.data::<PgPool>()?;
        let prompt = UpdatePrompt::update(pool, id, data).await?;
        Ok(prompt)
    }

    async fn delete_prompt(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Prompt> {
        let pool = ctx.data::<PgPool>()?;
        let prompt = Prompt::delete(pool, id).await?;
        Ok(prompt)
    }

    async fn restore_prompt(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Prompt> {
        let pool = ctx.data::<PgPool>()?;
        let prompt = Prompt::restore(pool, id).await?;
        Ok(prompt)
    }

    async fn destroy_prompt(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<bool> {
        let pool = ctx.data::<PgPool>()?;
        Prompt::destroy(pool, id).await?;
        Ok(true)
    }
}
