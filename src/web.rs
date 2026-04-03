use axum::{
    extract::{Json, ws::{WebSocketUpgrade, WebSocket, Message}},
    response::IntoResponse,
    routing::{get, post},
    Router, http::{StatusCode, header},
};
use futures::stream::StreamExt;
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::sync::mpsc;

use crate::fingerprint;
use crate::database;

/// Asset embedding for the web interface.
#[derive(RustEmbed)]
#[folder = "web/"]
struct Assets;

/// Data structure for incoming scan requests via HTTP.
#[derive(Deserialize)]
struct ScanRequest {
    target: String,
    ports: String,
    concurrency: Option<usize>,
    timeout: Option<u64>,
}

/// Starts the Axum web server on the specified port.
pub async fn start_server(port: u16) -> anyhow::Result<()> {
    // Initialize database
    database::init_db()?;

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/api/scan", post(scan_handler))
        .route("/api/history", get(history_handler))
        .route("/ws", get(ws_handler))
        .fallback(static_handler);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    axum::serve(listener, app).await?;
    Ok(())
}

/// Handler for the main index page.
async fn index_handler() -> impl IntoResponse {
    match Assets::get("index.html") {
        Some(content) => ([(header::CONTENT_TYPE, "text/html")], content.data).into_response(),
        None => (StatusCode::NOT_FOUND, "Index Not Found").into_response(),
    }
}

/// Handler for retrieving scan history from the SQLite database.
async fn history_handler() -> impl IntoResponse {
    match database::get_history() {
        Ok(history) => (StatusCode::OK, Json(history)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// Handler for standard HTTP scan requests (returns all results at once).
async fn scan_handler(Json(payload): Json<ScanRequest>) -> impl IntoResponse {
    let ports: Vec<u16> = payload.ports.split(',')
        .filter_map(|p| p.trim().parse::<u16>().ok())
        .collect();

    let concurrency = payload.concurrency.unwrap_or(100);
    let timeout = payload.timeout.unwrap_or(3);

    // This is the non-streaming version for standard REST
    let (tx, mut rx) = mpsc::unbounded_channel();
    let target_for_scan = payload.target.clone();
    
    tokio::spawn(async move {
        let _ = fingerprint::run_fingerprint_streaming(&target_for_scan, ports, concurrency, timeout, tx).await;
    });

    let mut results = vec![];
    while let Some(res) = rx.recv().await {
        results.push(res);
    }

    // Save to history before returning
    let _ = database::save_scan(&payload.target, &results);

    (StatusCode::OK, Json(results))
}

/// WebSocket handler for real-time result streaming.
async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

/// Internal logic for managing a WebSocket connection.
async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(text) = msg {
            if let Ok(req) = serde_json::from_str::<ScanRequest>(&text) {
                let ports: Vec<u16> = req.ports.split(',')
                    .filter_map(|p| p.trim().parse::<u16>().ok())
                    .collect();

                let (tx, mut rx) = mpsc::unbounded_channel();
                let target_for_scan = req.target.clone();
                let target_for_db = req.target.clone();
                let concurrency = req.concurrency.unwrap_or(100);
                let timeout = req.timeout.unwrap_or(3);

                tokio::spawn(async move {
                    let _ = fingerprint::run_fingerprint_streaming(&target_for_scan, ports, concurrency, timeout, tx).await;
                });

                let mut all_results = vec![];
                while let Some(res) = rx.recv().await {
                    all_results.push(res.clone());
                    let _ = socket.send(Message::Text(serde_json::to_string(&res).unwrap())).await;
                }
                
                // Save complete scan to history using the separate clone
                let _ = database::save_scan(&target_for_db, &all_results);
            }
        }
    }
}

/// Static file server for embedded assets (HTML, CSS, JS).
async fn static_handler(uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    
    if path.is_empty() {
        return index_handler().await.into_response();
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => (StatusCode::NOT_FOUND, "Not Found").into_response(),
    }
}
