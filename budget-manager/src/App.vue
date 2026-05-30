<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
type Transaction = {
  id: number;
  trx_date: string;
  description: string;
  amount: number;
  trx_type: "CR" | "DB"
}

const filePath = ref("");
const errorMessage = ref("");
const transactions = ref<Transaction[]>([]);

  const columns = [
    { title: "Date", dataIndex: "trx_date", key: "trx_date" },
    { title: "Description", dataIndex: "description", key: "description" },
    { title: "Amount", dataIndex: "amount", key: "amount" },
    { title: "Type", dataIndex: "trx_type", key: "trx_type" },
  ];

  const formatAmount = new Intl.NumberFormat("id-ID", {
    style: "currency",
    currency: "IDR",
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });

async function loadTransaction() {
  transactions.value = await invoke<Transaction[]>("list_transactions")
}

async function uploadFile() {
  errorMessage.value = "";
  filePath.value = "";

  const selectedFile = await open({
    multiple: false,
    filters: [
      {
        name: "Statement",
        extensions: ["csv", "pdf"],
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
  } catch (error) {
    errorMessage.value = String(error);
  }
}

async function resetDB() {
  try {
    await invoke("reset_db")
  } catch (error) {
    errorMessage.value = String(error);
  }
}

onMounted(loadTransaction);
</script>

<template>
  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>
    <a-button  @click="uploadFile">
      Pick Statement file
    </a-button>

    <a-table v-if="transactions.length" :data-source="transactions" :columns="columns" rowKey="id">
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
