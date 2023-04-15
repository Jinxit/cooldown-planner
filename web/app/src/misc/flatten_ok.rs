pub trait FlattenOk {
    type Output;

    fn flatten_ok(self) -> Option<Self::Output>;
}

impl<T, E> FlattenOk for Result<Option<T>, E> {
    type Output = T;

    fn flatten_ok(self) -> Option<Self::Output> {
        self.ok().flatten()
    }
}

impl<T, E> FlattenOk for Option<Result<T, E>> {
    type Output = T;

    fn flatten_ok(self) -> Option<Self::Output> {
        self.and_then(Result::ok)
    }
}