#!/usr/bin/env bash

FILENAME="c_$1_$2.svg"
echo "$1 $2" | python3 equivalence-class-explorer.py | dot -Tsvg > $FILENAME
firefox $FILENAME
