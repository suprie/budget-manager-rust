use statement_core::{TransactionLine, TrxType};
use rusqlite::{params, Connection};

#[derive(Debug, serde::Serialize)]
pub struct StoredTransaction {
    trx_date: String,
    description: String,
    amount: f64,
    trx_type: String
}

pub fn insert_transactions(
    conn: &mut Connection,
    transactions: &[TransactionLine],
) -> Result<usize, String> {
    let tx = conn.transaction().map_err(|error| error.to_string())?;

    {
        let mut stmt = tx
            .prepare(
                "
                INSERT OR IGNORE INTO transactions (trx_date, description, amount, trx_type)
                VALUES (?1, ?2, ?3, ?4)
            ",
            )
            .map_err(|error| error.to_string())?;

        for transaction in transactions {
            stmt.execute(params![
                transaction.trx_date,
                transaction.description,
                transaction.amount,
                trx_type_as_str(&transaction.trx_type)
            ])
            .map_err(|error| error.to_string())?;
        }
    }
    tx.commit().map_err(|error| error.to_string())?;
    Ok(transactions.len())
}

pub fn read_all_transactions(conn: &mut Connection) -> Result<Vec<StoredTransaction>, String> {
    let mut stmt = conn.prepare("SELECT trx_date, description, amount, trx_type from transactions").map_err(|error| error.to_string())?;
    let iter = stmt.query_map([], |row| {
        Ok(StoredTransaction {
            trx_date: row.get(0)?,
            description: row.get(1)?,
            amount: row.get(2)?,
            trx_type: row.get(3)?,
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
