use std::collections::HashMap;

use fight_domain::{AttackUuid, SpellUuid};
use optimizer::AssignmentState;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum InternalAssignmentState {
    Locked,
    Suggested,
    LockedAndSuggested,
}

impl InternalAssignmentState {
    fn is_locked(&self) -> bool {
        matches!(self, Self::Locked | Self::LockedAndSuggested)
    }

    fn is_suggested(&self) -> bool {
        matches!(self, Self::Suggested | Self::LockedAndSuggested)
    }
}

impl From<InternalAssignmentState> for AssignmentState {
    fn from(state: InternalAssignmentState) -> Self {
        use InternalAssignmentState::*;
        match state {
            Locked | LockedAndSuggested => AssignmentState::Locked,
            Suggested => AssignmentState::Suggested,
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct PlannerAssignments {
    assignments: HashMap<(SpellUuid, AttackUuid), InternalAssignmentState>,
}

impl PlannerAssignments {
    pub fn get(&self, spell: SpellUuid, attack: AttackUuid) -> AssignmentState {
        match self.assignments.get(&(spell, attack)) {
            Some(&state) => state.into(),
            None => AssignmentState::Unassigned,
        }
    }

    pub fn replace_suggestions(
        &mut self,
        new_suggestions: impl IntoIterator<Item = (SpellUuid, AttackUuid)>,
    ) {
        self.assignments.retain(|_, state| state.is_locked());
        self.assignments.extend(
            new_suggestions
                .into_iter()
                .map(|(spell, attack)| ((spell, attack), InternalAssignmentState::Suggested)),
        );
    }

    pub fn lock_suggestions(&mut self) {
        for state in self.assignments.values_mut() {
            if matches!(state, InternalAssignmentState::Suggested) {
                *state = InternalAssignmentState::LockedAndSuggested;
            }
        }
    }

    pub fn assign_locked(&mut self, spell: SpellUuid, attack: AttackUuid) {
        use InternalAssignmentState::*;
        let current = self.assignments.get(&(spell, attack));
        let new_state = match current {
            Some(Suggested | LockedAndSuggested) => LockedAndSuggested,
            _ => Locked,
        };
        self.assignments.insert((spell, attack), new_state);
    }

    pub fn unlock(&mut self, spell: SpellUuid, attack: AttackUuid) {
        use InternalAssignmentState::*;
        let current = self.assignments.get_mut(&(spell, attack));
        match current {
            Some(state @ LockedAndSuggested) => {
                *state = Suggested;
            }
            Some(Locked) => {
                self.assignments.remove(&(spell, attack));
            }
            _ => {}
        };
    }

    pub fn suggested(&self) -> impl Iterator<Item = &(SpellUuid, AttackUuid)> {
        self.assignments
            .iter()
            .filter(|(_, s)| s.is_suggested())
            .map(|(a, _)| a)
    }

    pub fn locked(&self) -> impl Iterator<Item = &(SpellUuid, AttackUuid)> {
        self.assignments
            .iter()
            .filter(|(_, s)| s.is_locked())
            .map(|(a, _)| a)
    }

    pub fn all(&self) -> impl Iterator<Item = &(SpellUuid, AttackUuid)> {
        self.assignments.keys()
    }

    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }

    /*
    pub fn iter(&self) -> Iter<'_, (SpellUuid, AttackUuid), Locked> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, (SpellUuid, AttackUuid), Locked> {
        self.into_iter()
    }

     */
}

/*
impl IntoIterator for PlannerAssignments {
    type Item = (SpellUuid, AttackUuid);
    type IntoIter = IntoIter<(SpellUuid, AttackUuid), Locked>;

    fn into_iter(self) -> Self::IntoIter {
        self.assignments.into_iter()
    }
}

impl<'a> IntoIterator for &'a PlannerAssignments {
    type Item = (&'a SpellUuid, &'a AttackUuid);
    type IntoIter = Iter<'a, (SpellUuid, AttackUuid), Locked>;

    fn into_iter(self) -> Self::IntoIter {
        self.assignments.iter()
    }
}

impl<'a> IntoIterator for &'a mut PlannerAssignments {
    type Item = (&'a SpellUuid, &'a AttackUuid);
    type IntoIter = IterMut<'a, (SpellUuid, AttackUuid), Locked>;

    fn into_iter(self) -> Self::IntoIter {
        self.assignments.iter_mut()
    }
}
*/
