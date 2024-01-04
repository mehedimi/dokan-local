<script setup lang="ts">
import { FwbNavbar, FwbNavbarCollapse } from "flowbite-vue";
import { open } from "@tauri-apps/api/shell";
import { useAppStore } from "./stores/app.ts";
import { useRoute, useRouter } from "vue-router";
import { onMounted } from "vue";

const router = useRouter();
const route = useRoute();

const appState = useAppStore();

onMounted(() => {
  (async () => {
    if (
      !(await appState.fetch()) &&
      !(route.name as string | undefined)?.startsWith("setup")
    ) {
      await router.push({
        name: "setup.basic",
      });
    }
  })();
});
</script>

<template>
  <div class="container mx-auto">
    <fwb-navbar class="my-4">
      <template #logo>
        <h2 class="text-3xl">Dokan Local</h2>
      </template>
      <template #default="{ isShowMenu }">
        <fwb-navbar-collapse :is-show-menu="isShowMenu">
          <router-link
            class="block py-2 pr-4 pl-3 rounded md:p-0 text-gray-700 hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
            to="/"
          >
            Services
          </router-link>
          <router-link
            class="block py-2 pr-4 pl-3 rounded md:p-0 text-gray-700 hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
            :to="{ name: 'env.index' }"
          >
            ENV
          </router-link>
          <router-link
            active-class="router-link-exact-active"
            class="block py-2 pr-4 pl-3 rounded md:p-0 text-gray-700 hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
            :to="{ name: 'setup.basic' }"
          >
            Setup
          </router-link>
        </fwb-navbar-collapse>
      </template>
    </fwb-navbar>
    <router-view v-slot="{ Component }">
      <keep-alive>
        <component :is="Component" />
      </keep-alive>
    </router-view>
  </div>
  <p class="text-center py-6 bg-gray-200">
    Made by
    <a
      class="text-blue-500"
      @click.prevent="open('https://github.com/mehedimi')"
      href="#"
      >Mehedi Hasan</a
    >
  </p>
</template>

<style lang="postcss">
.router-link-exact-active {
  @apply !text-blue-600;
}
</style>
