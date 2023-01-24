#[macro_use]
extern crate lazy_static;
use axum::{
    body::Bytes,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::Mutex;
use uuid::Uuid;

lazy_static! {
    static ref NOTES: Mutex<HashMap<Uuid, String>> = Mutex::new(HashMap::new());
}

#[derive(Serialize)]
struct Message {
    message: String,
}

#[derive(Serialize)]
#[serde(untagged)]
enum NoteOrError {
    Note { id: Uuid, content: String },
    Error { message: String },
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/note", post(create_note))
        .route("/note/:id", get(get_note));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// TODO: generalise state rather than lazy static
async fn create_note(body: Bytes) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let body_content: Vec<u8> = body.into_iter().collect();
    // TODO: handle error
    let body_content_str = String::from_utf8(body_content).unwrap();

    let mut guard = NOTES.lock().await;
    // TODO: remove the clone, if you can! (maybe references)
    (*guard).insert(id, body_content_str.clone());

    let new_note = NoteOrError::Note {
        id,
        content: body_content_str,
    };

    (StatusCode::CREATED, Json(new_note))
}

async fn get_note(Path(id): Path<Uuid>) -> impl IntoResponse {
    let guard = NOTES.lock().await;

    let note = (*guard).get(&id);

    if note.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(NoteOrError::Error {
                message: "Not found".to_string(),
            }),
        );
    }

    let message = note.unwrap();
    let note = NoteOrError::Note {
        id,
        content: message.clone(),
    };
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
