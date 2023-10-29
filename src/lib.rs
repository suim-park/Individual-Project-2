use std::fs::File;
use std::io::Write;
use reqwest;
use rusqlite::{params, Connection};

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

    for result  in rdr.deserialize() {
        let (year, month, passengers): (i32, String, i32) = result?;  // 타입과 변수는 실제 CSV 구조에 따라 조정하세요.
        conn.execute(
            "INSERT INTO data (year, month, passengers) VALUES (?1, ?2, ?3)",
            params![year, month, passengers],
        )?;
    }

    Ok(())
}