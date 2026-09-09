// This file contains the API for creating embedding
// We are using ollama with `nomic-embed-text` embedding model for now.
// Later we can read the embedding model from a env (from a file or actual env)

use crate::chunk::Chunk;
use reqwest::Client;
use serde_json::json;
use std::thread;
use std::time::Duration;

// We also need a function that can embed user query.
// Or we can refactor this function to contain that.
pub async fn create_embedding(chunks: Vec<Chunk>) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    for chunk in chunks {
        let response = client
            .post("http://localhost:11434/api/embeddings")
            .json(&json!({
                "model": "nomic-embed-text",
                "prompt": chunk.content
            }))
            .send()
            .await?;
        thread::sleep(Duration::from_secs(2));

        // TODO: Extract vector of embeddings from the response
        // Current response is like this:
        // "{\"embedding\":[1.0226936340332031,0.3684771656990051]}
        let body = response.text().await?;
    }

    Ok(())
}
