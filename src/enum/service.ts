export enum Service {
  DASHBOARD = "dashboard",
  STOREFRONT = "storefront",
  AUTH = "auth-service",
  USER = "user-service",
  CART = "cart-service",
  CATALOG = "catalog-service",
  ORDER = "order-service",
  PAYMENT = "payment-service",
  TAX = "tax-service",
  NOTIFICATION = "notification-service",
  CONTENT = "content-service",
  INTEGRATION = "integration-service",
  REPORT = "report-service",
  COUPON = "coupon-service",
  SSHIPPING = "shipping-service",
  ACTIVITY = "activity-service"
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
