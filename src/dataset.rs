use crate::schema::{self, Schema};
use crate::store::{Doc, Store};
use std::path::PathBuf;
use turso::{Builder, Connection, Database};

#[derive(Clone)]
pub struct Dataset {
    pub name: String,
    pub schema: Option<Schema>,
    pub store: Store,
    pub assets: PathBuf,
}

impl Dataset {
    pub async fn load(name: String, base: &PathBuf, var: &PathBuf) -> Self {
        Self::setup_paths(&name, var);
        let mut db_path = var.join(name.clone());
        db_path.add_extension("db");
        let db = Builder::new_local(&db_path.to_string_lossy().to_string())
            .build()
            .await
            .unwrap();
        let conn = db.connect().unwrap();
        let schema = schema::Schema::load_kdl(&name, &base);
        if let Err(e) = &schema {
            println!("schema loading failed for {}: {}", &name, e);
        }
        Self {
            name: name.clone(),
            assets: var.join("assets").join(name.clone()),
            schema: schema.ok(),
            store: Store::new(name, conn).await,
        }
    }
    pub fn setup_paths(name: &String, var: &PathBuf) {
        std::fs::create_dir_all(var.join("assets").join(name)).unwrap();
    }
}
