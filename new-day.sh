#!/usr/bin/env bash
YEAR=2023

if [ $# -ne 1 ]; then
    echo "usage: $0 <day>"
    exit 1
fi

DAY="$1"
SRC="src/day$DAY.rs"

if [ -f ".session-cookie" ]; then
    curl -b session=$(cat .session-cookie) \
         "https://adventofcode.com/$YEAR/day/$DAY/input" > "inputs/day$DAY.txt"

    echo "Fetched input!"
else
    "No .session-cookie found, skipped fetching input"
fi

if [ ! -f "$SRC" ]; then
    cat .template | sed s/DAY/$DAY/g > "$SRC"
    echo "created $SRC"
else
    echo "$SRC already exists"
fi

grep "mod day$DAY;" src/lib.rs -q
EXISTING=$?

if [ $EXISTING -eq 0 ]; then
    echo "module definition already found"
else
    echo "#[allow(dead_code)]" >> src/lib.rs
    echo "mod day$DAY;" >> src/lib.rs
    echo "added module definition"
fi
