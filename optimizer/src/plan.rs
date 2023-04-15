use crate::Assignment;
use fight_domain::{Attack, Character, Lookup, LookupKey};

#[derive(Debug, Clone)]
pub struct Plan {
    pub characters: Lookup<Character>,
    pub attacks: Lookup<Attack>,
    pub assignments: Lookup<Assignment>,
}

impl Plan {
    pub fn new(
        characters: Lookup<Character>,
        attacks: Lookup<Attack>,
        starting_plan: Lookup<Assignment>,
    ) -> Self {
        let mut plan = Self {
            characters,
            attacks,
            assignments: Default::default(),
        };

        for assignment in starting_plan {
            plan.assign_cooldown(assignment);
        }

        plan
    }

    pub fn assign_cooldown(&mut self, assignment: Assignment) {
        self.assignments.put(assignment)
    }

    pub fn unassign_cooldown(&mut self, assignment: &Assignment) {
        self.assignments.take(assignment.lookup_key());
    }
}
