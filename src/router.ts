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
      path: "/setup",
      component: () => import("./pages/setup/Index.vue"),
      name: "setup.index",
      children: [
        {
          path: "",
          component: () => import("./pages/setup/Basic.vue"),
          name: "setup.basic",
        },
        {
          path: "clone",
          component: () => import("./pages/setup/Clone.vue"),
          name: "setup.clone",
        },
        {
          path: "env",
          component: () => import("./pages/setup/Env.vue"),
          name: "setup.env",
        },
      ],
    },
  ],
});
