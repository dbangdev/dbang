DBang
============

DBang is a CLI to manage Deno scripts on GitHub

# Vocabulary

* Catalog: A collection of scripts that is dang-catalog.json
* Script: A script that is in dang-script.json, also called artifact.
* repo_name: GitHub's repo name, eg. `nbang-catalog`, `my-script`
* repo_full_name: repo name with GitHub's account or organization name, for example `linux-china/nbang-catalog`

# Features

* Deno versions management
* Install Apps
* Aliases & Catalogs
* Trust and Permissions

# Security strategies

* Prompt for permissions confirm on first run
* Prompt for permissions confirm after `dbang-catalog.json` changed, like Android app
* Trust user's scripts by `dbang trust <user>`
* Private repo support by DENO_AUTH_TOKENS

# References

* JBang: [jbang.dev](https://jbang.dev)
