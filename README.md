# rgit

Recreating git from scratch.

<br>

## Usage 

```
$ ./rgit --help
Usage: rgit <command> [<args>]

Options:
  -h, --help                         Show this help message and exit.
  --version                          Show the rgit version and exit.

Commands:
  init                               Initialize a new rgit repository.
  hash-object <file>                 Compute and store the hash of a file.
  cat-file <hash>                    Display the contents of an object.
  index --add <file> <blob_hash>      Add a file to the index.
  index --modify <file> <blob_hash>   Modify an entry in the index.
  index --remove <file>               Remove a file from the index.
  write-tree                         Write the current index to a tree object.
  commit-tree <message> <author> <tree_hash> [parent_hash]  Create a commit object.
  checkout <commit_hash|branch>       Checkout a specific commit or branch.
  log <commit_hash|branch>            Show the log starting from the given commit or branch.
  update-ref <ref_name> <commit_hash> Update a reference to a commit hash.
  symbolic-ref <ref_name> <target_ref> Set a symbolic reference.
  push <remote_path> <branch>         Push local changes to a remote repository.
  fetch <remote_path> <branch>        Fetch changes from a remote repository.
  get-head-hash                       Display the commit hash pointed to by HEAD.
  add <file_name>                     Add a file to the staging area.
  remove <file_name>                  Remove a file from the index.
  commit <commit_message> <author>    Commit the staged changes.
```

<br>

## Example

```
$ mkdir -p test-workspace
$ cd test-workspace
$ mkdir -p repo1
$ cd repo1

# initialize the repository
$ ../../rgit init

# create a first commit
$ echo "First commit: Hello, World!" > file1.txt
$ ../../rgit add file1.txt
$ ../../rgit commit "First commit" "eztaah"

# create another commit
$ echo "Second commit: Bye" > file1.txt
$ echo "Second commit: Hola" > file2.txt
$ ../../rgit add file1.txt
$ ../../rgit add file2.txt
$ ../../rgit commit "Second commit" "eztaah"

# navigate to the repo
$ cd test-workspace/repo1 

# check the commit history
$ ../../rgit log HEAD
Commit: ef7a8ed89593e0d327fcc91e62200073985a30d4
Author: eztaah
Message: Second commit

Commit: 5d1454489a0b1e3b9d02ee345ea54512de1fe3c1
Author: eztaah
Message: First commit

# tag the first commit 
$ ../../rgit update-ref refs/v0.1 5d1454489a0b1e3b9d02ee345ea54512de1fe3c1

# checkout on v0.1
$ ../../rgit checkout v0.1

# check the commit history 
$ ../../rgit log HEAD
Commit: 5d1454489a0b1e3b9d02ee345ea54512de1fe3c1
Author: eztaah
Message: First commit
```

