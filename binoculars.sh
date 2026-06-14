#!/usr/bin/env bash
# The purpose of this script is to find the highest prime order `r` with embedding degree `k`
# for an elliptic curve that I brute-forced — to the end that I can find a complere r-torsion
# in the extension field, so I can have a non-degenerate pairing for computing groth16 by hand :)
#
#
# cargo run > result.txt && ./binoculars.sh result.txt

awk '
function reset_block() {
    block = ""
    max_key = -1
    found = 0
}

function commit_block() {
    if (found) {
        if (max_key > best_key) {
            best_key = max_key
            best_block = block
        }
    }
}

BEGIN {
    best_key = -1
    reset_block()
}

{
    # blank line = end of block
    if ($0 ~ /^ *$/) {
        commit_block()
        reset_block()
        next
    }

    # accumulate full block text
    block = block $0 "\n"

    # match: <number>: Some(2)
    if (match($0, /([0-9]+): Some\(2\)/, m)) {
        found = 1
        key = m[1] + 0
        if (key > max_key) {
            max_key = key
        }
    }
}

END {
    commit_block()
    printf "%s", best_block
}
' "$1"
