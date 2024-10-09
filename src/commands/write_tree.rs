use crate::utils::hash_and_store;
use std::fs;
use std::path::{Path, PathBuf};

pub fn write_tree() -> String {
    let index_path: PathBuf = Path::new(".rgit").join("index");
    let index_content: String = fs::read_to_string(&index_path).expect("Failed to read index");

    let mut tree_entries: Vec<String> = Vec::new();

    for line in index_content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            eprintln!("Invalid index entry: {}", line);
            continue;
        }
        let (file_name, blob_hash): (&str, &str) = (parts[0], parts[1]);
        let entry: String = format!("100644 blob {} {}\n", blob_hash, file_name);
        tree_entries.push(entry);
    }

    let tree_data: String = tree_entries.join("");
    let tree_hash: String = hash_and_store("tree", &tree_data.into_bytes());

    tree_hash
}
