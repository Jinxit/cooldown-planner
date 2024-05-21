use std::time::Duration;

use localsearch;
use localsearch::{OptCallbackFn, OptModel};
pub use localsearch::optim::EpsilonGreedyOptimizer;
pub use localsearch::optim::HillClimbingOptimizer;
use localsearch::optim::LocalSearchOptimizer;
pub use localsearch::optim::LogisticAnnealingOptimizer;
pub use localsearch::optim::RelativeAnnealingOptimizer;
use ordered_float::NotNan;

pub struct SimulatedAnnealingOptimizer {
    optimizer: localsearch::optim::SimulatedAnnealingOptimizer,
    max_temperature: f64,
    min_temperature: f64,
}

impl SimulatedAnnealingOptimizer {
    /// Constructor of SimulatedAnnealingOptimizer
    ///
    /// - `patience` : the optimizer will give up
    ///   if there is no improvement of the score after this number of iterations
    /// - `n_trials` : number of trial states to generate and evaluate at each iteration
    /// - `max_temperature` : the initial temperature at the begining of the optimization
    /// - `min_temperature` : the final temperature at the end of the optimization
    pub fn new(
        patience: usize,
        n_trials: usize,
        max_temperature: f64,
        min_temperature: f64,
    ) -> Self {
        Self {
            optimizer: localsearch::optim::SimulatedAnnealingOptimizer::new(patience, n_trials),
            max_temperature,
            min_temperature,
        }
    }

    /// Start optimization
    ///
    /// - `model` : the model to optimize
    /// - `initial_state` : the initial state to start optimization. If None, a random state will be generated.
    /// - `n_iter`: maximum iterations
    /// - `callback` : callback function that will be invoked at the end of each iteration
    pub fn optimize<M, F>(
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
        let result = self.optimizer.optimize(
            model,
            initial_state,
            n_iter,
            time_limit,
            callback,
            (self.max_temperature, self.min_temperature),
        );
        (result.0, result.1)
    }
}
