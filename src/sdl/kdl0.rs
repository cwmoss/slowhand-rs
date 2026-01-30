#[derive(knus::Decode)]

enum TopLevelNode {
    Route(Route),
    Plugin(Plugin),
}

#[derive(knus::Decode)]
struct Route {
    #[knus(argument)]
    path: String,
    #[knus(children(name = "route"))]
    subroutes: Vec<Route>,
}

#[derive(knus::Decode)]
struct Plugin {
    #[knus(argument)]
    name: String,
    #[knus(property)]
    url: String,
}

fn main() -> miette::Result<()> {
    let config = knus::parse::<Vec<TopLevelNode>>(
        "example.kdl",
        r#"
    route "/api" {
        route "/api/v1"
    }
    plugin "http" url="https://example.org/http"
"#,
    )?;
    Ok(())
}
