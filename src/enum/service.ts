export enum Service {
  DASHBOARD = "dashboard",
  STOREFRONT = "storefront",
  AUTH_SERVICE = "auth-service",
  USER_SERVICE = "user-service",
  CART_SERVICE = "cart-service",
  CATALOG_SERVICE = "catalog-service",
  ORDER_SERVICE = "order-service",
  PAYMENT = "payment-service",
  TAX = "tax-service",
  NOTIFICATION = "notification-service",
  CONTENT = "content-service",
  INTEGRATION = "integration-service",
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
