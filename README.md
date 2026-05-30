# Budget Manager

A desktop app for parsing **Indonesian BCA bank statements** (CSV & PDF) and managing personal finances. Built with Rust + Tauri + Vue + Ant Design Vue.

## How It Works

1. **Pick a BCA statement file** (CSV from internet banking, or PDF from e-statement)
2. The app **parses** the file and stores transactions in a local SQLite database
3. You see your transactions in a table with **income vs. expense totals**
4. Data is fully **offline** — nothing leaves your machine

## Project Structure

```
bca-csv/              BCA CSV statement parser
bca-pdf/              BCA PDF statement parser
statement-core/       Shared types (TransactionLine, TrxType)
budget-manager/       Tauri v2 desktop app (Vite + Vue 3 + Ant Design Vue)
```

## Prerequisites

- [Rust](https://rustup.rs) (stable)
- [Bun](https://bun.sh) (or Node.js + npm)
- Tauri system dependencies: [see Tauri docs](https://v2.tauri.app/start/prerequisites/)

## Quick Start

```sh
# Install frontend dependencies
cd budget-manager
bun install

# Run the desktop app (dev mode)
bun run tauri dev
```

## Commands

| Command | What it does |
|---------|--------------|
| `cargo build` | Compile the workspace |
| `cargo test` | Run all tests |
| `cargo clippy --all-targets --all-features` | Lint everything |
| `cargo fmt --all` | Format all Rust code |
| `cargo run -p bca-csv --example main -- file.csv` | Parse a CSV from CLI |
| `cargo run -p bca-pdf --example main -- file.pdf` | Parse a PDF from CLI |

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop shell | Tauri v2 |
| Frontend | Vue 3 + Vite + Ant Design Vue |
| Database | SQLite via `rusqlite` |
| PDF parsing | `pdf-extract` |
| CSV parsing | `csv` crate |

## Status

This is a **learning project**. The parsers handle real BCA statements but edge cases remain. See `TODO.md` for next steps.

Bugs or ideas? Open an issue.
