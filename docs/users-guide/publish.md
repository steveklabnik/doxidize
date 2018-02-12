---
id = "publish"
title = "Publishing your docs to GitHub Pages"
---
# Publishing your docs to GitHub Pages

The `doxidize publish` command takes the markdown files you've generated
with `doxidize build` and publish them to GitHub pages.

To do so:

```shell
$ doxidize publish
```

--------------------------------

When you invoke `doxidize publish`, here's what happens:

First, it will load up `Doxidize.toml` to see if you've set a `base-url`. It
needs that to do its job properly! Especially with GitHub Pages.

Then, it will create a new `git` repository inside of the generated docs,
`git add` all of the files, then `git commit` them.

Finally, it will push that commit to a `gh-pages` branch inside of your
main git repository, and then finally `git push origin gh-pages`.

Your docs should now be live!
