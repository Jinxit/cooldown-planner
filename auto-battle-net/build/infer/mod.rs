use crate::infer::structs::Struct;
use convert_case::{Case, Casing};
use field::Field;
use field_type::FieldType;
use std::cell::RefCell;

pub mod field;
pub mod field_type;
pub mod structs;

pub type JsonValue = serde_json::Value;
pub type JsonMap = serde_json::Map<String, JsonValue>;

type Rule = fn(&Vec<String>, &str, &JsonValue) -> Option<FieldType>;

pub fn infer_from_json(
    path: Vec<String>,
    root_name: String,
    json: &JsonMap,
    structs: &mut Vec<Struct>,
) {
    let fields = fields(&path, json);
    let mut root = Struct {
        name: root_name,
        path,
        fields,
        serialize: false,
        deserialize: true,
    };
    recursively_insert_structs(&mut root, structs);
}

fn recursively_insert_structs(current: &mut Struct, structs: &mut Vec<Struct>) {
    /*let structurally_same = structs.iter_mut().find(|s| s.fields == current.fields);
    if let Some(other) = structurally_same {
        current.name = other.name.clone();
        let removed = other.remove_all_paths();
        for remove in removed {
            //eprintln!("REMOVING1 path {remove:?} {}", current.name);
            for s in &mut *structs {
                s.remove_specific_path(&remove)
            }
        }
        let removed = current.remove_all_paths();
        for remove in removed {
            //eprintln!("REMOVING2 path {remove:?} {}", current.name);
            for s in &mut *structs {
                s.remove_specific_path(&remove)
            }
        }
    } else */
    let structurally_same = structs
        .iter_mut()
        .find(|s| !s.fields.is_empty() && s.fields == current.fields && s.path == current.path);
    if let Some(other) = structurally_same {
        current.name = other.name.clone();
    } else {
        let name = (1..)
            .map(|i| {
                if i == 1 {
                    current.name.clone()
                } else {
                    format!("{}{i}", &current.name)
                }
            })
            .find(|indexed_name| {
                indexed_name != "Option"
                    && structs
                        .iter()
                        .all(|s| &s.name != indexed_name || s.path != current.path)
            });
        current.name = name.unwrap();
        let index = structs.len();
        structs.push(current.clone());
        for field in &mut current.fields {
            match &mut field.ty {
                FieldType::Primitive(_) => {}
                FieldType::Struct(field) => recursively_insert_structs(field, structs),
                FieldType::Array(field) => {
                    let mut borrow = field.borrow_mut();
                    if let FieldType::Struct(field) = &mut *borrow {
                        recursively_insert_structs(field, structs)
                    }
                }
                FieldType::Optional(field) => {
                    let mut borrow = field.borrow_mut();
                    if let FieldType::Struct(field) = &mut *borrow {
                        recursively_insert_structs(field, structs)
                    } else if let FieldType::Array(field) = &mut *borrow {
                        let mut borrow = field.borrow_mut();
                        if let FieldType::Struct(field) = &mut *borrow {
                            recursively_insert_structs(field, structs)
                        }
                    } else if let FieldType::Primitive(_) = &mut *borrow {
                        // pass
                    } else {
                        panic!("unexpected optional field {field:?}")
                    }
                }
            }
        }
        *structs.get_mut(index).unwrap() = current.clone();
    }
}

fn fields(path: &Vec<String>, map: &JsonMap) -> Vec<Field> {
    let rules: Vec<Rule> = vec![
        localized_string,
        locale,
        link,
        float,
        integer,
        boolean,
        string,
        object,
        array,
    ];

    let mut fields = vec![];
    for (key, value) in map {
        if key == "self" {
            continue;
        }
        let mut matched = false;
        for rule in &rules {
            if let Some(ty) = rule(path, key, value) {
                matched = true;
                fields.push(Field {
                    name: key.clone(),
                    ty,
                });
                break;
            }
        }

        if !matched {
            panic!("couldn't infer {path:?} {key} {value}");
        }
    }

    fields
}

