<script setup lang="ts">
import { AppEvent, Service } from "../../enum/service.ts";
import {
  FwbBadge,
  FwbButton,
  FwbListGroup,
  FwbListGroupItem,
} from "flowbite-vue";
import { FileEntry, readDir } from "@tauri-apps/api/fs";
import { useAppStore } from "../../stores/app.ts";
import { onMounted, ref } from "vue";
import { useSubmit } from "../../composables/useService.ts";
import { Terminal } from "xterm";
import "xterm/css/xterm.css";
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api";
import { Command } from "../../enum/command.ts";
import { useRouter } from "vue-router";

const appState = useAppStore();
const router = useRouter();
const existsDir = ref<FileEntry[]>([]);

const { setState, value } = useSubmit();

async function fetchRepo() {
  if (!appState.rootDir) return;

  existsDir.value = await readDir(appState.rootDir);
}

const terminal = new Terminal();

onMounted(() => {
  fetchRepo();
  terminal.open(document.getElementById("clone-terminal") as HTMLDivElement);

  appWindow.listen<{ service: Service; log: string }>(
    AppEvent.SERVICE_CLONE,
    ({ payload }) => {
      terminal.writeln(`[${payload.service}] ${payload.log}`);
    },
  );

  appWindow.listen(AppEvent.SERVICE_CLONE_FINISH, () => {
    setState(false);
  });
});

async function clone() {
  setState(true);
  await invoke(Command.CLONE_SERVICE, {
    rootDir: appState.rootDir,
    services: Object.values<string>(Service),
  });
}

async function next() {
  await router.push({ name: "setup.env" });
}
</script>

<template>
  <div class="my-4">
    <div class="flex justify-between mb-4">
      <h2 class="text-2xl">Clone Repo</h2>
      <fwb-button color="default" :disabled="value" @click="clone"
        >Clone</fwb-button
      >
    </div>
    <fwb-list-group class="w-full">
      <fwb-list-group-item v-for="service in Service">
        <template #prefix>
          <svg
            class="w-4 h-4 fill-current"
            fill="currentColor"
            viewBox="0 0 20 20"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              clip-rule="evenodd"
              d="M2 9.5A3.5 3.5 0 005.5 13H9v2.586l-1.293-1.293a1 1 0 00-1.414 1.414l3 3a1 1 0 001.414 0l3-3a1 1 0 00-1.414-1.414L11 15.586V13h2.5a4.5 4.5 0 10-.616-8.958 4.002 4.002 0 10-7.753 1.977A3.5 3.5 0 002 9.5zm9 3.5H9V8a1 1 0 012 0v5z"
              fill-rule="evenodd"
            />
          </svg> </template
        >{{ service }}
        <template #suffix>
          <fwb-badge
            v-if="existsDir.find((d) => d.name === service)"
            class="ml-auto"
            type="green"
            >Cloned</fwb-badge
          >
          <fwb-badge v-else type="pink">Missing</fwb-badge>
        </template>
      </fwb-list-group-item>
    </fwb-list-group>
    <div id="clone-terminal" class="mt-5"></div>
    <div class="my-6">
      <fwb-button color="default" @click="next">Next</fwb-button>
    </div>
  </div>
</template>

<style scoped lang="postcss"></style>
