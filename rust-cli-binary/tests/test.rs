#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::fs;
    use std::io::Write;
    use rusqlite::{Connection, OpenFlags};
    use rusqlite::params;
    use rust_cli_binary::{extract,transform,create,read,update,delete};

    #[test]
    fn test_extract() {
        // 테스트용 URL과 저장 경로를 정의합니다.
        let test_url = "https://github.com/suim-park/Individual-Project-2/raw/main/rust-cli-binary/test_flights.csv"; // 이 URL은 예제입니다. 실제로 액세스 가능한 URL로 바꿔주세요.
        let test_path = "test_flights.csv";

        // extract 함수를 실행합니다.
        let result = extract(test_url, test_path);

        // 결과가 Ok(())인지 확인합니다.
        assert!(result.is_ok(), "Extract function failed with {:?}", result);

        // 파일이 실제로 생성되었는지 확인합니다.
        assert!(fs::metadata(test_path).is_ok(), "Failed to create the file at {}", test_path);

    }

    #[test]
    fn test_transform() {
        // 샘플 CSV 데이터를 생성합니다.
        let csv_path = "test_flights.csv";
        let db_path = "test_flightsDB.db";

        // transform 함수를 실행합니다.
        let result = transform(csv_path, db_path);

        // 결과가 Ok(())인지 확인합니다.
        assert!(result.is_ok(), "Transform function failed with {:?}", result);
    }

    #[test]
    fn test_create() {
        let db_path = "test_flightsDB.db";

        // 데이터베이스에 데이터를 생성합니다.
        let result = create(db_path, 2023, "March", 200);
        assert!(result.is_ok(), "Create function failed with {:?}", result);

        // 생성된 데이터를 검증합니다.
        let conn = Connection::open(db_path).unwrap();
        let passengers: i32 = conn.query_row("SELECT passengers FROM test_flightsDB WHERE year = 2023 AND month = 'March'", params![], |row| row.get(0)).unwrap();
        assert_eq!(passengers, 200);

        std::fs::remove_file(db_path).unwrap();
    }

    #[test]
    fn test_read() {
        let db_path = "test_flightsDB.db";
        let conn = Connection::open(db_path).unwrap();
        conn.execute("CREATE TABLE IF NOT EXISTS data (year INTEGER, month TEXT, passengers INTEGER)", []).unwrap();
        conn.execute("INSERT INTO data (year, month, passengers) VALUES (2023, 'April', 250)", []).unwrap();

        // 데이터베이스에서 데이터를 읽습니다.
        let result = read(db_path);
        assert!(result.is_ok(), "Read function failed with {:?}", result);

        std::fs::remove_file(db_path).unwrap();
    }

    #[test]
    fn test_update() {
        let db_path = "test_flightsDB.db";
        let conn = Connection::open(db_path).unwrap();
        conn.execute("CREATE TABLE IF NOT EXISTS data (year INTEGER, month TEXT, passengers INTEGER)", []).unwrap();
        conn.execute("INSERT INTO data (year, month, passengers) VALUES (2023, 'May', 300)", []).unwrap();

        // 데이터베이스의 데이터를 수정합니다.
        let result = update(db_path, 2023, "May", 350);
        assert!(result.is_ok(), "Update function failed with {:?}", result);

        // 수정된 데이터를 검증합니다.
        let passengers: i32 = conn.query_row("SELECT passengers FROM data WHERE year = 2023 AND month = 'May'", params![], |row| row.get(0)).unwrap();
        assert_eq!(passengers, 350);

        std::fs::remove_file(db_path).unwrap();
    }

    #[test]
    fn test_delete() {
        let db_path = "test_flightsDB.db";
        let conn = Connection::open(db_path).unwrap();
        conn.execute("CREATE TABLE IF NOT EXISTS data (year INTEGER, month TEXT, passengers INTEGER)", []).unwrap();
        conn.execute("INSERT INTO data (year, month, passengers) VALUES (2023, 'June', 400)", []).unwrap();

        // 데이터베이스에서 데이터를 삭제합니다.
        let result = delete(db_path, 2023);
        assert!(result.is_ok(), "Delete function failed with {:?}", result);

        // 데이터가 정상적으로 삭제되었는지 검증합니다.
        let count: i32 = conn.query_row("SELECT COUNT(*) FROM data WHERE year = 2023", params![], |row| row.get(0)).unwrap();
        assert_eq!(count, 0);

        std::fs::remove_file(db_path).unwrap();
    }
}