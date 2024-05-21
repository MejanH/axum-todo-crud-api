use std::sync::{Arc, Mutex, RwLock};

use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateTodo {
    pub title: String,
    pub completed: bool,
}

pub type DB = Arc<RwLock<Vec<Todo>>>;

pub type TodoError = (StatusCode, Json<serde_json::Value>);
