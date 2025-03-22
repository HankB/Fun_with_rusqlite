use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Config {
    id: String,
    conf_tokens: String,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE ESP_config (
            MAC text not null,
            config text not null
        )",
        (), // empty list of parameters.
    )?;
    let conf = Config {
        id: "b827eb4f1eb7".to_string(),
        conf_tokens: "DS18B20|28d5275600000049|main level".to_string(),
    };
    conn.execute(
        "INSERT INTO ESP_config (MAC, config) VALUES (?1, ?2)",
        (&conf.id, &conf.conf_tokens),
    )?;
    let mut stmt = conn.prepare("SELECT MAC, config FROM ESP_config")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Config {
            id: row.get(0)?,
            conf_tokens: row.get(1)?,
        })
    })?;
    for person in person_iter {
        println!("Found config {:?}", person.unwrap());
        // println!("MAC:{}", person.unwrap().id);
        // How to access the individual components?
    }

    Ok(())
}
