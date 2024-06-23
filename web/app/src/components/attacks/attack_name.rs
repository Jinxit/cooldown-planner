use leptos::prelude::*;
use fight_domain::AttackUuid;

#[component]
pub fn AttackName(uuid: AttackUuid, name: String) -> impl IntoView {
    let column = "attack_name";
    let row = format!("attack_{}", uuid);

    view! {
        <div
            class="justify-center items-center"
            style:grid-column-start=column
            style:grid-row-start=row
        >
            {name}
        </div>
    }
}