<script setup>
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import AppHeader from "./components/AppHeader.vue";
import ErrorMessage from "./components/ErrorMessage.vue";
import LoadingOverlay from "./components/LoadingOverlay.vue";
import MetadataModal from "./components/MetadataModal.vue";
import ProjectList from "./components/ProjectList.vue";
import ScanProject from "./components/ScanProject.vue";
import TranscriptPreviewModal from "./components/TranscriptPreviewModal.vue";
import ToastNotification from "./components/ToastNotification.vue";
import {
  readCapCutProject,
  readCapCutTranscript,
  scanCapCutProjects,
} from "./services/capcutService";
import { generateMarkdown as createMarkdown, saveMarkdown } from "./services/markdownService";

const projects = ref([]);
const selectedProject = ref(null);
const projectDetails = ref(null);
const transcript = ref(null);
const isLoading = ref(false);
const loadingMessage = ref("");
const hasScanned = ref(false);
const error = ref(null);
const rootPath = ref("");
const searchQuery = ref("");
const isMetadataModalOpen = ref(false);
const isTranscriptPreviewOpen = ref(false);
const generateNotice = ref("");
const validationErrors = ref({});
const toastMessage = ref("");
let toastTimer = null;

const metadata = ref({
  namaKegiatan: "",
  tanggal: "",
  tempat: "",
  agenda: "",
  pimpinanRapat: "",
  peserta: "",
  notulis: "",
});

const osLabel = computed(() => {
  const platform = window.navigator.platform.toLowerCase();

  if (platform.includes("mac")) return "macOS";
  if (platform.includes("win")) return "Windows";
  return "Desktop";
});

const filteredProjects = computed(() => {
  const query = searchQuery.value.trim().toLowerCase();
  const sortedProjects = [...projects.value].sort(
    (a, b) => (b.modifiedTimestamp || 0) - (a.modifiedTimestamp || 0),
  );

  if (!query) {
    return sortedProjects;
  }

  return sortedProjects.filter((project) =>
    [project.folderName, project.path, project.modifiedAt]
      .join(" ")
      .toLowerCase()
      .includes(query),
  );
});

function delay(ms) {
  return new Promise((resolve) => {
    window.setTimeout(resolve, ms);
  });
}

function validateMetadata() {
  const errors = {};

  if (!metadata.value.namaKegiatan.trim()) {
    errors.namaKegiatan = "Nama Kegiatan wajib diisi.";
  }

  if (!metadata.value.tanggal.trim()) {
    errors.tanggal = "Hari / Tanggal wajib diisi.";
  }

  if (!metadata.value.tempat.trim()) {
    errors.tempat = "Tempat wajib diisi.";
  }

  validationErrors.value = errors;
  return Object.keys(errors).length === 0;
}

function showToast(message) {
  toastMessage.value = message;

  if (toastTimer) {
    window.clearTimeout(toastTimer);
  }

  toastTimer = window.setTimeout(() => {
    toastMessage.value = "";
    toastTimer = null;
  }, 3000);
}

async function scanProjects() {
  isLoading.value = true;
  loadingMessage.value = "Memindai Project CapCut...";
  hasScanned.value = true;
  error.value = null;
  generateNotice.value = "";
  validationErrors.value = {};
  selectedProject.value = null;
  projectDetails.value = null;
  transcript.value = null;
  isMetadataModalOpen.value = false;
  isTranscriptPreviewOpen.value = false;

  try {
    await delay(2000);
    const result = await scanCapCutProjects();
    projects.value = result.projects;
    rootPath.value = result.rootPath;
  } catch (scanError) {
    projects.value = [];
    error.value = {
      title: scanError?.message || "Folder project CapCut tidak ditemukan pada komputer ini.",
      detail: scanError?.rootPath ? `Lokasi yang diperiksa: ${scanError.rootPath}` : "",
    };
    rootPath.value = scanError?.rootPath || "";
  } finally {
    isLoading.value = false;
    loadingMessage.value = "";
  }
}

async function openProject(project) {
  isLoading.value = true;
  loadingMessage.value = "Membaca Project...";
  error.value = null;
  generateNotice.value = "";
  validationErrors.value = {};

  try {
    const details = await readCapCutProject(project.path);
    const transcriptResult = await readCapCutTranscript(project.path);
    selectedProject.value = details.project;
    projectDetails.value = details;
    transcript.value = transcriptResult;
    isMetadataModalOpen.value = true;
  } catch (readError) {
    selectedProject.value = null;
    projectDetails.value = null;
    transcript.value = null;
    error.value = {
      title: readError?.message || "Tidak dapat membaca project CapCut.",
      detail: readError?.detail || "",
    };
  } finally {
    isLoading.value = false;
    loadingMessage.value = "";
  }
}

