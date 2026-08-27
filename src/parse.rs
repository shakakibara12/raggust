use epub::doc::EpubDoc;
use std::path::PathBuf;

// Create a struct and store novel information to make it more generic?
#[derive(Debug)]
pub struct Novel {
    title: String,
    author: String,
}

impl Novel {
    pub fn open(novel: &str) -> Self {
        let novel = PathBuf::from(novel);
        let (title, author) = Self::extract_information(novel);
        Novel { title, author }
    }

    fn extract_information(epub: PathBuf) -> (String, String) {
        let doc = EpubDoc::new(epub).unwrap();
        let title = &doc
            .metadata
            .iter()
            .find(|d| d.property == "title")
            .unwrap()
            .value;
        let author = &doc
            .metadata
            .iter()
            .find(|d| d.property == "creator")
            .unwrap()
            .value;
        (title.to_owned(), author.to_owned())
    }
}

struct chapter {
    chapter_title: String,
    chapter_content: String,
}

pub fn extract_from_epub() {
    // Add better error message.
    let mut doc = EpubDoc::new("corpus/The_Silent_Patient.epub").unwrap();

    // Loop over every chapter and send them to `extract_text()`
    doc.set_current_chapter(50);
    if let Some(chapter_content) = doc.get_current_str() {
        dbg!(chapter_content);
    };
}

fn extract_text() -> Option<String> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn extract_data() {
    // }
}
