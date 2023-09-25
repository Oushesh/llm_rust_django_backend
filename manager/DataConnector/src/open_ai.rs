use shuttle_secrets::SecretStore;
use openai::{
    chat::{ChatCompletion, ChatCompletionBuilder, ChatCompletionDelta, ChatCompletionMessage},
    embeddings::{Embedding, Embeddings},
};
use openai::set_api_key;
use anyhow::Result;

pub fn setup(secrets: &SecretStore) -> Result<()> {
    let openai_key = secrets
        .get("OPENAI_API_KEY")
        .ok_or(Error::msg("OPENAI Key not available"))?; // Use Error::msg to create an error message

    // Set the OpenAI API key using the openai crate
    set_api_key(openai_key);

    Ok(())
}

pub async fn embed_file(file: &File) -> Result<Embeddings> {
    let sentence_as_str: Vec<&str> = file.sentences.iter().map(|s| s.as_str()).collect();
    Embeddings::create("text-embedding-ada-002", sentence_as_str, "stefan")
        .await
        .map_err(|_| EmbeddingError {}.into())
}