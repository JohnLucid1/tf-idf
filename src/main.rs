mod lexer;
use lexer::{
    lexing::{index_data, split_into_words, DocFreq, DocFreqExt, Document, Idf},
    lib::{read_from_pdf, search_filetype, serialize_and_save},
};
use serde_json::Result;
use std::fs::read_to_string;
use std::{
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};
const WEEK_IN_SECONDS: u64 = 604800;

/// Runs the search process on the given directory and search query.
///
/// This function takes a directory path, a vector of PDF file paths, and a search query string as input. It performs the search process, which includes indexing the data, checking if the indexed data needs to be updated, and performing the search query. The search results are printed to the console.
///
/// # Arguments
///
/// * `directory` - A mutable string representing the directory path.
/// * `all_pdf_paths` - A vector of `PathBuf` representing the paths of all PDF files.
/// * `query` - A string representing the search query.
///
/// # Errors
///
/// This function can return an error if there are issues with reading or writing the data files, or if there are errors during JSON deserialization.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// let directory = "data".to_string();
/// let pdf_paths = vec![
///     PathBuf::from("file1.pdf"),
///     PathBuf::from("file2.pdf"),
///     PathBuf::from("file3.pdf"),
/// ];
///
/// run(directory, pdf_paths, "example".to_string()).expect("Search process failed");
/// ```
///
/// The function can be used with a valid directory path, a vector of PDF file paths, and a search query string to perform the search process on the data and print the search results.
fn run(mut directory: String, all_pdf_paths: Vec<PathBuf>, query: String) -> Result<()> {
    let json_name = Path::new(&directory).join(".data.json");
    if json_name.exists() {
        let filedata = read_to_string(json_name).unwrap();
        let data: Vec<Document> = serde_json::from_str(&filedata)?;
        let date = data.get(0).unwrap().last_modified.elapsed().unwrap();

        if date > Duration::from_secs(WEEK_IN_SECONDS) {
            // If date saved is larger than a week we re-indexing the whole thing and then searching
            // Reindex data and search
            println!("Reindexing data");
            let saved_data = tokenize_data(all_pdf_paths);
            serialize_and_save(&saved_data, directory).expect("Couldn't serialize");
            search_query(saved_data, query);
            Ok(())
        } else {
            // Just search query
            println!("Searching for {}", query);
            search_query(data, query);
            Ok(())
        }
    } else {
        // Create new file, reindex data, and search query
        println!("Reindexing data");
        directory.push_str(&format!("{}", ".data.json"));
        let data = tokenize_data(all_pdf_paths);
        serialize_and_save(&data, directory).expect("Couldn't write to file");
        search_query(data, query);
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 2 {
        panic!("ERROR: Enter filetype, directory, word")
    }

    let filetype = args.get(1).expect("ERROR: Enter a filetype").to_string();
    let directory = args.get(2).expect("ERROR: Enter a directory").to_string();
    let query = args.get(3).expect("ERROR: Enter a query").to_string();
    let all_pdfs_paths = search_filetype(&directory, &filetype).expect("Couln't find pdfs");

    run(directory, all_pdfs_paths, query).expect("Couldn't run main");
}

/// Tokenizes the content of PDF files and creates a vector of Document structs.
///
/// This function takes a vector of file paths (`Vec<PathBuf>`) representing PDF files. It reads
/// the content of each file using the `read_from_pdf` function, tokenizes the content into
/// individual words using the `split_into_words` function, and creates a Document struct for each
/// file. The Document structs contain the tokenized data, file path, and the current system time
/// as the last modified timestamp.
///
/// # Arguments
///
/// * `paths` - A vector of file paths (`Vec<PathBuf>`) representing the PDF files to tokenize.
///
/// # Returns
///
/// A vector of Document structs representing the tokenized data from the PDF files.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use std::time::SystemTime;
///
/// let paths = vec![
///     PathBuf::from("file1.pdf"),
///     PathBuf::from("file2.pdf"),
///     PathBuf::from("file3.pdf"),
/// ];
///
/// let documents = tokenize_data(paths);
///
/// assert_eq!(documents.len(), 3);
/// // Check the contents of the first document
/// assert_eq!(documents[0].path, PathBuf::from("file1.pdf"));
/// assert!(documents[0].last_modified.elapsed().is_ok());
/// ```
fn tokenize_data(paths: Vec<PathBuf>) -> Vec<Document> {
    let mut documents: Vec<Document> = Vec::new();

    for path in paths {
        let content = read_from_pdf(&path);
        let data = split_into_words(&content);
        let tsk = index_data(data);

        let document = Document {
            data: DocFreq::single(path.clone(), tsk),
            path,
            last_modified: SystemTime::now(),
        };
        documents.push(document);
    }

    documents
}

/// Searches for the given query in the provided documents and prints the search results.
///
/// This function takes a vector of `Document` structs and a query string as input. It performs a
/// search by calculating the inverse document frequency (IDF) for each document and query term
/// combination. The search results are then printed to the console.
///
/// # Arguments
///
/// * `docs` - A vector of `Document` structs representing the documents to search.
/// * `query` - A string representing the query to search for.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// let doc1 = Document {
///     data: DocFreq::default(),
///     path: PathBuf::from("file1.pdf"),
///     last_modified: SystemTime::now(),
/// };
/// let doc2 = Document {
///     data: DocFreq::default(),
///     path: PathBuf::from("file2.pdf"),
///     last_modified: SystemTime::now(),
/// };
///
/// let docs = vec![doc1, doc2];
///
/// search_query(docs, "example".to_string());
/// ```
///
/// The function can be used with any valid vector of `Document` structs and a query string to
/// search for the query in the documents and print the search results.
pub fn search_query(docs: Vec<Document>, query: String) {
    let mut idf_buff: Vec<Idf> = Vec::new();

    for doc in &docs {
        let tf = doc
            .data
            .get(&doc.path)
            .and_then(|term_freq| term_freq.get(&query))
            .cloned()
            .unwrap_or(0.0);

        let idf = Idf {
            path: doc.path.clone(),
            tf,
        };

        if !idf_buff.contains(&idf) {
            idf_buff.push(idf);
        }
    }

    idf_buff.sort_by(|a, b| {
        b.tf.partial_cmp(&a.tf)
            .expect("Unable to compare arguments")
    });

    for (idx, elem) in idf_buff.into_iter().enumerate() {
        println!("{}: {:?}, {}", idx + 1, elem.path, elem.tf);
    }
}
