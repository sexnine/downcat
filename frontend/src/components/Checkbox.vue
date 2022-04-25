<template>
  <!-- <div class="flex cursor-pointer justify-items-center"> -->
  <div
    class="flex h-6 w-6 cursor-pointer items-center justify-center rounded-md text-center shadow-md transition duration-150 ease-in-out"
    :class="checked ? 'bounce bg-red-500' : 'bg-gray-500 bg-opacity-30'"
  >
    <fa-icon v-show="checked" class="" :icon="['fas', 'check']" />
  </div>
  <!-- <p class="select-none font-semibold">{{ text }}</p>
  </div> -->
</template>

<script>
  export default {
    props: {
      text: {
        type: String,
        required: true,
      },
      checked: {
        type: Boolean,
        default: false,
      },
    },
  };
</script>

<style>
  @keyframes bounce-anim {
    0% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.1);
    }
    100% {
      transform: scale(1);
    }
  }
  .bounce {
    animation: bounce-anim 200ms;
  }
</style>
