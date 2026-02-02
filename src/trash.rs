let db = Builder::new_local("sqlite.db").build().await?;
    let conn = db.connect()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        username TEXT NOT NULL,
        email TEXT
    )",
        (),
    )
    .await?;

    conn.execute("INSERT INTO users (username) VALUES (?)", ("alice",))
        .await?;
    let rows_affected = conn
        .execute("INSERT INTO users (username) VALUES (?)", ("bob",))
        .await?;

    println!("Inserted {} rows", rows_affected);

    let mut rows = conn.query("SELECT * FROM users", ()).await?;

    while let Some(row) = rows.next().await? {
        let id = row.get_value(0)?;
        let name = row.get_value(1)?;
        let email = row.get_value(2)?;
        println!(
            "User: {} - {} ({})",
            id.as_integer().unwrap_or(&0),
            name.as_text().unwrap_or(&"".to_string()),
            email.as_text().unwrap_or(&"".to_string())
        );
    }



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



    /*let json = row?
            .get_value(2)
            .ok()?
            .as_text()
            .unwrap_or(&"{}".to_string());*/
            /*
                        let json: String = row?.get(2).unwrap();
                        let jstr = serde_json::from_str(&json).ok().unwrap();
            */
            // Some(Doc {
            //     _id: "123".to_string(),
            //     _type: "_idk".to_string(),
            //     d: Value::Null,
            // })
            let json: String = row?.get(2).unwrap();
            Some(Doc {
                _id: row?.get(0).unwrap(),
                //row?.get_value(0)?.as_text().unwrap_or(&"".to_string()),
                _type: row?.get(1).unwrap(),
                d: row?.get_value(2).unwrap(), // serde_json::from_str(&json).ok().unwrap(),
            })

            /*
            println!(
                "User: {} - {} ({})",
                id.as_integer().unwrap_or(&0),
                name.as_text().unwrap_or(&"".to_string()),
                email.as_text().unwrap_or(&"".to_string())
            );*/