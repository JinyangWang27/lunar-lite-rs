# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Four-pillar (四柱 / 八字 BaZi) stem-branch calculation, a faithful port of the
  TypeScript `lunar-lite@0.2.8` `getHeavenlyStemAndEarthlyBranchBySolarDate`:
  - `get_heavenly_stem_and_earthly_branch_by_solar_date` (default options) and
    `..._with_options`, plus the shorter aliases `solar_date_to_ganzhi` and
    `solar_date_to_ganzhi_with_options`, returning a
    `HeavenlyStemAndEarthlyBranchDate` with `yearly`, `monthly`, `daily`, and
    `hourly` pillars.
  - `StemBranchOptions` (default `Exact`/`Exact`) with `YearDivide` (`Normal` =
    lunar year, `Exact` = 立春 at date granularity) and `MonthDivide` (`Normal` =
    lunar-month 五虎遁, `Exact` = 12 Jie solar terms at exact second).
  - Day pillar via Julian-day arithmetic with 晚子时 (late 子) day rollover at
    `time_index` 12.
  - Supported range 1850-01-01 ..= 2150-12-31.
- Generated 12-Jie solar-term boundary table (`src/generated/solar_terms.rs`) from
  `lunar-typescript@1.8.6`, plus `generate-solar-terms.mjs` and
  `generate-four-pillars-fixtures.mjs` generators.
- `LunarError::InvalidTimeIndex` and `LunarError::SolarTermOutOfRange` variants.
- `serde` support for the new four-pillar types (behind the `serde` feature).
- Fixture-driven compatibility tests against `lunar-lite@0.2.8` output.

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
