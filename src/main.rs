mod commands;
mod utils;

use commands::cat_file::cat_file;
use commands::checkout::checkout;
use commands::commit_tree::commit_tree;
use commands::hash_object::hash_object;
use commands::init::init;
use commands::update_index::update_index;
use commands::write_tree::write_tree;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rgit <command> [<args>]");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "init" => init(),
        "hash-object" => {
            if args.len() != 3 {
                eprintln!("Usage: rgit hash-object <file>");
                std::process::exit(1);
            }
            hash_object(&args[2]);
        }
        "cat-file" => {
            if args.len() != 3 {
                eprintln!("Usage: rgit cat-file <hash>");
                std::process::exit(1);
            }
            println!("{}", cat_file(&args[2]));
        }
        "update-index" => {
            if args.len() != 4 {
                eprintln!("Usage: rgit update-index <file_name> <blob_hash>");
                std::process::exit(1);
            }
            update_index(&args[2], &args[3]);
        }
        "write-tree" => write_tree(),
        "commit-tree" => {
            if args.len() < 5 || args.len() > 6 {
                println!("{}", args.len());
                eprintln!("Usage: rgit commit-tree <commit_name> <author> <tree_hash> [parent_hash]");
                std::process::exit(1);
            }

            let commit_name = &args[2];
            let author = &args[3];
            let tree_hash = &args[4];

            let parent = if args.len() == 6 && args[5].to_lowercase() != "none" {
                Some(&args[5])
            } else {
                None
            };

            commit_tree(commit_name, author, tree_hash, parent.map(|x| x.as_str()));
        }
        "checkout" => {
            if args.len() != 3 {
                eprintln!("Usage: rgit checkout <commit_hash>");
                std::process::exit(1);
            }
            checkout(&args[2]);
        }
        _ => eprintln!("Unknown command: {}", args[1]),
    }
}
