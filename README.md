# lunar-lite

[![Crates.io Version](https://img.shields.io/crates/v/lunar-lite.svg)](https://crates.io/crates/lunar-lite)
[![Crates.io Downloads](https://img.shields.io/crates/d/lunar-lite.svg)](https://crates.io/crates/lunar-lite)
[![Docs.rs](https://img.shields.io/docsrs/lunar-lite.svg)](https://docs.rs/lunar-lite)
[![CI](https://github.com/JinyangWang27/lunar-lite-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/JinyangWang27/lunar-lite-rs/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/JinyangWang27/lunar-lite-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/JinyangWang27/lunar-lite-rs)
[![License](https://img.shields.io/crates/l/lunar-lite.svg)](https://crates.io/crates/lunar-lite)

A compact, table-backed Rust library for Chinese lunisolar (农历) date conversion and stem-branch (干支) calculation.

## What it does

`lunar-lite` converts between Gregorian solar dates and Chinese lunisolar dates, including leap-month handling, traditional twelve-branch time index (时辰, shíchen) calculation, sexagenary (干支, ganzhi) stem-branch cycle positions, and four-pillar (四柱 / 八字 BaZi) year/month/day/hour stem-branch calculation.

**Supported lunar years: 1850..=2150**

## What it does not do

See [Non-goals](#non-goals).

## Design

`lunar-lite` aims to be small, deterministic, and idiomatic Rust. It does not embed a runtime astronomical engine, but it also does not store a huge day-by-day solar/lunar mapping table.

- **Lunar/solar conversion stores year structure, not every date mapping.** Each supported lunar year is represented by compact metadata: the Gregorian date of Chinese New Year, the leap-month position, the number of lunar months, the ordered month codes, and the length of each lunar month. Solar-to-lunar conversion resolves the lunar year and walks the month lengths by offset; lunar-to-solar conversion performs the inverse offset lookup.
- **Stem-branch calculation stores precise solar-term boundaries, not precomputed pillars.** The generated solar-term table stores the exact date-time of the 12 Jie (节) boundaries for each supported Gregorian year. In `Exact` mode, the month branch is determined by the most recent Jie boundary, while the month Heavenly Stem is derived from the relevant sui/year stem using 五虎遁.
- **Runtime stays pure Rust and lightweight.** Generated tables are committed under `src/generated/`; runtime users do not need Node.js, `lunar-typescript`, or `lunar-lite`. There is no runtime I/O, no runtime JavaScript dependency, and no allocations on the conversion hot path.

## Installation

```sh
cargo add lunar-lite
```

## Usage

### Solar → lunar

```rust
use lunar_lite::{SolarDate, solar_to_lunar};

let solar = SolarDate { year: 2023, month: 1, day: 22 };
let lunar = solar_to_lunar(solar).unwrap();
// LunarDate { year: 2023, month: 1, day: 1, is_leap_month: false }
```

### Lunar → solar

```rust
use lunar_lite::{LunarDate, lunar_to_solar};

let lunar = LunarDate { year: 2023, month: 2, day: 1, is_leap_month: true };
let solar = lunar_to_solar(lunar).unwrap();
// SolarDate { year: 2023, month: 3, day: 22 }
```

### Leap-month normalization

```rust
use lunar_lite::{LunarDate, normalize_lunar_date};

// 2024 has no leap month 1 — the flag is silently dropped.
let date = LunarDate { year: 2024, month: 1, day: 1, is_leap_month: true };
let normalized = normalize_lunar_date(date).unwrap();
// LunarDate { year: 2024, month: 1, day: 1, is_leap_month: false }
```

### Time index (时辰)

```rust
use lunar_lite::time_index;

assert_eq!(time_index(0, 30).unwrap(), 0);   // early Zi  00:00–00:59
assert_eq!(time_index(1, 0).unwrap(), 1);    // Chou      01:00–02:59
assert_eq!(time_index(23, 0).unwrap(), 12);  // late Zi   23:00–23:59
```

Index mapping:

| Index | Branch        | Hours       |
| ----- | ------------- | ----------- |
| 0     | 子 (early Zi) | 00:00–00:59 |
| 1     | 丑 Chou       | 01:00–02:59 |
| 2     | 寅 Yin        | 03:00–04:59 |
| 3     | 卯 Mao        | 05:00–06:59 |
| 4     | 辰 Chen       | 07:00–08:59 |
| 5     | 巳 Si         | 09:00–10:59 |
| 6     | 午 Wu         | 11:00–12:59 |
| 7     | 未 Wei        | 13:00–14:59 |
| 8     | 申 Shen       | 15:00–16:59 |
| 9     | 酉 You        | 17:00–18:59 |
| 10    | 戌 Xu         | 19:00–20:59 |
| 11    | 亥 Hai        | 21:00–22:59 |
| 12    | 子 (late Zi)  | 23:00–23:59 |

Zi hour is split: the early half (0) begins the current day, the late half (12) closes it.

When you only need the hour branch (时支) rather than the full hour pillar, use the
index helpers. Both 子 halves resolve to the same branch, and the predicates return
`Result<bool, _>` so an invalid index is never silently treated as `false`:

```rust
use lunar_lite::{is_early_zi, is_late_zi, time_index_to_branch, EarthlyBranch};

assert_eq!(time_index_to_branch(0).unwrap(), EarthlyBranch::Zi);  // early 子
assert_eq!(time_index_to_branch(12).unwrap(), EarthlyBranch::Zi); // late 子
assert_eq!(time_index_to_branch(1).unwrap(), EarthlyBranch::Chou);

assert!(is_early_zi(0).unwrap());
assert!(is_late_zi(12).unwrap());
assert!(time_index_to_branch(13).is_err()); // LunarError::InvalidTimeIndex
```

### Sexagenary cycle (干支)

The sexagenary cycle pairs the ten Heavenly Stems (天干) with the twelve Earthly
Branches (地支) into sixty positions (六十甲子). The conventional anchor `1984 = 甲子`
(JiaZi) is used for year pillars.

```rust
use lunar_lite::{EarthlyBranch, HeavenlyStem, StemBranch};

// Year pillar from a lunar year.
let pillar = StemBranch::from_lunar_year(2024);
assert_eq!(pillar.stem(), HeavenlyStem::Jia);    // 甲
assert_eq!(pillar.branch(), EarthlyBranch::Chen); // 辰  -> 甲辰

// Position within the sixty-step cycle (0 = JiaZi, 59 = GuiHai).
assert_eq!(pillar.cycle_index(), 40);
assert_eq!(StemBranch::from_cycle_index(0).stem(), HeavenlyStem::Jia);

// Validated construction: only the sixty parity-matched pairs are accepted.
assert!(StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Zi).is_ok());
assert!(StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Chou).is_err());
```

`HeavenlyStem` and `EarthlyBranch` each expose `index`, `from_index`, and a
wrapping `offset`; the `HEAVENLY_STEMS` and `EARTHLY_BRANCHES` constants give the
canonical cyclic ordering.

When a domain rule needs the lunar **birth-year** stem/branch (生年干/生年支) — as
opposed to the four-pillar `yearly` pillar, which may use the 立春 (LiChun) boundary
under `YearDivide::Exact` — use the lunar-year helpers, which read directly from the
lunar year:

```rust
use lunar_lite::{lunar_year_branch, lunar_year_stem, lunar_year_stem_branch};
use lunar_lite::{EarthlyBranch, HeavenlyStem, StemBranch};

assert_eq!(lunar_year_stem_branch(2024), StemBranch::from_lunar_year(2024));
assert_eq!(lunar_year_stem(2024), HeavenlyStem::Jia);    // 生年干 甲
assert_eq!(lunar_year_branch(2024), EarthlyBranch::Chen); // 生年支 辰
```

### Four pillars (四柱 / 八字)

The four-pillar stem-branch API computes the year, month, day, and hour pillars
(`FourPillars`) for a Gregorian date and a 时辰 index. It is a faithful port of the
TypeScript [`lunar-lite`](https://github.com/SylarLong/lunar-lite) function
`getHeavenlyStemAndEarthlyBranchBySolarDate` and is validated against its output.

The Rust-native entry points are `four_pillars_from_solar_date` (default options)
and `four_pillars_from_solar_date_with_options`. The long
`get_heavenly_stem_and_earthly_branch_by_solar_date[_with_options]` names are kept
to document parity with the TypeScript reference; `HeavenlyStemAndEarthlyBranchDate`
remains available as a type alias for `FourPillars` for the same reason.

```rust
use lunar_lite::{
    four_pillars_from_solar_date, four_pillars_from_solar_date_with_options,
    EarthlyBranch, FourPillars, HeavenlyStem, MonthDivide, SolarDate, StemBranchOptions,
    YearDivide,
};

let solar = SolarDate { year: 2000, month: 8, day: 16 };

// Simplest call: default options (Exact, Exact, matching the TypeScript reference).
// time_index 2 == 寅时 (03:00–04:59).
let pillars: FourPillars = four_pillars_from_solar_date(solar, 2).unwrap();

assert_eq!(pillars.yearly.stem(), HeavenlyStem::Geng);  // 庚辰
assert_eq!(pillars.monthly.branch(), EarthlyBranch::Shen); // 甲申

// Choose boundary conventions explicitly:
let options = StemBranchOptions { year: YearDivide::Exact, month: MonthDivide::Exact };
let _ = four_pillars_from_solar_date_with_options(solar, 2, options);
```

The wall-clock time is synthesized from `time_index` (`hour = max(time_index * 2 -
1, 0), minute = 30`), matching the reference. `time_index` is `0..=12`, where both
`0` (early 子) and `12` (late 子) map to the 子 branch; `12` additionally rolls the
day pillar forward to the next day (晚子时).

**Year pillar — `YearDivide`:**

- `Normal`: uses the lunar year, so the pillar changes at Chinese New Year.
- `Exact`: uses the 立春 (LiChun) boundary, compared at **date** granularity — on or
  after the 立春 calendar date counts as the new year.

**Month pillar — `MonthDivide`:**

- `Normal`: derived from the **lunar month** via 五虎遁 (not solar terms).
- `Exact`: derived from the 12 Jie (节) **solar-term** boundaries, switching at the
  **exact second** of each term.

> The month pillar uses solar terms, **not** the lunar month, in `Exact` mode. The
> two modes are intentionally asymmetric: `year:Exact` resolves at date granularity
> while `month:Exact` resolves at second granularity, reproducing the reference.

**Supported range:** four-pillar calculation covers **1850-01-01 ..= 2150-12-31**.
`Exact` options cover the whole range; `Normal` options additionally depend on the
lunar-year table, so solar dates before Chinese New Year 1850 (lunar year 1849)
return `LunarError::YearOutOfRange`.

## Leap months

The Chinese lunisolar calendar inserts an intercalary (leap) month roughly every three years. `LunarDate` carries an `is_leap_month: bool` field to distinguish the leap copy of a month from the regular one.

### Lunar month helpers

`lunar-lite` exposes helpers for leap-month lookup, lunar month length, and strict lunar date validation:

- `leap_month(year)`
- `has_leap_month(year)`
- `lunar_month_days(year, month, is_leap_month)`
- `validate_lunar_date(date)`

These APIs report calendar facts only. They do not encode downstream chart-placement policies such as whether a leap month should count as the previous month or split at mid-month. Invalid month or leap-month selections return `LunarError::InvalidLunarDate`.

`normalize_lunar_date` is the safe entry point for externally-supplied dates:

- If `is_leap_month = true` **and** the year actually has a leap month at that position, the date is kept as-is.
- If `is_leap_month = true` **but** the year has no leap month at that position, the flag is cleared and the date is treated as the regular month.
- After normalization the actual day count is validated; an out-of-range day returns `LunarError::InvalidLunarDate`.

`lunar_to_solar` calls `normalize_lunar_date` internally, so passing a fake leap flag is safe.

## Error handling

Date/time conversion functions return `Result<_, LunarError>`.
Stem-branch validation returns `Result<_, StemBranchError>`.

| Variant                                  | Meaning                                                        |
| ---------------------------------------- | -------------------------------------------------------------- |
| `LunarError::InvalidSolarDate`           | Solar date is not a valid calendar date                        |
| `LunarError::InvalidLunarDate`           | Lunar date is structurally invalid, requested month/leap month does not exist, or day exceeds month length |
| `LunarError::YearOutOfRange`             | Year is outside 1850..=2150                                    |
| `LunarError::InvalidTime`                | Hour > 23 or minute > 59                                       |
| `LunarError::InvalidTimeIndex`           | 时辰 index is outside 0..=12                                   |
| `LunarError::SolarTermOutOfRange`        | Gregorian year is outside the solar-term table (1850..=2150)   |
| `StemBranchError::InvalidStemBranchPair` | The stem and branch do not form a valid sexagenary pair        |

## Reference data generation

The static tables in `src/generated/` are produced by Node.js scripts under
`tools/lunar-lite-reference/scripts/`:

| Script                               | Generates                                                               |
| ------------------------------------ | ----------------------------------------------------------------------- |
| `dump-year-info.mjs`                 | `src/generated/year_info.rs` (lunar-year metadata) + year-info fixtures |
| `generate-solar-terms.mjs`           | `src/generated/solar_terms.rs` (the 12 Jie per year, 1850..=2150)       |
| `generate-four-pillars-fixtures.mjs` | `tests/fixtures/four_pillars.json` (four-pillar compatibility cases)    |

The solar-term and year-info scripts use [`lunar-typescript`](https://github.com/6tail/lunar-typescript) as their reference source; the four-pillar fixtures use [`lunar-lite`](https://github.com/SylarLong/lunar-lite). The solar-term generator fails unless every year yields exactly 12 strictly-ordered Jie boundaries.

**Runtime users do not need Node.js, `lunar-typescript`, or `lunar-lite`.** The generated files are committed to the repository; regeneration is only needed when extending the supported range or updating the reference data.

To regenerate:

```sh
cd tools/lunar-lite-reference
npm install
npm run dump-year-info
npm run generate-solar-terms
npm run generate-four-pillars-fixtures
```

## Compatibility with lunar-typescript

Conversion results are generated from `lunar-typescript` and are expected to match it for all dates in the supported range. The crate does not embed the full astronomical engine; it is a lightweight Rust consumer of pre-computed data. Results are described as "generated from the reference implementation" rather than independently verified against historical astronomical records.

## Non-goals

- **Solar terms (节气) API** — Jie boundaries back the four-pillar month pillar but are not exposed as a standalone public API.
- **True solar time correction** — time zone offsets based on longitude are not applied; the four-pillar time is synthesized from `time_index`.
- **Zi Wei Dou Shu (紫微斗数) charting** — out of scope.
- **Runtime JavaScript dependency** — the crate is pure Rust at runtime.

## Release process

Releases are managed by [release-plz](https://release-plz.dev/).

After changes are merged into `main`, release-plz opens or updates a Release PR.
Review the version bump and changelog in that PR. When the Release PR is merged,
the workflow publishes the crate to crates.io and creates the GitHub release/tag.

Required repository secret:

- `CARGO_REGISTRY_TOKEN`: crates.io API token with permission to publish `lunar-lite`.

## License

MIT — see [LICENSE](LICENSE).
