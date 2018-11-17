postman-collection-rs
=====================
[Postman Collection][postman_collection] serialization & deserialization library, written in Rust.

[![Build Status](https://travis-ci.org/mandrean/postman-collection-rs.svg?branch=master)](https://travis-ci.org/mandrean/postman-collection-rs)
[![Latest version](https://img.shields.io/crates/v/postman_collection.svg)](https://crates.io/crates/postman_collection)
[![Documentation](https://docs.rs/postman_collection/badge.svg)](https://docs.rs/postman_collection)
![License](https://img.shields.io/crates/l/postman_collection.svg)

Install
-------
Add the following to your `Cargo.toml` file:

```toml
[dependencies]
postman_collection = "0.1"
```

Use
---
```rust
extern crate postman_collection;

fn main() {
  match postman_collection::from_path("path/to/postman-collection.json") {
    Ok(spec) => println!("spec: {:?}", spec),
    Err(err) => println!("error: {}", err)
  }
}
```

See [examples/printer.rs](examples/printer.rs) for more.

Contribute
----------
This project follows [semver], [conventional commits] and semantic releasing using [semantic-rs].

Note
----
Inspired by [softprops/openapi](https://github.com/softprops/openapi).

[postman_collection]: https://www.getpostman.com/collection
[semver]: https://semver.org/
[conventional commits]: https://www.conventionalcommits.org
[semantic-rs]: https://github.com/semantic-rs/semantic-rs
