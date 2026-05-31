# TODO

## ~~Parse period + account number from CSV/PDF~~ (partially done)

Previously `parse_csv_file()` and `parse_pdf_file()` returned `Vec<TransactionLine>`. Changed to return a struct that wraps the transactions with statement metadata.

### Done
- [x] `SummaryTransaction` added to `statement-core` with `account_number`, `period`, `transactions`
- [x] `TransactionLine.posted_date: Option<NaiveDate>` added
- [x] PDF parser: `parse_pdf_file_to_statement()` extracts account number and period year, populates `posted_date`
- [x] `chrono` dependency wired across workspace
- [x] DB migration adds `posted_date TEXT NULL` column
- [x] `rusqlite` updated with `chrono` feature for `NaiveDate` ↔ SQLite binding
- [x] `StoredTransaction.posted_date: Option<NaiveDate>` synced with DB

### Remaining
- [ ] **CSV parser** — extract period + account number from header lines (currently `println!`-ed then skipped). Populate `posted_date` instead of hardcoding `None`.
- [ ] **Drop `period: String`** from `SummaryTransaction` — can be inferred from `MIN(posted_date)` / `MAX(posted_date)`. Decided to drop, not yet removed.

## New TODO

### Fix App.vue bugs
- [ ] **Line 12**: `posted_date: date` → `posted_date: string | null` (`date` is not a valid TypeScript type; `NaiveDate` serializes to string from Rust)
- [ ] **Line 31**: `key: "trx_type"` is duplicate of line 30. Change to `key: "posted_date"`

### bca-pdf cleanup
- [ ] **Old `parse_pdf_file()` / `parse_to_transaction_line()`** — dead code now, only called from their own tests. Remove after confirming `parse_pdf_file_to_statement` is stable.
- [ ] **Silent data loss in `parse_to_transaction_line_date`** — if `year` is empty string (period not parsed yet), `"22/01/"` parse fails silently and `posted_date` becomes `None`. Should at minimum `eprintln!` a warning.
- [ ] **Missing test** for `parse_to_transaction_line_date` — happy path with year, and PEND → `posted_date: None`.

### Cleanup
- [ ] **Debug `println!`** — remove from `lib.rs:26` and `transactions.rs:27,39` once feature is stable. Replace with a proper logging crate (`log` + `env_logger` or `tracing`).
