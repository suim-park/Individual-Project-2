fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("flights.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS flights (year INTEGER, month TEXT, passengers INTEGER)",
        params![],
    )?;

    let flights = extract("flights.csv")?;

    load(&conn, &flights)?;

    Ok(())
}
