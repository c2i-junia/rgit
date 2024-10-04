use std::fs;
use std::path::Path;

pub fn update_index(file_name: &str, blob_hash: &str) {
    let index_path = Path::new(".rgit").join("index");
    let mut index_content = fs::read_to_string(&index_path).unwrap_or_default();

    index_content.push_str(&format!("{} {}\n", file_name, blob_hash));
    fs::write(index_path, index_content).expect("Failed to update index");

    println!("Updated index with file: {}", file_name);
}
