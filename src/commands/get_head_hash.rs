use std::fs;
use std::path::Path;

pub fn get_head_hash() -> String {
    // Path to the HEAD file in the repository
    let head_path = Path::new(".rgit/HEAD");

    // Read the content of the HEAD file
    let head_content = fs::read_to_string(&head_path).expect("Failed to read .rgit/HEAD");

    // Check if HEAD is a symbolic reference
    if head_content.starts_with("ref: ") {
        // Extract the reference path (e.g., "refs/heads/main")
        let ref_path = head_content.trim_start_matches("ref: ").trim();

        // Construct the path to the reference file (e.g., ".rgit/refs/heads/main")
        let ref_full_path = Path::new(".rgit").join(ref_path);

        // Read and return the commit hash stored in the reference file
        fs::read_to_string(&ref_full_path)
            .expect(&format!(
                "Failed to read reference file at {}",
                ref_full_path.display()
            ))
            .trim()
            .to_string()
    } else {
        // If HEAD contains a commit hash, return it directly
        head_content.trim().to_string()
    }
}
