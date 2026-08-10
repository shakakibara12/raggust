use std::path::PathBuf;
use std::{fs, io};

struct corpus_details {
    filename: String,
    file_type: String,
    path: PathBuf,
    metadata: Metadata,
}

fn read_from_dir(dir: PathBuf) -> io::Result<()> {
    let mut entries = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
}
