#![allow(non_snake_case)]

use rusqlite::{params, Connection, Result}; // For database operations and result handling

// Define a struct to map query results
#[derive(Debug, Clone)]
pub struct Conf {
    pub id: i32,
    pub MAC: String,
    pub config: String,
}

pub fn create_database() -> Result<()> {
    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn = Connection::open("config.db")?;

    // Create a table named configs
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ESP_config (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            MAC TEXT NOT NULL,
            config TEXT NOT NULL
        )",
        [], // No parameters needed
    )?;

    println!("Database and table created successfully.");
    Ok(())
}

pub fn insert_config(MAC: &str, config: &str) -> Result<()> {
    let conn = Connection::open("config.db")?;

    // Insert a new config
    conn.execute(
        "INSERT INTO ESP_config (MAC, config) VALUES (?1, ?2)",
        params![MAC, config], // Bind parameters
    )?;

    println!("Config inserted successfully.");
    Ok(())
}

pub fn query_config(res: &mut Vec<Conf>) -> Result<()> {
    let conn = Connection::open("config.db")?;


// Retrieve data from configs table
    let mut stmt = conn.prepare("SELECT id, MAC, config FROM ESP_config")?;
    let conf_iter = stmt.query_map([], |row| {
        Ok(Conf {
            id: row.get(0)?,
            MAC: row.get(1)?,
            config: row.get(2)?,
        })
    })?;


// Iterate over the retrieved rows
    for conf in conf_iter {
        let Conf {
            id,
            MAC,
            config: conf,
        } = conf?;

        res.push(Conf {
            id,
            MAC,
            config: conf,
        });
    }

    Ok(())
}
pub fn update_config(id: i32, new_config: &str) -> Result<()> {
    let conn = Connection::open("config.db")?;

    // Update config's configuration parameters
    conn.execute(
        "UPDATE ESP_config SET config = ?1 WHERE id = ?2",
        params![new_config, id],
    )?;

    println!("Config updated successfully.");
    Ok(())
}

pub fn delete_config(id: i32) -> Result<()> {
    let conn = Connection::open("config.db")?;

    // Delete a config by ID
    conn.execute("DELETE FROM ESP_config WHERE id = ?1", params![id])?;

    println!("Config deleted successfully.");
    Ok(())
}


/*
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/