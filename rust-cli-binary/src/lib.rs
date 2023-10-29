use rusqlite::{params, Connection, Result};
use std::fs::File;
use std::io::Write;

pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch data from the URL via an HTTP request.
    let response = reqwest::blocking::get(url)?;

    // Read the content of the fetched data.
    let content = response.text()?;

    // Create a file at the specified path and write the content.
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn transform(csv_path: &str, db_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the CSV file.
    let mut rdr = csv::Reader::from_path(csv_path)?;

    // Create or connect to an SQLite database file.
    let conn = Connection::open(db_path)?;

    // Create the appropriate table. This example assumes a simple structure.
    // Adjust the table structure according to your actual use case.
    conn.execute(
        "CREATE TABLE IF NOT EXISTS data (year INTEGER, month TEXT, passengers INTEGER)", 
        [],
    )?;

    for result in rdr.deserialize() {
        let (year, month, passengers): (i32, String, i32) = result?;
        conn.execute(
            "INSERT INTO data (year, month, passengers) VALUES (?1, ?2, ?3)",
            params![year, month, passengers],
        )?;
    }

    Ok(())
}

pub fn create(
    db_path: &str,
    year: i32,
    month: &str,
    passengers: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the SQLite database file.
    let conn = rusqlite::Connection::open(db_path)?;

    // Insert data into the `data` table.
    conn.execute(
        "INSERT INTO data (year, month, passengers) VALUES (?1, ?2, ?3)",
        params![year, month, passengers],
    )?;

    println!("Data successfully created into the database.");

    Ok(())
}

pub fn read(db_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the SQLite database file.
    let conn = rusqlite::Connection::open(db_path)?;

    // Execute the query and fetch the results.
    let mut stmt = conn.prepare("SELECT year, month, passengers FROM data")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>("year")?,
            row.get::<_, String>("month")?,
            row.get::<_, i32>("passengers")?,
        ))
    })?;

    // Print the results.
    for row_result in rows {
        let (year, month, passengers) = row_result?;
        println!("{} - {}: {}", year, month, passengers);
    }

    Ok(())
}

pub fn update(
    db_path: &str,
    year: i32,
    month: &str,
    passengers: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the SQLite database file.
    let conn = rusqlite::Connection::open(db_path)?;

    // Update the number of passengers for the specified year and month.
    let rows_modified = conn.execute(
        "UPDATE data SET passengers = ?3 WHERE year = ?1 AND month = ?2",
        params![year, month, passengers],
    )?;

    if rows_modified == 0 {
        println!("No data found for the specified year and month.");
    } else {
        println!("Data successfully updated in the database.");
    }

    Ok(())
}

pub fn delete(db_path: &str, year: i32) -> Result<()> {
    let conn = Connection::open(db_path)?;

    conn.execute("DELETE FROM data WHERE year = ?1", params![year])?;

    Ok(())
}
