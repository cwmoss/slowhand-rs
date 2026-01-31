use kdl::{KdlDocument, KdlValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub base: PathBuf,
    pub name: String,
    objects: HashMap<String, Object>,
    image_types: Vec<String>,
    file_types: Vec<String>,
    object_types: Vec<String>,
    reference_types: Vec<String>,
    document_types: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    name: String,
    ty: String,
    title: String,
    description: String,
    preview: Preview,
    icon: String,
    fields: Vec<Field>,
    actions: Vec<Action>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    name: String,
    ty: FieldType,
    title: String,
    description: String,
    hidden: bool,
    read_only: bool,
    component: String,

    to: Vec<String>,
    of: Vec<String>,
    options: FieldOptions,
    initital_value: String,
    validation: FieldValidations,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldType {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldOptions {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValidations {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preview {
    select: Vec<String>,
    fun: String,
}
impl Schema {
    pub fn new(name: &str, base: &PathBuf) -> Self {
        Self {
            name: name.to_string(),
            base: base.join(name),
            objects: HashMap::new(),
            object_types: [].to_vec(),
            image_types: [].to_vec(),
            reference_types: [].to_vec(),
            file_types: [].to_vec(),
            document_types: [].to_vec(),
        }
    }

    pub fn add_object(&mut self, o: Object) {
        let ty = o.ty.to_string();
        match ty.as_str() {
            "image" => self.image_types.push(ty),
            _ => self.object_types.push(ty),
        }
        self.objects.insert(o.name.to_string(), o);
    }
}

impl Object {
    pub fn new(name: &str, ty: &str) -> Self {
        Self {
            name: name.to_string(),
            ty: ty.to_string(),
            title: "".to_string(),
            description: "".to_string(),
            preview: Preview::new(),
            icon: "".to_string(),
            fields: [].to_vec(),
            actions: [].to_vec(),
        }
    }
}

impl Preview {
    pub fn new() -> Self {
        Self {
            select: [].to_vec(),
            fun: "".to_string(),
        }
    }
}
