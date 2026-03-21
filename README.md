postman-collection-rs
=====================
[Postman Collection][postman_collection] serialization & deserialization library, written in Rust.

[![CI](https://github.com/mandrean/postman-collection-rs/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/mandrean/postman-collection-rs/actions/workflows/ci.yml)
[![Latest version](https://img.shields.io/crates/v/postman_collection.svg)](https://crates.io/crates/postman_collection)
[![Documentation](https://docs.rs/postman_collection/badge.svg)](https://docs.rs/postman_collection)
![License](https://img.shields.io/crates/l/postman_collection.svg)

Install
-------
Add the following to your `Cargo.toml` file:

```toml
[dependencies]
postman_collection = "0.3"
```

Use
---
```rust
fn main() -> postman_collection::Result<()> {
    let collection = postman_collection::from_path("path/to/postman-collection.json")?;
    println!(
        "Found {:?} collection named {}",
        collection.version(),
        collection.name()
    );
    Ok(())
}
```

See [examples/printer.rs](examples/printer.rs) for more.

Contribute
----------
This project follows [semver], [conventional commits], and uses GitHub Actions plus
[release-plz] for CI and releases. The release workflow expects a
`CARGO_REGISTRY_TOKEN` repository secret and GitHub Actions permissions that allow release PRs.

Note
----
Inspired by [softprops/openapi](https://github.com/softprops/openapi).

[postman_collection]: https://www.getpostman.com/collection
[semver]: https://semver.org/
[conventional commits]: https://www.conventionalcommits.org
[release-plz]: https://release-plz.dev/docs/github/quickstart
