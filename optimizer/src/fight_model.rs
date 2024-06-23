use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;

use localsearch::OptModel;
use num_traits::Zero;
use ordered_float::NotNan;
use rand::prelude::*;
use rand::Rng;

use fight_domain::SpellUuid;

use crate::{Assignment, AssignmentState, Plan};
use crate::score_functions::ScoreFunction;

pub struct FightModel {
    pub score_function: ScoreFunction,
}

impl OptModel for FightModel {
    type ScoreType = NotNan<f64>;
    type SolutionType = Plan;
    type TransitionType = Option<(bool, Assignment)>;

    fn generate_random_solution<R: Rng>(
        &self,
        _rng: &mut R,
    ) -> Result<Self::SolutionType, Box<dyn Error>> {
        // we always provide an initial state, so no random state is necessary
        unimplemented!()
    }

    fn generate_trial_solution<R: Rng>(
        &self,
        current_solution: &Self::SolutionType,
        mut rng: &mut R,
        _current_score: Option<Self::ScoreType>,
    ) -> (Self::SolutionType, Self::TransitionType, Self::ScoreType) {
        let add = rng.gen_bool(0.5);
        if add {
            let mut options = vec![];
            for character in &current_solution.characters {
                for spell in &character.spells {
                    // skip spells that are not enabled
                    if !spell.enabled {
                        continue;
                    }
                    if spell.power.is_zero() {
                        continue;
                    }

                    let character_assignments = current_solution
                        .assignments
                        .iter()
                        .filter(|assignment| assignment.character == character.uuid)
                        .collect::<Vec<_>>();

                    // check for assigned spells that are mutually exclusive
                    let exclusive_with_spell_uuids: HashSet<SpellUuid> = character
                        .spells
                        .iter()
                        .filter(|s| spell.exclusive_with.contains(&s.identifier))
                        .map(|spell| spell.uuid)
                        .collect();
                    let has_assigned_exclusives = character_assignments
                        .iter()
                        .any(|assignment| exclusive_with_spell_uuids.contains(&assignment.spell));

                    if has_assigned_exclusives {
                        continue;
                    }

                    // check for assigned spells with the same spell ID but different version
                    let other_usages: Vec<&&Assignment> = character_assignments
                        .iter()
                        .filter(|assignment| {
                            character
                                .spells
                                .get(&assignment.spell)
                                .map(|assigned_spell| assigned_spell.identifier == spell.identifier)
                                .unwrap_or(false)
                        })
                        .collect();
                    if other_usages
                        .iter()
                        .any(|other_usage| other_usage.spell != spell.uuid)
                    {
                        continue;
                    }

                    // create trial states for every attack
                    for attack in &current_solution.attacks {
                        // check if any spells are assigned within +- cooldown of this attack
                        let other_usages_within_cooldown = other_usages
                            .iter()
                            .filter(|other_usage| {
                                let other_timer = &current_solution
                                    .attacks
                                    .get(&other_usage.attack)
                                    .unwrap()
                                    .timer;
                                other_timer
                                    .static_timer()
                                    .abs_diff(attack.timer.static_timer())
                                    <= spell.cooldown
                            })
                            .count();
                        // it's ok if the spell has multiple charges
                        // btw it's amazing that the math works out to be this simple
                        if other_usages_within_cooldown >= spell.charges {
                            continue;
                        }

                        // check if any spells are assigned with cast time overlapping this attack
                        let has_overlapping_casts =
                            character_assignments.iter().any(|other_usage| {
                                let attack_timer = attack.timer.static_timer();
                                let other_timer = current_solution
                                    .attacks
                                    .get(&other_usage.attack)
                                    .unwrap()
                                    .timer
                                    .static_timer();
                                let other_spell = character.spells.get(&other_usage.spell).unwrap();
                                match attack_timer.cmp(&other_timer) {
                                    Ordering::Less => {
                                        // assigned is after, check if new cast time would overlap
                                        attack_timer.abs_diff(other_timer) < spell.cast_time
                                    }
                                    Ordering::Greater => {
                                        // assigned is before, check if assigned cast time would overlap
                                        attack_timer.abs_diff(other_timer) < other_spell.cast_time
                                    }
                                    Ordering::Equal => true,
                                }
                            });
                        if has_overlapping_casts {
                            continue;
                        }

                        options.push(Assignment::new(
                            character.uuid,
                            spell.uuid,
                            attack.uuid,
                            AssignmentState::Suggested,
                        ));
                    }
                }
            }

            let mut plan = current_solution.clone();
            let assignment = options[..].choose(&mut rng);

            let score = match assignment {
                Some(assignment) => {
                    plan.assign_cooldown(assignment.clone());
                    self.evaluate_solution(&plan)
                }
                None => NotNan::new(10000000000.0).unwrap(),
            };

            (
                plan,
                assignment.map(|assignment| (true, assignment.clone())),
                score,
            )
        } else {
            let mut plan = current_solution.clone();
            let assignment: Option<Assignment> = current_solution
                .assignments
                .iter()
                .filter(|assignment| assignment.state != AssignmentState::Locked)
                .choose(&mut rng)
                .cloned();

            let score = match &assignment {
                Some(assignment) => {
                    plan.unassign_cooldown(assignment);
                    self.evaluate_solution(&plan)
                }
                None => NotNan::new(10000000000.0).unwrap(),
            };

            (
                plan,
                assignment.map(|assignment| (false, assignment)),
                score,
            )
        }
    }

    fn evaluate_solution(&self, solution: &Self::SolutionType) -> Self::ScoreType {
        let result = self.score_function.apply(solution);
        // this is to deal with problems relating to -0.0
        // (probably how it compares to 0.0)
        if result.is_zero() {
            NotNan::zero()
        } else {
            // invert the result to make more sense, + is good, - is bad
            -result
        }
    }
}
