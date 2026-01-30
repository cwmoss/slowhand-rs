use kdl::{KdlDocument, KdlValue};
use serde_json::json;
use std::fs;

fn main() {
    let doc_str0 = r#"
hello 1 2 3

// Comment
world prop=string-value {
    child 1
    child 2
    child #inf
}
"#;
    let doc_str = fs::read_to_string("js/schema.kdl").unwrap();
    let doc: KdlDocument = doc_str.parse().expect("failed to parse KDL");
    let printfomat = "the doc: \n{}";
    println!("the doc: \n{}", doc);
    // println!("json: {}", json!({document: doc}));
    // dbg!(doc);
    return;
    assert_eq!(
        doc.iter_args("hello").collect::<Vec<&KdlValue>>(),
        vec![&1.into(), &2.into(), &3.into()]
    );

    assert_eq!(
        doc.get("world").map(|node| &node["prop"]),
        Some(&"string-value".into())
    );

    // Documents fully roundtrip:
    assert_eq!(doc.to_string(), doc_str);
}
