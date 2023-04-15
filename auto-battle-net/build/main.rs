#![allow(clippy::ptr_arg)]

mod fetch;
mod infer;
mod request;
mod resource_json;

use crate::request::method_request;
use crate::resource_json::{Method, Resources};
use convert_case::{Case, Casing};
use fetch::fetch;
use infer::infer_from_json;
use itertools::Itertools;
use reqwest::Url;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=resources/game-data-apis.json");
    println!("cargo:rerun-if-changed=resources/profile-apis.json");

    dotenvy::dotenv().unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let access_token = env::var("BNET_ACCESS_TOKEN").unwrap();
    let mut structs = vec![];

    let _ = fs::remove_dir_all(PathBuf::from(&out_dir).join("profile"));
    let profile_apis: Resources = {
        let file = File::open("resources/profile-apis.json").unwrap();
        serde_json::from_reader(file).unwrap()
    };

    for resource in profile_apis.resources {
        for method in resource.methods {
            let json = fetch(
                Url::parse("https://eu.api.blizzard.com").unwrap(),
                &method,
                &access_token,
            )
            .await;

            let root_name = method
                .name
                .replace(' ', "")
                .replace("PvP", "Pvp")
                .replace("WoW", "Wow")
                .replace(['(', ',', ')'], "");
            let resource_name = resource
                .name
                .replace(" API", "")
                .replace(' ', "")
                .replace("PvP", "Pvp")
                .replace("WoW", "Wow")
                .replace(['(', ',', ')'], "");
            let path = vec![
                "profile".to_string(),
                resource_name.to_case(Case::Snake),
                root_name.to_case(Case::Snake),
            ];
            structs.push(method_request(path.clone(), method.clone()));
            infer_from_json(
                path.clone(),
                root_name.clone() + "Response",
                &json,
                &mut structs,
            );

            let dest_path = Path::new(&out_dir)
                .join(path.iter().collect::<PathBuf>())
                .with_extension("rs");
            fs::create_dir_all(dest_path.parent().unwrap()).unwrap();

            let should_cache = !method.path.starts_with("/profile/user");
            let output = generate_api(&root_name, method, should_cache, "{}.api.blizzard.com");
            eprintln!("write 0 {dest_path:?}");
            File::options()
                .write(true)
                .create(true)
                .truncate(false)
                .append(true)
                .open(dest_path)
                .unwrap()
                .write_all(output.as_bytes())
                .unwrap();
        }
    }

    let _ = fs::remove_dir_all(PathBuf::from(&out_dir).join("game_data"));
    let game_data_apis: Resources = {
        let file = File::open("resources/game-data-apis.json").unwrap();
        serde_json::from_reader(file).unwrap()
    };

    for resource in game_data_apis.resources {
        for method in resource.methods {
            if method.cn_region {
                continue;
            }

            let json = fetch(
                Url::parse("https://eu.api.blizzard.com").unwrap(),
                &method,
                &access_token,
            )
            .await;

            let root_name = method
                .name
                .replace(' ', "")
                .replace("PvP", "Pvp")
                .replace("WoW", "Wow")
                .replace(['(', ',', ')'], "");
            let resource_name = resource
                .name
                .replace(" API", "")
                .replace(' ', "")
                .replace("PvP", "Pvp")
                .replace("WoW", "Wow")
                .replace(['(', ',', ')'], "");
            let path = vec![
                "game_data".to_string(),
                resource_name.to_case(Case::Snake),
                root_name.to_case(Case::Snake),
            ];
            structs.push(method_request(path.clone(), method.clone()));
            infer_from_json(
                path.clone(),
                root_name.clone() + "Response",
                &json,
                &mut structs,
            );

            let dest_path = Path::new(&out_dir)
                .join(path.iter().collect::<PathBuf>())
                .with_extension("rs");
            fs::create_dir_all(dest_path.parent().unwrap()).unwrap();

            let output = generate_api(&root_name, method, true, "{}.api.blizzard.com");
            eprintln!("write 0 {dest_path:?}");
            File::options()
                .write(true)
                .create(true)
                .truncate(false)
                .append(true)
                .open(dest_path)
                .unwrap()
                .write_all(output.as_bytes())
                .unwrap();
        }
    }

    {
        let dest_path = Path::new(&out_dir).join("mod.rs");
        fs::create_dir_all(dest_path.parent().unwrap()).unwrap();
        eprintln!("writing 1 {dest_path:?}");
        File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(dest_path)
            .unwrap()
            .write_all("pub mod profile;\npub mod game_data;".as_bytes())
            .unwrap();
    }

    for (parent, children) in &structs
        .iter()
        .filter(|s| s.path.len() >= 2)
        .sorted_by_key(|s| s.path.get(0).unwrap())
        .group_by(|s| s.path.get(0).unwrap())
    {
        eprintln!("writing 2 {parent}");
        let dest_path = Path::new(&out_dir).join(parent).join("mod.rs");
        fs::create_dir_all(dest_path.parent().unwrap()).unwrap();
        File::options()
            .write(true)
            .create(true)
            .truncate(false)
            .append(true)
            .open(dest_path)
            .unwrap()
            .write_all(
                children
                    .into_iter()
                    .map(|s| s.path.get(1).unwrap())
                    .unique()
                    .map(|mod_name| format!("pub mod {mod_name};"))
                    .join("\n")
                    .as_bytes(),
            )
            .unwrap();
    }

    for (path, structs) in &structs
        .iter()
        .filter(|s| !s.path.is_empty())
        .group_by(|s| s.path.clone())
    {
        let dest_path = Path::new(&out_dir)
            .join(path.iter().collect::<PathBuf>())
            .with_extension("rs");
        fs::create_dir_all(dest_path.parent().unwrap()).unwrap();
        eprintln!("writing 3 {dest_path:?}");
        File::options()
            .write(true)
            .create(true)
            .truncate(false)
            .append(true)
            .open(dest_path)
            .unwrap()
            .write_all(
                structs
                    .into_iter()
                    .map(|s| s.to_code())
                    .join("\n")
                    .as_bytes(),
            )
            .unwrap();
    }

    for (path, structs) in &structs.iter().filter(|s| !s.path.is_empty()).group_by(|s| {
        let mut path = s.path.clone();
        path.remove(path.len() - 1);
        path
    }) {
        let dest_path = Path::new(&out_dir)
            .join(path.iter().collect::<PathBuf>())
            .join("mod.rs");
        fs::create_dir_all(dest_path.parent().unwrap()).unwrap();
        eprintln!("writing 4 {dest_path:?}");
        File::options()
            .write(true)
            .create(true)
            .truncate(false)
            .append(true)
            .open(dest_path)
            .unwrap()
            .write_all(
                structs
                    .into_iter()
                    .map(|s| s.path.last().unwrap())
                    .unique()
                    .map(|mod_name| format!("pub mod {mod_name};"))
                    .join("\n")
                    .as_bytes(),
            )
            .unwrap();
    }
}

