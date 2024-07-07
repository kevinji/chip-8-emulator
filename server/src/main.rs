use axum::Router;
use clap::Parser;
use http::{
    header::{self, HeaderName, HeaderValue},
    Method,
};
use std::net::Ipv4Addr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::{self, CorsLayer},
    services::ServeDir,
    ServiceBuilderExt,
};

#[derive(Debug, Parser)]
struct Args {
    /// Port to start the server on
    #[clap(long, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Args { port } = Args::try_parse()?;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::OPTIONS])
        .allow_origin(cors::Any);

    let service_builder = ServiceBuilder::new()
        .layer(cors)
        .append_response_header(
            HeaderName::from_static("cross-origin-embedder-policy"),
            HeaderValue::from_static("require-corp"),
        )
        .append_response_header(
            HeaderName::from_static("cross-origin-opener-policy"),
            HeaderValue::from_static("same-origin"),
        )
        .append_response_header(
            HeaderName::from_static("cross-origin-resource-policy"),
            HeaderValue::from_static("same-origin"),
        )
        .append_response_header(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        )
        .append_response_header(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"))
        .append_response_header(header::X_XSS_PROTECTION, HeaderValue::from_static("0"));

    let router = Router::new()
        .nest_service("/", ServeDir::new("web-src"))
        .layer(service_builder);

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, port)).await?;
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
