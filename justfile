#!/usr/bin/env just --justfile

# display dbang help
help:
  cargo run --package dbang --bin dbang -- --help

# run hello@linux-china from run subcommand
run:
  cargo run --package dbang --bin dbang -- run hello@linux-china world

# run hello@linux-china from command line directly
run2:
  cargo run --package dbang --bin dbang -- hello@linux-china

# build with release and copy dbang to ~/bin
build:
  cargo build --release
  cp target/release/dbang ~/bin/
