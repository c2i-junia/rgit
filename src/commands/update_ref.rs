use std::fs;
use std::path::{Path, PathBuf};

pub fn update_ref(ref_name: &str, commit_hash: &str) {
    // construct the full path to the reference (e.g., ".rgit/refs/master")
    let ref_path: PathBuf = Path::new(".rgit").join(ref_name);

    // create necessary directories if they don't exist
    if let Some(parent_dir) = ref_path.parent() {
        fs::create_dir_all(parent_dir)
            .expect("Failed to create parent directories for the reference");
    }

    // write the commit hash to the reference file
    fs::write(&ref_path, commit_hash).expect("Failed to update reference");

    println!(
        "Updated reference '{}' to point to commit {}",
        ref_name, commit_hash
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests::{remove_test_repo, setup_test_repo};
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_update_ref_creates_reference_file() {
        setup_test_repo();

        // update a reference (e.g., "refs/heads/main") with a dummy commit hash
        let ref_name: &str = "refs/heads/main";
        let commit_hash: &str = "1234567890abcdef1234567890abcdef12345678"; // dummy commit hash

        // call the `update_ref` function
        update_ref(ref_name, commit_hash);

        // check that the reference file is created with the correct content
        let ref_file_path: String = format!(".rgit/{}", ref_name); // the full path to the reference file
        assert!(
            Path::new(&ref_file_path).exists(),
            "reference file should be created."
        );

        // read the content of the reference file to verify the commit hash
        let stored_hash: String =
            fs::read_to_string(&ref_file_path).expect("failed to read the reference file");
        assert_eq!(
            stored_hash.trim(),
            commit_hash,
            "the reference file should point to the correct commit hash."
        );

        remove_test_repo();
    }
}
