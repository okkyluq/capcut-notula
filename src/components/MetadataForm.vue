<script setup>
defineProps({
  errors: {
    type: Object,
    default: () => ({}),
  },
});

const model = defineModel({
  type: Object,
  required: true,
});
</script>

<template>
  <section class="panel metadata-panel">
    <div class="section-title">
      <p class="eyebrow">Input</p>
      <h2>Metadata Kegiatan</h2>
    </div>

    <div class="form-grid">
      <label>
        <span>Nama Kegiatan</span>
        <input v-model="model.namaKegiatan" type="text" />
        <small v-if="errors.namaKegiatan">{{ errors.namaKegiatan }}</small>
      </label>

      <label>
        <span>Hari / Tanggal</span>
        <input v-model="model.tanggal" type="date" />
        <small v-if="errors.tanggal">{{ errors.tanggal }}</small>
      </label>

      <label>
        <span>Tempat</span>
        <input v-model="model.tempat" type="text" />
        <small v-if="errors.tempat">{{ errors.tempat }}</small>
      </label>

      <label class="span-2">
        <span>Agenda</span>
        <textarea v-model="model.agenda" rows="3"></textarea>
      </label>

      <label>
        <span>Pimpinan Rapat</span>
        <input v-model="model.pimpinanRapat" type="text" />
      </label>

      <label>
        <span>Notulis</span>
        <input v-model="model.notulis" type="text" />
      </label>

      <label class="span-2">
        <span>Peserta</span>
        <textarea v-model="model.peserta" rows="3"></textarea>
      </label>
    </div>
  </section>
</template>
