//! REST route handling
//!
//! Handles HTTP requests related to REST APIs and forwards requests to business logic modules.
//!
//! Copyright © 2025 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

use crate::handlers::grid::{create, delete_by_id, get_by_id, list, update, AppState};
use axum::{routing::get, Router};

pub fn rest_routes() -> Router<AppState> {
    Router::new().route("/grid", get(list).post(create)).route(
        "/grid/{id}",
        get(get_by_id).put(update).delete(delete_by_id),
    )
}
