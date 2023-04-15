use leptos::{
    create_memo, create_signal, Memo, RwSignal, SignalGet, SignalSet, SignalWithUntracked,
    WriteSignal,
};

pub fn create_memo_signal<T>(value: T) -> (Memo<T>, WriteSignal<T>)
where
    T: Clone + PartialEq + 'static,
{
    let (value, set_value) = create_signal(value);
    let value_memo = create_memo(move |_| value.get());
    (value_memo, set_value)
}

pub trait Memoize<U> {
    fn memo(&self) -> Memo<U>;
}

impl<T, U> Memoize<U> for T
where
    T: SignalGet<U> + Clone + 'static,
    U: Clone + PartialEq + 'static,
{
    fn memo(&self) -> Memo<U> {
        let s = self.clone();
        create_memo(move |_| s.get())
    }
}

pub trait SetMemo<T> {
    fn set_memo(&self, new_value: T);
}

impl<T> SetMemo<T> for RwSignal<T>
where
    T: PartialEq,
{
    fn set_memo(&self, new_value: T) {
        if self.with_untracked(|old_value| old_value != &new_value) {
            self.set(new_value);
        }
    }
}
