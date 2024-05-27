import { defineStore } from "pinia";
import { persistState } from "../persist-state.ts";
import { State } from "../enum/state.ts";

export const useConfigState = defineStore("config", {
  state() {
    return {
      pg: {
        host: "127.0.0.1",
        user: "dokan_app",
        pass: "dokan_app",
        port: "5678",
        option: "",
      },
      mongo: {
        host: "127.0.0.1",
        user: "",
        pass: "",
        port: "27017",
        db_option: "",
      },
      rabbitmq: {
        host: "127.0.0.1",
        port: "5672",
        user: "guest",
        pass: "guest",
      },
      redis: {
        host: "127.0.0.1",
        port: "6379",
        user: "",
        pass: "",
      },
      others: {
        marketplaceId: "",
      },
    };
  },
  actions: {
    async save() {
      return persistState.set(State.CONFIG, this.$state);
    },
    async get() {
      const config = await persistState.get<typeof this.$state>(State.CONFIG);
      if (config) {
        this.$patch(config);
      }
    },
  },
});
