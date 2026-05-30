# TODO

## Parse period + account number from CSV/PDF

Currently `parse_csv_file()` and `parse_pdf_file()` return `Vec<TransactionLine>`. Change them to return a struct that wraps the transactions with statement metadata.

### Steps

1. **Add `StatementSummary` to `statement-core`:**
   ```rust
   pub struct StatementSummary {
       pub account_number: String,
       pub period: String,
       pub transactions: Vec<TransactionLine>,
   }
   ```

2. **Update `bca-csv`** — extract account number and period from the header lines (currently just `println!`-ed and skipped) and footer lines. Return `StatementSummary` instead of `Vec<TransactionLine>`.

3. **Update `bca-pdf`** — same idea, but from PDF-specific header/footer. The summary lines (`SALDO`, `MUTASI`) that are currently skipped are the right place to look.

4. **Update `transactions.rs` / `lib.rs`** in the Tauri app so it works with the new return type.
