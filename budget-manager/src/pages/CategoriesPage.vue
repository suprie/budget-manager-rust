<script setup lang="ts">
import { ref, computed, watch, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { Category } from "../types";

// ── Preset color palette ──────────────────────────────────────────
const PRESET_COLORS = [
  "#00685f", // primary teal
  "#3B82F6", // blue
  "#8B5CF6", // violet
  "#EC4899", // pink
  "#F59E0B", // amber
  "#10B981", // emerald
  "#64748B", // slate
  "#EF4444", // red
  "#14B8A6", // teal
  "#F97316", // orange
  "#84CC16", // lime
  "#06B6D4", // cyan
];

// ── Backend type for uncategorized rows ────────────────────────────
interface BackendUncategorized {
  id: number;
  trx_date: string;
  description: string;
  amount: number;
  trx_type: string;
  posted_date: string | null;
  source: string;
  category: Category;
}

// ── Add Category form state ───────────────────────────────────────
const categoryName = ref("");
const selectedColor = ref(PRESET_COLORS[0]);
const customColorInput = ref("");
const showCustomPicker = ref(false);
const adding = ref(false);

const activeColor = computed(() =>
  showCustomPicker.value ? customColorInput.value || "#000000" : selectedColor.value
);

function selectPreset(color: string) {
  selectedColor.value = color;
  showCustomPicker.value = false;
}

function openCustomPicker() {
  showCustomPicker.value = true;
}

async function handleAddCategory() {
  const name = categoryName.value.trim();
  if (!name) return;

  const color = activeColor.value;
  adding.value = true;
  try {
    const created = await invoke<Category>("create_category", {
      categoryName: name,
      color,
    });
    categories.value.push(created);

    categoryName.value = "";
    selectedColor.value = PRESET_COLORS[0];
    customColorInput.value = "";
    showCustomPicker.value = false;
  } catch (e) {
    console.error("Failed to create category:", e);
  } finally {
    adding.value = false;
  }
}

// ── Categories list ───────────────────────────────────────────────
const categories = ref<Category[]>([]);

async function loadCategories() {
  try {
    categories.value = await invoke<Category[]>("get_all_categories");
  } catch (e) {
    console.error("Failed to load categories:", e);
  }
}

// ── Bulk Assign state ─────────────────────────────────────────────
const keyword = ref("");
const selectedTxIds = ref<Set<number>>(new Set());
const targetCategoryId = ref<number | null>(null);
const uncategorizedTxns = ref<BackendUncategorized[]>([]);
const txLoading = ref(false);
const assigning = ref(false);

let searchTimer: ReturnType<typeof setTimeout> | null = null;

async function loadUncategorized(kw: string) {
  txLoading.value = true;
  try {
    uncategorizedTxns.value = await invoke<BackendUncategorized[]>(
      "get_uncategorized_transactions",
      { keyword: kw }
    );
  } catch (e) {
    console.error("Failed to load uncategorized transactions:", e);
  } finally {
    txLoading.value = false;
  }
}

// Search is debounced so we don't hit SQLite on every keystroke.
watch(keyword, (kw) => {
  if (searchTimer) clearTimeout(searchTimer);
  searchTimer = setTimeout(() => loadUncategorized(kw), 250);
});

function onSelectionChange(keys: number[]) {
  selectedTxIds.value = new Set(keys);
}

async function handleBulkAssign() {
  const ids = [...selectedTxIds.value];
  if (ids.length === 0 || !targetCategoryId.value) return;

  assigning.value = true;
  try {
    await invoke<number>("bulk_assign_category", {
      transactionIds: ids,
      categoryId: targetCategoryId.value,
    });
    selectedTxIds.value = new Set();
    targetCategoryId.value = null;
    // Refresh the list so assigned rows disappear.
    await loadUncategorized(keyword.value);
  } catch (e) {
    console.error("Failed to bulk assign:", e);
  } finally {
    assigning.value = false;
  }
}

onMounted(() => {
  loadCategories();
  loadUncategorized("");
});
</script>

<template>
  <div class="categories-page">
    <!-- ── Add New Category ─────────────────────────────────────── -->
    <section class="card add-category-card">
      <h3 class="card-title">Add New Category</h3>

      <div class="form-group">
        <label class="field-label">CATEGORY NAME</label>
        <a-input
          v-model:value="categoryName"
          placeholder="e.g. Travel, Groceries"
          size="large"
          class="field-input"
          @pressEnter="handleAddCategory"
        />
      </div>

      <div class="form-group">
        <label class="field-label">COLOR</label>

        <!-- preset swatches -->
        <div class="swatch-row">
          <button
            v-for="color in PRESET_COLORS"
            :key="color"
            type="button"
            class="swatch"
            :class="{ active: !showCustomPicker && selectedColor === color }"
            :style="{ backgroundColor: color }"
            :title="color"
            @click="selectPreset(color)"
          />

          <!-- custom color trigger -->
          <button
            type="button"
            class="swatch custom-swatch"
            :class="{ active: showCustomPicker }"
            :style="showCustomPicker ? { backgroundColor: customColorInput || '#000' } : {}"
            title="Custom color"
            @click="openCustomPicker"
          >
            <span v-if="!showCustomPicker" class="custom-swatch-icon">+</span>
          </button>
        </div>

        <!-- inline custom color input -->
        <div v-if="showCustomPicker" class="custom-color-row">
          <input
            v-model="customColorInput"
            type="color"
            class="native-color-input"
          />
          <a-input
            v-model:value="customColorInput"
            placeholder="#000000"
            size="small"
            class="hex-input"
            maxlength="7"
          />
          <a-button size="small" @click="showCustomPicker = false">Cancel</a-button>
        </div>
      </div>

      <a-button
        type="primary"
        block
        size="large"
        :disabled="!categoryName.trim()"
        :loading="adding"
        @click="handleAddCategory"
      >
        <template #icon>+</template>
        Add Category
      </a-button>
    </section>

    <!-- ── Existing Categories list ──────────────────────────────── -->
    <section v-if="categories.length" class="card">
      <h3 class="card-title">Your Categories</h3>
      <div class="category-list">
        <div
          v-for="cat in categories"
          :key="cat.id"
          class="category-row"
        >
          <span class="cat-swatch" :style="{ backgroundColor: cat.color }" />
          <span class="cat-name">{{ cat.category_name }}</span>
          <span class="cat-color-hex">{{ cat.color }}</span>
        </div>
      </div>
    </section>

    <!-- ── Bulk Assign Transactions ──────────────────────────────── -->
    <section class="card">
      <div class="bulk-header">
        <div>
          <h3 class="card-title">Bulk Assign Transactions</h3>
          <p class="card-sub">Easily map historical data to your new category.</p>
        </div>
        <div class="bulk-controls">
          <a-input
            v-model:value="keyword"
            placeholder="Search transactions..."
            size="large"
            class="bulk-search"
            allow-clear
          >
            <template #prefix>
              <span style="color: #6d7a77">&#8981;</span>
            </template>
          </a-input>
          <a-select
            v-if="categories.length"
            v-model:value="targetCategoryId"
            placeholder="Select target category"
            style="min-width: 220px"
            size="large"
            allow-clear
          >
            <a-select-option
              v-for="cat in categories"
              :key="cat.id"
              :value="cat.id"
            >
              <span class="cat-swatch sm" :style="{ backgroundColor: cat.color }" />
              {{ cat.category_name }}
            </a-select-option>
          </a-select>
          <a-button
            type="primary"
            size="large"
            :disabled="!targetCategoryId || selectedTxIds.size === 0"
            :loading="assigning"
            @click="handleBulkAssign"
          >
            Assign Selected to Category
          </a-button>
        </div>
      </div>

      <a-table
        :data-source="uncategorizedTxns"
        :loading="txLoading"
        :pagination="false"
        row-key="id"
        size="middle"
        :row-selection="{
          selectedRowKeys: [...selectedTxIds],
          onChange: onSelectionChange,
        }"
      >
        <a-table-column title="DATE" data-index="trx_date" :width="130">
          <template #default="{ text }">
            <span class="mono">{{ text }}</span>
          </template>
        </a-table-column>
        <a-table-column title="DESCRIPTION" data-index="description" />
        <a-table-column title="AMOUNT" data-index="amount" align="right" :width="140">
          <template #default="{ text }">
            <span class="mono">${{ text.toFixed(2) }}</span>
          </template>
        </a-table-column>
        <a-table-column title="STATUS" data-index="category" :width="140">
          <template #default>
            <a-tag>Uncategorized</a-tag>
          </template>
        </a-table-column>
      </a-table>

      <div class="bulk-footer">
        <span class="result-count">
          {{ uncategorizedTxns.length }} result{{ uncategorizedTxns.length !== 1 ? "s" : "" }}
        </span>
      </div>
    </section>
  </div>
