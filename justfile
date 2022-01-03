#!/usr/bin/env just --justfile

# display dbang help
help:
  cargo run --package dbang --bin dbang -- --help

# Catalog list
catalog-list:
  cargo run --package dbang --bin dbang -- catalog list

# Catalog info
catalog-info:
  cargo run --package dbang --bin dbang -- catalog show linux-china

# Catalog add
catalog-add:
  cargo run --package dbang --bin dbang -- catalog add linux-china

# Deno list
deno-list:
  cargo run --package dbang --bin dbang -- deno list

# List apps
apps:
  cargo run --package dbang --bin dbang -- apps

# run hello@linux-china from run subcommand
hello:
  cargo run --package dbang --bin dbang -- run hello@linux-china world

# run hello@linux-china from run subcommand
hello-verbose:
  cargo run --package dbang --bin dbang -- --verbose run hello@linux-china

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

# display dbang help
hello-shim:
  cargo build --package dbang --bin dbang-shim
  unlink ./target/hello
  ln -s {{justfile_directory()}}/target/debug/dbang-shim ./target/hello
  ./target/hello
