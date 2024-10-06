#!/bin/sh

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RGIT="$SCRIPT_DIR/../../rgit"

hash1=$($RGIT hash-object "$1")
$RGIT index --add "$1" "$hash1"

