use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use adapter::{database::connect_database_with, redis::RedisClient};
use anyhow::{Context, Result};
use api::route::{auth, v1};
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use shared::env::{which, Environment};
use tokio::net::TcpListener;
// use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
// use tower_http::LatencyUnit;
// use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    bootstrap().await
}

fn init_logger() -> Result<()> {
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;
    Ok(())
}

async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::new()?;
    let pool = connect_database_with(&app_config.database);
    let kv = Arc::new(RedisClient::new(&app_config.redis)?);
    let registry = AppRegistry::new(pool, kv, app_config);

    let app = Router::new()
        .merge(v1::routes())
        .merge(auth::routes())
        //.layer(cors())
        //.layer(
        //    TraceLayer::new_for_http()
        //        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        //        .on_request(
        //            DefaultOnResponse::new()
        //                .level(Level::INFO)
        //                .latency_unit(LatencyUnit::Millis),
        //        ),
        //)
        .with_state(registry);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app)
        .await
        .context("Unexpected error happened in server")
        .inspect_err(|e| {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "Unexpected error"
            )
        })
}

// #[tokio::test]
// async fn health_check_words() {
//     let status_code = health_check().await;
//     assert_eq!(status_code, StatusCode::OK);
// }
//
// #[sqlx::test]
// async fn health_check_db_works(pool: sqlx::PgPool) {
//     let status_code = health_check_db(State(pool)).await;
//     assert_eq!(status_code, StatusCode::OK);
// }
