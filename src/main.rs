use rust_cli_binary::extract;

fn main() {
    extract("https://github.com/suim-park/Individual-Project-2/raw/main/flights.csv", "flights.csv").unwrap();
}
