use crate::commands::cat_file::cat_file;
use crate::commands::get_head_hash::*;
use crate::utils::RepoPath;
use std::fs;
use std::path::Path;

pub fn log(target: &str) {
    // determine the commit hash to start from
    let commit_hash = if target == "HEAD" {
        // if target is HEAD, use get_head_hash to retrieve the current commit hash
        get_head_hash()
    } else {
        // check if the target is a branch
        let branch_ref_path = format!(".rgit/refs/{}", target);
        if Path::new(&branch_ref_path).exists() {
            // read the commit hash from the branch reference file
            fs::read_to_string(&branch_ref_path)
                .expect("Failed to read branch reference")
                .trim()
                .to_string()
        } else {
            // assume the target is a commit hash
            target.to_string()
        }
    };

    // start from the given commit
    let mut current_commit = commit_hash;

    // traverse all commits until there is no more parent
    while !current_commit.is_empty() {
        // read the content of the current commit
        let commit_content = cat_file(&RepoPath::Local, &current_commit);

        let author = commit_content
            .lines()
            .find(|line| line.starts_with("author "))
            .map(|line| line.trim_start_matches("author "))
            .unwrap_or("unknown author");

        let message = commit_content
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .collect::<Vec<&str>>()
            .join("\n");

        println!("Commit: {}", current_commit);
        println!("Author: {}", author);
        println!("Message: {}\n", message);

        // move to the parent commit (if present)
        current_commit = commit_content
            .lines()
            .find(|line| line.starts_with("parent "))
            .map(|line| line.split_whitespace().nth(1).unwrap().to_string())
            .unwrap_or(String::new());
    }
}
