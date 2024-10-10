# rgit Command Documentation

This document provides a detailed explanation of each command available in `rgit`. The commands are separated into two categories: high-level and low-level. Each command is described in bullet points to show its internal behavior and logic.

---

## High-level Commands
High-level commands are user-facing commands that perform common version control operations. They abstract away the details of manipulating the repository’s internal structure and are meant for day-to-day use.

### `init`
- Creates a `.rgit` directory in the current directory to store repository data.
- Initializes necessary subdirectories and files:
  - `.rgit/objects` to store objects (blobs, trees, commits).
  - `.rgit/refs` to store references (branches).
  - `.rgit/index` to track the staging area.
  - `.rgit/HEAD` file to point to the current branch or commit.

### `log <commit_hash|branch>`
- Reads the commit history starting from the specified commit or branch.
- Traverses the parent commits recursively and displays:
  - Commit hash
  - Author
  - Commit message
- Continues until it reaches the root commit (no parent).

### `commit <commit_message> <author>`
- Reads the staged files from the `.rgit/index` file.
- Creates a new tree object representing the current state of the staging area.
- Creates a commit object that includes:
  - A reference to the newly created tree.
  - The commit message.
  - The author name and email.
  - A parent commit reference (if there is one).
- Updates the `HEAD` file to point to the new commit.

### `add <file_name>`
- Hashes the file specified by `file_name` using the `hash-object` command.
- Stores the file’s hash in the `.rgit/objects` directory.
- Updates the `.rgit/index` to include the new file and its hash, marking it as staged for the next commit.

### `remove <file_name>`
- Removes the specified file from the staging area by modifying the `.rgit/index` file.
- Does not remove the file from the working directory, only from the index.

### `checkout <commit_hash|branch>`
- Resolves the specified commit hash or branch name.
- Updates the working directory to reflect the state of the files in the specified commit.
- If a branch name is provided, updates the `HEAD` to point to this branch.
- If a commit hash is provided, switches to a detached `HEAD` state.

### `push <remote_path> <branch>`
- Gathers all objects in the local repository that are missing from the remote repository.
- Transfers the missing objects to the remote using `scp` or a similar mechanism.
- Updates the remote branch reference to point to the latest commit hash.

### `fetch <remote_path> <branch>`
- Reads the remote branch reference to determine the latest commit hash.
- Checks for missing objects in the local repository.
- Copies the missing objects from the remote repository to the local `.rgit/objects` directory.
- Updates the local branch reference to point to the fetched commit.

---

## Low-level Commands
Low-level commands manipulate the internal structure of the repository. They are used to directly interact with the underlying objects and references without high-level abstractions.

### `hash-object <file>`
- Reads the content of the specified file.
- Computes the SHA-1 hash of the file content.
- Stores the file content in the `.rgit/objects` directory using the SHA-1 hash as its identifier.
- Outputs the hash of the file.

### `cat-file <hash>`
- Reads an object from the `.rgit/objects` directory using the provided hash.
- Decompresses the object file.
- Prints the contents of the object (e.g., commit, tree, or blob) to the console.

### `index --add <file> <blob_hash>`
- Adds an entry to the `.rgit/index` file.
- Associates the given file name with the specified blob hash.

### `index --modify <file> <blob_hash>`
- Updates an existing entry in the `.rgit/index` file.
- Modifies the blob hash associated with the specified file name.

### `index --remove <file>`
- Removes an entry from the `.rgit/index` file.
- The file is no longer tracked in the staging area.

### `write-tree`
- Reads the current state of the `.rgit/index` file.
- Creates a tree object representing the current directory structure.
- Each file in the index is represented as a blob in the tree.
- Outputs the SHA-1 hash of the newly created tree object.

### `commit-tree <message> <author> <tree_hash> [parent_hash]`
- Creates a new commit object.
- Links it to the specified `tree_hash` and optionally to the `parent_hash`.
- Stores the commit object in the `.rgit/objects` directory.
- Outputs the commit hash.

### `update-ref <ref_name> <commit_hash>`
- Creates or updates a reference in `.rgit/refs/`.
- Sets the specified `ref_name` to point to the given `commit_hash`.
- If the reference already exists, it is overwritten.

### `symbolic-ref <ref_name> <target_ref>`
- Creates or updates a symbolic reference in `.rgit/`.
- Sets the specified `ref_name` to point to another reference (`target_ref`).
- Typically used to update `HEAD` to point to a branch like `refs/heads/main`.

### `get-head-hash`
- Reads the `.rgit/HEAD` file.
- If `HEAD` points to a branch, resolves the branch to a commit hash.
- Outputs the commit hash pointed to by `HEAD`.
