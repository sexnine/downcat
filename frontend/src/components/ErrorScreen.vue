<template>
  <Transition name="fade">
    <div
      v-if="error"
      :class="class"
      class="absolute top-0 left-0 z-[51] flex h-full w-full flex-col bg-black bg-opacity-30 p-1 backdrop-blur-sm"
      style="border-radius: inherit"
    >
      <Transition name="bounce" appear>
        <div class="flex flex-col items-center justify-center text-center">
          <fa-icon
            :icon="['fas', 'exclamation-circle']"
            class="mb-2 text-4xl text-red-500"
          />
          <p class="text-2xl text-red-500">{{ error }}</p>
          <button
            v-if="buttonText"
            class="mt-2 max-w-sm grow transform-gpu rounded-md bg-red-500 px-2 py-1 text-lg font-semibold transition hover:-translate-y-1 hover:drop-shadow-lg"
            @click="emit('buttonClicked')"
          >
            {{ buttonText }}
          </button>
        </div>
      </Transition>
    </div>
  </Transition>
</template>

<script setup lang="ts">
  import { defineProps, withDefaults, defineEmits } from "vue";

  const props = withDefaults(
    defineProps<{
      error: string;
      buttonText?: string;
      class?: string;
    }>(),
    {
      class: "justify-center",
      buttonText: "",
    }
  );

  const emit = defineEmits<{
    (e: "buttonClicked"): void;
  }>();
</script>

<style scoped>
  .fade-enter-active,
  .fade-leave-active {
    transition-duration: 0.15s;
    transition-property: opacity;
    transition-timing-function: ease-in-out;
  }

  .fade-enter-from,
  .fade-leave-to {
    opacity: 0;
  }

  .bounce-enter-active {
    animation: bounce-in 0.3s;
  }

  @keyframes bounce-in {
    0% {
      transform: scale(0);
    }
    20% {
      transform: scale(0);
    }
    60% {
      transform: scale(1.3);
    }
    100% {
      transform: scale(1);
    }
  }
</style>

<style></style>
