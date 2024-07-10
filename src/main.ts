import { createApp } from "vue";
import App from "./App.vue";

import './style.css'
import './assets/css/tailwind.css'
import router from "@/router";
import i18n from "@/locales";
const app = createApp(App)
app.use(router).use(i18n)

app.mount("#app");
