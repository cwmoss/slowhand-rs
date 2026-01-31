pub mod config;
pub mod dataset;
pub mod kdl_schema;
pub mod schema;
pub mod store;

use config::Config;
use turso::Builder;

use crate::schema::Schema;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = Config::new();
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
