mod routes;
mod  users;

use routes::api_router;

#[tokio::main]
async fn main() {
    let app = api_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server is listening to PORT 3000");
    axum::serve(listener, app).await.unwrap();
}
