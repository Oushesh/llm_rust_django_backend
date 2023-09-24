use axum::prelude::*;
use axum_multipart::Multipart;
use axum::response::Response;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use transformers::{LanguageModel, LlmEmbedding};
use pinecone_sdk::pinecone::{Document, Pinecone};
use qdrant_sdk::index::{Index, IndexConfig};
use std::io::Write;
use tempfile::NamedTempFile;
use hyper::StatusCode;
use std::convert::Infallible;

mod utils;

#[derive(Debug, Serialize, Deserialize)]
struct Document {
    // Define your document data structure here
    // You can add fields like title, content, etc.
}

fn get_embeddings(markdown: &str, model: &LanguageModel) -> Vec<f32> {
    // Tokenize the Markdown content (customize as needed)
    let tokens = model.tokenize(markdown);

    // Generate embeddings from the model
    let embeddings = model.embed(&tokens).unwrap();

    embeddings
}

//File uploader.
pub async fn upload_file(
    mut payload: axum::extract::Multipart,
    model: axum::extract::Extension<LanguageModel>,
    pinecone: axum::extract::Extension<Pinecone>,
    qdrant: axum::extract::Extension<Index>,
) -> Result<impl axum::IntoResponse, Infallible> {
    while let Some(item) = payload.next().await {
        let field = item.unwrap();
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();

        // Define a temporary file to save the uploaded content
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let mut file = std::fs::File::create(temp_file.path()).unwrap();

        // Write the content of the uploaded file to the temporary file
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file.write_all(&data).unwrap();
        }
        // Read the Markdown content from the temporary file
        let markdown = std::fs::read_to_string(temp_file.path()).unwrap();

        // Obtain embeddings from the Rust Transformers LLM model
        let embeddings = get_embeddings(&markdown, &model);

        // Upload embeddings to Pinecone or Qdrant
        upload_to_pinecone_or_qdrant(embeddings, &pinecone, &qdrant);

        // Delete the temporary file
        temp_file.close().unwrap();
    }

    Ok(Response::new(StatusCode::OK))
}
