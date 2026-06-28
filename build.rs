//! Build-time generator for the astronomical data kernel.
//!
//! Reads the reviewable source data under `data/astronomical/` and emits a
//! single Rust file (`$OUT_DIR/astronomical_data.rs`) containing typed `static`
//! constants. The runtime crate `include!`s that file, so there is no file I/O
//! or parsing at runtime.
//!
//! Portions of the source data are adapted from MIT-licensed `6tail/tyme4rs`;
//! see `THIRD_PARTY_LICENSES.md`. Float values are parsed and re-emitted with
//! `{:?}` (shortest round-tripping form), so no numerical value changes.

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

// Lunar-year bounds; kept in sync with `src/astronomical/lunar_year.rs`.
const MIN_LUNAR_YEAR: i64 = -1;
const MAX_LUNAR_YEAR: i64 = 9_999;

fn data_path(name: &str) -> PathBuf {
    let manifest = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    Path::new(&manifest).join("data/astronomical").join(name)
}

fn read(name: &str) -> String {
    let path = data_path(name);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()))
}

/// Parses every comma- or whitespace-separated numeric field in a file,
/// flattening rows in order. `#` starts a comment; blank lines are skipped.
fn parse_floats_csv(name: &str) -> Vec<f64> {
    let text = read(name);
    let mut out = Vec::new();
    for (lineno, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        for field in line.split(',') {
            let field = field.trim();
            if field.is_empty() {
                continue;
            }
            let value = field
                .parse::<f64>()
                .unwrap_or_else(|_| panic!("{name}:{}: invalid f64 field {field:?}", lineno + 1));
            out.push(value);
        }
    }
    out
}

/// Parses a sectioned coefficient file into `(section_name, values)` pairs in
/// file order. `[name]` opens a section; `#` starts a comment.
fn parse_sections(name: &str) -> Vec<(String, Vec<f64>)> {
    let text = read(name);
    let mut sections: Vec<(String, Vec<f64>)> = Vec::new();
    for (lineno, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(inner) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            sections.push((inner.to_string(), Vec::new()));
            continue;
        }
        let value = line
            .parse::<f64>()
            .unwrap_or_else(|_| panic!("{name}:{}: invalid f64 {line:?}", lineno + 1));
        let current = sections
            .last_mut()
            .unwrap_or_else(|| panic!("{name}:{}: value before any [section]", lineno + 1));
        current.1.push(value);
    }
    sections
}

/// Reads a single encoded/text payload line (skipping `#` comments).
fn read_payload_line(name: &str) -> String {
    let text = read(name);
    let mut payload = String::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        payload.push_str(line);
    }
    assert!(!payload.is_empty(), "{name}: payload is empty");
    payload
}

/// Expands the compressed correction encoding into a digit string. This runs at
/// build time only; the decoded string is emitted as a `static`.
fn decode(os: &str) -> String {
    let mut s = os.replace('J', "00");
    s = s.replace('I', "000");
    s = s.replace('H', "0000");
    s = s.replace('G', "00000");
    s = s.replace('t', "02");
    s = s.replace('s', "002");
    s = s.replace('r', "0002");
    s = s.replace('q', "00002");
    s = s.replace('p', "000002");
    s = s.replace('o', "0000002");
    s = s.replace('n', "00000002");
    s = s.replace('m', "000000002");
    s = s.replace('l', "0000000002");
    s = s.replace('k', "01");
    s = s.replace('j', "0101");
    s = s.replace('i', "001");
    s = s.replace('h', "001001");
    s = s.replace('g', "0001");
    s = s.replace('f', "00001");
    s = s.replace('e', "000001");
    s = s.replace('d', "0000001");
    s = s.replace('c', "00000001");
    s = s.replace('b', "000000001");
    s = s.replace('a', "0000000001");
    s = s.replace(
        'A',
        "000000000000000000000000000000000000000000000000000000000000",
    );
    s = s.replace('B', "00000000000000000000000000000000000000000000000000");
    s = s.replace('C', "0000000000000000000000000000000000000000");
    s = s.replace('D', "000000000000000000000000000000");
    s = s.replace('E', "00000000000000000000");
    s = s.replace('F', "0000000000");
    s
}

fn emit_f64_array(out: &mut String, name: &str, values: &[f64]) {
    writeln!(out, "pub(crate) static {name}: [f64; {}] = [", values.len()).unwrap();
    for v in values {
        writeln!(out, "    {v:?},").unwrap();
    }
    out.push_str("];\n\n");
}

