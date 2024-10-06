use std::fs;
use std::path::Path;
use crate::commands::cat_file::cat_file;

pub fn checkout(commit_hash: &str) {
    // get the tree hash
    let commit_content = cat_file(commit_hash);
    let tree_hash = commit_content
        .lines()
        .find(|line| line.starts_with("tree "))
        .map(|line| line.split_whitespace().nth(1).unwrap())
        .expect("Commit does not contain a tree");

    clear_working_directory();

    restore_tree(tree_hash, Path::new("."));
}

fn clear_working_directory() {
    let current_dir = Path::new(".");
    for entry in fs::read_dir(current_dir).expect("Failed to read current directory") {
        let entry = entry.expect("Failed to get directory entry");
        let path = entry.path();

        // do not remove .rgit directory
        if path.ends_with(".rgit") {
            continue;
        }

        // remove files and folders
        if path.is_dir() {
            fs::remove_dir_all(&path).expect("Failed to remove directory");
        } else {
            fs::remove_file(&path).expect("Failed to remove file");
        }
    }
    println!("Cleared working directory.");
}

fn restore_tree(tree_hash: &str, base_path: &Path) {
    let tree_content = cat_file(tree_hash);

    for line in tree_content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }

        let _file_mode = parts[0];
        let object_type = parts[1];
        let hash = parts[2];
        let file_name = parts[3];

        let file_path = base_path.join(file_name);

        if object_type == "blob" {
            let blob_content = cat_file(hash);
            fs::write(&file_path, blob_content).expect("Failed to write file");
            println!("Restored file: {}", file_name);
        }
    }
}