</template>

<style scoped>
/* ── Layout ─────────────────────────────────────────────────────── */
.categories-page {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

/* ── Card ───────────────────────────────────────────────────────── */
.card {
  background: #fff;
  border: 1px solid #e0e3e5;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0px 4px 12px rgba(15, 23, 42, 0.03);
}

.card-title {
  font-size: 18px;
  font-weight: 700;
  color: #191c1e;
  margin: 0 0 20px;
  line-height: 1.3;
}

.card-sub {
  font-size: 14px;
  color: #6d7a77;
  margin: -12px 0 0;
}

/* ── Add Category Card ──────────────────────────────────────────── */
.add-category-card {
  max-width: 480px;
}

.form-group {
  margin-bottom: 20px;
}

.field-label {
  display: block;
  font-size: 12px;
  font-weight: 700;
  color: #3d4947;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 6px;
}

.field-input {
  background: #f8fafc;
  border-color: #e0e3e5;
}

/* ── Color Swatches ─────────────────────────────────────────────── */
.swatch-row {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  align-items: center;
}

.swatch {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: none;
  cursor: pointer;
  transition: transform 0.15s, box-shadow 0.15s;
  padding: 0;
}
.swatch:hover {
  transform: scale(1.15);
}
.swatch.active {
  outline: 2px solid #00685f;
  outline-offset: 2px;
}

.custom-swatch {
  background: #f1f5f9;
  border: 1px dashed #bcc9c6;
  display: flex;
  align-items: center;
  justify-content: center;
}
.custom-swatch.active {
  border-style: solid;
  border-color: #00685f;
}
.custom-swatch-icon {
  font-size: 16px;
  font-weight: 700;
  color: #6d7a77;
  line-height: 1;
}

/* ── Custom color input row ─────────────────────────────────────── */
.custom-color-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 10px;
}

