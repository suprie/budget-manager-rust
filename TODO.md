# TODO

## Done

- [x] `SummaryTransaction` with `account_number`, `period`, `transactions`
- [x] `TransactionLine.posted_date: Option<NaiveDate>`
- [x] PDF parser extracts account number + period year, populates `posted_date`
- [x] `chrono` and `rusqlite` wired across workspace
- [x] DB: `posted_date TEXT NULL`, `line INTEGER`, `source TEXT NOT NULL`
- [x] Idempotency: `UNIQUE (line, trx_date, description, amount, trx_type)`
- [x] Dead code removed: `parse_pdf_file()`, `parse_to_transaction_line()`, `read_all_transactions()`, `list_transactions`
- [x] App.vue bugs: `posted_date: string | null`, duplicate key fixed
- [x] Debug `println!` cleanup
- [x] Migration refactored to `match` + loop pattern
- [x] Categories scaffold: `categories` table, `category_id` FK, JOIN on load, `<a-tag>` in frontend

## Remaining

### Categories — CRUD + editing

- [ ] **Rust commands**: `list_categories`, `create_category`, `update_transaction_category` (single + bulk)
- [ ] **Frontend inline edit**: click category tag → dropdown picker → save via Tauri command

### bca-pdf

- [ ] **Silent data loss** in `parse_to_transaction_line_date` — empty `year` → parse fails silently, `posted_date` becomes `None`. Add `eprintln!` warning.
- [ ] **Missing test** for `parse_to_transaction_line_date` — happy path with year, PEND → `None`

### CSV parser

- [ ] **Extract account number** from header lines (currently `println!`-ed then skipped)

### Cleanup

- [ ] **Drop `period: String`** from `SummaryTransaction` — can be inferred from `MIN(posted_date)` / `MAX(posted_date)`
- [ ] **Replace `println!`** with `log` + `env_logger` or `tracing`
