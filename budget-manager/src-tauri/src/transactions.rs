use statement_core::{Source, TransactionLine, TrxType};
use rusqlite::{params, Connection};
use chrono::NaiveDate;

#[derive(Debug, serde::Serialize)]
pub struct StoredTransaction {
    line: usize,
    trx_date: String,
    description: String,
    amount: f64,
    trx_type: String,
    posted_date: Option<NaiveDate>,
    source: String
}

#[derive(Debug, serde::Serialize)]
pub struct TransactionSummary {
    total_income: f64,
    total_expenses: f64,
    transactions: Vec<StoredTransaction>
}

pub fn insert_transactions(
    conn: &mut Connection,
    transactions: &[TransactionLine],
) -> Result<usize, String> {
    let tx = conn.transaction().map_err(|error| error.to_string())?;
    {
        println!("LINE : {:?}", transactions);
        let mut stmt = tx
            .prepare(
                "
                INSERT OR IGNORE INTO transactions (line, trx_date, description, amount, trx_type, posted_date, source)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ",
            )
            .map_err(|error| error.to_string())?;

        for transaction in transactions {
            stmt.execute(params![
                transaction.line,
                transaction.trx_date,
                transaction.description,
                transaction.amount,
                trx_type_as_str(&transaction.trx_type),
                transaction.posted_date,
                source_as_str(&transaction.source)
            ])
            .map_err(|error| error.to_string())?;
        }
    }
    tx.commit().map_err(|error| error.to_string())?;
    Ok(transactions.len())
} 

pub fn load_data(conn: &mut Connection) -> Result<TransactionSummary, String> {
    let mut stmt = conn.prepare("SELECT line, trx_date, description, amount, trx_type, posted_date, source from transactions").map_err(|error| error.to_string())?;
    let iter = stmt.query_map([], |row| {
        Ok(StoredTransaction {
            line: row.get(0)?,
            trx_date: row.get(1)?,
            description: row.get(2)?,
            amount: row.get(3)?,
            trx_type: row.get(4)?,
            posted_date: row.get(5)?,
            source: row.get(6)?
        })
    }).map_err(|error| error.to_string())?;

    let mut transactions = Vec::new();
    let mut total_income = 0.0;
    let mut total_expenses = 0.0;
    for row in iter {
        let temp_row = row.map_err(|error| error.to_string())?;
        println!("row {:?}", temp_row);

        if temp_row.trx_type == "CR" {
            total_income += temp_row.amount
        } else {
            total_expenses += temp_row.amount
        }
        transactions.push(temp_row);
    }

    Ok(TransactionSummary {
        total_income,
        total_expenses,
        transactions
    })

}

pub fn read_all_transactions(conn: &mut Connection) -> Result<Vec<StoredTransaction>, String> {
    let mut stmt = conn.prepare("SELECT line, trx_date, description, amount, trx_type, posted_date, source from transactions").map_err(|error| error.to_string())?;
    let iter = stmt.query_map([], |row| {
        Ok(StoredTransaction {
            line: row.get(0)?,
            trx_date: row.get(1)?,
            description: row.get(2)?,
            amount: row.get(3)?,
            trx_type: row.get(4)?,
            posted_date: row.get(5)?,
            source: row.get(6)?
        })
    }).map_err(|error| error.to_string())?;

    let mut transactions = Vec::new();

    for row in iter {
        transactions.push(row.map_err(|error| error.to_string())?);
    }

    Ok(transactions)

}

fn trx_type_as_str(trx_type: &TrxType) -> &'static str {
      match trx_type {
          TrxType::CR => "CR",
          TrxType::DB => "DB",
      }
  }

fn source_as_str(trx_type: &Source) -> &'static str {
      match trx_type {
          Source::PDF => "PDF",
          Source::CSV => "CSV"
      }
}
