#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <depth> <fen>"
    exit 1
fi

depth=$1
fen=$2

./test "$depth" "$fen"

