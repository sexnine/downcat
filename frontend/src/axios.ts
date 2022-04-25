import axiosLib from "axios";
import { watch } from "vue";
import router from "./router";
import { store } from "./store";

const baseURL = import.meta.env.DEV
  ? (import.meta.env.VITE_API_BASE as string)
  : "/api";

const axios = axiosLib.create({
  baseURL: baseURL,
  headers: {
    "X-Api-Key": store.token,
  },
});

axios.interceptors.response.use(
  (res) => res,
  (err) => {
    if (err.response && err.response.status == 401) {
      router.replace("/auth");
    }
    return Promise.reject(err);
  }
);

watch(store, (newVal) => {
  console.log(`Store update`);
  console.log(newVal);
  // @ts-ignore
  axios.defaults.headers["X-Api-Key"] = newVal.token;
});

export { axios, baseURL };
