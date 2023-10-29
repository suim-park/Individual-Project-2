extern crate rusqlite;
extern crate csv;
extern crate serde;

use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File;
use serde_derive::Deserialize;
use rusqlite::params;

#[derive(Debug, Deserialize)]
struct Flight {
    year: u32,
    month: String,
    passengers: u32,
}

fn extract(filename: &str) -> Result<Vec<Flight>, Box<dyn Error>> {
    let mut flights = Vec::new();
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let flight: Flight = result?;
        flights.push(flight);
    }
    Ok(flights)
}

fn load(conn: &Connection, flights: &[Flight]) -> Result<(), rusqlite::Error> {
    for flight in flights {
        conn.execute(
            "INSERT INTO flights (year, month, passengers) VALUES (?1, ?2, ?3)",
            params![flight.year, flight.month, flight.passengers],
        )?;
    }
    Ok(())
}

