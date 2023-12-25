import { Service } from "../enum/service.ts";

export interface StartService extends Record<string, string | number> {
  rootDir: string;
  port: number;
  service: Service;
}

export interface LogMessage {
  service: Service;
  log: string;
}

export interface ServiceStart {
  service: Service;
  p_id: number;
}

export interface ServiceStop {
  service: Service;
}
