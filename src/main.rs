pub mod config;
pub mod dataset;
pub mod schema;
pub mod store;

use config::Config;
use turso::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = Config::new();
    conf.setup();
    Ok(())
}
