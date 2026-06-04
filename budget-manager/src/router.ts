import { createRouter, createWebHistory } from "vue-router";
import DashboardPage from "./pages/DashboardPage.vue";
import CategoriesPage from "./pages/CategoriesPage.vue";
import SettingsPage from "./pages/SettingsPage.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", name: "dashboard", component: DashboardPage },
    { path: "/categories", name: "categories", component: CategoriesPage },
    { path: "/settings", name: "settings", component: SettingsPage },
  ],
});

export default router;
