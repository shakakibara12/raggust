mod chunk;
mod embed;
mod parse;
use epub::doc::EpubDoc;

type E = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), E> {
    let novel = EpubDoc::new("corpus/The_Silent_Patient.epub")?;
    let the_silent_patient = parse::Novel::open(&novel);

    println!("Novel Details:\n{}", the_silent_patient);

    // We can do two ways here:
    // First: we take the Vec<String> we got from extract_from_epub and send it directly to the
    // chunk_text function and change it's parameter accordingly.
    // -> In this, there will be occurrences where the date inside the vector are just single lines,
    // which will increase the number of embeddings we get per item.
    // Second: We merge all the String inside the vector and return the result as a huge String.
    // -> If we go this path, we leave the entire chunking to the function, pretty straightforward
    // in my opinion. This seems easy to understand and implement but the resultant string is going
    // to be really huge not sure how "performant" that will be. At least we are sending a string reference
    // to the function.
    // Also If we squash everything at once, we also losing the perfect chapter wise splits we got
    // from the epub extraction, that the first method will easily take care. In this case, The
    // first method could yield a better efficiency for embeddings.
    //
    // Let's go for the 2nd method, because it's easy to do and let's see after we have everything
    // implemented we can go for the second method.
    //
    // That brings us to how do we squash every string into one. I know as_ref() can give us
    // reference
    // &str -> is made up of 2 components, a pointer and a length
    let mut novel_content = String::new();

    // TODO: implement with iterators.
    for content in parse::extract_from_epub(novel) {
        novel_content += &content;
    }
    // let novel_contents = parse::extract_from_epub(novel)
    //     .into_iter()
    //     .reduce(|acc, x| acc + x);

    let _chunks = chunk::chunk_text(novel_content.as_ref(), 200, 50);
    // TODO: Then embed
    // let embeddings = embed::create_embedding(chunks);
    // dbg!(embeddings);
    embed::create_embedding().await?;
    Ok(())
}
