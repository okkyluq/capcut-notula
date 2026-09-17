<script setup>
import ProjectItem from "./ProjectItem.vue";

defineProps({
  projects: {
    type: Array,
    required: true,
  },
  selectedProjectId: {
    type: String,
    default: null,
  },
});

const emit = defineEmits(["select-project"]);
</script>

<template>
  <div class="project-list-panel">
    <ProjectItem
      v-for="project in projects"
      :key="project.id"
      :project="project"
      :selected="project.id === selectedProjectId"
      @click="emit('select-project', project)"
    />
  </div>
</template>