function closeMetadataModal() {
  isMetadataModalOpen.value = false;
  isTranscriptPreviewOpen.value = false;
}

function closeError() {
  error.value = null;
}

async function generateMarkdown() {
  if (!validateMetadata()) {
    return;
  }

  if (!transcript.value || transcript.value.captionCount === 0) {
    error.value = {
      title: "Caption belum dapat dibaca dari project ini.",
      detail: "Generate Markdown membutuhkan transcript caption CapCut.",
    };
    return;
  }

  const markdown = createMarkdown(metadata.value, projectDetails.value, transcript.value.items);

  console.log("Selected Project", selectedProject.value);
  console.log("Project Details", projectDetails.value);
  console.log("Transcript", transcript.value);
  console.log("Metadata Kegiatan", metadata.value);
  console.log("Markdown Preview", markdown);

  try {
    const savedFile = await saveMarkdown(markdown, metadata.value);

    if (!savedFile) {
      return;
    }

    isMetadataModalOpen.value = false;
    generateNotice.value = "";
    showToast(`${savedFile.fileName} berhasil disimpan`);
  } catch (saveError) {
    console.error("File Markdown tidak dapat disimpan.", saveError);
    error.value = {
      title: "File Markdown tidak dapat disimpan.",
      detail: "Periksa lokasi file dan izin penulisan, lalu coba lagi.",
    };
  }
}

function handleEscape(event) {
  if (event.key === "Escape") {
    if (isMetadataModalOpen.value) closeMetadataModal();
    if (error.value) closeError();
  }
}

onMounted(() => {
  window.addEventListener("keydown", handleEscape);
});

onBeforeUnmount(() => {
  window.removeEventListener("keydown", handleEscape);

  if (toastTimer) {
    window.clearTimeout(toastTimer);
  }
});
</script>

<template>
  <main class="app-frame">
    <AppHeader :os-label="osLabel" />

    <section class="content-area">
      <ScanProject :loading="isLoading" :has-scanned="hasScanned" @scan="scanProjects" />

      <section v-if="hasScanned && !isLoading" class="scan-results">
        <div v-if="projects.length === 0 && !error" class="state-panel">
          <h2>Tidak ada project CapCut ditemukan.</h2>
          <p v-if="rootPath">
            Lokasi yang diperiksa:
            <span>{{ rootPath }}</span>
          </p>
          <button class="button secondary" type="button" @click="scanProjects">
            Scan Ulang
          </button>
        </div>

        <div v-else-if="projects.length > 0" class="project-results">
          <div class="results-header">
            <div>
              <h2>Project CapCut</h2>
              <p>{{ filteredProjects.length }} dari {{ projects.length }} Project ditemukan</p>
            </div>

            <div class="results-actions">
              <input v-model="searchQuery" type="search" placeholder="Cari project..." />
              <button class="button secondary" type="button" @click="scanProjects">
                Scan Ulang
              </button>
            </div>
          </div>

          <ProjectList
            :projects="filteredProjects"
            :selected-project-id="selectedProject?.id"
            @select-project="openProject"
          />

          <p v-if="filteredProjects.length === 0" class="state-panel compact">
            Project tidak ditemukan untuk pencarian ini.
          </p>

          <p v-if="generateNotice" class="notice">{{ generateNotice }}</p>
        </div>
      </section>
    </section>

    <footer class="app-footer">
      © 2026 mrluqman14
    </footer>

    <LoadingOverlay v-if="isLoading" :message="loadingMessage" />

    <ErrorMessage
      v-if="error"
      :title="error.title"
      :detail="error.detail"
      @close="closeError"
    />

    <MetadataModal
      v-if="isMetadataModalOpen && selectedProject && projectDetails && transcript"
      v-model:metadata="metadata"
      :project="selectedProject"
      :project-details="projectDetails"
      :transcript="transcript"
      :validation-errors="validationErrors"
      @close="closeMetadataModal"
      @generate="generateMarkdown"
      @preview-transcript="isTranscriptPreviewOpen = true"
    />

    <TranscriptPreviewModal
      v-if="isTranscriptPreviewOpen && transcript"
      :transcript="transcript"
      @close="isTranscriptPreviewOpen = false"
    />

    <ToastNotification v-if="toastMessage" :message="toastMessage" />
  </main>
</template>
