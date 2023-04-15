use crate::infer::field_type::FieldType;
use convert_case::{Case, Casing};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Field {
    pub name: String,
    pub ty: FieldType,
}

impl Field {
    pub fn to_code(&self, serialize: bool, deserialize: bool) -> String {
        let snake_cased = match self.name.as_str() {
            "self" => "_self".to_string(),
            "type" => "r#type".to_string(),
            other => other.to_case(Case::Snake),
        };

        if (serialize || deserialize) && snake_cased != self.name {
            return format!(
                "    #[serde(rename = \"{original_name}\")]\n    pub {snake_cased}: {ty},",
                original_name = self.name,
                ty = self.ty.to_code()
            );
        }
        if deserialize && matches!(self.ty, FieldType::Array(_))
            || matches!(self.ty, FieldType::Optional(_))
        {
            return format!(
                "    #[serde(default)]\n    pub {snake_cased}: {ty},",
                ty = self.ty.to_code()
            );
        }

        /*
        if let FieldType::Primitive("ordered_float::NotNan<f64>") = self.ty {
            return format!(
                "    #[serde(serialize_with = \"crate::serialize_not_nan\", deserialize_with = \"crate::deserialize_not_nan\")]\n    pub {snake_cased}: {ty},",
                ty = self.ty.to_code(),
            );
        }
         */

        format!("    pub {snake_cased}: {ty},", ty = self.ty.to_code())
    }
}
