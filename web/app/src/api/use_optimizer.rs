use std::future::Future;
use std::sync::{Arc, Mutex};

use futures_util::SinkExt;
use itertools::Itertools;
use leptos::prelude::*;
//use leptos_use::{signal_throttled, use_throttle_fn, use_throttle_fn_with_arg, watch_throttled};
use leptos_workers::executors::AbortHandle;
use tracing::{info, warn};
use wasm_bindgen_futures::spawn_local;
use fight_domain::{Character, Lookup};

use optimize_worker::{OptimizeRequest, OptimizeWorkerCallback};

use crate::context::{use_planner, with_workers};
use crate::reactive::memo::Memoize;


pub fn use_optimizer() {
    let planner = use_planner();
    let planning = RwSignal::new(false);

    let request = {
        let request_id = Arc::new(Mutex::new(0_usize));
        let characters: Memo<Lookup<Character>> = Memo::new(move |_| planner.read().characters().iter().cloned().map(Into::into).collect());
        let attacks = Memo::new(move |_| planner.read().attacks().clone());
        let locked_assignments = Memo::new(move |_| planner.read().locked_assignments().clone());
        Memo::new(move |_| {
            let mut request_id = request_id.lock().unwrap();
            *request_id += 1;
            let next_request_id = *request_id;
            warn!("next_request_id {}", next_request_id);
            OptimizeRequest {
                request_id: next_request_id,
                characters: characters.get(),
                attacks: attacks.get(),
                initial_assignments: locked_assignments.get(),
            }
        })
    };

    let response_id = Arc::new(Mutex::new(0_usize));
    let throttled_request = request; //signal_throttled(request, MaybeSignal::Static(1000.0));

    Effect::new(
        move |prev_abort_handle: Option<AbortHandle<OptimizeWorkerCallback>>| {
            planning.set(true);
            let request: OptimizeRequest = throttled_request.get();
            let request_id = request.request_id;
            {
                let mut response_id = response_id.lock().unwrap();
                *response_id = request_id;
            }
            let response_id = response_id.clone();

            with_workers(move |workers| {
                let (new_abort_handle, future) = workers
                    .stream_callback(request, move |response| {
                        // untracked because a new request has already been triggered, this output should just be abandoned
                        if response.request_id >= *response_id.lock().unwrap() {
                            planner.update(|planner| {
                                planner.replace_assignment_suggestions(response.assignments);
                            })
                        }
                    })
                    .expect("worker creation failed");

                if let Some(abort_handle) = prev_abort_handle {
                    abort_handle.abort();
                }
                spawn_local(async move {
                    future.await;
                    planning.set(false);
                    warn!("done with request_id {}", request_id);
                });
                new_abort_handle
            })
        },
    );
}
