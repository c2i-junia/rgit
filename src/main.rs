mod commands;
mod utils;

use crate::utils::RepoPath;
use commands::cat_file::cat_file;
use commands::checkout::checkout;
use commands::commit::*;
use commands::commit_tree::commit_tree;
use commands::fetch::*;
use commands::get_head_hash::*;
use commands::hash_object::hash_object;
use commands::init::init;
use commands::log::*;
use commands::push::*;
use commands::symbolic_ref::*;
use commands::update_index::*;
use commands::update_ref::*;
use commands::write_tree::write_tree;
use std::env;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

fn print_usage() {
    println!("Usage: rgit <command> [<args>]");
    println!();
    println!("Options:");
    println!("  -h, --help                         Show this help message and exit.");
    println!("  --version                          Show the rgit version and exit.");
    println!();
    println!("Commands:");
    println!("  init                               Initialize a new rgit repository.");
    println!("  hash-object <file>                 Compute and store the hash of a file.");
    println!("  cat-file <hash>                    Display the contents of an object.");
    println!("  index --add <file> <blob_hash>      Add a file to the index.");
    println!("  index --modify <file> <blob_hash>   Modify an entry in the index.");
    println!("  index --remove <file>               Remove a file from the index.");
    println!("  write-tree                         Write the current index to a tree object.");
    println!("  commit-tree <message> <author> <tree_hash> [parent_hash]  Create a commit object.");
    println!("  checkout <commit_hash|branch>       Checkout a specific commit or branch.");
    println!("  log <commit_hash|branch>            Show the log starting from the given commit or branch.");
    println!("  update-ref <ref_name> <commit_hash> Update a reference to a commit hash.");
    println!("  symbolic-ref <ref_name> <target_ref> Set a symbolic reference.");
    println!("  push <remote_path> <branch>         Push local changes to a remote repository.");
    println!("  fetch <remote_path> <branch>        Fetch changes from a remote repository.");
    println!("  get-head-hash                       Display the commit hash pointed to by HEAD.");
    println!("  add <file_name>                     Add a file to the staging area.");
    println!("  remove <file_name>                  Remove a file from the index.");
    println!("  commit <commit_message> <author>    Commit the staged changes.");
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(0);
    }

    match args[1].as_str() {
        "--help" | "-h" => {
            print_usage();
        }
        "--version" => {
            println!("rgit v0.0");
        }
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

            let operation: &str = args[2].as_str();
            let file_name: &str = &args[3];

            match operation {
                "--add" => {
                    if args.len() != 5 {
                        eprintln!("Usage: rgit index --add <file_name> <blob_hash>");
                        std::process::exit(1);
                    }
                    let blob_hash: &str = &args[4];
                    add_index(file_name, blob_hash);
                }
                "--modify" => {
                    if args.len() != 5 {
                        eprintln!("Usage: rgit index --modify <file_name> <blob_hash>");
                        std::process::exit(1);
                    }
                    let blob_hash: &str = &args[4];
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
        "write-tree" => {
            let tree_hash: String = write_tree();
            println!("{}", tree_hash);
        }
        "commit-tree" => {
            if args.len() < 5 || args.len() > 6 {
                println!("{}", args.len());
                eprintln!(
                    "Usage: rgit commit-tree <commit_message> <author> <tree_hash> [parent_hash]"
                );
                std::process::exit(1);
            }

            let commit_name: &str = &args[2];
            let author: &str = &args[3];
            let tree_hash: &str = &args[4];

            let parent: Option<&str> = if args.len() == 6 && args[5].to_lowercase() != "none" {
                Some(&args[5])
            } else {
                None
            };

            let commit_hash: String = commit_tree(
                commit_name,
                author,
                tree_hash.to_string(),
                parent.map(|x| x),
            );
            println!("{}", commit_hash);
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
            log(&args[2]);
        }
        "update-ref" => {
            if args.len() != 4 {
                eprintln!("Usage: rgit update-ref <ref_name> <commit_hash>");
                std::process::exit(1);
            }
            let ref_name: &str = &args[2];
            let commit_hash: &str = &args[3];
            update_ref(ref_name, commit_hash);
        }
        "symbolic-ref" => {
            if args.len() != 4 {
                eprintln!("Usage: rgit symbolic-ref <ref_name> <target_ref>");
                std::process::exit(1);
            }
            let ref_name: &str = &args[2];
            let target_ref: &str = &args[3];
            symbolic_ref(ref_name, target_ref);
        }
        "push" => {
            if args.len() != 4 {
                eprintln!("Usage: rgit push <remote_path> <branch>");
                std::process::exit(1);
            }
            let remote_path: &str = &args[2];
            let branch: &str = &args[3];
            push(remote_path, branch);
        }
        "fetch" => {
            if args.len() != 4 {
                eprintln!("Usage: rgit fetch <remote_path> <branch>");
                std::process::exit(1);
            }
            let remote_path: &str = &args[2];
            let branch: &str = &args[3];
            fetch(remote_path, branch);
        }
        "get-head-hash" => {
            if args.len() != 2 {
                eprintln!("Usage: rgit get-head-hash");
                std::process::exit(1);
            }
            println!("{}", get_head_hash());
        }
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: rgit add <file_name>");
                std::process::exit(1);
            }

            let exe_path: PathBuf = env::current_exe().expect("Failed to get path of executable");
            let mut script_path: PathBuf = exe_path.parent().unwrap().to_path_buf();
            script_path.push("src/commands/add.sh");

            let status: ExitStatus = Command::new(script_path)
                .args(&args[2..])
                .status()
                .expect("Failed to execute add.sh");

            if !status.success() {
                eprintln!("Error: add.sh failed");
                std::process::exit(1);
            }
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Usage: rgit remove <file_name>");
                std::process::exit(1);
            }

            let exe_path: PathBuf = env::current_exe().expect("Failed to get path of executable");
            let mut script_path: PathBuf = exe_path.parent().unwrap().to_path_buf();
            script_path.push("src/commands/remove.sh");

            let status: ExitStatus = Command::new(script_path)
                .args(&args[2..])
                .status()
                .expect("Failed to execute remove.sh");

            if !status.success() {
                eprintln!("Error: remove.sh failed");
                std::process::exit(1);
            }
        }
        "commit" => {
            if args.len() != 4 {
                println!("{}", args.len());
                eprintln!("Usage: rgit commit <commit_message> <author>");
                std::process::exit(1);
            }

            let commit_message: &str = &args[2];
            let author: &str = &args[3];

            commit(commit_message, author);
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
