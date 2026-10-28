//! Exact, caller-owned scheduling foundation for an opt-in future ReplayV1.
//!
//! Not connected to PhysicsState, evaluate, transports or serialization. Logical
//! time is an absolute rational in an explicit epoch, never accumulated f32 dt.
//! A synchronous consumer must report success before its tick is committed.

pub const TICKS_PER_SECOND: u64 = 120;
pub const INTERACTIVE_BUDGET: u32 = 8;
pub const MAX_BUDGET: u32 = 65_536;

/// Descriptive activation boundary; existing consumers remain LegacyT839.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SimulationProfile {
    #[default]
    LegacyT839,
    ReplayV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockError {
    Uninitialized,
    InvalidEpoch,
    InvalidTime,
    Overflow,
    BackwardTime,
    InvalidBudget,
}
impl ClockError {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Uninitialized => "uninitialized",
            Self::InvalidEpoch => "invalid_epoch",
            Self::InvalidTime => "invalid_time",
            Self::Overflow => "overflow",
            Self::BackwardTime => "backward_time",
            Self::InvalidBudget => "invalid_budget",
        }
    }
}
impl std::fmt::Display for ClockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for ClockError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RationalTime {
    pub numerator: u64,
    pub denominator: u64,
}
impl RationalTime {
    pub(crate) fn checked(numerator: u64, denominator: u64) -> Result<(Self, u64), ClockError> {
        if denominator == 0 {
            return Err(ClockError::InvalidTime);
        }
        let target = u128::from(numerator)
            .checked_mul(u128::from(TICKS_PER_SECOND))
            .ok_or(ClockError::Overflow)?
            / u128::from(denominator);
        let target = u64::try_from(target).map_err(|_| ClockError::Overflow)?;
        let (mut a, mut b) = (numerator, denominator);
        while b != 0 {
            (a, b) = (b, a % b);
        }
        Ok((
            Self {
                numerator: numerator / a,
                denominator: denominator / a,
            },
            target,
        ))
    }
    fn compare(self, other: Self) -> std::cmp::Ordering {
        // Both cross-products fit exactly: each operand is at most u64::MAX.
        (u128::from(self.numerator) * u128::from(other.denominator))
            .cmp(&(u128::from(other.numerator) * u128::from(self.denominator)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClockSnapshot {
    pub initialized: bool,
    pub epoch: u64,
    pub requested: u64,
    pub completed: u64,
    pub backlog: u64,
    pub next: Option<u64>,
    pub time: Option<RationalTime>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScheduledTick {
    pub epoch: u64,
    pub index: u64,
}
/// Cancel/Failed do not commit the offered tick. Consumer side effects are the
/// caller's responsibility: this clock is not a transaction over physics state.
pub enum TickResult<E> {
    Complete,
    Cancelled,
    Failed(E),
}
#[derive(Debug, PartialEq, Eq)]
pub enum Stop<E> {
    TargetReached,
    BudgetExhausted,
    Unchanged,
    Cancelled,
    Failed(E),
}
#[derive(Debug, PartialEq, Eq)]
pub struct Progress<E> {
    pub state: ClockSnapshot,
    pub processed: u32,
    pub attempted: u32,
    pub stop: Stop<E>,
}

#[derive(Debug, Default)]
pub struct ReplayClock {
    epoch: u64,
    requested: u64,
    completed: u64,
    time: Option<RationalTime>,
}
impl ReplayClock {
    pub fn new() -> Self {
        Self::default()
    }
    pub const fn profile(&self) -> SimulationProfile {
        SimulationProfile::ReplayV1
    }
    pub fn snapshot(&self) -> ClockSnapshot {
        ClockSnapshot {
            initialized: self.time.is_some(),
            epoch: self.epoch,
            requested: self.requested,
            completed: self.completed,
            backlog: self.requested - self.completed,
            next: if self.completed < self.requested {
                Some(self.completed + 1)
            } else {
                None
            },
            time: self.time,
        }
    }
    /// Caller chooses a strictly increasing nonzero epoch. No implicit replay.
    /// Epoch IDs are scoped to this clock; do not exchange them across instances.
    pub fn reset(&mut self, epoch: u64) -> Result<(), ClockError> {
        if epoch == 0 || epoch <= self.epoch {
            return Err(ClockError::InvalidEpoch);
        }
        self.epoch = epoch;
        self.requested = 0;
        self.completed = 0;
        self.time = Some(RationalTime {
            numerator: 0,
            denominator: 1,
        });
        Ok(())
    }
    fn check_epoch(&self, epoch: u64) -> Result<(), ClockError> {
        if self.time.is_none() {
            return Err(ClockError::Uninitialized);
        }
        if epoch != self.epoch {
            return Err(ClockError::InvalidEpoch);
        }
        Ok(())
    }
    pub fn request_tick<E>(
        &mut self,
        epoch: u64,
        tick: u64,
        consume: impl FnMut(ScheduledTick) -> TickResult<E>,
    ) -> Result<Progress<E>, ClockError> {
        self.request_time(epoch, tick, TICKS_PER_SECOND, consume)
    }
    /// Positive sub-tick changes may process backlog; an exactly equal rational
    /// is observational, even when expressed with a different denominator.
    pub fn request_time<E>(
        &mut self,
        epoch: u64,
        numerator: u64,
        denominator: u64,
        consume: impl FnMut(ScheduledTick) -> TickResult<E>,
    ) -> Result<Progress<E>, ClockError> {
        self.check_epoch(epoch)?;
        let (time, target) = RationalTime::checked(numerator, denominator)?;
        match time.compare(self.time.expect("checked initialized epoch")) {
            std::cmp::Ordering::Less => return Err(ClockError::BackwardTime),
            std::cmp::Ordering::Equal => return Ok(self.progress(0, 0, Stop::Unchanged)),
            std::cmp::Ordering::Greater => {}
        }
        self.time = Some(time);
        self.requested = target;
        Ok(self.process(INTERACTIVE_BUDGET, consume))
    }
    /// Explicit bounded work on already requested ticks. Call again to drain.
    pub fn pump<E>(
        &mut self,
        epoch: u64,
        budget: u32,
        consume: impl FnMut(ScheduledTick) -> TickResult<E>,
    ) -> Result<Progress<E>, ClockError> {
        self.check_epoch(epoch)?;
        if budget == 0 || budget > MAX_BUDGET {
            return Err(ClockError::InvalidBudget);
        }
        Ok(self.process(budget, consume))
    }
    fn progress<E>(&self, processed: u32, attempted: u32, stop: Stop<E>) -> Progress<E> {
        Progress {
            state: self.snapshot(),
            processed,
            attempted,
            stop,
        }
    }
    fn process<E>(
        &mut self,
        budget: u32,
        mut consume: impl FnMut(ScheduledTick) -> TickResult<E>,
    ) -> Progress<E> {
        let mut processed = 0;
        while processed < budget && self.completed < self.requested {
            // Safe even at MAX: completed < requested <= u64::MAX.
            let index = self.completed + 1;
            match consume(ScheduledTick {
                epoch: self.epoch,
                index,
            }) {
                TickResult::Complete => {
                    self.completed = index;
                    processed += 1;
                }
                TickResult::Cancelled => {
                    return self.progress(processed, processed + 1, Stop::Cancelled);
                }
                TickResult::Failed(error) => {
                    return self.progress(processed, processed + 1, Stop::Failed(error));
                }
            }
        }
        self.progress(
            processed,
            processed,
            if self.completed == self.requested {
                Stop::TargetReached
            } else {
                Stop::BudgetExhausted
            },
        )
    }
}

#[cfg(test)]
#[path = "replay_clock_tests.rs"]
mod tests;
