use crate::commands::cat_file::cat_file;

pub fn log(commit_hash: &str) {
    // start from the given commit
    let mut current_commit = commit_hash.to_string();

    // traverse all commits until there is no more parent
    while !current_commit.is_empty() {
        // read the content of the current commit
        let commit_content = cat_file(&current_commit);

        let author = commit_content
            .lines()
            .find(|line| line.starts_with("author "))
            .map(|line| line.trim_start_matches("author "))
            .unwrap_or("Unknown author");

        let message = commit_content
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .collect::<Vec<&str>>()
            .join("\n");

        println!("Commit: {}", current_commit);
        println!("Author: {}", author);
        println!("Message: {}\n", message);

        current_commit = commit_content
            .lines()
            .find(|line| line.starts_with("parent "))
            .map(|line| line.split_whitespace().nth(1).unwrap().to_string())
            .unwrap_or(String::new());
    }
}
