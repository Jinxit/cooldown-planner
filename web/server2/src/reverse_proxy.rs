use axum::body::Body;
use axum::response::IntoResponse;
use battle_net_auth::OAuthToken;
use http::{HeaderMap, HeaderName, HeaderValue};
use url::Url;

pub async fn reverse_proxy(url: Url, token: Option<OAuthToken>) -> impl IntoResponse {
    let resp = {
        let mut client = reqwest::Client::new().get(url);
        if let Some(token) = token {
            client = client.bearer_auth(token.expose_secret())
        }
        client.send().await.unwrap()
    };

    let mut headers = HeaderMap::with_capacity(resp.headers().len());
    headers.extend(resp.headers().into_iter().map(|(name, value)| {
        let name = HeaderName::from_bytes(name.as_ref()).unwrap();
        let value = HeaderValue::from_bytes(value.as_ref()).unwrap();
        (name, value)
    }));
    let body = Body::from_stream(resp.bytes_stream());

    (headers, body)
}
