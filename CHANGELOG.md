# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

### Breaking changes

- detect Postman Collection versions exactly across `v1.0.0`, `v2.0.0`, and `v2.1.0` instead of relying on first-match enum deserialization
- preserve `v1` `helperAttributes` objects losslessly
- model `v2` `item` and `item-group` branches explicitly

### Fixes

- audit live fixture coverage and preserve previously missing fields such as `v1` header `enabled` and `v2` URL-encoded parameter `type`
- add regression fixtures and tests for version detection, helper attribute coverage, item-group parsing, and unsupported schema handling
