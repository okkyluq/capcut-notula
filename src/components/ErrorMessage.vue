<script setup>
defineProps({
  title: {
    type: String,
    required: true,
  },
  detail: {
    type: String,
    default: "",
  },
});

const emit = defineEmits(["close"]);
</script>

<template>
  <div class="modal-backdrop" @click.self="emit('close')">
    <section class="error-dialog" role="alertdialog" aria-modal="true" aria-labelledby="error-title">
      <h2 id="error-title">{{ title }}</h2>
      <p v-if="detail">
        Detail:
        <span>{{ detail }}</span>
      </p>
      <button class="button secondary" type="button" @click="emit('close')">
        Tutup
      </button>
    </section>
  </div>
</template>
