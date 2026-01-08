//! Unified error handling module
//!
//! Contains unified error type definitions and error response formats.
//!
//! Copyright © 2025 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

/// Unified error response structure
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorInfo,
}

/// Error information structure
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorInfo {
    pub code: i32,
    pub message: String,
}

/// Unified error code definition
#[derive(Debug, Clone)]
pub enum ErrorCode {
    // 通用错误 1000-1999
    InternalError = 1000,
    ValidationError = 1001,
    NotFound = 1002,
    Unauthorized = 1003,
    Forbidden = 1004,

    // 业务错误 2000-2999
    GridItemNotFound = 2001,
    GridItemCreationFailed = 2002,
    GridItemUpdateFailed = 2003,

    // JSON-RPC 错误 3000-3999
    JsonRpcParseError = 3001,
    JsonRpcMethodNotFound = 3002,
    JsonRpcInvalidParams = 3003,
}

impl ErrorCode {
    pub fn code(&self) -> i32 {
        self.clone() as i32
    }

    pub fn message(&self) -> &str {
        match self {
            ErrorCode::InternalError => "Internal server error",
            ErrorCode::ValidationError => "Validation error",
            ErrorCode::NotFound => "Resource not found",
            ErrorCode::Unauthorized => "Unauthorized",
            ErrorCode::Forbidden => "Forbidden",
            ErrorCode::GridItemNotFound => "Grid item not found",
            ErrorCode::GridItemCreationFailed => "Grid item creation failed",
            ErrorCode::GridItemUpdateFailed => "Grid item update failed",
            ErrorCode::JsonRpcParseError => "JSON-RPC parse error",
            ErrorCode::JsonRpcMethodNotFound => "JSON-RPC method not found",
            ErrorCode::JsonRpcInvalidParams => "JSON-RPC invalid params",
        }
    }
}

/// 应用程序自定义错误类型
#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    InternalError,
    ValidationError,
    NotFound,
    Unauthorized,
    Forbidden,
    GridItemNotFound,
    GridItemCreationFailed,
    GridItemUpdateFailed,
    JsonRpcParseError,
    JsonRpcMethodNotFound,
    JsonRpcInvalidParams,
}

impl AppError {
    /// 转换为错误码
    pub fn error_code(&self) -> ErrorCode {
        match self {
            AppError::InternalError => ErrorCode::InternalError,
            AppError::ValidationError => ErrorCode::ValidationError,
            AppError::NotFound => ErrorCode::NotFound,
            AppError::Unauthorized => ErrorCode::Unauthorized,
            AppError::Forbidden => ErrorCode::Forbidden,
            AppError::GridItemNotFound => ErrorCode::GridItemNotFound,
            AppError::GridItemCreationFailed => ErrorCode::GridItemCreationFailed,
            AppError::GridItemUpdateFailed => ErrorCode::GridItemUpdateFailed,
            AppError::JsonRpcParseError => ErrorCode::JsonRpcParseError,
            AppError::JsonRpcMethodNotFound => ErrorCode::JsonRpcMethodNotFound,
            AppError::JsonRpcInvalidParams => ErrorCode::JsonRpcInvalidParams,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_code = self.error_code();
        let status_code = match self {
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::GridItemNotFound => StatusCode::NOT_FOUND,
            AppError::GridItemCreationFailed => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::GridItemUpdateFailed => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JsonRpcParseError => StatusCode::BAD_REQUEST,
            AppError::JsonRpcMethodNotFound => StatusCode::NOT_FOUND,
            AppError::JsonRpcInvalidParams => StatusCode::BAD_REQUEST,
        };

        let error_response = ErrorResponse {
            success: false,
            error: ErrorInfo {
                code: error_code.code(),
                message: error_code.message().to_string(),
            },
        };

        (status_code, Json(error_response)).into_response()
    }
}

/// 将标准错误转换为 AppError
impl From<serde_json::Error> for AppError {
    fn from(_err: serde_json::Error) -> Self {
        AppError::InternalError
    }
}

impl From<std::io::Error> for AppError {
    fn from(_err: std::io::Error) -> Self {
        AppError::InternalError
    }
}

impl From<config::ConfigError> for AppError {
    fn from(_err: config::ConfigError) -> Self {
        AppError::InternalError
    }
}
