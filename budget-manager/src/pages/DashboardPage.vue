<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { Category, TransactionSummary } from "../types";
import { formatAmount, textColorForBg } from "../utils";

const filePath = ref("");
const errorMessage = ref("");
const transactionSummary = ref<TransactionSummary>();
const showCSVForm = ref(false);
const csvYear = ref(new Date().getFullYear());

// ── Pagination ──────────────────────────────────────────────────────
const pageSize = ref(50);
const currentPage = ref(1);

// ── Inline category edit ───────────────────────────────────────────
const categories = ref<Category[]>([]);
const editingTxId = ref<number | null>(null);

async function loadCategories() {
  try {
    categories.value = await invoke<Category[]>("get_all_categories");
  } catch (e) {
    console.error("Failed to load categories:", e);
  }
}

function startEditing(txId: number) {
  editingTxId.value = txId;
}

async function reassignCategory(txId: number, categoryId: number) {
  editingTxId.value = null;
  try {
    await invoke<number>("bulk_assign_category", {
      transactionIds: [txId],
      categoryId,
    });
    await loadTransaction();
  } catch (e) {
    console.error("Failed to reassign category:", e);
  }
}

const columns = [
  { title: "Date", dataIndex: "trx_date", key: "trx_date" },
  { title: "Description", dataIndex: "description", key: "description" },
  { title: "Amount", dataIndex: "amount", key: "amount", align: "right" },
  { title: "Category", dataIndex: "category.category_name", key: "category_name" },
  { title: "Posted Date", dataIndex: "posted_date", key: "posted_date" },
  { title: "Source", dataIndex: "source", key: "source" },
];

async function loadTransaction() {
  const offset = (currentPage.value - 1) * pageSize.value;
  transactionSummary.value = await invoke<TransactionSummary>("load_transactions", {
    limit: pageSize.value,
    offset,
  });
}

function handlePageChange(page: number, size: number) {
  currentPage.value = page;
  pageSize.value = size;
  loadTransaction();
}

async function uploadPDFFile() {
  errorMessage.value = "";
  filePath.value = "";

  const selectedFile = await open({
    multiple: false,
    filters: [{ name: "Statement", extensions: ["pdf"] }],
  });

  if (!selectedFile || Array.isArray(selectedFile)) return;

  try {
    filePath.value = await invoke<string>("read_statement_file", { filepath: selectedFile });
    await loadTransaction();
  } catch (error) {
    errorMessage.value = String(error);
  }
}

async function uploadCSVFile() {
  errorMessage.value = "";
  filePath.value = "";
  showCSVForm.value = true;
}

async function submitCSVWithYear() {
  showCSVForm.value = false;

  const selectedFile = await open({
    multiple: false,
    filters: [{ name: "Statement", extensions: ["csv"] }],
  });

  if (!selectedFile || Array.isArray(selectedFile)) return;

  try {
    filePath.value = await invoke<string>("read_csv_statement_file", {
      filepath: selectedFile,
      year: csvYear.value,
    });
    await loadTransaction();
  } catch (error) {
    errorMessage.value = String(error);
  }
}

onMounted(() => {
  loadTransaction();
  loadCategories();
});
</script>

