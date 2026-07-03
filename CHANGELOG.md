# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.3.0](https://github.com/JinyangWang27/lunar-lite-rs/compare/v1.2.1...v1.3.0) - 2026-07-03

### Added

- enhance date validation and normalization functions

### Other

- Refactor error taxonomy ([#27](https://github.com/JinyangWang27/lunar-lite-rs/pull/27))
- clarify lunar date ordering behavior in comments
- add self-standing invariant tests for lunar-lite functionality
- update comments and clarify behavior

## [1.2.1](https://github.com/JinyangWang27/lunar-lite-rs/compare/v1.2.0...v1.2.1) - 2026-06-29

### Other

- remove tests from crate inclusion ([#23](https://github.com/JinyangWang27/lunar-lite-rs/pull/23))

## [1.2.0](https://github.com/JinyangWang27/lunar-lite-rs/compare/v1.1.0...v1.2.0) - 2026-06-29

### Added

- expose exact LiChun datetime ([#21](https://github.com/JinyangWang27/lunar-lite-rs/pull/21))

## [1.1.0](https://github.com/JinyangWang27/lunar-lite-rs/compare/v1.0.0...v1.1.0) - 2026-06-28

### Added

- use tyme-compatible astronomical conversion ([#19](https://github.com/JinyangWang27/lunar-lite-rs/pull/19))
- add lunar month helpers ([#14](https://github.com/JinyangWang27/lunar-lite-rs/pull/14))

### Other

- add broad tyme4rs oracle compatibility suite ([#20](https://github.com/JinyangWang27/lunar-lite-rs/pull/20))

### Breaking

- Replaced the generated lunar-year and solar-term conversion tables with a
  tyme-compatible astronomical backend. Conversion results may differ from the
  previous `lunar-typescript` table-derived backend.
- Solar date validation now follows tyme-compatible Julian/Gregorian reform
  semantics: solar years `1..=9999`, Julian-calendar semantics before
  `1582-10-15`, and the invalid Gregorian reform gap
  `1582-10-05..=1582-10-14`.
- Full lunar-to-solar conversion now requires the resulting solar date to fall
  within solar years `1..=9999`. Lunar-month fact APIs still accept lunar years
  `-1..=9999`.

### Changed

- Replaced generated lunar-year and solar-term conversion tables with a small
  internal astronomical backend for new-moon and solar-term calculation, with
  tyme4rs-compatible calendar behaviour. Portions of the kernel are adapted from
  MIT-licensed `6tail/tyme4rs`; see `THIRD_PARTY_LICENSES.md`.
- Conversion support now follows tyme-compatible policy: solar years `1..=9999`,
  Julian-calendar semantics before `1582-10-15`, and the invalid Gregorian
  reform gap `1582-10-05..=1582-10-14`.
- The astronomical kernel's raw data now lives as reviewable source files under
  `data/astronomical/` and is compiled into typed constants by `build.rs` at
  build time. No runtime file I/O or parsing is added, public API and numerical
  behaviour are unchanged, and the data files are shipped in the crate package so
  the build works from crates.io.

### Fixed

- Four-pillar exact-mode instants are built on the Julian Day so user dates and
  Jie solar-term boundaries share one Julian/Gregorian calendar policy; pre-reform
  Julian leap days (e.g. `1500-02-29`) no longer collapse onto the next day.
- Lunar-month facts (`leap_month`, `lunar_month_days`) now resolve for the
  lowest supported lunar year `-1` instead of failing on an out-of-range
  previous-year lookup. Full lunar-to-solar conversion still requires the
  resulting solar date to fall in `1..=9999`.

### Added

- `THIRD_PARTY_LICENSES.md` reproducing the upstream MIT notice for the adapted
  astronomical kernel, included in crate packaging.

## [1.0.0](https://github.com/JinyangWang27/lunar-lite-rs/compare/v0.3.2...v1.0.0) - 2026-06-12

### Other

- expand design overview

## [0.3.2](https://github.com/JinyangWang27/lunar-lite-rs/compare/v0.3.1...v0.3.2) - 2026-06-12

### Fixed

- add missing badges for Crates.io version and downloads in README.md ([#10](https://github.com/JinyangWang27/lunar-lite-rs/pull/10))

## [0.3.1](https://github.com/JinyangWang27/lunar-lite-rs/compare/v0.3.0...v0.3.1) - 2026-06-12

### Fixed

- correct crates.io badge URL for downloads ([#9](https://github.com/JinyangWang27/lunar-lite-rs/pull/9))
