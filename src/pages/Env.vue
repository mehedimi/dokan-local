<template>
  <h2 class="text-2xl text-center">ENV</h2>

  <div class="grid grid-cols-2 gap-x-10 my-10">
    <div>
      <h3 class="mb-5 text-lg">Frontend ENV</h3>
      <pre class="overflow-x-auto bg-gray-100 p-5"
        >{{ "#Dashboard ENV\n\n" }}{{ dashboardEnv.join("\n")
        }}{{ "\n\n# Storefront ENV\n\n" }}{{ storeFrontEnv.join("\n") }}
      </pre>
    </div>
    <div>
      <h3 class="mb-5 text-lg">BACKEND ENV</h3>
      <pre class="overflow-x-auto bg-gray-100 p-5">{{
        backendEnv.join("\n")
      }}</pre>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Service } from "../enum/service.ts";
import { computed } from "vue";
let port = 3002;

const dashboardEnv = Object.values(Service)
  .filter((name) => name.endsWith("-service"))
  .map(
    (name) =>
      `VITE_${name
        .replace("-service", "")
        .toUpperCase()}_ENDPOINT=http://localhost:${port++}`,
  );

const storeFrontEnv = computed(() => {
  return dashboardEnv.map((item) => item.replace("VITE_", "NEXT_PUBLIC_"));
});

const backendEnv = computed(() => {
  return dashboardEnv.map((item) => item.replace("VITE_", "") + "/api");
});
</script>
