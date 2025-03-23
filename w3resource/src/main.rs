#![allow(non_snake_case)]

// Import the rusqlite crate
use rusqlite::Result; // For database operations and result handling

use db;

fn main() -> Result<()> {
    // Create a database and table
    db::create_database()?;
    // Insert configs
    db::insert_config("b827eb4f1eb7", "DS18B20|28d5275600000049|main level")?;
    db::insert_config("b827eb4f1eb7", "hostname|cheshire")?;

    // Query configs
    println!("Configs in database:");
    println!("Configs in database:");
    let mut result = Vec::with_capacity(1000);
    db::query_config(&mut result)?;

    for conf in result {
        println!("id: {}, MAC: {}, config {}", conf.id, conf.MAC, conf.config);
    }

    let mut result = Vec::with_capacity(1000);
    db::query_config(&mut result)?;
    for conf in result {
        println!("id: {}, MAC: {}, config {}", conf.id, conf.MAC, conf.config);
    }

    /*
    db::query_config()?;

    // Update a config
    db::update_config(2, "spartan")?;
    db::query_config()?;

    // Delete a config
    db::delete_config(2)?;
    db::query_config()?;
    */
    Ok(())
}
