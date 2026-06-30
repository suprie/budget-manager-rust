# PRD: Debt Tracking

| Field    | Value                                   |
|----------|-----------------------------------------|
| Title    | Debt Tracking (KPR, credit card, pinjol, other) |
| Status   | Draft                                   |
| Date     | 2026-06-30                              |
| Owner    | Suprie                                  |
| Related  | `docs/debt-phases-and-decisions.md`     |

> This feature is **100% offline and rule-based. No AI.** Every "strategy" and
> "recommendation" is computed from explicit formulas the user can audit.

---

## 1. Problem & Overview

The app already parses BCA statements (CSV + PDF) and categorizes transactions.
What it cannot do today is answer: *"How much do I owe in total, what is costing
me the most, and where should my spare rupiah go this month?"*

This feature adds **debt tracking** for the common Indonesian consumer debts:

- **KPR / mortgage** — large, fixed monthly installment, changes rarely.
- **Credit card** — revolving balance, flexible payments, usually a monthly fee.
- **Pinjol** (pinjaman online) — short term, frequently quoted as a **daily** fee.
- **Other** — motor kredit, KKB, personal loan, KTA, arisan debt, IOU to family, etc.

A debt may be **paid down two ways**:

1. **Statement-derived** — a transaction on an imported BCA statement is
   categorized under the debt's category, so it counts as a payment.
2. **Manual** — the user types it in (cash payment, transfer from another bank,
   settlement, etc.).

This mirrors how debts behave in real life: most payments show up on the
statement, but not all of them.

## 2. Goals

- Let the user record every debt they owe, with the terms that matter for payoff
  decisions (principal, current balance, fee/interest, schedule type).
- Detect payments automatically from categorized statement transactions, while
  still allowing manual entry.
- Recommend a payoff order using a transparent, auditable rule (avalanche with a
  fixed-payment carve-out).
- Show progress per debt and for the whole portfolio, offline, in the same app
  that already holds the user's transactions.

## 3. Non-Goals

Explicitly **out of scope** for this PRD:

- **No AI** of any kind. No ML, no "smart suggestions," no natural-language
  parsing of debts. Everything is a documented formula.
- **No auto-generated amortization schedule.** Fixed-installment debts (KPR) are
  modeled by their fixed payment, not a computed schedule. See decision D6.
- **No reminders / push notifications / due-date alerts.** The desktop app is
  passive; the user opens it when they want to look.
- **No multi-currency.** All amounts are IDR, reusing `formatAmount()`.
- **No credit-limit / utilization tracking** for credit cards.
- **No lender/bank account integration** beyond reading BCA statements.
- **No PDF footer validation** of debt totals against statement totals.

## 4. Debt types & their quirks

| Type          | Schedule | Payment size        | Fee/interest quote     | Notes                                             |
|---------------|----------|---------------------|------------------------|---------------------------------------------------|
| KPR (mortgage)| Fixed    | Fixed monthly (editable) | Annual %           | Largest, slowest. Editable when the rate/term changes. |
| Credit card   | Flexible | Any amount ≥ min     | Monthly %             | Revolving; min payment may be a % of balance.     |
| Pinjol        | Flexible (short term) | Fixed install or lump | Often **daily** % | Trickiest to rank fairly. See §6 normalization.   |
| Other         | Either   | Varies               | Any period            | Catch-all (motor kredit, KTA, IOU).               |

The `debt_type` field drives UI labels and sensible defaults (e.g. selecting
"Pinjol" defaults `rate_period` to `daily`), but the **strategy logic treats
every debt the same way** once normalized.

## 5. Data model

Two new tables, added as **migration v4** in `budget-manager/src-tauri/src/db.rs`
(following the existing `PRAGMA user_version` + `while current < target_version`
pattern; current `target_version` is 3).

### 5.1 `debts`

| Column                  | Type      | Notes                                                                 |
|-------------------------|-----------|-----------------------------------------------------------------------|
| `id`                    | INTEGER PK| Autoincrement.                                                        |
| `name`                  | TEXT      | User label, e.g. "KPR BTN", "CC BCA Gold".                            |
| `debt_type`             | TEXT      | `mortgage` / `credit_card` / `pinjol` / `other`.                      |
| `original_principal`    | REAL      | Amount originally borrowed (informational).                           |
| `current_balance`       | REAL      | **Manually editable.** What the user still owes right now.            |
| `interest_rate`         | REAL      | Numeric rate. Units depend on `rate_period`.                          |
| `rate_period`           | TEXT      | `annual` / `monthly` / `daily` / `flat`.                              |
| `min_payment`           | REAL NULL | Minimum payment (flexible debts, e.g. credit card 10% of balance).    |
| `payment_schedule_type` | TEXT      | `fixed` / `flexible`.                                                 |
| `fixed_payment_amount`  | REAL NULL | Required when `payment_schedule_type = 'fixed'`; NULL otherwise.      |
| `linked_category_id`    | INTEGER   | → `categories.id`. The category that means "payment to this debt."    |
| `start_date`            | TEXT      | ISO date the debt started.                                            |
| `target_payoff_date`    | TEXT NULL | Optional target.                                                      |
| `status`                | TEXT      | `active` / `paid_off`. Default `active`.                              |
| `created_at`            | TEXT      | `DEFAULT CURRENT_TIMESTAMP`.                                          |
| `updated_at`            | TEXT      | Updated on edits.                                                     |

> No enforced FK to `categories` — consistent with the existing
> `transactions.category_id` convention (link-by-convention, not a SQL FK).

### 5.2 `debt_payments`

