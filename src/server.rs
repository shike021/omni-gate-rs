//! Server module
//!
//! Responsible for configuring and starting the REST API server.
//!
//! Copyright © 2025 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

use crate::config::Config;
use crate::handlers::grid::AppState;
use crate::handlers::grpc_helloworld::GreeterService;
use crate::handlers::grpc_user::UserServiceImpl;
use crate::protos::helloworld::greeter_server::GreeterServer;
use crate::protos::user::user_service_server::UserServiceServer;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use std::sync::{Arc, RwLock};
use tonic::transport::Server;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import route modules
use crate::routes;

/// Start the server
pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::from_file("config").unwrap_or_else(|_| {
        tracing::warn!("Failed to load config file, using defaults");
        Config::default()
    });

    // Initialize logging
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| config.logging.level.clone());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize application state
    let state = AppState {
        grid_items: Arc::new(RwLock::new(Vec::new())),
    };

    // Build application routes
    let app = routes::app_routes()
        .with_state(state)
        .layer(CorsLayer::permissive());

    // Get REST server address
    let rest_addr = config.rest_addr()?;
    tracing::info!("Starting REST API server on {}", rest_addr);

    // Get JSON-RPC server address
    let jsonrpc_addr = config.jsonrpc_addr()?;
    tracing::info!("Starting JSON-RPC server on {}", jsonrpc_addr);

    // Start JSON-RPC server
    let jsonrpc_server = tokio::spawn(async move {
        let server = ServerBuilder::default().build(jsonrpc_addr).await?;
        let rpc_module = routes::json_rpc::create_rpc_module();
        let handle: ServerHandle = server.start(rpc_module);
        handle.stopped().await;
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    });

    // Start REST API server
    let rest_server = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(rest_addr).await?;
        axum::serve(listener, app).await?;
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    });

    // Get gRPC server address
    let grpc_addr = config.grpc_addr()?;
    let grpc_server = Server::builder()
        .add_service(GreeterServer::new(GreeterService))
        .add_service(UserServiceServer::new(UserServiceImpl::default()))
        .serve(grpc_addr);

    tracing::info!("Starting GRPC server on {}", grpc_addr);
    let grpc_server = tokio::spawn(async move {
        grpc_server.await?;
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    });

    // Wait for all servers to complete
    let _ = tokio::try_join!(jsonrpc_server, rest_server, grpc_server);

    Ok(())
}
