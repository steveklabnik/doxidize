---
id = "flags"
title = "Command-line flags"
---
# Command-line flags

While each subcommand can take its own flags, some flags apply to all subcommands,
and are therefore passed before the subcommand.

## `--manifest-path`

By default, Doxidize assumes that you're running it in the same directory as
a `Cargo.toml` describing your project. If this file isn't in the same directory
that you invoke Doxidize with, you may pass this flag to indicate its location.
For example:

```shell
$ doxidize --manifest-path=../foo/Cargo.toml build
```

This would generate the documentation for some project one directory above
where you're running `doxidize`.

This option is quite useful when working on Doxidize itself; though there,
you'll usually be using `cargo run` instead:

```shell
$ cargo run -- --manifest-path=../test-project/Cargo.toml build
```
