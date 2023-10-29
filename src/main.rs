use rust_cli_binary::extract;
use rust_cli_binary::transform;

fn main() {
    extract("https://github.com/suim-park/Individual-Project-2/raw/main/flights.csv", "flights.csv").unwrap();
    
    let csv_path = "flights.csv";
    let db_path = "flightsDB.db";
    match transform(csv_path, db_path) {
        Ok(_) => println!("CSV file has been successfully converted to SQLite DB."),
        Err(e) => println!("An error occurred during conversion: {}", e),
    }
}