fn generate_api(root_name: &str, method: Method, should_cache: bool, authority: &str) -> String {
    let namespace_category = match method
        .parameters
        .iter()
        .find(|p| p.name == "namespace")
        .unwrap()
        .default_value
        .as_str()
        .unwrap()
    {
        "static-us" => "Static",
        "dynamic-us" => "Dynamic",
        "profile-us" => "Profile",
        s => panic!("Unknown namespace category {s}"),
    };

    let substitutions = method
        .parameters
        .iter()
        .filter(|p| p.name.starts_with('{'))
        .map(|p| {
            let var = p.name[1..p.name.len() - 1].to_string();
            let var_snake = var.to_case(Case::Snake);
            if p.r#type == "string" {
                format!("{var} = urlencoding::encode(&self.{var_snake}),\n                ")
            } else {
                format!("{var} = &self.{var_snake},\n                ")
            }
        })
        .join("");

    let path = &method.path;
    format!(
        r#"use serde::Serialize;
use serde::Deserialize;

impl crate::BattleNetRequest for {root_name}Request {{
    type Response = {root_name}Response;

    fn uri(&self, region: crate::Region) -> http::uri::Uri {{
        http::uri::Uri::builder()
            .scheme("https")
            .authority(format!("{authority}", region))
            .path_and_query(format!(
                "{path}?namespace={{namespace}}",
                {substitutions}namespace = crate::Namespace {{
                    region,
                    category: crate::NamespaceCategory::{namespace_category}
                }}
            ))
            .build()
            .unwrap()
    }}

    fn should_cache() -> bool {{
        {should_cache}
    }}
}}

"#,
    )
}
