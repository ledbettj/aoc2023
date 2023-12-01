#!/usr/bin/env bash
# e.g. ./watch.sh day1
cargo watch -x "test --release -- --nocapture $@"
