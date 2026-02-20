use axum::{Router, routing::{get, post, put, delete}};
use crate::users::handlers::{
    root_path,
    create,
    all_users,
    update_user,
    delete_user
};
use sqlx::PgPool;

pub fn user_router() -> Router<PgPool> {
    Router::new()
        .route("/", get(root_path))
        .route("/create", post(create))
        .route("/getallusers", get(all_users))
        .route("/update/:id", put(update_user))
        .route("/delete/:id", delete(delete_user))
}