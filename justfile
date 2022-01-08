#!/usr/bin/env just --justfile

DBANG_VERSION := "0.1.5"

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
  cp target/release/dbang ~/.dbang/bin/
  cp target/release/dbang-shim ~/.dbang/bin/

x64-tar:
  cargo build --release
  rm -rf target/x86_64
  mkdir -p target/x86_64/dbang-{{DBANG_VERSION}}/bin
  cp target/release/dbang target/x86_64/dbang-{{DBANG_VERSION}}/bin
  cp target/release/dbang-shim target/x86_64/dbang-{{DBANG_VERSION}}/bin
  (cd target/x86_64 ; tar cf dbang-{{DBANG_VERSION}}-x86_64-apple-darwin.tar dbang-{{DBANG_VERSION}})
  shasum -a 256 target/x86_64/dbang-{{DBANG_VERSION}}-x86_64-apple-darwin.tar

arm-tar $MACOSX_DEPLOYMENT_TARGET=`xcrun -sdk macosx --show-sdk-platform-version` $SDKROOT=`xcrun --sdk macosx --show-sdk-path`:
  cargo build --release --target=aarch64-apple-darwin
  rm -rf target/aarch64
  mkdir -p target/aarch64/dbang-{{DBANG_VERSION}}/bin
  cp target/aarch64-apple-darwin/release/dbang target/aarch64/dbang-{{DBANG_VERSION}}/bin
  cp target/aarch64-apple-darwin/release/dbang-shim target/aarch64/dbang-{{DBANG_VERSION}}/bin
  (cd target/aarch64 ; tar cf dbang-{{DBANG_VERSION}}-aarch64-apple-darwin.tar dbang-{{DBANG_VERSION}})
  shasum -a 256 target/aarch64/dbang-{{DBANG_VERSION}}-aarch64-apple-darwin.tar
