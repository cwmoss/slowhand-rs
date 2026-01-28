pub struct Store {
    name: String,
}

impl Store {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

fn get_migrations() -> Vec<String> {
    include_str!("../resources/schema.sql")
        .split("----")
        .filter_map(|p| {
            if p.trim().starts_with("#") {
                None
            } else {
                Some(p.trim().to_string())
            }
        })
        .collect()
}
