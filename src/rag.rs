use crate::db;
use crate::embed;
use libsql::Connection;
use libsql::Error;
use reqwest::Client;
use std::error;
use std::fmt::Write;

// Number of top results to return.
const TOP_K: usize = 5;

const LLM_MODEL_NAME: &str = "deepseek-r1";

pub struct RagResponse {
    pub response: String,
}
pub async fn query(conn: &Connection, question: &str) -> Result<RagResponse, Error> {
    // 1. Embed the question.
    let query_embedding = embed::create_embedding(question).await.unwrap();

    // 2. Get the top results.
    let hits = db::vector_search(conn, &query_embedding, TOP_K).await?;

    // 3. Build context (Get content from the top results for our LLM)
    let mut context = String::new();
    for (i, hit) in hits.into_iter().enumerate() {
        let _ = write!(context, " [{}] {} ", i + 1, hit.content);
    }

    let preamble = "You are Duck, the most profound duck there ever is. \
             You are precise like a needle and knowledgeable like a saint. \
             Use the provided context to answer accurately. If the \
             context doesn't contain enough information, You have to answer \
             honestly, never under no circumstances make up answers."
        .to_string();

    let query = format!("preamble: {preamble}\nContext: {context}\n Question: {question}");

    // 4. Return the answer
    let response = fetch_llm_output(&query).await.unwrap();
    Ok(RagResponse { response })
}

pub async fn fetch_llm_output(prompt: &str) -> Result<String, Box<dyn error::Error>> {
    let client = Client::new();
    let response = client
        .post("http://localhost:11434/api/chat")
        .json(&serde_json::json!({
            "model": LLM_MODEL_NAME,
            "messages": [
                {
                  "role": "user",
                  "content": prompt
                }
            ],
            "stream": false
        }))
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = response.json().await?;
    Ok(body["message"]["content"]
        .as_str()
        .unwrap()
        .trim()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_ollama_output() {
        let question = "Why is the sky blue?";
        let output = fetch_llm_output(question).await;
        // dbg!(&output);
        assert!(!output.is_err());
    }
}
