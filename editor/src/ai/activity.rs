//! Observable execution phases, never an inference about a model's thoughts.
use std::time::{Duration, Instant};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Phase {
    #[default]
    Ready,
    Connecting,
    Generating,
    NativeTool(String),
    Validating(usize),
    ProposalReady,
    AwaitingApproval,
    Applying,
    Complete,
    Error,
    Cancelling,
    Cancelled,
}
impl Phase {
    pub fn label(&self) -> String {
        match self {
            Self::Ready => "Ready".into(),
            Self::Connecting => "Connecting…".into(),
            Self::Generating => "Generating / receiving…".into(),
            Self::NativeTool(name) => format!("Native tool: {name}"),
            Self::Validating(n) => format!("Validating {n} native actions…"),
            Self::ProposalReady => "Proposal validated".into(),
            Self::AwaitingApproval => "Awaiting your approval".into(),
            Self::Applying => "Applying native edits…".into(),
            Self::Complete => "Completed".into(),
            Self::Error => "Error".into(),
            Self::Cancelling => "Cancelling active request…".into(),
            Self::Cancelled => "Cancelled".into(),
        }
    }
}
#[derive(Default)]
pub struct Activity {
    pub phase: Phase,
    started: Option<Instant>,
    elapsed: Duration,
}
impl Activity {
    pub fn begin(&mut self) {
        self.started = Some(Instant::now());
        self.elapsed = Duration::ZERO;
        self.phase = Phase::Connecting;
    }
    pub fn set(&mut self, phase: Phase) {
        self.phase = phase;
    }
    pub fn finish(&mut self, phase: Phase) {
        if let Some(started) = self.started.take() {
            self.elapsed = started.elapsed();
        }
        self.phase = phase;
    }
    pub fn elapsed(&self) -> Duration {
        self.started.map(|s| s.elapsed()).unwrap_or(self.elapsed)
    }
}
