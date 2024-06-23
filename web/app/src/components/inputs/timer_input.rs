use fight_domain::FromMinutesSeconds;
use leptos::prelude::*;
use std::str::FromStr;
use std::time::Duration;
use crate::components::inputs::validated_input::ValidatedInput;

#[component]
pub fn TimerInput(
    #[prop(into)] label: String,
    #[prop(into)] set_value: WriteSignal<Duration>,
    #[prop(into)] initial_value: Duration,
    #[prop(into, optional)] min_value: Option<MaybeSignal<Duration>>,
    #[prop(into, optional)] max_value: Option<MaybeSignal<Duration>>,
) -> impl IntoView {
    let min_memo = min_value.map(|min_value| Memo::new(move |_| min_value.get()));
    let max_memo = max_value.map(|max_value| Memo::new(move |_| max_value.get()));
    let try_parse = move |s: &str| {
        s.split_once(':')
            .filter(|(minutes, seconds)| minutes.len() <= 2 && seconds.len() <= 2)
            .and_then(|(minutes, seconds)| {
                let minutes = u64::from_str(minutes).ok();
                let seconds = u64::from_str(seconds).ok();
                minutes.zip(seconds)
            })
            .filter(|(_, seconds)| *seconds < 60)
            .map(|(minutes, seconds)| Duration::mm_ss(minutes, seconds))
            .or_else(|| u64::from_str(s).ok().map(Duration::from_secs))
            .filter(|duration| duration.as_secs() / 60 < 100)
            .filter(|duration| {
                min_memo
                    .map(|min_memo| duration > &min_memo.get())
                    .unwrap_or(true)
            })
            .filter(|duration| {
                max_memo
                    .map(|max_memo| duration < &max_memo.get())
                    .unwrap_or(true)
            })
    };
    let reformat = |duration: &Duration| {
        let total_seconds = duration.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{minutes:0>2}:{seconds:0>2}")
    };
    view! {
        <ValidatedInput
            set_result=set_value
            placeholder="00:00"
            initial_value
            try_parse
            reformat
            label
        />
    }
}
