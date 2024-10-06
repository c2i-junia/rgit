#!/bin/sh

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RGIT="$SCRIPT_DIR/../../rgit"

$RGIT index --remove "$1"
