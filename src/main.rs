use axum::{http::Uri, response::IntoResponse, routing::get, Router};
#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|| async {"Hello, world!".to_string()}),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
