import { Store } from "tauri-plugin-store-api";
import { State } from "./enum/state.ts";

export const persistState = new Store("dokan-local.dat");

export function getRootDir() {
  return persistState.get<string>(State.ROOT_DIR);
}

export function setRootDir(dir: string) {
  return persistState.set(State.ROOT_DIR, dir);
}

export function setGitHubToken(token: string) {
  return persistState.set(State.GH_TOKEN, token);
}

export function getGitHubToken() {
  return persistState.get<string>(State.GH_TOKEN);
}
