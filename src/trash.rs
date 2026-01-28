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