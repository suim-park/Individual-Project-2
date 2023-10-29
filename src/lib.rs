extern crate rusqlite;
extern crate csv;
extern crate serde;

use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Flight {
    year: u32,
    month: String,
    passengers: u32,
}

fn read_csv(filename: &str) -> Result<Vec<Flight>, Box<dyn Error>> {
    let mut flights = Vec::new();
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let flight: Flight = result?;
        flights.push(flight);
    }
    Ok(flights)
}

fn save_to_db(conn: &Connection, flights: &[Flight]) -> Result<(), rusqlite::Error> {
    for flight in flights {
        conn.execute(
            "INSERT INTO flights (year, month, passengers) VALUES (?1, ?2, ?3)",
            params![flight.year, flight.month, flight.passengers],
        )?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("flights.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS flights (year INTEGER, month TEXT, passengers INTEGER)",
        params![],
    )?;

    let flights = read_csv("flights.csv")?;

    save_to_db(&conn, &flights)?;

    Ok(())
}
