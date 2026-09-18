mod chunk;
mod cli;
mod db;
mod embed;
mod parse;
mod rag;
use epub::doc::EpubDoc;
use std::io::{self, BufRead, Write};

type E = Box<dyn std::error::Error>;

const SOURCE: &str = "corpus/The_Silent_Patient.epub";

#[tokio::main]
async fn main() -> Result<(), E> {
    let novel = EpubDoc::new(SOURCE)?;
    let the_silent_patient = parse::Novel::open(&novel);

    println!("Novel Details:\n{}", the_silent_patient);

    let mut novel_content = String::new();

    // TODO: implement with iterators.
    for content in parse::extract_from_epub(novel) {
        novel_content.push_str(&content);
    }

    let chunks = chunk::chunk_text(&novel_content, 2000, 500);
    let chunk_len = chunks.len();
    println!("Total number of chunks: {chunk_len}");
    // Database intialization:
    let connection = db::init_db().await.expect("Error initializing database");
    for chunk in chunks {
        let chunk_content = chunk.content;
        let chunk_length = chunk_content.len();
        let index = chunk.index;
        println!("Embedding chunk {index} with length: {chunk_length}");
        // Embed chunk
        let embedding = embed::create_embedding(&chunk_content)
            .await
            .expect("Unable to embed chunk");
        // Insert in database
        match db::insert_chunk(&connection, index, &chunk_content, &embedding).await {
            Ok(_) => println!("Successfully inserted chunk"),
            Err(_) => eprintln!("Failed inserting chunks into database"),
        };
    }
    cli_help();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        println!("> ");
        stdout.flush()?;
        let mut user_input = String::new();
        let mut handle = stdin.lock();
        handle.read_line(&mut user_input)?;

        let question = user_input.trim();
        // Exit
        if question.is_empty() || question == "exit" || question == "quit" {
            break;
        }
        let result = rag::query(&connection, question).await;
        match result {
            Ok(res) => {
                let response = res.response;
                println!("\n{response}");
            }
            Err(err) => eprintln!("Failed to fetch llm output: {err}"),
        }
    }
    Ok(())
}

fn cli_help() -> String {
    "Welcome to raggust!\n
    You can type 'exit' or 'quit' to exit."
        .to_string()
}
