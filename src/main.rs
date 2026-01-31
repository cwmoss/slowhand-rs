pub mod config;
pub mod dataset;
pub mod kdl_schema;
pub mod schema;
pub mod store;

use config::Config;
use turso::Builder;

use crate::{schema::Schema, store::Doc};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = Config::new();

    let t1 = r#"
        {
            
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    let t2 = r#"
        {
            
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "_id":"jdoe",
            "_type": "person"
        }"#;
    let d1: Doc = serde_json::from_str(t1)?;
    dbg!(&d1);
    print!("doc json: {}", serde_json::to_string(&d1).unwrap());
    let s = "movies";
    let schema = Schema::load_from_kdl(s, &conf.projects);
    println!("{}", json!(schema));
    conf.setup();
    Ok(())
}

/*

https://kdl.dev/play/
https://crates.io/crates/knus/
https://github.com/bearcove/styx


*/
