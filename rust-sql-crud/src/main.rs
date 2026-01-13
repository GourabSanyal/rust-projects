mod routes;
mod  users;

use routes::api_router;
use std::env;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let app = api_router().with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server is listening to PORT 3000");
    axum::serve(listener, app).await.unwrap();
}
