use axum::{routing::get, Extension, Router};

use crate::{
    handlers::{create_todo, delete_todo, get_all_todos, get_todo, update_todo},
    models::DB,
};

pub fn create_routes(db: DB) -> Router {
    Router::new()
        .route("/todos", get(get_all_todos).post(create_todo))
        .route(
            "/todos/:id",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        .layer(Extension(db))
}
