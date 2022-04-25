import { createApp } from "vue";
import App from "./App.vue";
import "./assets/main.css";
import router from "./router";
// import i18n from "./i18n";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import library from "./fontAwesome";

library;

const app = createApp(App);

app.use(router);
// app.use(i18n);
app.component("fa-icon", FontAwesomeIcon);

app.mount("#app");
