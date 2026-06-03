<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

type Category = {
  id: number,
  category_name: string,
  color: string,
}

type Transaction = {
  id: number;
  trx_date: string;
  description: string;
  amount: number;
  trx_type: "CR" | "DB";
  posted_date: string | null;
  category: Category
}

type TransactionSummary = {
  total_income: number;
  total_expenses: number;
  transactions: Transaction[];
}

const currentPage = ref(["dashboard"]);
const filePath = ref("");
const errorMessage = ref("");
const transactionSummary = ref<TransactionSummary>();
const showCSVForm = ref(false);
const csvYear = ref(new Date().getFullYear());

function textColorForBg(hex: string): string {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  return (r * 0.299 + g * 0.587 + b * 0.114) > 140 ? "#1e293b" : "#ffffff";
}

const columns = [
  { title: "Date", dataIndex: "trx_date", key: "trx_date" },
  { title: "Description", dataIndex: "description", key: "description" },
  { title: "Amount", dataIndex: "amount", key: "amount", align: "right" },
  { title: "Category", dataIndex: "category.category_name", key: "category_name" },
  { title: "Posted Date", dataIndex: "posted_date", key: "posted_date" },
  { title: "Source", dataIndex: "source", key: "source" },
];

function formatAmount(value: number): string {
  const abs = Math.abs(value);
  const formatted = new Intl.NumberFormat("id-ID", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(abs);
  return value < 0 ? `Rp -${formatted}` : `Rp ${formatted}`;
}

async function loadTransaction() {
  transactionSummary.value = await invoke<TransactionSummary>("load_transactions")
}

async function uploadPDFFile() {
  errorMessage.value = "";
  filePath.value = "";

  const selectedFile = await open({
    multiple: false,
    filters: [
      {
        name: "Statement",
        extensions: ["pdf"],
      },
    ],
  });

  if (!selectedFile) {
    return;
  }

  if (Array.isArray(selectedFile)) {
    return;
  }

  try {
    filePath.value = await invoke<string>("read_statement_file", {
      filepath: selectedFile,
    });
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
    filters: [
      {
        name: "Statement",
        extensions: ["csv"],
      },
    ],
  });

  if (!selectedFile) {
    return;
  }

  if (Array.isArray(selectedFile)) {
    return;
  }

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

async function resetDB() {
  try {
    await invoke("reset_db");
    await loadTransaction();
  } catch (error) {
    errorMessage.value = String(error);
  }
}

onMounted(loadTransaction);
</script>

<template>
  <a-layout class="layout">
    <!-- Sidebar -->
    <a-layout-sider
      width="240"
      class="sider"
    >
      <div class="sider-brand">
        <h1 class="sider-title">Budget Manager</h1>
        <p class="sider-subtitle">Personal Finance</p>
      </div>

      <a-menu
        v-model:selectedKeys="currentPage"
        mode="inline"
        class="sider-menu"
      >
        <a-menu-item key="dashboard">
          <span class="menu-icon">&#9632;</span>
          Dashboard
        </a-menu-item>
        <a-menu-item key="categories">
          <span class="menu-icon">&#9632;</span>
          Categories
        </a-menu-item>
        <a-menu-item key="settings">
          <span class="menu-icon">&#9632;</span>
          Settings
        </a-menu-item>
      </a-menu>
    </a-layout-sider>

    <!-- Main Content -->
    <a-layout>
      <a-layout-content class="content">

        <!-- Dashboard Page -->
        <div v-if="currentPage[0] === 'dashboard'">
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
            >
              <template #bodyCell="{ column, record }">
                <span v-if="column.dataIndex == 'amount'" class="amount-cell" :class="record.trx_type.toLowerCase()">
                  {{ formatAmount(record.trx_type === 'DB' ? -record.amount : record.amount) }}
                </span>
                <span v-else-if="column.dataIndex === 'category.category_name'">
                  <span
                    class="category-tag"
                    :style="{
                      backgroundColor: record.category.color,
                      color: textColorForBg(record.category.color)
                    }"
                  >
                    {{ record.category.category_name }}
                  </span>
                </span>
              </template>
            </a-table>
          </section>
        </div>

        <!-- Categories Page -->
        <div v-if="currentPage[0] === 'categories'">
          <header class="content-header">
            <p class="header-label">Manage</p>
            <h2 class="header-title">Categories</h2>
          </header>
          <p class="placeholder-text">Category management coming soon.</p>
        </div>

        <!-- Settings Page -->
        <div v-if="currentPage[0] === 'settings'">
          <header class="content-header">
            <p class="header-label">Configuration</p>
            <h2 class="header-title">Settings</h2>
          </header>
          <section class="settings-section">
            <div class="card card-danger">
              <div class="card-top">
                <span class="card-label">Danger Zone</span>
              </div>
              <p class="card-description">Reset the database and delete all imported transactions.</p>
              <a-button @click="resetDB" danger type="primary">Reset Database</a-button>
            </div>
          </section>
        </div>

        <p v-if="errorMessage" class="error-msg">{{ errorMessage }}</p>
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>

