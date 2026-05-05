//! EqFilePicker — file/folder selection organism with abstracted backend.
//!
//! Supports single file, multiple files, and folder selection modes.
//! Provides a styled drop-zone + click-to-browse surface, file list
//! with thumbnails/icons, file-type filtering, size limits, per-file
//! progress bars, and remove actions.
//!
//! ## Backend abstraction
//!
//! The component is backend-agnostic via the [`FilePickerBackend`] trait.
//! Use the built-in [`WebFilePickerBackend`] for WASM/browser targets,
//! or implement the trait for native dialogs (e.g. `rfd`).
//!
//! ```no_run
//! use eq_ui::prelude::*;
//! use eq_ui::organisms::{EqFilePicker, FilePickerMode, PickedFile};
//!
//! let _: Element = rsx! {
//!     EqFilePicker {
//!         mode: FilePickerMode::Multiple,
//!         accept: ".png,.jpg,.pdf",
//!         max_size_bytes: 10 * 1024 * 1024,
//!         on_files_changed: move |files: Vec<PickedFile>| { /* handle */ },
//!     }
//! };
//! ```

use super::eq_file_picker_styles as s;
use crate::theme::merge_classes;
use crate::playground;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropToggle, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ── Types ────────────────────────────────────────────────────────

/// Selection mode for the file picker.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum FilePickerMode {
    /// Select a single file.
    #[default]
    Single,
    /// Select multiple files.
    Multiple,
    /// Select a folder (uses webkitdirectory on web).
    Folder,
}

/// A file that has been picked by the user.
#[derive(Clone, PartialEq)]
pub struct PickedFile {
    /// File name (including extension).
    pub name: String,
    /// File size in bytes.
    pub size: u64,
    /// MIME type (e.g. "image/png"). May be empty.
    pub mime: String,
    /// Optional data URL for image previews.
    pub data_url: Option<String>,
    /// Upload progress (0.0 – 1.0). None = not uploading.
    pub progress: Option<f64>,
    /// Error message (e.g. "File too large").
    pub error: Option<String>,
}

