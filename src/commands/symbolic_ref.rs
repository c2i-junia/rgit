use std::fs;
use std::path::{Path, PathBuf};

pub fn symbolic_ref(ref_name: &str, target_ref: &str) {
    // construct the full path to the symbolic reference file (e.g., ".rgit/HEAD")
    let ref_path: PathBuf = Path::new(".rgit").join(ref_name);

    // create necessary directories if they don't exist
    if let Some(parent_dir) = ref_path.parent() {
        fs::create_dir_all(parent_dir)
            .expect("Failed to create parent directories for the reference");
    }

    // write the target reference (e.g., "ref: refs/heads/main") to the file
    let symbolic_content: String = format!("ref: {}", target_ref);
    fs::write(&ref_path, symbolic_content).expect("Failed to update symbolic reference");

    println!(
        "Updated symbolic reference '{}' to point to '{}'",
        ref_name, target_ref
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests::{remove_test_repo, setup_test_repo};
    use std::fs;

    #[test]
    fn test_symbolic_ref_updates_reference() {
        setup_test_repo();

        // initialize a symbolic HEAD reference pointing to the `refs/heads/main` branch
        let ref_name: &str = "HEAD";
        let target_ref: &str = "refs/heads/main";
        symbolic_ref(ref_name, target_ref);

        // check that the `.rgit/HEAD` file contains the correct symbolic reference
        let head_content: String = fs::read_to_string(".rgit/HEAD").unwrap();
        assert_eq!(
            head_content.trim(),
            format!("ref: {}", target_ref),
            "head should point to the correct reference."
        );

        remove_test_repo();
    }
}
