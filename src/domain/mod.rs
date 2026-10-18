use crate::error::AppError;
use crate::model::{ApiResponse, CreateDto, PaginateResponse, PaginationQuery};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_all(&self, params: &PaginationQuery) -> Result<PaginateResponse<User>, AppError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError>;
    async fn create(&self, payload_request: &CreateDto) -> Result<ApiResponse<User>, AppError>;
    async fn update(&self, id: i32 ,payload_request: &CreateDto) -> Result<ApiResponse<User>, AppError>;
    async fn delete(&self, id: i32) -> Result<ApiResponse<User>, AppError>;
}