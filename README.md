
# TF-IDF
The project aims to provide functionalities for indexing and searching PDF documents based on specific queries. It utilizes Rust programming language and various libraries for PDF parsing, data manipulation, and file operations.

# Features
- **Indexing**: The project can index the content of PDF documents, extract relevant information, and store it for efficient searching.
- **Searching**: Users can search for specific queries within the indexed PDF documents and retrieve relevant results.
- **Serialization**: The project provides functionality to serialize the indexed data into a JSON file for persistent storage and faster retrieval.
- **Performance Optimization**: The codebase incorporates optimizations such as caching and intelligent re-indexing to enhance search performance.

# Prerequisites
**Rust programming language**
**__Library dependencies__**

# Installation
1. Clone the repository: git clone https://github.com/your-username/project-name.git
2. Change to the project directory: cd project-name
3. Build the project: cargo build
4. Run the project: cargo run

# Usage
Indexing Data: Use the tokenize_data function to index PDF documents. Provide a list of PDF file paths as input, and it will generate a list of Document objects containing the indexed data.
Searching: Use the search_query function to search for specific queries within the indexed data. Provide the list of Document objects and the query string as input, and it will return the relevant search results.
Serialization: Utilize the serialize_and_save function to serialize the indexed data into a JSON file. Provide the list of Document objects and the output file path as input, and it will save the serialized data for future use.
Library Dependencies
serde: A powerful serialization framework for Rust.
serde_json: A JSON serialization and deserialization library.
poppler-rs: A Rust binding for the Poppler PDF library.
