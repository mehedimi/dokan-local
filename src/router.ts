import { createRouter, createWebHashHistory } from "vue-router";

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      component: () => import("./pages/Home.vue"),
      name: "home.index",
    },
    {
      path: "/env",
      component: () => import("./pages/Env.vue"),
      name: "env.index",
    },
    {
      path: "/nginx",
      component: () => import("./pages/Nginx.vue"),
      name: "nginx.index",
    },
  ],
});
