use crate::infer::field::Field;
use itertools::Itertools;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Struct {
    pub name: String,
    pub path: Vec<String>,
    pub fields: Vec<Field>,
    pub serialize: bool,
    pub deserialize: bool,
}

impl Struct {
    pub fn to_code(&self) -> String {
        let derive_default = if self.fields.is_empty() {
            ", Default"
        } else {
            ""
        };

        format!(
            "#[derive({ser}{de}Clone, Debug, PartialEq, Hash{derive_default})]\npub struct {name} {{\n{fields}\n}}\n",
            name = self.name,
            fields = self
                .fields
                .iter()
                .map(|f| f.to_code(self.serialize, self.deserialize))
                .join("\n"),
            ser = if self.serialize || self.deserialize { "Serialize, " } else { "" },
            de = if self.deserialize || self.serialize { "Deserialize, " } else { "" },
        )
    }

    /*
    pub fn remove_all_paths(&mut self) -> HashSet<Vec<String>> {
        let mut removed = HashSet::new();
        if !self.path.is_empty() {
            removed.insert(self.path.clone());
        }
        self.path = vec![];
        for field in &mut self.fields {
            if let FieldType::Struct(s) = &mut field.ty {
                for other in s.remove_all_paths() {
                    if !other.is_empty() {
                        removed.insert(other);
                    }
                }
            }
        }

        removed
    }

    pub fn remove_specific_path(&mut self, path: &Vec<String>) {
        if &self.path == path {
            self.path = vec![];
        }

        for field in &mut self.fields {
            if let FieldType::Struct(s) = &mut field.ty {
                s.remove_specific_path(path);
            }
        }
    }

     */
}
