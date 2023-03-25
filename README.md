# Updating packages

- Update the package list in WORKSPACE.
- Run any command depending on cargo packages with `CARGO_BAZEL_REPIN=true`, e.g.

```bash
CARGO_BAZEL_REPIN=true bazel test //src:rust_tests_test1
```

# Testing

```bash
bazel test //src:rust_tests
```
