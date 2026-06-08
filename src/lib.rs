//! `prism` — the shared DSP core for the q-lib audio crates.
//!
//! prism is the base layer everything else stands on: the leaf DSP crates
//! ([`reed`] pitch detection, `warble` time-stretching, `damper` denoising) and
//! the `maestro` engine all depend on it. It holds the primitives and common
//! value types that would otherwise be duplicated across them.
//!
//! Today it provides the shared [`Latency`] currency every pipeline stage
//! reports in. The heavier DSP primitives (a `Float` abstraction, a scratch
//! `BufferPool`, cached FFT plans, windowing, framing) are distilled in here as
//! the split progresses — see `docs/ROADMAP.md`.
//!
//! [`reed`]: https://github.com/QsKue/reed

mod latency;

pub use latency::Latency;
