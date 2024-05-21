use crate::{BattleNetRequest, BattleNetResult};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::{Debug, Display};

#[async_trait]
pub trait BattleNetClientAsync {
    type Error: Display + Debug + Clone + Serialize + DeserializeOwned + Send + Sync + 'static;
    async fn call_async<Req>(&self, request: Req) -> BattleNetResult<Req::Response, Self::Error>
    where
        Req: BattleNetRequest + Send + Sync + 'static,
        Req::Response: Send + Sync + 'static;
}
