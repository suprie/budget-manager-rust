# Repository Guidelines

## How to Work With Me

This is a Rust learning project. Do not write implementation code for the maintainer by default. Guide with questions, hints, docs, and crate references; review submitted code; challenge design choices before accepting them. Only show concrete code when the maintainer is stuck. Direct edits are acceptable for docs, tests, tooling, and explicitly requested changes.

### Challenge Me — Prevent Overengineering and Underengineering

The maintainer is learning Rust. Push back on both extremes:

**Overengineering flags — interrupt and ask "what problem does this solve right now?":**
- Adding a trait, generic, or abstraction before there are two concrete implementations that actually differ.
- Introducing a new crate dependency when the standard library or an existing dep already covers it.
- Designing for future banks/formats before BCA parsing is solid.
- Adding CLI flags, config files, or plugin systems before the happy path works end-to-end.
- "Clean architecture" layering (repository pattern, service layer, DTO mapping) for a single SQLite table.
- Deriving `Serialize` and `Deserialize` together when only one direction is used.

**Underengineering flags — interrupt and ask "will this survive real data?":**
- `.unwrap()` or `panic!` in library code instead of `Result` propagation.
- Magic numbers (fixed slice indices, hardcoded line counts) without a comment explaining why they're safe.
- Silent error swallowing — a mapped `Result` that isn't `?`-propagated or explicitly handled.
- No test for the unhappy path (malformed input, empty file, missing fields).
- Parsing that assumes fixed column positions or exact whitespace instead of using delimiters/regex.
- Zero logging or visibility into what the parser did with a file.

**How to challenge:** When you spot either pattern, ask a single pointed question rather than prescribing the fix. Example: "This skips exactly 3 lines after a page marker — what happens if the PDF extractor changes its output format?" Let the maintainer reach the answer.

**When to let it go:** If the maintainer acknowledges the tradeoff and chooses to ship anyway, accept it. This is a learning project. Not every `TODO` needs to be resolved today.

## Project Structure & Module Organization

This is a Rust Cargo workspace for parsing Indonesian BCA bank statements and managing budgets through a Tauri desktop app.

```
bca-csv/          # BCA CSV statement parser → returns JSON string
  src/lib.rs
  examples/main.rs
bca-pdf/          # BCA PDF statement parser → returns Vec<TransactionLine>
  src/lib.rs
  examples/main.rs
budget-manager/   # Tauri v2 desktop app (Vite + React frontend)
  src-tauri/
    src/
      main.rs          # Entry point
      lib.rs           # Tauri commands (read_statement_file, reset_db, list_transactions)
      db.rs            # SQLite connection, migration, reset
      transactions.rs  # Insert/read stored transactions
target/           # Generated build output (gitignored)
```

### Current State & Known Gaps

- `bca-csv` and `bca-pdf` return different types (JSON `String` vs. `Vec<TransactionLine>`). The plan is to extract a shared `TransactionLine` type into a common crate so both parsers return the same thing and the Tauri commands unify.
- `bca-pdf` parsing is functional but has a fragile `i += 3` page-marker skip and the main loop has no unit tests.
- `bca-csv` hardcodes trimming exactly 4 header and 4 footer lines — safe for the known BCA format but brittle.
- `reset_db` in `db.rs` and `insert_transactions` in `transactions.rs` both have a dropped-`Result` bug: `.map_err(...)` without `?` means errors are silently swallowed. Run `cargo clippy` — it should catch these.
- `println!` is used for logging throughout; no proper logging crate is wired up yet.
- The CSV path inserts into the frontend's state directly (returns JSON), while the PDF path persists to SQLite first — inconsistent behavior across the two file types.
- PDF footer validation (MUTASI CR, MUTASI DB, SALDO AKHIR) is noted in `PDF_PARSER_CHECKPOINT.md` but not yet implemented.

## Build, Test, and Development Commands

Run commands from the repository root unless noted otherwise.

- `cargo build` compiles the workspace.
- `cargo test` runs all workspace tests.
- `cargo run -p bca-csv --example main -- path/to/file.csv` runs the CSV parser.
- `cargo run -p bca-pdf --example main -- path/to/file.pdf` runs the PDF parser.
- `cargo test -p bca-csv` tests only the CSV parser.
- `cargo test -p bca-pdf` tests only the PDF parser.
- `cargo fmt --all` formats all Rust code with `rustfmt`.
- `cargo clippy --all-targets --all-features` runs lints across library, tests, and examples. Run this before committing — it catches dropped `Result` values and other common mistakes.
- `cargo build -p budget-manager` builds the Tauri app (requires system dependencies for Tauri on macOS/Linux/Windows).

## Coding Style & Naming Conventions

Use `rustfmt` and Rust 2024 edition where available. Prefer `snake_case` for functions and variables, `PascalCase` for structs and enums, and CSV-matching enum variants such as `CR` and `DB`.

Keep parsing logic small and testable. Use `once_cell::sync::Lazy` for compiled regexes rather than recompiling per call. Keep helpers private to their crate unless another crate needs them. Use `Box<dyn Error>` for library error returns; convert to `String` at the application boundary (Tauri commands).

## Testing Guidelines

Use Rust's built-in test framework. Add focused unit tests near the functions they cover in `#[cfg(test)]` modules. Name tests by behavior: `test_parse_valid_csv`, `test_parse_invalid_line`.

- Every parser path needs both a happy-path test and at least one malformed-input test.
- The PDF parser's main loop (`parse_pdf_file`) currently has zero tests — this is the highest-priority gap.
- `cargo test` must pass before submitting changes.
- Parser changes should cover: successful rows, malformed input, the leading `'` date artifact, `PEND` rows, multiline descriptions, and header/footer trimming.

## Commit & Pull Request Guidelines

The workspace root has minimal `.git` history. Use short, imperative subjects such as `Add CSV parse error test` or `Fix dropped Result in reset_db`.

Pull requests should include a concise description, rationale, and verification commands. Include sample input or output when parser behavior changes.

## Architecture Notes

**Shared schema goal:** Both parsers should eventually produce records with `trx_date`, `description`, `amount`, and `trx_type`. Branch and balance fields are read but not serialized (CSV) or not yet captured (PDF). PDF parser should eventually validate totals against `MUTASI CR`, `MUTASI DB`, and `SALDO AKHIR` footer lines.

**Database:** SQLite via `rusqlite`, migrated with `PRAGMA user_version`. The `transactions` table has a `UNIQUE (trx_date, description, amount, trx_type)` constraint and inserts use `INSERT OR IGNORE` for idempotent re-imports.

**Tauri commands** are defined in `budget-manager/src-tauri/src/lib.rs` and bridge the Rust backend to a Vite/React frontend.

## Security & Configuration Tips

Do not commit private bank statements, real account data, or generated JSON containing sensitive transactions. Use minimal synthetic CSV/PDF fixtures in tests and examples.
