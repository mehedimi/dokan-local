<template>
  <div class="my-6">
    <fwb-input
      :model-value="rootDir"
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
        <fwb-accordion-header
          >{{ service }}
          <fwb-button
            v-if="!runningServices.hasOwnProperty(service)"
            size="xs"
            @click.stop="startService(service)"
            color="light"
            >Start</fwb-button
          >
          <template v-else>
            <fwb-badge class="!inline-block" size="xs" type="green"
              >Running</fwb-badge
            >
            <fwb-badge class="!inline-block" size="xs"
              >PID: {{ runningServices[service] }}</fwb-badge
            >
            <fwb-button size="xs" @click.stop="stopService(service)" color="red"
              >Stop</fwb-button
            >
          </template>
        </fwb-accordion-header>
        <fwb-accordion-content>
          <div
            :id="service"
            class="font-fira-code text-sm h-72 overflow-x-auto"
          >
            <p v-for="log in logs[service]">{{ log }}</p>
          </div>
        </fwb-accordion-content>
      </fwb-accordion-panel>
    </fwb-accordion>
  </div>
</template>

<script setup lang="ts">
import {
  FwbAccordion,
  FwbAccordionContent,
  FwbAccordionHeader,
  FwbAccordionPanel,
  FwbButton,
  FwbInput,
  FwbBadge,
} from "flowbite-vue";
import { AppEvent, Service } from "../enum/service.ts";
import { nextTick, reactive, Ref, ref } from "vue";
import { message, open } from "@tauri-apps/api/dialog";
import { useService } from "../composables/useService.ts";
import { appWindow } from "@tauri-apps/api/window";
import { LogMessage, ServiceStart, ServiceStop } from "../types/service.ts";

const rootDir = ref("/Users/mehedi/Dokan-SAAS");

const logs = reactive(
  Object.values<Service>(Service).reduce<{ [key: string]: Ref<string[]> }>(
    (carry, current) => {
      carry[current] = ref([]);

      return carry;
    },
    {},
  ),
);

const runningServices = ref<{ [k: string]: number }>({});

const service = useService(rootDir.value, runningServices);

service.getRunning();

appWindow.listen<LogMessage>(AppEvent.SERVICE_LOG, (payload) => {
  const { service, log } = payload.payload;
  (logs[service] as unknown as string[]).push(log);

  nextTick(() => {
    const el = document.getElementById(service) as HTMLDivElement;
    el.scroll({
      top: el.clientHeight + el.scrollTop,
      behavior: "smooth",
    });
  });
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
  service.start(name).catch(async (err: string) => {
    await message(err, { type: "error" });
  });
}

async function stopService(name: string) {
  return service.stop(name).catch(async (msg) => {
    await message(msg, { type: "error" });
  }).then((log) => {
    console.log(log)
  });
}

async function browseRootFolder() {
  const dir = await open({ directory: true, multiple: false });

  rootDir.value = dir as string;
}
</script>
