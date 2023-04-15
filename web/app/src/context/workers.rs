use leptos::*;
use leptos_workers::executors::PoolExecutor;
use optimize_worker::OptimizeWorkerCallback;

pub fn provide_workers_context() {
    let (workers, set_workers) = create_signal(None);
    create_effect(move |_| {
        let workers = PoolExecutor::<OptimizeWorkerCallback>::new(4).unwrap();
        set_workers(Some(workers));
    });
    provide_context(workers);
}

pub fn with_workers<U>(f: impl FnOnce(&PoolExecutor<OptimizeWorkerCallback>) -> U) -> U {
    expect_context::<Option<PoolExecutor<OptimizeWorkerCallback>>>()
        .as_ref()
        .map(f)
        .unwrap()
}
