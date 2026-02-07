use crate::schema::{self, BaseType, FieldType, Preview, DOCOPTS};
use async_graphql::dynamic::Object;
// use async_graphql::dynamic::Object;
use kdl::{KdlDocument, KdlNode, KdlValue};
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;
//use std::error::Error;
use anyhow::Result;
use std::f32::consts::E;
use std::fs;
// use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use strum_macros::EnumString;

enum SchemaError {
    KdlError,
    IoError,
}
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
enum RootType {
    Graphql,
    Doc,
    Object,
    Image,
    File,
}
impl Into<schema::BaseType> for RootType {
    fn into(self) -> schema::BaseType {
        match self {
            RootType::Doc => schema::BaseType::Doc,
            RootType::Object => schema::BaseType::Object,
            RootType::Image => schema::BaseType::Image,
            RootType::File => schema::BaseType::File,
            _ => schema::BaseType::Object,
        }
    }
}
impl schema::Schema {
    pub fn load_kdl(name: &str, base: &PathBuf) -> Result<Self> {
        let fopts: Vec<String> = "title hidden readOnly description icon initialValue preview"
            .to_string()
            .split(" ")
            .map(|s| s.to_string())
            .collect();

        // let mut s = Self::new(name, base);
        let mut path = base.join(name).join(name);
        path.add_extension("kdl");
        let doc_str = fs::read_to_string(path)?;
        Self::parse_kdl(doc_str, name, base)
    }

    pub fn parse_kdl(doc_str: String, name: &str, base: &PathBuf) -> Result<Self> {
        let mut s = Self::new(name, base.clone());
        let doc: KdlDocument = doc_str.parse()?;
        for node in doc.nodes().into_iter() {
            dbg!(node.name());
            let ntype = RootType::from_str(node.name().value())?;

            let name = match node.entry(0) {
                Some(t) => t.value().as_string().unwrap_or("object"),
                None => &"anon",
            };

            match ntype {
                RootType::Graphql => (),
                _ => s.add_object(handle_object(node, ntype, name)?),
            };
        }
        Ok(s)
    }
}

impl schema::Object {
    pub fn from_kdl(name: &str, base_type: BaseType) -> Self {
        let o = Self::new(&name, base_type);
        o
    }
}
fn handle_object(node: &KdlNode, rtype: RootType, name: &str) -> Result<schema::Object> {
    let mut o = schema::Object::from_kdl(name, rtype.into());
    if let Some(t) = node.entry(1) {
        o.title = t.value().as_string().unwrap_or_default().to_string();
    }
    // first look at node arguments
    for e in node.entries() {
        if None == e.name() {
            continue;
        }
        match e.name().unwrap().value() {
            "title" => o.title = e.value().as_string().unwrap_or_default().to_string(),
            "icon" => o.icon = e.value().as_string().unwrap_or_default().to_string(),
            "description" => o.description = e.value().as_string().unwrap_or_default().to_string(),
            "preview" => {
                o.preview = Preview::new(e.value().as_string().unwrap_or_default().to_string())
            }
            "fieldsets" => (),
            _ => (),
        }
    }
    // then look at children
    handle_object_fields(&mut o, node.children())?;
    Ok(o)
}
fn handle_object_fields(o: &mut schema::Object, doc: Option<&KdlDocument>) -> Result<()> {
    // let mut fields: Vec<schema::Field> = vec![];
    let Some(doc) = doc else {
        return Ok(());
    };

    // println!(" > object.title? {:?}", &doc.get_arg(".title"));

    for node in doc.nodes().into_iter() {
        // check doc/object nodes with leading dot
        let ftype = node.name().value();
        match ftype {
            "title" => o.title = node.entry(0).unwrap().value().to_string(),
            "icon" => o.icon = node.entry(0).unwrap().value().to_string(),
            "description" => o.description = node.entry(0).unwrap().value().to_string(),
            "preview" => o.preview = handle_preview(node),
            "fieldsets" => (),
            _ => o.fields.push(handle_field(ftype, node)?),
        };
        // old code
        // match (name, node.entry(0)) {
        //    (".title", Some(e)) => o.title = e.value().as_string().unwrap_or_default().to_string(),
    }
    Ok(())
}

