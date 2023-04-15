use leptos::{IntoSignal, Signal, SignalWith};

pub trait Map<T, U> {
    fn map<F>(&self, f: F) -> Signal<U>
    where
        F: Fn(&T) -> U + 'static;
}

impl<V, T, U> Map<T, U> for V
where
    V: SignalWith<T> + Clone + 'static,
    U: Clone + PartialEq + 'static,
{
    fn map<F>(&self, f: F) -> Signal<U>
    where
        F: Fn(&T) -> U + 'static,
    {
        let s = self.clone();
        (move || s.with(|v| f(v))).derive_signal()
    }
}
