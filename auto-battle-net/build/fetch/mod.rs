use std::{env, fs};
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use governor::{Jitter, Quota, RateLimiter};
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use lazy_static::lazy_static;
use reqwest::Url;

use crate::infer::JsonMap;
use crate::resource_json::Method;

type DefaultRateLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;
lazy_static! {
    static ref RATE_LIMITER: DefaultRateLimiter =
        RateLimiter::direct(Quota::per_second(NonZeroU32::new(50).unwrap()));
}

pub async fn fetch(base_url: Url, method: &Method, access_token: &str) -> JsonMap {
    let substitutions: HashMap<&'static str, &'static str> = [
        ("{realmId}", "506"),
        ("{realmSlug}", "draenor"),
        ("{characterId}", "172800938"),
        (
            "{characterName}",
            match method.name.as_str() {
                "Character Hunter Pets Summary" => "mejlej",
                _ => "arkohn",
            },
        ),
        ("{nameSlug}", "latinus-namus"),
        ("{seasonId}", "10"),
        ("{connectedRealmId}", "1080"),
        ("{conduitId}", "20"),
        ("{regionId}", "3"),
        ("{talentId}", "92667"),
        /*
        ("{pvpBracket}", "2v2"),
        ("{achievementCategoryId}", "15441"),
        ("{achievementId}", "545"),
        ("{azeriteEssenceId}", "16"),
        ("{covenantId}", "2"),
        ("{soulbindId}", "3"),
        ("{creatureFamilyId}", "127"),
        ("{creatureTypeId}", "1"),
        ("{creatureId}", "3815"),
        ("{borderId}", "0"),
        ("{emblemId}", "0"),
        ("{heirloomId}", "2"),
        ("{itemClassId}", "0"),
        ("{itemSetId}", "757"),
        ("{itemSubclassId}", "0"),
        ("{itemId}", "47548"),
        ("{journalExpansionId}", "47548"),

         */
    ]
    .into_iter()
    .collect();

    let _out_dir = env::var_os("OUT_DIR").unwrap();
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let cache_key = method.path.replace('/', "_");
    let cache_path = PathBuf::from(manifest_dir)
        .join("cache")
        .join(cache_key)
        .with_extension("json");

    if cache_path.exists() {
        serde_json::from_str(&fs::read_to_string(cache_path).unwrap()).unwrap()
    } else {
        fs::create_dir_all(cache_path.as_path().parent().unwrap()).unwrap();
        let namespace = method
            .parameters
            .iter()
            .find(|p| p.name == "namespace")
            .map(|p| p.default_value.as_str().unwrap().replace("-us", "-eu"))
            .unwrap();

        let params = method
            .parameters
            .iter()
            .filter(|p| p.name.starts_with('{'))
            .collect::<Vec<_>>();
        let mut url = method.path.clone();
        for param in params {
            url = url.replace(
                &param.name,
                &substitutions
                    .get(param.name.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| {
                        if let Some(string) = &param.default_value.as_str() {
                            string.to_string()
                        } else {
                            param.default_value.to_string()
                        }
                    }),
            );
        }
        let url = base_url.join(&url).unwrap();

        RATE_LIMITER
            .until_ready_with_jitter(Jitter::up_to(Duration::from_secs(2)))
            .await;

        let client = reqwest::Client::new();
        let request = client
            .request(
                reqwest::Method::from_str(&method.http_method).unwrap(),
                url.clone(),
            )
            .bearer_auth(access_token)
            .query(&[
                ("namespace", namespace.as_str()),
                ("access_token", access_token),
            ]);
        let response = request.try_clone().unwrap().send().await.unwrap();
        if response.status().is_success() {
            let text = response.text().await.unwrap();
            eprintln!("Trying to write to {cache_path:?}");
            fs::write(cache_path.as_path(), &text).unwrap();
            serde_json::from_str(&text).unwrap()
        } else {
            let response = request.send().await.unwrap();
            if response.status().is_success() {
                let text = response.text().await.unwrap();
                fs::write(cache_path.as_path(), &text).unwrap();
                serde_json::from_str(&text).unwrap()
            } else if response.status().as_u16() == 403 {
                JsonMap::new()
            } else {
                panic!(
                    "Failed when fetching {} {} ({}): {} {}",
                    method.name,
                    url,
                    method.path,
                    response.status(),
                    response.text().await.unwrap()
                )
            }
        }
    }
}
