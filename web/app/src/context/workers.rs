use leptos::prelude::*;
use leptos_workers::executors::PoolExecutor;

use optimize_worker::OptimizeWorkerCallback;

pub fn provide_workers_context() {
    let (workers, set_workers) = arc_signal(None);
    RenderEffect::new(move |_| {
        warn!("starting workers");
        set_workers.maybe_update(|workers| {
            match workers {
                Some(workers) => { false },
                None => {
                    *workers = Some(PoolExecutor::<OptimizeWorkerCallback>::new(4).unwrap());
                    true
                }
            }
        });
    });
    provide_context(workers);
}

pub fn with_workers<U>(
    f: impl FnOnce(&PoolExecutor<OptimizeWorkerCallback>) -> U + Clone + Send + Sync + 'static,
) -> U {
    let workers =
        use_context::<ArcReadSignal<Option<PoolExecutor<OptimizeWorkerCallback>>>>().unwrap();

    workers.get().as_ref().map(f).unwrap()
}
