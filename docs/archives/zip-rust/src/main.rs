use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn compress(data: &[u8], verbose: bool) -> Vec<u32> 
{
    let mut dictionary: HashMap<Vec<u8>, u32> = HashMap::new();

    // assign a code to all individual ASCII characters
    for i in 0..=255 {
        dictionary.insert(vec![i as u8], i as u32);
    }

    let mut compressed_data: Vec<u32> = Vec::new();
    let mut next_code: u32 = 256;
    let mut current_sequence: Vec<u8> = Vec::new();

    for &byte in data {
        // store the next sequence
        let mut next_sequence = current_sequence.clone();
        next_sequence.push(byte);

        if dictionary.contains_key(&next_sequence) {
            // update the current sequence
            current_sequence = next_sequence;
        } 
        else {
            // add the sequence to the dict, and reset the current sequence
            compressed_data.push(dictionary[&current_sequence]);
            dictionary.insert(next_sequence.clone(), next_code);
            next_code += 1;
            current_sequence = vec![byte];
        }
    }

    if !current_sequence.is_empty() {
        compressed_data.push(dictionary[&current_sequence]);
    }

    if verbose {
        println!("Final dictionary state:");
        for (seq, code) in &dictionary {
            println!("{:?} -> {}", seq, code);
        }
    }

    compressed_data
}

fn decompress(compressed_data: &[u32], verbose: bool) -> Vec<u8>
{
    let mut dictionary: HashMap<u32, Vec<u8>> = HashMap::new();
    for i in 0..=255 {
        dictionary.insert(i as u32, vec![i as u8]);
    }

    let mut next_code: u32 = 256;
    let mut current_sequence = vec![compressed_data[0]];
    let mut result = dictionary[&current_sequence[0]].clone();

    for &code in &compressed_data[1..] {
        let entry = if dictionary.contains_key(&code) {
            dictionary[&code].clone()
        } 
        else if code == next_code {
            let mut new_entry = dictionary[&current_sequence[0]].clone();
            new_entry.push(new_entry[0]);
            new_entry
        }
        else {
            panic!("Error during decompression with code: {}", code);
        };

        result.extend(&entry);
        let mut new_sequence = dictionary[&current_sequence[0]].clone();
        new_sequence.push(entry[0]);
        dictionary.insert(next_code, new_sequence);
        next_code += 1;

        current_sequence = vec![code];
    }

    result
}

fn parse_arguments(args: &[String]) -> (&str, &str, &str, bool) 
{
    if args.len() < 3 {
        eprintln!(
            "Usage: {} [--decompress | --compress] [--verbose] <input_file> <output_file>",
            args[0]
        );
        std::process::exit(1);
    }

    let mut mode;
    let input_file;
    let output_file;
    let mut verbose = false;

    for arg in args {
        if arg == "--decompress" {
            mode = "decompress";
        } 
        else if arg == "--compress" {
            mode = "compress";
        } 
        else if arg == "--verbose" {
            verbose = true;
        }
    }

    if mode.is_empty()

    let file_args: Vec<&String> = args.iter().filter(|&arg| !arg.starts_with("--")).collect();
    if file_args.len() < 2 {
        eprintln!("Missing input or output file.");
        std::process::exit(1);
    }

    input_file = &file_args[0];
    output_file = &file_args[1];

    (mode, input_file, output_file, verbose)

}

fn main() -> io::Result<()> 
{
    // get options
    let args: Vec<String> = env::args().collect();
    let (mode, input_file, output_file, verbose) = parse_arguments(&args);

    // store the input file in binary mode in the buffer variable
    let mut file = File::open(&Path::new(input_file)).expect("Error opening input file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read the input file");


    if mode == "compress" {
        let compressed_data = compress(&buffer, verbose);

        // write the compressed data to the output file
        let mut output = File::create(&Path::new(output_file)).expect("Error creating output file");
        for code in compressed_data {
            output
                .write_all(&code.to_be_bytes())
                .expect("Failed to write compressed data");
        }
        println!("Compression complete. Output saved to {}", output_file);
    } 
    else if mode == "decompress" {
        
        let mut compressed_data: Vec<u32> = Vec::new();
        // right now we have a vector that contains u8, but we want to have
        // a vec that contains the codes, and the codes are stored on 32 bits
        // so we create a compressed_data vector
        for chunk in buffer.chunks_exact(4) {
            let code = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            compressed_data.push(code);
        }

        // decompress the data
        let decompressed_data = decompress(&compressed_data, verbose);

        // write the decompressed data to the output file
        let mut output = File::create(&Path::new(output_file)).expect("Error creating output file");
        output
            .write_all(&decompressed_data)
            .expect("Failed to write decompressed data");

        println!("Decompression complete. Output saved to {}", output_file);
    }

    Ok(())
}
