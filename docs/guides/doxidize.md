# `doxidize`

When run with no subcommand, `doxidize` will initialize your project for use with Doxidize.

```shell
$ doxidize
```

It takes no parameters.

## Generated files

`doxidize` will create a directory at the top-level of your project, `docs`.
It will fill it with these files:

* `README.md`: the home page of your documentation
* An `api` directory
* A markdown file inside of the `api` directory for each item in your crate.
  These files will be pre-filled with any doc comments you've written in your
  source.