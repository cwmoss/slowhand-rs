use crate::schema::{self, BaseType, FieldType, Preview, DOCOPTS};
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

enum SchemaError {
    KdlError,
    IoError,
}
impl schema::Schema {
    pub fn load_from_kdl(name: &str, base: &PathBuf) -> Result<Self> {
        let fopts: Vec<String> = "title hidden readOnly description icon initialValue preview"
            .to_string()
            .split(" ")
            .map(|s| s.to_string())
            .collect();

        let mut s = Self::new(name, base);
        let mut path = s.base.join(&s.name);
        path.add_extension("kdl");
        let doc_str = fs::read_to_string(path)?;
        let doc: KdlDocument = doc_str.parse()?;
        for node in doc.nodes().into_iter() {
            //let ty = node.entry(0).unwrap_or("object")

            let base_type = match node.entry(0) {
                Some(t) => t.value().as_string().unwrap_or("object"),
                None => &"object",
            };
            let mut o = schema::Object::from_kdl(
                node.name().value(),
                BaseType::from_str(base_type).expect("unable to parse object type"),
            );

            if let Some(t) = node.entry(1) {
                o.title = t.value().as_string().unwrap_or_default().to_string();
            }

            println!("{} type: {}", node.name(), base_type);

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
            // let fields: Vec<Field> = vec![];
            println!(" > down to object {}", o.name);
            let fields = handle_object(&mut o, node.children());
            o.fields = fields;
            s.add_object(o);
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

fn handle_object(o: &mut schema::Object, doc: Option<&KdlDocument>) -> Vec<schema::Field> {
    let mut fields: Vec<schema::Field> = vec![];
    let Some(doc) = doc else {
        return fields;
    };

    println!(" > object.title? {:?}", &doc.get_arg(".title"));

    // let name = "popularity";

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
        /*
        for e in node.entries() {
            // property entry
            if None == e.name() {
                continue;
            }
            println!(" > object name {:?}", &e.name());
            // println!("  -- props/args {:?} {:?}", e.name(), e.value());
        }*/
        // fields.push(schema::Field::new(name, FieldType::String));
    }
    return fields;
}

fn handle_field(name: &str) -> schema::Field {
    schema::Field::new(name, FieldType::String)
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
    }
    dbg!("preview handled", &preview);
    preview
}
/*

    let Some(doc) = doc else {
        return;
    };
    for node in doc.nodes().into_iter() {
        //let ty = node.entry(0).unwrap_or("object")

        let field_type = match node.entry(0) {
            Some(t) => t.value().as_string().unwrap_or("object"),
            None => &"object",
        };
        let mut o = schema::Object::from_kdl(
            node.name().value(),
            BaseType::from_str(base_type).expect("unable to parse object type"),
        );

        if let Some(t) = node.entry("title") {
            // o.title = t.value().as_string().unwrap_or_default().to_string();
        }
        let t = "title";
        for k in DOCOPTS {
            println!("test string {}", k == t);
        }

        for e in node.entries() {
            if None == e.name() {
                continue;
            }
            match e.name().unwrap().value() {
                "title" => o.title = e.value().as_string().unwrap_or_default().to_string(),
                "icon" => o.icon = e.value().as_string().unwrap_or_default().to_string(),
                _ => (),
            }
            println!("  -- props/args {:?} {:?}", e.name(), e.value());
        }
        handle_object(o, node.children());
        s.add_object(o);
        println!("{} type: {}", node.name(), base_type);
    }

*/

/*

          let t = "title";
            for k in DOCOPTS {
                println!("test string {}", k == t);
            }


*/
