import { StartService } from "../types/service.ts";
import { invoke } from "@tauri-apps/api";
import { Command } from "../enum/command.ts";
import { ports } from "../enum/service.ts";
import { ref, Ref } from "vue";

export function useService(
  rootDir: string,
  running: Ref<{ [k: string]: number }>,
) {
  function start(name: string) {
    const port = ports[name];

    return invoke(Command.START_SERVICE, {
      rootDir,
      port,
      service: name,
    } as StartService);
  }

  async function getRunning() {
    return invoke<{ [k: string]: number }>(Command.RUNNING_SERVICE).then(
      (data) => {
        running.value = data;
      },
    );
  }

  async function stop(name: string) {
    return invoke<string>(Command.STOP_SERVICE, { service: name });
  }

  async function pull(service: string, rootDir: string) {
    return invoke<string>(Command.GIT_PULL, { service, rootDir });
  }

  return { start, getRunning, stop, pull };
}

export function useSubmit() {
  const value = ref(false);

  function setState(state: boolean) {
    value.value = state;
  }

  return { value, setState };
}
