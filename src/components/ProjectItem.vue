<script setup>
defineProps({
  project: {
    type: Object,
    required: true,
  },
  selected: {
    type: Boolean,
    default: false,
  },
});

function formatModifiedAt(value) {
  if (!value) {
    return "-";
  }

  return new Intl.DateTimeFormat("id-ID", {
    day: "2-digit",
    month: "long",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(new Date(value));
}
</script>

<template>
  <button class="project-item" :class="{ selected }" type="button">
    <div>
      <div class="project-item-top">
        <strong>{{ project.folderName }}</strong>
        <span class="folder-code">Pilih</span>
      </div>

      <div class="project-subline">
        <span>Project CapCut</span>
        <span class="status-badge">✓ Valid</span>
      </div>
    </div>

    <div class="project-meta">
      <time>{{ formatModifiedAt(project.modifiedTimestamp) }}</time>
    </div>
  </button>
</template>
