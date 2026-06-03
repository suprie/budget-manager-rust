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
- [ ] **CSV parser** — extract account number from header lines (currently `println!`-ed then skipped). Note: BCA CSV has no period info in the header (only account number, name, currency), so `posted_date` will remain `None` for CSV imports unless the year is provided via parameter.
- [ ] **Drop `period: String`** from `SummaryTransaction` — can be inferred from `MIN(posted_date)` / `MAX(posted_date)`. Decided to drop, not yet removed.

## New TODO

### Fix App.vue bugs
- [ ] **Line 12**: `posted_date: date` → `posted_date: string | null` (`date` is not a valid TypeScript type; `NaiveDate` serializes to string from Rust)
- [ ] **Line 31**: `key: "trx_type"` is duplicate of line 30. Change to `key: "posted_date"`

### bca-pdf cleanup
- [ ] **Old `parse_pdf_file()` / `parse_to_transaction_line()`** — dead code now, only called from their own tests. Remove after confirming `parse_pdf_file_to_statement` is stable.
- [ ] **Silent data loss in `parse_to_transaction_line_date`** — if `year` is empty string (period not parsed yet), `"22/01/"` parse fails silently and `posted_date` becomes `None`. Should at minimum `eprintln!` a warning.
- [ ] **Missing test** for `parse_to_transaction_line_date` — happy path with year, and PEND → `posted_date: None`.

### Separate CSV vs PDF upload flow

Currently `read_statement_file` handles both file types in one Tauri command with a single file picker. This breaks for CSV because:

- **PDF**: period is extractable from the file (`PERIODE : JANUARI 2026`)
- **CSV**: no period info in the file — needs a separate period input from the user (month/year picker)

**Plan:**
- [ ] **Frontend** — split upload UI: "Pick PDF" (single file) vs "Pick CSV" (file + month/year form)
- [ ] **Tauri commands** — either add a new `read_statement_file_csv(filepath, period)` command, or change `read_statement_file` to accept an optional `period` argument
- [ ] **CSV parser** — accept `year: i32` parameter so it can populate `posted_date`

### Cleanup
- [ ] **Debug `println!`** — remove from `lib.rs:26` and `transactions.rs:27,39` once feature is stable. Replace with a proper logging crate (`log` + `env_logger` or `tracing`).
