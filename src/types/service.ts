import { Service } from "../enum/service.ts";

export interface StartService
  extends Record<string, string | number | boolean> {
  rootDir: string;
  port: number;
  service: Service;
  isDev: boolean
}

export interface LogMessage {
  service: Service;
  log: string;
}

export interface ServiceStart {
  service: Service;
  p_id: number;
  is_dev: boolean
}

export interface ServiceStop {
  service: Service;
}
