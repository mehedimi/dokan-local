<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Terminal } from "xterm";
import { AppEvent, Service } from "../enum/service.ts";
import { FitAddon } from "xterm-addon-fit";
import { useDebounceFn } from "@vueuse/core";
import { appWindow } from "@tauri-apps/api/window";
import { LogMessage } from "../types/service.ts";

const props = defineProps<{ service: Service }>();
const el = ref<HTMLDivElement>();
let terminal: Terminal;

onMounted(() => {
  terminal = new Terminal({
    theme: {
      background: "rgb(243 244 246)",
      foreground: "#1e293b",
      cursor: "#1e293b",
      selectionBackground: "#ddd",
    },
    cols: 150,
  });

  const fitAddon = new FitAddon();

  terminal.loadAddon(fitAddon);
  terminal.open(el.value as HTMLDivElement);

  fitAddon.fit();

  window.addEventListener(
    "resize",
    useDebounceFn(() => {
      fitAddon.fit();
    }, 100),
  );

  appWindow.listen<LogMessage>(AppEvent.SERVICE_LOG, (payload) => {
    const { service, log } = payload.payload;

    if (service === props.service) {
      terminal.writeln(log);
    }
  });
});
</script>

<template>
  <div ref="el"></div>
</template>
