use rusqlite::{params, Connection, Result};
use std::fs::File;
use std::io::Write;

pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // HTTP 요청을 통해 URL에서 데이터를 가져옵니다.
    let response = reqwest::blocking::get(url)?;

    // 가져온 데이터의 내용을 읽습니다.
    let content = response.text()?;

    // 지정된 경로에 파일을 생성하고 내용을 기록합니다.
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn transform(csv_path: &str, db_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // CSV 파일을 엽니다.
    let mut rdr = csv::Reader::from_path(csv_path)?;

    // SQLite 데이터베이스 파일을 생성하거나 연결합니다.
    let conn = Connection::open(db_path)?;

    // 적절한 테이블을 생성합니다. 이 예제에서는 단순한 구조로 가정합니다.
    // 실제 사용 사례에 따라 테이블 구조를 조정할 필요가 있을 수 있습니다.
    conn.execute(
        "CREATE TABLE IF NOT EXISTS data (year INTEGER, month TEXT, passengers INTEGER)", // 컬럼 이름과 타입은 실제 CSV 구조에 맞게 조정하세요.
        [],
    )?;

    for result in rdr.deserialize() {
        let (year, month, passengers): (i32, String, i32) = result?; // 타입과 변수는 실제 CSV 구조에 따라 조정하세요.
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
    // SQLite 데이터베이스 파일에 연결합니다.
    let conn = rusqlite::Connection::open(db_path)?;

    // 데이터를 `data` 테이블에 삽입합니다.
    conn.execute(
        "INSERT INTO data (year, month, passengers) VALUES (?1, ?2, ?3)",
        params![year, month, passengers],
    )?;

    println!("Data successfully created into the database.");

    Ok(())
}

pub fn read(db_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // SQLite 데이터베이스 파일에 연결합니다.
    let conn = rusqlite::Connection::open(db_path)?;

    // 쿼리를 실행하여 결과를 가져옵니다.
    let mut stmt = conn.prepare("SELECT year, month, passengers FROM data")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>("year")?,
            row.get::<_, String>("month")?,
            row.get::<_, i32>("passengers")?,
        ))
    })?;

    // 결과를 출력합니다.
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
    // SQLite 데이터베이스 파일에 연결합니다.
    let conn = rusqlite::Connection::open(db_path)?;

    // 지정된 연도와 월에 해당하는 데이터의 승객 수를 변경합니다.
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
