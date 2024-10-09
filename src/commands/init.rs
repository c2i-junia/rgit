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
