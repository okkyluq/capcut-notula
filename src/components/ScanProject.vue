<script setup>
defineProps({
  loading: {
    type: Boolean,
    default: false,
  },
  hasScanned: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(["scan"]);
</script>

<template>
  <section class="scan-hero">
    <h1>CapCut Project Scanner</h1>
    <p>Ubah Caption CapCut menjadi bahan Notula / Laporan Kegiatan</p>

    <button class="button primary scan-button" type="button" :disabled="loading" @click="emit('scan')">
      {{ hasScanned ? "Scan Ulang" : "Scan Project" }}
    </button>
  </section>
</template>
