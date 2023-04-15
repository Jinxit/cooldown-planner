use crate::assignment::Assignment;
use crate::Plan;
use fight_domain::{Attack, AttackType};
use num_traits::identities::Zero;
use ordered_float::NotNan;
use std::cmp::max;
use std::collections::BTreeMap;
use std::ops::{Add, Mul};

type StaticScoreFunction = fn(&Plan) -> NotNan<f64>;
type BoxedScoreFunction = Box<dyn Fn(&Plan) -> NotNan<f64> + Send + Sync>;

pub enum ScoreFunction {
    Static(StaticScoreFunction),
    Boxed(BoxedScoreFunction),
}

impl ScoreFunction {
    pub fn apply(&self, plan: &Plan) -> NotNan<f64> {
        match self {
            ScoreFunction::Static(func) => func(plan),
            ScoreFunction::Boxed(func) => func(plan),
        }
    }
}

impl From<fn(&Plan) -> NotNan<f64>> for ScoreFunction {
    fn from(value: fn(&Plan) -> NotNan<f64>) -> Self {
        ScoreFunction::Static(value)
    }
}

impl<T: Into<NotNan<f64>> + Copy> Mul<T> for ScoreFunction {
    type Output = ScoreFunction;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        ScoreFunction::Boxed(Box::new(move |plan: &Plan| self.apply(plan) * rhs))
    }
}

impl Add<ScoreFunction> for ScoreFunction {
    type Output = ScoreFunction;

    fn add(self, rhs: ScoreFunction) -> Self::Output {
        ScoreFunction::Boxed(Box::new(move |plan: &Plan| {
            self.apply(plan) + rhs.apply(plan)
        }))
    }
}

fn attack_power(attack: &Attack) -> NotNan<f64> {
    use AttackType::*;
    let power_mapping: BTreeMap<AttackType, f64> = [
        (RaidDamage, 60.0),        // Raid damage
        (RaidDamageStacked, 30.0), // Raid damage, stacked raid
        (RotDamage, 30.0),         // Rot damage
        (Movement, 0.0),           // Movement
        (Dispels, 0.0),            // Dispels
        (Debuffs, 0.0),            // Debuff related
        (Adds, 10.0),              // Adds?
        (Generic, 0.0),            // Generic
    ]
    .into_iter()
    .collect();

    attack.power * power_mapping.get(&attack.r#type).unwrap()
}

pub const COVER_ATTACKS: ScoreFunction = ScoreFunction::Static(|plan: &Plan| {
    let mut total_score: NotNan<f64> = NotNan::zero();
    for attack in &plan.attacks {
        let assigned_cooldowns: Vec<&Assignment> = plan
            .assignments
            .iter()
            .filter(|assignment| assignment.attack == attack.uuid)
            .collect();
        let summed_healing_power: NotNan<f64> = assigned_cooldowns
            .iter()
            .map(|assignment| {
                let spell = plan
                    .characters
                    .get(&assignment.character)
                    .unwrap()
                    .spells
                    .get(&assignment.spell)
                    .unwrap();
                spell.power * NotNan::from(spell.cooldown.as_secs())
            })
            .sum();

        total_score -= max(attack_power(attack) - summed_healing_power, NotNan::zero());
    }

    total_score
});

pub const MAXIMIZE_HEALING: ScoreFunction = ScoreFunction::Static(|plan: &Plan| {
    let assigned_cooldowns: Vec<&Assignment> = plan.assignments.iter().collect();
    let summed_healing_power: NotNan<f64> = assigned_cooldowns
        .iter()
        .map(|assignment| {
            let spell = plan
                .characters
                .get(&assignment.character)
                .unwrap()
                .spells
                .get(&assignment.spell)
                .unwrap();
            spell.power * NotNan::from(spell.cooldown.as_secs())
        })
        .sum();

    summed_healing_power
});
