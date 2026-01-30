use facet::Facet;
use std::fs;

#[derive(Debug, Facet)]
struct Doc {
    result: MyResult,
}

#[derive(Debug, Facet)]
enum MyResult {
    MErr(MyError),
}
#[derive(Debug, Facet)]
struct MyError {
    message: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("src/sdl/config.styx")?;
    // let config = facet_styx::from_str::<Config>(&content);
    match facet_styx::from_str::<Doc>(&content) {
        Ok(config) => println!("Port: {:?}", config),
        Err(e) => {
            // Error includes span information for nice diagnostics
            eprintln!("Configuration error: {}", e);
        }
    }
    Ok(())
}
