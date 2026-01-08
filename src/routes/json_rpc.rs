//! JSON-RPC route handling
//!
//! Handles JSON-RPC related requests using jsonrpsee native server.
//!
//! Copyright © 2025 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

use jsonrpsee::core::error::StringError;
use jsonrpsee::server::RpcModule;
use jsonrpsee::server::SubscriptionMessage;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
use std::time::Duration;

use crate::handlers::user_info as rpc;

/// Create and configure JSON-RPC module
pub fn create_rpc_module() -> RpcModule<()> {
    let mut module = RpcModule::new(());

    module
        .register_async_method("get_user_info", |_params, _subscription, _ctx| async move {
            rpc::get_user_info().await.map_err(|e: ErrorObjectOwned| e)
        })
        .unwrap();

    module
        .register_async_method(
            "update_user_info",
            |params, _subscription, _ctx| async move {
                let value: Value = params.parse().unwrap_or_default();
                rpc::update_user_info(value)
                    .await
                    .map_err(|e: ErrorObjectOwned| e)
            },
        )
        .unwrap();

    module
        .register_async_method(
            "verify_credentials",
            |params, _subscription, _ctx| async move {
                let value: Value = params.parse().unwrap_or_default();
                rpc::verify_credentials(value)
                    .await
                    .map_err(|e: ErrorObjectOwned| e)
            },
        )
        .unwrap();

    module
        .register_subscription(
            "subscribe_user_updates",
            "subscribe_user_updates",
            "unsubscribe_user_updates",
            |params, pending, _ctx, _extensions| async move {
                let value: Value = params.parse().unwrap_or_default();
                let user_id = value.get("user_id").and_then(|v| v.as_i64()).unwrap_or(1);
                let interval_secs = value
                    .get("interval_seconds")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(2);
                let interval = Duration::from_secs(interval_secs);

                let sink = pending.accept().await.unwrap();

                let mut counter = 0;
                loop {
                    tokio::time::sleep(interval).await;

                    counter += 1;
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    let update_type = match counter % 3 {
                        0 => "profile_update",
                        1 => "activity_update",
                        _ => "status_update",
                    };

                    let update = serde_json::json!({
                        "user_id": user_id,
                        "name": format!("User {}", user_id),
                        "email": format!("user{}@example.com", user_id),
                        "age": 30 + (counter % 10),
                        "update_type": update_type,
                        "timestamp": timestamp,
                        "counter": counter
                    });

                    let msg = SubscriptionMessage::from_json(&update).unwrap();
                    if sink.send(msg).await.is_err() {
                        break;
                    }
                }
                Ok::<(), StringError>(())
            },
        )
        .unwrap();

    module
}
