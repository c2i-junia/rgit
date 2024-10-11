use crate::utils::RepoPath;
use crate::utils::*;
use std::collections::HashSet;
use std::fs;
use std::process::Command;

pub fn fetch(remote_repo_path: &str, branch: &str) {
    println!("Fetching from remote repository: {}", remote_repo_path);

    // determine the remote branch reference (e.g., "refs/heads/main")
    let remote_branch_ref: String = format!("{}/refs/{}", remote_repo_path, branch);

    // read the remote branch reference to get the latest commit hash
    let remote_commit_hash: String = fs::read_to_string(&remote_branch_ref)
        .expect("Failed to read remote branch reference")
        .trim()
        .to_string();

    println!(
        "Remote branch {} points to commit {}",
        branch, remote_commit_hash
    );

    let repo_path: RepoPath = RepoPath::Remote(remote_repo_path.to_string());

    // collect all necessary objects
    let objects: HashSet<String> = collect_objects(&repo_path, &remote_commit_hash);

    // check which objects are missing on the remote
    let missing_objects: HashSet<String> = get_missing_objects(&RepoPath::Local, &objects);

    for object_hash in missing_objects {
        let subdir: &str = &object_hash[0..2];
        let filename: &str = &object_hash[2..];
        let remote_object_path: String =
            format!("{}/objects/{}/{}", remote_repo_path, subdir, filename);
        let local_object_dir: String = format!(".rgit/objects/{}", subdir);
        let local_object_path: String = format!("{}/{}", local_object_dir, filename);

        // ensure the local directory exists
        fs::create_dir_all(&local_object_dir).expect("Failed to create local object directory");

        // copy the file from remote to local
        let status: std::process::ExitStatus = Command::new("cp")
            .arg(&remote_object_path)
            .arg(&local_object_path)
            .status()
            .expect("Failed to copy object from remote repository");

        if status.success() {
            println!("Copied object {} to local repository", object_hash);
        } else {
            eprintln!("Failed to copy object {}", object_hash);
        }
    }

    // update the local reference to point to the fetched commit
    let local_branch_ref: String = format!(".rgit/refs/remotes/{}", branch);
    fs::write(&local_branch_ref, remote_commit_hash)
        .expect("Failed to update local branch reference");

    println!(
        "Successfully updated local reference for branch '{}'",
        branch
    );
}
