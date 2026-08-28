mod chunk;
mod parse;
use epub::doc::EpubDoc;

type E = Box<dyn std::error::Error>;

fn main() -> Result<(), E> {
    let novel = EpubDoc::new("corpus/The_Silent_Patient.epub")?;
    let the_silent_patient = parse::Novel::open(&novel);

    println!("Novel Details:\n{}", the_silent_patient);

    let novel_contents = parse::extract_from_epub(novel);

    // TODO: Chunk here
    // TODO: Then embed
    Ok(())
}
