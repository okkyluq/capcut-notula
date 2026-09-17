<script setup>
defineProps({
  message: {
    type: String,
    required: true,
  },
});
</script>

<template>
  <div class="loading-overlay" aria-live="polite" aria-busy="true">
    <div class="loading-box">
      <div class="spinner"></div>
      <p>{{ message }}</p>
      <span>Mohon tunggu sebentar</span>
    </div>
  </div>
</template>
