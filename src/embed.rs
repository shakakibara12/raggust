// This file contains the API for creating embedding

// We are using ollama with `nomic-embed-text` embedding model for now.
// Sample curl request for interacting with the ollama API:
// curl http://localhost:11434/api/embeddings -d
// '{
// "model": "nomic-embed-text",
// "prompt": "The sky is blue because of Rayleigh scattering"
// }'

// Later we can read the embedding model from a env (from a file or actual env)

// use crate::chunk;

// pub struct Embedding {
//     index: usize,
//     vectors: Vec<i32>,
// }

// TODO: Async.
// TODO: Change parameter
// pub async fn create_embedding(chunk: Vec<chunk::Chunk>) -> Result<Vec<Embedding>> {
//     unimplemented!();
// }

use reqwest::Client;
use serde_json::json;

pub async fn create_embedding() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client
        .post("http://localhost:11434/api/embeddings")
        .json(&json!({
            "model": "nomic-embed-text",
            "prompt": "The sky is blue because of Rayleigh scattering"
        }))
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    println!("Status: {status}");
    println!("{body}");

    Ok(())
}