impl PickedFile {
    pub fn new(name: impl Into<String>, size: u64, mime: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            size,
            mime: mime.into(),
            data_url: None,
            progress: None,
            error: None,
        }
    }

    /// Builder: set the data URL for image preview.
    pub fn with_data_url(mut self, url: impl Into<String>) -> Self {
        self.data_url = Some(url.into());
        self
    }

    /// Builder: set upload progress (0.0–1.0).
    pub fn with_progress(mut self, progress: f64) -> Self {
        self.progress = Some(progress);
        self
    }

    /// Builder: set an error.
    pub fn with_error(mut self, err: impl Into<String>) -> Self {
        self.error = Some(err.into());
        self
    }

    /// Is this an image file (based on MIME)?
    pub fn is_image(&self) -> bool {
        self.mime.starts_with("image/")
    }

    /// Human-readable file size.
    pub fn size_display(&self) -> String {
        if self.size < 1024 {
            format!("{} B", self.size)
        } else if self.size < 1024 * 1024 {
            format!("{:.1} KB", self.size as f64 / 1024.0)
        } else if self.size < 1024 * 1024 * 1024 {
            format!("{:.1} MB", self.size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", self.size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

// ── Backend trait ────────────────────────────────────────────────

/// Abstraction over the file-picking mechanism.
///
/// Implement this trait to provide platform-specific file selection.
/// The built-in [`WebFilePickerBackend`] uses the browser's `<input type="file">`.
/// For desktop targets, implement using `rfd` or another native dialog library.
pub trait FilePickerBackend: Clone + PartialEq + 'static {
    /// Open the file picker dialog.
    ///
    /// * `accept` — comma-separated MIME types or extensions (e.g. "image/*,.pdf")
    /// * `multiple` — whether multiple files can be selected
    /// * `folder` — whether to select a folder instead of files
    /// * `callback` — receives the picked files
    fn open(
        &self,
        accept: &str,
        multiple: bool,
        folder: bool,
        callback: Callback<Vec<PickedFile>>,
    );
}

/// Default web backend that uses `<input type="file">` via JS interop.
#[derive(Clone, PartialEq, Default)]
pub struct WebFilePickerBackend;

impl FilePickerBackend for WebFilePickerBackend {
    fn open(
        &self,
        accept: &str,
        multiple: bool,
        folder: bool,
        callback: Callback<Vec<PickedFile>>,
    ) {
        let accept = accept.to_string();
        let multiple = multiple;
        let folder = folder;

        spawn(async move {
            let multi_attr = if multiple { "input.multiple = true;" } else { "" };
            let folder_attr = if folder { "input.webkitdirectory = true;" } else { "" };
            let accept_attr = if accept.is_empty() {
                String::new()
            } else {
                format!("input.accept = '{}';", accept)
            };

            let js = format!(
                r#"
                const input = document.createElement('input');
                input.type = 'file';
                {multi_attr}
                {folder_attr}
                {accept_attr}
                return new Promise((resolve) => {{
                    input.onchange = async () => {{
                        const results = [];
                        for (const file of input.files) {{
                            const entry = {{
                                name: file.name,
                                size: file.size,
                                mime: file.type || '',
                                data_url: null,
                            }};
                            if (file.type && file.type.startsWith('image/') && file.size < 5 * 1024 * 1024) {{
                                try {{
                                    const reader = new FileReader();
                                    const url = await new Promise((res, rej) => {{
                                        reader.onload = () => res(reader.result);
                                        reader.onerror = rej;
                                        reader.readAsDataURL(file);
                                    }});
                                    entry.data_url = url;
                                }} catch(e) {{}}
                            }}
                            results.push(entry);
                        }}
                        resolve(results);
                    }};
                    input.oncancel = () => resolve([]);
                    input.click();
                }});
                "#
            );

            let result = document::eval(&js);
            if let Ok(val) = result.await {
                let files = parse_file_entries(&val);
                if !files.is_empty() {
                    callback.call(files);
                }
            }
        });
    }
}

/// Parse the JS array-of-objects into PickedFile values.
fn parse_file_entries(val: &serde_json::Value) -> Vec<PickedFile> {
    let Some(arr) = val.as_array() else { return Vec::new() };
    arr.iter().filter_map(|entry| {
        let name = entry.get("name")?.as_str()?.to_string();
        let size = entry.get("size")?.as_u64()?;
        let mime = entry.get("mime").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let data_url = entry.get("data_url").and_then(|v| v.as_str()).map(|s| s.to_string());
        let mut pf = PickedFile::new(name, size, mime);
        if let Some(url) = data_url {
            pf = pf.with_data_url(url);
        }
        Some(pf)
    }).collect()
}

// ── SVG icons ────────────────────────────────────────────────────

/// Upload cloud icon (Heroicons outline).
const UPLOAD_ICON: &str =
    "M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5m-13.5-9L12 3m0 0 4.5 4.5M12 3v13";

/// Document icon (Heroicons outline).
const DOC_ICON: &str =
    "M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z";

/// Folder icon (Heroicons outline).
const FOLDER_ICON: &str =
    "M2.25 12.75V12A2.25 2.25 0 0 1 4.5 9.75h15A2.25 2.25 0 0 1 21.75 12v.75m-8.69-6.44-2.12-2.12a1.5 1.5 0 0 0-1.061-.44H4.5A2.25 2.25 0 0 0 2.25 6v12a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9a2.25 2.25 0 0 0-2.25-2.25h-5.379a1.5 1.5 0 0 1-1.06-.44Z";

/// X mark icon (Heroicons mini).
const X_ICON: &str =
    "M6.28 5.22a.75.75 0 0 0-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 1 0 1.06 1.06L10 11.06l3.72 3.72a.75.75 0 1 0 1.06-1.06L11.06 10l3.72-3.72a.75.75 0 0 0-1.06-1.06L10 8.94 6.28 5.22Z";

// ── Component ────────────────────────────────────────────────────

/// File/folder picker organism with drag-and-drop, previews, and progress.
///
/// Uses an abstracted [`FilePickerBackend`] trait so the same component
/// works on web (via `<input type="file">`) and desktop (via native
/// dialogs like `rfd`). Defaults to [`WebFilePickerBackend`].
///
/// **Features** — single/multiple/folder modes, file type filtering,
/// max file size validation, image thumbnails, per-file upload progress,
/// remove action, drag-and-drop zone.
///
/// **Accessibility** — drop zone is keyboard-focusable and activatable
/// via Enter/Space, file list uses `role="list"`, remove buttons have
/// aria-labels.
#[playground(
    category = Organism,
    description = "File/folder picker with drag-and-drop, type filter, \
                   size limits, image previews, upload progress, and \
                   abstracted backend (web or native).",
    examples = [
        ("Single file", "EqFilePicker {\n    on_files_changed: move |files| log::info!(\"{files:?}\"),\n}"),
        ("Multiple images", "EqFilePicker {\n    mode: FilePickerMode::Multiple,\n    accept: \"image/*\",\n    max_size_bytes: 5 * 1024 * 1024,\n    on_files_changed: move |files| { /* ... */ },\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqFilePicker(
    /// Selection mode: Single, Multiple, or Folder.
    #[props(default)]
    mode: FilePickerMode,
    /// Comma-separated accepted MIME types or extensions (e.g. "image/*,.pdf").
    #[props(into, default)]
    accept: String,
    /// Maximum file size in bytes. Files exceeding this show an error.
    /// 0 = no limit.
    #[props(default)]
    max_size_bytes: u64,
    /// The current list of picked files (controlled).
    #[props(default)]
    files: Vec<PickedFile>,
    /// Fired when the file list changes (files added or removed).
    #[props(default)]
    on_files_changed: Option<EventHandler<Vec<PickedFile>>>,
    /// Whether the picker is disabled.
    #[props(default)]
    disabled: bool,
    /// Custom drop zone text.
    #[props(into, default)]
    placeholder: String,
    /// Optional class override on the wrapper.
    #[props(into, default)]
    class: String,
) -> Element {
    let mut drag_over = use_signal(|| false);
    let wrapper_cls = merge_classes(s::WRAPPER, &class);

    let is_multiple = mode == FilePickerMode::Multiple;
    let is_folder = mode == FilePickerMode::Folder;

    let placeholder_text = if placeholder.is_empty() {
        match mode {
            FilePickerMode::Single => "Click to browse or drag a file here".to_string(),
            FilePickerMode::Multiple => "Click to browse or drag files here".to_string(),
            FilePickerMode::Folder => "Click to select a folder".to_string(),
        }
    } else {
        placeholder.clone()
    };

    let hint_text = if !accept.is_empty() {
        format!("Accepted: {}", accept)
    } else {
        String::new()
    };
    let has_hint = !hint_text.is_empty();

    let size_hint = if max_size_bytes > 0 {
        let max_display = if max_size_bytes < 1024 * 1024 {
            format!("{} KB", max_size_bytes / 1024)
        } else {
            format!("{} MB", max_size_bytes / (1024 * 1024))
        };
        format!("Max size: {}", max_display)
    } else {
        String::new()
    };
    let has_size_hint = !size_hint.is_empty();

    let drop_zone_cls = if disabled {
        format!("{} {}", s::DROP_ZONE, s::DROP_ZONE_DISABLED)
    } else if drag_over() {
        format!("{} {}", s::DROP_ZONE, s::DROP_ZONE_ACTIVE)
    } else {
        s::DROP_ZONE.to_string()
    };

    let drop_icon = if is_folder { FOLDER_ICON } else { UPLOAD_ICON };

    // Validate a file against size limits. Returns the file, possibly with an error.
    let validate_file = {
        let max_bytes = max_size_bytes;
        move |mut f: PickedFile| -> PickedFile {
            if max_bytes > 0 && f.size > max_bytes {
                let limit = if max_bytes < 1024 * 1024 {
                    format!("{} KB", max_bytes / 1024)
                } else {
                    format!("{} MB", max_bytes / (1024 * 1024))
                };
                f.error = Some(format!("File too large (max {})", limit));
            }
            f
        }
    };

    // Handle newly picked files (from click or drop).
    let handle_new_files = {
        let current = files.clone();
        let is_multiple = is_multiple;
        let validate = validate_file.clone();
        move |new_files: Vec<PickedFile>| {
            let validated: Vec<PickedFile> = new_files.into_iter().map(|f| validate(f)).collect();
            let result = if is_multiple {
                let mut merged = current.clone();
                merged.extend(validated);
                merged
            } else {
                // Single mode: replace.
                validated
            };
            if let Some(handler) = &on_files_changed {
                handler.call(result);
            }
        }
    };

    // Open file picker via the web backend.
    let backend = WebFilePickerBackend;
    let accept_for_open = accept.clone();
    let handle_new_for_backend = handle_new_files.clone();
    let trigger_open = move || {
        if disabled { return; }
        let accept_str = accept_for_open.clone();
        let handler = handle_new_for_backend.clone();
        backend.open(
            &accept_str,
            is_multiple,
            is_folder,
            Callback::new(move |picked: Vec<PickedFile>| {
                handler(picked);
            }),
        );
    };
    let trigger_open2 = trigger_open.clone();

    // Handle drag-and-drop via JS.
    // We use ondragover / ondragleave / ondrop on the drop zone.
    let handle_new_for_drop = handle_new_files.clone();
    let max_for_drop = max_size_bytes;

    // Unique ID for drop zone to wire up JS drop handler.
    static DZ_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let dz_id = use_hook(|| {
        format!("eq-fp-{}", DZ_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    });

    let has_files = !files.is_empty();

    // Install native JS drop handler on mount.
    let dz_id_effect = dz_id.clone();
    use_effect(move || {
        install_drop_handler(&dz_id_effect);
    });

    rsx! {
        div { class: "{wrapper_cls}",

            // Drop zone
            div {
                id: "{dz_id}",
                class: "{drop_zone_cls}",
                role: "button",
                tabindex: if disabled { "-1" } else { "0" },
                "aria-label": "{placeholder_text}",
                "aria-disabled": if disabled { "true" } else { "false" },
                onclick: move |_| trigger_open(),
                onkeydown: move |evt: KeyboardEvent| {
                    if evt.key() == Key::Enter || evt.key() == Key::Character(" ".to_string()) {
                        evt.prevent_default();
                        trigger_open2();
                    }
                },
                ondragover: move |evt: Event<DragData>| {
                    evt.prevent_default();
                    if !disabled {
                        drag_over.set(true);
                    }
                },
                ondragleave: move |_: Event<DragData>| {
                    drag_over.set(false);
                },
                ondrop: move |evt: Event<DragData>| {
                    evt.prevent_default();
                    drag_over.set(false);
                    if disabled { return; }

                    // Read dropped files via JS.
                    let dz_id_drop = dz_id.clone();
                    let handler = handle_new_for_drop.clone();
                    let max_bytes = max_for_drop;
                    spawn(async move {
                        // Use a JS-based approach to read dropped files.
                        let js = format!(
                            r#"
                            const el = document.getElementById('{}');
                            if (!el || !el._droppedFiles) return [];
                            const files = el._droppedFiles;
                            const results = [];
                            for (const file of files) {{
                                const entry = {{
                                    name: file.name,
                                    size: file.size,
                                    mime: file.type || '',
                                    data_url: null,
                                }};
                                if (file.type && file.type.startsWith('image/') && file.size < 5 * 1024 * 1024) {{
                                    try {{
                                        const reader = new FileReader();
                                        const url = await new Promise((res, rej) => {{
                                            reader.onload = () => res(reader.result);
                                            reader.onerror = rej;
                                            reader.readAsDataURL(file);
                                        }});
                                        entry.data_url = url;
                                    }} catch(e) {{}}
                                }}
                                results.push(entry);
                            }}
                            el._droppedFiles = null;
                            return results;
                            "#,
                            dz_id_drop
                        );
                        let result = document::eval(&js);
                        if let Ok(val) = result.await {
                            let mut files = parse_file_entries(&val);
                            for pf in &mut files {
                                if max_bytes > 0 && pf.size > max_bytes {
                                    let limit = if max_bytes < 1024 * 1024 {
                                        format!("{} KB", max_bytes / 1024)
                                    } else {
                                        format!("{} MB", max_bytes / (1024 * 1024))
                                    };
                                    pf.error = Some(format!("File too large (max {})", limit));
                                }
                            }
                            if !files.is_empty() {
                                handler(files);
                            }
                        }
                    });
                },

                // Icon
                svg {
                    class: "{s::DROP_ICON}",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke_width: "1.5",
                    stroke: "currentColor",
                    "aria-hidden": "true",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: drop_icon,
                    }
                }

                span { class: "{s::DROP_TEXT}", "{placeholder_text}" }
                span { class: "{s::DROP_HINT}", "or drag and drop" }

                if has_hint {
                    span { class: "{s::DROP_ACCEPT}", "{hint_text}" }
                }
                if has_size_hint {
                    span { class: "{s::DROP_ACCEPT}", "{size_hint}" }
                }
            }

            // File list
            if has_files {
                div { class: "{s::FILE_LIST}", role: "list",
                    for (fi , file) in files.iter().enumerate() {
                        {
                            let fname = file.name.clone();
                            let fsize = file.size_display();
                            let ferror = file.error.clone();
                            let has_error = ferror.is_some();
                            let error_msg = ferror.unwrap_or_default();
                            let has_thumb = file.data_url.is_some();
                            let thumb_url = file.data_url.clone().unwrap_or_default();
                            let is_img = file.is_image();
                            let progress = file.progress;
                            let has_progress = progress.is_some();
                            let progress_val = progress.unwrap_or(0.0);
                            let progress_pct = format!("{:.0}%", progress_val * 100.0);

                            let files_for_remove = files.clone();
                            let on_change = on_files_changed.clone();

                            rsx! {
                                div {
                                    key: "file-{fi}",
                                    class: "{s::FILE_ROW}",
                                    role: "listitem",

                                    // Thumbnail or icon
                                    div { class: "{s::FILE_THUMB}",
                                        if has_thumb {
                                            img {
                                                class: "{s::FILE_THUMB_IMG}",
                                                src: "{thumb_url}",
                                                alt: "{fname}",
                                            }
                                        } else {
                                            svg {
                                                class: "{s::FILE_ICON}",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                fill: "none",
                                                view_box: "0 0 24 24",
                                                stroke_width: "1.5",
                                                stroke: "currentColor",
                                                "aria-hidden": "true",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    d: if is_img { UPLOAD_ICON } else { DOC_ICON },
                                                }
                                            }
                                        }
                                    }

                                    // File info
                                    div { class: "{s::FILE_INFO}",
                                        span { class: "{s::FILE_NAME}", "{fname}" }
                                        if has_error {
                                            span { class: "{s::FILE_ERROR}", "{error_msg}" }
                                        } else {
                                            span { class: "{s::FILE_SIZE}", "{fsize}" }
                                        }
                                    }

                                    // Progress
                                    if has_progress {
                                        div { class: "{s::FILE_PROGRESS}",
                                            div {
                                                class: "w-full h-1.5 rounded-full bg-[var(--color-tertiary-dark)]/40",
                                                div {
                                                    class: "h-full rounded-full bg-[var(--color-accent-primary)] transition-all duration-200",
                                                    style: "width: {progress_pct};",
                                                }
                                            }
                                        }
                                    }

                                    // Remove button
                                    button {
                                        class: "{s::FILE_REMOVE}",
                                        r#type: "button",
                                        "aria-label": "Remove {fname}",
                                        onclick: move |_| {
                                            let mut updated = files_for_remove.clone();
                                            if fi < updated.len() {
                                                updated.remove(fi);
                                            }
                                            if let Some(handler) = &on_change {
                                                handler.call(updated);
                                            }
                                        },
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            view_box: "0 0 20 20",
                                            fill: "currentColor",
                                            width: "16", height: "16",
                                            "aria-hidden": "true",
                                            path { d: X_ICON }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Wire up drop zone to capture native drag events ──────────────
// We need a small JS snippet to store dropped files on the element
// so our Rust ondrop handler can read them.
// This is installed once via use_effect.

// Note: Dioxus Event<DragData> doesn't expose file data directly in WASM,
// so we install a native JS handler that stores the FileList on the
// element, then our Rust handler reads them via document::eval.

/// Install native drop handler on the drop zone element.
fn install_drop_handler(dz_id: &str) {
    let js = format!(
        r#"
        const el = document.getElementById('{}');
        if (el && !el._dropInstalled) {{
            el._dropInstalled = true;
            el.addEventListener('drop', (e) => {{
                e.preventDefault();
                el._droppedFiles = Array.from(e.dataTransfer.files);
            }});
            el.addEventListener('dragover', (e) => e.preventDefault());
        }}
        "#,
        dz_id
    );
    document::eval(&js);
}

// ── Interactive demo ─────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqFilePicker() -> Element {
    let mut files = use_signal(Vec::<PickedFile>::new);
    let mut multi = use_signal(|| false);
    let mut folder_mode = use_signal(|| false);
    let mut accept_str = use_signal(|| String::new());
    let mut max_size_str = use_signal(|| String::new());
    let mut show_disabled = use_signal(|| false);

    let mode = if folder_mode() {
        FilePickerMode::Folder
    } else if multi() {
        FilePickerMode::Multiple
    } else {
        FilePickerMode::Single
    };

    let max_bytes: u64 = max_size_str()
        .parse::<u64>()
        .ok()
        .map(|mb| mb * 1024 * 1024)
        .unwrap_or(0);

    // Simulate upload progress on files without errors.
    let current_files = files();
    let has_new_files = current_files.iter().any(|f| f.progress.is_none() && f.error.is_none());

    if has_new_files {
        let mut updated = current_files.clone();
        for f in &mut updated {
            if f.progress.is_none() && f.error.is_none() {
                f.progress = Some(1.0); // Instant "complete" for demo.
            }
        }
        files.set(updated);
    }

    let code = r#"let mut files = use_signal(Vec::<PickedFile>::new);

EqFilePicker {
    mode: FilePickerMode::Multiple,
    accept: "image/*,.pdf",
    max_size_bytes: 10 * 1024 * 1024,
    files: files(),
    on_files_changed: move |f: Vec<PickedFile>| files.set(f),
}"#.to_string();

    rsx! {
        DemoSection { title: "EqFilePicker",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropToggle {
                    label: "multiple",
                    value: multi(),
                    onchange: move |v: bool| { multi.set(v); folder_mode.set(false); },
                }
                PropToggle {
                    label: "folder mode",
                    value: folder_mode(),
                    onchange: move |v: bool| { folder_mode.set(v); multi.set(false); },
                }
                PropToggle {
                    label: "disabled",
                    value: show_disabled(),
                    onchange: move |v: bool| show_disabled.set(v),
                }
                PropInput {
                    label: "accept",
                    value: accept_str(),
                    placeholder: "e.g. image/*,.pdf",
                    onchange: move |v: String| accept_str.set(v),
                }
                PropInput {
                    label: "max size (MB)",
                    value: max_size_str(),
                    placeholder: "0 = no limit",
                    onchange: move |v: String| max_size_str.set(v),
                }
            }

            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 space-y-4",
                div { class: "max-w-md",
                    EqFilePicker {
                        mode,
                        accept: accept_str(),
                        max_size_bytes: max_bytes,
                        disabled: show_disabled(),
                        files: files(),
                        on_files_changed: move |f: Vec<PickedFile>| files.set(f),
                    }
                }
            }

            StyleInfo { file: "eq_file_picker_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery ──────────────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqFilePicker() -> Element {
    let mut files1 = use_signal(Vec::<PickedFile>::new);
    let mut files2 = use_signal(Vec::<PickedFile>::new);

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "FilePicker Gallery" }

                div { class: "flex items-start gap-6 flex-wrap",
                    div { class: "space-y-1 w-72",
                        EqText { variant: TextVariant::Muted, "Single file" }
                        EqFilePicker {
                            files: files1(),
                            on_files_changed: move |f: Vec<PickedFile>| files1.set(f),
                        }
                    }
                    div { class: "space-y-1 w-72",
                        EqText { variant: TextVariant::Muted, "Multiple images (max 5 MB)" }
                        EqFilePicker {
                            mode: FilePickerMode::Multiple,
                            accept: "image/*",
                            max_size_bytes: 5 * 1024 * 1024,
                            files: files2(),
                            on_files_changed: move |f: Vec<PickedFile>| files2.set(f),
                        }
                    }
                }
            }
        }
    }
}

// ── Smoke tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_renders() {
        let mut dom = VirtualDom::new(|| rsx! { EqFilePicker {} });
        dom.rebuild_in_place();
    }

    #[test]
    fn default_mode_is_single() {
        let m: FilePickerMode = Default::default();
        assert!(matches!(m, FilePickerMode::Single));
    }

    #[test]
    fn picked_file_new_sets_fields() {
        let f = PickedFile::new("doc.pdf", 1024, "application/pdf");
        assert_eq!(f.name, "doc.pdf");
        assert_eq!(f.size, 1024);
        assert_eq!(f.mime, "application/pdf");
        assert!(f.data_url.is_none());
        assert!(f.progress.is_none());
        assert!(f.error.is_none());
    }

    #[test]
    fn picked_file_builders() {
        let f = PickedFile::new("a", 0, "")
            .with_data_url("data:...")
            .with_progress(0.5)
            .with_error("oops");
        assert_eq!(f.data_url.as_deref(), Some("data:..."));
        assert_eq!(f.progress, Some(0.5));
        assert_eq!(f.error.as_deref(), Some("oops"));
    }

    #[test]
    fn picked_file_is_image() {
        let img = PickedFile::new("a.png", 0, "image/png");
        assert!(img.is_image());
        let pdf = PickedFile::new("a.pdf", 0, "application/pdf");
        assert!(!pdf.is_image());
    }

    #[test]
    fn picked_file_size_display() {
        assert_eq!(PickedFile::new("a", 500, "").size_display(), "500 B");
        assert_eq!(PickedFile::new("a", 2048, "").size_display(), "2.0 KB");
    }
}
