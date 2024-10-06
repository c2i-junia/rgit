mod commands;
mod utils;

use crate::utils::RepoPath;
use commands::cat_file::cat_file;
use commands::checkout::checkout;
use commands::commit_tree::commit_tree;
use commands::fetch::*;
use commands::hash_object::hash_object;
use commands::init::init;
use commands::log::*;
use commands::push::*;
use commands::update_index::*;
use commands::update_ref::*;
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
            println!("{}", cat_file(&RepoPath::Local, &args[2]));
        }
        "index" => {
            if args.len() < 4 {
                eprintln!("Usage: rgit index <operation> <file_name> [<blob_hash>]");
                std::process::exit(1);
            }

            let operation = args[2].as_str();
            let file_name = &args[3];

            match operation {
                "--add" => {
                    if args.len() != 5 {
                        eprintln!("Usage: rgit index --add <file_name> <blob_hash>");
                        std::process::exit(1);
                    }
                    let blob_hash = &args[4];
                    add_index(file_name, blob_hash);
                }
                "--modify" => {
                    if args.len() != 5 {
                        eprintln!("Usage: rgit index --modify <file_name> <blob_hash>");
                        std::process::exit(1);
                    }
                    let blob_hash = &args[4];
                    update_index(file_name, blob_hash);
                }
                "--remove" => {
                    if args.len() != 4 {
                        eprintln!("Usage: rgit index --remove <file_name>");
                        std::process::exit(1);
                    }
                    remove_index(file_name);
                }
                _ => {
                    eprintln!("Unknown index operation: {}", operation);
                    eprintln!("Supported operations: --add, --modify, --remove");
                    std::process::exit(1);
                }
            }
        }
        "write-tree" => write_tree(),
        "commit-tree" => {
            if args.len() < 5 || args.len() > 6 {
                println!("{}", args.len());
                eprintln!(
                    "Usage: rgit commit-tree <commit_name> <author> <tree_hash> [parent_hash]"
                );
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
        "log" => {
            if args.len() != 3 {
                eprintln!("Usage: rgit log <commit_hash>");
                std::process::exit(1);
            }
            log(&args[2]); // Appel de la fonction `log` avec le hash de commit.
        }
        "update-ref" => {
            if args.len() != 4 {
                eprintln!("Usage: rgit update-ref <ref_name> <commit_hash>");
                std::process::exit(1);
            }
            let ref_name = &args[2];
            let commit_hash = &args[3];
            update_ref(ref_name, commit_hash);
        }
        "push" => {
            if args.len() != 4 {
                eprintln!("Usage: rgit push <remote_path> <branch>");
                std::process::exit(1);
            }
            let remote_path = &args[2];
            let branch = &args[3];
            push(remote_path, branch);
        }
        "fetch" => {
            if args.len() != 4 {
                eprintln!("Usage: rgit fetch <remote_path> <branch>");
                std::process::exit(1);
            }
            let remote_path = &args[2];
            let branch = &args[3];
            fetch(remote_path, branch);
        }
        _ => eprintln!("Unknown command: {}", args[1]),
    }
}

// fn main() {
//     // handle arguments
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 2 {
//         eprintln!("usage: rgit <command> [<args>]");
//         std::process::exit(1);
//     }
//
//     match args[1].as_str() {
//         "init" => init(&args[2..]),
//         "hash-object" => hash_object(&args[2..]),
//         "cat-file" => cat_file(&args[2..]),
//         "update-index" => update_index(&args[2..]),
//         "write-tree" => write_tree(&args[2..]),
//         "commit-tree" => commit_tree(&args[2..]),
//         "checkout" => checkout(&args[2..]),
//         _ => eprintln!("unknown command: {}", args[1]),
//     }
// }
