<template>
  <div>
    <input ref="inputEl" type="file" class="w-min bg-red-500" />
  </div>
  <button @click="upload">Upload</button>
</template>

<script setup lang="ts">
  import { ref, defineProps } from "vue";
  import { axios } from "../axios";

  const inputEl = ref<HTMLInputElement>(null);

  const upload = () => {
    let files = inputEl.value.files;
    console.log(files);
    console.log(files[0]);

    let formData = new FormData();
    formData.append("options", JSON.stringify({path: props.path}))
    
    for (let i = 0; i < files?.length; i++) {
      formData.append("file", files[i])
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
    path?: string | null
  }>();
</script>
