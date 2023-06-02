#!/bin/bash

set -e -u

for dir in day??; do
    ( cd "$dir" && cargo run)
done

