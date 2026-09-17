use libsql::{Builder, Connection, errors::Error, params};

pub async fn init_db() -> Result<Connection, Error> {
    // Storing database in the memory for now.
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS documents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            source_path TEXT NOT NULL,
            mtime INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS chunks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            document_id INTEGER NOT NULL REFERENCES documents(id),
            chunk_index INTEGER NOT NULL,
            content TEXT NOT NULL,
            embedding F32_BLOB(768)
        );

        CREATE INDEX IF NOT EXISTS idx_chunks_embedding
            ON chunks(libsql_vector_idx(embedding, 'metric=cosine'));",
    )
    .await?;

    Ok(conn)
}

pub async fn insert_document(
    conn: &Connection,
    title: &str,
    source_path: &str,
    mtime: i64,
) -> Result<i64, Error> {
    conn.execute(
        "INSERT INTO documents (title, mtime) VALUES (?1, ?2)",
        params![title, source_path, mtime],
    )
    .await?;

    let mut rows = conn.query("SELECT last_insert_rowid()", params![]).await?;
    match rows.next().await? {
        Some(row) => {
            let id = row.get::<i64>(0)?;
            Ok(id)
        }
        // TODO: To panic or to no panic?
        None => Err(Error::QueryReturnedNoRows),
    }
}

pub async fn insert_chunk(
    conn: &Connection,
    document_id: i64,
    chunk_index: usize,
    content: &str,
    embedding: &[f32],
) -> Result<(), Error> {
    // Serialize embedding into json for libsql.
    let embedding_json = serde_json::to_string(embedding).unwrap();
    conn.execute(
        "INSERT INTO chunks (document_id, chunk_index, content, embedding) VALUES (?1, ?2, ?3, vector32(?4))",
        params![document_id, i64::try_from(chunk_index).unwrap(), content, embedding_json],
    ).await?;

    Ok(())
}

pub async fn find_document_by_id(
    conn: &Connection,
    source_path: &str,
) -> Result<Option<(i64, i64)>, Error> {
    let mut rows = conn
        .query(
            "SELECT id, mtime FROM documents WHERE source_path = ?1",
            params![source_path],
        )
        .await?;

    match rows.next().await? {
        Some(row) => {
            let id = row.get::<i64>(0)?;
            let mtime = row.get::<i64>(1)?;
            Ok(Some((id, mtime)))
        }
        None => Ok(None),
    }
}