.native-color-input {
  width: 36px;
  height: 36px;
  border: 1px solid #e0e3e5;
  border-radius: 6px;
  padding: 2px;
  cursor: pointer;
  background: none;
}

.hex-input {
  width: 110px;
}

/* ── Category List ──────────────────────────────────────────────── */
.category-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.category-row {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 12px;
  border-radius: 6px;
  background: #f8fafc;
}

.cat-swatch {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  flex-shrink: 0;
}
.cat-swatch.sm {
  width: 14px;
  height: 14px;
}

.cat-name {
  font-weight: 600;
  font-size: 14px;
  color: #191c1e;
  flex: 1;
}

.cat-color-hex {
  font-family: "IBM Plex Mono", monospace;
  font-size: 12px;
  color: #6d7a77;
}

/* ── Bulk Assign ────────────────────────────────────────────────── */
.bulk-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-wrap: wrap;
  gap: 16px;
  margin-bottom: 20px;
}

.bulk-controls {
  display: flex;
  gap: 12px;
  align-items: center;
}

.bulk-search {
  width: 260px;
}

.bulk-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 16px;
}

.result-count {
  font-size: 13px;
  color: #6d7a77;
}

/* ── Shared ─────────────────────────────────────────────────────── */
.mono {
  font-family: "IBM Plex Mono", monospace;
  font-size: 13px;
}
</style>
