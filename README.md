# lunar-lite

[![CI](https://github.com/JinyangWang27/lunar-lite-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/JinyangWang27/lunar-lite-rs/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/JinyangWang27/lunar-lite-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/JinyangWang27/lunar-lite-rs)

A small, table-backed Rust library for Chinese lunisolar (农历) date conversion.

## What it does

`lunar-lite` converts between Gregorian solar dates and Chinese lunisolar dates, including leap-month handling, traditional twelve-branch time index (时辰, shíchen) calculation, and sexagenary (干支, ganzhi) stem-branch cycle positions.

**Supported lunar years: 1850..=2150**

## What it does not do

See [Non-goals](#non-goals).

## Design

All conversion data is stored in a generated static table compiled into the binary — there is no runtime astronomical calculation. The table is generated ahead of time from a reference implementation and committed to the crate; at runtime the crate is a pure Rust lookup with no I/O and no allocations on the hot path.

## Installation

```toml
[dependencies]
lunar-lite = "0.2"
```

With Serde support:

```toml
[dependencies]
lunar-lite = { version = "0.2", features = ["serde"] }
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

## Leap months

The Chinese lunisolar calendar inserts an intercalary (leap) month roughly every three years. `LunarDate` carries an `is_leap_month: bool` field to distinguish the leap copy of a month from the regular one.

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
| `LunarError::InvalidLunarDate`           | Lunar date is structurally invalid or day exceeds month length |
| `LunarError::YearOutOfRange`             | Year is outside 1850..=2150                                    |
| `LunarError::InvalidTime`                | Hour > 23 or minute > 59                                       |
| `StemBranchError::InvalidStemBranchPair` | The stem and branch do not form a valid sexagenary pair        |

## Reference data generation

The static table in `src/generated/year_info.rs` is produced by:

```
tools/lunar-lite-reference/scripts/dump-year-info.mjs
```

This Node.js script uses the [`lunar-typescript`](https://github.com/6tail/lunar-typescript) library as its reference source and writes both the Rust table and a JSON fixture used by the compatibility test suite.

**Runtime users do not need Node.js or `lunar-typescript`.** The generated file is committed to the repository and regeneration is only needed if you extend the supported year range or update the reference data.

To regenerate:

```sh
cd tools/lunar-lite-reference
npm install
node scripts/dump-year-info.mjs
```

## Compatibility with lunar-typescript

Conversion results are generated from `lunar-typescript` and are expected to match it for all dates in the supported range. The crate does not embed the full astronomical engine; it is a lightweight Rust consumer of pre-computed data. Results are described as "generated from the reference implementation" rather than independently verified against historical astronomical records.

## Non-goals

- **BaZi (八字) engine** — pillar calculation is not included.
- **Solar terms (节气) API** — not yet exposed.
- **True solar time correction** — time zone offsets based on longitude are not applied.
- **Zi Wei Dou Shu (紫微斗数) charting** — out of scope.
- **Runtime JavaScript dependency** — the crate is pure Rust at runtime.

## License

MIT — see [LICENSE](LICENSE).
