#!/usr/bin/env bash
npm run tauri dev
if [ $? -ne 0 ]; then
    echo "Error running Rust program."
    exit 1
fi
display "./testimg.png"
cargo clean

