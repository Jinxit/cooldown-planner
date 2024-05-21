use fight_domain::Identifier;
use leptos::prelude::*;

pub fn icon_url(identifier: Identifier) -> Option<String> {
    match identifier {
        Identifier::Spell(id) => Some(format!("/spell_icon/{id}")),
        Identifier::Icon(path, _) => Some(format!("/icon/{path}")),
        Identifier::Marker(marker) => Some(format!("/raid_marker/{}.png", marker as u8)),
        Identifier::Text(text) => None,
    }
}
