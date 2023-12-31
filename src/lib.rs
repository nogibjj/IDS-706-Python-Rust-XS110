extern crate rusqlite;
use rusqlite::{params, Connection, Result};
pub fn connect_to_database() -> Result<Connection> {
    let conn = Connection::open("GroceryDB.db")?;
    println!("Connected to SQLite database");
    Ok(conn)
}

pub fn create_operation() -> Result<()> {
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

pub fn read_operation() -> Result<()> {
    let conn = Connection::open("GroceryDB.db")?;
    let mut stmt = conn.prepare("SELECT * FROM GroceryDB LIMIT 5;")?;
    let rows = stmt.query_map(params![], |row| {
        Ok((
            row.get::<usize, String>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?,
            row.get::<usize, String>(3)?,
            row.get::<usize, String>(4)?,
            row.get::<usize, String>(5)?,
            row.get::<usize, String>(6)?,
            row.get::<usize, String>(7)?,
        ))
    })?;

    println!("Data in the table:");
    for row in rows {
        println!("{:?}", row?);
    }

    Ok(())
}

pub fn delete_operation() -> Result<()> {
    let conn = Connection::open("GroceryDB.db")?;
    conn.execute(
        "DELETE FROM GroceryDB WHERE general_name=?",
        params!["yellow bell pepper"],
    )?;
    println!("Record deleted successfully!");
    Ok(())
}
pub fn update_operation() -> Result<()> {
    let mut conn = Connection::open("GroceryDB.db")?;

    let data_to_insert = vec![
        (
            "general name",
            "count_products",
            "ingred_FPro",
            "avg_FPro_products",
            "avg_distance_root",
            "ingred_normalization_term",
            "semantic_tree_name",
            "semantic_tree_node",
        ),
        (
            "arabica coffee",
            "21",
            "0.18903204038025467",
            "0.2754401549508692",
            "2.0476190476190474",
            "15.16666666666667",
            "",
            "",
        ),
        (
            "grape tomatoes",
            "18",
            "0.21119429773632484",
            "0.4212998456790123",
            "3.111111111111111",
            "10.594047619047616",
            "",
            "",
        ),
    ];

    let tx = conn.transaction()?;
    for data in &data_to_insert {
        tx.execute(
            "INSERT INTO GroceryDB 
            (general_name, count_products, ingred_FPro, avg_FPro_products, 
            avg_distance_root, ingred_normalization_term, semantic_tree_name, 
            semantic_tree_node) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![data.0, data.1, data.2, data.3, data.4, data.5, data.6, data.7],
        )?;
    }
    tx.commit()?;

    println!("Record updated successfully!");
    Ok(())
}