fn emit_nested_f64(out: &mut String, name: &str, sections: &[(String, Vec<f64>)]) {
    writeln!(out, "pub(crate) static {name}: &[&[f64]] = &[").unwrap();
    for (_, values) in sections {
        out.push_str("    &[\n");
        for v in values {
            writeln!(out, "        {v:?},").unwrap();
        }
        out.push_str("    ],\n");
    }
    out.push_str("];\n\n");
}

fn emit_str(out: &mut String, name: &str, value: &str) {
    // The decoded correction strings and leap-month codes are plain ASCII
    // digits/letters, so a raw string literal is safe and exact.
    writeln!(out, "pub(crate) static {name}: &str = r\"{value}\";\n").unwrap();
}

fn main() {
    for file in [
        "delta_t.csv",
        "nutation_terms.csv",
        "earth_longitude_terms.txt",
        "moon_longitude_terms.txt",
        "qi_calibration.csv",
        "shuo_calibration.csv",
        "qi_corrections.txt",
        "shuo_corrections.txt",
        "leap_month_codes.txt",
    ] {
        println!("cargo:rerun-if-changed=data/astronomical/{file}");
    }
    println!("cargo:rerun-if-changed=build.rs");

    let nutation = parse_floats_csv("nutation_terms.csv");
    assert!(
        !nutation.is_empty() && nutation.len() % 5 == 0,
        "nutation_terms.csv: expected a multiple of 5 fields, got {}",
        nutation.len()
    );

    let delta_t = parse_floats_csv("delta_t.csv");
    assert!(
        delta_t.len() >= 7,
        "delta_t.csv: too few fields ({})",
        delta_t.len()
    );

    let earth_sections = parse_sections("earth_longitude_terms.txt");
    assert_eq!(
        earth_sections.len(),
        1,
        "earth_longitude_terms.txt: expected exactly one section"
    );
    let earth = &earth_sections[0].1;
    assert!(!earth.is_empty(), "earth_longitude_terms.txt: empty series");

    let moon_sections = parse_sections("moon_longitude_terms.txt");
    assert!(
        !moon_sections.is_empty(),
        "moon_longitude_terms.txt: no sections"
    );
    for (sec, values) in &moon_sections {
        assert!(
            !values.is_empty(),
            "moon_longitude_terms.txt: section [{sec}] is empty"
        );
    }

    let qi_cal = parse_floats_csv("qi_calibration.csv");
    assert!(
        qi_cal.len() >= 3 && qi_cal.len() % 2 == 1,
        "qi_calibration.csv: expected pairs plus a terminal (odd count), got {}",
        qi_cal.len()
    );

    let shuo_cal = parse_floats_csv("shuo_calibration.csv");
    assert!(
        shuo_cal.len() >= 3 && shuo_cal.len() % 2 == 1,
        "shuo_calibration.csv: expected pairs plus a terminal (odd count), got {}",
        shuo_cal.len()
    );

    let qi_corrections = decode(&read_payload_line("qi_corrections.txt"));
    assert!(
        !qi_corrections.is_empty(),
        "qi_corrections.txt: decoded to empty"
    );

    let shuo_corrections = decode(&read_payload_line("shuo_corrections.txt"));
    assert!(
        !shuo_corrections.is_empty(),
        "shuo_corrections.txt: decoded to empty"
    );

    let leap_codes = read_payload_line("leap_month_codes.txt");
    let expected_leap_len = (MAX_LUNAR_YEAR - MIN_LUNAR_YEAR + 1) as usize;
    assert_eq!(
        leap_codes.len(),
        expected_leap_len,
        "leap_month_codes.txt: expected {expected_leap_len} chars for years \
         {MIN_LUNAR_YEAR}..={MAX_LUNAR_YEAR}, got {}",
        leap_codes.len()
    );

    let mut out = String::new();
    out.push_str("// @generated by build.rs from data/astronomical/*. Do not edit.\n\n");
    emit_f64_array(&mut out, "NUTATION_TERMS", &nutation);
    emit_f64_array(&mut out, "DELTA_T_TABLE", &delta_t);
    emit_f64_array(&mut out, "EARTH_LONGITUDE_TERMS", earth);
    emit_nested_f64(&mut out, "MOON_LONGITUDE_TERMS", &moon_sections);
    emit_f64_array(&mut out, "QI_CALIBRATION", &qi_cal);
    emit_f64_array(&mut out, "SHUO_CALIBRATION", &shuo_cal);
    emit_str(&mut out, "QI_CORRECTIONS", &qi_corrections);
    emit_str(&mut out, "SHUO_CORRECTIONS", &shuo_corrections);
    emit_str(&mut out, "LEAP_MONTH_CODES", &leap_codes);

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR");
    let dest = Path::new(&out_dir).join("astronomical_data.rs");
    fs::write(&dest, out).unwrap_or_else(|e| panic!("writing {}: {e}", dest.display()));
}
