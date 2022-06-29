<template>
  <div class="relative flex min-h-screen flex-col">
    <error-screen
      :error="error"
      button-text="Retry"
      class="z-[70] justify-center"
      @button-clicked="getServerInfo"
    />
    <div class="flex w-full items-end p-2 shadow-md">
      <img :src="downcatImg" draggable="false" class="select-none" />
      <div class="w-full pr-2">
        <p class="mb-2 mt-2 text-lg font-semibold">
          Downcat
          <span
            class="rounded-md bg-red-500 px-1 text-base font-medium text-white"
            >v{{ downcat_version }}</span
          >
        </p>
        <div
          class="relative mb-4 flex h-min w-fit items-center gap-x-2 rounded-lg bg-gray-800 px-4 py-2"
        >
          <loading-screen :loading="path == null" />
          <fa-icon :icon="['fas', 'folder']" class="text-lg" />
          <p>{{ path || "####################################" }}</p>
        </div>
        <div class="mb-2 flex items-center gap-2">
          <fa-icon
            :icon="['fas', 'arrow-left']"
            class="aspect-square text-2xl"
            :class="cantGoBack ? 'text-gray-500' : 'cursor-pointer text-white'"
            @click="goBack"
          />
          <fa-icon
            :icon="['fas', 'arrow-right']"
            class="aspect-square text-2xl"
            :class="
              cantGoForward ? 'text-gray-500' : 'cursor-pointer text-white'
            "
            @click="goForward"
          />
          <fa-icon
            ref="refreshButton"
            :icon="['fas', 'arrow-rotate-right']"
            class="cursor-pointer text-2xl"
            @click="refresh"
          />
          <fa-icon
            :icon="['fas', 'house']"
            class="aspect-square cursor-pointer text-2xl"
            @click="switchToHomeDir"
          />
          <upload-btn :path="path" @upload-complete="refresh" />
          <div class="absolute right-0 flex items-center pr-4">
            <div class="mr-2 flex h-3 w-3">
              <span
                class="absolute inline-flex h-3 w-3 animate-ping rounded-full opacity-75"
                :class="connected ? 'bg-green-400' : 'bg-red-400'"
              ></span>
              <span
                class="relative inline-flex h-3 w-3 rounded-full"
                :class="connected ? 'bg-green-500' : 'bg-red-500'"
              ></span>
            </div>
            <p>{{ connected ? "Connected" : "Disconnected" }}</p>
          </div>
        </div>
        <div class="mb-2 flex items-center justify-end gap-2">
          <input
            v-model="searchInput"
            placeholder="Search..."
            class="rounded-md bg-gray-800 px-2 py-1 shadow-md outline-0 ring-red-500 transition focus:ring-2"
          />
        </div>
      </div>
      <div class="relative grow"></div>
    </div>
    <file-table
      ref="filesTable"
      :files="files"
      :loading="filesLoading"
      :search-input-raw="searchInput"
      @switch-dir="switchDir"
      @err-btn-clicked="refresh"
    />
  </div>
</template>

<script setup lang="ts">
  import { onMounted, ref, computed, onUnmounted } from "vue";
  import LoadingScreen from "../components/LoadingScreen.vue";
  import FileTable from "../components/FileTable.vue";
  import { axios } from "../axios";
  import downcatImg from "../assets/downcat-sm.png";
  import ErrorScreen from "../components/ErrorScreen.vue";
  import UploadBtn from "../components/UploadBtn.vue";

  const filesLoading = ref(true);
  const files = ref([]);
  const searchInput = ref<string>("");
  const homePath = ref<string | null>(null);
  const path = ref<string | null>(null);
  const pathHistory = ref<string[]>([]);
  const historyIndex = ref(-1);
  const error = ref("");
  const connected = ref(false);
  const downcat_version = import.meta.env.VITE_DOWNCAT_VERSION as string;

  const filesTable = ref<InstanceType<typeof FileTable> | null>(null);

  const switchDir = (ipath: string) => {
    pathHistory.value.splice(historyIndex.value + 1);
    historyIndex.value++;
    pathHistory.value.push(ipath);
    switchDirWithoutHistory(ipath);
  };

  const switchDirWithoutHistory = (ipath: string) => {
    if (ipath == path.value) return;
    // @ts-ignore
    filesTable.value?.removeError();
    // @ts-ignore
    filesTable.value?.setLoading(true);
    console.log(`✨ Switching dir: ` + ipath);
    path.value = ipath;
    getFiles();
  };

  const refresh = () => {
    spinRotateButton();
    // @ts-ignore
    filesTable.value?.removeError();
    // @ts-ignore
    filesTable.value?.setLoading(true);
    getFiles();
  };
  const switchToHomeDir = () => {
    // if (path.value == homePath.value) return
    historyIndex.value = -1;
    pathHistory.value = [];
    if (homePath.value == null) return console.error("Home path not set");
    switchDirWithoutHistory(homePath.value);
  };

  const cantGoBack = computed(() => historyIndex.value < 0);
  const goBack = () => {
    if (cantGoBack.value) return;
    historyIndex.value--;
    if (historyIndex.value > -1) {
      switchDirWithoutHistory(pathHistory.value[historyIndex.value]);
      return;
    }
    if (homePath.value == null) return console.error("Home path not set");
    switchDirWithoutHistory(homePath.value);
  };
  const cantGoForward = computed(
    () => historyIndex.value >= pathHistory.value.length - 1
  );
  const goForward = () => {
    if (cantGoForward.value) return;
    historyIndex.value++;
    switchDirWithoutHistory(pathHistory.value[historyIndex.value]);
  };

  const refreshButton = ref<any | null>(null); // TODO: Figure out the type for this lol
  const spinRotateButton = () => {
    refreshButton.value?.$el.classList.add("rotate-once-animation");
    setTimeout(
      () => refreshButton.value?.$el.classList.remove("rotate-once-animation"),
      400
    );
  };

  const getFiles = () => {
    axios
      .post("/get_files", { path: path.value })
      .then((res) => {
        // @ts-ignore
        filesTable.value?.setFiles(res.data["files"]);
      })
      .catch((err) => {
        const error = err.response ? err.response.data.error : "Unknown error"; // TODO: implement timeout error
        // @ts-ignore
        filesTable.value?.setError(error);
      });
  };

  const ping = () => {
    axios
      .get("/ping")
      .then(() => (connected.value = true))
      .catch(() => (connected.value = false));
  };
  const getServerInfo = () => {
    error.value = "";
    axios
      .get("/info")
      .then((res) => {
        console.log(res);
        connected.value = true;
        path.value = res.data["path"];
        homePath.value = res.data["path"];
        getFiles();
      })
      .catch((err) => {
        error.value = "Couldn't connect to server";
      });
  };

  let pingTask: ReturnType<typeof setInterval>;
  onMounted(() => {
    console.debug("HomeView Mounted");

    getServerInfo();

    pingTask = setInterval(ping, 4000);
  });

  onUnmounted(() => {
    if (pingTask != null) clearInterval(pingTask);
  });
</script>

<style>
  .rotate-once-animation {
    animation-name: spin;
    animation-duration: 0.4s;
    animation-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
