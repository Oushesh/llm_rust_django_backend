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

async fn upload_file(
    req: Request<()>,
    model: tide::State<LanguageModel>,
    pinecone: tide::State<Pinecone>,
    qdrant: tide::State<Index>,
) -> tide::Result{
    let mut multipart =

}