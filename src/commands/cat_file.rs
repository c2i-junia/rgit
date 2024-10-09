use crate::utils::{create_object_path, decompress_object, RepoPath};
use std::fs;

pub fn cat_file(repo_path: &RepoPath, hash: &str) -> String {
    let object_path: std::path::PathBuf = create_object_path(repo_path, hash);
    if !object_path.exists() {
        eprintln!("Object {} not found.", hash);
        std::process::exit(1);
    }

    let compressed_data: Vec<u8> = fs::read(object_path).expect("Failed to read object file");
    let decompressed_data: Vec<u8> = decompress_object(&compressed_data);

    let null_byte_pos: usize = decompressed_data.iter().position(|&b| b == 0).unwrap();
    let (_, contents): (&[u8], &[u8]) = decompressed_data.split_at(null_byte_pos + 1);

    String::from_utf8_lossy(contents).into_owned()
}
