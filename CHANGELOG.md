TBD: aki-json-pick
===
Unreleased changes. Release notes have not yet been written.

0.1.8 (2022-06-18)
=====

* changes to edition 2021
* update depends: flood-tide(0.2.5)
* update depends: memx(0.1.21), memx-cdy(0.1.8), runnel(0.3.11)
* update depends: exec-target(v0.2.6), flood-tide-gen(0.1.16)
* update depends: rust-version-info-file(v0.1.6)
* update depends: semver(1.0.10)
* update depends: jql(4.0.4)

0.1.7 (2022-05-22)
=====

* update depends: jql(4.0.3)

0.1.6 (2022-05-22)
=====

* update depends: runnel(0.3.10)
* update depends: anyhow(1.0.57), libc(0.2.126), jql(3.3.0), serde_json(1.0.81)
* update depends: exec-target(v0.2.5), rust-version-info-file(v0.1.5)

0.1.5 (2021-11-15)
=====

* update depends: jql(3.0.4)
* minimum support rustc 1.54.0 (a178d0322 2021-07-26)

0.1.4 (2021-11-15)
=====

* minimum support rustc 1.47.0 (18bf6b4f0 2020-10-07)
* add more documents
* update depends: flood-tide(0.2.4), memx(0.1.18), memx-cdy(0.1.7), runnel(0.3.9)
* update depends: anyhow(1.0.45), libc(0.2.107)
* update depends: exec-target(v0.2.4), flood-tide-gen(0.1.15), rust-version-info-file(v0.1.3)

0.1.3 (2021-09-11)
=====

* pass cargo clippy
* update depends: anyhow(1.0.43), flood-tide-gen(0.1.14), flood-tide(0.2.3), memx-cdy(0.1.6), runnel(0.3.8)
* rewite TARGET_EXE_PATH with `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`
* update depends: exec-target(0.2.3)

0.1.2 (2021-06-24)
=====

* add `memx_cdy::memx_init(); // fast mem operation.`
* rewite TARGET_EXE_PATH with `env!("CARGO_BIN_EXE_aki-json-pick")`
* bug fix: `#[cfg(feature = "debian_build")]`

0.1.1 (2021-06-03)
=====

* add support features = \["debian_build"\]
* bug fix command option: -X rust-version-info
* update depends: flood-tide(0.2.2)
* update depends: regex(1.5.4)

0.1.0 (2021-04-28)
=====
first commit
