use std::env;
use reqwest;
use serde_json;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Get the natural language input from the command line arguments
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");

    // Call OpenAI LLM API to get the suggested ffmpeg command
    let ffmpeg_command = get_ffmpeg_command_from_llm(&input).await?;

    // Execute the generated ffmpeg command
    execute_ffmpeg_command(&ffmpeg_command).map_err(|e| reqwest::Error::from(e))?;

    Ok(())
}

async fn get_ffmpeg_command_from_llm(input: &str) -> Result<String,reqwest::Error> {
    // Use the OpenAI API endpoint (replace )
    let api_endpoint = "https://api.openai.com/v1/engines/davinci/completions";
    let api_key = "";

    // Prepare the request payload
    let request_payload = json!({
        "prompt": input,
        "max_tokens": 100,
    });

    // Make a POST request to the OpenAI API
    let client = reqwest::Client::new();
    /*
    let response = client
        .post(api_endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_payload)
        .send()
        .await?;
    */

    let response = client
        .post(api_endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .body(serde_json::to_string(&request_payload).unwrap()) // Serialize manually
        .send()
        .await?;

    // Parse and return the generated ffmpeg command
    let result: serde_json::Value = response.json().await?;
    let ffmpeg_command = result["choices"][0]["text"].as_str().unwrap_or_default().to_string();
    Ok(ffmpeg_command)
}

fn execute_ffmpeg_command(ffmpeg_command: &str) -> Result<(), reqwest::Error> {
    // Print the generated ffmpeg command

    // TODO: Execute the ffmpeg command using std::process::Command
    // Example:
    // let output = std::process::Command::new("ffmpeg")
    //     .arg("-i")
    //     .arg("input.mp4")
    //     .arg("output.mp4")
    //     .output()?;
    // println!("ffmpeg command output: {:?}", output);
    Ok(())
}
