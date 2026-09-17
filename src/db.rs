use libsql::{Builder, Connection, errors::Error, params};

pub async fn init_db() -> Result<Connection, Error> {
    // Storing database in the memory for now.
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS chunks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            chunk_index INTEGER NOT NULL,
            content TEXT NOT NULL,
            embedding F32_BLOB(768)
        );

        CREATE INDEX IF NOT EXISTS idx_chunks_embedding
            ON chunks(libsql_vector_idx(embedding, 'metric=cosine'));
        ",
    )
    .await?;

    Ok(conn)
}

pub async fn insert_chunk(
    conn: &Connection,
    chunk_index: usize,
    content: &str,
    embedding: &[f32],
) -> Result<(), Error> {
    // Serialize embedding into json for libsql.
    let embedding_json = serde_json::to_string(embedding)
        .unwrap_or_else(|error| panic!("Unable to serialize embedding with error: {error:?}"));
    conn.execute(
        "INSERT INTO chunks (chunk_index, content, embedding) VALUES (?1, ?2, ?3, vector32(?4))",
        params![i64::try_from(chunk_index).unwrap(), content, embedding_json],
    )
    .await?;

    Ok(())
}

// Implementation of Vector search

pub struct SearchHit {
    pub score: f64,
    pub content: String,
    pub doc_path: String,
}

// query_embedding: Take the user's input and convert to embeddings to pass it in here.
// In embed.rs
// create_embedding(content: &str) -> Result<Vec<f32>, Box<dyn error::Error>>
// top_k is how many top results we want. E.g., 5 -> would provide 5 top searches.
pub async fn vector_search(
    conn: &Connection,
    query_embedding: &[f32],
    top_k: usize,
) -> Result<Vec<SearchHit>, Error> {
    let embedding_json = serde_json::to_string(query_embedding)
        .unwrap_or_else(|error| panic!("Unable to serialize user embedding with error: {error:?}"));
    let sql = format!(
        "
        SELECT c.content \
        FROM vector_top_k('idx_chunks_embedding', vector32(?1), {top_k}) AS v \
        JOIN chunks AS c ON c.rowid = v.id
        "
    );
    let mut rows = conn.query(&sql, params![embedding_json]).await?;

    let mut results = Vec::new();
    let mut rank = 0_u32;
    let total = u32::try_from(top_k).unwrap_or(u32::MAX);
    while let Some(row) = rows.next().await? {
        let content = row.get::<String>(0)?;
        let doc_path = row.get::<String>(1)?;
        // We compute score manually because libsql doesn't return a score, it only returns the
        // actual rows containing the top search and then the subsequent rows return the next top
        // search. By doing this, we get scores like 0.8, 0.6, 0.4 ... This is fine for the scope
        // of this project. Consider using something like `usearch` for something more complete,
        // which provides us with actual distances and supports custom metrics.
        // NOTE: As we get down to scores like 0.4, 0.2. The actual content may be questionable.
        let score = 1.0 - f64::from(rank) / f64::from(total);

        results.push(SearchHit {
            score,
            content,
            doc_path,
        });
        rank += 1;
    }

    Ok(results)
}
