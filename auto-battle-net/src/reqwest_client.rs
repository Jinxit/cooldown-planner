use std::num::NonZeroU32;
use std::time::Duration;

use async_trait::async_trait;
use governor::{DefaultKeyedRateLimiter, Jitter, Quota, RateLimiter};
use lazy_static::lazy_static;
use tracing::{error, instrument, trace};

use crate::{BattleNetError, BattleNetRequest, BattleNetResult, BattleNetServerError, Region};
use crate::access_token::BattleNetAccessToken;
use crate::clients::BattleNetClientAsync;

type StringKeyedRateLimiter = DefaultKeyedRateLimiter<String>;
lazy_static! {
    static ref RATE_LIMITER: StringKeyedRateLimiter =
        RateLimiter::keyed(Quota::per_second(NonZeroU32::new(50).unwrap()));
}

#[derive(Clone)]
pub struct ReqwestBattleNetClient {
    pub region: Region,
    pub access_token: BattleNetAccessToken,
}

#[async_trait]
impl BattleNetClientAsync for ReqwestBattleNetClient {
    type Error = String;

    #[instrument(skip(self))]
    async fn call_async<Req>(&self, request: Req) -> BattleNetResult<Req::Response, Self::Error>
    where
        Req: BattleNetRequest + Send + Sync + 'static,
    {
        if let BattleNetAccessToken::ClientCredentials(_) = &self.access_token {
            if Req::is_user_dependent() {
                return Err(BattleNetError::ClientError(format!("/profile/user/* requests like {request:?} require user credentials")));
            }
        }

        RATE_LIMITER
            .until_key_ready_with_jitter(&self.access_token.to_string(), Jitter::up_to(Duration::from_secs(2)))
            .await;
        let uri = request.uri(self.region);
        trace!("fetching {uri}");
        let response = reqwest::Client::new()
            .get(uri.to_string())
            .bearer_auth(&self.access_token.to_string())
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
            let error: BattleNetServerError = serde_json::from_slice(&bytes)
                .map_err(|e| BattleNetError::ClientError(e.to_string()))?;
            if status == http::StatusCode::NOT_FOUND {
                trace!("{uri} not found: {error:?}");
            } else {
                error!("{uri} failed: {error:?}");
            }
            Err(BattleNetError::ServerError(error))
        }
    }
}
