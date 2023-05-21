use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

type TermFreq = HashMap<String, f32>;
pub type MainH = HashMap<PathBuf, TermFreq>;

#[derive(Debug, Clone)]
pub struct Document {
    pub data: MainH,
    pub path: PathBuf,
    pub last_modified: SystemTime,
}

#[derive(Debug, Clone)]
pub struct Idf {
    pub path: PathBuf,
    pub tf: f32,
}

pub fn tokenize(content: Vec<String>) -> TermFreq {
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

impl PartialEq for Idf {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

pub fn my_split(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_word = String::with_capacity(input.len());

    for c in input.to_lowercase().chars() {
        match c {
            '\'' => {
                result.push(current_word.clone());
                current_word.clear();
            }
            ',' => {
                result.push(current_word.clone());
                current_word.clear();
            }
            ' ' => {
                result.push(current_word.clone());
                current_word.clear();
            }
            '\n' => {
                result.push(current_word.clone());
                current_word.clear();
            }
            _ => current_word.push(c),
        }
    }

    result
}