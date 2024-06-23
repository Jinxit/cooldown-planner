use leptos::html::Div;
use leptos::prelude::*;

#[derive(Clone)]
pub struct AutocompleteItemRef {
    pub on_select: Callback<()>,
    pub node_ref: NodeRef<Div>,
}
