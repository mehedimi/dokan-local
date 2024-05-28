<template>
  <h2 class="text-2xl text-center mb-2">ENV</h2>
  <fwb-accordion :open-first-item="false">
    <fwb-accordion-panel>
      <fwb-accordion-header>Config</fwb-accordion-header>
      <fwb-accordion-content class="border-b">
        <div class="grid md:grid-cols-2 gap-x-5">
          <div>
            <fieldset class="mb-5">
              <legend class="mb-3">PostgreSQL</legend>
              <fwb-input
                label="Host"
                class="mb-1"
                size="sm"
                v-model="state.pg.host"
              />
              <fwb-input
                label="Port"
                class="mb-1"
                size="sm"
                v-model="state.pg.port"
              />
              <fwb-input
                label="User"
                class="mb-1"
                size="sm"
                v-model="state.pg.user"
              />
              <fwb-input
                label="Password"
                class="mb-1"
                size="sm"
                v-model="state.pg.pass"
              />

              <fwb-input
                label="Options"
                class="mb-1"
                size="sm"
                v-model="state.pg.option"
              />
            </fieldset>
            <fieldset class="mb-5">
              <legend class="mb-3">Redis</legend>
              <fwb-input
                label="Host"
                class="mb-1"
                size="sm"
                v-model="state.redis.host"
              />
              <fwb-input
                label="Port"
                class="mb-1"
                size="sm"
                v-model="state.redis.port"
              />
              <fwb-input
                label="User"
                class="mb-1"
                size="sm"
                v-model="state.redis.user"
              />
              <fwb-input
                label="Password"
                class="mb-1"
                size="sm"
                v-model="state.redis.pass"
              />
            </fieldset>

            <!-- Others -->
            <fieldset class="mb-5">
              <legend class="mb-3">Others</legend>
              <fwb-input
                label="Marketplace ID"
                class="mb-1"
                size="sm"
                v-model="state.marketplaceId"
              />
            </fieldset>
          </div>
          <div>
            <fieldset class="mb-5">
              <legend class="mb-3">MongoDB</legend>
              <fwb-input
                label="Host"
                class="mb-1"
                size="sm"
                v-model="state.mongo.host"
              />
              <fwb-input
                label="Port"
                class="mb-1"
                size="sm"
                v-model="state.mongo.port"
              />
              <fwb-input
                label="User"
                class="mb-1"
                size="sm"
                v-model="state.mongo.user"
              />
              <fwb-input
                label="Password"
                class="mb-1"
                size="sm"
                v-model="state.mongo.pass"
              />

              <fwb-input
                label="Options"
                class="mb-1"
                size="sm"
                v-model="state.mongo.db_option"
              />
            </fieldset>
            <fieldset>
              <legend class="mb-3">RabbitMQ</legend>
              <fwb-input
                label="Host"
                class="mb-1"
                size="sm"
                v-model="state.rabbitmq.host"
              />
              <fwb-input
                label="Port"
                class="mb-1"
                size="sm"
                v-model="state.rabbitmq.port"
              />
              <fwb-input
                label="User"
                class="mb-1"
                size="sm"
                v-model="state.rabbitmq.user"
              />
              <fwb-input
                label="Password"
                class="mb-1"
                size="sm"
                v-model="state.rabbitmq.pass"
              />
            </fieldset>
          </div>
        </div>
        <fwb-button
          size="lg"
          class="mt-4 w-full"
          @click="saveEnv"
          color="default"
          >Save</fwb-button
        >
      </fwb-accordion-content>
    </fwb-accordion-panel>
  </fwb-accordion>
  <div class="grid md:grid-cols-2 gap-x-10 my-10">
    <div>
      <div class="flex justify-between">
        <h3 class="mb-5 text-lg align-middle">FRONTEND ENV</h3>
      </div>

      <!-- Dashboard ENV -->
      <div class="flex justify-between items-center mb-3">
        <h4 class="text-base">Dashboard ENV</h4>
        <fwb-button
          @click="copyDashboardEnv(dashboardEnvRef?.textContent ?? '')"
          size="xs"
          color="light"
          :disabled="copiedDashboardEnv"
          >{{ copiedDashboardEnv ? "Copied" : "Copy" }}</fwb-button
        >
      </div>
      <pre ref="dashboardEnvRef" class="overflow-x-auto bg-gray-100 p-5 mb-5"
        >{{ dashboardEnv.join("\n") }}
      </pre>

      <!-- Storefront ENV -->
      <div class="flex justify-between items-center mb-3">
        <h4 class="text-base">Storefront ENV</h4>
        <fwb-button
          @click="copyStoreFrontEnv(storeFrontEnvRef?.textContent ?? '')"
          size="xs"
          color="light"
          :disabled="copiedStoreFrontEnv"
          >{{ copiedStoreFrontEnv ? "Copied" : "Copy" }}</fwb-button
        >
      </div>
      <pre ref="storeFrontEnvRef" class="overflow-x-auto bg-gray-100 p-5"
        >{{ storeFrontEnv.join("\n") }}
      </pre>
    </div>
    <div>
      <div class="flex justify-between">
        <h3 class="mb-5 text-lg">BACKEND ENV</h3>
        <div>
          <fwb-button
            @click="copyBackendEnv(backendEnv.join('\n'))"
            size="xs"
            color="light"
            :disabled="copiedBackendEnv"
            >{{ copiedBackendEnv ? "Copied" : "Copy" }}</fwb-button
          >
        </div>
      </div>

      <pre class="overflow-x-auto bg-gray-100 p-5">{{
        backendEnv.join("\n")
      }}</pre>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref } from "vue";
