<template>
  <div>
    <input
      ref="inputEl"
      type="file"
      class="w-64 rounded-md bg-gray-800 file:rounded-l-md file:border-0 file:bg-red-500 file:text-white"
      multiple="true"
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
  import { ref, defineProps, defineEmits } from "vue";
  import { axios } from "../axios";
  import { useToast, POSITION } from "vue-toastification";

  const toast = useToast();

  const inputEl = ref<HTMLInputElement>();

  const showUploadBtn = ref(false);

  const filesChanged = () =>
    (showUploadBtn.value = (inputEl.value?.files?.length || 0) > 0);

  const upload = () => {
    let files = inputEl.value?.files;

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
      .then(() => onUploadComplete());
  };

  const onUploadComplete = () => {
    emit("uploadComplete");
    toast.success("Upload complete", {
      position: POSITION.TOP_CENTER,
      pauseOnHover: false,
      hideProgressBar: true,
      timeout: 1500,
      showCloseButtonOnHover: true
    });
    inputEl.value && (inputEl.value.value = "");
    filesChanged();
  };

  const props = defineProps<{
    path?: string | null;
  }>();

  const emit = defineEmits<{
    (e: "uploadComplete"): void;
  }>();
</script>
