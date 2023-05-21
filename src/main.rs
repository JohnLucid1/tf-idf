mod lexer;
use lexer::{my_split, tokenize, Document, Idf, MainH};
use poppler::PopplerDocument;
use std::{fs::read_dir, path::PathBuf, time::SystemTime};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        panic!("ERROR: Enter directory")
    }

    let directory = args.get(1).expect("ERROR: Enter a directory").to_string();
    let query = args.get(2).expect("ERROR: Enter a query").to_string();

    let mut documents: Vec<Document> = Vec::new();
    let all_pdfs_paths = find_pdfs(&directory).expect("Couln't find pdfs");

    let mut main_hs = MainH::default();

    for path in all_pdfs_paths.into_iter() {
        let content = read_from_pdf(&path);
        let data = my_split(&content);
        let tks = tokenize(data);
        main_hs.insert(path.clone(), tks);

        documents.push(Document {
            data: main_hs.clone(),
            path,
            last_modified: SystemTime::now(),
        });

        main_hs.clear();
    }

    search_query(documents, query);
}

fn find_pdfs(path: &String) -> Option<Vec<PathBuf>> {
    let mut files_vec: Vec<PathBuf> = Vec::new();
    let something = read_dir(path).expect("Couldn't read directory {path}");

    for fp in something {
        let path = fp.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "pdf" {
                files_vec.push(path)
            }
        }
    }

    Some(files_vec)
}

fn read_from_pdf(doc: &PathBuf) -> String {
    let pdf = PopplerDocument::new_from_file(doc, "").expect("Coulnd't read the document");
    let mut buff: String = String::new();
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
TODO maybe try to do it for bigger query
TODO maybe try to multithread the process if indexing docs
*/
