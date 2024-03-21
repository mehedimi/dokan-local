<template>
  <div class="my-6">
    <fwb-input
      :model-value="appState.rootDir"
      disabled
      placeholder="Please select your product root folder"
      size="lg"
    >
      <template #suffix>
        <fwb-button @click="browseRootFolder">Browse</fwb-button>
      </template>
    </fwb-input>

    <fwb-accordion open-first-item always-open class="mt-6">
      <fwb-accordion-panel v-for="service in Object.values(Service)">
        <fwb-accordion-header class="[&>button]:py-3">
          <div class="flex items-center">
            <div>
              {{ service }}:{{ ports[service] }}
              <template v-if="!runningServices.hasOwnProperty(service)">
                <fwb-button
                  size="xs"
                  @click.stop="startService(service, false)"
                  color="light"
                  :disabled="
                    startServiceButton.value.value || !appState.rootDir
                  "
                  >Start
                </fwb-button>
                <fwb-button
                  size="xs"
                  @click.stop="startService(service, true)"
                  color="light"
                  :disabled="
                    startServiceButton.value.value || !appState.rootDir
                  "
                  class="ml-2"
                  >Dev
                </fwb-button>
                <fwb-button
                  size="xs"
                  @click.stop="buildService(service)"
                  color="light"
                  :disabled="builds.includes(service)"
                  class="ml-2"
                  >{{
                    builds.includes(service as Service) ? "Building" : "Build"
                  }}
                </fwb-button>
              </template>

              <template v-else>
                <fwb-badge class="!inline-block" size="xs" type="green"
                  >Running</fwb-badge
                >
                <fwb-badge class="!inline-block" size="xs"
                  >PID:{{ runningServices[service].pid }}:{{
                    runningServices[service].is_dev ? "dev" : "start"
                  }}</fwb-badge
                >
                <fwb-button
                  class="!inline-block"
                  size="xs"
                  @click.stop="stopService(service)"
                  color="red"
                  >Stop</fwb-button
                >

                <fwb-button
                  size="xs"
                  @click.stop="restartService(service)"
                  color="yellow"
                  class="ml-3"
                  >Restart</fwb-button
                >
              </template>
              <fwb-button
                size="xs"
                @click.stop="gitPull(service)"
                color="alternative"
                class="ml-3"
                :disabled="pulls.includes(service)"
                >{{
                  pulls.includes(service as Service) ? "Pulling" : "Pull"
                }}</fwb-button
              >
            </div>

            <button
              @click.stop="linkOpen(`https://github.com/getdokan/${service}`)"
              class="inline-block w-6 ml-auto mr-2"
            >
              <github-icon />
            </button>
          </div>
        </fwb-accordion-header>
        <fwb-accordion-content
          class="font-fira-code bg-gray-100 overflow-x-auto"
        >
          <Terminal ref="terminals" :service="service" />
        </fwb-accordion-content>
      </fwb-accordion-panel>
    </fwb-accordion>
  </div>
</template>

<script setup lang="ts">
import GithubIcon from "../components/icons/GithubIcon.vue";
import {
  FwbAccordion,
  FwbAccordionContent,
  FwbAccordionHeader,
  FwbAccordionPanel,
  FwbButton,
  FwbInput,
  FwbBadge,
} from "flowbite-vue";
import "xterm/css/xterm.css";
import { AppEvent, ports, Service } from "../enum/service.ts";
import { ref } from "vue";
import { message, open } from "@tauri-apps/api/dialog";
import { useService, useSubmit } from "../composables/useService.ts";
import { appWindow } from "@tauri-apps/api/window";
import { ServiceStart, ServiceStop } from "../types/service.ts";
import { useAppStore } from "../stores/app.ts";
import { setRootDir } from "../persist-state.ts";
import { open as linkOpen } from "@tauri-apps/api/shell";
import Terminal from "../components/Terminal.vue";

const appState = useAppStore();

const runningServices = ref<{ [k: string]: { pid: number; is_dev: boolean } }>(
  {},
);

const service = useService(appState.rootDir, runningServices);
const startServiceButton = useSubmit();
const pulls = ref<Service[]>([]);
const builds = ref<Service[]>([]);

service.getRunning();

appWindow.listen<ServiceStart>(AppEvent.SERVICE_START, (payload) => {
  const { service, p_id, is_dev } = payload.payload;
  runningServices.value[service] = { pid: p_id, is_dev };
});

appWindow.listen<ServiceStop>(AppEvent.SERVICE_STOP, (payload) => {
  const { service } = payload.payload;
  delete runningServices.value[service];
});

function startService(name: string, isDev: boolean) {
  if (startServiceButton.value.value) {
    return;
  }

  startServiceButton.setState(true);
  service
    .start(name, isDev)
    .then((res) => {
      console.log(res);
    })
    .catch((err: string) => {
      message(err, { type: "error" });
    })
    .finally(() => {
      startServiceButton.setState(false);
    });
}

async function buildService(name: Service) {
  builds.value.push(name);
  console.log(builds);

  service
    .build(name)
    .catch((err) => {
      message(err, { type: "error" });
    })
    .finally(() => {
      const i = builds.value.findIndex((b) => name === b);
      if (i === -1) return;
      builds.value.splice(i, 1);
    });
}

async function stopService(name: string) {
  return service.stop(name).catch(async (msg) => {
    await message(msg, { type: "error" });
  });
}

async function browseRootFolder() {
  const dir = await open({ directory: true, multiple: false });

  if (!dir) return;

  await setRootDir(dir as string);
  appState.rootDir = dir as string;
}

async function restartService(service: Service) {
  const se = runningServices.value[service];

  if (!se) return;

  return stopService(service).then(() => {
    startService(service, se.is_dev);
  });
}

async function gitPull(name: Service) {
  pulls.value.push(name);
  return service
    .pull(name, appState.rootDir as string)
    .then(() => {})
    .catch(() => {})
    .finally(() => {
      const i = pulls.value.findIndex((p) => name === p);
      if (i === -1) return;
      pulls.value.splice(i, 1);
    });
}
</script>
