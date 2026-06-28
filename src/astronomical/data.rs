//! Build-time generated astronomical data.
//!
//! These `static` constants are emitted by `build.rs` from the reviewable source
//! data under `data/astronomical/`. They are compiled into the crate, so there
//! is no runtime file I/O or parsing. Portions of the source data are adapted
//! from MIT-licensed `6tail/tyme4rs`; see `THIRD_PARTY_LICENSES.md`.

// Generated coefficients include values close to mathematical constants.
#![allow(clippy::approx_constant)]

include!(concat!(env!("OUT_DIR"), "/astronomical_data.rs"));
