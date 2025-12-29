# ci_group

[![Crates.io](https://img.shields.io/crates/v/ci_group.svg)](https://crates.io/crates/ci_group)
[![Docs.rs](https://docs.rs/ci_group/badge.svg)](https://docs.rs/ci_group)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

RAII log groups for GitHub Actions and Azure Pipelines.

Groups close automatically when dropped, even on panic. No more lost CI logs.

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

## Local development

No output outside CI. To preview locally:

```bash
GITHUB_ACTIONS=true cargo run
```

## License

MIT OR Apache-2.0
