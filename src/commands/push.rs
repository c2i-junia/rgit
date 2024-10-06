use crate::update_ref;
use crate::utils::RepoPath;
use crate::utils::*;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn push(remote_path: &str, branch: &str) {
    let local_branch_ref = format!(".rgit/refs/{}", branch);
    let commit_hash = fs::read_to_string(&local_branch_ref)
        .expect("Failed to read local branch reference")
        .trim()
        .to_string();

    let repo_path: RepoPath = RepoPath::Local;

    // collect all necessary objects
    let objects = collect_objects(&repo_path, &commit_hash);
    println!("objects: {:?}", objects);

    // check which objects are missing on the remote
    let missing_objects = get_missing_objects(&RepoPath::Remote(remote_path.to_string()), &objects);
    println!("missing_objects: {:?}", missing_objects);

    // transfer each missing object using scp
    println!("transfer object with scp");
    for object_hash in missing_objects {
        // construct the local path based on the object's hash
        let subdir = &object_hash[0..2]; // first 2 characters of the hash
        let filename = &object_hash[2..]; // rest of the hash

        // create the correct local and remote paths
        let local_object_path = format!(".rgit/objects/{}/{}", subdir, filename);
        let remote_object_dir = format!("{}/objects/{}", remote_path, subdir);
        let remote_object_path = format!("{}/{}", remote_object_dir, filename);

        // check if the object exists locally
        if !Path::new(&local_object_path).exists() {
            eprintln!(
                "Error: Object {} not found in local repository.",
                object_hash
            );
            continue;
        }

        // create the remote directory if it doesn't exist
        println!("Creating remote directory: {}", remote_object_dir);
        let mkdir_status = Command::new("mkdir")
            .arg("-p") // create parent directories as needed
            .arg(&remote_object_dir)
            .status()
            .expect("Failed to create remote object directory");

        if !mkdir_status.success() {
            eprintln!("Failed to create remote directory {}", remote_object_dir);
            continue;
        }

        // use `cp` to transfer the object to the remote repository
        let status = Command::new("cp")
            .arg(&local_object_path)
            .arg(&remote_object_path)
            .status()
            .expect("Failed to transfer object");

        if !status.success() {
            eprintln!("Failed to transfer object {} to remote", object_hash);
        }
    }

    println!("update_ref");
    // update the remote reference for the branch
    update_ref(&format!("{}/refs/{}", remote_path, branch), &commit_hash);
}
