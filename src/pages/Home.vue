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
              <fwb-button
                v-if="!runningServices.hasOwnProperty(service)"
                size="xs"
                @click.stop="startService(service)"
                color="light"
                :disabled="startServiceButton.value.value || !appState.rootDir"
                >Start
              </fwb-button>
              <template v-else>
                <fwb-badge class="!inline-block" size="xs" type="green"
                  >Running</fwb-badge
                >
                <fwb-badge class="!inline-block" size="xs"
                  >PID: {{ runningServices[service] }}</fwb-badge
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
import { LogMessage, ServiceStart, ServiceStop } from "../types/service.ts";
import { useAppStore } from "../stores/app.ts";
import { setRootDir } from "../persist-state.ts";
import { open as linkOpen } from "@tauri-apps/api/shell";
import Terminal from "../components/Terminal.vue";
import { useLogStore } from "../stores/log.ts";

const appState = useAppStore();

const runningServices = ref<{ [k: string]: number }>({});

const service = useService(appState.rootDir, runningServices);
const startServiceButton = useSubmit();
const logStore = useLogStore();

service.getRunning();

appWindow.listen<LogMessage>(AppEvent.SERVICE_LOG, (payload) => {
  const { service, log } = payload.payload;
  logStore.$state[service] = log;
});

appWindow.listen<ServiceStart>(AppEvent.SERVICE_START, (payload) => {
  const { service, p_id } = payload.payload;
  runningServices.value[service] = p_id;
});

appWindow.listen<ServiceStop>(AppEvent.SERVICE_STOP, (payload) => {
  const { service } = payload.payload;
  delete runningServices.value[service];
});

function startService(name: string) {
  if (startServiceButton.value.value) {
    return;
  }

  startServiceButton.setState(true);
  service
    .start(name)
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
  return stopService(service).then(() => {
    startService(service);
  });
}
</script>
