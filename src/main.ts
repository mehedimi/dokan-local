import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";
import { router } from "./router.ts";
import { createPinia } from "pinia";

const app = createApp(App);
const pina = createPinia();

app.use(router);
app.use(pina);

app.mount("#app");
