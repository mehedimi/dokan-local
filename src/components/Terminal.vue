<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { Terminal } from "xterm";
import { Service } from "../enum/service.ts";
import { useLogStore } from "../stores/log.ts";
import { FitAddon } from "xterm-addon-fit";
import { useDebounceFn } from "@vueuse/core";

const logStore = useLogStore();
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
});

watch(
  () => logStore.$state[props.service],
  (val) => {
    terminal.writeln(val);
  },
);
</script>

<template>
  <div ref="el"></div>
</template>
