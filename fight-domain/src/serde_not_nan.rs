use ordered_float::NotNan;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize_not_nan<S>(value: &NotNan<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_f64(value.into_inner())
}

pub fn deserialize_not_nan<'de, D>(deserializer: D) -> Result<NotNan<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let num = f64::deserialize(deserializer)?;
    NotNan::new(num).map_err(serde::de::Error::custom)
}
