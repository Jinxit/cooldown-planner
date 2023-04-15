use leptos::*;

pub trait ResourceGetExt<T> {
    fn get(&self) -> Option<T>;
}

impl<S, T> ResourceGetExt<T> for Resource<S, T>
where
    S: Clone,
    T: Clone,
{
    fn get(&self) -> Option<T> {
        self.read()
    }
}

pub trait ResourceMapExt<T> {
    fn map<U>(&self, f: impl FnOnce(&T) -> U) -> Option<U>;
}

impl<S, T> ResourceMapExt<T> for Resource<S, T>
where
    S: Clone,
    T: Clone,
{
    fn map<U>(&self, f: impl FnOnce(&T) -> U) -> Option<U> {
        self.get().as_ref().map(f)
    }
}

pub trait ResourceAndThenExt<T, E>
where
    E: Clone,
{
    fn and_then<U>(&self, f: impl FnOnce(&T) -> U) -> Option<Result<U, E>>;
}

impl<S, T, E> ResourceAndThenExt<T, E> for Resource<S, Result<T, E>>
where
    S: Clone,
    T: Clone,
    E: Clone,
{
    fn and_then<U>(&self, f: impl FnOnce(&T) -> U) -> Option<Result<U, E>> {
        self.map(|data| data.as_ref().map(f).map_err(|e| e.clone()))
    }
}
