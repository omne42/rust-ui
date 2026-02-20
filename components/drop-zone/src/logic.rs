#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DroppedFile {
    pub name: String,
    pub size: u64,
    pub mime: String,
}

pub const DEFAULT_ARIA_LABEL: &str = "Drop files";

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_labels(
    label: Option<String>,
    aria_label: Option<String>,
) -> (Option<String>, String, bool) {
    let label = normalize_optional_text(label);
    let aria_label = normalize_optional_text(aria_label)
        .or_else(|| label.clone())
        .unwrap_or_else(|| DEFAULT_ARIA_LABEL.into());
    let has_custom_aria_label = aria_label != DEFAULT_ARIA_LABEL;

    (label, aria_label, has_custom_aria_label)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct DragDepth {
    depth: usize,
}

impl DragDepth {
    pub fn enter(self) -> Self {
        Self {
            depth: self.depth.saturating_add(1),
        }
    }

    pub fn leave(self) -> Self {
        Self {
            depth: self.depth.saturating_sub(1),
        }
    }

    pub fn reset(self) -> Self {
        Self { depth: 0 }
    }

    pub fn is_active(self) -> bool {
        self.depth > 0
    }
}

#[cfg(target_arch = "wasm32")]
fn collect_files_from_data_transfer(dt: &leptos::web_sys::DataTransfer) -> Vec<DroppedFile> {
    let Some(files) = dt.files() else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for idx in 0..files.length() {
        let Some(file) = files.get(idx) else {
            continue;
        };
        out.push(DroppedFile {
            name: file.name(),
            size: file.size().max(0.0) as u64,
            mime: file.type_(),
        });
    }
    out
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn collect_files_from_drag_event(ev: &leptos::ev::DragEvent) -> Vec<DroppedFile> {
    let Some(dt) = ev.data_transfer() else {
        return Vec::new();
    };
    collect_files_from_data_transfer(&dt)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn collect_files_from_clipboard_event(
    ev: &leptos::ev::ClipboardEvent,
) -> Vec<DroppedFile> {
    let Some(dt) = ev.clipboard_data() else {
        return Vec::new();
    };
    collect_files_from_data_transfer(&dt)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn collect_files_from_drag_event(_ev: &leptos::ev::DragEvent) -> Vec<DroppedFile> {
    Vec::new()
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn collect_files_from_clipboard_event(
    _ev: &leptos::ev::ClipboardEvent,
) -> Vec<DroppedFile> {
    Vec::new()
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
