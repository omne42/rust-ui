pub use ui_state_primitives::asset::{
    AssetState, AssetStateInput, AssetVariant, DEFAULT_CUSTOM_LABEL, DEFAULT_FILE_LABEL,
    DEFAULT_FOLDER_LABEL, compose_class_name, normalize_optional_text, resolve_label,
    resolve_state,
};
pub use ui_state_primitives::thumbnail::ThumbnailSize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetResolvedInput {
    pub variant: AssetVariant,
    pub size: ThumbnailSize,
    pub is_selected: bool,
    pub is_focused: bool,
    pub label: Option<String>,
    pub class_name: Option<String>,
    pub has_children: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetResolvedViewState {
    pub label: String,
    pub class_name: String,
    pub state: AssetState,
}

pub fn resolve_view_state(input: AssetResolvedInput) -> AssetResolvedViewState {
    let normalized_label = normalize_optional_text(input.label);
    let has_custom_label = normalized_label.is_some();
    let label = resolve_label(normalized_label, input.variant);

    let class_name = normalize_optional_text(input.class_name);
    let has_custom_content = input.has_children && input.variant == AssetVariant::Custom;

    let state = resolve_state(AssetStateInput {
        variant: input.variant,
        size: input.size,
        selected: input.is_selected,
        focused: input.is_focused,
        has_custom_label,
        has_custom_class_name: class_name.is_some(),
        has_custom_content,
    });

    let class_name = compose_class_name(class_name, state);

    AssetResolvedViewState {
        label,
        class_name,
        state,
    }
}

const _: Option<AssetState> = None;
const _: &str = DEFAULT_FILE_LABEL;
const _: &str = DEFAULT_CUSTOM_LABEL;
const _: &str = DEFAULT_FOLDER_LABEL;

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
