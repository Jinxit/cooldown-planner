use crate::infer::structs::Struct;
use std::cell::RefCell;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum FieldType {
    Primitive(&'static str),
    Struct(Struct),
    Array(Box<RefCell<FieldType>>),
    Optional(Box<RefCell<FieldType>>),
}

impl FieldType {
    pub fn to_code(&self) -> String {
        match self {
            FieldType::Primitive(p) => p.to_string(),
            FieldType::Struct(s) => s.name.replace(['(', ',', ')'], ""),
            FieldType::Array(ty) => format!("Vec<{}>", ty.borrow().to_code()),
            FieldType::Optional(ty) => format!("Option<{}>", ty.borrow().to_code()),
        }
    }
}
