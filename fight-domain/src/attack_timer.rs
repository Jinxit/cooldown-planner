use crate::TimeStep;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct AttackTimer {
    pub phase_start: Option<TimeStep>,
    pub phase_end: Option<TimeStep>,
    pub dynamic_timer: Option<TimeStep>,
    pub dynamic_trigger_cleu_event: Option<CleuEvent>,
}

impl AttackTimer {
    pub fn static_timer(&self) -> TimeStep {
        self.phase_start.unwrap_or(TimeStep::zero())
            + self.dynamic_timer.unwrap_or(TimeStep::zero())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct CleuEvent {
    pub r#type: CleuEventType,
    pub event: u64,
    pub counter: u64,
}

#[allow(clippy::enum_variant_names)]
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum CleuEventType {
    SpellCastSuccess,
    SpellCastStart,
    SpellAuraApplied,
    SpellAuraRemoved,
}

impl Display for CleuEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CleuEventType::SpellCastSuccess => "SCC",
                CleuEventType::SpellCastStart => "SCS",
                CleuEventType::SpellAuraApplied => "SAA",
                CleuEventType::SpellAuraRemoved => "SAR",
            }
        )
    }
}
