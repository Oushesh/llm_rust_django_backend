pub fn get_embeddings(markdown: &str, model: &LanguageModel) -> Vec<f32> {
    // Tokenize the Markdown content (customize as needed)
    let tokens = model.tokenize(markdown);

    // Generate embeddings from the model
    let embeddings = model.embed(&tokens).unwrap();

    embeddings
}