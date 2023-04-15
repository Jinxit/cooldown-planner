use crate::{context::PlannerContext, reactive::blank_suspense::BlankSuspense};
use auto_battle_net::LocalizedString;
use leptos::*;
use std::fmt::{Display, Formatter};

pub trait LocalizedStringWithContext {
    fn localize(&self) -> LocalizedStringView;
}

impl LocalizedStringWithContext for LocalizedString {
    fn localize(&self) -> LocalizedStringView {
        LocalizedStringView(self.clone())
    }
}

#[derive(Clone)]
pub struct LocalizedStringView(LocalizedString);

impl IntoView for LocalizedStringView {
    fn into_view(self) -> View {
        view! {
            <BlankSuspense>
                {self.to_string()}
            </BlankSuspense>
        }
    }
}

impl Display for LocalizedStringView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for LocalizedStringView {
    fn as_ref(&self) -> &str {
        let planner_context = expect_context::<PlannerContext>();
        let locale = planner_context.locale.get();
        self.0.get(locale)
    }
}
