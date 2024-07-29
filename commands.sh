#!/bin/bash

# Check if at least one argument (command name) is provided
if [ $# -lt 1 ]; then
    echo "Usage: $0 <command-name> [arguments...]"
    exit 1
fi

# Capture the command name
command_name=$1

# Shift the positional parameters to get the arguments
shift

# Run the cargo command with the provided command name and arguments
cargo run -- "$command_name" "$@"
