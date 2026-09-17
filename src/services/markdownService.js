import { save } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";

function normalizeText(value, fallback = "-") {
  const text = String(value || "").trim();
  return text || fallback;
}

function formatDate(value) {
  if (!value) {
    return "-";
  }

  const [year, month, day] = value.split("-").map(Number);
  const date = new Date(year, month - 1, day);

  if (Number.isNaN(date.getTime())) {
    return value;
  }

  return new Intl.DateTimeFormat("id-ID", {
    day: "numeric",
    month: "long",
    year: "numeric",
  }).format(date);
}

function formatParticipants(value) {
  const participants = String(value || "")
    .split("\n")
    .map((item) => item.trim())
    .filter(Boolean);

  if (participants.length === 0) {
    return "-";
  }

  return participants.map((item) => `  - ${item}`).join("\n");
}

function sanitizeFilename(value) {
  return String(value || "")
    .replace(/[<>:"/\\|?*]+/g, " ")
    .replace(/\s+/g, " ")
    .trim();
}

function ensureMarkdownExtension(value) {
  return value.toLowerCase().endsWith(".md") ? value : `${value}.md`;
}

export function createMarkdownFilename(metadata) {
  const baseName = sanitizeFilename(metadata.namaKegiatan) || "Transkrip Kegiatan";
  return ensureMarkdownExtension(baseName);
}

export function generateMarkdown(metadata, projectDetails, transcript = []) {
  const title = normalizeText(metadata.namaKegiatan, "Notula Rapat");
  const projectName = projectDetails?.project?.folderName || "-";
  const duration = transcript.at(-1)?.endMs
    ? formatDuration(transcript.at(-1).endMs)
    : "00:00:00";
  const transcriptText =
    transcript.length > 0
      ? transcript.map((line) => `[${line.timestamp}] ${line.text}`).join("\n\n")
      : "Transkrip caption CapCut akan ditempatkan pada bagian ini.";

  return `# ${title}

## Metadata Kegiatan

- Nama Kegiatan: ${normalizeText(metadata.namaKegiatan)}
- Hari/Tanggal: ${formatDate(metadata.tanggal)}
- Tempat: ${normalizeText(metadata.tempat)}
- Agenda: ${normalizeText(metadata.agenda)}
- Pimpinan Rapat: ${normalizeText(metadata.pimpinanRapat)}
- Peserta/Instansi:
${formatParticipants(metadata.peserta)}
- Notulis: ${normalizeText(metadata.notulis)}

---

## Informasi Sumber

**Sumber:** Caption CapCut

**Project:** ${projectName}

**Jumlah Caption:** ${transcript.length}

**Durasi:** ${duration}

---

## Instruksi Untuk ChatGPT

Ubah transkrip berikut menjadi notula rapat yang rapi. Pertahankan pokok pembahasan, keputusan, tindak lanjut, pihak terkait, dan gunakan timestamp sebagai referensi bila diperlukan.

---

## Transkrip

${transcriptText}
`;
}

function formatDuration(ms) {
  const totalSeconds = Math.floor(ms / 1000);
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;

  return [hours, minutes, seconds]
    .map((value) => String(value).padStart(2, "0"))
    .join(":");
}

export async function saveMarkdown(markdown, metadata) {
  const filePath = await save({
    title: "Simpan Markdown",
    defaultPath: createMarkdownFilename(metadata),
    filters: [
      {
        name: "Markdown",
        extensions: ["md"],
      },
    ],
  });

  if (!filePath) {
    return null;
  }

  const targetPath = ensureMarkdownExtension(filePath);
  await writeTextFile(targetPath, markdown);

  return {
    path: targetPath,
    fileName: targetPath.split(/[\\/]/).pop(),
  };
}
