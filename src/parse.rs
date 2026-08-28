use epub::doc::EpubDoc;
use std::fs::File;
use std::io::BufReader;
use tl::parse;

// Create a struct and store novel information to make it more generic?
#[derive(Debug)]
pub struct Novel {
    title: String,
    author: String,
}

impl Novel {
    pub fn open(novel: &EpubDoc<BufReader<File>>) -> Self {
        let (title, author) = Self::extract_information(novel);
        Novel { title, author }
    }

    fn extract_information(doc: &EpubDoc<BufReader<File>>) -> (String, String) {
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

// struct chapter {
//     chapter_title: String,
//     chapter_content: String,
// }

pub fn extract_from_epub(mut doc: EpubDoc<BufReader<File>>) -> Vec<String> {
    // Loop over every chapter and send them to `extract_text()`

    let mut novel_content = Vec::new();

    while let Some((content, _)) = doc.get_current_str() {
        let chapter_text = extract_text(content);
        novel_content.push(chapter_text);
        // doc.go_next() returns bool.
        if !doc.go_next() {
            break;
        }
    }

    novel_content.into_iter().flatten().collect()
}

fn extract_text(raw_html: String) -> Vec<String> {
    let dom = parse(raw_html.as_ref(), tl::ParserOptions::default()).unwrap();

    // Get all the tags we actually want.
    let tags = ["title", "p.co", "p.text-standard-tx"];

    let mut extracted: Vec<_> = Vec::new();

    // That's a lotta shit to just get the raw text outta the html.
    for tag in tags {
        let title = dom.query_selector(tag).unwrap();
        let node = title.map(|node| node.get(dom.parser()).unwrap());
        for raw_text in node {
            // We do &* to get the `str` value from the `Cow<'_, str>`
            let inner_text = &*raw_text.inner_text(dom.parser());
            extracted.push(inner_text.parse::<String>().unwrap());
        }
    }

    extracted
}
