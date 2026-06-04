<script setup lang="ts">
import { computed } from "vue";
import { useRouter, useRoute } from "vue-router";

const router = useRouter();
const route = useRoute();

const selectedKeys = computed(() => {
  const key = route.name as string;
  return [key];
});

function navigate(info: { key: string }) {
  router.push({ name: info.key });
}
</script>

<template>
  <a-layout class="layout">
    <!-- Sidebar -->
    <a-layout-sider width="240" class="sider">
      <div class="sider-brand">
        <h1 class="sider-title">Budget Manager</h1>
        <p class="sider-subtitle">Personal Finance</p>
      </div>

      <a-menu
        :selectedKeys="selectedKeys"
        mode="inline"
        class="sider-menu"
        @click="navigate"
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
        <router-view />
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>

<style scoped>
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

.ant-layout {
  background: #f7f9fb !important;
}

.ant-layout-sider {
  background: #f7f9fb !important;
}
</style>
