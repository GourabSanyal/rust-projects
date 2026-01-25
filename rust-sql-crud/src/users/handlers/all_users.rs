use axum::{body::Body,
    Json,
    http::StatusCode,
    extract::State
};
use sqlx::PgPool;
use crate::users::models::{
    ApiResponse,
    UserResponse,
    PublicUser
};

pub async fn all_users(
    State(pool): State<PgPool>,
) -> (
    StatusCode,
    Json<ApiResponse<Vec<PublicUser>>>
) {
    let result = sqlx::query!(
        "SELECT id, name, email FROM users"
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(records) => {
            let response: Vec<PublicUser> = records.into_iter().map(|record| {
                PublicUser {
                    id: record.id.to_string(),
                    name: record.name,
                    email: record.email,
                }
            }).collect();

            (StatusCode::OK, 
                Json(ApiResponse::Success{
                    data: response,
                    message: "All users fetched successfully".to_string()
                })
            )
        },
        Err(e) => {
            println!("Error fetching all users: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::Error{
                    message: format!("Failed to fetch all the users: {}", e)
                })
            )
        }
    }
}