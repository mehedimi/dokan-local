import { Service } from "../enum/service";

export type ServiceConfig = {
  name: Service;
  repoUrl: string;
};

export const services: ServiceConfig[] = [
  {
    name: Service.DASHBOARD,
    repoUrl: "https://github.com/getdokan/dashboard",
  },
  {
    name: Service.STOREFRONT,
    repoUrl: "https://github.com/getdokan/storefront",
  },
  {
    name: Service.AUTH,
    repoUrl: "https://github.com/getdokan/auth-service",
  },
  {
    name: Service.USER,
    repoUrl: "https://github.com/getdokan/user-service",
  },
  {
    name: Service.CART,
    repoUrl: "https://github.com/getdokan/cart-service",
  },
  {
    name: Service.CATALOG,
    repoUrl: "https://github.com/getdokan/catalog-service",
  },
  {
    name: Service.ORDER,
    repoUrl: "https://github.com/getdokan/order-service",
  },
  {
    name: Service.PAYMENT,
    repoUrl: "https://github.com/getdokan/payment-service",
  },
  {
    name: Service.TAX,
    repoUrl: "https://github.com/getdokan/tax-service",
  },
  {
    name: Service.NOTIFICATION,
    repoUrl: "https://github.com/getdokan/notification-service",
  },
  {
    name: Service.CONTENT,
    repoUrl: "https://github.com/getdokan/content-service",
  },
  {
    name: Service.INTEGRATION,
    repoUrl: "https://github.com/getdokan/integration-service",
  },
  {
    name: Service.REPORT,
    repoUrl: "https://github.com/getdokan/report-service",
  },
  {
    name: Service.COUPON,
    repoUrl: "https://github.com/getdokan/coupon-service",
  },
  {
    name: Service.SHIPPING,
    repoUrl: "https://github.com/getdokan/shipping-service",
  },
];
