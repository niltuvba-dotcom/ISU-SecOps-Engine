use axum::{
    extract::Json,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router, http::{StatusCode, header, Uri},
};
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::fingerprint;

#[derive(RustEmbed)]
#[folder = "web/"]
struct Asset;

#[derive(Deserialize)]
pub struct FingerprintRequest {
    pub target: String,
    pub ports: String,
    pub concurrency: Option<usize>,
    pub timeout: Option<u64>,
}

pub async fn start_server(port: u16) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/api/fingerprint", post(api_fingerprint))
        .fallback(static_handler);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("🌐 SEC-OPS Engine UI is live! Open http://127.0.0.1:{} in your browser.", port);
    println!("Press Ctrl+C to stop the server.");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn api_fingerprint(Json(payload): Json<FingerprintRequest>) -> impl IntoResponse {
    let parsed_ports: Result<Vec<u16>, _> = payload.ports.split(',')
        .map(|p| p.trim().parse::<u16>())
        .collect();

    match parsed_ports {
        Ok(ports) => {
            let concurrency = payload.concurrency.unwrap_or(100);
            let timeout_sec = payload.timeout.unwrap_or(3);
            
            match fingerprint::run_fingerprint_logic(&payload.target, ports, concurrency, timeout_sec).await {
                Ok(results) => (StatusCode::OK, Json(results)).into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response(),
            }
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Invalid port format".to_string()).into_response(),
    }
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.is_empty() {
        path = "index.html".to_string();
    }

    match Asset::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}
