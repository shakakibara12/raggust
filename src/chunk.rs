/// This will be the core API for chunking text

#[derive(Debug)]
pub struct Chunk {
    pub index: usize,
    pub content: String,
}

/// Using fixed-length overlapping chunking
///   Chunk 1
/// ===============>
///                  Chunk 2
///           =================>
///           ^    ^             Chunk 3
///           |    |     =================>
///           |____|
///           overlap
/// Initially I thought of chunking by chapters but, then I realized how varying the chapters really
/// are in terms of the content. Therefore, I have decided on on overlapping and chunking on a fixed
/// size, this way we will have good readable chunks without losing any information stored between 2
/// chunks
/// Note: This API needs clean operable text (not a requirement, just better for efficient chunking
/// so make sure the text is sanitized enough).
#[must_use]
pub fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Vec<Chunk> {
    let chars: Vec<char> = text.chars().collect();
    // suppose the overlap we provided is bigger than the chunk size?
    let overlap = (chunk_size / 2).min(overlap);

    let step = chunk_size - overlap;

    if chars.len() < chunk_size {
        // TODO: Add a informational text, for the user, that there were no more than one chunk.
        let out: Vec<Chunk> = vec![Chunk {
            index: 0,
            content: chars.iter().collect(),
        }];
        return out;
    }

    let mut chunks: Vec<Chunk> = chars
        .windows(chunk_size)
        .step_by(step)
        .enumerate()
        .map(|(index, content)| {
            let content: String = content.iter().collect();
            let trimmed = content.trim();
            // Handle if trimmed is empty
            (!trimmed.is_empty())
                .then(|| Chunk {
                    index,
                    content: trimmed.to_string(),
                })
                .unwrap()
        })
        .collect();

    // handle the case when there is still text left after the chunking is done
    // We will push the leftover text into the last chunk instead of creating a new one.
    //
    // Note: I have no clue how this works, but it took a lot of time to figure it out.
    if !chunk_size.is_multiple_of(overlap) || !text.len().is_multiple_of(chunk_size) {
        let chunks_without_overlap = text.len() / chunk_size;
        let total_with_overlap = chunks_without_overlap * overlap;
        let the_last_bit = (text.len() + total_with_overlap) % chunk_size;
        if the_last_bit != 0 {
            let last = &text[text.len() - the_last_bit..text.len()];
            dbg!(last);
            if let Some(data) = chunks.last_mut() {
                data.content.push_str(last);
            }
        }
    }

    chunks
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_out_chars() {
        let text = "hello";
        let chars: Vec<char> = text.chars().collect();
        assert_eq!(chars, ['h', 'e', 'l', 'l', 'o']);
    }

    #[test]
    fn test_out_windows() {
        let text: Vec<char> = "omoshiroi".chars().collect();
        let windows: Vec<_> = text.windows(3).step_by(2).collect();

        println!("{:?}", windows);
    }

    // Suppose text len = 100
    // chunk size = 50
    // overlap = 25

    // #[test]
    // fn get_those_thick_chunks_ma_boy() {
    //     let text: &str = "this is a amazing and big text";
    //
    //     println!("length of text is: {}", text.len());
    //
    //     let chunks: Vec<Chunk> = chunk_text(text, 5, 2);
    // }

    #[test]
    fn exactly_100_char() {
        let text: &str = r#"Zenvora qeltrins murvekrix vexta grunelvash zorp keldrima farnoku trelix quambelor dravencost"#;

        println!("length of text is: {}", text.len());

        let chunks: Vec<Chunk> = chunk_text(text, 40, 29);

        println!("{:?}", chunks);
    }
    // TODO: Create a test for when size is 0.
}
