use std::time::Duration;

use localsearch::optim::{
    EpsilonGreedyOptimizer, HillClimbingOptimizer, LocalSearchOptimizer,
    LogisticAnnealingOptimizer, RelativeAnnealingOptimizer,
};
use localsearch::{OptCallbackFn, OptModel};
use ordered_float::NotNan;

use crate::optimizers::SimulatedAnnealingOptimizer;

pub trait Optimizer {
    fn optimize<M, F>(
        &self,
        model: &M,
        initial_state: Option<M::SolutionType>,
        n_iter: usize,
        time_limit: Duration,
        callback: Option<&F>,
    ) -> (M::SolutionType, M::ScoreType)
    where
        M: OptModel<ScoreType = NotNan<f64>> + Sync + Send,
        F: OptCallbackFn<M::SolutionType, M::ScoreType>;
}

impl Optimizer for EpsilonGreedyOptimizer {
    fn optimize<M, F>(
        &self,
        model: &M,
        initial_state: Option<M::SolutionType>,
        n_iter: usize,
        time_limit: Duration,
        callback: Option<&F>,
    ) -> (M::SolutionType, M::ScoreType)
    where
        M: OptModel<ScoreType = NotNan<f64>> + Sync + Send,
        F: OptCallbackFn<M::SolutionType, M::ScoreType>,
    {
        let result = <EpsilonGreedyOptimizer as LocalSearchOptimizer<M>>::optimize(
            self,
            model,
            initial_state,
            n_iter,
            time_limit,
            callback,
            (),
        );
        (result.0, result.1)
    }
}

impl Optimizer for HillClimbingOptimizer {
    fn optimize<M, F>(
        &self,
        model: &M,
        initial_state: Option<M::SolutionType>,
        n_iter: usize,
        time_limit: Duration,
        callback: Option<&F>,
    ) -> (M::SolutionType, M::ScoreType)
    where
        M: OptModel<ScoreType = NotNan<f64>> + Sync + Send,
        F: OptCallbackFn<M::SolutionType, M::ScoreType>,
    {
        let result = <HillClimbingOptimizer as LocalSearchOptimizer<M>>::optimize(
            self,
            model,
            initial_state,
            n_iter,
            time_limit,
            callback,
            (),
        );
        (result.0, result.1)
    }
}

impl Optimizer for LogisticAnnealingOptimizer {
    fn optimize<M, F>(
        &self,
        model: &M,
        initial_state: Option<M::SolutionType>,
        n_iter: usize,
        time_limit: Duration,
        callback: Option<&F>,
    ) -> (M::SolutionType, M::ScoreType)
    where
        M: OptModel<ScoreType = NotNan<f64>> + Sync + Send,
        F: OptCallbackFn<M::SolutionType, M::ScoreType>,
    {
        let result = <LogisticAnnealingOptimizer as LocalSearchOptimizer<M>>::optimize(
            self,
            model,
            initial_state,
            n_iter,
            time_limit,
            callback,
            (),
        );
        (result.0, result.1)
    }
}

impl Optimizer for RelativeAnnealingOptimizer {
    fn optimize<M, F>(
        &self,
        model: &M,
        initial_state: Option<M::SolutionType>,
        n_iter: usize,
        time_limit: Duration,
        callback: Option<&F>,
    ) -> (M::SolutionType, M::ScoreType)
    where
        M: OptModel<ScoreType = NotNan<f64>> + Sync + Send,
        F: OptCallbackFn<M::SolutionType, M::ScoreType>,
    {
        let result = <RelativeAnnealingOptimizer as LocalSearchOptimizer<M>>::optimize(
            self,
            model,
            initial_state,
            n_iter,
            time_limit,
            callback,
            (),
        );
        (result.0, result.1)
    }
}

impl Optimizer for SimulatedAnnealingOptimizer {
    fn optimize<M, F>(
        &self,
        model: &M,
        initial_state: Option<M::SolutionType>,
        n_iter: usize,
        time_limit: Duration,
        callback: Option<&F>,
    ) -> (M::SolutionType, M::ScoreType)
    where
        M: OptModel<ScoreType = NotNan<f64>> + Sync + Send,
        F: OptCallbackFn<M::SolutionType, M::ScoreType>,
    {
        let result = SimulatedAnnealingOptimizer::optimize(
            self,
            model,
            initial_state,
            n_iter,
            time_limit,
            callback,
        );
        (result.0, result.1)
    }
}
