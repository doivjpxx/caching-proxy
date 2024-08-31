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
    headers.insert("Content-type", HeaderValue::from_static("text/html"));

    let response = reqwest::get(&origin).await.unwrap().text().await;

    if let Err(e) = response {
        return Err(e.to_string());
    }

    let response = response.unwrap();

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
        if let Some(_) = cache.get(&uri_string) {
            headers.insert("X-Cache", HeaderValue::from_static("HIT"));

            return Ok((headers, response));
        }
    }

    headers.insert("X-Cache", HeaderValue::from_static("MISS"));

    CACHE.lock().await.insert(uri_string, response.clone());

    Ok((headers, response))
}

pub async fn clear_cache() {
    CACHE.lock().await.clear();
}
