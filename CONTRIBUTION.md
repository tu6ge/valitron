# Contribution Guide

1. if you using vscode and rust-analysis, you should be open `full` feature, like:

```
{
  "rust-analyzer.cargo.features": [
    "full"
  ],
}
```

or format on save.
```
{
  "rust-analyzer.cargo.features": [
    "full"
  ],
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "rust-lang.rust-analyzer"
}
```

2. one thing once commit

so in once pull request, it can contain multiple commits.

3. if you add new features, you should be add some test case of the features.

4. run `cargo fmt`

5. run test case:

```
cargo t --lib

// lib, doc, examples, integration
cargo t --features="full"
```

## Thanks