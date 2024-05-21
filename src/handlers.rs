use std::io;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;
use uuid::Uuid;

use crate::models::{CreateTodo, Todo, TodoError, DB};

pub async fn get_all_todos(Extension(db): Extension<DB>) -> impl IntoResponse {
    let db = db.read().unwrap();
    let result = Json(db.clone());

    println!("{:?}", result);
    result
}

pub async fn create_todo(
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let mut db = db.write().unwrap();
    let new_todo = Todo {
        id: Uuid::new_v4(),
        title: payload.title,
        completed: payload.completed,
    };

    db.push(new_todo.clone());

    (StatusCode::CREATED, Json(new_todo))
}

pub async fn get_todo(
    Path(id): Path<Uuid>,
    Extension(db): Extension<DB>,
) -> Result<impl IntoResponse, TodoError> {
    let db = db.read().unwrap();
    if let Some(todo) = db.iter().find(|todo| todo.id == id) {
        Ok((StatusCode::OK, Json(todo.clone())))
    } else {
        let error = json!({
            "message": "todo not found",
        });
        return Err((StatusCode::NOT_FOUND, Json(error)));
    }
}

pub async fn update_todo(
    Extension(db): Extension<DB>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateTodo>,
) -> Result<impl IntoResponse, TodoError> {
    let mut db = db.write().unwrap();

    if let Some(todo) = db.iter_mut().find(|t| t.id == id) {
        todo.completed = payload.completed;
        todo.title = payload.title.clone();
        Ok((StatusCode::OK, Json(todo.clone())))
    } else {
        let error = json!({
            "message": "todo not found",
        });
        return Err((StatusCode::NOT_FOUND, Json(error)));
    }
}

pub async fn delete_todo(
    Path(id): Path<Uuid>,
    Extension(db): Extension<DB>,
) -> Result<impl IntoResponse, TodoError> {
    let mut db = db.write().unwrap();
    if let Some(index) = db.iter().position(|todo| todo.id == id) {
        db.remove(index);
        Ok(StatusCode::NO_CONTENT)
    } else {
        let error = json!({
            "message": "todo not found",
        });
        return Err((StatusCode::NOT_FOUND, Json(error)));
    }
}
