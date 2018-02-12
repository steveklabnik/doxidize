---
id = "serve"
title = "Previewing your docs"
---
# Previewing your docs

The `doxidize serve` command takes the markdown files you've generated
with `doxidize build` and serve them with a local web server, so that
you can see what they will look like once published.

To do so:

```shell
$ doxidize serve
```

--------------------------------

When you invoke `doxidize serve`, here's what happens:

First, it will spin up a background thread to watch the contents of
your `docs` directory. If anything changes, it will invoke `doxidize build`
in the background, so a refresh will give you nice new docs!

Second, it will start a web server, serving the contents of `target/docs`.
If you have set a `base-url`, it will be respected.

Then, it will print the URL it's serving to the terminal, so you can
copy/paste it into a web browser and view your docs!
