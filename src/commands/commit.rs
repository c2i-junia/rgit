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

    // write the current index to a tree object
    let tree_hash = write_tree();

    // get the current HEAD hash or reference
    let head_path = Path::new(".rgit").join("HEAD");
    let head_content = fs::read_to_string(&head_path).expect("Failed to read .rgit/HEAD");

    // check if HEAD points to a symbolic reference or is a direct commit hash
    let (head_target, is_symbolic_ref) = if head_content.starts_with("ref: ") {
        (
            head_content.trim_start_matches("ref: ").trim().to_string(),
            true,
        )
    } else {
        (head_content.trim().to_string(), false)
    };

    // get the current HEAD hash, if available
    let parent_hash = get_head_hash();

    // convert `String` to `Option<&str>`
    let parent_option: Option<&str> = if parent_hash.is_empty() {
        None
    } else {
        Some(parent_hash.as_str())
    };

    // create the new commit
    let commit_hash = commit_tree(commit_message, author, tree_hash, parent_option);

    // update the reference
    if is_symbolic_ref {
        // If HEAD points to a symbolic reference (e.g., a branch), update the branch itself
        update_ref(&head_target, &commit_hash);
        println!(
            "Committed as commit {} and updated branch '{}'",
            commit_hash, head_target
        );
    } else {
        // If HEAD is a direct commit hash (detached HEAD), update HEAD only
        update_ref("HEAD", &commit_hash);
        println!("Committed as commit {} in detached HEAD state", commit_hash);
    }
}
