import { defineStore } from "pinia";
import { Service } from "../enum/service.ts";

export const useLogStore = defineStore<"log", Record<Service, string>>("log", {
  state() {
    return {
      [Service.AUTH]: "",
      [Service.TAX]: "",
      [Service.CART]: "",
      [Service.CONTENT]: "",
      [Service.USER]: "",
      [Service.CATALOG]: "",
      [Service.SHIPPING]: "",
      [Service.REPORT]: "",
      [Service.INTEGRATION]: "",
      [Service.COUPON]: "",
      [Service.ORDER]: "",
      [Service.NOTIFICATION]: "",
      [Service.STOREFRONT]: "",
      [Service.DASHBOARD]: "",
      [Service.PAYMENT]: "",
    };
  },
});
