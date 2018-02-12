---
id = "doxidize-toml"
title = "Doxidize.toml"
---
# Doxidize.toml

This page describes the format of `Doxidize.toml`, a top-level configuration
file for customozing how Doxidize generates your documentation.

At the moment, there is only one setting, and it is optional. By default,
your `Doxidize.toml` will be empty.

## `base-url`

By default, links will be generated relative to the root, that is, `/`, in
the web version of your documentation. By setting the `base-url`, you can
instead have your documentation relative to whatever path you set.

```
[docs]
base-url = "foo"
```

Now, all links will be rooted by `/foo` instead of `/`.

This option is most useful when deploying to GitHub Pages with `doxidize publish`;
their URLs are relative to a path that's the same name as your project.