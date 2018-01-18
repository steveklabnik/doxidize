# Doxidize

Excellent documentation for Rust.

## Installation

To get Doxidize:

```shell
$ cargo install doxidize
```

## Workflow

First, initialize Doxidize for your project:

```shell
$ doxidize
```

This will create a top-level `docs` dir and a `README.md` file inside of it.

Edit the `README.md` to your heart's content.

To generate HTML documentation:

```shell
$ doxidize generate
```

Your generated docs will appear in `target/docs`. `README.md` will be
transformed into `index.html`, so that on GitHub, you can see it rendered,
but when you put your docs on the web, you can also see them rendered.

To publish your docs to GitHub Pages:

```shell
$ doxidize publish
```

This will create your rendered docs in a local `gh-pages` branch, and then
push that branch to `origin`.