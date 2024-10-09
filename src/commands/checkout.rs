use crate::commands::cat_file::cat_file;
use crate::commands::symbolic_ref::*;
use crate::commands::update_ref::*;
use crate::utils::RepoPath;
use std::fs;
use std::path::{Path, PathBuf};

pub fn checkout(target: &str) {
    // check if the target is a branch
    let branch_ref_path: String = format!(".rgit/refs/{}", target);
    let commit_hash: String = if Path::new(&branch_ref_path).exists() {
        // read the commit hash from the branch reference file
        fs::read_to_string(&branch_ref_path)
            .expect("Failed to read branch reference")
            .trim()
            .to_string()
    } else {
        // assume the target is a commit hash
        target.to_string()
    };

    // obtain the tree hash from the commit
    let commit_content: String = cat_file(&RepoPath::Local, &commit_hash);
    let tree_hash: &str = commit_content
        .lines()
        .find(|line| line.starts_with("tree "))
        .map(|line| line.split_whitespace().nth(1).unwrap())
        .expect("Commit does not contain a tree");

    // clear the current working directory
    clear_working_directory();

    // restore the tree associated with the commit
    restore_tree(tree_hash, Path::new("."));
    println!("Checked out to {}", target);

    // update HEAD
    if Path::new(&branch_ref_path).exists() {
        // If the target is a branch, make HEAD point to this branch
        symbolic_ref("HEAD", &format!("refs/{}", target));
        println!("Checked out to branch '{}'", target);
    } else {
        // If the target is a commit hash, set HEAD directly to this commit
        update_ref("HEAD", &commit_hash);
        println!("Checked out to commit '{}'", commit_hash);
    }
}

fn clear_working_directory() {
    let current_dir: &Path = Path::new(".");
    for entry in fs::read_dir(current_dir).expect("Failed to read current directory") {
        let entry: fs::DirEntry = entry.expect("Failed to get directory entry");
        let path: PathBuf = entry.path();

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
    let tree_content: String = cat_file(&RepoPath::Local, tree_hash);

    for line in tree_content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }

        let _file_mode: &str = parts[0];
        let object_type: &str = parts[1];
        let hash: &str = parts[2];
        let file_name: &str = parts[3];

        let file_path: PathBuf = base_path.join(file_name);

        if object_type == "blob" {
            let blob_content: String = cat_file(&RepoPath::Local, hash);
            fs::write(&file_path, blob_content).expect("Failed to write file");
            println!("Restored file: {}", file_name);
        }
    }
}
