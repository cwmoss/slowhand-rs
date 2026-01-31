use crate::schema;
use kdl::{KdlDocument, KdlValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

impl schema::Schema {
    pub fn load_from_kdl(name: &str, base: &PathBuf) -> Self {
        let mut s = Self::new(name, base);
        let mut path = s.base.join(&s.name);
        path.add_extension("kdl");
        let doc_str = fs::read_to_string(path).unwrap();
        let doc: KdlDocument = doc_str.parse().expect("failed to parse KDL");
        for node in doc.nodes().into_iter() {
            //let ty = node.entry(0).unwrap_or("object")
            let ty = match node.entry(0) {
                Some(t) => t.value().as_string().unwrap_or("object"),
                None => &"object",
            };
            let o = schema::Object::new(node.name().value(), ty);
            s.add_object(o);
            println!("{} type: {}", node.name(), ty);
        }
        s
    }
}

impl schema::Object {
    pub fn from_kdl(name: String, ty: String) -> Self {
        let o = Self::new(&name, &ty);
        o
    }
}
