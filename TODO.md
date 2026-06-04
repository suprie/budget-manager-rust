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
- [x] Categories CRUD: `get_all_categories`, `create_category(name, color)`, `get_uncategorized_transactions(keyword)`, `bulk_assign_category(ids, category_id)`
- [x] CategoriesPage UI: add-category form with preset + custom color picker, bulk-assign table with keyword search
- [x] Dashboard inline category reassign: click tag → popover → pick new category
- [x] Query limits: `load_transactions` with `LIMIT/OFFSET` + `total_count` for server-side pagination; `get_uncategorized_transactions` capped at 200 rows

## Remaining

### bca-pdf

- [ ] **Silent data loss** in `parse_to_transaction_line_date` — empty `year` → parse fails silently, `posted_date` becomes `None`. Add `eprintln!` warning.
- [ ] **Missing test** for `parse_to_transaction_line_date` — happy path with year, PEND → `None`

### CSV parser

- [ ] **Extract account number** from header lines (currently `println!`-ed then skipped)

### Categories — pagination polish

- [ ] `get_uncategorized_transactions` — return `total_count` so the bulk-assign table can show full pagination instead of a hard 200-row cap

### Cleanup

- [ ] **Drop `period: String`** from `SummaryTransaction` — can be inferred from `MIN(posted_date)` / `MAX(posted_date)`
- [ ] **Replace `println!`** with `log` + `env_logger` or `tracing`
