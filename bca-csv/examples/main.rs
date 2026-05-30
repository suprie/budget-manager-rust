use bca_csv::parse_csv_file;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Need to pass the csv file".to_string().into())
    }

    let file_path = &args[1];

    let result = parse_csv_file(&file_path)?;
    println!("{:?}", result);
    Ok(())
}
