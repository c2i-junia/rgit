#!/bin/sh

mkdir -p test-workspace
cd test-workspace
mkdir -p repo1
cd repo1

# create a first commit
echo "First commit: Hello, World!" > file1.txt
../../rgit add file1.txt
../../rgit commit "First commit" "eztaah"

# create another commit
echo "Second commit: Bye" > file1.txt
echo "Second commit: Hola" > file2.txt
../../rgit add file1.txt
../../rgit add file2.txt
../../rgit commit "Second commit" "eztaah"
