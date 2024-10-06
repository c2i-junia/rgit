use crate::commands::cat_file::cat_file;
use crate::utils::RepoPath;
use std::fs;
use std::path::Path;

pub fn checkout(target: &str) {
    // check if the target is a branch
    let branch_ref_path = format!(".rgit/refs/{}", target);
    let commit_hash = if Path::new(&branch_ref_path).exists() {
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
    let commit_content = cat_file(&RepoPath::Local, &commit_hash);
    let tree_hash = commit_content
        .lines()
        .find(|line| line.starts_with("tree "))
        .map(|line| line.split_whitespace().nth(1).unwrap())
        .expect("Commit does not contain a tree");

    // clear the current working directory
    clear_working_directory();

    // restore the tree associated with the commit
    restore_tree(tree_hash, Path::new("."));
    println!("Checked out to {}", target);
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
    let tree_content = cat_file(&RepoPath::Local, tree_hash);

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
            let blob_content = cat_file(&RepoPath::Local, hash);
            fs::write(&file_path, blob_content).expect("Failed to write file");
            println!("Restored file: {}", file_name);
        }
    }
}
