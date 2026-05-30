use once_cell::sync::Lazy;
use pdf_extract::extract_text;
use regex::Regex;
use std::error::Error;
use statement_core::{ TransactionLine, TrxType };

const REG_STRING: &str = r"^(?<Date>\d{2}\/\d{2}|PEND)\s+(?<Description>.+?)[ \t]+(?<Amount>[\d,.]+)(?:\s+(?<Type>CR|DB))?(?: +(?<Balance>[\d,.]+))?$";
static TRANSACTION_RE: Lazy<Regex> = Lazy::new(|| Regex::new(REG_STRING).unwrap());

pub fn parse_pdf_file(file_path: &str) -> Result<Vec<TransactionLine>, Box<dyn Error>> {
    let mut should_pick: bool = false;

    let mut i = 0;

    let content = extract_text(file_path)?;
    let lines: Vec<&str> = content.lines().collect();
    let mut transaction_lines: Vec<TransactionLine> = Vec::new();

    while i < lines.len() {
        let tmp_line = lines[i].trim();
        println!("line {:} : {:}", i, tmp_line);
        if is_table_header(&tmp_line) {
            should_pick = true;
            i += 1;
            continue;
        }

        if is_page_marker_prefix(&tmp_line) {
            i += 1;
            continue;
        }

        if is_empty_line(&tmp_line) {
            i += 1;
            continue;
        }

        if tmp_line.ends_with("/") {
            let tmp2 = tmp_line.replace("/", "");
            if tmp2.trim().chars().all(|c| c.is_ascii_digit()) {
                i += 3;
                should_pick = false;
                continue;
            }
        }

        if is_summary(&tmp_line) {
            i += 1;
            continue;
        }

        if should_pick {
            let s = tmp_line;

            let line = match parse_to_transaction_line(s) {
                Ok(line) => line,
                Err(_) => {
                    if let Some(last_line) = transaction_lines.last_mut() {
                        last_line.description.push_str(" ");
                        last_line.description.push_str(s);
                    }
                    i += 1;
                    continue;
                }
            };

            transaction_lines.push(line);
        }
        i += 1;
    }

    Ok(transaction_lines)
}

fn parse_to_transaction_line(line: &str) -> Result<TransactionLine, Box<dyn Error>> {
    let Some(caps) = TRANSACTION_RE.captures(line) else {
        return Err("Unable to parse to transaction_line".into());
    };
    let trx_type = match caps.name("Type").map(|m| m.as_str()) {
        Some("DB") => TrxType::DB,
        _ => TrxType::CR,
    };

    let amount = caps["Amount"].replace(",", "").trim().parse::<f64>()?;

    let transaction_line = TransactionLine {
        trx_date: caps["Date"].to_string(),
        description: caps["Description"].to_string(),
        amount,
        trx_type,
    };
    Ok(transaction_line)
}

fn is_table_header(line: &str) -> bool {
    line == "TANGGAL KETERANGAN CBG MUTASI SALDO"
}

fn is_page_marker_prefix(line: &str) -> bool {
    line == "Bersambung ke Halaman berikut"
}

fn is_empty_line(line: &str) -> bool {
    line.is_empty()
}

fn is_summary(line: &str) -> bool {
    line.starts_with("SALDO") || line.starts_with("MUTASI")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "22/01 KARTU DEBIT SPBU 34.12114,GAND 100,000.00 DB 7,269,919.48";
        let result = parse_to_transaction_line(&line).unwrap();

        assert!(result.trx_date == "22/01");
        assert!(result.description == "KARTU DEBIT SPBU 34.12114,GAND");
        assert!(result.amount == 100000.00);
        assert!(result.trx_type == TrxType::DB);
    }

    #[test]
    fn test_parse_invalid_line() {
        let line = "1231241412";
        let result = parse_to_transaction_line(&line);
        assert!(result.is_err());
    }
}
