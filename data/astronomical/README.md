# Astronomical source data

These files are the reviewable source data for `lunar-lite`'s internal
astronomical backend (new-moon and solar-term calculation).

## How they are used

- They are **build-time inputs**. `build.rs` reads them and generates typed Rust
  constants into `$OUT_DIR/astronomical_data.rs`, which the crate `include!`s via
  `src/astronomical/data.rs`.
- They are **included in the published crate package** so `build.rs` can run when
  the crate is built from crates.io.
- They are **not loaded at runtime**. There is no runtime file I/O or parsing;
  the constants are compiled into the library.

## Files

| File | Contents |
| --- | --- |
| `delta_t.csv` | ΔT (TT−UT1) polynomial table |
| `nutation_terms.csv` | Nutation-in-longitude series terms |
| `earth_longitude_terms.txt` | Earth ecliptic-longitude periodic series |
| `moon_longitude_terms.txt` | Moon ecliptic-longitude periodic series (sectioned) |
| `qi_calibration.csv` | Solar-term (qi) calibration table |
| `shuo_calibration.csv` | New-moon (shuo) calibration table |
| `qi_corrections.txt` | Solar-term correction string (compressed encoding) |
| `shuo_corrections.txt` | New-moon correction string (compressed encoding) |
| `leap_month_codes.txt` | Per-year leap-month codes |

CSV files are flattened in row order; `#` begins a comment. The `.txt`
coefficient files use `[section]` headers with one `f64` per line. The
correction strings use a compact encoding that `build.rs` expands (`decode`).

## Changing this data

Do not regenerate or edit these files silently. The values are calibrated; any
change must be validated against the existing tyme-compatibility and conversion
tests before being committed.

## Provenance and licensing

Portions of this data are adapted from the MIT-licensed project
[`6tail/tyme4rs`](https://github.com/6tail/tyme4rs). This is an independent
adaptation; it does not imply any endorsement by, or affiliation with, `6tail`
or the `tyme4rs` project. The upstream MIT notice is reproduced in
[`../../THIRD_PARTY_LICENSES.md`](../../THIRD_PARTY_LICENSES.md).
