# function `strip_leading_space`

```
fn (s: &str) -> String
```

this removes the first space from each line of its input.

this is useful because doc comments generally look like this:

```text
/// some words
```

see that space before `some`? it's technically part of the comment,
so the RLS will give it to us in the docs.