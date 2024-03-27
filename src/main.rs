use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyRequestType {
    message: String,
}

async fn hello_world(Path(id): Path<i32>) -> impl IntoResponse {
    let string = format!("Hello, world! {}", id);
    (StatusCode::OK, string)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/:id", get(hello_world));

    Ok(router.into())
}
