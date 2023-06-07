use super::lexing::Document;
use poppler::PopplerDocument;
use serde_json::Result;
use std::fs;
use std::{fs::read_dir, path::PathBuf};

/// Returns an optional vector of a specified filetype
pub fn search_filetype(path: &String, filetype: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut files_vec: Vec<PathBuf> = Vec::new();
    let files = read_dir(path)?;

    for fp in files {
        let path = fp?.path();
        if let Some(extension) = path.extension() {
            if extension == filetype {
                files_vec.push(path)
            }
        }
    }

    Ok(files_vec)
}

pub fn read_from_pdf(doc: &PathBuf) -> String {
    let pdf = PopplerDocument::new_from_file(doc, "").expect("Coulnd't read the document");
    let mut buff = String::new();
    let num_of_pgs = pdf.get_n_pages();

    for page_num in 0..num_of_pgs {
        if let Some(page) = pdf.get_page(page_num) {
            match page.get_text() {
                Some(content) => buff.push_str(content),
                None => continue,
            }
        }
    }

    buff
}

/*
 TODO serialize ifd indexing in .data.json file in a directery of search
os if search is in ./books/, .data.json location is also in books
*/
pub fn serialize_and_save(stuff: &Vec<Document>, path: String) -> Result<()> {
    let somedata = serde_json::to_string_pretty(&stuff)?;
    fs::write(path, somedata).expect("Couldn't write to file");
    Ok(())
}
