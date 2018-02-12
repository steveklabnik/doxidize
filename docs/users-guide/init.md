---
id = "init"
title = "Initializing a project"
---
# Initializing a project

The `doxidize init` command sets up your project for use with Doxidize.

To do so:

```shell
$ doxidize init
```

--------------------------------

When you invoke `doxidize init`, here's what happens:

First, it makes sure that you haven't already initialized a project;
that would be an error!

Next, it creates the top-level `docs/` directory, as well as a
`docs/README.md` file.

Then, it creates a `Doxidize.toml` at the root of your project.
This will be empty.

Next, it will copy each of your examples over into the `docs/examples/`
directory.

Once that's done, it knows enough info to build an initial `Menu.toml`,
containing the files it's generated so far.

Finally, it will analyze your source code. To do this, it invokes `cargo check`
with some special flags. It will then load up the result of the compilation,
and use that to generate API docs inside of the `docs/api` directory. It'll
include any docs that you have previously written in documentation comments.

Your project is ready to be documented!
