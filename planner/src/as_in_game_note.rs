use std::fmt::{Display, Formatter};

use fight_domain::Identifier;

pub trait AsInGameNote {
    fn in_game_note(&self) -> InGameNoteFormat;
}

pub struct InGameNoteFormat(String);

impl Display for InGameNoteFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsInGameNote for Identifier {
    fn in_game_note(&self) -> InGameNoteFormat {
        match self {
            Identifier::Spell(id) => InGameNoteFormat(format!("{{spell:{id}}}")),
            Identifier::Icon(_, file_data_id) => {
                InGameNoteFormat(format!("{{icon:{file_data_id}}}"))
            }
            Identifier::Marker(marker) => InGameNoteFormat(format!("{{rt{}}}", *marker as u8)),
            Identifier::Text(text) => InGameNoteFormat(text.clone()),
        }
    }
}
