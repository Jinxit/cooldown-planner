use axum::extract::Path;
use axum::response::IntoResponse;
use tracing::error;
use url::Url;

#[axum::debug_handler]
pub async fn icon(Path(icon): Path<String>) -> impl IntoResponse {
    let url = Url::parse(&format!(
        "https://render.worldofwarcraft.com/eu/icons/56/{icon}.jpg"
    ))
    .map_err(|e| {
        error!("{e:#?}");
        e
    })
    .unwrap_or(
        Url::parse("https://render.worldofwarcraft.com/eu/icons/56/inv_misc_questionmark.jpg")
            .unwrap(),
    );

    crate::reverse_proxy::reverse_proxy(url, None).await
}
