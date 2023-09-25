use std::{path::PathBuf,sync::Arc};
use axum_macros::debug_handler;
use utils::File;
use errors::PromptError;
use finder::Finder;

use serde::Deserialize;

mod utils;
mod open_ai;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder="static")] static_foldder: PathBuf,
    #[shuttle_static_folder::StaticFolder(folder="docs")] docs_folder: PathBuf,
    #[shuttle_static_folder::StaticFolder(folder=".")] prefix: PathBuf,
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum{
    let embedding = false;
    open_ai::setup(&secrets)?;

    let files = utils::load_files_from_dir(docs_folder,".mdx",&prefix)?;

    let app_state = AppState {files, vector_db};
    let app_state = Arc::new(Mutex::new(app_state));

    let router = Router::new()
        .route("/prompt",post(prompt))
        .route("/embedd",get(embed))
        .nest_service("/",ServeDir::new(static_foldder))
        .with_state(app_state);

    Ok(router.into())
}