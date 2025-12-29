# ci_group

RAII log groups for GitHub Actions.

## Install

```toml
[dependencies]
ci_group = "0.1"
```

## Usage

```rust
let _g = ci_group::open("Build");
build(); // group closes automatically, even on panic
```

Or use the macro:

```rust
ci_group::group!("Build", {
    build()?;
});
```

## Status

Work in progress. Currently only supports GitHub Actions.

