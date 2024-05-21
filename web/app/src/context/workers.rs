use leptos::prelude::*;
use leptos_workers::executors::PoolExecutor;
use optimize_worker::OptimizeWorkerCallback;

pub fn provide_workers_context() {
    let (workers, set_workers) = signal::<Option<PoolExecutor<OptimizeWorkerCallback>>>(None);
    Effect::new(move |_| {
        let workers = PoolExecutor::<OptimizeWorkerCallback>::new(4).unwrap();
        set_workers(Some(workers));
    });
    provide_context(workers);
}

pub fn with_workers<U>(f: impl FnOnce(&PoolExecutor<OptimizeWorkerCallback>) -> U) -> U {
    use_context::<Option<PoolExecutor<OptimizeWorkerCallback>>>()
        .unwrap()
        .as_ref()
        .map(f)
        .unwrap()
}
