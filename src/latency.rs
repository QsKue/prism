/// Latency contributed by an audio stage, measured in frames.
///
/// This is the shared currency every stage in the pipeline reports in, so the
/// engine can sum the latency of analyzers, denoisers, time-stretchers, and
/// processors into a single end-to-end figure. Each field is a distinct kind of
/// delay so they can be reasoned about (and summed) independently.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Latency {
    /// Input frames required before useful output can begin.
    pub input_frames: usize,
    /// Frames retained or delayed on the output side.
    pub output_frames: usize,
    /// Additional analysis/control lookahead before a decision is available.
    pub lookahead_frames: usize,
}

impl Latency {
    pub const fn new(input_frames: usize, output_frames: usize, lookahead_frames: usize) -> Self {
        Self {
            input_frames,
            output_frames,
            lookahead_frames,
        }
    }

    /// Total end-to-end delay in frames (the sum of all three components).
    pub const fn total_frames(self) -> usize {
        self.input_frames + self.output_frames + self.lookahead_frames
    }

    /// Total delay in milliseconds at `sample_rate`, or `None` if the rate is 0.
    pub fn total_ms(self, sample_rate: u32) -> Option<f64> {
        (sample_rate > 0).then(|| self.total_frames() as f64 * 1000.0 / sample_rate as f64)
    }
}
