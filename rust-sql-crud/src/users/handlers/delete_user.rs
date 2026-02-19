use axum::{Json, http::StatusCode, extract::{State, Path}};
use sqlx::PgPool;
use crate::users::models::{ApiResponse, UserResponse, CreateUser, User};
use uuid::Uuid;

pub async fn delete_user (
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> (StatusCode, Json<ApiResponse<UserResponse>>){
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1
        RETURNING id, name, email
        ",
        id
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(records) => {
            let response = UserResponse{
                id: records.id.to_string(),
                name: records.name,
                email: records.email
            };

            (StatusCode::OK,
                Json(ApiResponse::Success{
                    data: response,
                    message: "User deleted successfully".to_string()
                })
            )
        },
          Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::Error{
                    message: format!("Failed to fetch all the users: {}", e)
                })
            )
        }
    }
}