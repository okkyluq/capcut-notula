<script setup>
defineProps({
  message: {
    type: String,
    required: true,
  },
});
</script>

<template>
  <div class="toast-notification" role="status">
    <span>✓</span>
    {{ message }}
  </div>
</template>
