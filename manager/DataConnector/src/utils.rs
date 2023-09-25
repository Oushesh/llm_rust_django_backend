pub fn get_embeddings(markdown: &str, model: &LanguageModel) -> Vec<f32> {
    // Tokenize the Markdown content (customize as needed)
    let tokens = model.tokenize(markdown);

    // Generate embeddings from the model
    let embeddings = model.embed(&tokens).unwrap();

    embeddings
}


pub fn upload_to_pinecone_or_qdrant(embeddings: Vec<f32>)
{
    // Upload embeddings to Pinecone or Qdrant
    // You should use Pinecone or Qdrant SDKs to perform the actual upload.
    // Refer to the respective SDK documentation
}
