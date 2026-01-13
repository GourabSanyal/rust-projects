use axum::{http::StatusCode, Json};

pub async fn root_path() -> (StatusCode, Json<Vec<String>>){
    let data = vec!["Welcome to the CRUD build using Rus and SQL".to_string()];
    (StatusCode::OK, Json(data))
}