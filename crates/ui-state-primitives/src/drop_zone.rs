pub const DEFAULT_ARIA_LABEL: &str = "Drop files";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DropZoneLabels {
    pub label: Option<String>,
    pub aria_label: String,
    pub has_custom_aria_label: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DragDepth {
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

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_labels(label: Option<String>, aria_label: Option<String>) -> DropZoneLabels {
    let label = normalize_optional_text(label);
    let aria_label = normalize_optional_text(aria_label)
        .or_else(|| label.clone())
        .unwrap_or_else(|| DEFAULT_ARIA_LABEL.into());
    let has_custom_aria_label = aria_label != DEFAULT_ARIA_LABEL;

    DropZoneLabels {
        label,
        aria_label,
        has_custom_aria_label,
    }
}

#[cfg(test)]
#[path = "test/drop_zone.rs"]
mod tests;
