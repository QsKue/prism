# prism

The shared **DSP core** for the q-lib audio crates — the base layer that the leaf
crates ([`reed`](https://github.com/QsKue/reed) pitch detection,
[`warble`](https://github.com/QsKue/warble) time-stretching,
[`damper`](https://github.com/QsKue/damper) denoising) and the
[`maestro`](https://github.com/QsKue/maestro) engine all stand on.

prism holds the primitives and common value types that would otherwise be
duplicated across those crates. Today that is the shared `Latency` currency every
pipeline stage reports in; the heavier primitives (a `Float` abstraction, a
scratch `BufferPool`, cached FFT plans, windowing and framing) are distilled in
here as the audio split progresses — see [`docs/ROADMAP.md`](docs/ROADMAP.md).

## License

Licensed under the GNU Affero General Public License v3.0 or later
([AGPL-3.0-or-later](LICENSE)).
