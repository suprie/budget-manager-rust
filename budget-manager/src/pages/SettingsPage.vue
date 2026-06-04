<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";

const router = useRouter();

async function resetDB() {
  try {
    await invoke("reset_db");
    router.push("/");
  } catch (error) {
    console.error(error);
  }
}
</script>

<template>
  <div>
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

.settings-section {
  margin-top: 8px;
}

.card {
  background: white;
  border: 1px solid #e0e3e5;
  border-radius: 8px;
  padding: 24px;
  max-width: 480px;
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

.card-description {
  font-size: 14px;
  color: #6d7a77;
  margin: 0 0 16px;
}
</style>
