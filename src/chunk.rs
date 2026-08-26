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

    // PANIC: If text is empty.
    assert!(!text.is_empty(), "Well the text was empty, bud.");

    let length = chars.len();
    // suppose the overlap we provided is bigger than the chunk size?
    let overlap = (chunk_size / 2).min(overlap);

    let step = chunk_size - overlap;

    if length < chunk_size {
        // TODO: Add a informational text, for the user, that there were no more than one chunk.
        let trimmed = text.trim();
        return vec![Chunk {
            index: 0,
            content: trimmed.to_owned(),
        }];
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
    let last_chunk = chunks.len() * step;
    if last_chunk < chunks.len() {
        let last_content = text.get(last_chunk..).unwrap();
        if let Some(data) = chunks.last_mut() {
            data.content.push_str(last_content.trim());
        }
    }

    chunks
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn short_text_produces_single_chunk() {
        let chunks = chunk_text("hello world", 500, 50);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].content, "hello world");
        assert_eq!(chunks[0].index, 0);
    }

    #[test]
    fn text_is_split_with_overlap() {
        let text = "a".repeat(1100);
        let chunks = chunk_text(&text, 500, 50);
        dbg!(&chunks);
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].content.len(), 500);
        assert_eq!(chunks[1].content.len(), 500);
    }

    #[test]
    fn chunks_have_sequential_indices() {
        let text = "x".repeat(2000);
        let chunks = chunk_text(&text, 500, 50);
        for (i, chunk) in chunks.iter().enumerate() {
            assert_eq!(chunk.index, i);
        }
    }

    #[test]
    fn overlap_clamped_to_half_chunk_size() {
        let text = "a".repeat(1000);
        // overlap 999 > chunk_size/2 (250), gets clamped to 250
        let chunks = chunk_text(&text, 500, 999);
        let expected = chunk_text(&text, 500, 250);
        assert_eq!(chunks.len(), expected.len());
        for (a, b) in chunks.iter().zip(expected.iter()) {
            assert_eq!(a.content, b.content);
        }
    }

    // #[test]
    // fn exactly_100_char() {
    //     let text: &str = "Zenvora qeltrins murvek plandorix \
    //         vexta grunelvash zorp keldrima farnoku \
    //         trelix quambelor draven";
    //
    //     // let length = text.len();
    //     let chunk_size = 30;
    //     let overlap = 20;
    //     let chunks: Vec<Chunk> = chunk_text(text, chunk_size, overlap);
    //
    //     dbg!(chunks);
    // }
}
