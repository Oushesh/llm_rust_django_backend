use axum::prelude::*;
use axum::Router;
use axum::routing::nest;
use axum::handler::{get, post};
use axum::AddExtension;
use axum::serve::bind;
use axum::routing::nest::Nest;
use std::net::SocketAddr;
use transformers::{LanguageModel, LlmEmbedding};
use pinecone_sdk::pinecone::{Document, Pinecone};
use qdrant_sdk::index::{Index, IndexConfig};
use axum::handler::delete;


mod utils;
mod uploader;

async fn hello_world() -> &'static str
{
    "Hello, world!"
}

//The macros of shuttle_runtime main allows
//you to run async fn axum as main.
#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum
{
    let model = LanguageModel::new("path/to/your/model").unwrap();
    let pinecone = Pinecone::new("pinecone_api_key");
    let qdrant = Index::new("qdrant_api_key",IndexConfig::default());
    let app = nest(
        "/",
        Router::new()
            .route("/", get(hello_world))
            .route("/upload", post(uploader::upload_file))
            .nest("/assets", axum_static_files::StaticFiles::new("assets"))
    )
        .layer(model)
        .layer(pinecone)
        .layer(qdrant);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
