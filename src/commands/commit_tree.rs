use crate::utils::hash_and_store;

/// Creates a new commit object in the database.
pub fn commit_tree(
    commit_name: &str,
    author: &str,
    tree_hash: String,
    parent_hash: Option<&str>,
) -> String {
    let parent_line: String = if let Some(parent) = parent_hash {
        format!("parent {}\n", parent)
    } else {
        String::new()
    };

    let commit_content: String = format!(
        "tree {}\n{}author {}\n\n{}\n",
        tree_hash.as_str(),
        parent_line,
        author,
        commit_name
    );
    let commit_hash: String = hash_and_store("commit", &commit_content.into_bytes());

    commit_hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::cat_file::cat_file;
    use crate::utils::tests::{remove_test_repo, setup_test_repo}; // import test utilities
    use crate::utils::RepoPath;
    use std::path::Path;

    #[test]
    fn test_commit_tree_creates_commit_object() {
        setup_test_repo();

        // simulate a `tree_hash` for the commit
        let tree_hash: &str = "dummy_tree_hash";

        // call the `commit_tree` function to create the commit
        let commit_message: &str = "Initial commit";
        let author: &str = "John Doe <john.doe@example.com>";
        let commit_hash: String = commit_tree(commit_message, author, tree_hash.to_string(), None);

        // verify that the commit was created in the `.rgit/objects` directory
        let object_path: String =
            format!(".rgit/objects/{}/{}", &commit_hash[0..2], &commit_hash[2..]);
        assert!(
            Path::new(&object_path).exists(),
            "commit object should be created."
        );

        // read the content of the commit using `cat_file`
        let commit_content: String = cat_file(&RepoPath::Local, &commit_hash);
        println!("commit content:\n{}", commit_content);

        // verify that the commit content is correct
        assert!(
            commit_content.contains("tree dummy_tree_hash"),
            "commit should contain the correct tree hash."
        );
        assert!(
            commit_content.contains("author John Doe <john.doe@example.com>"),
            "commit should contain the correct author."
        );
        assert!(
            commit_content.contains("Initial commit"),
            "commit should contain the correct commit message."
        );

        remove_test_repo();
    }

    #[test]
    fn test_commit_tree_with_parent_hash() {
        setup_test_repo();

        // simulate two commits using a `parent_hash`
        let tree_hash: &str = "dummy_tree_hash";
        let commit_message1: &str = "First commit";
        let author: &str = "John Doe <john.doe@example.com>";
        let commit_hash1: String =
            commit_tree(commit_message1, author, tree_hash.to_string(), None);

        // create a second commit with `commit_hash1` as the parent
        let commit_message2: &str = "Second commit";
        let commit_hash2: String = commit_tree(
            commit_message2,
            author,
            tree_hash.to_string(),
            Some(&commit_hash1),
        );

        // verify that the second commit has a parent
        let object_path: String = format!(
            ".rgit/objects/{}/{}",
            &commit_hash2[0..2],
            &commit_hash2[2..]
        );
        assert!(
            Path::new(&object_path).exists(),
            "commit object with parent should be created."
        );

        // read the content of the commit using `cat_file`
        let commit_content: String = cat_file(&RepoPath::Local, &commit_hash2);
        println!("commit content with parent:\n{}", commit_content);

        // verify the commit content with the parent
        assert!(
            commit_content.contains("parent"),
            "commit should contain a parent reference."
        );
        assert!(
            commit_content.contains(&commit_hash1),
            "commit should contain the correct parent hash."
        );
        assert!(
            commit_content.contains("tree dummy_tree_hash"),
            "commit should contain the correct tree hash."
        );
        assert!(
            commit_content.contains("author John Doe <john.doe@example.com>"),
            "commit should contain the correct author."
        );
        assert!(
            commit_content.contains("Second commit"),
            "commit should contain the correct commit message."
        );

        remove_test_repo();
    }
}
