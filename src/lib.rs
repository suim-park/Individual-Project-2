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
    let path = Path::new("flights.csv");
    let file = File::open(&path)?;

    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let flight: Flight = result?;
        println!("{:?}", flight);
    }

    Ok(())
}