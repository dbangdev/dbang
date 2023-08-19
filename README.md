DBang
============

DBang is a CLI to manage Deno scripts on GitHub.

# Vocabulary

* Catalog: A collection of scripts that are saved in `dang-catalog.json` file.
* Script: A script that is in `dbang-catalog.json`, also called artifact.
* repo_name: GitHub's repo name, eg. `dbang-catalog`, `my-script`
* repo_full_name: repo name with GitHub's account or organization name, for example `linux-china/dbang-catalog`

# Features

* Deno versions management
* Install Apps
* Aliases & Catalogs
* Trust and Permissions
* Unstable
* Default arguments

# File format for dang-catalog.json

`dbang-catalog.json` is the file to describe scripts, example as following:

```json
{
  "scripts": {
    "hello": {
      "script-ref": "hello.ts",
      "description": "Hello world"
    },
    "http-server": {
      "script-ref": "simple-http-server.js",
      "description": "Simple HTTP Server on http://localhost:8000",
      "permissions": [
        "--allow-net=localhost:8000"
      ]
    },
    "cowsay": {
      "deno": "1.36.1",
      "script-ref": "npm:cowsay@1.5.0/cowthink",
      "args": [
        "Hello World"
      ],
      "description": "cowsay by npm",
      "unstable": true,
      "permissions": [
        "--allow-read"
      ]
    }
  }
}
```

Script elements explanation:

- `deno`: Deno version, default is latest version
- `script-ref`: Script reference, it can be a local file, a URL or a npm package
- `args`: Default arguments for the script
- `description`: Script description
- `unstable`: Use unstable Deno version
- `permissions`: Permissions for the script
- `import-map`: Import map for the script, it can be a local file, a URL.
- `v8-flags`: v8 flags for the script, for example `--experimental-wasm-typed-funcref,--experimental-wasm-gc`.
- `platform`: Platform name for the script. Format is `{os}-{arch}`, os: `macos`, `linux`, `windows`, and arch: `x86_64`, `aarch64`, `arm`.

# Security strategies

* Prompt for permissions confirm on first run
* Prompt for permissions confirm after `dbang-catalog.json` changed, like Android app
* Trust user's scripts by `dbang trust <user>`
* Private repo support by DENO_AUTH_TOKENS

# Web Storage Support

DBang use `--config ~/.dbang/catalogs/github/linux-china/dbang-catalog.json` to separate different catalogs, and it
means scripts in same dbang-catalog.json would share the same storage.

# References

* JBang: [jbang.dev](https://jbang.dev)
* Update-informer: https://github.com/mgrachev/update-informer
