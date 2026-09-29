mod api;
mod config;
mod database;
mod domain;
mod error;
mod state;
mod worker;

use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::Config, database::create_db_pool, state::AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let config = Config::load();

    let db_pool = create_db_pool(&config.database_url).await?;

    let state = AppState { db_pool };

    let app = api::router(state).layer(TraceLayer::new_for_http());

    let address = format!("{}:{}", config.server_host, config.server_port);

    let listener = tokio::net::TcpListener::bind(address).await?;

    tracing::info!("Server running on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
