---
id = "build"
title = "Building your docs"
---
# Building your docs

The `doxidize build` command takes the markdown files you've written in
`docs/` and produces web-based documentation inside of `target/docs`.

To do so:

```shell
$ doxidize build
```

--------------------------------

When you invoke `doxidize build`, here's what happens:

First, it makes sure that you have a top-level `docs` directory at all! If you
don't, it will produce an error.

Next, it creates the output directory inside of `target/docs`. If you've configured
Doxidize to have a base url, this will be taken into account and generate a
subdirectory.

After that, it will scan the `docs` directory, processing all of the metadata at
the top of each file. It then uses this metadata plus the contents of your
`Menu.toml` to generate the sidebar.

Finally, it will go through each file, generating the appropriate `.html` file
that it would correspond to.