extern crate alloc;
extern crate core;

use alloc::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::time::Duration;

use leptos_workers::worker;
use localsearch::optim::RelativeAnnealingOptimizer;
use localsearch::OptProgress;
use ordered_float::{Float, NotNan};
use serde::{Deserialize, Serialize};

use fight_domain::{Attack, Character, Lookup};
use optimizer::{Assignment, FightModel, Optimizer, Plan};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OptimizeRequest {
    pub request_id: usize,
    pub characters: Lookup<Character>,
    pub attacks: Lookup<Attack>,
    pub initial_assignments: Lookup<Assignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizeResponse {
    pub request_id: usize,
    pub assignments: Lookup<Assignment>,
    pub score: f64,
}

#[worker(OptimizeWorkerCallback)]
pub async fn optimize_worker_callback(
    request: OptimizeRequest,
    callback: impl Fn(OptimizeResponse),
) {
    let callback = Rc::new(callback);
    let best_so_far = Rc::new(RefCell::new(f64::neg_infinity()));
    callback(OptimizeResponse {
        request_id: request.request_id,
        assignments: request.initial_assignments.clone(),
        score: f64::neg_infinity(),
    });
    let optimizer = RelativeAnnealingOptimizer::new(500, 10, 500, 0.5);
    optimize(
        request.characters,
        request.attacks,
        request.initial_assignments,
        optimizer,
        {
            let callback = callback.clone();
            move |assignments, score| {
                if score > *best_so_far.borrow() {
                    let mut best = best_so_far.borrow_mut();
                    if score > *best {
                        *best = score;
                        callback(OptimizeResponse {
                            request_id: request.request_id,
                            assignments,
                            score,
                        });
                    }
                }
            }
        },
    );
}

#[worker(OptimizeWorkerFuture)]
pub async fn optimize_worker_future(request: OptimizeRequest) -> OptimizeResponse {
    let optimizer = RelativeAnnealingOptimizer::new(500, 500, 500, 0.5);
    let (assignments, score) = optimize(
        request.characters,
        request.attacks,
        request.initial_assignments,
        optimizer,
        move |_assignments, _score| {},
    );

    OptimizeResponse {
        request_id: request.request_id,
        assignments,
        score,
    }
}

#[worker(OptimizeWorkerChannel)]
pub async fn optimize_worker_channel(
    rx: leptos_workers::Receiver<OptimizeRequest>,
    tx: leptos_workers::Sender<OptimizeResponse>,
) {
    let tx = Arc::new(tx);
    while let Ok(request) = rx.recv_async().await {
        let best_so_far = Rc::new(RefCell::new(f64::neg_infinity()));
        let _ = tx.send(OptimizeResponse {
            request_id: request.request_id,
            assignments: request.initial_assignments.clone(),
            score: f64::neg_infinity(),
        });
        let optimizer = RelativeAnnealingOptimizer::new(500, 10, 500, 0.5);
        optimize(
            request.characters,
            request.attacks,
            request.initial_assignments,
            optimizer,
            {
                let tx = tx.clone();
                move |assignments, score| {
                    if score > *best_so_far.borrow() {
                        let mut best = best_so_far.borrow_mut();
                        if score > *best {
                            *best = score;
                            let _ = tx.send(OptimizeResponse {
                                request_id: request.request_id,
                                assignments,
                                score,
                            });
                        }
                    }
                }
            },
        );
    }
}

pub fn optimize(
    characters: Lookup<Character>,
    attacks: Lookup<Attack>,
    initial_assignments: Lookup<Assignment>,
    optimizer: impl Optimizer,
    callback: impl Fn(Lookup<Assignment>, f64),
) -> (Lookup<Assignment>, f64) {
    let score_function = {
        use optimizer::score_functions::*;
        COVER_ATTACKS * 10 + MAXIMIZE_HEALING
    };

    let initial_plan = Plan::new(characters, attacks, initial_assignments);
    let model = FightModel { score_function };

    let iterations = 100000;
    let time_limit = Duration::from_secs(3);

    let (plan, score) = optimizer.optimize(
        &model,
        Some(initial_plan.clone()),
        iterations,
        time_limit,
        Some(&|status: OptProgress<Plan, NotNan<f64>>| {
            callback(
                status.solution.borrow().assignments.clone(),
                -status.score.into_inner(),
            );
        }),
    );

    (plan.assignments, -score.into_inner())
}
