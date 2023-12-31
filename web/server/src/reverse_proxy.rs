use axum::body::StreamBody;
use axum::response::IntoResponse;
use battle_net_auth::OAuthToken;
use url::Url;

pub async fn reverse_proxy(url: Url, token: Option<OAuthToken>) -> impl IntoResponse {
    let resp = {
        let mut client = reqwest::Client::new().get(url);
        if let Some(token) = token {
            client = client.bearer_auth(token.expose_secret())
        }
        client.send().await.unwrap()
    };

    let headers = resp.headers().clone();
    let body = StreamBody::new(resp.bytes_stream());

    (headers, body)
}
