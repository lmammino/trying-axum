use axum::{
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[derive(Serialize)]
struct Note {
    id: Uuid,
    content: String,
}

type AppState = Arc<Mutex<HashMap<Uuid, String>>>;

#[tokio::main]
async fn main() {
    let shared_state: AppState = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/note", post(create_note))
        .route("/note/:id", get(get_note))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// TODO: generalise state rather than lazy static
async fn create_note(State(state): State<AppState>, body: Bytes) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let body_content: Vec<u8> = body.into_iter().collect();
    // TODO: handle error
    let body_content_str = String::from_utf8(body_content).unwrap();

    let mut guard = state.lock().await;
    // TODO: remove the clone, if you can! (maybe references)
    (*guard).insert(id, body_content_str.clone());

    let new_note = Note {
        id,
        content: body_content_str,
    };

    (StatusCode::CREATED, Json(new_note))
}

async fn get_note(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let guard = state.lock().await;

    let note = (*guard).get(&id);

    if note.is_none() {
        let error = Message {
            message: "Not found".to_string(),
        };
        let error: serde_json::Value = serde_json::to_value(error).unwrap();
        return (StatusCode::NOT_FOUND, Json(error));
    }

    let message = note.unwrap();
    let note = Note {
        id,
        content: message.clone(),
    };
    let note: serde_json::Value = serde_json::to_value(note).unwrap();
    (StatusCode::OK, Json(note))
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn hello() -> impl IntoResponse {
    let message = Message {
        message: "Hello, Json!".to_string(),
    };

    (StatusCode::OK, Json(message))
}
