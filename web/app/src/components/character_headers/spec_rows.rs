use leptos::prelude::*;
use planner::PlannerCharacter;
use crate::components::character_headers::spec_rows_normal::SpecRowsNormal;
use crate::components::character_headers::spec_rows_picking::SpecRowsPicking;

#[component]
pub fn SpecRows(character: PlannerCharacter) -> impl IntoView {
    // if there is no class (or spec??), instantly go into picking mode
    let picking = RwSignal::new(character.class.is_none() || character.spec.is_none());

    let character_class = character.class.clone();
    let character_spec = character.spec.clone();
    view! {
        {
            let character_class = character.class.clone();
            move || (!picking.get())
                .then(|| {
                    character_class.clone()
                })
                .flatten()
                .map(|character_class| view! {
                    <SpecRowsNormal
                        picking=picking
                        character_class
                        character_spec=character.spec.clone()
                    />
                })
        }
        <div
            class=("invisible", Signal::derive(move || {
                !picking.get()
            }))
            class=("-z-50", Signal::derive(move || {
                !picking.get()
            }))
        >
            <SpecRowsPicking
                picking=picking
                character_uuid=character.uuid
                character_class=character_class.clone()
                character_spec=character_spec.clone()
            />
        </div>
    }
}
