use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, SimpleObject, FromRow)]
pub struct User {
    pub id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub async fn list(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, first_name, last_name, email, password, created_at, updated_at, deleted_at
            FROM users
            WHERE deleted_at IS NULL
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    pub async fn get(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, first_name, last_name, email, password, created_at, updated_at, deleted_at
            FROM users
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET deleted_at = now()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, first_name, last_name, email, password, created_at, updated_at, deleted_at
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn destroy(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn restore(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET deleted_at = NULL
            WHERE id = $1 AND deleted_at IS NOT NULL
            RETURNING id, first_name, last_name, email, password, created_at, updated_at, deleted_at
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}

#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct UserInput {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

impl UserInput {
    pub async fn create(pool: &PgPool, data: UserInput) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (first_name, last_name, email, password, created_at, updated_at)
            VALUES ($1, $2, $3, $4, now(), now())
            RETURNING id, first_name, last_name, email, password, created_at, updated_at, deleted_at
            "#,
        )
        .bind(data.first_name)
        .bind(data.last_name)
        .bind(data.email)
        .bind(data.password)
        .fetch_one(pool)
        .await?;

        println!("UserInput::create: {:?}", user);

        Ok(user)
    }
}

#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl UpdateUser {
    pub async fn update(pool: &PgPool, id: Uuid, data: UpdateUser) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET
                first_name = COALESCE($2, first_name),
                last_name = COALESCE($3, last_name),
                email = COALESCE($4, email),
                password = COALESCE($5, password),
                updated_at = now()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, first_name, last_name, email, password, created_at, updated_at, deleted_at
            "#,
        )
        .bind(id)
        .bind(data.first_name)
        .bind(data.last_name)
        .bind(data.email)
        .bind(data.password)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}
