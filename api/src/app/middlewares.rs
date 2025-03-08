use axum::http::{Method, header::HeaderName};
use tower_http::cors::CorsLayer;

pub fn cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers(vec![
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
        ])
        .allow_credentials(true)
}
