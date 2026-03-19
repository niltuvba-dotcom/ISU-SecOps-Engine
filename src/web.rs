use axum::{
    extract::{Json, ws::{WebSocketUpgrade, WebSocket, Message}},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router, http::{StatusCode, header, Uri},
};
use futures::{sink::SinkExt, stream::StreamExt};
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

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
        .route("/api/ws/fingerprint", get(ws_handler))
        .fallback(static_handler);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("🌐 SEC-OPS Engine UI is live! Open http://127.0.0.1:{} in your browser.", port);
    println!("Press Ctrl+C to stop the server.");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    if let Some(Ok(Message::Text(text))) = socket.next().await {
        if let Ok(payload) = serde_json::from_str::<FingerprintRequest>(&text) {
            let parsed_ports: Result<Vec<u16>, _> = payload.ports.split(',')
                .map(|p| p.trim().parse::<u16>())
                .collect();

            if let Ok(ports) = parsed_ports {
                let concurrency = payload.concurrency.unwrap_or(100);
                let timeout_sec = payload.timeout.unwrap_or(3);
                let (tx, mut rx) = mpsc::unbounded_channel();

                // Start scanning in background
                tokio::spawn(async move {
                    let _ = fingerprint::run_fingerprint_streaming(&payload.target, ports, concurrency, timeout_sec, tx).await;
                });

                // Stream results back to socket
                while let Some(res) = rx.recv().await {
                    if let Ok(json) = serde_json::json!(res).to_string().try_into() {
                        if socket.send(Message::Text(json)).await.is_err() {
                            break;
                        }
                    }
                }
                
                // Send completion message
                let _ = socket.send(Message::Text("DONE".into())).await;
            }
        }
    }
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
