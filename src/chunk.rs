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
        // TODO: return the first chunk with the index 0
        todo!();
    }

    // TODO: Create a test for when size is 0.
    let chunks: Vec<_> =
        chars
            .windows(chunk_size)
            .step_by(step)
            .enumerate()
            .map(|(index, content)| Chunk {
                index,
                content: content.trim().collect(),
            });

    let mut output: Vec<Chunk> = Vec::new();
    // for (index, content) in chunks.enumerate().collect() {
    //     output.push(Chunk { index, content });
    // }
    output
    // Suppose text len = 100
    // chunk size = 50
    // overlap = 25
    // What if the text size is smaller than the chunk size?
    //
}

#[cfg(test)]
mod tests {
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
}
