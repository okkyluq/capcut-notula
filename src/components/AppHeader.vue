<script setup>
import { openUrl } from "@tauri-apps/plugin-opener";

defineProps({
  osLabel: {
    type: String,
    default: "",
  },
});

const NOTULA_PROMPT_URL =
  "https://drive.google.com/file/d/1F06kaexYBdQCZdOPKgpb0-NU7-3DxVgm/view?usp=sharing";

async function openNotulaPrompt() {
  await openUrl(NOTULA_PROMPT_URL);
}
</script>

<template>
  <header class="app-header">
    <div>
      <h1>CapCut Notula <span>by mrluqman14</span></h1>
    </div>

    <div class="header-actions">
      <button class="header-link-button" type="button" @click="openNotulaPrompt">
        Prompt Notula
      </button>
      <div class="header-status">{{ osLabel }}</div>
    </div>
  </header>
</template>
