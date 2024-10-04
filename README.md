# rgit

Recreating git from scratch.

<br>

## Example

```
# initialize a new rgit repository
$ mkdir test-rgit
$ cd test-rgit
$ ../target/debug/rgit init
Initialized empty rgit repository in .rgit

# create and hash a new file
$ echo "First commit: Hello, World!" > file1.txt
$ ../target/debug/rgit hash-object file1.txt
992c7b7ce84c66ff44a5a71af422e8f32b533faf

# add the file to the index
$ ../target/debug/rgit update-index file1.txt 992c7b7ce84c66ff44a5a71af422e8f32b533faf
Updated index with file: file1.txt

# write the current index to a tree object
$ ../target/debug/rgit write-tree
4dc7c1badedb359d41eb5e2c136998d42935c91e

# create the first commit (no parent)
$ ../target/debug/rgit commit-tree "Initial commit" "eztaah" 4dc7c1badedb359d41eb5e2c136998d42935c91e none
9fa611741454bc43740caf8caa5890fb5bd37b88
Index has been cleared.

# modify the file and add a new file
$ echo "Second commit: Modified content" > file1.txt
$ echo "This is a new file called file2.txt" > file2.txt

# hash the new files
$ ../target/debug/rgit hash-object file1.txt
269053dd2a6033fe2b2f5a47efbf8342136b3794
$ ../target/debug/rgit hash-object file2.txt
3dabd70d81126c5f08ef63213d3fce5fb1f7d6e6

# update the index with the modified and new files
$ ../target/debug/rgit update-index file1.txt 269053dd2a6033fe2b2f5a47efbf8342136b3794
Updated index with file: file1.txt
$ ../target/debug/rgit update-index file2.txt 3dabd70d81126c5f08ef63213d3fce5fb1f7d6e6
Updated index with file: file2.txt

# write the new index to a tree object
$ ../target/debug/rgit write-tree
5425b6571ae918f2d5f254a425d49200ce5c839b

# create a second commit with the previous commit as its parent
$ ../target/debug/rgit commit-tree "Second commit" "eztaah" 5425b6571ae918f2d5f254a425d49200ce5c839b 9fa611741454bc43740caf8caa5890fb5bd37b88
736fbe931620e21cae8969a5e63dfdec8c49ab1a
Index has been cleared.

# checkout the initial commit
$ ../target/debug/rgit checkout 9fa611741454bc43740caf8caa5890fb5bd37b88
Cleared working directory.
Restoring tree for commit: 9fa611741454bc43740caf8caa5890fb5bd37b88
Restored file: file1.txt

# checkout the second commit
$ ../target/debug/rgit checkout 736fbe931620e21cae8969a5e63dfdec8c49ab1a
Cleared working directory.
Restoring tree for commit: 9fa611741454bc43740caf8caa5890fb5bd37b88
Restored file: file1.txt
Restoring tree for commit: 736fbe931620e21cae8969a5e63dfdec8c49ab1a
Restored file: file1.txt
Restored file: file2.txt

# checkout back to the initial commit
$ ../target/debug/rgit checkout 9fa611741454bc43740caf8caa5890fb5bd37b88
Cleared working directory.
Restoring tree for commit: 9fa611741454bc43740caf8caa5890fb5bd37b88
Restored file: file1.txt
