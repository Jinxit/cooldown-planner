use fight_domain::{Lookup, Spell};

pub mod death_knight;
pub mod demon_hunter;
pub mod druid_restoration;
pub mod evoker_preservation;
pub mod general;
pub mod monk_mistweaver;
pub mod paladin_holy;
pub mod priest_discipline;
pub mod priest_holy;
pub mod priest_shadow;
pub mod shaman_restoration;
pub mod warrior;

pub fn spells_for_spec(class: &str, spec: &str) -> Lookup<Spell> {
    match (class, spec) {
        ("Death Knight", _) => death_knight::spells(),
        ("Demon Hunter", _) => demon_hunter::spells(),
        ("Druid", "Restoration") => druid_restoration::spells(),
        ("Evoker", "Preservation") => evoker_preservation::spells(),
        ("Monk", "Mistweaver") => monk_mistweaver::spells(),
        ("Paladin", "Holy") => paladin_holy::spells(),
        ("Priest", "Discipline") => priest_discipline::spells(),
        ("Priest", "Holy") => priest_holy::spells(),
        ("Priest", "Shadow") => priest_shadow::spells(),
        ("Shaman", "Restoration") => shaman_restoration::spells(),
        ("Warrior", _) => warrior::spells(),
        _ => Lookup::default(),
    }
}
