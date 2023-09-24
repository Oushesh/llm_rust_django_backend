use axum_tide_compat::AxumExt;
use tide::multipart::Multipart;
use tide::Body;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tide::{Request,Response,StatusCode};
use transformers::{LanguageModel,LlmEmbedding};
use pinecone_sdk::pinecone::{Document,Pinecone};
use qdrant_sdk::index::{Index,IndexConfig};
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;

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
    req: Request<()>,
    model: Extension<LanguageModel>,
    pinecone: Extension<Pinecone>,
    qdrant: Extension<Index>,
) -> tide::Result {
    let mut multipart = Multipart::new(req).await?;

    while let Some(mut field) = multipart.next().await {
        let content_disposition = field.content_disposition().ok_or_else(|| tide::Error::from_str(StatusCode::BadRequest, "Invalid Content-Disposition header"))?;
        let filename = content_disposition.get_filename().ok_or_else(|| tide::Error::from_str(StatusCode::BadRequest, "Missing filename"))?;

        // Define a temporary file to save the uploaded content
        let temp_file = NamedTempFile::new().map_err(|e| tide::Error::new(StatusCode::InternalServerError, e))?;
        let mut file = File::create(&temp_file).map_err(|e| tide::Error::new(StatusCode::InternalServerError, e))?;

        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| tide::Error::new(StatusCode::InternalServerError, e))?;
            file.write_all(&data).map_err(|e| tide::Error::new(StatusCode::InternalServerError, e))?;
        }
        // Read the Markdown content from the temporary file
        let markdown = std::fs::read_to_string(temp_file.path()).map_err(|e| tide::Error::new(StatusCode::InternalServerError, e))?;

        // Obtain embeddings from the Rust Transformers LLM model
        let embeddings = get_embeddings(&markdown, &model);

        // Upload embeddings to Pinecone or Qdrant
        upload_to_pinecone_or_qdrant(embeddings, &pinecone, &qdrant); //This one is our own function.

        // Delete the temporary file
        temp_file.close().map_err(|e| tide::Error::new(StatusCode::InternalServerError, e))?;
    }
    Ok(Response::new(StatusCode::Ok))
}

