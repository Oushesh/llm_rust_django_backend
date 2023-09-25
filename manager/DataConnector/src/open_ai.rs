use shuttle_secrets::SecretStore;

pub fn setup(secrets: &SecretStore)-> Result<()>{
    let openai_key = secrets
        .get("OPENAI_API_KEY")
        .ok_or(SetupError("OPENAI Key not available"))?;
    openai_key::set_key

}