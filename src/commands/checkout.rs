use std::fs;
use std::path::Path;
use crate::commands::cat_file::cat_file;

pub fn checkout(commit_hash: &str) {
    // get the list of all parents, actual commit is also included
    let commit_chain = get_commit_chain(commit_hash.to_string());

    clear_working_directory();

    // we restore the tree for each commit
    for commit in &commit_chain {
        println!("Restoring tree for commit: {}", commit);
        let commit_content = cat_file(commit);
        let tree_hash = commit_content
            .lines()
            .find(|line| line.starts_with("tree "))
            .map(|line| line.split_whitespace().nth(1).unwrap())
            .expect("Commit does not contain a tree");

        restore_tree(tree_hash, Path::new("."));
    }
}

// retrieves the complete chain of parent commits, from the oldest to the newest
fn get_commit_chain(mut commit_hash: String) -> Vec<String> {
    let mut chain = Vec::new();

    // iterate through all parents until reaching a commit without a parent
    while !commit_hash.is_empty() {
        chain.push(commit_hash.clone()); 
        let commit_content = cat_file(&commit_hash);

        // get the parent, if there is one
        commit_hash = commit_content
            .lines()
            .find(|line| line.starts_with("parent "))
            .map(|line| line.split_whitespace().nth(1).unwrap().to_string()) // Convertir le parent en String
            .unwrap_or(String::new());  // Si pas de parent, arrêter la boucle
    }

    // reverse the output to have the most recent commit hash first
    chain.reverse();
    chain
}

fn clear_working_directory() {
    let current_dir = Path::new(".");
    for entry in fs::read_dir(current_dir).expect("Failed to read current directory") {
        let entry = entry.expect("Failed to get directory entry");
        let path = entry.path();

        // be careful to not remove the `.rgit/` directory
        if path.ends_with(".rgit") {
            continue;
        }

        // remove files and directory
        if path.is_dir() {
            fs::remove_dir_all(&path).expect("Failed to remove directory");
        } else {
            fs::remove_file(&path).expect("Failed to remove file");
        }
    }
    println!("Cleared working directory.");
}

/// restores a tree object into the working directory
pub fn restore_tree(tree_hash: &str, base_path: &Path) {
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

        if object_type == "blob" {
            let file_path = base_path.join(file_name);
            let blob_content = cat_file(hash);
            fs::write(file_path, blob_content).expect("Failed to write file");
            println!("Restored file: {}", file_name);
        }
    }
}
