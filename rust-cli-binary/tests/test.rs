#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use rusqlite::params;
    use rusqlite::Connection;
    use rust_cli_binary::{create, delete, extract, read, transform, update};
    use std::fs;
    use std::sync::Once;

    lazy_static! {
        static ref INIT: Once = Once::new();
    }

    #[test]
    fn test_extract() {
        // Define test URL and save path.
        let test_url = "https://github.com/nogibjj/IDS706-Individual-Project-2-sp699/raw/main/rust-cli-binary/test_flights.csv"; // This URL is an example. Please replace with an actual accessible URL.
        let test_path = "test_flights.csv";

        // Execute the extract function.
        let result = extract(test_url, test_path);

        // Check if the result is Ok(()).
        assert!(result.is_ok(), "Extract function failed with {:?}", result);

        // Check if the file was actually created.
        assert!(
            fs::metadata(test_path).is_ok(),
            "Failed to create the file at {}",
            test_path
        );
    }

    #[test]
    fn test_transform() {
        // Create sample CSV data.
        let csv_path = "test_flights.csv";
        let db_path = "test_flightsDB.db";

        // Execute the transform function.
        let result = transform(csv_path, db_path);

        // Check if the result is Ok(()).
        assert!(
            result.is_ok(),
            "Transform function failed with {:?}",
            result
        );
    }

    #[test]
    fn test_create() {
        let db_path = "test_flightsDB.db";

        // Create data in the database.
        let result = create(db_path, 2023, "March", 200);
        assert!(result.is_ok(), "Create function failed with {:?}", result);

        // Verify the created data.
        let conn = Connection::open(db_path).unwrap();
        let passengers: i32 = conn
            .query_row(
                "SELECT passengers FROM data WHERE year = 2023 AND month = 'March'",
                params![],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(passengers, 200);
    }

    #[test]
    fn test_read() {
        let db_path = "test_flightsDB.db";

        let conn = Connection::open(db_path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (year INTEGER, month TEXT, passengers INTEGER)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO data (year, month, passengers) VALUES (2023, 'April', 250)",
            [],
        )
        .unwrap();

        // Read data from the database.
        let result = read(db_path);
        assert!(result.is_ok(), "Read function failed with {:?}", result);
    }

    #[test]
    fn test_update() {
        let db_path = "test_flightsDB.db";
        let conn = Connection::open(db_path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (year INTEGER, month TEXT, passengers INTEGER)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO data (year, month, passengers) VALUES (2023, 'May', 300)",
            [],
        )
        .unwrap();

        // Update data in the database.
        let result = update(db_path, 2023, "May", 350);
        assert!(result.is_ok(), "Update function failed with {:?}", result);

        // Verify the updated data.
        let passengers: i32 = conn
            .query_row(
                "SELECT passengers FROM data WHERE year = 2023 AND month = 'May'",
                params![],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(passengers, 350);
    }

    #[test]
    fn test_delete() {
        let db_path = "test_flightsDB.db";
        let conn = Connection::open(db_path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (year INTEGER, month TEXT, passengers INTEGER)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO data (year, month, passengers) VALUES (2023, 'June', 400)",
            [],
        )
        .unwrap();

        // Delete data from the database.
        let result = delete(db_path, 2023);
        assert!(result.is_ok(), "Delete function failed with {:?}", result);

        // Verify the data was properly deleted.
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM data WHERE year = 2023",
                params![],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);
    }
}
