<template>
  <Transition name="fade">
    <div
      v-if="loading"
      :class="class"
      class="absolute top-0 left-0 z-[50] flex h-full w-full flex-col bg-black bg-opacity-30 p-1 backdrop-blur-sm"
      style="border-radius: inherit"
    >
      <Transition name="bounce" appear>
        <LoadingSpinner class="w-16 self-center text-red-500" />
      </Transition>
    </div>
  </Transition>
</template>

<script setup lang="ts">
  import { defineProps, withDefaults } from "vue";
  // @ts-ignore
  import LoadingSpinner from "../assets/circular-spinner.svg?component";

  const props = withDefaults(
    defineProps<{
      loading: boolean;
      class?: string;
    }>(),
    {
      class: "justify-center",
    }
  );
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

<style>
  @keyframes Circular-_-rotate {
    100% {
      transform: rotate3d(0, 0, 1, 360deg);
    }
  }
  @keyframes Circular-_-dash {
    0% {
      stroke-dasharray: 1, 200;
      stroke-dashoffset: 0;
    }
    50% {
      stroke-dasharray: 89, 200;
      stroke-dashoffset: -35px;
    }
    100% {
      stroke-dasharray: 89, 200;
      stroke-dashoffset: -124px;
    }
  }

  .circular {
    font-size: 18px;
  }
</style>
