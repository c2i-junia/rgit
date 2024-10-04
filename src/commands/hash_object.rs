use std::fs;
use std::io::Read;
use std::path::Path;
use sha1::{Digest, Sha1};
use crate::utils::{compress_object, create_object_path};

pub fn hash_object(file_path: &str) {
    let path = Path::new(file_path);
    let mut file = fs::File::open(path).expect("Failed to open file");

    // read the file content
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Failed to read file");

    // create the object header (blob) and concatenate with content
    let header = format!("blob {}\0", contents.len());
    let mut store_data = Vec::new();
    store_data.extend(header.as_bytes());
    store_data.extend(&contents);

    // compute the SHA-1 hash
    let hash = Sha1::digest(&store_data);
    let hash_str = format!("{:x}", hash);

    // create the corresponding object in .rgit/objects/
    let object_path = create_object_path(&hash_str);
    let compressed_data = compress_object(&store_data);
    fs::write(&object_path, compressed_data).expect("Failed to write object to database");

    println!("{}", hash_str);
}
