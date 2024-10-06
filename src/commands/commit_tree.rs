use crate::utils::hash_and_store;

/// creates a new commit object in the database
pub fn commit_tree(commit_name: &str, author: &str, tree_hash: &str, parent_hash: Option<&str>) {
    let parent_line = if let Some(parent) = parent_hash {
        format!("parent {}\n", parent)
    } else {
        String::new()
    };

    let commit_content = format!(
        "tree {}\n{}author {}\n\n{}\n",
        tree_hash, parent_line, author, commit_name
    );
    let commit_hash = hash_and_store("commit", &commit_content.into_bytes());

    println!("{}", commit_hash);
}
