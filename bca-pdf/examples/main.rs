use std::error::Error;
use std::env;
use bca_pdf::parse_pdf_file_to_statement;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Need to pass the pdf file".to_string().into())
    }

    let file_path = &args[1];
    let transaction_line = parse_pdf_file_to_statement(&file_path).unwrap(); 
    println!("{:?}", transaction_line.account_number);
    println!("{:?}", transaction_line.period);

    for i in transaction_line.transactions.iter() {
        println!("{:?}",i);
    } 
    Ok(())
}

