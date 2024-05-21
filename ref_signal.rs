use std::rc::Rc;

use leptos::prelude::*;

#[derive(Clone)]
pub struct RefSignal<T> {
    inner: Rc<InnerRefSignal<T>>,
}

pub struct InnerRefSignal<T> {
    f: Box<dyn for<'a> Fn(bool, Box<dyn FnOnce(&T) + 'a>)>,
}

impl<Target> RefSignal<Target> {
    pub fn new<GetSignal, Original, Key>(
        cx: Scope,
        signal: GetSignal,
        getter: impl Fn(&Original) -> &Target + Copy + 'static,
        key: impl Fn(&Target) -> Key + 'static,
    ) -> Self
    where
        GetSignal: SignalWith<Original> + SignalWithUntracked<Original> + Clone + 'static,
        Key: PartialEq + 'static,
    {
        let signal_tmp = signal.clone();
        let memo = Memo::new(cx, move |_| {
            signal_tmp.with(|value| {
                let value = getter(value);
                key(value)
            })
        });

        let f: Box<dyn for<'a> Fn(bool, Box<dyn FnOnce(&Target) + 'a>)> =
            Box::new(move |track, f| {
                if track {
                    memo.track();
                }

                signal.try_with_untracked(move |value| {
                    f(getter(value));
                });
            });

        Self {
            inner: Rc::new(InnerRefSignal { f }),
        }
    }
}

impl<T> SignalWith<T> for RefSignal<T> {
    fn with<O>(&self, f: impl FnOnce(&T) -> O) -> O {
        let ret = Rc::new(std::cell::Cell::new(None));

        let ret2 = ret.clone();
        (self.inner.f)(
            true,
            Box::new(move |value| {
                ret2.set(Some(f(value)));
            }),
        );

        ret.take().unwrap()
    }

    fn try_with<O>(&self, f: impl FnOnce(&T) -> O) -> Option<O> {
        let ret = Rc::new(std::cell::Cell::new(None));

        let ret2 = ret.clone();
        (self.inner.f)(
            true,
            Box::new(move |value| {
                ret2.set(Some(f(value)));
            }),
        );

        ret.take()
    }
}

impl<T> SignalWithUntracked<T> for RefSignal<T> {
    fn with_untracked<O>(&self, f: impl FnOnce(&T) -> O) -> O {
        let ret = Rc::new(std::cell::Cell::new(None));

        let ret2 = ret.clone();
        (self.inner.f)(
            false,
            Box::new(move |value| {
                ret2.set(Some(f(value)));
            }),
        );

        ret.take().unwrap()
    }

    fn try_with_untracked<O>(&self, f: impl FnOnce(&T) -> O) -> Option<O> {
        let ret = Rc::new(std::cell::Cell::new(None));

        let ret2 = ret.clone();
        (self.inner.f)(
            false,
            Box::new(move |value| {
                ret2.set(Some(f(value)));
            }),
        );

        ret.take()
    }
}

impl<T: Clone> SignalGet<T> for RefSignal<T> {
    fn get(&self) -> T {
        self.with(Clone::clone)
    }

    fn try_get(&self) -> Option<T> {
        self.try_with(Clone::clone)
    }
}

impl<T: Clone> SignalGetUntracked<T> for RefSignal<T> {
    fn get_untracked(&self) -> T {
        self.with_untracked(Clone::clone)
    }

    fn try_get_untracked(&self) -> Option<T> {
        self.try_with_untracked(Clone::clone)
    }
}
