use std::fs;
use std::path::Path;

pub fn init() {
    let rgit_path = Path::new(".rgit");
    let objects_path = rgit_path.join("objects");

    if rgit_path.exists() {
        eprintln!("Error: .rgit already exists.");
        std::process::exit(1);
    }

    fs::create_dir(rgit_path).expect("Failed to create .rgit directory");
    fs::create_dir(&objects_path).expect("Failed to create .rgit/objects directory");

    let index_path = rgit_path.join("index");
    fs::write(index_path, "").expect("Failed to write index file");

    println!("Initialized empty rgit repository in {}", rgit_path.display());
}
