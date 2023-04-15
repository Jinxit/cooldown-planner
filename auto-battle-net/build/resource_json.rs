use crate::infer::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Resources {
    pub resources: Vec<Resource>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub name: String,
    pub methods: Vec<Method>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Method {
    pub name: String,
    pub description: String,
    pub path: String,
    pub http_method: String,
    pub cn_region: bool,
    pub parameters: Vec<Parameter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub required: bool,
    pub default_value: JsonValue,
}
