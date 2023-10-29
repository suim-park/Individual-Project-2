use rust_cli_binary::extract;
use rust_cli_binary::transform;
use rust_cli_binary::create;
use rust_cli_binary::read;
use rust_cli_binary::update;
use rust_cli_binary::delete;
use std::fs;

fn main() {
    // 데이터베이스 파일 삭제
    let _ = fs::remove_file("flightsDB.db");
    
    extract("https://github.com/suim-park/Individual-Project-2/raw/main/rust-cli-binary/flights.csv", "flights.csv").unwrap();
    
    let csv_path = "flights.csv";
    let db_path = "flightsDB.db";
    match transform(csv_path, db_path) {
        Ok(_) => println!("CSV file has been successfully converted to SQLite DB."),
        Err(e) => println!("An error occurred during conversion: {}", e),
    }

    match create("flightsDB.db", 2023, "October", 800) {
        Ok(_) => println!("Successfully inserted data into the SQLite DB."),
        Err(e) => println!("Error occurred while inserting data: {}", e),
    }
   
    match read("flightsDB.db") {
        Ok(_) => println!("Successfully read from the SQLite DB."),
        Err(e) => println!("Error occurred while reading data: {}", e),
    }
  
    match update("flightsDB.db", 2023, "October", 1000) {
        Ok(_) => println!("Successfully updated data in the SQLite DB."),
        Err(e) => println!("Error occurred while updating data: {}", e),
    }
    
    match read("flightsDB.db") {
        Ok(_) => println!("Successfully read from the SQLite DB."),
        Err(e) => println!("Error occurred while reading data: {}", e),
    }

    let year_to_delete = 2023;
    match delete(db_path, year_to_delete) {
        Ok(_) => println!("Successfully deleted data for year {}.", year_to_delete),
        Err(e) => println!("An error occurred: {}", e),
    }

    match read("flightsDB.db") {
        Ok(_) => println!("Successfully read from the SQLite DB."),
        Err(e) => println!("Error occurred while reading data: {}", e),
    }
}