fn handle_field(ftype: &str, node: &KdlNode) -> Result<schema::Field> {
    dbg!(ftype);
    let name = node.entry(0).unwrap().value().to_string();

    let t = match FieldType::from_str(ftype) {
        Ok(t) => t,
        _ => FieldType::Object(ftype.to_string()),
    };

    let mut f = schema::Field::new(&name, t);
    Ok(f)
}

fn handle_preview(node: &KdlNode) -> schema::Preview {
    let mut preview = Preview::new("".to_string());
    if let Some(e) = node.entry("title") {
        preview.title = e.value().as_string().unwrap_or_default().to_string();
    }
    if let Some(e) = node.entry("subtitle") {
        preview.subtitle = Some(e.value().as_string().unwrap_or_default().to_string());
    }
    if let Some(e) = node.entry("media") {
        preview.media = Some(e.value().as_string().unwrap_or_default().to_string());
    }
    if let Some(e) = node.entry("js") {
        preview.fun = Some(e.value().as_string().unwrap_or_default().to_string());
    }
    if let Some(doc) = node.children() {
        if let Some(e) = doc.get_arg("title") {
            preview.title = e.as_string().unwrap_or_default().to_string();
        }
        if let Some(e) = doc.get_arg("subtitle") {
            preview.subtitle = Some(e.as_string().unwrap_or_default().to_string());
        }
        if let Some(e) = doc.get_arg("media") {
            preview.media = Some(e.as_string().unwrap_or_default().to_string());
        }
        if let Some(e) = doc.get_arg("js") {
            preview.fun = Some(e.as_string().unwrap_or_default().to_string());
        }
    }
    dbg!("preview handled", &preview);
    preview
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enums() {
        assert_eq!(RootType::from_str("doc").unwrap(), RootType::Doc);
        assert_eq!(RootType::from_str("doc").unwrap(), RootType::Doc);
    }

    #[test]
    fn toplevel() {
        let schema = schema::Schema::parse_kdl(
            r#"doc  person "A Person"  icon=ppl
            object address ; object CEO
            "#
            .to_string(),
            "test",
            &PathBuf::new(),
        )
        .unwrap();
        assert_eq!(schema.name, "test");
        assert_eq!(schema.object_types.len(), 3);
        assert!(schema.object_types.contains(&"person".to_string()));

        let p = schema.get_object("person").unwrap();
        assert_eq!(p.name, "person");
        assert_eq!(p.title, "A Person");
        assert_eq!(p.icon, "ppl");
    }
}
/*
old style:
TOP
            for e in node.entries() {
                if None == e.name() {
                    continue;
                }
                match e.name().unwrap().value() {
                    "title" => o.title = e.value().as_string().unwrap_or_default().to_string(),
                    "icon" => o.icon = e.value().as_string().unwrap_or_default().to_string(),
                    "description" => {
                        o.description = e.value().as_string().unwrap_or_default().to_string()
                    }
                    "preview" => {
                        o.preview =
                            Preview::new(e.value().as_string().unwrap_or_default().to_string())
                    }
                    _ => (),
                }
                // println!("  -- props/args {:?} {:?}", e.name(), e.value());
            }

OBJ
for node in doc.nodes().into_iter() {
        // check doc/object nodes with leading dot
        let name = node.name().value();
        match (name, node.entry(0)) {
            (".title", Some(e)) => o.title = e.value().as_string().unwrap_or_default().to_string(),
            (".icon", Some(e)) => o.icon = e.value().as_string().unwrap_or_default().to_string(),
            (".description", Some(e)) => {
                o.description = e.value().as_string().unwrap_or_default().to_string()
            }
            (".preview", _) => o.preview = handle_preview(node),
            (_, _) => fields.push(handle_field(name)),
        }
    }
    return fields;

mut obj in :
fn handle_object(o: &mut schema::Object, doc: Option<&KdlDocument>) -> Vec<schema::Field> {
*/
