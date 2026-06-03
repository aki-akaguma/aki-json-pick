# Changelog: aki-json-pick
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2026-05-20

### Changed
- Update crates: `flood-tide` (0.2.14) and `flood-tide-gen` (0.2.2)
- Migrate to Rust 2024 edition
- Raise minimum supported Rust version (MSRV) to 1.85.0
- Update `runnel` crate to 0.4.2
- Refactor error handling in `run.rs`, `parse.rs`, and `build.rs` to replace `unwrap()` with robust error management

### Fixed
- Address clippy lint: `uninlined-format-args`

### Removed
- Drop `memx-cdy` dependency

## [0.2.0] - 2025-09-15

### Added
- Introduce `specs` directory for project specifications

### Changed
- Enable `IntoIterator` compatibility for arguments in `execute()`
- Update `runnel` crate to 0.4.0
- Update `rust-version-info-file` crate to 0.2
- Update `jql` crate to 5.2.0
- Set Rust version requirement to 1.80.0
- Refactor `run.rs`

### Fixed
- Correct minimum supported version in documentation

### Removed
- Drop `base_dir=` parameter from `-X` options

## [0.1.10] - 2024-06-20

### Added
- Incorporate GitHub Actions workflows for Ubuntu, macOS, and Windows
- Include test status badges in `README.tpl`
- Enable Miri support for tests
- Support Tarpaulin in `Makefile`

### Changed
- Rename `config` to `config.toml`
- Eliminate `cfg(has_not_matches)`
- Refactor `Makefile`
- Update dependencies: `flood-tide` (0.2.11), `flood-tide-gen` (0.1.22), `memx-cdy` (0.1.13), `runnel` (0.3.19), `exec-target` (v0.2.8), `indoc` (2.0.0), `jql` (5.2.0), and `colored_json` (3.2.0)

### Fixed
- Correct license files: `LICENSE-APACHE` and `LICENSE-MIT`
- Address clippy lints: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`, `uninlined_format_args`, `unused_imports`, and `derivable_impls`
- Resolve bug where `jql` used default features unintentionally
- Bump Rust version requirement from 1.56.0 to 1.63.0

### Removed
- Delete `COPYING` file

## [0.1.9] - 2023-01-11

### Added
- Include badges in `README.tpl`
- Specify Rust version requirement (1.56.0) in `Cargo.toml`

### Changed
- Reformat `CHANGELOG.md`
- Update dependencies: `anyhow` (1.0.68), `flood-tide` (0.2.8), `flood-tide-gen` (0.1.19), `memx-cdy` (0.1.10), `runnel` (0.3.15), `jql` (5.1.4), `colored_json` (3.0.1), and `serde_json` (1.0.91)

### Fixed
- Address clippy lints: `Eq` implementation for `PartialEq` derivations and `uninlined_format_args`

## [0.1.8] - 2022-06-18

### Changed
- Migrate to Rust 2021 edition
- Update dependencies: `flood-tide` (0.2.5), `memx` (0.1.21), `memx-cdy` (0.1.8), `runnel` (0.3.11), `exec-target` (v0.2.6), `flood-tide-gen` (0.1.16), `rust-version-info-file` (v0.1.6), `semver` (1.0.10), and `jql` (4.0.4)

## [0.1.7] - 2022-05-22

### Changed
- Update `jql` dependency to 4.0.3

## [0.1.6] - 2022-05-22

### Changed
- Update dependencies: `runnel` (0.3.10), `anyhow` (1.0.57), `libc` (0.2.126), `jql` (3.3.0), `serde_json` (1.0.81), `exec-target` (v0.2.5), and `rust-version-info-file` (v0.1.5)

## [0.1.5] - 2021-11-15

### Changed
- Update `jql` dependency to 3.0.4
- Raise minimum supported Rust version to 1.54.0

## [0.1.4] - 2021-11-15

### Added
- Improve documentation

### Changed
- Raise minimum supported Rust version to 1.47.0
- Update dependencies: `flood-tide` (0.2.4), `memx` (0.1.18), `memx-cdy` (0.1.7), `runnel` (0.3.9), `anyhow` (1.0.45), `libc` (0.2.107), `exec-target` (v0.2.4), `flood-tide-gen` (0.1.15), and `rust-version-info-file` (v0.1.3)

## [0.1.3] - 2021-09-11

### Changed
- Address `cargo clippy` warnings
- Update dependencies: `anyhow` (1.0.43), `flood-tide-gen` (0.1.14), `flood-tide` (0.2.3), `memx-cdy` (0.1.6), `runnel` (0.3.8), and `exec-target` (0.2.3)
- Use `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))` for `TARGET_EXE_PATH`

## [0.1.2] - 2021-06-24

### Added
- Integrate `memx_cdy::memx_init()` for optimized memory operations

### Changed
- Use `env!("CARGO_BIN_EXE_aki-json-pick")` for `TARGET_EXE_PATH`

### Fixed
- Address build issue with `#[cfg(feature = "debian_build")]`

## [0.1.1] - 2021-06-03

### Added
- Enable support for `debian_build` feature

### Changed
- Update dependencies: `flood-tide` (0.2.2) and `regex` (1.5.4)

### Fixed
- Resolve bug in `-X rust-version-info` command option

## [0.1.0] - 2021-04-28

### Added
- Perform initial release

[Unreleased]: https://github.com/aki-akaguma/aki-json-pick/compare/v0.2.1..HEAD
[0.2.1]: https://github.com/aki-akaguma/aki-json-pick/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/aki-json-pick/compare/v0.1.10..v0.2.0
[0.1.10]: https://github.com/aki-akaguma/aki-json-pick/compare/v0.1.9..v0.1.10
[0.1.9]: https://github.com/aki-akaguma/aki-json-pick/compare/v0.1.8..v0.1.9
[0.1.8]: https://github.com/aki-akaguma/aki-json-pick/compare/v0.1.7..v0.1.8
[0.1.7]: https://github.com/aki-akaguma/aki-json-pick/compare/v0.1.6..v0.1.7
[0.1.6]: https://github.com/aki-akaguma/aki-json-pick/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/aki-json-pick/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/aki-json-pick/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/aki-json-pick/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/aki-json-pick/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/aki-json-pick/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/aki-json-pick/releases/tag/v0.1.0
