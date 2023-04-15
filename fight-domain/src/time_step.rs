use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[serde(transparent)]
pub struct TimeStep(i32);

impl TimeStep {
    pub fn zero() -> TimeStep {
        TimeStep(0)
    }
    pub fn abs_diff(&self, rhs: TimeStep) -> TimeStep {
        TimeStep(self.0.abs_diff(rhs.0) as i32)
    }
    pub fn as_secs(&self) -> i32 {
        self.0
    }
}

impl Display for TimeStep {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let minutes = self.0 / 60;
        let seconds = self.0 % 60;
        write!(f, "{minutes:0>2}:{seconds:0>2}")
    }
}

impl Default for TimeStep {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<Duration> for TimeStep {
    fn from(d: Duration) -> Self {
        TimeStep(d.as_secs() as i32)
    }
}

impl From<TimeStep> for Duration {
    fn from(value: TimeStep) -> Self {
        Duration::from_secs(value.0 as u64)
    }
}

impl Sub<TimeStep> for TimeStep {
    type Output = TimeStep;

    fn sub(self, rhs: TimeStep) -> Self::Output {
        TimeStep(self.0 - rhs.0)
    }
}

impl Add<TimeStep> for TimeStep {
    type Output = TimeStep;

    fn add(self, rhs: TimeStep) -> Self::Output {
        TimeStep(self.0 + rhs.0)
    }
}

pub trait FromMinutesSeconds {
    fn mm_ss(minutes: u64, seconds: u64) -> Self;
}

impl FromMinutesSeconds for TimeStep {
    fn mm_ss(minutes: u64, seconds: u64) -> Self {
        TimeStep((minutes * 60 + seconds) as i32)
    }
}

impl FromMinutesSeconds for Duration {
    fn mm_ss(minutes: u64, seconds: u64) -> Self {
        Duration::from_secs(minutes * 60 + seconds)
    }
}
