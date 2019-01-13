#!/bin/bash

DIR="$(dirname "$0")"

cargo build
cp -r "$DIR/resources" "$DIR/target/debug"