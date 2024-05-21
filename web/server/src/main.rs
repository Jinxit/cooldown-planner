#![allow(unused_imports)]

mod bnet_access_token;
mod fallback;
mod handlers;
mod reverse_proxy;

use crate::fallback::file_and_error_handler;
use app_package::session::{BattleNetUser, CooldownPlannerSession};
use app_package::{App, AppProps};
use auto_battle_net::game_data::realm::realms_index::Realms;
use auto_battle_net::game_data::spell::spell_media::SpellMediaRequest;
use auto_battle_net::oauth::user_authentication::user_info::UserInfoRequest;
use auto_battle_net::{
    BattleNetClientAsync, Locale, Namespace, NamespaceCategory, Region, ReqwestBattleNetClient,
};
use axum::error_handling::HandleErrorLayer;
use axum::extract::{FromRequestParts, Path, Query, RawQuery, State};
use axum::middleware::from_fn;
use axum::response::{AppendHeaders, Html, IntoResponse, Response};
use axum::routing::{any, get, post};
use axum::{BoxError, Extension, Router, ServiceExt};
use battle_net_auth::axum::OAuthTokenExt;
use bnet_access_token::*;
use bytes::Bytes;
use handlers::*;
use http::header::{ACCEPT, ACCEPT_ENCODING, AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE, ORIGIN};
use http::request::Parts;
use http::{HeaderMap, HeaderName, HeaderValue, Method, Request, StatusCode};
use leptos::prelude::*;
use leptos_meta::MetaTags;
use leptos_axum::{
    generate_route_list, handle_server_fns, handle_server_fns_with_context, LeptosRoutes,
};
use leptos_dom::warn;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, PkceCodeChallenge, RedirectUrl,
    TokenResponse, TokenUrl,
};
use paseto_sessions::{session_middleware, Session};
use reverse_proxy::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use std::any::TypeId;
use std::convert::Infallible;
use std::env::var;
use std::net::SocketAddr;
use std::num::NonZeroU16;
use std::sync::Arc;
use std::time::Duration;
use storage::axum::StoreExt;
use storage::sqlite::SqLiteConnection;
use storage::Storage;
use tower::ServiceBuilder;
use tower_governor::errors::display_error;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::GovernorLayer;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use url::Url;

#[tokio::main]
async fn main() {
    {
        let subscriber = tracing_subscriber::fmt().pretty();
        let subscriber = subscriber.with_ansi(true);
        let subscriber = subscriber.with_env_filter(EnvFilter::from_default_env());
        subscriber.init();
    }
    dotenvy::dotenv().unwrap();


    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let _cors_layer = CorsLayer::new()
        .allow_headers(vec![
            ACCEPT,
            ACCEPT_ENCODING,
            AUTHORIZATION,
            CONTENT_TYPE,
            ORIGIN,
        ])
        .allow_methods(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any);

    let _governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .use_headers()
            .finish()
            .unwrap(),
    );

    let app = Router::new()
        .route("/bnet/*fn_name", post(handle_server_fns))
        .route("/bnet/*fn_name", get(handle_server_fns).layer(SetResponseHeaderLayer::appending(CACHE_CONTROL, HeaderValue::from_str(&format!("max-age={}", 60 * 60)).unwrap())))
        .route("/bnet/login-callback", any(battle_net_login_callback))
        .route("/spell_icon/:spell_id", get(spell_icon))
        .route("/icon/:icon", get(icon))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || view! {
                <!DOCTYPE html> 
                <html lang="en" class="bg-slate-700 text-gray-300 selection:bg-slate-700">
                    <head>
                        <meta charset="utf-8"/>
                        <meta name="viewport" content="width=device-width, initial-scale=1"/>
                        // required to load JS/WASM for client-side interactivity
                        <HydrationScripts options=leptos_options.clone()/>
                        // optional: can be toggled to include/exclude cargo-leptos live-reloading code
                        // <AutoReload options=leptos_options.clone()/>
                        // required if using leptos_meta
                        <MetaTags/>
                    </head>
                    <body>
                        <App/>
                    </body>
                </html>
            }
        })
        .fallback(file_and_error_handler)
        //.layer(cors_layer)
        //.layer(CompressionLayer::new().gzip(true).deflate(true))
        .layer(from_fn(session_middleware::<CooldownPlannerSession>))
        //.layer(SetResponseHeaderLayer::appending(HeaderName::from_lowercase(b"cross-origin-opener-policy").unwrap(), HeaderValue::from_static("same-origin")))
        //.layer(SetResponseHeaderLayer::appending(HeaderName::from_lowercase(b"cross-origin-embedder-policy").unwrap(), HeaderValue::from_static("credentialless")))
        .with_store(SqLiteConnection::new("sqlite://target/storage.sqlite").await.unwrap())
        .with_battle_net_auth(var("BNET_CLIENT_ID").unwrap(), var("BNET_CLIENT_SECRET").unwrap())
        .with_state(leptos_options)
        //.layer(TraceLayer::new_for_http())
        /*.layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|e: BoxError| async move {
                    display_error(e)
                }))
                .layer(GovernorLayer {
                    config: Box::leak(governor_conf),
                }),
        )*/;

    #[cfg(not(feature = "lambda"))]
    {
        info!("listening on http://{}", &addr);
        let listener = TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }

    #[cfg(feature = "lambda")]
    {
        lambda_web::run_hyper_on_lambda(app).await.unwrap();
    }
}
