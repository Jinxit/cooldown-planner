use std::cmp::min;
use std::collections::HashMap;
use leptos::html::{Div, Input};
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, ScrollIntoViewOptions, ScrollLogicalPosition};
use crate::components::autocomplete::autocomplete_id::AutocompleteId;
use crate::components::autocomplete::autocomplete_item_ref::AutocompleteItemRef;

#[derive(Clone)]
pub struct AutocompleteContext {
    pub active: RwSignal<bool>,

    pub input_node_ref: NodeRef<Input>,
    pub items_node_ref: NodeRef<Div>,

    pub highlighted_item: Signal<Option<AutocompleteId>>,
    pub ordered_ids: Signal<Vec<AutocompleteId>>,
    pub should_show_empty: Signal<bool>,
    pub on_blur: Option<Callback<()>>,

    keyboard_highlighted_item: RwSignal<Option<AutocompleteId>>,
    mouse_highlighted_item: RwSignal<Option<AutocompleteId>>,
    item_refs: RwSignal<HashMap<AutocompleteId, AutocompleteItemRef>>,
}

pub fn provide_autocomplete_context(
    on_blur: Option<Callback<()>>,
) -> AutocompleteContext {
    if use_context::<AutocompleteContext>().is_some() {
        tracing::warn!("Nested Autocomplete is not supported");
    }
    let active = RwSignal::new(false);

    let input_node_ref: NodeRef<Input> = NodeRef::new();
    let items_node_ref: NodeRef<Div> = NodeRef::new();

    let keyboard_highlighted_item = RwSignal::new(None);
    let mouse_highlighted_item = RwSignal::new(None);
    let highlighted_item = Signal::derive(move || {
        mouse_highlighted_item
            .get()
            .or_else(|| keyboard_highlighted_item.get())
    });

    let ordered_ids = Signal::derive(move || {
        let Some(items_ref) = items_node_ref.get() else {
            return vec![];
        };

        if items_ref.is_null() {
            return vec![];
        }

        let Ok(ordered_nodes) =
            items_ref.query_selector_all("[data-autocomplete-id]")
            else {
                return vec![];
            };
        (0..ordered_nodes.length())
            .filter_map(|i| ordered_nodes.item(i))
            .filter_map(|node| node.dyn_into::<Element>().ok())
            .filter_map(|node| node.get_attribute("data-autocomplete-id"))
            .map(|id| AutocompleteId::from(id.as_str()))
            .collect()
    });
    let should_show_empty = Signal::derive(move || {
        active.get()
            && ordered_ids.get().is_empty()
    });
    let item_refs = RwSignal::new(HashMap::new());

    let context = AutocompleteContext {
        active,

        input_node_ref,
        items_node_ref,

        highlighted_item,
        ordered_ids,
        should_show_empty,
        on_blur,

        keyboard_highlighted_item,
        mouse_highlighted_item,
        item_refs,
    };
    provide_context(context.clone());
    context
}

impl AutocompleteContext {
    pub fn register_item(
        &self,
        key: AutocompleteId,
        item_ref: AutocompleteItemRef,
    ) {
        self.item_refs.update(|v| {
            v.insert(key, item_ref);
        });
        self.move_keyboard_highlight_to_first_option();
    }

    pub fn unregister_item(&self, key: AutocompleteId) {
        self.item_refs.update(|v| {
            v.remove(&key);
        });
        self.move_keyboard_highlight_to_first_option();
    }

    pub fn select_highlighted(&self) -> bool {
        if let Some(id) = self.highlighted_item.get() {
            if let Some(item_ref) = self.item_refs.read().get(&id) {
                item_ref.on_select.call(());
                return true;
            }
        }
        false
    }

    pub fn move_keyboard_highlight_to_first_option(&self) {
        let s = self.clone();
        // delay by a frame to allow the items to render
        request_animation_frame(move || {
            // as this is now a frame late, the ids might be gone (if we were just unmounted)
            // that's ok, just do nothing if the signal is disposed
            if let Some(id) = s.ordered_ids.try_get_untracked().and_then(|ids| ids.first().cloned()) {
                s.keyboard_highlighted_item.set(Some(id.clone()));
                s.mouse_highlighted_item.set(None);
                s.active.set(true);
            }
        });
    }

    pub fn move_keyboard_highlight(&self, delta: isize) {
        let current_index = self.highlighted_item.get().and_then(|id| {
            self.ordered_ids.get().iter().position(|k| k == &id)
        });
        let new_index = if let Some(current_index) = current_index {
            min(
                self.ordered_ids.get().len() - 1,
                current_index.saturating_add_signed(delta),
            )
        } else {
            0
        };
        if let Some(id) = self.ordered_ids.get().get(new_index) {
            self.keyboard_highlighted_item.set(Some(id.clone()));
            self.mouse_highlighted_item.set(None);
            let Some(node_ref) = self.item_refs.read().get(&id).and_then(|i| i.node_ref.get()) else {
                return;
            };

            node_ref.scroll_into_view_with_scroll_into_view_options(ScrollIntoViewOptions::new().block(ScrollLogicalPosition::Nearest));
        }
    }
    
    pub fn set_keyboard_highlight_to_mouse_highlight(&self) {
        if let Some(mouse_highlighted) = self.mouse_highlighted_item.get() {
            self.keyboard_highlighted_item.set(Some(mouse_highlighted));
        }
        self.mouse_highlighted_item.set(None);
    }

    pub fn set_mouse_highlight(&self, id: AutocompleteId) {
        self.mouse_highlighted_item.set(Some(id));
    }
}

pub fn use_autocomplete_context() -> AutocompleteContext {
    use_context::<AutocompleteContext>().expect("Autocomplete* compound components can only be used inside an Autocomplete root")
}
