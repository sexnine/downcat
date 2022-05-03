<template>
  <div class="flex h-screen items-center justify-center">
    <div
      class="relative flex flex-col items-center rounded-lg bg-gray-800 py-4 px-16"
    >
      <loading-screen :loading="loading" />
      <img :src="downcatImg" />
      <h1 class="mb-2 text-2xl font-semibold">Downcat</h1>
      <div v-if="errorMsg" class="mb-2 flex items-center gap-x-1 text-red-500">
        <fa-icon :icon="['fas', 'circle-exclamation']" />
        <p>
          {{ errorMsg }}
        </p>
      </div>
      <input
        ref="passwordFieldEl"
        v-model="passwordField"
        class="mb-4 rounded-md bg-gray-900 px-2 py-1 outline-0 ring-red-500 transition focus:ring-2 focus:drop-shadow-md"
        autofocus
        type="password"
        placeholder="Password"
        @keyup.enter="signInButton"
      />
      <button
        class="transform-gpu rounded-md bg-red-500 px-2 py-1 text-lg font-semibold transition hover:-translate-y-1 hover:drop-shadow-lg"
        @click="signInButton"
      >
        Sign in
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
  import downcatImg from "../assets/downcat-sm.png";
  import { ref } from "vue";
  import LoadingScreen from "../components/LoadingScreen.vue";
  import { axios } from "../axios";
  import { store } from "../store";
  import router from "../router";
  import { onStartTyping } from "@vueuse/core";

  const loading = ref(false);
  const errorMsg = ref("");

  const passwordField = ref("");
  const passwordFieldEl = ref<HTMLInputElement>();

  onStartTyping(() => passwordFieldEl.value?.focus());

  const signInButton = () => {
    errorMsg.value = "";
    if (passwordField.value == "") {
      errorMsg.value = "Input a password";
      return;
    }

    loading.value = true;
    axios
      .post("/auth", { password: passwordField.value })
      .then((res) => {
        store.token = res.data.key;
        router.replace("/");
      })
      .catch((err) => {
        errorMsg.value = err.response.data.error;
        loading.value = false;
      });
  };
</script>
