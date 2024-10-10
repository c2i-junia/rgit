## rgit commands

### high-level commands

- `init`
    - Creates a `.rgit` directory in the current directory
    - Initializes necessary subdirectories and files:
      - `.rgit/objects` to store objects (blobs, trees, commits).
      - `.rgit/refs` to store references (branches).
      - `.rgit/index` to track the staging area.
      - `.rgit/HEAD` file to point to the current branch or commit.

- `log <commit_hash|branch>`
    - Reads the commit history starting from the specified commit hash or reference (HEAD, refs).
    - Continues until it reaches the root commit (no parent).

- `commit <commit_message> <author>`
    - Write the current index to a tree object with `write-tree`
    - get the current HEAD hash or reference
    - commit with `commit-tree`
    - update branch (if HEAD point to branch) or HEAD refs (if detached HEAD) to point to the new commit

- `add <file_name>`
    - Hashes the file specified using the `hash-object` command.
    - Stores the file’s hash in the `.rgit/objects` directory.
    - Updates the `.rgit/index` to include the new file and its hash.

- `remove <file_name>`
    - Removes the specified file from the staging area by modifying the `.rgit/index` file.
    - Does not remove the file from the working directory, only from the index.

- `checkout <commit_hash|branch>`
    - Get the tree_hash from the commit hash
    - Clear all files in the working directory
    - Restore the tree with `restore_tree()`
    - Update HEAD

- `push <remote_path> <branch>`
    - Gather all necessary objects (starts from the current commit hash and go recursively (commit, trees and blobs))
    - Get all missing objects in the remote repo
    - Transfers the missing objects to the remote with `scp`.
    - Updates the remote branch reference to point to the latest commit hash.

- `fetch <remote_path> <branch>`
    - Gather all necessary objects (starts from the current commit hash and go recursively (commit, trees and blobs))
    - Get all missing objects in the local repo
    - Transfers the missing objects to the remote with `scp`.
    - Updates the local reference to point to the latest commit hash.

<br>

### low-level commands

- `hash-object <file>`
    - Computes the SHA-1 hash of the file content.
    - Stores the file content in the `.rgit/objects` directory using the SHA-1 hash as its identifier.
    - Outputs the hash of the file.

- `cat-file <hash>`
    - Reads an object from the `.rgit/objects` directory using the provided hash.
    - Decompresses the object file.
    - Prints the contents of the object (e.g., commit, tree, or blob) to the console.

- `index --add <file> <blob_hash>`
    - Adds an entry to the `.rgit/index` file.
    - Associates the given file name with the specified blob hash.

- `index --modify <file> <blob_hash>`
    - Updates an existing entry in the `.rgit/index` file.
    - Modifies the blob hash associated with the specified file name.

- `index --remove <file>`
    - Removes an entry from the `.rgit/index` file.
    - The file in itself is not removed

- `write-tree`
    - Reads the current state of the `.rgit/index` file.
    - Creates a tree object with all the files in the index file.
    - Outputs the SHA-1 hash of the newly created tree object.

- `commit-tree <message> <author> <tree_hash> [parent_hash]`
    - Creates a new commit object with `message`, `author`, `tree_hash` and `parent_hash`.
    - Outputs the commit hash.

- `update-ref <ref_name> <commit_hash>`
    - Creates or updates a reference in `.rgit/`.
    - Sets the specified `ref_name` to point to the given `commit_hash`.
    - If the reference already exists, it is overwritten.
    - You need to write the path relative to `.rgit/` directory

- `symbolic-ref <ref_name> <target_ref>`
    - Creates or updates a symbolic reference in `.rgit/`.
    - Sets the specified `ref_name` to point to another reference (`target_ref`).

- `get-head-hash`
    - Reads the `.rgit/HEAD` file.
    - If `HEAD` points to a branch, resolves the branch to a commit hash.
    - Outputs the commit hash pointed to by `HEAD`.
