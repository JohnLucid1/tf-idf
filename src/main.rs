mod lexer;
use std::fs::read_to_string;

use std::{
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

use lexer::{
    lexing::{index_data, my_split, Document, Idf, MainH},
    lib::{read_from_pdf, search_filetype, serialize_and_save},
};

const WEEK_IN_SECONDS: u64 = 604800;

fn run(mut directory: String, all_pdf_paths: Vec<PathBuf>, query: String) {
    let json_name = Path::new(&directory).join(".data.json");
    if json_name.exists() {
        let filedata = read_to_string(json_name).unwrap();

        let data: Vec<Document> =
            serde_json::from_str(&filedata).expect("Couldn't deserialize data");

        let date = data.get(0).unwrap().last_modified.elapsed().unwrap();
        if date > Duration::from_secs(WEEK_IN_SECONDS) {
            // If date saved is larger than a week we re-indexing the whole thing and then searching
            // Reindex data and search
            println!("Reindexing data");
            let saved_data = tokenize_data(all_pdf_paths);
            serialize_and_save(&saved_data, directory).expect("Couldn't searilize data");
            search_query(saved_data, query);
        } else {
            // Just search query
            println!("Searching for {}", query);
            search_query(data, query)
        }
    } else {
        // Create new file, reindex data, and search query
        println!("Reindexing data");
        directory.push_str(&format!("{}", ".data.json"));
        let data = tokenize_data(all_pdf_paths);
        serialize_and_save(&data, directory).expect("Couldn't write to file");
        search_query(data, query)
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

    run(directory, all_pdfs_paths, query);
}

fn tokenize_data(paths: Vec<PathBuf>) -> Vec<Document> {
    let mut main_hs = MainH::default();
    let mut documents: Vec<Document> = Vec::new();

    for path in paths.into_iter() {
        let content = read_from_pdf(&path);
        let data = my_split(&content);
        let tks = index_data(data);
        main_hs.insert(path.clone(), tks);

        documents.push(Document {
            data: main_hs.clone(),
            path,
            last_modified: SystemTime::now(),
        });
        main_hs.clear();
    }
    documents
}

// how the fuck does this function work?????
// TODO Optimize the shit out of this
fn search_query(docs: Vec<Document>, query: String) {
    let mut tfs: Vec<Idf> = Vec::with_capacity(docs.len());
    let mut idf_buff: Vec<Idf> = Vec::with_capacity(docs.len());

    for doc in docs {
        let path_doc = doc.path;
        for val in doc.data.values() {
            let tf = val.get(&query).unwrap_or(&0.0).to_owned();
            let path = path_doc.clone();
            let temp = Idf { path, tf };
            tfs.push(temp)
        }
    }

    let tfs_len = tfs.len();

    for i in 0..tfs_len {
        for j in 0..tfs.len() {
            let mut divided = tfs[i].tf / tfs[j].tf;
            if divided.is_nan() || divided.is_infinite() {
                divided = 0.0
            }

            let idf = Idf {
                path: tfs[i].path.clone(),
                tf: divided,
            };

            if idf_buff.contains(&idf) {
                continue;
            }

            idf_buff.push(idf);
        }
    }

    idf_buff.sort_by(|a, b| {
        a.tf.partial_cmp(&b.tf)
            .expect("Coun't compare args")
            .reverse()
    });

    for (idx, elem) in idf_buff.into_iter().enumerate() {
        println!("{}: {:?}, {}", idx + 1, elem.path, elem.tf)
    }
}

/*
TODO make it work with multiple words
TODO maybe try to multithread the process if indexing docs
*/