<style scoped>
/* Layout */
.layout {
  min-height: 100vh;
}

.content {
  padding: 40px 48px;
  max-width: 1200px;
  margin: 0 auto;
  width: 100%;
}

/* Sidebar */
.sider {
  background: #f7f9fb !important;
  border-right: 1px solid #e0e3e5;
}

.sider-brand {
  padding: 28px 28px 24px;
}

.sider-title {
  font-family: Inter, sans-serif;
  font-size: 18px;
  font-weight: 700;
  color: #00685f;
  margin: 0;
  line-height: 1.2;
}

.sider-subtitle {
  font-size: 11px;
  font-weight: 600;
  color: #6d7a77;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  margin: 4px 0 0;
}

.sider-menu {
  background: transparent !important;
  border-right: none !important;
  margin-top: 8px;
}

.sider-menu :deep(.ant-menu-item) {
  margin: 2px 16px;
  padding-left: 16px !important;
  border-radius: 8px;
  font-weight: 600;
  font-size: 13px;
  color: #6d7a77;
}

.sider-menu :deep(.ant-menu-item-selected) {
  background: #d0e1fb !important;
  color: #38485d !important;
}

.menu-icon {
  margin-right: 8px;
  font-size: 10px;
  vertical-align: middle;
}

/* Content Header */
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

.card-amount.income {
  color: #16a34a;
}

.card-amount.expenses {
  color: #dc2626;
}

.card-danger {
  max-width: 480px;
}

.card-description {
  font-size: 14px;
  color: #6d7a77;
  margin: 0 0 16px;
}

/* Actions */
.actions-section {
  margin-bottom: 24px;
}

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

.csv-form label {
  font-weight: 500;
  font-size: 14px;
}

.csv-form input {
  width: 80px;
  margin-left: 4px;
  border: 1px solid #e0e3e5;
  border-radius: 4px;
  padding: 6px 8px;
  font-size: 14px;
}

.action-btn {
  min-width: 160px;
}

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

.table-section :deep(.ant-table) {
  font-family: Inter, sans-serif;
}

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

.amount-cell.cr {
  color: #16a34a;
}

.amount-cell.db {
  color: #dc2626;
}

/* Category Tag */
.category-tag {
  display: inline-block;
  padding: 2px 10px;
  border-radius: 9999px;
  font-size: 10px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.02em;
}

/* Settings */
.settings-section {
  margin-top: 8px;
}

/* Placeholder */
.placeholder-text {
  font-size: 14px;
  color: #6d7a77;
}

/* Error */
.error-msg {
  color: #dc2626;
  margin-top: 16px;
  font-size: 14px;
}
</style>

<style>
@font-face {
  font-family: "Inter";
  src: url("./assets/fonts/Inter-VariableFont_opsz,wght.ttf") format("ttf");
  font-weight: 400;
}

@font-face {
  font-family: "IBMPlex Mono";
  src: url("./assets/fonts/IBMPlexMono-Regular.ttf") format("ttf");
  font-weight: 400;
}

:root {
  line-height: 24px;
  font-weight: 400;
  color: #0f0f0f;
  background-color: #f7f9fb;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

body {
  font-family: Inter, Arial, sans-serif;
  font-size: 16px;
  margin: 0;
}

/* Override Ant Design default styles */
.ant-layout {
  background: #f7f9fb !important;
}

.ant-layout-sider {
  background: #f7f9fb !important;
}
</style>
