use std::fs::read_to_string;
use csv::{Reader, StringRecord};

use std::error::Error;
use serde::{Serialize, Deserialize};
use statement_core:: { TransactionLine, TrxType as StatementCoreTrxType };

#[derive(Debug, Deserialize, Serialize)]
enum TrxType {
    CR,
    DB
}

#[derive(Debug, Deserialize, Serialize)]
struct TrxRecord {
    trx_date: String,
    description: String,
    #[serde(skip_serializing)]
    _branch: String,
    amount: f64, 
    trx_type: TrxType,
    #[serde(skip_serializing)]
    _balance: String
}

pub fn parse_csv_file(file_path: &str) -> Result<Vec<TransactionLine>, Box<dyn Error>> {

    let v:Vec<String> = read_lines(file_path)?;
    let header = &v[0 .. 4];
    println!("HEADER {:?}", header);
    let a = &v[4 .. v.len() - 4];
    let joined = a.join("\n");

    parse_csv(&joined)

}

fn parse_csv(csv_content: &str) -> Result<Vec<TransactionLine>, Box<dyn Error>> {
    let mut rdr = Reader::from_reader(csv_content.as_bytes());

    let mut records:Vec<TransactionLine> = Vec::new();

    for result in rdr.records() {
        let record: StringRecord = result?;
        let trx_record: TrxRecord = record.deserialize(None)?;
        let transaction_line = TransactionLine {
            trx_date: trx_record.trx_date,
            description: trx_record.description,
            amount: trx_record.amount,
            trx_type: trx_type(trx_record.trx_type)
        };

        records.push(transaction_line);
    }


    Ok(records)
}

fn trx_type(trx_type: TrxType) -> StatementCoreTrxType {
    match trx_type {
       TrxType::CR => StatementCoreTrxType::CR,
       TrxType::DB => StatementCoreTrxType::DB
    }
}


fn read_lines(filename: &str)  -> Result<Vec<String>, Box<dyn Error>> {
    let mut result = Vec::new();

    for line in read_to_string(filename)?.lines() {
        result.push(line.to_string());
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_csv() {
        let csv = "data, , , , , \n'19/09,TRSF E-BANKING,'0000,4700.00,DB,48219.21";
        let result = parse_csv(&csv).unwrap();
        assert!(result[0].description  ==  "TRSF E-BANKING");
    
    }

    #[test]
    fn test_parse_invalid_csv() {
        let csv = "data, , , , , \n'19/09,TRSF E-BANKING,4700.00,DB,48219.21";
        let result = parse_csv(&csv);
        assert!(result.is_err());
    
    }
}
