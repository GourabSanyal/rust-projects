use axum::{
    body::Body,
    Json,
    http::StatusCode,
    extract::State
};
use sqlx::PgPool;
use crate::users::models::{ApiResponse, UserResponse, CreateUser}
use uuid::Uuid;

pub async fn update_user(
    State(pool) : State<PgPool>,
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

    match result {
        Ok(recors) => {
            
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::Error{
                    message: format!("Failed to update user details {}", e)
                }))
        }
    }
}