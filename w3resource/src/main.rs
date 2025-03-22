// Import the rusqlite crate
use rusqlite::{params, Connection, Result}; // For database operations and result handling

fn main() -> Result<()> {
    // Create a database and table
    create_database()?;
    // Insert configs
    insert_config("b827eb4f1eb7", "DS18B20|28d5275600000049|main level")?;
    insert_config("b827eb4f1eb7", "hostname|cheshire")?;

    // Query configs
    println!("Configs in database:");
    query_config()?;

    // Update a config
    update_config(2, "spartan")?;
    query_config()?;

    // Delete a config
    delete_config(2)?;
    query_config()?;
    Ok(())
}

fn create_database() -> Result<()> {
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

fn insert_config(MAC: &str, config: &str) -> Result<()> {
    let conn = Connection::open("config.db")?;

    // Insert a new config
    conn.execute(
        "INSERT INTO ESP_config (MAC, config) VALUES (?1, ?2)",
        params![MAC, config], // Bind parameters
    )?;

    println!("Config inserted successfully.");
    Ok(())
}

fn query_config() -> Result<()> {
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
        //println!("{:?}", conf?.clone());
        //let id = conf.unwrap().id;
        //let MAC = conf.unwrap().MAC;
        let Conf { id: id, MAC: MAC, config: conf } = conf?;
        //println!("id:{}, MAC:{}, ", id, MAC)
        println!("id:{} MAC:{} config:{}", id, MAC, conf);
    }

    Ok(())
}

// Define a struct to map query results
#[derive(Debug, Clone)]
struct Conf {
    id: i32,
    MAC: String,
    config: String,
}

fn update_config(id: i32, new_config: &str) -> Result<()> {
    let conn = Connection::open("config.db")?;

    // Update config's configuration parameters
    conn.execute(
        "UPDATE ESP_config SET config = ?1 WHERE id = ?2",
        params![new_config, id],
    )?;

    println!("Config updated successfully.");
    Ok(())
}

fn delete_config(id: i32) -> Result<()> {
    let conn = Connection::open("config.db")?;

    // Delete a config by ID
    conn.execute("DELETE FROM ESP_config WHERE id = ?1", params![id])?;

    println!("Config deleted successfully.");
    Ok(())
}
