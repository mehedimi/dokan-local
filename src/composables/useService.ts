import { StartService } from "../types/service.ts";
import { invoke } from "@tauri-apps/api";
import { Command } from "../enum/command.ts";
import { ports } from "../enum/service.ts";
import { ref, Ref } from "vue";

export function useService(
  rootDir: string,
  running: Ref<{ [k: string]: { pid: number; is_dev: boolean } }>,
) {
  function start(name: string, isDev: boolean) {
    const port = ports[name];

    return invoke(Command.START_SERVICE, {
      rootDir,
      port,
      service: name,
      isDev,
    } as StartService);
  }

  async function build(name: string) {
    return invoke(Command.BUILD_SERVICE, {
      rootDir,
      service: name,
    })
  }

  async function getRunning() {
    return invoke<{ [k: string]: { pid: number; is_dev: boolean } }>(Command.RUNNING_SERVICE).then(
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

  return { start, getRunning, stop, pull, build };
}

export function useSubmit() {
  const value = ref(false);

  function setState(state: boolean) {
    value.value = state;
  }

  return { value, setState };
}
