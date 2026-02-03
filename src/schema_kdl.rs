use crate::schema::{self, BaseType, DOCOPTS};
use async_graphql::dynamic::Object;
use kdl::{KdlDocument, KdlValue};
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;
//use std::error::Error;
use anyhow::Result;
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
        Ok(s)
    }
}

impl schema::Object {
    pub fn from_kdl(name: &str, base_type: BaseType) -> Self {
        let o = Self::new(&name, base_type);
        o
    }
}

fn handle_object(o: Object, doc: Option<KdlDocument>) {
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
}
