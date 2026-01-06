use axum::Router;
use crate::users::routes::user_router;

pub fn api_router() -> Router {
    Router::new()
        .nest("/users", user_router())
}