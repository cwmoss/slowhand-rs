use kdl::{KdlDocument, KdlValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use strum_macros::EnumString;

pub const DOCOPTS: [&'static str; 4] = ["title", "icon", "description", "preview"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub base: PathBuf,
    pub name: String,
    pub objects: HashMap<String, Object>,
    image_types: Vec<String>,
    file_types: Vec<String>,
    pub object_types: Vec<String>,
    reference_types: Vec<String>,
    pub document_types: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Object {
    pub name: String,
    pub base_type: BaseType,
    pub title: String,
    pub description: String,
    pub preview: Preview,
    pub icon: String,
    pub fields: Vec<Field>,
    pub actions: Vec<Action>,
}
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Default)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "_t", content = "_c")]
#[strum(serialize_all = "snake_case")]
pub enum ObjectType {
    #[strum(serialize = "doc", serialize = "document")]
    Doc,
    #[default]
    Object,
    Image,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Default)]
#[serde(rename_all = "snake_case")]
// #[serde(tag = "_t", content = "_c")]
#[strum(serialize_all = "snake_case")]
pub enum BaseType {
    #[strum(serialize = "doc", serialize = "document")]
    Doc,
    #[default]
    Object,
    Image,
    File,
    Action,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Default)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FieldType {
    Array,
    Bloc,
    #[strum(serialize = "bool", serialize = "boolean")]
    Boolean,
    Date,
    Datetime,
    File,
    Geopoint,
    Image,
    Number,
    #[strum(serialize = "ref", serialize = "reference")]
    Reference,
    Slug,
    Span,
    #[default]
    String,
    Text,
    Url,
    Object(String),
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub title: String,
    pub description: String,
    pub hidden: bool,
    pub read_only: bool,
    pub component: String,

    pub to: Vec<String>,
    pub of: Vec<String>,
    pub options: FieldOptions,
    pub initital_value: String,
    pub validation: FieldValidations,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
//pub struct FieldType {}
//#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldOptions {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FieldValidations {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Action {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Preview {
    pub title: String,
    pub subtitle: Option<String>,
    pub media: Option<String>,
    pub select: Vec<String>,
    pub fun: Option<String>,
}
impl Schema {
    pub fn new(name: &str, base: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            base: base,
            objects: HashMap::new(),
            object_types: [].to_vec(),
            image_types: [].to_vec(),
            reference_types: [].to_vec(),
            file_types: [].to_vec(),
            document_types: [].to_vec(),
        }
    }

    pub fn add_object(&mut self, o: Object) {
        let bt = o.base_type.clone();
        let name = o.name.clone();
        self.objects.insert(name.to_string(), o);
        match bt {
            BaseType::Image => self.image_types.push(name),
            BaseType::Doc => self.document_types.push(name),
            BaseType::File => self.file_types.push(name),
            _ => self.object_types.push(name),
        }
    }
    pub fn get_object(&self, name: &str) -> Option<&Object> {
        self.objects.get(name)
    }

    pub fn documents(&self) -> Vec<&Object> {
        self.document_types
            .iter()
            .map(|t| self.objects.get(t).unwrap())
            .collect()
    }
}

impl Object {
    pub fn new(name: &str, base_type: BaseType) -> Self {
        Self {
            name: name.to_string(),
            base_type,
            ..Default::default() /*title: "".to_string(),
                                 description: "".to_string(),
                                 preview: Preview::new(),
                                 icon: "".to_string(),
                                 fields: [].to_vec(),
                                 actions: [].to_vec(),*/
        }
    }
}

impl Preview {
    pub fn new(title: String) -> Self {
        Self {
            title,
            ..Default::default()
        }
    }
}

impl Field {
    pub fn new(name: &str, field_type: FieldType) -> Self {
        Self {
            name: name.to_string(),
            field_type,
            ..Default::default()
        }
    }
}
