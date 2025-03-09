use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use crate::prompts::entity::{Prompt, PromptInput, UpdatePrompt};

#[derive(SimpleObject, Serialize, Deserialize, Debug, FromRow)]
pub struct Haiku {
    id: i32,
    content: String,
    is_funny: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl Haiku {
    pub async fn list(pool: &PgPool) -> Result<Vec<Haiku>, sqlx::Error> {
        let haikus = sqlx::query_as::<_, Haiku>(
            r#"
            SELECT id, content, is_funny, created_at, updated_at, deleted_at
            FROM haikus
            WHERE deleted_at IS NULL
            "#,
        ).fetch_all(pool).await?;

        Ok(haikus)
    }

    pub async fn get(pool: &PgPool, id: Uuid) -> Result<Haiku, sqlx::Error> {
        let prompt = sqlx::query_as::<_, Haiku>(
            r#"
            SELECT id, title, created_at, updated_at, deleted_at
            FROM haikus
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(prompt)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Haiku, sqlx::Error> {
        let prompt = sqlx::query_as::<_, Haiku>(
            r#"
            UPDATE haikus
            SET deleted_at = now()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, content, created_at, updated_at, deleted_at
            "#,
        )
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(prompt)
    }

    pub async fn destroy(pool: &PgPool, id: Uuid) -> Result<Haiku, sqlx::Error> {
        let prompt = sqlx::query_as::<_, Haiku>(
            r#"
            DELETE FROM haikus
            WHERE id = $1
            "#,
        )
            .bind(id)
            .execute(pool)
            .await?;

        Ok(prompt)
    }

    pub async fn restore(pool: &PgPool, id: Uuid) -> Result<Haiku, sqlx::Error> {
        let prompt = sqlx::query_as::<_, Haiku>(
            r#"
            UPDATE haikus
            SET deleted_at = NULL
            WHERE id = $1 AND deleted_at IS NOT NULL
            RETURNING id, content, created_at, updated_at, deleted_at
            "#,
        )
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(prompt)
    }
}

pub struct InputHaiku {
    pub content: String,
    pub is_funny: bool,
}

impl InputHaiku {
    pub async fn create(pool: &PgPool, input: InputHaiku) -> Result<Haiku, sqlx::Error> {
        let prompt = sqlx::query_as::<_, Haiku>(
            r#"
            INSERT INTO haikus (content)
            VALUES ($1)
            RETURNING id, content, created_at, updated_at, deleted_at
            "#,
        )
            .bind(input.content)
            .fetch_one(pool)
            .await?;

        Ok(prompt)
    }
}

pub struct UpdateHaiku {
    pub id: i32,
    pub content: Option<String>,
    pub is_funny: Option<bool>,
}

impl UpdateHaiku {
    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        input: UpdateHaiku,
    ) -> Result<Haiku, sqlx::Error> {
        let prompt = sqlx::query_as::<_, Haiku>(
            r#"
            UPDATE haikus
            SET content = COALESCE($1, content), is_funny_, updated_at = now()
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

#[derive(Debug, Serialize)]
pub struct HaikuRequest {
    prompt: String,
    max_tokens: i32,
    temperature: f32,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct HaikuResponse {
    pub haiku: String,
    pub is_funny: bool,
}

pub struct DeepseekClient {
    url: String,
    api_key: String,
    client: Client,
}

impl DeepseekClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let url = std::env::var("DEEPSEEK_API_URL")?;
        let api_key = std::env::var("DEEPSEEK_API_KEY")?;

        Ok(Self {
            url,
            api_key,
            client: Client::new(),
        })
    }

    pub async fn generate_haiku(
        &self,
        prompt: &Prompt,
        max_tokens: i32,
        temperature: f32,
    ) -> Result<HaikuResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/generate", self.url);
        let request_body = HaikuRequest {
            prompt: prompt.content.clone(),
            max_tokens,
            temperature,
        };

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let haiku_response: HaikuResponse = response.json().await?;
            Ok(haiku_response)
        } else {
            Err(format!("API request failed with status: {}", response.status()).into())
        }
    }
}