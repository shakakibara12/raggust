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

pub fn extract_from_epub(mut doc: EpubDoc<BufReader<File>>) {
    // Loop over every chapter and send them to `extract_text()`
    doc.set_current_chapter(50);
    if let Some((content, _)) = doc.get_current_str() {
        extract_text(content);
    };
}

fn extract_text(raw_html: String) -> Option<String> {
    let dom = parse(raw_html.as_ref(), tl::ParserOptions::default()).unwrap();

    // That's a lotta shit to just get the raw text outta the html.
    let title = dom.query_selector("p.text-standard-tx").unwrap();
    let node = title.map(|node| node.get(dom.parser()).unwrap());
    for raw_text in node {
        let inner_text = raw_text.inner_text(dom.parser());
        println!("{:?}", &*inner_text);
    }
    // Some(String::from(&*inner_text))
    Some("return shit".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn extract_data() {
    // }
}
