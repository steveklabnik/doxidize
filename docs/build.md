# Building

This is what happens when you run

```shell
$ doxidize build
```

When this command is run, it will inspect your `docs` directory, and
generate rendered HTML documentation inside of the `target/docs` directory.

## README.md

`README.md` will be rendered as `index.html`.

## Other Markdown Files

Any other file ending in `.md` will be rendered inside of `target/docs`,
with a `.html` extension.
