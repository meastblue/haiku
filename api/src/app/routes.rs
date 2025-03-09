use crate::app::schemas::AppSchema;
use crate::prompts::schema::PromptSchema;
use crate::users::schema::UserSchema;
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::routing::post;
use axum::{
    Router,
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
};
use sqlx::PgPool;

pub fn config_routes(pool: &PgPool) -> Router {
    let app_schema = AppSchema::new(pool);

    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route(
            "/users",
            get(graphql_handler_users).post(graphql_handler_users),
        )
        .route(
            "/prompts",
            get(graphql_handler_prompts).post(graphql_handler_prompts),
        )
        .route("/gql", get(graphql))
        .layer(Extension(app_schema.user_schema))
}

async fn graphql_handler_users(
    Extension(schema): Extension<UserSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_handler_prompts(
    Extension(schema): Extension<PromptSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
