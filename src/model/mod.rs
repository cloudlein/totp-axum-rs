use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder};
use tracing_subscriber::registry::Data;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateDto{
    #[validate(length(min = 1, max = 255, message = "name cannot be empty"))]
    pub name: String,
    #[validate(length(min = 4, max = 255, message = "username must be at least 4 character long"))]
    pub username: String,
    #[validate(email(message = "invalid email format"))]
    pub email: String,
    #[validate(length(min = 6 , message= "password must be at least 6 characters long"))]
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateDto {
    pub name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>
}

pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>
}

#[derive(Serialize)]
pub struct PaginateResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub limit: u32,
    pub total: i64
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery{
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>
}

impl <T: Serialize> ApiResponse<T> {
    pub fn success(message: impl Into<String>, data: T) -> Self{
        Self {
            success: true,
            message: message.into(),
            data: Some(data)
        }
    }
}
impl PaginationQuery {
    pub fn apply_filters(self, builder: &mut QueryBuilder<Postgres>){
        if let Some(search) = &self.search {
            let pattern = format!("%{}%", search);

            builder.push(" WHERE (name ILIKE ");
            builder.push_bind(pattern.clone());
            builder.push("username ILIKE ");
            builder.push_bind(pattern.clone());
            builder.push("email ILIKE ");
            builder.push_bind(pattern);
            builder.push(")");
        }
    }
}