//! `sinerack` — the shared DSP core for the q-lib audio crates.
//!
//! sinerack is the base layer everything else stands on: the leaf DSP crates
//! ([`pitchrack`] pitch detection, `phaserack` time-stretching, `noiserack` denoising) and
//! the `mixrack` engine all depend on it. It holds the primitives and common
//! value types that would otherwise be duplicated across them.
//!
//! It provides the shared [`Latency`] currency every pipeline stage reports in,
//! the [`Float`] numeric abstraction (`f32`/`f64`), and the [`buffer`] scratch
//! pool / complex helpers the FFT-based stages share. More primitives (cached
//! FFT plans, windowing, framing) are distilled in here as the split progresses
//! — see `docs/ROADMAP.md`.
//!
//! [`pitchrack`]: https://github.com/QsKue/pitchrack

mod float;
mod latency;

pub mod buffer;

pub use float::Float;
pub use latency::Latency;
