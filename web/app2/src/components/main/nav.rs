use leptos::*;

use crate::context::PlannerContext;
use crate::serverfns::character_avatar;

#[slot]
pub struct Tab {
    tab_header: TabHeader,
    tab_body: TabBody,
}

#[slot]
pub struct TabHeader {
    children: Children,
}

#[slot]
pub struct TabBody {
    children: ChildrenFn,
}

#[component]
pub fn Nav(#[prop(default=vec![])] tab: Vec<Tab>, tab_open: RwSignal<bool>) -> impl IntoView {
    let (headers, bodies): (Vec<TabHeader>, Vec<TabBody>) =
        tab.into_iter().map(|t| (t.tab_header, t.tab_body)).unzip();
    let active_tab_index = RwSignal::new(None);

    view! {
        <NavBar tabs=headers active_tab_index=active_tab_index tab_open=tab_open/>
        <NavTabBody active=Memo::new(move |_| {
            active_tab_index().is_some() && tab_open()
        })>
            {move || active_tab_index().and_then(|i| bodies.get(i)).map(|b| (b.children)())}
        </NavTabBody>
    }
}

#[component]
fn NavBar(
    tabs: Vec<TabHeader>,
    active_tab_index: RwSignal<Option<usize>>,
    tab_open: RwSignal<bool>,
) -> impl IntoView {
    let planner_context = use_context::<PlannerContext>().unwrap();
    let user = planner_context.user.clone();
    let user = Signal::derive(move || user.get());
    let name = Signal::derive(move || user.get().map(|u| u.name));
    let realm = Signal::derive(move || user.get().map(|u| u.realm));
    let guild = Signal::derive(move || user.get().and_then(|u| u.guild));
    let region = planner_context.region.clone();
    let avatar = Resource::new(
        move || (name(), realm(), region.get()),
        move |(name, realm, region)| async move {
            character_avatar(name?, realm?.slug, region).await.ok()
        },
    );
    view! {
        <nav class="border-b-2 border-slate-900 bg-gradient-to-b from-slate-600 to-slate-700">
            <div class="relative flex items-baseline">
                <div class="m-2 hidden flex-1 flex-grow self-end sm:flex">
                    <div class="-mb-[2px] flex space-x-4">
                        {tabs
                            .into_iter()
                            .enumerate()
                            .map(|(index, tab)| {
                                view! {
                                    <NavTab
                                        active=Memo::new(move |_| {
                                            Some(index) == active_tab_index() && tab_open()
                                        })

                                        on:click=move |_| {
                                            if Some(index) == active_tab_index() && tab_open() {
                                                tab_open.set(false);
                                            } else {
                                                active_tab_index.set(Some(index));
                                                tab_open.set(true);
                                            }
                                        }
                                    >

                                        {(tab.children)()}
                                    </NavTab>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </div>
                <div class="m-2 space-y-2 self-center sm:hidden">
                    <div class="h-0.5 w-8 bg-gray-300"></div>
                    <div class="h-0.5 w-8 bg-gray-300"></div>
                    <div class="h-0.5 w-8 bg-gray-300"></div>
                </div>
                <div class="block self-center">
                    <h1 class="whitespace-nowrap font-title text-2xl">"Cooldown Planner"</h1>
                </div>
                <div class="flex flex-1 flex-grow justify-end self-center">
                    <div class="flex text-xs font-medium">
                        <div class="hidden flex-col justify-center text-end sm:flex">
                            <span class="overflow-x-visible whitespace-nowrap font-bold">
                                {name}
                            </span>
                            <span class="overflow-x-visible whitespace-nowrap">{guild}</span>
                        </div>
                        <img
                            class="m-2 w-8 h-8 -scale-x-100 rounded-full border-2 border-slate-900 bg-slate-800"
                            alt=move || {
                                avatar.get().flatten().map(|_| "Character Picture")
                            }

                            src=move || avatar.get().flatten().map(|u| u.to_string())
                        />
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn NavTab(#[prop(into)] active: Signal<bool>, children: Children) -> impl IntoView {
    view! {
        <a
            href="#"
            class="-mb-2 whitespace-nowrap rounded-t-md border-2 border-transparent \
            px-3 py-2 pb-4 text-sm font-medium hover:text-white cursor-pointer \
            focus-visible:outline focus-visible:outline-1 focus-visible:outline-offset-2 focus-visible:outline-slate-300"
            class=("border-x-slate-900", active)
            class=("border-t-slate-900", active)
            class=("bg-slate-800", active)
            class=("text-white", active)
            class=("hover:border-b-slate-900", move || !active())
            class=("hover:bg-slate-700", move || !active())
        >
            {children()}
        </a>
    }
}

#[component]
fn NavTabBody(#[prop(into)] active: Signal<bool>, children: Children) -> impl IntoView {
    view! {
        <div
            class="w-100 relative flex items-end justify-end overflow-hidden border-slate-900 transition-all"
            class=("border-b-2", active)
            class=("bg-slate-800", active)
            class=("shadow-md", active)
            class=("h-36", active)

            class=("h-0", move || !active())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn NavTabBodyBackground(#[prop(into)] image: Signal<(String, i32)>) -> impl IntoView {
    view! {
        <div class="absolute h-full w-full overflow-hidden">
            <div
                class="absolute right-0 -mr-[9%] h-full w-[55%] max-w-[38rem] overflow-hidden \
                bg-cover bg-no-repeat opacity-30 blur-sm brightness-0"
                style:background-position-y=move || format!("{}%", &image().1 - 2)
                style:background-image=move || format!("url('{}')", &image().0)
            ></div>
            <div
                class="absolute right-0 -mr-[10%] h-full w-[60%] max-w-[36rem] overflow-hidden \
                bg-cover bg-no-repeat opacity-60"
                style:background-position-y=move || format!("{}%", &image().1)
                style:background-image=move || format!("url('{}')", &image().0)
            ></div>
        </div>
    }
}
