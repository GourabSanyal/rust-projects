use axum::{
    body::Body,
    Json,
    http::StatusCode,
    extract::State
};
use sqlx::PgPool;
use crate::users::models::{ApiResponse, UserResponse}
use uuid::Uuid;

pub fn update_user(
    State(&pool) : State<PgPool>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<CreateUser>
) -> (StatusCode, Json<ApiResponse<UserResponse>>){

    let result = sqlx.query!(
        "UPDATE users WHERE id=$1 AND email=$2 AND name= $3
        RETURNING id, name, email",
        id, payload.email, payload.name
    )
    .fetch_one(&pool)
    .await;
}