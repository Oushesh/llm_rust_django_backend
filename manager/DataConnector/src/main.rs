use std::{path::PathBuf, sync::Arc};

use axum::{extract::Json, extract::State, response::IntoResponse, routing::post, Router};
use axum_macros::debug_handler;

use shuttle_secrets::SecretStore;
//use errors::PromptError;
//use finder::Finder;
use tower_http::services::ServeDir;
use vector::VectorDB;

mod utils;
mod open_ai;

#[derive(Deserialize)]
struct Prompt {
    prompt: String,
}


#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
    #[shuttle_static_folder::StaticFolder(folder = "docs")] docs_folder: PathBuf,
    #[shuttle_static_folder::StaticFolder(folder = ".")] prefix: PathBuf,
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let embedding = false;
    open_ai::setup(&secrets)?;
    let mut vector_db = VectorDB::new(&secrets)?;
    let files_result = utils::load_files_from_dir(docs_folder, ".mdx", &prefix)?;

    match files_result {
        Ok(files) => {
            // Handle the case where loading files succeeded
            // You can work with the 'files' Vec<File> here
        }
        Err(err) => {
            eprintln!("Error loading files: {}", err);
            // Handle the case where an error occurred
        }
    }

    println!("Setup done");

    if embedding {
        vector_db.reset_collection().await?;
        embed_documentation(&mut vector_db, &files).await?;
        println!("Embedding done");
    }

    let app_state = AppState { files, vector_db };
    let app_state = Arc::new(app_state);

    let router = Router::new()
        .route("/prompt", post(prompt))
        .nest_service("/", ServeDir::new(static_folder))
        .with_state(app_state);
    Ok(router.into())
}