// This file contains the API for creating embedding
// We are using ollama with `nomic-embed-text` embedding model for now.
// Later we can read the embedding model from a env (from a file or actual env)

use reqwest::Client;
use serde::Deserialize;
use std::error;

const MODEL_NAME: &str = "nomic-embed-text";

// TODO: Look into implementing a custom error type that wraps both the reqwest error type and the
// serde error type.
// Also, implement Display, Error and From trait.
// Display -> Let's us use descriptive error message
// Error -> Let's us know which error was invoked (reqwest or serde)
// From -> Let's us propagate error using the `?` mark. NEAT.

#[derive(Deserialize)]
struct EmbeddingOutput {
    embedding: Vec<f32>,
}

// TODO: Reuse reqwest client across ollama functions
pub async fn create_embedding(content: &str) -> Result<Vec<f32>, Box<dyn error::Error>> {
    let client = Client::new();
    let response = client
        .post("http://localhost:11434/api/embeddings")
        .json(&serde_json::json!({
            "model": MODEL_NAME,
            "prompt": content
        }))
        .send()
        .await?
        .error_for_status()?;

    // This deserializes the json output to EmbeddingOutput.
    // now by default, we get embeds as f64 but libsql only supports f32
    // serde automatically handles the conversion to f64 -> f32 as well. Neat.
    let body: EmbeddingOutput = response.json().await?;
    Ok(body.embedding)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_out_embedding_api() {
        let data = "こんにちわ、世界";
        let result = create_embedding(data).await;

        assert!(!result.is_err());
    }
}