fn object(path: &Vec<String>, key: &str, value: &JsonValue) -> Option<FieldType> {
    let optionals = vec![
        (
            "profile",
            "character_profile",
            "character_profile_summary",
            "active_title",
        ),
        (
            "profile",
            "character_profile",
            "character_profile_summary",
            "covenant_progress",
        ),
        (
            "profile",
            "character_profile",
            "character_profile_summary",
            "guild",
        ),
        (
            "profile",
            "character_profile",
            "character_profile_summary",
            "achievements_statistics",
        ),
        (
            "profile",
            "character_profile",
            "character_profile_summary",
            "active_spec",
        ),
        (
            "profile",
            "character_mythic_keystone_profile",
            "character_mythic_keystone_profile_index",
            "current_mythic_rating",
        ),
        ("game_data", "journal", "journal_instance", "area"),
    ];

    if let Some(obj) = value.as_object() {
        return if optionals.contains(&(
            path.get(0).unwrap().as_ref(),
            path.get(1).unwrap().as_ref(),
            path.get(2).unwrap().as_ref(),
            key,
        )) {
            Some(FieldType::Optional(Box::new(RefCell::new(
                FieldType::Struct(Struct {
                    name: key.to_case(Case::Pascal),
                    path: path.clone(),
                    fields: fields(path, obj),
                    serialize: false,
                    deserialize: true,
                }),
            ))))
        } else {
            Some(FieldType::Struct(Struct {
                name: key.to_case(Case::Pascal),
                path: path.clone(),
                fields: fields(path, obj),
                serialize: false,
                deserialize: true,
            }))
        };
    }

    None
}

fn array(path: &Vec<String>, key: &str, value: &JsonValue) -> Option<FieldType> {
    let rules: Vec<Rule> = vec![
        localized_string,
        locale,
        link,
        float,
        integer,
        boolean,
        string,
        object,
        array,
    ];

    if let Some(arr) = value.as_array() {
        for rule in rules {
            if let Some(ty) = rule(path, key, arr.get(0).unwrap()) {
                return Some(FieldType::Array(Box::new(RefCell::new(ty))));
            }
        }
    }

    None
}

fn boolean(_path: &Vec<String>, _key: &str, value: &JsonValue) -> Option<FieldType> {
    if value.is_boolean() {
        return Some(FieldType::Primitive("bool"));
    }

    None
}

fn integer(_path: &Vec<String>, _key: &str, value: &JsonValue) -> Option<FieldType> {
    if value.is_i64() {
        return Some(FieldType::Primitive("i64"));
    }

    None
}

fn float(_path: &Vec<String>, _key: &str, value: &JsonValue) -> Option<FieldType> {
    if value.is_f64() {
        return Some(FieldType::Primitive("ordered_float::NotNan<f64>"));
    }

    None
}

fn string(_path: &Vec<String>, _key: &str, value: &JsonValue) -> Option<FieldType> {
    if value.is_string() {
        return Some(FieldType::Primitive("String"));
    }

    None
}

fn localized_string(path: &Vec<String>, key: &str, value: &JsonValue) -> Option<FieldType> {
    let optionals = [("game_data", "journal", "journal_encounter", "body_text")];
    if let Some(value) = value.as_object() {
        if value.contains_key("en_US") {
            return if optionals.contains(&(
                path.get(0).unwrap().as_ref(),
                path.get(1).unwrap().as_ref(),
                path.get(2).unwrap().as_ref(),
                key,
            )) {
                Some(FieldType::Optional(Box::new(RefCell::new(
                    FieldType::Primitive("crate::LocalizedString"),
                ))))
            } else {
                Some(FieldType::Primitive("crate::LocalizedString"))
            };
        }
    }

    None
}

fn link(_path: &Vec<String>, _key: &str, value: &JsonValue) -> Option<FieldType> {
    if let Some(value) = value.as_object() {
        if value.contains_key("href") && value.keys().len() == 1 {
            return Some(FieldType::Primitive("crate::Link"));
        }
    }

    None
}

fn locale(_path: &Vec<String>, key: &str, value: &JsonValue) -> Option<FieldType> {
    if value.is_string() && key == "locale" {
        Some(FieldType::Primitive("crate::Locale"))
    } else {
        None
    }
}
