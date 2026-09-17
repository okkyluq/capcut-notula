<script setup>
import { computed, ref } from "vue";

const props = defineProps({
  transcript: {
    type: Object,
    required: true,
  },
});

const emit = defineEmits(["close"]);
const searchQuery = ref("");

const filteredItems = computed(() => {
  const query = searchQuery.value.trim().toLowerCase();

  if (!query) {
    return props.transcript.items;
  }

  return props.transcript.items.filter((item) =>
    [item.timestamp, item.text].join(" ").toLowerCase().includes(query),
  );
});
</script>

<template>
  <div class="modal-backdrop transcript-backdrop" @click.self="emit('close')">
    <section class="transcript-modal" role="dialog" aria-modal="true" aria-labelledby="transcript-title">
      <div class="modal-header">
        <div>
          <p class="eyebrow">Project: {{ transcript.projectId }}</p>
          <h2 id="transcript-title">Preview Transcript</h2>
          <div class="modal-status">
            <span>Caption: {{ transcript.captionCount }}</span>
            <span>Durasi: {{ transcript.duration }}</span>
          </div>
        </div>
        <button class="icon-button" type="button" title="Tutup preview" @click="emit('close')">
          x
        </button>
      </div>

      <div class="transcript-toolbar">
        <input v-model="searchQuery" type="search" placeholder="Cari dalam transcript..." />
      </div>

      <div class="transcript-rows">
        <div v-for="item in filteredItems" :key="item.id" class="transcript-row">
          <time>{{ item.timestamp }}</time>
          <p>{{ item.text }}</p>
        </div>

        <p v-if="filteredItems.length === 0" class="empty-transcript">
          Tidak ada caption yang cocok.
        </p>
      </div>

      <div class="modal-actions">
        <button class="button secondary" type="button" @click="emit('close')">
          Close
        </button>
      </div>
    </section>
  </div>
</template>
