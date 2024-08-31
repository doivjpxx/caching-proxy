mod cli;
mod handler;

use axum::{routing::get, Router};
use clap::Parser;
use handler::handle_request;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    if args.clear_cache {
        handler::clear_cache().await;
        return;
    }

    let app = Router::new().route("/", get(move || handle_request(args.origin, args.port)));

    let addr = format!("0.0.0.0:{}", args.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
