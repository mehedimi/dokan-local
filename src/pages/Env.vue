<template>
  <h2 class="text-2xl text-center">ENV</h2>
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
            <fieldset>
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
      </fwb-accordion-content>
    </fwb-accordion-panel>
  </fwb-accordion>
  <div class="grid md:grid-cols-2 gap-x-10 my-10">
    <div>
      <div class="flex justify-between my-2">
        <h3 class="mb-5 text-lg align-middle">FRONTEND ENV</h3>
      </div>

      <pre class="overflow-x-auto bg-gray-100 p-5"
        >{{ "#Dashboard ENV\n\n" }}{{ "MARKETPLACE_ID=\n"
        }}{{ "VITE_STORAGE_URL=http://localhost:3010\n"
        }}{{ dashboardEnv.join("\n") }}
        {{ "\n\n# Storefront ENV\n\n" }}{{ "MARKETPLACE_ID=\n"
        }}{{ storeFrontEnv.join("\n") }}
      </pre>
    </div>
    <div>
      <div class="flex justify-between">
        <h3 class="mb-5 text-lg">BACKEND ENV</h3>
        <div>
          <fwb-button
            @click="copy(backendEnv.join('\n'))"
            size="xs"
            color="light"
            :disabled="copied"
            >{{ copied ? "Copied" : "Copy" }}</fwb-button
          >
          <fwb-button size="xs" class="ml-2" @click="saveEnv" color="default"
            >Save</fwb-button
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
import { computed } from "vue";
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

const { copy, copied } = useClipboard();

let port = 3002;
const state = useConfigState();

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
  return [
    "NODE_ENV=development",
    "MARKETPLACE_ID=",
    "APP_DEBUG=true",
    "JWT_SECRET=THIS_SECRET",
    "STORAGE_URL=http://localhost:3010",
    "DOKAN_APP_ENDPOINT=127.0.0.1:8000",
    "GOOGLE_MAP_API_KEY=",
    "GOOGLE_CLIENT_ID=",
    "GOOGLE_CLIENT_SECRET=",
    "GOOGLE_REDIRECT=",
  ]
    .concat(postgresEnv.value)
    .concat(mongoEnv.value)
    .concat(["\n"])
    .concat(dashboardEnv.map((item) => item.replace("VITE_", "") + "/api"))
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
    backendEnv.value.join("\n"),
  );
  return message("Env file saved", { type: "info", title: "Info" });
}
</script>
