# Decision: SineRack as the shared DSP core — primitives + value types, not domain traits

## Status

Accepted (documents the crate's founding boundary)

## Context

The q-lib audio system is split into independent leaf DSP crates — pitchrack (pitch detection), phaserack
(time-stretching), noiserack (denoising) — orchestrated by the mixrack engine. The leaves share genuine
low-level machinery: a `f32`/`f64` numeric abstraction, allocation-free FFT scratch management, and a
common way to report stage delay. Without a shared base, that machinery is **copied** into each leaf
(it originally lived in pitchrack), which means divergent copies and duplicated maintenance.

But the leaves do **not** share their *domain* shape — a pitch detector, a time-stretcher, and a
denoiser have genuinely different interfaces. Forcing them under one trait, or putting all three
domain traits in one crate, would chain unrelated crates together: a change for one leaf's interface
would force a version bump on the others.

Three things shape the design and should not be undone casually:

1. The shared primitives must live in exactly one place so the leaves can't diverge.
2. The leaves must stay decoupled — one leaf's churn must not ripple into the others.
3. Existing leaf call sites should not break when machinery is centralized.

## Decision

- **SineRack is the shared base: primitives + common value types only.** It holds `buffer` (the
  `BufferPool` scratch + copy/convert helpers, allocation-free and `Send`), `Float` (the `f32`/`f64`
  abstraction), and `Latency` (the frames-based delay currency every stage reports in, which mixrack
  sums and re-exports as `AudioLatency`). Its only runtime dependency is `rustfft`.
- **Domain traits stay in the leaves.** `PitchDetector` stays in pitchrack, `TimeStretcher` in phaserack,
  `Denoiser` in noiserack — never in SineRack. This keeps the leaves on independent version timelines. A
  common `Processor` super-trait may graduate into SineRack later **only if** real cross-leaf
  duplication appears — not speculatively.
- **A primitive graduates by distillation, with a second consumer.** Code moves *out of* a leaf into
  SineRack when a second crate would otherwise copy it (this is how `Float` and `buffer` arrived from
  pitchrack). The origin leaf then **re-exports** it so its call sites are unchanged — pitchrack does this with
  `pitchrack::float = sinerack::Float` and `pitchrack::utils::buffer::* = sinerack::buffer::*`.
- **AGPL-3.0-or-later.** SineRack is licensed `AGPL-3.0-or-later`, matching the copyleft posture of the
  audio system it underpins.

## Consequences

- New shared machinery lands in SineRack, generic over `Float` and allocation-conscious, and is
  re-exported from the crate it was distilled out of — not duplicated.
- Leaf crates stay decoupled: SineRack changes can affect all of them, but one leaf's domain changes
  never touch the others.
- SineRack must stay standalone-buildable and free of any leaf/engine knowledge; a primitive that only
  one crate uses does **not** belong here yet (premature distillation).
- The future distill targets (cached FFT plans, windowing, framing) still live in pitchrack and graduate
  here only when a second consumer needs them — see `docs/ROADMAP.md`.
- AGPL imposes its network-copyleft obligations on downstream use; that is intentional for this
  system.
