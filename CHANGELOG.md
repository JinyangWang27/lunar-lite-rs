# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-06-11

### Added

- Sexagenary (干支, ganzhi) stem-branch cycle support:
  - `StemBranch` — a validated Heavenly Stem / Earthly Branch pair, with
    `try_new`, `from_cycle_index`, `from_lunar_year` (anchored at `1984 = 甲子`),
    `cycle_index`, `stem`, and `branch`.
  - `HeavenlyStem` and `EarthlyBranch` enums with `index`, `from_index`, and a
    wrapping `offset`.
  - `HEAVENLY_STEMS` and `EARTHLY_BRANCHES` canonical ordering constants.
  - `StemBranchError::InvalidStemBranchPair` for parity-mismatched pairs.
- `serde` support for the new sexagenary types (behind the `serde` feature).
- Regression and property-style integration tests for the sexagenary and
  stem-branch APIs.

## [0.1.0]

### Added

- Gregorian ↔ Chinese lunisolar date conversion (`solar_to_lunar`,
  `lunar_to_solar`) over supported lunar years 1850..=2150.
- `LunarDate` and `SolarDate` types.
- Leap-month normalization via `normalize_lunar_date`.
- Twelve-branch time index (时辰, shíchen) via `time_index`.
- `LunarError` for date and time validation failures.
- Optional `serde` support behind the `serde` feature.

[Unreleased]: https://github.com/JinyangWang27/lunar-lite-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/JinyangWang27/lunar-lite-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/JinyangWang27/lunar-lite-rs/releases/tag/v0.1.0
