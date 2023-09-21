# Contribution Guide

1. If you using vscode and rust-analysis, you should be open `full` feature, like:

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

2. One thing once commit

so in once pull request, it can contain multiple commits.

3. If you add new features, you should be add some test case of the features.

4. Run `cargo fmt` with your need.

5. Run test case:

```
cargo t --lib

// lib, doc, examples, integration
cargo t --features="full"
```

## Thanks