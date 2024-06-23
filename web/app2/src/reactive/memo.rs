use std::ops::Deref;

use leptos::*;

pub trait Memoize<U: Send + Sync> {
    fn memo(&self) -> Memo<U>;
}

impl<T, U> Memoize<U> for T
where
    T: SignalGet<Value = U> + Clone + Send + Sync + 'static,
    U: Clone + PartialEq + Send + Sync + 'static,
{
    fn memo(&self) -> Memo<U> {
        let s = self.clone();
        Memo::new(move |_| s.get())
    }
}

pub trait SetMemo<T> {
    fn set_memo(&self, new_value: T);
}

impl<S, T> SetMemo<T> for S
where
    S: SignalWithUntracked<Value = T> + SignalSet<Value = T>,
    T: Deref + PartialEq,
    <T as Deref>::Target: PartialEq<T>,
{
    fn set_memo(&self, new_value: T) {
        if self.with_untracked(|old_value| old_value != &new_value) {
            self.set(new_value);
        }
    }
}
