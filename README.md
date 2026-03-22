postman-collection-rs
=====================
[Postman Collection][postman_collection] serialization and deserialization library, written in Rust.

[![CI](https://github.com/mandrean/postman-collection-rs/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/mandrean/postman-collection-rs/actions/workflows/ci.yml)
[![Latest version](https://img.shields.io/crates/v/postman_collection.svg)](https://crates.io/crates/postman_collection)
[![Documentation](https://docs.rs/postman_collection/badge.svg)](https://docs.rs/postman_collection)
![License](https://img.shields.io/crates/l/postman_collection.svg)

Overview
--------
`postman_collection` provides typed Rust models for working with Postman Collection files.
It can deserialize collections from JSON or YAML, identify the supported collection version,
and serialize the typed model back to JSON or YAML.

Current highlights:

- typed support for Postman Collection `v1.0.0`, `v2.0.0`, and `v2.1.0`
- version-specific models exposed as `v1_0_0`, `v2_0_0`, and `v2_1_0`
- strict version detection through the top-level `PostmanCollection` enum
- convenient parsing helpers: `from_path`, `from_reader`, `from_str`, and `from_slice`
- serialization helpers: `to_json` and `to_yaml`
- regression coverage for version dispatch, round-tripping, and representative schema branches

Supported Versions
------------------
This crate currently supports Postman Collection:

- `v1.0.0`
- `v2.0.0`
- `v2.1.0`

Version detection follows the collection format:

- `v1.0.0` is detected from the legacy root collection shape.
- `v2.0.0` and `v2.1.0` are detected from `info.schema`.

Collections that look like `v2` but omit `info.schema`, or point at an unsupported schema
version, fail to deserialize instead of being guessed into the wrong version module.

Install
-------
Add the following to your `Cargo.toml` file:

```toml
[dependencies]
postman_collection = "0.2"
```

Usage
-----
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

If you need version-specific fields, match on `PostmanCollection` directly:

```rust
use postman_collection::{from_path, PostmanCollection};

fn main() -> postman_collection::Result<()> {
    match from_path("path/to/postman-collection.json")? {
        PostmanCollection::V1_0_0(spec) => println!("v1 collection: {}", spec.name),
        PostmanCollection::V2_0_0(spec) => println!("v2.0.0 collection: {}", spec.info.name),
        PostmanCollection::V2_1_0(spec) => println!("v2.1.0 collection: {}", spec.info.name),
    }

    Ok(())
}
```

See [examples/printer.rs](examples/printer.rs) for a complete runnable example.

Current State
-------------
The crate currently focuses on:

- accurate deserialization into version-specific Rust types
- stable serialization back to JSON and YAML
- explicit handling of collection version selection
- preserving representative schema branches exercised by the bundled fixtures and regression tests

Examples of supported modeled areas include:

- collection metadata and variables
- folders and nested items
- requests, headers, bodies, and URLs
- responses and response metadata
- auth helpers across the supported versions

Contribute
----------
This project follows [semver], [conventional commits], and uses GitHub Actions plus
[release-plz] for CI and releases.

Note
----
Inspired by [softprops/openapi](https://github.com/softprops/openapi).

[postman_collection]: https://www.getpostman.com/collection
[semver]: https://semver.org/
[conventional commits]: https://www.conventionalcommits.org
[release-plz]: https://release-plz.dev/docs/github/quickstart
