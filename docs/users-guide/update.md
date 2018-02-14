---
id = "update"
title = "Updating your API docs"
---
# Updating your API docs

The `doxidize update` command refreshes the API reference portion of your markdown docs with any
changes to your source code. This allows you to keep your API docs up to date.

To do so:

```shell
$ doxidize update
```

--------------------------------

When you invoke `doxidize update`, here's what happens:

First, it checks to make sure the `docs/api` directory in your Markdown docs exists. If it's not
there, that's a sign that the project needs to be initialized first.

Then it makes a list of any existing Markdown files that are already in that directory.

Next, it analyzes your source code the same way as when initializing your project, with a `cargo
check` command with some special flags. With this analysis, it can construct new API docs in the
`docs/api` directory. This includes any docs written in documentation comments.

After that, it cleans up any files and folders that aren't part of the new set of documentation.
This way it keeps the `docs/api` folder clean if you move or remove anything from the public API.

And that's it! Your API docs are up to date. Running `doxidize build` after this will render the new
Markdown into your final docs.
