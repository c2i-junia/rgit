use crate::commands::commit_tree::commit_tree;
use crate::commands::get_head_hash::get_head_hash;
use crate::commands::update_ref::update_ref;
use crate::commands::write_tree::write_tree;
use std::fs;
use std::path::Path;

pub fn commit(commit_message: &str, author: &str) {
    // check if the .rgit directory exists
    if !Path::new(".rgit").exists() {
        eprintln!("Error: No .rgit directory found. Are you in a repository?");
        std::process::exit(1);
    }

    // verify the index is not empty
    let index_path = Path::new(".rgit").join("index");
    let index_content = fs::read_to_string(&index_path).expect("Failed to read index");
    if index_content.trim().is_empty() {
        eprintln!("Error: Nothing to commit. The index is empty.");
        std::process::exit(1);
    }

    let tree_hash = write_tree(); 

    let parent_hash = get_head_hash();

    // convert `String` to `Option<&str>`
    let parent_option: Option<&str> = if parent_hash.is_empty() {
        None
    } else {
        Some(parent_hash.as_str())
    };

    let commit_hash = commit_tree(commit_message, author, tree_hash, parent_option);

    // update the reference of HEAD
    update_ref("HEAD", &commit_hash);

    println!("Committed as commit {}", commit_hash);
}
