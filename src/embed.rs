// This file contains the API for creating embedding
// We are using ollama with `nomic-embed-text` embedding model for now.
// Later we can read the embedding model from a env (from a file or actual env)

use reqwest::Client;
use serde::Deserialize;
use std::process;

const EMBED_MODEL_NAME: &str = "nomic-embed-text";

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
pub async fn create_embedding(content: &str) -> Option<Vec<f32>> {
    let client = Client::new();
    let response = client
        .post("http://localhost:11434/api/embeddings")
        .json(&serde_json::json!({
            "model": EMBED_MODEL_NAME,
            "prompt": content
        }))
        .send()
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to request ollama with error: {e}, is ollama installed and running (ollama serve)?");
                process::exit(1);
        }).error_for_status();

    match response {
        Ok(res) => {
            // This deserializes the json output to EmbeddingOutput.
            // now by default, we get embeds as f64 but libsql only supports f32
            // serde automatically handles the conversion to f64 -> f32 as well. Neat.
            let body: EmbeddingOutput = res
                .json()
                .await
                .expect("Failed to deserialize embedding json response");
            Some(body.embedding)
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_out_embedding_api() {
        let data = "こんにちわ、世界";
        let result = create_embedding(data).await;

        assert!(!result.is_none());
    }
}
