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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests::{remove_test_repo, setup_test_repo};
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_write_tree_creates_tree_object() {
        setup_test_repo();

        // add files to the index
        let file1: &str = "file1.txt";
        let file2: &str = "file2.txt";
        fs::write(file1, "content of file 1").unwrap();
        fs::write(file2, "content of file 2").unwrap();

        // generate blob hashes for the files and add them to the index
        let hash1: String = crate::commands::hash_object::hash_object(file1);
        let hash2: String = crate::commands::hash_object::hash_object(file2);
        crate::commands::update_index::add_index(file1, &hash1);
        crate::commands::update_index::add_index(file2, &hash2);

        // create the tree object from the index
        let tree_hash: String = write_tree();

        // verify that the tree object file was created
        let tree_path: String = format!(".rgit/objects/{}/{}", &tree_hash[0..2], &tree_hash[2..]);
        assert!(
            Path::new(&tree_path).exists(),
            "tree object should be created."
        );

        remove_test_repo();
    }
}
