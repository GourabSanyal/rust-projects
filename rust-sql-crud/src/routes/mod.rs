use axum::Router;
use crate::users::routes::user_router;
use sqlx::PgPool;

pub fn api_router() -> Router<PgPool> {
    Router::new()
        .nest("/users", user_router())
}