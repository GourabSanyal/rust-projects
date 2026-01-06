// exports user route
//then imports in routes/mod.rs

use axum::{Router, routing::{get, post, put, delete}};
use crate::users::handlers::{root, create, all_users, update, delete_user};

pub fn user_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/create", post(create))
        .route("/getallusers", get(all_users))
        .route("/update/:id", put(update))
        .route("/delete/:id", delete(delete_user))
}