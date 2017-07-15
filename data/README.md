# Example Data

This directory holds files that will be installed.

Test cases should copy `data/foo.txt` directly, and copy the directory
`data/data/` recursively.

The end result should have the following layout in the target location:

```text
foo.txt
bar.txt
baz/
  quux.txt
```

This file should not be moved at all. Installing `data/` is an error.
