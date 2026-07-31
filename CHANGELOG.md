# Changelog

## Unreleased

## 28.0.0

- Update `mysql` dependency to `28`.

## 27.0.0

- Update `mysql` dependency to `27`.
- Upgrade to Edition 2024.
- Minimum supported Rust version (MSRV) is now 1.88 to align with `mysql` dependency.

## 26.0.0

- Update `mysql` dependency to `26`.
- Minimum supported Rust version (MSRV) is now 1.81 to align with `mysql` dependency.

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
