use std::fs::read_to_string;
use csv::{Reader, StringRecord};

use std::error::Error;
use serde::{Serialize, Deserialize};
use serde_json;

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

pub fn parse_csv_file(file_path: &str) -> Result<String, Box<dyn Error>> {

    let v:Vec<String> = read_lines(file_path)?;

    let a = &v[4 .. v.len() - 4];
    let joined = a.join("\n");

    parse_csv(&joined)

}

fn parse_csv(csv_content: &str) -> Result<String, Box<dyn Error>> {
    let mut rdr = Reader::from_reader(csv_content.as_bytes());

    let mut records:Vec<TrxRecord> = Vec::new();

    for result in rdr.records() {
        let record: StringRecord = result?;
        let trx_record: TrxRecord = record.deserialize(None)?;
        records.push(trx_record);
    }


    let json = serde_json::to_string(&records)?;

    Ok(json)
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
        let expected_outcome = "[{\"trx_date\":\"'19/09\",\"description\":\"TRSF E-BANKING\",\"amount\":4700.0,\"trx_type\":\"DB\"}]";
        let result = parse_csv(&csv).unwrap();
        assert!(result == expected_outcome);
    
    }

    #[test]
    fn test_parse_invalid_csv() {
        let csv = "data, , , , , \n'19/09,TRSF E-BANKING,4700.00,DB,48219.21";
        let result = parse_csv(&csv);
        assert!(result.is_err());
    
    }
}
