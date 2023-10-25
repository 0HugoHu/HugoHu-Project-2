// lib.rs

extern crate mysql;
use my::prelude::*;
use mysql as my;

pub fn run(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pool = my::Pool::new(database_url)?;

    // Create table if it doesn't exist
    let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS items (
            id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT
        )
    "#;

    pool.get_conn()?.query_drop(create_table_query)?;

    // Insert a new item
    let new_item = ("Example Item", "This is an example description");
    let insert_query = "INSERT INTO items (name, description) VALUES (?, ?)";
    let mut conn = pool.get_conn()?;
    conn.exec_drop(insert_query, new_item)?;

    println!("Items in the database after insertion:");
    display_items(&mut conn)?;

    // Update an item
    let update_item = ("New Name", 1);
    let update_query = "UPDATE items SET name = ? WHERE id = ?";
    let mut conn = pool.get_conn()?; // Re-obtain a mutable connection
    conn.exec_drop(update_query, update_item)?;

    println!("Items in the database after update:");
    display_items(&mut conn)?;

    // Delete an item
    let item_id_to_delete = 1;
    let delete_query = "DELETE FROM items WHERE id = ?";
    let mut conn = pool.get_conn()?; // Re-obtain a mutable connection
    conn.exec_drop(delete_query, (item_id_to_delete,))?;

    println!("Items in the database after deletion:");
    display_items(&mut conn)?;

    // Drop the items table
    let drop_query = "DROP TABLE items";
    conn.query_drop(drop_query)?;

    Ok(())
}

fn display_items(conn: &mut my::PooledConn) -> Result<(), Box<dyn std::error::Error>> {
    let select_query = "SELECT id, name, description FROM items";
    let items: Vec<(i32, String, Option<String>)> = conn
        .query_map(select_query, |(id, name, description)| {
            (id, name, description)
        })?;

    for item in &items {
        println!(
            "Item: id={}, name={}, description={:?}",
            item.0, item.1, item.2
        );
    }

    if items.is_empty() {
        println!("No items found.");
    }

    Ok(())
}
