use leptos::html::Div;
use leptos::prelude::*;
use crate::components::autocomplete::autocomplete_context::AutocompleteContext;
use crate::components::autocomplete::autocomplete_id::AutocompleteId;
use crate::components::autocomplete::autocomplete_item_ref::AutocompleteItemRef;

#[component]
pub fn AutocompleteItem<IV>(
    #[prop(optional, into)] on_select: Option<Callback<()>>,
    children: impl Fn(Signal<bool>) -> IV + Send + 'static,
) -> impl IntoView where
    IV: IntoView + 'static,
{
    let context = use_context::<AutocompleteContext>().unwrap();
    let node_ref: NodeRef<Div> = NodeRef::new();
    let id = RwSignal::new(None);
    let on_select = on_select.unwrap_or_else(|| Callback::new(|_| ()));

    RenderEffect::new({
        let context = context.clone();
        let node_ref = node_ref.clone();
        let on_select = on_select.clone();
        move |_| {
            let new_id = AutocompleteId::new();
            id.set(Some(new_id.clone()));

            context.register_item(
                new_id,
                AutocompleteItemRef {
                    node_ref,
                    on_select: on_select.clone(),
                },
            );
        }
    });

    on_cleanup({
        let context = context.clone();
        move || {
            let Some(id) = id.try_get_untracked().flatten() else {
                return;
            };

            context.unregister_item(id);
        }
    });

    let is_highlighted = Signal::derive(move || id.get().is_some_and(|id| Some(id) == context.highlighted_item.get()));

    view! {
        <div
            node_ref=node_ref
            on:mousedown=move |ev| {
                if ev.button() != 0 {
                    return;
                }
                ev.prevent_default();
                context.active.set(false);
                on_select.call(());
            }
            on:mousemove=move |_| {
                context.active.set(true);
                let Some(id) = id.get() else { return };
                context.set_mouse_highlight(id.clone());
            }
            data-autocomplete-id=move || id.get().as_ref().map(|id| id.to_string())
            data-autocomplete-highlighted=move || is_highlighted.get().to_string()
        >
            {children(is_highlighted)}
        </div>
    }
}
