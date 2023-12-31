use crate::clients::BattleNetClientAsync;
use crate::{BattleNetError, BattleNetRequest, BattleNetResult, Region};
use clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{clock, Jitter, Quota, RateLimiter};
use lazy_static::lazy_static;
use tracing::warn;
use std::num::NonZeroU32;
use std::time::Duration;

type DefaultRateLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;
lazy_static! {
    static ref RATE_LIMITER: DefaultRateLimiter =
        RateLimiter::direct(Quota::per_second(NonZeroU32::new(50).unwrap()));
}

#[derive(Clone)]
pub struct ReqwestBattleNetClient {
    pub region: Region,
    pub access_token: String,
}

impl BattleNetClientAsync for ReqwestBattleNetClient {
    type Error = String;

    async fn call_async<Req>(&self, request: Req) -> BattleNetResult<Req::Response, Self::Error>
    where
        Req: BattleNetRequest,
    {
        RATE_LIMITER
            .until_ready_with_jitter(Jitter::up_to(Duration::from_secs(2)))
            .await;
        let uri = request.uri(self.region);
        warn!("fetching {uri}");
        let response = reqwest::Client::new()
            .get(uri.to_string())
            .bearer_auth(&self.access_token)
            .send()
            .await
            .map_err(|e| BattleNetError::ClientError(e.to_string()))?;
        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .map_err(|e| BattleNetError::ClientError(e.to_string()))?;
        if status.is_success() {
            serde_json::from_slice(&bytes).map_err(|e| BattleNetError::ClientError(e.to_string()))
        } else {
            Err(BattleNetError::ServerError(
                serde_json::from_slice(&bytes)
                    .map_err(|e| BattleNetError::ClientError(e.to_string()))?,
            ))
        }
    }
}
