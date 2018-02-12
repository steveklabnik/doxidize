---
id = "clean"
title = "Cleaning up your generated documentation"
---
# Cleaning up your generated documentation

The `doxidize clean` command removes all of the generated documentation
from `target/docs`.

To do so:

```shell
$ doxidize clean
```

--------------------------------

When you invoke `doxidize clean`, here's what happens:

It deletes the `target/docs` directory. While this seems straightforward,
note that there's [at least one bug](https://github.com/steveklabnik/doxidize/issues/61)
in the current implementation.
