# Basic Usage

Here's the basic workflow to get going with Doxidize:

## Initialization

First, initialize Doxidize for your project:

```shell
$ doxidize
```

This will create a top-level `docs` dir and a `README.md` file inside of it.

Edit the `README.md` to your heart's content.

## Generate your docs

To generate HTML documentation:

```shell
$ doxidize generate
```

Your generated docs will appear in `target/docs`. `README.md` will be
transformed into `index.html`, so that on GitHub, you can see it rendered,
but when you put your docs on the web, you can also see them rendered.

## Viewing your docs

To view your docs in a web browser:

```shell
$ doxidize serve
```

And then open `http://127.0.0.1:7878` in your browser. Doxidize does not need
any special kind of server to run; but absolute paths will not work if you
view via `file://`.

## Publishing to GitHub Pages

To publish your docs to GitHub Pages:

```shell
$ doxidize publish
```

This will create your rendered docs in a local `gh-pages` branch, and then
push that branch to `origin`.