<template>
  <div>
    <header class="content-header">
      <p class="header-label">Account Overview</p>
      <h2 class="header-title">Portfolio Activity</h2>
    </header>

    <!-- Summary Cards -->
    <section class="summary-cards" v-if="transactionSummary">
      <div class="card card-income">
        <div class="card-top">
          <span class="card-label">Total Income</span>
        </div>
        <p class="card-amount income">{{ formatAmount(transactionSummary.total_income) }}</p>
      </div>
      <div class="card card-expenses">
        <div class="card-top">
          <span class="card-label">Total Expenses</span>
        </div>
        <p class="card-amount expenses">{{ formatAmount(transactionSummary.total_expenses) }}</p>
      </div>
      <div class="card card-net">
        <div class="card-top">
          <span class="card-label">Net Balance</span>
        </div>
        <p
          class="card-amount"
          :class="transactionSummary.total_income - transactionSummary.total_expenses >= 0 ? 'income' : 'expenses'"
        >
          {{ formatAmount(transactionSummary.total_income - transactionSummary.total_expenses) }}
        </p>
      </div>
    </section>

    <!-- File Upload Actions -->
    <section class="actions-section">
      <div class="actions">
        <a-button class="action-btn" @click="uploadPDFFile">Pick Statement PDF file</a-button>
        <a-button class="action-btn" @click="uploadCSVFile">Pick Statement CSV file</a-button>
      </div>

      <div v-if="showCSVForm" class="csv-form">
        <label>
          Year:
          <input v-model.number="csvYear" type="number" min="2000" max="2099" />
        </label>
        <a-button class="action-btn" @click="submitCSVWithYear">Choose CSV File</a-button>
        <a-button class="action-btn" @click="showCSVForm = false">Cancel</a-button>
      </div>
    </section>

    <!-- Transaction Table -->
    <section class="table-section">
      <div class="table-header">
        <h3 class="table-title">Recent Transactions</h3>
      </div>
      <a-table
        v-if="transactionSummary?.transactions?.length"
        :data-source="transactionSummary.transactions"
        :columns="columns"
        rowKey="id"
        :pagination="{
          current: currentPage,
          pageSize: pageSize,
          total: transactionSummary.total_count,
          showSizeChanger: true,
          pageSizeOptions: ['25', '50', '100'],
          onChange: handlePageChange,
          onShowSizeChange: handlePageChange,
        }"
      >
        <template #bodyCell="{ column, record }">
          <span v-if="column.dataIndex == 'amount'" class="amount-cell" :class="record.trx_type.toLowerCase()">
            {{ formatAmount(record.trx_type === 'DB' ? -record.amount : record.amount) }}
          </span>
          <span v-else-if="column.dataIndex === 'category.category_name'">
            <a-popover
              :open="editingTxId === record.id"
              placement="bottomLeft"
              @openChange="(visible: boolean) => { if (!visible) editingTxId = null; }"
            >
              <template #content>
                <div class="popover-category-list">
                  <div
                    v-for="cat in categories"
                    :key="cat.id"
                    class="popover-category-item"
                    :class="{ active: cat.id === record.category.id }"
                    @click="reassignCategory(record.id, cat.id)"
                  >
                    <span class="cat-swatch" :style="{ backgroundColor: cat.color }" />
                    {{ cat.category_name }}
                  </div>
                </div>
              </template>
              <span
                class="category-tag"
                :style="{
                  backgroundColor: record.category.color,
                  color: textColorForBg(record.category.color),
                }"
                @click="startEditing(record.id)"
              >
                {{ record.category.category_name }}
              </span>
            </a-popover>
          </span>
        </template>
      </a-table>
    </section>

    <p v-if="errorMessage" class="error-msg">{{ errorMessage }}</p>
  </div>
</template>

<style scoped>
.content-header {
  margin-bottom: 32px;
}

.header-label {
  font-size: 12px;
  font-weight: 700;
  color: #00685f;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin: 0 0 4px;
}

.header-title {
  font-size: 32px;
  font-weight: 700;
  color: #191c1e;
  margin: 0;
  line-height: 1.15;
}

/* Summary Cards */
.summary-cards {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-bottom: 32px;
}

.card {
  background: white;
  border: 1px solid #e0e3e5;
  border-radius: 8px;
  padding: 24px;
}

.card-top {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.card-label {
  font-size: 12px;
  font-weight: 700;
  color: #6d7a77;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.card-amount {
  font-family: "IBMPlex Mono", monospace;
  font-size: 28px;
  font-weight: 600;
  margin: 8px 0;
}

.card-amount.income { color: #16a34a; }
.card-amount.expenses { color: #dc2626; }

/* Actions */
.actions-section { margin-bottom: 24px; }

.actions {
  display: flex;
  gap: 8px;
}

.csv-form {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 12px;
}

.csv-form label { font-weight: 500; font-size: 14px; }

.csv-form input {
  width: 80px;
  margin-left: 4px;
  border: 1px solid #e0e3e5;
  border-radius: 4px;
  padding: 6px 8px;
  font-size: 14px;
}

.action-btn { min-width: 160px; }

/* Table */
.table-section {
  background: white;
  border: 1px solid #e0e3e5;
  border-radius: 8px;
  overflow: hidden;
}

.table-header {
  padding: 20px 24px;
  border-bottom: 1px solid #e0e3e5;
}

.table-title {
  font-size: 18px;
  font-weight: 600;
  color: #191c1e;
  margin: 0;
}

.table-section :deep(.ant-table) { font-family: Inter, sans-serif; }

.table-section :deep(.ant-table-thead > tr > th) {
  background: #f2f4f6;
  font-size: 12px;
  font-weight: 700;
  color: #6d7a77;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  padding: 14px 24px;
}

.table-section :deep(.ant-table-pagination) {
  padding: 16px 24px;
  margin: 0 !important;
}

.amount-cell {
  font-family: "IBMPlex Mono", monospace;
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
}

.amount-cell.cr { color: #16a34a; }
.amount-cell.db { color: #dc2626; }

/* Category Tag */
.category-tag {
  display: inline-block;
  padding: 2px 10px;
  border-radius: 9999px;
  font-size: 10px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.02em;
  cursor: pointer;
  transition: opacity 0.15s;
}
.category-tag:hover {
  opacity: 0.8;
}

.error-msg {
  color: #dc2626;
  margin-top: 16px;
  font-size: 14px;
}

/* Category Popover */
.popover-category-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 160px;
  max-height: 260px;
  overflow-y: auto;
}

.popover-category-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.1s;
}
.popover-category-item:hover {
  background: #f2f4f6;
}
.popover-category-item.active {
  background: #d0e1fb;
  color: #38485d;
}

.cat-swatch {
  width: 14px;
  height: 14px;
  border-radius: 50%;
  flex-shrink: 0;
}
</style>
