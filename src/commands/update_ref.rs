use std::fs;
use std::path::Path;

pub fn update_ref(ref_name: &str, commit_hash: &str) {
    // construct the full path to the reference (e.g., ".rgit/refs/master")
    let ref_path = Path::new(".rgit").join(ref_name);

    // create necessary directories if they don't exist
    if let Some(parent_dir) = ref_path.parent() {
        fs::create_dir_all(parent_dir).expect("Failed to create parent directories for the reference");
    }

    // write the commit hash to the reference file
    fs::write(&ref_path, commit_hash).expect("Failed to update reference");

    println!("Updated reference '{}' to point to commit {}", ref_name, commit_hash);
}
