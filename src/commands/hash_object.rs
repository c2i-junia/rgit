use crate::utils::{compress_object, create_object_path, RepoPath};
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn hash_object(file_path: &str) -> String {
    let path: &Path = Path::new(file_path);
    let mut file: fs::File = fs::File::open(path).expect("Failed to open file");

    // read the file content
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Failed to read file");

    // create the object header (blob) and concatenate with content
    let header: String = format!("blob {}\0", contents.len());
    let mut store_data: Vec<u8> = Vec::new();
    store_data.extend(header.as_bytes());
    store_data.extend(&contents);

    // compute the SHA-1 hash
    let hash: sha1::digest::Output<Sha1> = Sha1::digest(&store_data);
    let hash_str: String = format!("{:x}", hash);

    // create the corresponding object in .rgit/objects/
    let object_path: PathBuf = create_object_path(&RepoPath::Local, &hash_str);
    let compressed_data: Vec<u8> = compress_object(&store_data);
    fs::write(&object_path, compressed_data).expect("Failed to write object to database");

    hash_str
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests::*;
    use std::fs;

    #[test]
    fn test_hash_object_creates_object_file() {
        setup_test_repo();

        let test_content: &str = "test file content";
        let test_file_path: &str = "test_file.txt";
        fs::write(test_file_path, test_content).unwrap();

        hash_object(test_file_path);

        let expected_hash: &str = "2211df3faee131ad21edcb844e098a42c1fbb4e5"; // SHA-1 hash of the content

        let object_file: String = format!(".rgit/objects/22/{}", &expected_hash[2..]);
        assert!(
            Path::new(&object_file).exists(),
            "Object file should be created with correct hash."
        );

        // Cleanup
        remove_test_repo();
    }
}
