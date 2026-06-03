use bca_csv::parse_csv_file;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: cargo run -p bca-csv --example main -- <file.csv> [year]".to_string().into())
    }

    let file_path = &args[1];
    let year: i32 = args.get(2).and_then(|y| y.parse().ok()).unwrap_or(2026);

    let result = parse_csv_file(&file_path, year)?;
    println!("{:?}", result);
    Ok(())
}
