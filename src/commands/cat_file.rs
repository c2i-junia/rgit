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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests::{remove_test_repo, setup_test_repo};
    use crate::utils::{hash_and_store, RepoPath};

    #[test]
    fn test_cat_file_reads_stored_object() {
        setup_test_repo();

        // create and store a blob object
        let object_type: &str = "blob";
        let content: &str = "This is a test content for the cat_file function.";
        let object_hash: String = hash_and_store(object_type, content.as_bytes());

        // read the object content using `cat_file`
        let output: String = cat_file(&RepoPath::Local, &object_hash);

        // verify that the returned content is correct
        assert_eq!(
            output, content,
            "The content read by `cat_file` should match the original content."
        );

        // clean up the test repository
        remove_test_repo();
    }
}
