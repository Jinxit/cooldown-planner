use crate::optimizers::SimulatedAnnealingOptimizer;
use localsearch::optim::{
    EpsilonGreedyOptimizer, HillClimbingOptimizer, LogisticAnnealingOptimizer,
    RelativeAnnealingOptimizer,
};
use localsearch::{OptCallbackFn, OptModel};
use ordered_float::NotNan;

pub trait Optimizer {
    fn optimize<M, F>(
        &self,
        model: &M,
        initial_state: Option<M::StateType>,
        n_iter: usize,
        callback: Option<&F>,
    ) -> (M::StateType, M::ScoreType)
    where
        M: OptModel<ScoreType = NotNan<f64>> + Sync + Send,
        F: OptCallbackFn<M::StateType, M::ScoreType>;
}

impl Optimizer for EpsilonGreedyOptimizer {
    fn optimize<M, F>(
        &self,
        model: &M,
        initial_state: Option<M::StateType>,
        n_iter: usize,
        callback: Option<&F>,
    ) -> (M::StateType, M::ScoreType)
    where
        M: OptModel<ScoreType = NotNan<f64>> + Sync + Send,
        F: OptCallbackFn<M::StateType, M::ScoreType>,
    {
        EpsilonGreedyOptimizer::optimize(self, model, initial_state, n_iter, callback)
    }
}

impl Optimizer for HillClimbingOptimizer {
    fn optimize<M, F>(
        &self,
        model: &M,
        initial_state: Option<M::StateType>,
        n_iter: usize,
        callback: Option<&F>,
    ) -> (M::StateType, M::ScoreType)
    where
        M: OptModel<ScoreType = NotNan<f64>> + Sync + Send,
        F: OptCallbackFn<M::StateType, M::ScoreType>,
    {
        HillClimbingOptimizer::optimize(self, model, initial_state, n_iter, callback)
    }
}

impl Optimizer for LogisticAnnealingOptimizer {
    fn optimize<M, F>(
        &self,
        model: &M,
        initial_state: Option<M::StateType>,
        n_iter: usize,
        callback: Option<&F>,
    ) -> (M::StateType, M::ScoreType)
    where
        M: OptModel<ScoreType = NotNan<f64>> + Sync + Send,
        F: OptCallbackFn<M::StateType, M::ScoreType>,
    {
        LogisticAnnealingOptimizer::optimize(self, model, initial_state, n_iter, callback)
    }
}

impl Optimizer for RelativeAnnealingOptimizer {
    fn optimize<M, F>(
        &self,
        model: &M,
        initial_state: Option<M::StateType>,
        n_iter: usize,
        callback: Option<&F>,
    ) -> (M::StateType, M::ScoreType)
    where
        M: OptModel<ScoreType = NotNan<f64>> + Sync + Send,
        F: OptCallbackFn<M::StateType, M::ScoreType>,
    {
        RelativeAnnealingOptimizer::optimize(self, model, initial_state, n_iter, callback)
    }
}

impl Optimizer for SimulatedAnnealingOptimizer {
    fn optimize<M, F>(
        &self,
        model: &M,
        initial_state: Option<M::StateType>,
        n_iter: usize,
        callback: Option<&F>,
    ) -> (M::StateType, M::ScoreType)
    where
        M: OptModel<ScoreType = NotNan<f64>> + Sync + Send,
        F: OptCallbackFn<M::StateType, M::ScoreType>,
    {
        SimulatedAnnealingOptimizer::optimize(self, model, initial_state, n_iter, callback)
    }
}
