<script setup lang="ts">
import EnvPage from "../Env.vue";
import { FwbButton } from "flowbite-vue";

async function next() {}
</script>

<template>
  <EnvPage></EnvPage>
  <div class="my-6">
    <fwb-button color="default" @click="next">Next</fwb-button>
  </div>
</template>

<style scoped lang="postcss"></style>
