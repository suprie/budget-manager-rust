# BCA PDF Parser Checkpoint

Date: 2026-05-16

## Current Goal

Build `bca-pdf` so it can parse BCA Rekening Tahapan PDF statements and eventually return the exact same JSON schema as `bca-csv`:

```json
[
  {
    "trx_date": "06/01",
    "description": "DB OTOMATIS Donation00000001 NONPROFIT ORG REF-X001-00050001",
    "amount": 120000.0,
    "trx_type": "DB"
  }
]
```

Longer term, both CSV and PDF parsers should sit behind a shared parser trait so the app can switch between parser implementations and later support other banks.

## Current Implementation

`bca-pdf/src/main.rs` is a raw spike, not final library code. It currently:

1. Reads a PDF file path from CLI args.
2. Uses `pdf_extract::extract_text` to convert the PDF into text.
3. Splits extracted text into lines.
4. Starts collecting only after the table header:
   `TANGGAL KETERANGAN CBG MUTASI SALDO`.
5. Skips empty lines, page marker noise, `Bersambung ke Halaman berikut`, and summary lines beginning with `SALDO` or `MUTASI`.
6. Builds multiline transaction blocks:
   - line starts with `DD/MM` shape -> new transaction block
   - otherwise -> append to previous transaction block

This handles cases where transaction descriptions continue onto later lines, such as:

```text
06/01 DB OTOMATIS 1234567890123 0305 120,000.00 DB 10,000,000.00
Donation00000001
NONPROFIT ORG
REF-X001-00050001
```

## Known Problems

- `extract_text(file_path).unwrap()` should eventually propagate errors instead of panicking.
- Date detection only checks `a.len() == 5 && a[2] == '/'`; it should verify `DD/MM` digits.
- Page marker handling uses `i += 3`, which is fragile and tied to current extracted output.
- Footer lines like `MUTASI CR`, `MUTASI DB`, and `SALDO AKHIR` are skipped, but later they should be collected for validation.
- The code is still in `main.rs`; reusable logic should move into testable functions before becoming a library API.
- There are current compiler warnings for unused imports/variables.

## Next Step

Do not parse JSON yet. First make transaction block extraction reliable and testable.

Suggested functions to extract:

- `is_table_header(line)`
- `is_page_marker_prefix(line)`
- `is_summary_line(line)`
- `starts_with_pdf_date(line)`
- `build_transaction_blocks(lines)`

After transaction blocks are reliable, feed those blocks to the AI parser. The AI should only convert cleaned transaction blocks into JSON; Rust should still handle PDF extraction, cleanup, JSON parsing, and validation against footer totals.
