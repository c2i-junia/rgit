use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::commands::cat_file::cat_file;
use std::collections::HashSet;
use std::process::Command;

pub enum RepoPath {
    Local,
    Remote(String),
}

pub fn create_object_path(repo_path: &RepoPath, hash: &str) -> PathBuf {
    match repo_path {
        RepoPath::Local => {
            let (dir, file) = hash.split_at(2);
            let rgit_objects = Path::new(".rgit").join("objects");
            let object_dir = rgit_objects.join(dir);
            if !object_dir.exists() {
                fs::create_dir_all(&object_dir).expect("Failed to create object subdirectory");
            }
            object_dir.join(file)
        }
        RepoPath::Remote(remote_repo_path) => {
            let (dir, file) = hash.split_at(2);
            let remote_objects_path = PathBuf::from(remote_repo_path).join("objects");
            remote_objects_path.join(dir).join(file)
        }
    }
}

pub fn compress_object(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).expect("Failed to compress data");
    encoder.finish().expect("Failed to finalize compression")
}

pub fn decompress_object(data: &[u8]) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed_data = Vec::new();
    decoder
        .read_to_end(&mut decompressed_data)
        .expect("Failed to decompress data");
    decompressed_data
}

pub fn hash_and_store(object_type: &str, content: &[u8]) -> String {
    let header = format!("{} {}\0", object_type, content.len());
    let mut store_data = Vec::new();
    store_data.extend(header.as_bytes());
    store_data.extend(content);

    let hash = Sha1::digest(&store_data);
    let hash_str = format!("{:x}", hash);
    let object_path = create_object_path(&RepoPath::Local, &hash_str);

    let compressed_data = compress_object(&store_data);
    fs::write(object_path, compressed_data).expect("Failed to write object to database");

    hash_str
}

pub fn collect_objects(repo_path: &RepoPath, commit_hash: &str) -> HashSet<String> {
    println!("collect_objects");
    let mut visited = HashSet::new();
    let mut to_visit = vec![commit_hash.to_string()];

    while let Some(current_hash) = to_visit.pop() {
        if visited.contains(&current_hash) {
            continue;
        }

        visited.insert(current_hash.clone());

        // read the content of the current commit or tree
        let content = cat_file(&repo_path, &current_hash);

        if content.contains("author") {
            // for commits, add the parent(s) and tree
            if let Some(tree_line) = content.lines().find(|line| line.starts_with("tree ")) {
                let tree_hash = tree_line.split_whitespace().nth(1).unwrap().to_string();
                to_visit.push(tree_hash);
            }

            // add parents to the list
            for parent_line in content.lines().filter(|line| line.starts_with("parent ")) {
                let parent_hash = parent_line.split_whitespace().nth(1).unwrap().to_string();
                to_visit.push(parent_hash);
            }
        } else if content.contains("tree") {
            // for trees, add blobs and subtrees
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 4 {
                    continue;
                }

                // let object_type = parts[1]; // type of the object (`blob` or `tree`)
                let object_hash = parts[2].to_string();

                // add blobs and subtrees to visit
                to_visit.push(object_hash);
            }
        }
    }

    visited
}

pub fn get_missing_objects(repo_path: &RepoPath, objects: &HashSet<String>) -> HashSet<String> {
    println!("get_missing_objects");

    match repo_path {
        // check for missing objects in the local repository
        RepoPath::Local => {
            // list of object hashes that are present locally
            let local_objects: HashSet<String> = objects
                .iter()
                .filter(|hash| {
                    // construct the local path for each object hash
                    let object_path = create_object_path(repo_path, hash);
                    Path::new(&object_path).exists()
                })
                .cloned()
                .collect();

            // return the objects that are in the input set but not found locally
            objects.difference(&local_objects).cloned().collect()
        }

        // check for missing objects in the remote repository
        RepoPath::Remote(remote_repo_path) => {
            // remote path to the `objects` directory
            let remote_objects_path = format!("{}/objects", remote_repo_path);

            // use SSH to list the objects in the remote repository
            let ssh_output = Command::new("ls")
                .arg(&remote_objects_path)
                .output()
                .expect("Failed to list objects on remote");

            // parse the output of the `ls` command and collect remote objects
            let remote_objects = String::from_utf8_lossy(&ssh_output.stdout)
                .lines()
                .map(|s| s.trim().to_string())
                .collect::<HashSet<_>>();

            // return the objects that are in the input set but not in the remote set
            objects.difference(&remote_objects).cloned().collect()
        }
    }
}
