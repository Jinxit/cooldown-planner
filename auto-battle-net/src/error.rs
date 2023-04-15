use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

pub type BattleNetResult<T, E> = Result<T, BattleNetError<E>>;

#[derive(Serialize, Deserialize, Debug, Clone, Error)]
pub enum BattleNetError<E>
where
    E: Debug + Clone,
{
    #[error("Client error: {0}")]
    ClientError(E),
    #[error("Server error: {0:?}")]
    ServerError(BattleNetServerError),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BattleNetServerError {
    code: u64,
    r#type: String,
    detail: String,
}
