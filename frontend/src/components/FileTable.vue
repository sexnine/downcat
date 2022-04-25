<template>
  <div class="relative h-full grow pt-4">
    <loading-screen :loading="loading" class="justify-start pt-8" />
    <error-screen
      :error="error"
      button-text="Try Again"
      class="justify-start pt-8"
      @button-clicked="errBtnClicked"
    />
    <table ref="table" class="files-table w-full items-end px-2 text-left">
      <thead>
        <tr>
          <th>
            <!-- <fa-icon :icon="['fas', 'check']" /> -->
          </th>
          <th v-for="field in fields" :key="field" class="flex items-center">
            <fa-icon
              :icon="['fas', sortIcon(field)]"
              :class="sortField == field ? 'text-red-500' : 'text-gray-400'"
              class="mr-2 cursor-pointer"
              @click="sortClicked(field)"
            />{{ field }}
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="f in files" :key="f.name">
          <td>
            <Checkbox text=" " :checked="false" />
          </td>
          <td class="flex items-center">
            <div class="">
              <fa-icon
                :icon="['fas', f.file_type == 'file' ? 'file' : 'folder']"
                class="mr-2 aspect-square text-xl"
              />
              <a
                class="cursor-pointer text-ellipsis text-blue-400"
                v-bind="getDownloadAttributes(f)"
                v-on="
                  f.file_type == 'dir'
                    ? { click: () => dirClicked(f.path) }
                    : {}
                "
                >{{ f.name }}</a
              >
            </div>
            <div class="mx-4 h-[1px] grow bg-gray-400 bg-opacity-10" />
            <a
              v-if="f.file_type == 'file'"
              :href="getViewURL(f.path)"
              target="_blank"
            >
              <fa-icon
                :icon="['fas', 'up-right-from-square']"
                class="cursor-pointer text-xl text-blue-400"
              />
            </a>
          </td>
          <td>{{ formatSize(f.size) }}</td>
          <td>{{ formatTime(f.modified) }}</td>
          <td>{{ formatTime(f.created) }}</td>
        </tr>
      </tbody>
    </table>
    <p class="px-4 text-gray-300">{{ files.length }} Items</p>
  </div>
</template>

<script setup lang="ts">
  import { ref, defineProps, defineExpose, defineEmits, nextTick } from "vue";
  import Checkbox from "./Checkbox.vue";
  import LoadingScreen from "./LoadingScreen.vue";
  import ErrorScreen from "./ErrorScreen.vue";
  import { formatBytes } from "bytes-formatter";
  import dayjs from "dayjs";
  import { baseURL } from "../axios";
  import { store } from "../store";

  // const checked = ref(false);
  const loading = ref(true);
  const error = ref("");
  const files = ref<File[]>([]);

  const setLoading = (val: boolean) => (loading.value = val);
  const setError = (val: string) =>
    (error.value = "Error loading files: " + val);
  const removeError = () => (error.value = "");
  const setFiles = (ifiles: File[]) => {
    files.value = ifiles;
    sortFiles();
    nextTick(() => {
      loading.value = false;
    });
  };

  const formatSize = (size: number | undefined) => {
    return size != null ? formatBytes(size) : "--";
  };
  const formatTime = (timestamp: number | undefined) => {
    return timestamp
      ? dayjs.unix(timestamp).format("MMM D YYYY, h:mm A")
      : "--";
  };
  const getTokenParam = () =>
    store.token ? "&key=" + encodeURIComponent(store.token) : "";
  const getDownloadAttributes = (file: File) => {
    if (file.file_type != "file") return;
    return {
      href: `${baseURL}/get_file?path=${encodeURIComponent(
        file.path
      )}${getTokenParam()}`,
      download: file.name,
    };
  };
  const getViewURL = (path: string) =>
    `${baseURL}/get_file?view=1&path=${encodeURIComponent(
      path
    )}${getTokenParam()}`;

  const fields = ref(["Name", "Size", "Modified", "Created"]);
  const sortField = ref("Name");
  const sortDirection = ref("up");

  const sortIcon = (field: string) => {
    if (field != sortField.value) {
      return "sort";
    }
    return "sort-" + sortDirection.value;
  };
  const sortClicked = (field: string) => {
    if (field == sortField.value) {
      sortDirection.value = sortDirection.value == "up" ? "down" : "up";
    } else {
      sortField.value = field;
      sortDirection.value = "up";
    }
    sortFiles();
  };

  const sortFns: { [key: string]: (a: File, b: File) => number } = {
    Name: (a: File, b: File) => a.name.localeCompare(b.name),
    Size: (a: File, b: File) => (a.size || 0) - (b.size || 0),
    Modified: (a: File, b: File) => (a.modified || 0) - (b.modified || 0),
    Created: (a: File, b: File) => (a.created || 0) - (b.created || 0),
  };
  const sortFiles = () => {
    files.value.sort(
      sortDirection.value == "up"
        ? sortFns[sortField.value]
        : (a, b) => sortFns[sortField.value](b, a)
    );
  };

  // const check = () => (checked.value = !checked.value);
  const dirClicked = (path: string) => emit("switchDir", path);
  const errBtnClicked = () => emit("errBtnClicked");

  interface File {
    name: string;
    modified?: number;
    created?: number;
    size?: number;
    file_type: string;
    path: string;
  }

  const props = defineProps<{
    error?: string;
  }>();

  const emit = defineEmits<{
    (e: "switchDir", path: string): void;
    (e: "errBtnClicked"): void;
  }>();

  defineExpose({
    setLoading,
    setError,
    setFiles,
    removeError,
  });
</script>

<style scoped>
  table {
    -webkit-box-flex: 1;
    flex: 1;
    display: grid;
    border-collapse: collapse;
    grid-template-columns:
      40px
      minmax(200px, 1.67fr)
      minmax(100px, 150px)
      minmax(200px, 230px)
      minmax(200px, 230px);
  }

  thead,
  tbody,
  tr {
    display: contents;
  }

  th {
    @apply pl-2 text-lg font-semibold;
  }

  td {
    @apply px-2 pb-2;
  }
</style>
