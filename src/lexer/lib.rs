use super::lexing::Document;
use poppler::PopplerDocument;
use std::{fs, io};
use std::{fs::read_dir, path::PathBuf};

/// Searches for files with a specific filetype in a directory.
///
/// This function takes a directory path represented as a `String` and a filetype as a `&str`,
/// and returns a `Result` containing a `Vec<PathBuf>` with the paths of the matching files found in the directory.
///
/// # Arguments
///
/// * `path` - A `String` representing the directory path to search in.
/// * `filetype` - A `&str` representing the desired filetype to search for.
///
/// # Returns
///
/// A `Result` that contains a `Vec<PathBuf>` with the paths of the matching files found in the directory,
/// or an `io::Error` if there was an issue reading the directory or iterating over its entries.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// let path = String::from("/path/to/directory");
/// let filetype = "txt";
///
/// match search_filetype(&path, filetype) {
///     Ok(files) => {
///         for file in files {
///             println!("{}", file.display());
///         }
///     },
///     Err(error) => {
///         eprintln!("Error: {}", error);
///     }
/// }
/// ```
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

/// This function takes a `PathBuf` argument representing the path to a PDF document and returns a `String`
/// containing the concatenated text content of all pages in the PDF document.
///
/// # Arguments
///
/// * `doc` - A `PathBuf` representing the path to the PDF document.
///
/// # Returns
///
/// A `String` containing the concatenated text content of all pages in the PDF document.
///
/// # Panics
///
/// This function will panic if it encounters any errors while reading the document.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// let doc = PathBuf::from("path/to/my/document.pdf");
/// let content = read_from_pdf(&doc);
/// println!("{}", content);
/// ```
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

/// Serializes a vector of documents to JSON and saves it to a file.
///
/// This function takes a reference to a vector of documents (`&Vec<Document>`) and a file path as a `String`.
/// It serializes the vector of documents into a prettified JSON string and saves it to the specified file.
///
/// # Arguments
///
/// * `data` - A reference to a vector of documents (`&Vec<Document>`).
/// * `path` - A `String` representing the file path to save the serialized JSON data.
///
/// # Returns
///
/// An `io::Result` indicating the success or failure of the serialization and saving operation.
///
/// # Errors
///
/// This function can return an `io::Error` if there is an issue writing to the file or a `serde_json::Error`
/// if there is an issue serializing the data to JSON.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// let data: &Vec<Document> = &vec![/* ... */];
/// let path = String::from("path/to/save.json");
///
/// match serialize_and_save(data, path) {
///     Ok(()) => {
///         println!("Data serialized and saved successfully.");
///     },
///     Err(error) => {
///         eprintln!("Error: {}", error);
///     }
/// }
/// ```
pub fn serialize_and_save(data: &Vec<Document>, path: String) -> io::Result<()> {
    let serialized_data = serde_json::to_string_pretty(&data)?;
    fs::write(&path, serialized_data)
}
