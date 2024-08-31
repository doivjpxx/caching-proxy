use std::collections::HashMap;

use axum::{
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static CACHE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn handle_request(origin: String, port: u16) -> Result<impl IntoResponse, String> {
    let mut headers = HeaderMap::new();

    let uri_string = format!(
        "{}{}",
        origin,
        if port == 80 {
            "".to_string()
        } else {
            format!(":{}", port)
        }
    );

    {
        let cache = CACHE.lock().await;
        if let Some(value) = cache.get(&uri_string) {
            headers.insert("X-Cache", HeaderValue::from_static("HIT"));

            return Ok((headers, "from cache: ".to_string() + value));
        }
    }

    headers.insert("X-Cache", HeaderValue::from_static("MISS"));

    let response = "Hello, world!".to_string();

    CACHE.lock().await.insert(uri_string, response.clone());

    Ok((headers, response))
}

pub async fn clear_cache() {
    CACHE.lock().await.clear();
}
