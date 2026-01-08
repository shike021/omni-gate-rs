//! JSON-RPC handler module
//!
//! Implements JSON-RPC business logic related to user information.
//!
//! Copyright © 2025 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

use jsonrpsee::types::ErrorObjectOwned;
use serde_json::json;

/// Get user information
pub async fn get_user_info() -> Result<serde_json::Value, ErrorObjectOwned> {
    Ok(json!({
        "name": "John Doe",
        "age": 30,
        "email": "john@example.com",
        "status": "active"
    }))
}

/// Update user information
pub async fn update_user_info(
    params: serde_json::Value,
) -> Result<serde_json::Value, ErrorObjectOwned> {
    let name = params
        .get("name")
        .and_then(|v: &serde_json::Value| v.as_str())
        .unwrap_or("");
    let age = params
        .get("age")
        .and_then(|v: &serde_json::Value| v.as_u64())
        .unwrap_or(0);

    Ok(json!({
        "success": true,
        "message": format!("User information updated: {} ({} years old)", name, age)
    }))
}

/// Verify user credentials
pub async fn verify_credentials(
    params: serde_json::Value,
) -> Result<serde_json::Value, ErrorObjectOwned> {
    let username = params
        .get("username")
        .and_then(|v: &serde_json::Value| v.as_str())
        .unwrap_or("");
    let password = params
        .get("password")
        .and_then(|v: &serde_json::Value| v.as_str())
        .unwrap_or("");

    Ok(json!({
        "authenticated": username == "admin" && password == "123456",
        "token": "sample-jwt-token"
    }))
}
