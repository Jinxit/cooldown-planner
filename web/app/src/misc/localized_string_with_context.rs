use std::fmt::{Display, Formatter};

use leptos::prelude::*;
use leptos::tachys::view::any_view::AnyView;

use i18n::{Locale, LocalizedString};
use crate::context::UserContext;

pub trait LocalizedStringWithContext {
    type Target;
    fn localize(&self, user_context: UserContext) -> Self::Target;
}

impl LocalizedStringWithContext for LocalizedString {
    type Target = impl IntoView;

    fn localize(&self, user_context: UserContext) -> Self::Target {
        let s = self.clone();
        move || {
            let s = s.clone();
            view! {
                <Suspense>
                    {
                        let s = s.clone();
                        Suspend(async move {
                            let locale = user_context.locale.await;
                            s.get(locale).to_owned()
                        })
                    }
                </Suspense>
            }
        }
    }
}

impl LocalizedStringWithContext for Option<LocalizedString> {
    type Target = Option<<LocalizedString as LocalizedStringWithContext>::Target>;

    fn localize(&self, user_context: UserContext) -> Self::Target {
        self.as_ref().map(|s| s.localize(user_context))
    }
}
