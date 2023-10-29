extern crate rusqlite;
extern crate csv;
extern crate serde;

use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Flight {
    year: u32,
    month: String,
    passengers: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // SQLite 데이터베이스 연결 및 테이블 생성
    let conn = Connection::open("flights.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS flights (year INTEGER, month TEXT, passengers INTEGER)",
        params![],
    )?;

    let file = File::open("flights.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let flight: Flight = result?;

        conn.execute(
            "INSERT INTO flights (year, month, passengers) VALUES (?1, ?2, ?3)",
            params![flight.year, flight.month, flight.passengers],
        )?;
    }

    Ok(())
}