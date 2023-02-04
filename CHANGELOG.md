# Changelog: aki-json-pick

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] *
### Added
* `.github/workflows/test-ubuntu.yml`
* `.github/workflows/test-macos.yml`
* `.github/workflows/test-windows.yml`
* test status badges into `README.tpl`

### Changed
* update depends: flood-tide(0.2.9)
* update depends: memx-cdy(0.1.11), runnel(0.3.16)
* update depends: exec-target(v0.2.8), flood-tide-gen(0.1.16)
* update depends: jql(5.1.6)

### Fixed
* license files
* clippy: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`
* bug: `jql` was default-features
* rust-version: "1.56.0" to "1.58.0"


## [0.1.9] (2023-01-11)
### Added
* badges into `README.tpl`
* rust-version = "1.56.0" into Cargo.toml

### Changed
* reformat `CHANGELOG.md`
* update depends: anyhow(1.0.68)
* update depends: flood-tide(0.2.8), flood-tide-gen(0.1.19)
* update depends: memx-cdy(0.1.10), runnel(0.3.15)
* update depends: jql(5.1.4), colored_json(3.0.1), serde_json(1.0.91)

### Fixed
* clippy: you are deriving `PartialEq` and can implement `Eq`
* clippy: uninlined_format_args

## [0.1.8] (2022-06-18)
### Changed
* changes to edition 2021
* update depends: flood-tide(0.2.5)
* update depends: memx(0.1.21), memx-cdy(0.1.8), runnel(0.3.11)
* update depends: exec-target(v0.2.6), flood-tide-gen(0.1.16)
* update depends: rust-version-info-file(v0.1.6)
* update depends: semver(1.0.10)
* update depends: jql(4.0.4)

## [0.1.7] (2022-05-22)
### Changed
* update depends: jql(4.0.3)

## [0.1.6] (2022-05-22)
### Changed
* update depends: runnel(0.3.10)
* update depends: anyhow(1.0.57), libc(0.2.126), jql(3.3.0), serde_json(1.0.81)
* update depends: exec-target(v0.2.5), rust-version-info-file(v0.1.5)

## [0.1.5] (2021-11-15)
### Changed
* update depends: jql(3.0.4)
* minimum support rustc 1.54.0 (a178d0322 2021-07-26)

## [0.1.4] (2021-11-15)
### Added
* more documents

### Changed
* minimum support rustc 1.47.0 (18bf6b4f0 2020-10-07)
* update depends: flood-tide(0.2.4), memx(0.1.18), memx-cdy(0.1.7), runnel(0.3.9)
* update depends: anyhow(1.0.45), libc(0.2.107)
* update depends: exec-target(v0.2.4), flood-tide-gen(0.1.15), rust-version-info-file(v0.1.3)

## [0.1.3] (2021-09-11)
### Changed
* pass cargo clippy
* update depends: anyhow(1.0.43), flood-tide-gen(0.1.14), flood-tide(0.2.3), memx-cdy(0.1.6), runnel(0.3.8)
* rewite TARGET_EXE_PATH with `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`
* update depends: exec-target(0.2.3)

## [0.1.2] (2021-06-24)
### Added
* `memx_cdy::memx_init(); // fast mem operation.`

### Changed
* rewite TARGET_EXE_PATH with `env!("CARGO_BIN_EXE_aki-json-pick")`

### Fixed
* bug: `#[cfg(feature = "debian_build")]`

## [0.1.1] (2021-06-03)
### Added
* support `features = \["debian_build"\]`

### Changed
* update depends: flood-tide(0.2.2)
* update depends: regex(1.5.4)

### Fixed
* bug: command option: `-X rust-version-info`

## [0.1.0] (2021-04-28)
* first commit

[Unreleased]: https://github.com/aki-akaguma/aki-json-pick/compare/v0.1.9..HEAD
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
