use axum::{body::Body, Json, http::StatusCode, extract::State};
use sqlx::PgPool;
use crate::users::models::{ApiResponse, UserResponse, CreateUser};

pub async fn root() -> (StatusCode, Json<Vec<String>>) {
    let data =  vec!["Welcome to CRUD built using Rust and SQL".to_string()];
    (StatusCode::OK, Json(data))
}

pub async fn create(
    State(pool): State<PgPool>, 
    Json(payload): Json<CreateUser>
    ) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
    
    let result = sqlx::query!(
        "INSERT INTO users (name, email, password)
         VALUES ($1, $2, $3)
         RETURNING ID
        ",
        payload.name,
        payload.email,
        payload.password
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(record) => { //record gets us the ID from the DB query 'RETURNING ID'
            let response = UserResponse {
                id: record.id,
                name: payload.name,
                email: payload.email
            };

            (StatusCode::CREATED,
                Json(ApiResponse::Success {
                    data: response,
                    message: "User created successfully".to_string()
                })
            )
        },
        Err(_e) => {
            (StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::Error{
                    message: "Failed to update the user".to_string()
                })
            )
        }
    }
}