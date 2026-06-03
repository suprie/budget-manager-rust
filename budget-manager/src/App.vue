<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

type Transaction = {
  id: number;
  trx_date: string;
  description: string;
  amount: number;
  trx_type: "CR" | "DB";
  posted_date: date
}

type TransactionSummary = {
  total_income: number;
  total_expenses: number;
  transactions: Transaction[];
}

const filePath = ref("");
const errorMessage = ref("");
const transactions = ref<Transaction[]>([]);
const transactionSummary = ref<TransactionSummary>();
const showCSVForm = ref(false);
const csvYear = ref(new Date().getFullYear());

const columns = [
  { title: "Date", dataIndex: "trx_date", key: "trx_date" },
  { title: "Description", dataIndex: "description", key: "description" },
  { title: "Amount", dataIndex: "amount", key: "amount" },
  { title: "Type", dataIndex: "trx_type", key: "trx_type" },
  { title: "Posted Date", dataIndex: "posted_date", key: "trx_type" },
];

const formatAmount = new Intl.NumberFormat("id-ID", {
  style: "currency",
  currency: "IDR",
  minimumFractionDigits: 2,
  maximumFractionDigits: 2,
});

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
  <main class="container">
    <div class="summary" v-if="transactionSummary">
      <span>Income: <strong class="income">{{ formatAmount.format(transactionSummary.total_income) }}</strong></span>
      <span>Expenses: <strong class="expenses">{{ formatAmount.format(transactionSummary.total_expenses) }}</strong></span>
    </div>

    <div class="actions">
      <a-button class="action-btn" @click="uploadPDFFile">Pick Statement PDF file</a-button>
      <a-button class="action-btn" @click="uploadCSVFile">Pick Statement CSV file</a-button>
      <a-button class="action-btn" @click="resetDB" :danger="true" :variant="solid">Reset Database</a-button>
    </div>

    <div v-if="showCSVForm" class="csv-form">
      <label>
        Year:
        <input v-model.number="csvYear" type="number" min="2000" max="2099" />
      </label>
      <a-button class="action-btn" @click="submitCSVWithYear">Choose CSV File</a-button>
      <a-button class="action-btn" @click="showCSVForm = false">Cancel</a-button>
    </div>

    <a-table v-if="transactionSummary?.transactions?.length" :data-source="transactionSummary.transactions" :columns="columns" rowKey="id">
      <template #bodyCell="{ column, record }">
        <span v-if="column.dataIndex == 'amount'" :class="'amount'"> {{ formatAmount.format(record.amount )}}</span>
        <span v-else-if="column.dataIndex === 'trx_type'" :class="['trx-type', record.trx_type.toLowerCase()]">
          {{ record.trx_type }}
        </span>
      </template>
    </a-table>
    <button  @click="resetDB">
      Reset Database
    </button>
    <p v-if="errorMessage">{{errorMessage}}</p>
  </main>
</template>

<style scoped>
.summary {
  display: flex;
  gap: 2rem;
  justify-content: center;
  margin-bottom: 1rem;
  font-size: 1.1rem;
}

.summary .income {
  color: #16a34a;
}

.summary .expenses {
  color: #dc2626;
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.csv-form {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.csv-form label {
  font-weight: 500;
}

.csv-form input {
  width: 80px;
  margin-left: 0.25rem;
}

.action-btn {
  min-width: 160px;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
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

  .trx-type {
    font-weight: 600;
  }

  .trx-type.cr {
    color: #dc2626;
  }

  .trx-type.db {
    color: #16a34a;
  }

.debit {
    color: #006666;
}

.credit {
    color: #666600;
}

:root {
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.a-table .amount {
    font-family: "IBMPlex Mono", monospace;
    font-variant-numeric: tabular-nums;
}

body {
  font-family: Inter, Arial;
  font-size: 16px;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

.description {
  text-align: left;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

.trx-date {
  margin-left: 20px;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #64748B;
    background-color: #f8fafc;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
