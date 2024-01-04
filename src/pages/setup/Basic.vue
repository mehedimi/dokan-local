<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { FwbButton, FwbInput } from "flowbite-vue";
import { useAppStore } from "../../stores/app.ts";
import { setGitHubToken, setRootDir } from "../../persist-state.ts";
import { useRouter } from "vue-router";

const appState = useAppStore();
const router = useRouter();

async function browseRootFolder() {
  const dir = await open({ directory: true, multiple: false });

  if (!dir) return;

  await setRootDir(dir as string);
  appState.rootDir = dir as string;
}

async function save() {
  await setGitHubToken(appState.githubToken);
  await router.push({ name: "setup.clone" });
}
</script>

<template>
  <div class="py-6">
    <form @submit.prevent="save">
      <fwb-input
        label="Product root folder"
        :model-value="appState.rootDir"
        readonly
        placeholder="Please select your product root folder"
        size="lg"
        required
      >
        <template #suffix>
          <fwb-button @click="browseRootFolder" type="button"
            >Browse</fwb-button
          >
        </template>
      </fwb-input>

      <div class="my-5">
        <fwb-input
          label="GitHub Token"
          v-model="appState.githubToken"
          placeholder="Enter your GitHub Token"
          size="lg"
          required
        />
      </div>
      <fwb-button type="submit" color="default" size="lg"
        >Save & Next</fwb-button
      >
    </form>
  </div>
</template>

<style scoped lang="postcss"></style>
