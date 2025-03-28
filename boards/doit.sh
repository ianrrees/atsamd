#!/bin/bash

set +e

for b in *; do
    [ "$b" == "$(basename "$0")" ] && continue
    for e in "$b"/examples/*.rs; do
        newname="$b"-$(basename "$e")
        cp "$e" ../examples/"$newname"
    done
done