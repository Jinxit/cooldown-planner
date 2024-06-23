use std::fmt::{Display, Formatter};

use leptos::*;

use i18n::LocalizedString;

use crate::context::PlannerContext;

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

/*
impl IntoView for LocalizedStringView {
    fn into_view(self) -> View {
        view! {
            <Suspense>
                {self.to_string()}
            </Suspense>
        }
    }
}
 */

impl Display for LocalizedStringView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for LocalizedStringView {
    fn as_ref(&self) -> &str {
        let planner_context = use_context::<PlannerContext>().unwrap();
        let locale = planner_context.locale.get();
        self.0.get(locale)
    }
}
