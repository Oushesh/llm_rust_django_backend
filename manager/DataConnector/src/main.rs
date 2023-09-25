use std::{path::PathBuf,sync::Arc};
use axum_macros::debug_handler;
use contents::File;
use errors::PromptError;
use finder::Finder;

use serde::Deserialize;

mod utils;
mod open_ai;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder="static")] static_foldder: PathBuf,
    #[shuttle_static_folder::]
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum{
    let embedding = false;
    open_ai::

}