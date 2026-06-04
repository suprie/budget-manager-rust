use statement_core::{Source, TransactionLine, TrxType};
use rusqlite::{params, Connection};
use chrono::NaiveDate;
use crate::categories::Category;

#[derive(Debug, serde::Serialize)]
pub struct StoredTransaction {
    id: i64,
    line: usize,
    trx_date: String,
    description: String,
    amount: f64,
    trx_type: String,
    posted_date: Option<NaiveDate>,
    source: String,
    category: Category
}

#[derive(Debug, serde::Serialize)]
pub struct TransactionSummary {
    total_income: f64,
    total_expenses: f64,
    total_count: i64,
    transactions: Vec<StoredTransaction>
}

pub fn insert_transactions(
    conn: &mut Connection,
    transactions: &[TransactionLine],
) -> Result<usize, String> {
    let tx = conn.transaction().map_err(|error| error.to_string())?;
    {
        let mut update_pend = tx
            .prepare(
                "
                UPDATE transactions
                SET trx_date = ?1, posted_date = ?2
                WHERE line = ?3 AND source = ?4 AND description = ?5
                  AND amount = ?6 AND trx_type = ?7 AND trx_date = 'PEND'
                ",
            )
            .map_err(|error| error.to_string())?;

        let mut insert = tx
            .prepare(
                "
                INSERT OR IGNORE INTO transactions (line, trx_date, description, amount, trx_type, posted_date, source)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ",
            )
            .map_err(|error| error.to_string())?;

        for transaction in transactions {
            let trx_type = trx_type_as_str(&transaction.trx_type);
            let source = source_as_str(&transaction.source);

            if transaction.trx_date != "PEND" {
                update_pend.execute(params![
                    transaction.trx_date,
                    transaction.posted_date,
                    transaction.line,
                    source,
                    transaction.description,
                    transaction.amount,
                    trx_type,
                ])
                .map_err(|error| error.to_string())?;
            }

            insert.execute(params![
                transaction.line,
                transaction.trx_date,
                transaction.description,
                transaction.amount,
                trx_type,
                transaction.posted_date,
                source,
            ])
            .map_err(|error| error.to_string())?;
        }
    }
    tx.commit().map_err(|error| error.to_string())?;
    Ok(transactions.len())
}

pub fn load_data(
    conn: &mut Connection,
    limit: i64,
    offset: i64,
) -> Result<TransactionSummary, String> {
    // Aggregate totals from all rows (not limited).
    let total_income: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(amount), 0) FROM transactions WHERE trx_type = 'CR'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let total_expenses: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(amount), 0) FROM transactions WHERE trx_type = 'DB'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    // Total row count for pagination.
    let total_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM transactions", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    // Paged transaction rows.
    let mut stmt = conn
        .prepare(
            "SELECT t.id, t.line, t.trx_date, t.description, t.amount, t.trx_type,
                    t.posted_date, t.source,
                    c.id AS category_id, c.category_name AS category_name, c.color AS category_color
             FROM transactions t LEFT JOIN categories c ON t.category_id = c.id
             ORDER BY t.trx_date DESC, t.id DESC
             LIMIT ?1 OFFSET ?2",
        )
        .map_err(|error| error.to_string())?;

    let iter = stmt
        .query_map(params![limit, offset], |row| {
            Ok(StoredTransaction {
                id: row.get("id")?,
                line: row.get("line")?,
                trx_date: row.get("trx_date")?,
                description: row.get("description")?,
                amount: row.get("amount")?,
                trx_type: row.get("trx_type")?,
                posted_date: row.get("posted_date")?,
                source: row.get("source")?,
                category: Category {
                    id: row.get("category_id")?,
                    category_name: row.get("category_name")?,
                    color: row.get("category_color")?,
                },
            })
        })
        .map_err(|error| error.to_string())?;

    let transactions: Vec<StoredTransaction> = iter
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;

    Ok(TransactionSummary {
        total_income,
        total_expenses,
        total_count,
        transactions,
    })
}

fn trx_type_as_str(trx_type: &TrxType) -> &'static str {
      match trx_type {
          TrxType::CR => "CR",
          TrxType::DB => "DB",
      }
  }

fn source_as_str(source: &Source) -> &'static str {
      match source {
          Source::PDF => "PDF",
          Source::CSV => "CSV"
      }
}
