# Publishing your documentation

After you've run `doxidize build` to build your docs, run this:

```shell
$ doxidize publish
```

This will create a local branch, `gh-pages`, containing your generated docs.
It will then push it to the `origin` remote. Your docs should be live on
GitHub Pages!
