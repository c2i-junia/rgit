#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RGIT="$SCRIPT_DIR/../../rgit"

tree_hash=$($RGIT write-tree)
parent_hash=$($RGIT get-head-hash)

# if HEAD is valid, create a commit with parent
if [ -n "$parent_hash" ]; then
    commit_hash=$($RGIT commit-tree "$1" "$2" "$tree_hash" "$parent_hash")
else
    commit_hash=$($RGIT commit-tree "$1" "$2" "$tree_hash")
fi

# update HEAD to point to the new commit
$RGIT update-ref HEAD "$commit_hash"
