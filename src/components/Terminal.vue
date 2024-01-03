<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { Terminal } from "xterm";
import { Service } from "../enum/service.ts";
import { useLogStore } from "../stores/log.ts";

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
  });

  terminal.open(el.value as HTMLDivElement);
});

watch(
  () => logStore.$state[props.service],
  (val) => {
    terminal.writeln(val);
  },
  { deep: true },
);
</script>

<template>
  <div ref="el"></div>
</template>

<style scoped lang="postcss"></style>
