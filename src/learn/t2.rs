use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use std::ops::Deref;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Doc {
    _id: String,
    _type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Document {
    doc: Doc,
    d: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Asset {
    doc: Doc,
    w: usize,
    h: usize,
}
impl Deref for Asset {
    type Target = Doc;

    fn deref(&self) -> &Self::Target {
        &self.doc
    }
}

fn store(d: &Doc) {
    print!("storing {:?}", d);
}

fn main() {
    let a1 = Asset {
        doc: Doc {
            _id: "i1".into(),
            _type: "asset".into(),
        },
        w: 111,
        h: 1222,
    };
    store(&a1);

    println!("{:#?}", a1);
    ()
}
