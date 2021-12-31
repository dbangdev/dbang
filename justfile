#!/usr/bin/env just --justfile

# display dbang help
help:
  cargo run --package dbang --bin dbang -- --help

# run hello@linux-china
run:
  cargo run --package dbang --bin dbang -- run hello@linux-china world
