//! gRPC User service module
//!
//! Implements gRPC service for User functionality.
//!
//! Copyright © 2025 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

use crate::protos::user::{user_service_server::UserService, *};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct UserServiceImpl {
    next_id: Arc<AtomicU32>,
}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    type SubscribeUserUpdatesStream = ReceiverStream<Result<UserUpdate, Status>>;

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let user_id = request.into_inner().user_id;

        if user_id == 0 {
            return Err(Status::not_found("User not found"));
        }

        Ok(Response::new(GetUserResponse {
            user: Some(User {
                id: user_id,
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
                age: 30,
            }),
        }))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();
        let user_id = self.next_id.fetch_add(1, Ordering::SeqCst) as i32 + 1;

        Ok(Response::new(CreateUserResponse {
            user: Some(User {
                id: user_id,
                name: req.name,
                email: req.email,
                age: req.age,
            }),
        }))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        let req = request.into_inner();

        if req.user_id == 0 {
            return Err(Status::not_found("User not found"));
        }

        Ok(Response::new(UpdateUserResponse {
            user: Some(User {
                id: req.user_id,
                name: req.name,
                email: req.email,
                age: req.age,
            }),
        }))
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        let user_id = request.into_inner().user_id;

        if user_id == 0 {
            return Err(Status::not_found("User not found"));
        }

        Ok(Response::new(DeleteUserResponse {
            success: true,
            message: format!("User {} deleted successfully", user_id),
        }))
    }

    async fn subscribe_user_updates(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeUserUpdatesStream>, Status> {
        let req = request.into_inner();
        let user_id = req.user_id;
        let interval = if req.interval_seconds > 0 {
            Duration::from_secs(req.interval_seconds as u64)
        } else {
            Duration::from_secs(2)
        };

        let (tx, rx) = mpsc::channel(32);

        tokio::spawn(async move {
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

                let user_update = UserUpdate {
                    user: Some(User {
                        id: user_id,
                        name: format!("User {}", user_id),
                        email: format!("user{}@example.com", user_id),
                        age: 30 + (counter % 10),
                    }),
                    update_type: update_type.to_string(),
                    timestamp: timestamp as i64,
                };

                if tx.send(Ok(user_update)).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