| Column                 | Type      | Notes                                                              |
|------------------------|-----------|--------------------------------------------------------------------|
| `id`                   | INTEGER PK| Autoincrement.                                                     |
| `debt_id`              | INTEGER   | → `debts.id`.                                                      |
| `amount`               | REAL      | Amount paid.                                                       |
| `paid_date`            | TEXT      | ISO date of the payment.                                           |
| `source`               | TEXT      | `manual` / `statement`.                                            |
| `linked_transaction_id`| INTEGER NULL | → `transactions.id` when derived from a statement row.         |
| `note`                 | TEXT      | Free text, optional.                                               |
| `created_at`           | TEXT      | `DEFAULT CURRENT_TIMESTAMP`.                                       |

### 5.3 Debt ↔ category link (1:1)

Each debt links to **one** category whose categorized transactions mean "a
payment toward this debt." Recommended flow: **creating a debt auto-creates a
category** named after the debt (e.g. debt "KPR BTN" → category "KPR BTN"), and
the user can relabel it. The user may also pick an existing category instead.
See decision **D4** (Proposed).

### 5.4 Hybrid payment detection

When the user imports a statement and assigns a transaction to a debt's linked
category, that transaction counts as a payment to that debt:

- Insert a `debt_payments` row with `source = 'statement'` and
  `linked_transaction_id` set to that transaction's id.
- Manual payments get `source = 'manual'`, `linked_transaction_id = NULL`.

**Double-counting risk:** if a user *also* records a manual payment for a
transaction that already came in via the statement, the same rupiah is counted
twice. This is handed to the decisions doc (D5) because the right answer depends
on whether `current_balance` or the payment log is the source of truth (D6).

## 6. Strategy logic — Avalanche with a fixed carve-out

The user said: *"fixed first, and highest fee first."* Translated into a rule:

1. **Pay every debt's scheduled obligation in full.**
   - Fixed debts (KPR, fixed-installment pinjol): pay `fixed_payment_amount`.
   - Flexible debts (credit card): pay at least `min_payment`.
2. **Any surplus** (the amount the user can afford beyond the sum of all
   scheduled obligations) is directed entirely to **one** flexible debt — the
   one with the **highest normalized fee**.
3. Fixed debts are **never accelerated** by surplus (accelerating a mortgage is
   rarely practical and out of scope). They receive only their fixed payment.

This is "avalanche" (highest-interest-first) with a "fixed carve-out" (fixed
debts are insulated from the surplus competition). It is the mathematically
cheapest strategy that still respects the reality that you can't meaningfully
overpay a KPR.

### 6.1 Rate normalization (so fees are comparable)

To rank a 24%/yr mortgage against a 0.4%/day pinjol on one scale, normalize each
`rate_period` to a **nominal annual rate (APR)**:

| `rate_period` | To nominal APR        | Caveat                                                   |
|---------------|-----------------------|----------------------------------------------------------|
| `annual`      | as-is                 | Already APR.                                             |
| `monthly`     | `rate × 12`           | Nominal APR (not compounded).                            |
| `daily`       | `rate × 365`          | Nominal APR (not compounded).                            |
| `flat`        | `rate × ~1.8` (approx)| Flat interest is charged on original principal; effective APR is higher. Marked approximate — see decision/risks doc. |

> The ranking uses **nominal** APR for simplicity and predictability. An
> effective (compounded) APR variant is a future option, not a Phase 1 need.

## 7. Progress & metrics

**Per debt:**
- `current_balance` (source of truth — see D6).
- Total paid (sum of `debt_payments.amount`).
- % paid down = `(original_principal − current_balance) / original_principal`.
- Projected payoff date (rough: based on current payment rate — optional, see D6).

**Portfolio:**
- Total debt outstanding.
- Total monthly obligation (sum of fixed payments + flexible minimums).
- Suggested surplus target (user-entered: "I can put Rp X toward debt this month")
  → which single debt to overpay, and by how much.

## 8. Functional requirements

| ID   | Requirement                                                                 |
|------|-----------------------------------------------------------------------------|
| FR-1 | User can create / read / update / delete a debt.                            |
| FR-2 | Creating a debt auto-creates (or links to) a category for its payments.     |
| FR-3 | User can record a **manual** payment against any debt.                      |
| FR-4 | A categorized statement transaction counts as a `statement`-source payment. |
| FR-5 | A "Strategy" view lists flexible debts ranked by normalized APR and shows the recommended surplus target. |
| FR-6 | A "Progress" view shows per-debt and portfolio metrics.                     |
| FR-7 | User can mark a debt `paid_off` (and reopen it).                            |
| FR-8 | User can edit a fixed payment amount manually (e.g. when KPR changes).      |

## 9. Non-functional requirements

- **Fully offline.** No network calls, no external APIs.
- **SQLite-only**, migrated via `PRAGMA user_version` (no external migration runner).
- **No AI / no ML.** All strategy output is a documented formula.
- Amounts formatted as IDR via the existing `formatAmount()` helper.
- New persistence follows the existing `debts.rs` + thin `#[tauri::command]`
  pattern (mirror `categories.rs`), with `LIMIT ?1 OFFSET ?2` pagination where
  lists grow long (notably `debt_payments` history).

## 10. Out of scope / future

- Auto-computed amortization schedule.
- Due-date reminders & notifications.
- Multi-currency / FX.
- Credit-limit and utilization tracking.
- Lender portal integration.
- Linking debt totals to statement footer validation (MUTASI CR/DB, SALDO AKHIR).
- Snowball alternative (smallest-balance-first) as a user-selectable strategy.

## 11. Open questions

Captured in `docs/debt-phases-and-decisions.md`, decisions D4–D6. The big ones:

- Is `current_balance` the source of truth, or derived from the payment log?
- How do we avoid double-counting statement + manual payments?
- Confirm: auto-create a category on debt creation, or always pick an existing one?
