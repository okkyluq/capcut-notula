use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CapCutProject {
    id: String,
    folder_name: String,
    path: String,
    info_path: String,
    modified_at: String,
    modified_timestamp: Option<u64>,
    valid: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ScanResult {
    root_path: String,
    projects: Vec<CapCutProject>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ScanError {
    message: String,
    root_path: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectReadResult {
    project: CapCutProject,
    has_draft_info: bool,
    has_draft_content: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectReadError {
    message: String,
    detail: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TranscriptItem {
    id: String,
    start_ms: u64,
    end_ms: u64,
    duration_ms: u64,
    timestamp: String,
    text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CapCutTranscriptResult {
    project_id: String,
    caption_file_found: bool,
    caption_count: usize,
    duration_ms: u64,
    duration: String,
    items: Vec<TranscriptItem>,
}

type TranscriptError = ProjectReadError;

fn get_capcut_projects_path() -> Result<PathBuf, ScanError> {
    #[cfg(target_os = "macos")]
    {
        let home = env::var_os("HOME").ok_or_else(|| ScanError {
            message: "Home directory user tidak dapat ditemukan.".to_string(),
            root_path: None,
        })?;

        return Ok(PathBuf::from(home)
            .join("Movies")
            .join("CapCut")
            .join("User Data")
            .join("Projects")
            .join("com.lveditor.draft"));
    }

    #[cfg(target_os = "windows")]
    {
        let local_app_data = env::var_os("LOCALAPPDATA").ok_or_else(|| ScanError {
            message: "LOCALAPPDATA user tidak dapat ditemukan.".to_string(),
            root_path: None,
        })?;

        return Ok(PathBuf::from(local_app_data)
            .join("CapCut")
            .join("User Data")
            .join("Projects")
            .join("com.lveditor.draft"));
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Err(ScanError {
            message: "Sistem operasi ini belum didukung oleh scanner CapCut.".to_string(),
            root_path: None,
        })
    }
}

fn get_modified_timestamp(path: &Path) -> Option<u64> {
    let modified = fs::metadata(path).ok()?.modified().ok()?;
    let duration = modified.duration_since(UNIX_EPOCH).ok()?;

    Some(duration.as_millis() as u64)
}

fn format_modified_time(timestamp: Option<u64>) -> String {
    timestamp
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn project_from_directory(path: PathBuf) -> Option<CapCutProject> {
    let info_path = path.join("draft_info.json");

    if !info_path.is_file() {
        return None;
    }

    let folder_name = path.file_name()?.to_string_lossy().to_string();
    let modified_timestamp = get_modified_timestamp(&path);

    Some(CapCutProject {
        id: folder_name.clone(),
        folder_name,
        path: path.to_string_lossy().to_string(),
        info_path: info_path.to_string_lossy().to_string(),
        modified_at: format_modified_time(modified_timestamp),
        modified_timestamp,
        valid: true,
    })
}

fn read_project_directories(root_path: &Path) -> io::Result<(Vec<CapCutProject>, usize)> {
    let mut projects = Vec::new();
    let mut directories_scanned = 0;

    for entry in fs::read_dir(root_path)? {
        let Ok(entry) = entry else {
            continue;
        };

        let Ok(file_type) = entry.file_type() else {
            continue;
        };

        if !file_type.is_dir() {
            continue;
        }

        directories_scanned += 1;

        if let Some(project) = project_from_directory(entry.path()) {
            projects.push(project);
        }
    }

    projects.sort_by(|a, b| b.modified_timestamp.cmp(&a.modified_timestamp));

    Ok((projects, directories_scanned))
}

fn canonical_project_path(project_path: String) -> Result<PathBuf, ProjectReadError> {
    let root_path = get_capcut_projects_path().map_err(|error| ProjectReadError {
        message: "Tidak dapat membaca project CapCut.".to_string(),
        detail: error.message,
    })?;

    let canonical_root = root_path.canonicalize().map_err(|_| ProjectReadError {
        message: "Tidak dapat membaca project CapCut.".to_string(),
        detail: "Folder root project CapCut tidak ditemukan.".to_string(),
    })?;

    let requested_path = PathBuf::from(project_path);
    let canonical_project = requested_path.canonicalize().map_err(|_| ProjectReadError {
        message: "Tidak dapat membaca project CapCut.".to_string(),
        detail: "Folder project tidak ditemukan.".to_string(),
    })?;

    if !canonical_project.starts_with(&canonical_root) {
        return Err(ProjectReadError {
            message: "Tidak dapat membaca project CapCut.".to_string(),
            detail: "Path project berada di luar folder CapCut yang diizinkan.".to_string(),
        });
    }

    if canonical_project.parent() != Some(canonical_root.as_path()) {
        return Err(ProjectReadError {
            message: "Tidak dapat membaca project CapCut.".to_string(),
            detail: "Path project bukan folder project langsung dari root CapCut.".to_string(),
        });
    }

    if !canonical_project.is_dir() {
        return Err(ProjectReadError {
            message: "Tidak dapat membaca project CapCut.".to_string(),
            detail: "Path project bukan folder.".to_string(),
        });
    }

    if !canonical_project.join("draft_info.json").is_file() {
        return Err(ProjectReadError {
            message: "Tidak dapat membaca project CapCut.".to_string(),
            detail: "draft_info.json tidak ditemukan.".to_string(),
        });
    }

    Ok(canonical_project)
}

fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn inspect_draft_content(value: &Value) {
    println!("[CapCut Draft Content Inspection]");

    if let Some(object) = value.as_object() {
        let keys = object.keys().cloned().collect::<Vec<_>>();
        println!("Top-level keys: {}", keys.join(", "));

        for (key, child) in object {
            match child {
                Value::Array(items) => println!("{}: array({})", key, items.len()),
                Value::Object(map) => println!("{}: object({} keys)", key, map.len()),
                _ => println!("{}: {}", key, value_kind(child)),
            }
        }

        if let Some(materials) = object.get("materials").and_then(Value::as_object) {
            let material_keys = materials.keys().cloned().collect::<Vec<_>>();
            println!("materials keys: {}", material_keys.join(", "));
            if let Some(texts) = materials.get("texts").and_then(Value::as_array) {
                println!("materials.texts: {}", texts.len());
            }
        }

        if let Some(tracks) = object.get("tracks").and_then(Value::as_array) {
            let segment_count = tracks
                .iter()
                .filter_map(|track| track.get("segments").and_then(Value::as_array))
                .map(Vec::len)
                .sum::<usize>();
            println!("tracks: {}", tracks.len());
            println!("track segments: {}", segment_count);
        }
    }
}

fn string_field<'a>(object: &'a Map<String, Value>, keys: &[&str]) -> Option<&'a str> {
    keys.iter().find_map(|key| object.get(*key)?.as_str())
}

fn numeric_field(object: &Map<String, Value>, keys: &[&str]) -> Option<u64> {
    keys.iter().find_map(|key| {
        let value = object.get(*key)?;
        value.as_u64().or_else(|| value.as_f64().map(|number| number.max(0.0) as u64))
    })
}

fn contains_caption_signal(value: &Value) -> bool {
    match value {
        Value::String(text) => {
            let lower = text.to_lowercase();
            lower.contains("caption") || lower.contains("subtitle") || lower.contains("recognize")
        }
        Value::Array(items) => items.iter().any(contains_caption_signal),
        Value::Object(object) => object.iter().any(|(key, child)| {
            let lower_key = key.to_lowercase();
            lower_key.contains("caption")
                || lower_key.contains("subtitle")
                || lower_key.contains("recognize")
                || contains_caption_signal(child)
        }),
        _ => false,
    }
}

fn normalize_caption_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn extract_text_from_content(value: &Value) -> Option<String> {
    match value {
        Value::String(text) => {
            let trimmed = text.trim();
            if trimmed.starts_with('{') || trimmed.starts_with('[') {
                if let Ok(parsed) = serde_json::from_str::<Value>(trimmed) {
                    return extract_text_from_content(&parsed);
                }
            }

            Some(trimmed.to_string())
        }
        Value::Array(items) => {
            let text = items
                .iter()
                .filter_map(extract_text_from_content)
                .filter(|item| !item.trim().is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            Some(text)
        }
        Value::Object(object) => {
            if let Some(text) = string_field(object, &["text", "content", "words"]) {
                return Some(text.to_string());
            }

            for key in ["texts", "children", "items"] {
                if let Some(value) = object.get(key) {
                    if let Some(text) = extract_text_from_content(value) {
                        return Some(text);
                    }
                }
            }

            None
        }
        _ => None,
    }
}

fn text_materials(value: &Value) -> HashMap<String, (String, bool)> {
    let mut materials = HashMap::new();

    let Some(texts) = value
        .get("materials")
        .and_then(|materials| materials.get("texts"))
        .and_then(Value::as_array)
    else {
        return materials;
    };

    for item in texts {
        let Some(object) = item.as_object() else {
            continue;
        };
        let Some(id) = string_field(object, &["id", "material_id"]).map(str::to_string) else {
            continue;
        };

        let raw_text = object
            .get("content")
            .and_then(extract_text_from_content)
            .or_else(|| string_field(object, &["text", "words"]).map(str::to_string));

        let Some(text) = raw_text.map(|value| normalize_caption_text(&value)) else {
            continue;
        };

        if text.is_empty() {
            continue;
        }

        materials.insert(id, (text, contains_caption_signal(item)));
    }

    materials
}

fn timerange(segment: &Map<String, Value>) -> Option<(u64, u64)> {
    let source = segment
        .get("target_timerange")
        .or_else(|| segment.get("timerange"))
        .or_else(|| segment.get("source_timerange"));

    if let Some(timerange) = source.and_then(Value::as_object) {
        let start = numeric_field(timerange, &["start", "start_time", "startTime"])?;
        let duration = numeric_field(timerange, &["duration"])?;
        return Some((start, duration));
    }

    let start = numeric_field(segment, &["start", "start_time", "startTime"])?;
    let duration = numeric_field(segment, &["duration"]).or_else(|| {
        let end = numeric_field(segment, &["end", "end_time", "endTime"])?;
        Some(end.saturating_sub(start))
    })?;
    Some((start, duration))
}

fn format_timestamp(ms: u64) -> String {
    let total_seconds = ms / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn convert_time_units(items: &mut [TranscriptItem]) {
    let max_end = items.iter().map(|item| item.end_ms).max().unwrap_or(0);

    if max_end <= 86_400_000 {
        for item in items {
            item.timestamp = format_timestamp(item.start_ms);
        }
        return;
    }

    for item in items {
        item.start_ms /= 1000;
        item.end_ms /= 1000;
        item.duration_ms /= 1000;
        item.timestamp = format_timestamp(item.start_ms);
    }
}

fn extract_transcript(value: &Value) -> Vec<TranscriptItem> {
    let materials = text_materials(value);
    let mut items = Vec::new();
    let mut seen_segments = HashSet::new();

    let Some(tracks) = value.get("tracks").and_then(Value::as_array) else {
        return items;
    };

    for track in tracks {
        let track_has_caption_signal = contains_caption_signal(track);
        let Some(segments) = track.get("segments").and_then(Value::as_array) else {
            continue;
        };

        for segment in segments {
            let Some(segment_object) = segment.as_object() else {
                continue;
            };

            let Some(material_id) = string_field(segment_object, &["material_id", "materialId", "id"]) else {
                continue;
            };

            let Some((text, material_has_caption_signal)) = materials.get(material_id) else {
                continue;
            };

            if !track_has_caption_signal && !*material_has_caption_signal && !contains_caption_signal(segment) {
                continue;
            }

            let Some((start, duration)) = timerange(segment_object) else {
                continue;
            };

            let segment_id = string_field(segment_object, &["id"])
                .map(str::to_string)
                .unwrap_or_else(|| format!("{}:{}:{}", material_id, start, duration));

            if !seen_segments.insert(segment_id.clone()) {
                continue;
            }

            items.push(TranscriptItem {
                id: segment_id,
                start_ms: start,
                duration_ms: duration,
                end_ms: start.saturating_add(duration),
                timestamp: String::new(),
                text: text.clone(),
            });
        }
    }

    convert_time_units(&mut items);
    items.sort_by(|a, b| a.start_ms.cmp(&b.start_ms));
    items
}

#[tauri::command]
fn scan_capcut_projects() -> Result<ScanResult, ScanError> {
    let root_path = get_capcut_projects_path()?;
    let root_path_string = root_path.to_string_lossy().to_string();

    println!("[CapCut Scanner]");
    println!("OS: {}", env::consts::OS);
    println!("Root: {}", root_path_string);

    if !root_path.is_dir() {
        println!("Directories scanned: 0");
        println!("Valid projects: 0");

        return Err(ScanError {
            message: "Folder project CapCut tidak ditemukan pada komputer ini.".to_string(),
            root_path: Some(root_path_string),
        });
    }

    let (projects, directories_scanned) = read_project_directories(&root_path).map_err(|_| {
        ScanError {
            message: "Direktori project CapCut tidak dapat dibaca.".to_string(),
            root_path: Some(root_path_string.clone()),
        }
    })?;

    println!("Directories scanned: {}", directories_scanned);
    println!("Valid projects: {}", projects.len());

    Ok(ScanResult {
        root_path: root_path_string,
        projects,
    })
}

#[tauri::command]
fn read_capcut_project(project_path: String) -> Result<ProjectReadResult, ProjectReadError> {
    let canonical_project = canonical_project_path(project_path)?;

    let info_path = canonical_project.join("draft_info.json");
    let content_path = canonical_project.join("draft_content.json");

    if !info_path.is_file() {
        return Err(ProjectReadError {
            message: "Tidak dapat membaca project CapCut.".to_string(),
            detail: "draft_info.json tidak ditemukan.".to_string(),
        });
    }

    let info_text = fs::read_to_string(&info_path).map_err(|_| ProjectReadError {
        message: "Tidak dapat membaca project CapCut.".to_string(),
        detail: "draft_info.json tidak dapat dibaca.".to_string(),
    })?;

    let _: Value = serde_json::from_str(&info_text).map_err(|_| ProjectReadError {
        message: "Tidak dapat membaca project CapCut.".to_string(),
        detail: "draft_info.json bukan JSON yang valid.".to_string(),
    })?;

    let project = project_from_directory(canonical_project.clone()).ok_or_else(|| ProjectReadError {
        message: "Tidak dapat membaca project CapCut.".to_string(),
        detail: "Folder project tidak valid.".to_string(),
    })?;

    let has_draft_content = content_path.is_file();

    println!("[CapCut Project Reader]");
    println!("Project: {}", project.folder_name);
    println!("Path: {}", canonical_project.to_string_lossy());
    println!("draft_info.json: FOUND");
    println!(
        "draft_content.json: {}",
        if has_draft_content { "FOUND" } else { "NOT FOUND" }
    );

    Ok(ProjectReadResult {
        project,
        has_draft_info: true,
        has_draft_content,
    })
}

#[tauri::command]
fn read_capcut_transcript(project_path: String) -> Result<CapCutTranscriptResult, TranscriptError> {
    let canonical_project = canonical_project_path(project_path)?;
    let content_path = canonical_project.join("draft_content.json");
    let info_path = canonical_project.join("draft_info.json");
    let project_id = canonical_project
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "-".to_string());

    println!("[CapCut Transcript Parser]");
    println!("Project: {}", project_id);

    let caption_source_path = if content_path.is_file() {
        println!("draft_content.json: FOUND");
        content_path
    } else {
        println!("draft_content.json: NOT FOUND");
        println!("draft_info.json: USING AS FALLBACK");
        info_path
    };

    let file_size = fs::metadata(&caption_source_path)
        .map(|metadata| metadata.len())
        .unwrap_or(0);
    println!("File size: {:.1} MB", file_size as f64 / 1_048_576.0);

    let content_text = fs::read_to_string(&caption_source_path).map_err(|_| ProjectReadError {
        message: "Caption belum dapat dibaca dari project ini.".to_string(),
        detail: "File project CapCut tidak dapat dibaca.".to_string(),
    })?;

    let content_json: Value = serde_json::from_str(&content_text).map_err(|_| ProjectReadError {
        message: "Caption belum dapat dibaca dari project ini.".to_string(),
        detail: "File project CapCut bukan JSON yang valid.".to_string(),
    })?;

    inspect_draft_content(&content_json);

    let items = extract_transcript(&content_json);
    let duration_ms = items.iter().map(|item| item.end_ms).max().unwrap_or(0);

    println!("Valid captions: {}", items.len());
    if let Some(first) = items.first() {
        println!("First caption start: {}", first.timestamp);
        println!("First caption text length: {}", first.text.len());
    }
    if let Some(last) = items.last() {
        println!("Last caption start: {}", last.timestamp);
        println!("Last caption text length: {}", last.text.len());
    }

    Ok(CapCutTranscriptResult {
        project_id,
        caption_file_found: true,
        caption_count: items.len(),
        duration_ms,
        duration: format_timestamp(duration_ms),
        items,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scan_capcut_projects,
            read_capcut_project,
            read_capcut_transcript
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
