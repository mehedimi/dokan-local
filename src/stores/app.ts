import { defineStore } from "pinia";

export const useAppStore = defineStore<"app", { rootDir: string }>("app", {
  state() {
    return {
      rootDir: "",
    };
  },
});
