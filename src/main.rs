mod chunk;
mod db;
mod embed;
mod parse;
mod rag;

// Imports
use epub::doc::EpubDoc;
use std::io::{self, BufRead, Write};

type E = Box<dyn std::error::Error>;

const SOURCE: &str = "corpus/The_Silent_Patient.epub";
const CHUNK_SIZE: usize = 3000;
const OVERLAP: usize = 1000;

#[tokio::main]
async fn main() -> Result<(), E> {
    let novel = EpubDoc::new(SOURCE)?;
    let the_silent_patient = parse::Novel::open(&novel);

    println!("Novel Details:\n{}", the_silent_patient);

    let mut novel_content = String::new();

    // concatenate every content extracted to a single String.
    for content in parse::extract_from_epub(novel) {
        novel_content.push_str(&content);
    }

    let chunks = chunk::chunk_text(&novel_content, CHUNK_SIZE, OVERLAP);
    let chunk_len = chunks.len();
    println!("Total number of chunks: {chunk_len}");

    // Database intialization:
    let connection = db::init_db().await.expect("Error initializing database");
    for (i, chunk) in chunks.into_iter().enumerate() {
        let chunk_content = chunk.content;
        let index = chunk.index;
        println!("Remaining chunks left: {}", chunk_len - i);

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
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    println!(
        "{}\n\nWelcome to Raggust, ask anything about: 'The Silent Patient'
            Type exit or quit to leave.\n{}",
        "-".repeat(60),
        "-".repeat(60)
    );
    loop {
        print!("> ");
        stdout.flush()?;
        let mut user_input = String::new();
        let mut handle = stdin.lock();
        handle.read_line(&mut user_input)?;

        let question = user_input.trim();

        // Do nothing, if empty
        if question.is_empty() {
            continue;
        }

        // Clear screen, because why not?
        if question == "clear" {
            print!("\x1B[2J");
            continue;
        }

        // Exit
        if question == "exit" || question == "quit" {
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
