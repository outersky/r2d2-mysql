# Changelog

## Unreleased

## 25.0.0

- Update `mysql` dependency to `25`.
- Minimum supported Rust version (MSRV) is now 1.70 to align with `mysql` dependency.

## 24.0.0

- Update `mysql` dependency to `24`.
- Remove deprecated `MysqlConnectionManager` type alias.
- Minimum supported Rust version (MSRV) is now 1.65 due to transitive dependencies.

## 23.0.0

- Update `mysql` dependency to `23`.
- Rename `MysqlConnectionManager` to `MySqlConnectionManager`. The old name remains available under a deprecated type alias.
- Hide `pool` module.
