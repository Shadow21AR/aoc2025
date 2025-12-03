#!/bin/bash
set -e

day=$(printf "day%02d" "$1")

# Create the new binary crate
cargo new --bin "$day"
touch "$day/input.txt"
touch "$day/example.txt"

# Rebuild workspace members list
{
    echo "[workspace]"
    echo "resolver = \"3\""
    echo "members = ["
    for d in day*/; do
        echo "    \"${d%/}\","
    done
    echo "]"
} > Cargo.toml