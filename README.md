# sinerack

The shared **DSP core** for the q-lib audio crates — the base layer that the leaf
crates ([`pitchrack`](https://github.com/QsKue/pitchrack) pitch detection,
[`phaserack`](https://github.com/QsKue/phaserack) time-stretching,
[`noiserack`](https://github.com/QsKue/noiserack) denoising) and the
[`mixrack`](https://github.com/QsKue/mixrack) engine all stand on.

SineRack holds the primitives and common value types that would otherwise be
duplicated across those crates. Today that is the shared `Latency` currency every
pipeline stage reports in; the heavier primitives (a `Float` abstraction, a
scratch `BufferPool`, cached FFT plans, windowing and framing) are distilled in
here as the audio split progresses — see [`docs/ROADMAP.md`](docs/ROADMAP.md).

## License

Licensed under the GNU Affero General Public License v3.0 or later
([AGPL-3.0-or-later](LICENSE)).
