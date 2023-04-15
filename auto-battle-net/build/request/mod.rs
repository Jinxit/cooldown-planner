use crate::infer::field::Field;
use crate::infer::field_type::FieldType;
use crate::infer::structs::Struct;
use crate::resource_json::Method;

pub fn method_request(path: Vec<String>, method: Method) -> Struct {
    let name = method
        .name
        .replace(" API", "")
        .replace(' ', "")
        .replace("PvP", "Pvp")
        .replace("WoW", "Wow")
        .replace(['(', ',', ')'], "")
        + "Request";

    let params = method
        .parameters
        .iter()
        .filter(|p| p.name.starts_with('{'))
        .collect::<Vec<_>>();
    let fields = params
        .iter()
        .map(|p| Field {
            name: p.name[1..p.name.len() - 1].to_string(),
            ty: match p.r#type.as_str() {
                "integer" => FieldType::Primitive("i64"),
                "string" => FieldType::Primitive("String"),
                _ => panic!("Unknown type {}", p.r#type),
            },
        })
        .collect();

    Struct {
        name,
        path,
        fields,
        serialize: true,
        deserialize: false,
    }
}
