use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn read_index() -> HashMap<String, String> {
    let index_path = Path::new(".rgit").join("index");
    let mut index_map = HashMap::new();

    if let Ok(index_content) = fs::read_to_string(&index_path) {
        for line in index_content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let file_name = parts[0].to_string();
                let blob_hash = parts[1].to_string();
                index_map.insert(file_name, blob_hash);
            }
        }
    }

    index_map
}

fn write_index(index_map: &HashMap<String, String>) {
    let index_path = Path::new(".rgit").join("index");

    let new_index_content: String = index_map
        .iter()
        .map(|(file, hash)| format!("{} {}\n", file, hash))
        .collect();

    fs::write(index_path, new_index_content).expect("Failed to update index");
}

pub fn add_index(file_name: &str, blob_hash: &str) {
    let mut index_map = read_index();
    index_map.insert(file_name.to_string(), blob_hash.to_string());

    println!(
        "Added or updated index with file: {} -> {}",
        file_name, blob_hash
    );
    write_index(&index_map);
}

pub fn update_index(file_name: &str, blob_hash: &str) {
    let mut index_map = read_index();

    if index_map.contains_key(file_name) {
        index_map.insert(file_name.to_string(), blob_hash.to_string());
        println!("Updated index with file: {} -> {}", file_name, blob_hash);
        write_index(&index_map);
    } else {
        eprintln!(
            "Error: File {} not found in the index. Use add_index to add new files.",
            file_name
        );
    }
}

pub fn remove_index(file_name: &str) {
    let mut index_map = read_index();

    if index_map.remove(file_name).is_some() {
        println!("Removed {} from index.", file_name);
        write_index(&index_map);
    } else {
        eprintln!("Error: File {} not found in the index.", file_name);
    }
}
