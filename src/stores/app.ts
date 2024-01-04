import { defineStore } from "pinia";
import { getGitHubToken, getRootDir } from "../persist-state.ts";

export const useAppStore = defineStore("app", {
  state() {
    return {
      rootDir: "",
      githubToken: "",
    };
  },

  actions: {
    async fetch(): Promise<boolean> {
      const [rootDir, githubToken] = await Promise.all([
        getRootDir(),
        getGitHubToken(),
      ]);

      this.$patch({ rootDir: rootDir ?? "", githubToken: githubToken ?? "" });

      return !(!rootDir || !githubToken);
    },
  },
});
