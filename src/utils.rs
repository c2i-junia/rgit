use std::fs;
use std::path::{Path, PathBuf};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::{Read, Write};
use sha1::{Digest, Sha1};

pub fn create_object_path(hash: &str) -> PathBuf {
    let (dir, file) = hash.split_at(2);
    let rgit_objects = Path::new(".rgit").join("objects");
    let object_dir = rgit_objects.join(dir);
    if !object_dir.exists() {
        fs::create_dir_all(&object_dir).expect("Failed to create object subdirectory");
    }
    object_dir.join(file)
}

pub fn compress_object(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).expect("Failed to compress data");
    encoder.finish().expect("Failed to finalize compression")
}

pub fn decompress_object(data: &[u8]) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).expect("Failed to decompress data");
    decompressed_data
}

pub fn hash_and_store(object_type: &str, content: &[u8]) -> String {
    let header = format!("{} {}\0", object_type, content.len());
    let mut store_data = Vec::new();
    store_data.extend(header.as_bytes());
    store_data.extend(content);

    let hash = Sha1::digest(&store_data);
    let hash_str = format!("{:x}", hash);
    let object_path = create_object_path(&hash_str);

    let compressed_data = compress_object(&store_data);
    fs::write(object_path, compressed_data).expect("Failed to write object to database");

    hash_str
}
