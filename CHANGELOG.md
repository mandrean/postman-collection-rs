# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

## [0.3.1](https://github.com/mandrean/postman-collection-rs/compare/v0.3.0...v0.3.1) - 2026-03-22

### Added

- gate YAML support behind opt-in feature ([#19](https://github.com/mandrean/postman-collection-rs/pull/19))

### Other

- close coverage audit gaps ([#17](https://github.com/mandrean/postman-collection-rs/pull/17))

## [0.3.0](https://github.com/mandrean/postman-collection-rs/compare/v0.2.0...v0.3.0) - 2026-03-22

### Added

- improve coverage across supported postman specs
- complete remaining v2.1.0 schema coverage ([#11](https://github.com/mandrean/postman-collection-rs/pull/11))
- Add apikey auth with fixture coverage ([#10](https://github.com/mandrean/postman-collection-rs/pull/10))

### Fixed

- prefer explicit schema over v1 heuristics

### Other

- trim release-plz token usage
- pin dependencies to latest versions
- modernize CI, releases, and parser
- Update readme instructions ([#4](https://github.com/mandrean/postman-collection-rs/pull/4))

### Breaking changes

- detect Postman Collection versions exactly across `v1.0.0`, `v2.0.0`, and `v2.1.0` instead of relying on first-match enum deserialization
- preserve `v1` `helperAttributes` objects losslessly
- model `v2` `item` and `item-group` branches explicitly

### Fixes

- audit live fixture coverage and preserve previously missing fields such as `v1` header `enabled` and `v2` URL-encoded parameter `type`
- add regression fixtures and tests for version detection, helper attribute coverage, item-group parsing, and unsupported schema handling
