# Basic Usage

Here's the basic workflow to get going with Doxidize:

## Initialize your project

First, initialize Doxidize for your project:

```shell
$ doxidize
```

This will create a top-level `docs` dir and a `README.md` file inside of it.

Edit the `README.md` to your heart's content.

For more about the `doxidize` command, [see its guide](../guides/doxidize.html).

## Generate your documentation

To generate HTML documentation:

```shell
$ doxidize generate
```

Your generated docs will appear in `target/docs`.

For more about the `doxidize generate` command, [see its guide](../guides/doxidize-generate.html).

## Viewing your documentation

To view your docs in a web browser:

```shell
$ doxidize serve
```

For more about the `doxidize serve` command, [see its guide](../guides/doxidize-serve.html).

## Publishing to GitHub Pages

To publish your docs to GitHub Pages:

```shell
$ doxidize publish
```

For more about the `doxidize publish` command, [see its guide](../guides/doxidize-publish.html).