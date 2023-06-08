use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

/// Represents a mapping of terms to their frequencies.
pub type TermFreq = HashMap<String, f32>;

/// Represents a mapping of document paths to their term frequencies.
pub type DocFreq = HashMap<PathBuf, TermFreq>;

pub trait DocFreqExt {
    fn single(path: PathBuf, term_freq: TermFreq) -> DocFreq;
}

impl DocFreqExt for DocFreq {
    fn single(path: PathBuf, term_freq: TermFreq) -> DocFreq {
        let mut doc_freq = DocFreq::new();
        doc_freq.insert(path, term_freq);
        doc_freq
    }
}

/// Represents a document.
///
/// This struct holds information about a document, including its term frequencies,
/// path, and last modified time.
///
/// # Fields
///
/// * `data` - A `DocFreq` representing the mapping of document paths to their term frequencies.
/// * `path` - A `PathBuf` representing the path of the document.
/// * `last_modified` - A `SystemTime` representing the last modified time of the document.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use std::time::SystemTime;
///
/// let document = Document {
///     data: DocFreq::new(),
///     path: PathBuf::from("path/to/document.txt"),
///     last_modified: SystemTime::now(),
/// };
///
/// println!("{:?}", document);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub data: DocFreq,
    pub path: PathBuf,
    pub last_modified: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Idf {
    pub path: PathBuf,
    pub tf: f32,
}

impl PartialEq for Idf {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

/// Indexes data by calculating the term frequencies.
///
/// This function takes a vector of strings (`Vec<String>`) representing the content to be indexed.
/// It calculates the term frequencies for each term in the content and returns a `TermFreq` mapping
/// the terms to their corresponding frequencies.
///
/// # Arguments
///
/// * `content` - A vector of strings (`Vec<String>`) representing the content to be indexed.
///
/// # Returns
///
/// A `TermFreq` mapping the terms to their corresponding frequencies.
///
/// # Examples
///
/// ```
/// let content = vec![
///     String::from("apple"),
///     String::from("banana"),
///     String::from("apple"),
///     String::from("orange"),
///     String::from("apple"),
/// ];
///
/// let term_freq = index_data(content);
///
/// assert_eq!(term_freq["apple"], 0.6);
/// assert_eq!(term_freq["banana"], 0.2);
/// assert_eq!(term_freq["orange"], 0.2);
/// ```
pub fn index_data(content: Vec<String>) -> TermFreq {
    let mut data: TermFreq = HashMap::new();
    let full_length = content.len() as f32;

    for term in content {
        if data.contains_key(&term) {
            *data.get_mut(&term).unwrap() += 1.0 / full_length;
        } else {
            data.insert(term, 1.0);
        }
    }

    data
}

/// Splits the input string into individual words based on specified delimiters.
///
/// This function takes an input string and splits it into individual words based on the specified
/// delimiters. The delimiters used for splitting include: single quote ('), period (.), closing
/// parenthesis (')', opening parenthesis ('('), backtick ('`'), comma (,), double quote ("), space (' '),
/// and newline ('\n').
///
/// # Arguments
///
/// * `input` - The input string to be split into words.
///
/// # Returns
///
/// A vector of strings containing the individual words extracted from the input string.
///
/// # Examples
///
/// ```
/// let input = "Hello, World! How are you today?";
///
/// let result = split_into_words(input);
///
/// assert_eq!(result, vec!["hello", "world", "how", "are", "you", "today"]);
/// ```
pub fn split_into_words(input: &str) -> Vec<String> {
    let delimiters = ['\'', '.', ')', '(', '`', ',', '"', ' ', '\n'];

    let mut result = input
        .to_lowercase()
        .split(|c| delimiters.contains(&c))
        .filter(|word| !word.is_empty())
        .map(String::from)
        .collect::<Vec<String>>();

    result.shrink_to_fit();
    result
}
