#!/bin/bash

if [ "$1" = "production" ]; then
    echo "doing nothing..."
else
    echo "installing cargo-watch..."
    cargo install cargo-watch
fi