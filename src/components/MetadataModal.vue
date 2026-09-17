<script setup>
import MetadataForm from "./MetadataForm.vue";

defineProps({
  project: {
    type: Object,
    required: true,
  },
  projectDetails: {
    type: Object,
    required: true,
  },
  transcript: {
    type: Object,
    required: true,
  },
  validationErrors: {
    type: Object,
    default: () => ({}),
  },
});

const model = defineModel("metadata", {
  type: Object,
  required: true,
});

const emit = defineEmits(["close", "generate", "preview-transcript"]);
</script>

<template>
  <div class="modal-backdrop" @click.self="emit('close')">
    <section class="metadata-modal" role="dialog" aria-modal="true" aria-labelledby="metadata-title">
      <div class="modal-header">
        <div>
          <p class="eyebrow">Project CapCut: {{ project.folderName }}</p>
          <h2 id="metadata-title">Metadata Kegiatan</h2>
          <div class="modal-status">
            <span>draft_info.json ✓</span>
            <span>{{ projectDetails.hasDraftContent ? "draft_content.json ✓" : "caption fallback: draft_info.json" }}</span>
          </div>
        </div>
        <button class="icon-button" type="button" title="Tutup modal" @click="emit('close')">
          x
        </button>
      </div>

      <div class="modal-body">
        <section class="caption-summary">
          <div>
            <h3>Caption CapCut</h3>
            <p v-if="transcript.captionCount > 0">
              ✓ {{ transcript.captionCount.toLocaleString("id-ID") }} caption ditemukan
            </p>
            <p v-else>Tidak ada caption yang ditemukan.</p>
            <span v-if="transcript.captionCount > 0">
              Durasi transcript: {{ transcript.duration }}
            </span>
          </div>

          <button
            class="button secondary"
            type="button"
            :disabled="transcript.captionCount === 0"
            @click="emit('preview-transcript')"
          >
            Preview Transcript
          </button>
        </section>

        <MetadataForm v-model="model" :errors="validationErrors" />
      </div>

      <div class="modal-actions">
        <button class="button secondary" type="button" @click="emit('close')">
          Close
        </button>
        <button
          class="button primary"
          type="button"
          :disabled="transcript.captionCount === 0"
          @click="emit('generate')"
        >
          Generate Markdown
        </button>
      </div>
    </section>
  </div>
</template>
