export enum Service {
  DASHBOARD = "dashboard",
  STOREFRONT = "storefront",
  AUTH = "auth-service",
  USER = "user-service",
  ORDER = "order-service",
  PAYMENT = "payment-service",
  CONTENT = "content-service",
  INTEGRATION = "integration-service",
  REPORT = "report-service",
  COUPON = "coupon-service",
  SHIPPING = "shipping-service",
  DOKAN_CLOUD = "dokan-cloud",
  ACTIVITY = "activity-service",
}

let initPort = 3000;

export const ports: Record<string, number> = Object.values<string>(
  Service,
).reduce<Record<string, number>>((carry, current) => {
  carry[current] = initPort++;
  return carry;
}, {});

export enum AppEvent {
  SERVICE_LOG = "service:logs",
  SERVICE_START = "service:start",
  SERVICE_STOP = "service:stop",
}
