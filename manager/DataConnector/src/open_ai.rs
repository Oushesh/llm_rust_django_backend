use shuttle_secrets::SecretStore;
use openai::{
    chat::{ChatCompletion, ChatCompletionBuilder, ChatCompletionDelta, ChatCompletionMessage},
    embeddings::{Embedding, Embeddings},
};
use anyhow::Result;

pub fn setup(secrets: &SecretStore)-> Result<()>{
    let openai_key = secrets
        .get("OPENAI_API_KEY")
        .ok_or(SetupError("OPENAI Key not available"))?;
    openai_key::set_key

}