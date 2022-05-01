<template>
  <div>
    <input
      ref="inputEl"
      type="file"
      class="w-64 rounded-md bg-gray-800 file:rounded-l-md file:border-0 file:bg-red-500 file:text-white"
      @change="filesChanged"
    />
  </div>
  <button
    v-if="showUploadBtn"
    class="rounded-md bg-red-500 px-2"
    @click="upload"
  >
    Upload
  </button>
</template>

<script setup lang="ts">
  import { ref, defineProps } from "vue";
  import { axios } from "../axios";

  const inputEl = ref<HTMLInputElement>(null);

  const showUploadBtn = ref(false);

  const filesChanged = () =>
    (showUploadBtn.value = inputEl.value.files?.length > 0);

  const upload = () => {
    let files = inputEl.value.files;

    if (!files) return;

    let formData = new FormData();
    formData.append("options", JSON.stringify({ path: props.path }));

    for (let i = 0; i < files?.length; i++) {
      formData.append("file", files[i]);
    }

    axios
      .post("/upload", formData, {
        headers: {
          "Content-Type": "multipart/form-data",
        },
      })
      .then(() => console.log("done"));
  };

  const props = defineProps<{
    path?: string | null;
  }>();
</script>
