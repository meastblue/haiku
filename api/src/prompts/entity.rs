use async_graphql::SimpleObject;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Pool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, SimpleObject, FromRow)]
pub struct Prompt {
    id: Uuid,
    title: String,
    pub(crate) content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl Prompt {
    pub async fn list(pool: &PgPool) -> Result<Vec<Prompt>, sqlx::Error> {
        let prompts = sqlx::query_as::<_, Prompt>(
            r#"
            SELECT id, title, content, created_at, updated_at, deleted_at
            FROM prompts
            WHERE deleted_at IS NULL
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(prompts)
    }

    pub async fn get(pool: &PgPool, id: Uuid) -> Result<Prompt, sqlx::Error> {
        let prompt = sqlx::query_as::<_, Prompt>(
            r#"
            SELECT id, title, content, created_at, updated_at, deleted_at
            FROM prompts
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(prompt)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Prompt, sqlx::Error> {
        let prompt = sqlx::query_as::<_, Prompt>(
            r#"
            UPDATE prompts
            SET deleted_at = now()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, title, content, created_at, updated_at, deleted_at
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(prompt)
    }

    pub async fn destroy(pool: &PgPool, id: Uuid) -> Result<Prompt, sqlx::Error> {
        let prompt = sqlx::query_as::<_, Prompt>(
            r#"
            DELETE FROM prompts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(prompt)
    }

    pub async fn restore(pool: &PgPool, id: Uuid) -> Result<Prompt, sqlx::Error> {
        let prompt = sqlx::query_as::<_, Prompt>(
            r#"
            UPDATE prompts
            SET deleted_at = NULL
            WHERE id = $1 AND deleted_at IS NOT NULL
            RETURNING id, title, content, created_at, updated_at, deleted_at
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(prompt)
    }
}

pub struct PromptInput {
    title: String,
    content: String,
}

impl PromptInput {
    pub async fn create(pool: &PgPool, input: PromptInput) -> Result<Prompt, sqlx::Error> {
        let prompt = sqlx::query_as::<_, Prompt>(
            r#"
            INSERT INTO prompts (title, content)
            VALUES ($1, $2)
            RETURNING id, title, content, created_at, updated_at, deleted_at
            "#,
        )
        .bind(input.title)
        .bind(input.content)
        .fetch_one(pool)
        .await?;

        Ok(prompt)
    }
}

pub struct UpdatePrompt {
    title: Option<String>,
    content: Option<String>,
}

impl UpdatePrompt {
    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        input: UpdatePrompt,
    ) -> Result<Prompt, sqlx::Error> {
        let prompt = sqlx::query_as::<_, Prompt>(
            r#"
            UPDATE prompts
            SET title = COALESCE($1, title), content = COALESCE($2, content), updated_at = now()
            WHERE id = $3 AND deleted_at IS NULL
            RETURNING id, title, content, created_at, updated_at, deleted_at
            "#,
        )
        .bind(input.title)
        .bind(input.content)
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(prompt)
    }
}
