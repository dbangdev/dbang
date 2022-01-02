#!/usr/bin/env just --justfile

# display dbang help
help:
  cargo run --package dbang --bin dbang -- --help

# run hello@linux-china from run subcommand
hello:
  cargo run --package dbang --bin dbang -- run hello@linux-china world

# run hello@linux-china/demo from run subcommand
hello-demo:
  cargo run --package dbang --bin dbang -- run hello@linux-china/demo world

# run myip@linux-china from run subcommand
myip:
  cargo run --package dbang --bin dbang -- run myip@linux-china

# run hello@linux-china from command line directly
run2:
  cargo run --package dbang --bin dbang -- hello@linux-china

# build with release and copy dbang to ~/bin
build:
  cargo build --release
  cp target/release/dbang ~/bin/
