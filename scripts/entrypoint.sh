#!/bin/bash

if [ "$1" = "production" ]; then
    cargo run --release
else
    cargo watch -x run
fi