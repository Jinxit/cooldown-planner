use crate::api::ui_assignment::{UiAssignment, UiAssignmentState};
use crate::api::ui_character::UiCharacter;
use crate::context::with_workers;
use crate::reactive::memo::Memoize;
use fight_domain::{Attack, AttackUuid, Character, Lookup, Spell, SpellUuid};
use futures_channel::oneshot;
use futures_util::SinkExt;
use itertools::Itertools;
use leptos::*;
use leptos_use::{signal_throttled, use_throttle_fn, use_throttle_fn_with_arg, watch_throttled};
use leptos_workers::executors::{AbortHandle, PoolExecutor};
use leptos_workers::workers;
use optimize_worker::{OptimizeRequest, OptimizeWorkerCallback};
use optimizer::Assignment;
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;

use super::ui_state::UiState;

pub fn use_optimizer() {
    let ui_state = expect_context::<UiState>();

    let request = {
        let request_id = Rc::new(RefCell::new(0_usize));
        create_memo(move |_| {
            let mut request_id = request_id.borrow_mut();
            *request_id += 1;
            let next_request_id = *request_id;
            warn!("next_request_id {}", next_request_id);
            OptimizeRequest {
                request_id: next_request_id,
                characters: ui_state.characters(),
                attacks: ui_state.attacks(),
                initial_assignments: ui_state.locked_assignments(),
            }
        })
    };

    let response_id = Rc::new(RefCell::new(0_usize));
    let throttled_request = signal_throttled(request, MaybeSignal::Static(1000.0));

    create_effect(
        move |prev_abort_handle: Option<AbortHandle<OptimizeWorkerCallback>>| {
            ui_state.set_planning(true);
            let request: OptimizeRequest = throttled_request.get();
            let request_id = request.request_id;
            *response_id.borrow_mut() = request_id;
            let response_id = response_id.clone();
            with_workers(move |workers /* PoolExecutor */| {
                let (new_abort_handle, future) = workers
                    .stream_callback(request, move |response| {
                        // untracked because a new request has already been triggered, this output should just be abandoned
                        if response.request_id >= *response_id.borrow() {
                            warn!("updating for request_id {}", response.request_id);
                            ui_state.update_assignment_suggestions(response.assignments);
                        }
                    })
                    .expect("worker creation failed");

                if let Some(abort_handle) = prev_abort_handle {
                    abort_handle.abort();
                }
                spawn_local(async move {
                    future.await;
                    ui_state.set_planning(false);
                    warn!("done with request_id {}", request_id);
                });
                new_abort_handle
            })
        },
    );
}

mod future {
    use futures_channel::oneshot;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use wasm_bindgen::UnwrapThrowExt;

    pub fn request_animation_frame() -> impl Future {
        RequestAnimationFrameFuture::new()
    }

    #[derive(Debug)]
    #[must_use = "futures do nothing unless polled or spawned"]
    struct RequestAnimationFrameFuture {
        rx: oneshot::Receiver<()>,
    }

    impl RequestAnimationFrameFuture {
        fn new() -> RequestAnimationFrameFuture {
            let (tx, rx) = oneshot::channel();
            leptos::request_animation_frame(move || {
                let _ = tx.send(());
            });

            Self { rx }
        }
    }

    impl Future for RequestAnimationFrameFuture {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
            Future::poll(Pin::new(&mut self.rx), cx).map(|t| t.unwrap_throw())
        }
    }
}
