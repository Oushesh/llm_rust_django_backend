use std::path::PathBuf;

use axum::{routing::get, Router};
use tower_http::services::ServeDir;

mod utils;
use utils::get_embeddings:

mod uploader;
use uploader::

async fn hello_world() -> &'static str {
    "Hello, world!"
}



//The macros of shuttle_runtime main allows
//you to run async fn axum as main.
#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/upload",post(upload_file))
        .nest_service("/assets", ServeDir::new(PathBuf::from("assets")))
        .layer(tide::middleware::StateMiddleware::new(model))
        .layer(tide::middleware::StateMiddleware::new(pinecone))
        .layer(tide::middleware::StateMiddleware::new(qdrant));

    let port = "127.0.0.1:8080";
    let addr = port.parse()?;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(router.into())
}
