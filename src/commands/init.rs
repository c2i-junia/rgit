use std::fs;
use std::path::{Path, PathBuf};

pub fn init() {
    let rgit_path: &Path = Path::new(".rgit");
    let objects_path: PathBuf = rgit_path.join("objects");
    let refs_path: PathBuf = rgit_path.join("refs");

    if rgit_path.exists() {
        eprintln!("Error: .rgit already exists.");
        std::process::exit(1);
    }

    fs::create_dir(rgit_path).expect("Failed to create .rgit directory");
    fs::create_dir(&objects_path).expect("Failed to create .rgit/objects directory");
    fs::create_dir(&refs_path).expect("Failed to create .rgit/refs directory");

    let index_path: PathBuf = rgit_path.join("index");
    let head_path: PathBuf = rgit_path.join("HEAD");
    fs::write(index_path, "").expect("Failed to write index file");
    fs::write(head_path, "").expect("Failed to write HEAD file");

    println!(
        "Initialized empty rgit repository in {}",
        rgit_path.display()
    );
}

#[cfg(test)]
mod tests {
    use crate::utils::tests::*;
    use std::path::Path;

    #[test]
    fn test_init_creates_rgit_directory() {
        setup_test_repo();

        assert!(
            Path::new(".rgit").exists(),
            ".rgit directory should be created."
        );
        assert!(
            Path::new(".rgit/objects").exists(),
            "objects directory should be created."
        );
        assert!(
            Path::new(".rgit/refs").exists(),
            "refs directory should be created."
        );
        assert!(
            Path::new(".rgit/index").exists(),
            "index file should be created."
        );
        assert!(
            Path::new(".rgit/HEAD").exists(),
            "HEAD file should be created."
        );

        remove_test_repo();
    }
}
