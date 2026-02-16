use std::collections::HashMap;

use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use turso::Connection;

#[derive(Clone)]
pub struct Store {
    name: String,
    conn: Connection,
}

// https://gist.github.com/green-s/fbd0d374b290781ac9b3f8ff03e3245d
impl Store {
    pub async fn get_docs(&self, ids: Vec<&str>) -> Vec<Doc> {
        let placeholder = ids
            .iter()
            .map(|_| "?".to_string())
            .collect::<Vec<String>>()
            .join(",");
        let mut res = Vec::new();
        let q = format!("SELECT body as d FROM docs WHERE _id IN ({placeholder})");
        dbg!(&q);
        let Some(mut rows) = self.conn.query(q, ids).await.ok() else {
            return res;
        };

        while let Some(row) = rows.next().await.unwrap() {
            // if let Some(r) = row {
            // dbg!(&row);
            let j = row.get_value(0).unwrap();
            let jt = j.as_text().unwrap();
            let doc: Doc = serde_json::from_str(jt).ok().unwrap();
            res.push(doc);
        }
        res
    }

    pub async fn get_doc(&self, id: String) -> Result<Option<Doc>, serde_json::error::Error> {
        let rows = self
            .conn
            .query("SELECT body as d FROM docs WHERE _id=?", ((id),))
            .await
            .ok();
        let Some(mut rows) = rows else {
            return Ok(None);
        };
        if let Some(Some(row)) = rows.next().await.ok() {
            let j = row.get_value(0).unwrap();
            let jt = j.as_text().unwrap();
            serde_json::from_str(jt)
        } else {
            Ok(None)
        }
    }

    pub async fn create_or_replace(&self, doc: Doc) -> Result<u64> {
        /*
        if ($keep_modify_date) {
            if (!isset($row['_updatedAt']) || !$row['_updatedAt']) {
                $row['_updatedAt'] = date("Y-m-d H:i:s");
            }
        } else {
            $row['_updatedAt'] = date("Y-m-d H:i:s");
        }
        $row = $this->defaults($row);
        */
        if self.exists(&doc._id).await {
            println!("update");
            self.update(doc).await
        } else {
            println!("insert");
            self.insert(doc).await
        }
    }

    pub async fn update(&self, doc: Doc) -> Result<u64> {
        let body = serde_json::to_string(&doc).unwrap();
        let rows_affected = self
            .conn
            .execute(
                "UPDATE docs SET _type=?, body=? WHERE _id=?",
                (doc._type, body, doc._id),
            )
            .await?;

        Ok(rows_affected)
    }
    /*
    conn.execute("INSERT INTO users (username) VALUES (?)", ("alice",))
            .await?;
        let rows_affected = conn
            .execute("INSERT INTO users (username) VALUES (?)", ("bob",))
            .await?;

        println!("Inserted {} rows", rows_affected);
    */
    pub async fn insert(&self, doc: Doc) -> Result<u64> {
        let body = serde_json::to_string(&doc).unwrap();
        let rows_affected = self
            .conn
            .execute(
                "INSERT INTO docs (_id, _type, _createdAt, _updatedAt, _rev, body, _btext) VALUES (?, ?, ?, ?, ?, ?, ?)",
                (doc._id, doc._type, "", "", "", body, ""),
            )
            .await?;
        println!("#rows {}", rows_affected);
        Ok(rows_affected)
    }

    pub async fn exists(&self, id: &str) -> bool {
        let rows = self
            .conn
            .query("SELECT count(_id) from docs WHERE _id=?", [id])
            .await
            .ok();

        if let Some(r) = rows.expect("exists query failed").next().await.ok() {
            let total: u64 = r.expect("row fetch failed").get(0).unwrap();
            total > 0
        } else {
            false
        }
    }

    pub async fn new(name: String, conn: Connection) -> Self {
        conn.execute_batch(get_migrations().join(";\n"))
            .await
            .unwrap();
        Self { name, conn }
    }
}

trait Storable {
    fn id(&self) -> &String;
    fn ty(&self) -> &String;
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

impl Storable for Doc {
    fn id(&self) -> &String {
        &self._id
    }
    fn ty(&self) -> &String {
        &self._type
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    _id: String,
    _type: String,
    asset_id: String,
    extension: String,
    mime_type: String,
    original_filename: String,
    path: String,
    upload_id: String,
    size: usize,
    sha1hash: String,
    url: String,
    width: usize,
    height: usize,
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
