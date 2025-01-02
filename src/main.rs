use axum::{
    routing::{get, post},
    Json, Router,
    http::StatusCode,
    extract::State,
};
use std::sync::{Arc, Mutex};
use serde_json::json;
use std::net::SocketAddr;
use tokio::signal;

#[derive(Clone)]
struct AppState {
    simulate_error: Arc<Mutex<bool>>,
}

#[tokio::main]
async fn main() { 
    let state = AppState {
        simulate_error: Arc::new(Mutex::new(false)),
    };
 
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/simulate", post(simulate_handler))
        .with_state(state);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running at http://{}/", addr);

    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service());
 
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(err) = graceful.await {
        eprintln!("Server error: {}", err);
    }
}

async fn shutdown_signal() { 
    let _ = signal::ctrl_c().await;
    println!("Signal received, shutting down...");
}

async fn health_handler(State(state): State<AppState>) -> (StatusCode, &'static str) {
    let simulate_error = state.simulate_error.lock().unwrap();
    if *simulate_error {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    } else {
        (StatusCode::OK, "OK")
    }
}

async fn simulate_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    let mut simulate_error = state.simulate_error.lock().unwrap();
    *simulate_error = !*simulate_error;

    Json(json!({
        "simulate_error": *simulate_error
    }))
}