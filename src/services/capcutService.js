import { invoke } from "@tauri-apps/api/core";

export async function scanCapCutProjects() {
  return await invoke("scan_capcut_projects");
}

export async function readCapCutProject(projectPath) {
  return await invoke("read_capcut_project", { projectPath });
}

export async function readCapCutTranscript(projectPath) {
  return await invoke("read_capcut_transcript", { projectPath });
}
