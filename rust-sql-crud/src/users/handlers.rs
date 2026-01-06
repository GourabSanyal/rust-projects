// app logic for all the route handlers
use axum::{body::Body, Json, http::StatusCode};

pub async fn root() -> (StatusCode, Json<Vec<String>>) {
    let data =  vec!["Welcome to CRUD built using Rust and SQL".to_string()];
    (StatusCode::OK, Json(data))
}

pub async fn create() {}
pub async fn all_users() {}
pub async fn update() {}
pub async fn delete_user() {}