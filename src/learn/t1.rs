use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::{self, json};
use std::fmt::Debug;
use std::ops::Deref;

trait Storable {
    fn id(&self) -> &String;
    fn ty(&self) -> &String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Document {
    _id: String,
    _type: String,
    #[serde(flatten)]
    d: Value,
}

impl Storable for Document {
    fn id(&self) -> &String {
        &self._id
    }
    fn ty(&self) -> &String {
        &self._type
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Asset {
    _id: String,
    _type: String,
    w: usize,
    h: usize,
    #[serde(flatten)]
    d: Option<Value>,
}
impl Storable for Asset {
    fn id(&self) -> &String {
        &self._id
    }
    fn ty(&self) -> &String {
        &self._type
    }
}
fn store0(d: &impl Storable) {
    let id = d.id();
    println!("storing {:?} {}", id, d.ty());
}
fn store_save<T: Storable + Debug + serde::Serialize>(doc: &T) {
    let id = doc.id();
    let body = serde_json::to_string(&doc).unwrap();
    println!("storing {:?} {}", id, body);
}

const T1: &str = r#"{"_createdAt":"2018-06-13T08:57:45Z","_id":"person_alex-cameron","_rev":"BLPnjZv07vOwXtY5LQbIj6","_type":"person","_updatedAt":"2018-06-13T08:57:45Z","name":"Alex Cameron","slug":{"_type":"slug","current":"alex-cameron","source":"name"}}"#;
const T2: &str = r#"{"_createdAt":"2018-06-13T08:57:45Z","_id":"person_alex-cameron","_rev":"BLPnjZv07vOwXtY5LQbIj6","_type":"asset","_updatedAt":"2018-06-13T08:57:45Z","w":1000,"h":600}"#;

fn store_get1() -> Option<Document> {
    // serde_json::from_str(T1.into()).ok().unwrap()
    match serde_json::from_str::<Document>(T1) {
        Ok(doc) => Some(doc),
        Err(e) => {
            println!("loading error {:?}", e);
            None
        }
    }
}

fn store_get(id: String) -> Result<Option<Document>, serde_json::error::Error> {
    if id == "idk" {
        return Ok(None::<Document>);
    }
    // serde_json::from_str(T1.into()).ok().unwrap()
    serde_json::from_str(T1)
}

fn store_get_asset(id: String) -> Result<Option<Asset>, serde_json::error::Error> {
    if id == "idk" {
        return Ok(None::<Asset>);
    }
    // serde_json::from_str(T1.into()).ok().unwrap()
    serde_json::from_str(T2)
}

type DocRes = Result<Option<Document>, serde_json::error::Error>;

fn main() {
    let a1 = Asset {
        _id: "i1".into(),
        _type: "asset".into(),

        w: 111,
        h: 1222,
        d: None,
    };
    let d1 = Document {
        _id: "d2".into(),
        _type: "wichtig".into(),
        d: json!({"title":"superwichtig"}),
    };
    store_save(&a1);
    store_save(&d1);
    let l1: DocRes = store_get("23".into()).into();
    println!("{:#?}", l1);

    let l2 = store_get_asset("23".into());
    println!("{:#?}", l2);
    ()
}
