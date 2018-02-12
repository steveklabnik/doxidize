---
id = "quickstart"
title = "Quickstart"
---
# Quickstart

To get started with Doxidize, install it from `crates.io`:

```shell
$ cargo install doxidize
```

## Initialization

Then, run the `init` command inside of the project you'd like
to document:

```shell
$ cd /path/to/my/project
$ doxidize init
```

This will create two things at the top-level of your project:

* `Doxidize.toml`, used for configuration
* `docs/`, where your documentation lives.

Let's look inside of `docs/`:

```shell
$ tree docs
docs
 └── api
     └── ...
 └── examples
     └── ...
 └── Menu.toml
 └── README.md
```

First, the `api` directory. Doxidize generates these files from your source
code, and even includes the documentation that you've written with documentation
comments. It also generates a few overview pages. This directory is generally
considered to be controlled by Doxidize itself, and so while you can edit these
files to write documentation, if you rename or move files around, it may get
confused. For more details here, consult the Users Guide.

Second, the `examples` directory contains a copy of each example program
you've included in `examples`. Feel free to annotate these examples however
you'd like, including splitting up their source code to interleave comments.

Next, we have `Menu.toml`. Let's take a look:

```toml
"Getting Started" = [
    "overview",
]

"Examples" = [
    # ...
]
```

This file allows you to control the sidebar of your documentation. Each section
becomes a drop-down, with the contents you've listed here. By default, we generate
two sections: named "Getting Started," with an overview, and one named "Examples",
containing your examples. Note that "API" is not listed here, as Doxidize handles
organizing that itself.

What is `overview` here, anyway? To answer that, let's look at our last file:
`README.md`

```text
---
id = "overview"
title = "Overview"
---
# Overview
```

This markdown file has a TOML heading, written between the `---`s. This is
the way you can communicate metadata about the file to Doxidize. The first
bit we've got here is called `id`, and corresponds to the name we use to
link to this page in `Menu.toml`. The second is the title, which controls how
it's displayed in the menu. After that, comes the body itself, which has nothing
but a header to start with.

At this point, feel free to edit up some pages, create new ones and put them
in `Menu.toml`, or whatever else! We're not going to get into those details in
this Quickstart, see the User's Guide for more details here.

## Building your docs

Now that we've looked at our Markdown, let's turn those into web docs!

```shell
$ doxidize build
```

This will write out the files inside of the `target/docs` directory. Let's
take a look!

## Previewing your docs in a browser

To view our docs, we could open them in a browser directly. Doxidize also
includes a basic web server as well:

```shell
$ doxidize serve
```

It will print out the URL you need to open in your browser. Load that up,
and you should see your docs!

To quit the server, type `control-c`.

## Publishing your docs to GitHub Pages

Doxidize provides an easy way to publish your docs to GitHub pages, but
before that, let's talk about URLs. By default, your docs are located at
the site's root, that is, `/`. But GitHub Pages is usually served in a
subdirectory. Consider Doxidize's own documentation:

    https://steveklabnik.github.io/doxidize/index.html

See that `doxidize`? We want any links in our docs to include this base.
To do that, add this to your `Doxidize.toml`:

```toml
[docs]
base-url = "doxidize"
```

Well, you'd want to name it the name of your project, not literally "doxidize"!

After doing this, re-build your docs:

```shell
$ doxidize build
```

And then you can publish them:

```shell
$ doxidize publish
```

This will create a git repo inside your generated docs, commit them to a `gh-pages`
branch, and push it up to GitHub.

## Learning More

This is a very brief overview of Doxidize; to learn more about these steps in
detail, please consult the User's Guide.
