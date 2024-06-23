use serde::{Deserialize, Serialize};

impl crate::BattleNetRequest for UserInfoRequest {
    type Response = UserInfoResponse;

    fn uri(&self, _region: crate::Region) -> http::uri::Uri {
        http::uri::Uri::builder()
            .scheme("https")
            .authority("oauth.battle.net")
            .path_and_query("/userinfo")
            .build()
            .unwrap()
    }

    fn is_user_dependent() -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Default)]
pub struct UserInfoRequest {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash)]
pub struct UserInfoResponse {
    pub id: u64,
    pub battletag: String,
}
