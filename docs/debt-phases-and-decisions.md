# Debt Tracking — Phases & Decisions

| Field   | Value                               |
|---------|-------------------------------------|
| Date    | 2026-06-30                          |
| PRD     | `docs/prd-adding-debt.md`           |
| Purpose | Rollout phases + decision log for the debt feature |

> Living document. Update decision status (Proposed → Decided) and phase
> checklists as work proceeds.

---

## 1. Decision log

Lightweight ADR-style. Each item has a status: **Decided** / **Proposed** / **Open**.

### D1 — Payoff strategy = Avalanche with fixed carve-out — **Decided**
Pay every debt's scheduled obligation (fixed installment or flexible minimum),
then direct all surplus to the single **highest-normalized-fee flexible** debt.
Fixed debts are never accelerated. See PRD §6.

### D2 — Rate storage = numeric `interest_rate` + `rate_period` enum — **Decided**
One numeric field plus a period (`annual` / `monthly` / `daily` / `flat`).
Ranking normalizes to nominal APR so daily pinjol and annual KPR compare fairly.
Alternatives considered: single APR field (rejected — pushes error-prone mental
math onto the user); free-text fee (rejected — not numerically rankable).

### D3 — Progress tracking = hybrid (statement-derived + manual) — **Decided**
Payments come from two sources: categorized statement transactions and manual
entries. Both live in `debt_payments` and are tagged by `source`.

### D4 — Debt ↔ category relationship = 1:1, auto-create on debt creation — **Proposed**
Creating a debt auto-creates a category of the same name; the user may relabel
it or choose an existing category instead.
**Open sub-question:** what if a user wants one category to feed *multiple*
debts (e.g. a generic "Loan payment" split across two loans)? 1:1 disallows
that. Confirm whether 1:1 is acceptable for v1.

### D5 — Double-counting handling — **Open**
If a transaction is imported via statement **and** the user also logs a manual
payment for the same rupiah, the payment log double-counts. Options:

1. **Manual = only for non-statement payments.** UX nudge: if a transaction is
   already linked, warn when adding a manual payment on the same date+amount.
2. **Link a manual payment to an existing transaction.** Lets the user say
   "this manual entry is really that transaction."
3. **Balance is truth; log is informational.** `current_balance` is edited by
   hand; `debt_payments` is a read-only audit trail that never mutates balance.
   Double-counting then only inflates the "total paid" display, not the balance.

Option 3 is the simplest and composes well with D6. **Recommend resolving D6
first** — D5 follows from it.

### D6 — Source of truth: `current_balance` vs. derived from payments — **Open**
Two honest models:

- **(a) `current_balance` is truth (manual-first).** User types/edits the
  balance after each statement. `debt_payments` is an audit trail. Matches the
  PRD's "change manually" note for KPR. Simple, robust to re-imports, but the
  "total paid" metric can drift from `original_principal − current_balance`.
