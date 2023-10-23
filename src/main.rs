extern crate rusqlite;
use rusqlite::{params, Connection, Result};

fn connect_to_database() -> Result<Connection> {
    let conn = Connection::open("GroceryDB.db")?;
    println!("Connected to SQLite database");
    Ok(conn)
}

fn create_operation() -> Result<()> {
    let conn = Connection::open("newDB.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS employees (
            id INTEGER PRIMARY KEY,
            name TEXT,
            department TEXT
        )",
        params![],
    )?;
    println!("Table created successfully!");
    Ok(())
}

fn read_operation() -> Result<()> {
    let conn = Connection::open("GroceryDB.db")?;
    let mut stmt = conn.prepare("SELECT * FROM GroceryDB LIMIT 5;")?;
    let rows = stmt.query_map(params![], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
        ))
    })?;

    println!("Data in the table:");
    for row in rows {
        println!("{:?}", row?);
    }

    Ok(())
}

fn update_operation() -> Result<()> {
    let conn = Connection::open("GroceryDB.db")?;
    let data_to_insert = vec![
        // your data here...
        // example: ("general name", "count_products", ...),
    ];

    for data in &data_to_insert {
        conn.execute(
            "INSERT INTO GroceryDB (general_name, count_products, ingred_FPro, avg_FPro_products, 
            avg_distance_root, ingred_normalization_term, semantic_tree_name, 
            semantic_tree_node) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![data.0, data.1, data.2, data.3, data.4, data.5, data.6, data.7],
        )?;
    }

    println!("Record updated successfully!");
    Ok(())
}

fn delete_operation() -> Result<()> {
    let conn = Connection::open("GroceryDB.db")?;
    conn.execute("DELETE FROM GroceryDB WHERE general_name=?", params!["yellow bell pepper"])?;
    println!("Record deleted successfully!");
    Ok(())
}

fn main() {
    connect_to_database().unwrap();
    create_operation().unwrap();
    read_operation().unwrap();
    update_operation().unwrap();
    delete_operation().unwrap();
}
