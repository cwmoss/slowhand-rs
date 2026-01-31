use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct Store {
    name: String,
}

impl Store {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Doc {
    #[serde(default = "Doc::gen_id")]
    pub _id: String,
    #[serde(default = "Doc::default_type")]
    pub _type: String,
    #[serde(flatten)]
    pub d: Value,
}

impl Doc {
    pub fn gen_id() -> String {
        let mut buf = vec![0u8; 16];
        rand::rng().fill_bytes(&mut buf);
        let b64 = general_purpose::STANDARD.encode(&buf);
        b64.trim_end_matches('=')
            .replace('+', "-")
            .replace('/', "_")
    }
    pub fn default_type() -> String {
        "_idk".to_string()
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