- **(b) Balance is derived.** `current_balance = original_principal − Σ(payments)`.
  No drift, but re-imports, corrections, and interest accrual make this fragile
  (the balance would also need fees added back, which we don't model).

**Lean: (a).** It survives real data (re-imports via `INSERT OR IGNORE`, manual
fixes, interest the app doesn't compute) and matches how the user already thinks
about a KPR balance. The tradeoff is accepted: "total paid" is approximate.

### D7 — Schema delivery = migration v4 in `db.rs` — **Decided**
Add `debts` and `debt_payments` as migration version 4, following the existing
inline `PRAGMA user_version` + `while current < target_version` loop
(`target_version` is currently 3). No new migration runner, no `.sql` files.

### D8 — Code layout = new `debts.rs` + thin Tauri commands — **Decided**
Mirror `categories.rs`: plain `pub fn`s taking `&Connection`, returning
`Result<_, String>`; thin `#[tauri::command]` wrappers in `lib.rs` that open the
DB and delegate; register in the existing `generate_handler!` list. Frontend
adds types to `src/types.ts`, a `src/pages/DebtsPage.vue`, a route in
`src/router.ts`, and a sidebar item in `App.vue`.

---

## 2. Phases

Each phase ships something usable and defers polish. Matches the repo ethos
(happy path first, tests for the gnarly bits later but not never).

### Phase 0 — Schema & model
**Goal:** the data exists and can be read/written, no UI yet.
**Scope:**
- Migration v4: `debts` + `debt_payments` tables (PRD §5.1, §5.2).
- Rust structs (`Debt`, `DebtPayment`) deriving `serde::Serialize`.
- `debts.rs` helpers: insert/read/list/update/delete debt; insert/list payment.
**Deferred:** category link, statement detection, strategy, UI.
**Tests:** migration runs cleanly from a fresh DB and from an existing v3 DB;
insert→read round-trip for both tables; `paid_off` status flips.

### Phase 1 — Manual debt CRUD + manual payment entry
**Goal:** a user can manage debts and log payments by hand.
**Scope:**
- Tauri commands for debt CRUD + manual payment.
- `DebtsPage.vue`: add/list/edit/delete debts, record a manual payment, edit
  `current_balance` and `fixed_payment_amount` inline.
- `linked_category_id` is nullable / stubbed off.
**Deferred:** category auto-create, statement-derived payments, strategy, charts.
**Tests:** malformed debt input (negative balance, empty name); delete cascades
or guards `debt_payments` (decide in this phase); command error paths.

### Phase 2 — Category link + statement-derived payments
**Goal:** imported transactions automatically count as payments.
**Scope:**
- On debt creation, auto-create/link a category (D4).
- When a transaction is assigned (or re-assigned) to a debt's category, insert a
  `debt_payments` row (`source = 'statement'`, `linked_transaction_id` set).
- Surface statement vs. manual in the payment list.
**Deferred:** strategy, charts, double-count UX.
**Tests:** re-importing the same statement does not duplicate payment rows
(respect the transactions `INSERT OR IGNORE` dedup); re-categorizing a
transaction moves/creates the payment correctly; double-count scenario
documented even if not yet resolved (D5).

### Phase 3 — Strategy recommendation view
**Goal:** tell the user where to put their surplus this month.
**Scope:**
- Rate normalization (PRD §6.1).
- Compute the avalanche-with-fixed-carve-out suggestion.
- "Overpay this debt" panel; user enters a surplus amount.
**Deferred:** projected payoff dates, portfolio charts.
**Tests:** normalization for each `rate_period` (incl. flat approximation);
ranking with a mix of fixed + flexible debts; a fixed debt never gets surplus.

### Phase 4 — Progress & metrics UI
**Goal:** show per-debt and portfolio progress.
**Scope:**
- Per-debt: current balance, total paid, % paid down, (optional) projected payoff.
- Portfolio summary cards matching the Dashboard styling (reuse `formatAmount`,
  the `.summary-cards` grid, and the color tokens).
**Deferred:** time-series charts.
**Tests:** metrics math; % paid down when balance > principal (overpaid/fees);
empty portfolio (no debts) renders cleanly.

### Phase 5 — Polish & edge cases
**Goal:** harden for real data.
**Scope:**
- Pagination on `debt_payments` history (`LIMIT ?1 OFFSET ?2`).
- Mark-paid-off / reopen flow (FR-7) with a guard against closing a debt with a
  non-zero balance unless confirmed.
- Resolve D5 (double-counting) per the decision made.
- Tests for rate normalization edge cases and double-counting.
**Deferred:** anything in PRD §10.

---

## 3. Risks & challenges

- **Flat vs. effective interest (pinjol/motor kredit).** Indonesian flat-rate
  loans quote a rate on the *original* principal for the whole term, so the true
  APR is much higher than the quoted number. The `×1.8` approximation is a
  ranking heuristic, not a precise APR. Document this clearly in the UI so the
  user isn't misled.
- **Double counting (D5).** Statement-derived + manual payments can overlap.
  Resolve only after D6.
- **`current_balance` vs. derived balance (D6).** Re-imports, manual
  corrections, and un-modeled interest make a derived balance fragile. Lean
  manual-first (option a).
- **Re-import idempotency.** The existing `transactions` `INSERT OR IGNORE`
  dedup means a re-imported transaction keeps its id; payment detection must key
  off `linked_transaction_id` (and dedup on it) so re-importing a statement does
  not double-count payments.
- **Editing a fixed payment mid-life.** KPR rates/terms change. `fixed_payment_amount`
  is editable, but history (what was paid *before* the change) lives only in
  `debt_payments` — there is no per-period schedule. Acceptable for v1.
- **Currency.** Single-currency (IDR) only. If a debt is in USD, the user must
  convert manually today.

---

## 4. Open questions for the maintainer

1. **D6 first, then D5:** is `current_balance` the manual source of truth
   (option a)? If yes, double-counting only affects the "total paid" display.
2. **D4:** is 1:1 debt↔category acceptable for v1, or does one category need to
   feed multiple debts?
3. Should "mark paid off" require `current_balance == 0`, or allow a write-off
   with a note?
4. Do we want a projected-payoff date at all in v1 (it implies a payment-rate
   assumption that may mislead), or defer it entirely?