import {
  FwbAccordion,
  FwbAccordionContent,
  FwbAccordionHeader,
  FwbAccordionPanel,
  FwbInput,
  FwbButton,
} from "flowbite-vue";
import { Service } from "../enum/service.ts";
import { useConfigState } from "../stores/config.ts";
import { useClipboard } from "@vueuse/core";
import { message } from "@tauri-apps/api/dialog";
import { useAppStore } from "../stores/app.ts";
import { writeTextFile } from "@tauri-apps/api/fs";

const dashboardEnvRef = ref<HTMLPreElement>();
const storeFrontEnvRef = ref<HTMLPreElement>();

const { copy: copyBackendEnv, copied: copiedBackendEnv } = useClipboard();
const { copy: copyDashboardEnv, copied: copiedDashboardEnv } = useClipboard();
const { copy: copyStoreFrontEnv, copied: copiedStoreFrontEnv } = useClipboard();

const state = useConfigState();

const port = 3002;
let contentPort: number;
const dashboardEnv = computed(() => {
  return Object.values(Service)
    .filter((name) => name.endsWith("-service"))
    .map((name, i) => {
      if (name === Service.CONTENT) {
        contentPort = port + i;
      }
      return `VITE_${name
        .replace("-service", "")
        .toUpperCase()}_ENDPOINT=http://localhost:${port + i}`;
    })
    .concat([
      `VITE_STORAGE_URL=http://localhost:${contentPort}`,
      `MARKETPLACE_ID=${state.marketplaceId}`,
    ]);
});

const storeFrontEnv = computed(() => {
  return dashboardEnv.value.map((item) =>
    item.replace("VITE_", "NEXT_PUBLIC_"),
  );
});

const backendEnv = computed(() => {
  return [
    "NODE_ENV=development",
    `MARKETPLACE_ID=${state.marketplaceId}`,
    "APP_DEBUG=true",
    "JWT_SECRET=THIS_SECRET",
    `STORAGE_URL=http://localhost:${contentPort}`,
    "DOKAN_APP_ENDPOINT=127.0.0.1:8000/api",
    "GOOGLE_MAP_API_KEY=test123",
    "GOOGLE_CLIENT_ID=test123",
    "GOOGLE_CLIENT_SECRET=test123",
    "GOOGLE_REDIRECT=",
    "WEBHOOK_APP_URL=",
  ]
    .concat(postgresEnv.value)
    .concat(mongoEnv.value)
    .concat(["\n"])
    .concat(
      dashboardEnv.value.map((item) => item.replace("VITE_", "") + "/api"),
    )
    .concat([
      "\n# Redis",
      `REDIS_HOST=${state.redis.host}`,
      `REDIS_PORT=${state.redis.port}`,
      `REDIS_USERNAME=${state.redis.user}`,
      `REDIS_PASSWORD=${state.redis.pass}`,
      `\n# RabbitMQ`,
      `RABBITMQ_HOST=${state.rabbitmq.host}`,
      `RABBITMQ_PORT=${state.rabbitmq.port}`,
      `RABBITMQ_USERNAME=${state.rabbitmq.user}`,
      `RABBITMQ_PASSWORD=${state.rabbitmq.pass}`,
      `\n# Storage`,
      `STORAGE_DRIVER=file`,
      `FILE_STORAGE_PATH=storage/uploads`,
      "\n",
      `SHIPPO_API_KEY=test123`,
      "SHIPPO_CLIENT_ID=test123",
      "SHIPPO_CLIENT_SECRET=test123",
    ]);
});

const postgresServices = [
  Service.ORDER,
  Service.AUTH,
  Service.TAX,
  Service.COUPON,
  Service.INTEGRATION,
  Service.PAYMENT,
  Service.REPORT,
  Service.SHIPPING,
  Service.USER,
];

const postgresEnv = computed(() => {
  return postgresServices.flatMap((key: string) => {
    key = key.replace("-service", "").toUpperCase();

    return [
      `\n# ${key}`,
      `${key}_DB_HOST=${state.pg.host}`,
      `${key}_DB_PORT=${state.pg.port}`,
      `${key}_DB_USER=${state.pg.user}`,
      `${key}_DB_PASS=${state.pg.pass}`,
      `${key}_DB_NAME=${key.toLowerCase()}`,
      `${key}_DB_OPTION=${state.pg.option}`,
    ];
  });
});

const mongodbServices = [Service.CART, Service.CATALOG, Service.CONTENT];

const mongoEnv = computed(() => {
  return mongodbServices.flatMap((key: string) => {
    key = key.replace("-service", "").toUpperCase();

    return [
      `\n# ${key}`,
      `${key}_NOSQL_DB_HOST=${state.mongo.host}`,
      `${key}_NOSQL_DB_PORT=${state.mongo.port}`,
      `${key}_NOSQL_DB_USER=${state.mongo.user}`,
      `${key}_NOSQL_DB_NAME=${key.toLowerCase()}`,
      `${key}_NOSQL_DB_PASS=${state.mongo.pass}`,
      `${key}_NOSQL_DB_OPTION=${state.mongo.db_option || ""}`,
    ];
  });
});

const appStore = useAppStore();

async function saveEnv() {
  await writeTextFile(
    `${appStore.rootDir}/main.env`,
    `${backendEnv.value.join("\n")}

# Storefront
${storeFrontEnv.value
  .filter((item) => !item.startsWith("MARKETPLACE_ID"))
  .join("\n")}

# Dashboard
${dashboardEnv.value
  .filter((item) => !item.startsWith("MARKETPLACE_ID"))
  .join("\n")}`,
  );
  return message("Env file saved", { type: "info", title: "Info" });
}
</script>
