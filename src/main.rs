use axum::{
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::{env, net::SocketAddr, sync::Arc};
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

type Conn = Arc<Pool<Sqlite>>;

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let state_pool = Arc::new(pool);

    let app = Router::new()
        .route("/note", post(create_note))
        .route("/note/:id", get(get_note))
        .with_state(state_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// TODO: generalise state rather than lazy static
async fn create_note(State(pool): State<Conn>, body: Bytes) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let body_content: Vec<u8> = body.into_iter().collect();
    // TODO: handle error
    let body_content_str = String::from_utf8(body_content).unwrap();

    // TODO: handle error properly
    //let mut conn = pool.acquire().await.unwrap();

    sqlx::query(
        r#"
    INSERT INTO notes ( id, content )
    VALUES ( ?1, ?2 )
            "#,
    )
    .bind(id.to_string().as_str())
    .bind(body_content_str.clone())
    .execute(&(*pool))
    .await
    .unwrap();

    let new_note = Note {
        id,
        content: body_content_str,
    };

    (StatusCode::CREATED, Json(new_note))
}

async fn get_note(State(pool): State<Conn>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let row = sqlx::query("SELECT content FROM notes WHERE id = ?")
        .bind(id.to_string().as_str())
        .fetch_one(&(*pool))
        .await
        .unwrap();

    let content: &str = row.try_get("content").unwrap();

    // TODO: figure out how to check if the record is missing
    // if note.is_none() {
    //     let error = Message {
    //         message: "Not found".to_string(),
    //     };
    //     let error: serde_json::Value = serde_json::to_value(error).unwrap();
    //     return (StatusCode::NOT_FOUND, Json(error));
    // }

    let note = Note {
        id,
        content: content.to_string(),
    };
    let note: serde_json::Value = serde_json::to_value(note).unwrap();
    (StatusCode::OK, Json(note))
}
