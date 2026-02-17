use ui_state_primitives::button_copy::{
    ButtonCopyStateInput, normalize_optional_text as normalize_state_text,
    resolve_state as resolve_copy_state,
};

pub const DEFAULT_COPY_LABEL: &str = "Copy";
pub const DEFAULT_COPIED_LABEL: &str = "Copied";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonCopyMode {
    #[default]
    TextOnly,
    IconOnly,
    IconAndText,
}

impl ButtonCopyMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::TextOnly => "text-only",
            Self::IconOnly => "icon-only",
            Self::IconAndText => "icon-and-text",
        }
    }

    pub fn shows_text(self) -> bool {
        !matches!(self, Self::IconOnly)
    }

    pub fn shows_icon(self) -> bool {
        !matches!(self, Self::TextOnly)
    }

    pub fn is_icon_only(self) -> bool {
        matches!(self, Self::IconOnly)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonCopyViewState {
    pub is_copyable: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_text: bool,
    pub mode: ButtonCopyMode,
    pub mode_attr: &'static str,
    pub shows_text: bool,
    pub shows_icon: bool,
    pub is_icon_only: bool,
    pub has_custom_label: bool,
    pub has_custom_copied_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ButtonCopyTextContract {
    pub label: String,
    pub copied_label: String,
    pub aria_label: String,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    normalize_state_text(value)
}

pub fn resolve_text_contract(
    label: Option<String>,
    copied_label: Option<String>,
    aria_label: Option<String>,
) -> ButtonCopyTextContract {
    let label = normalize_optional_text(label).unwrap_or_else(|| DEFAULT_COPY_LABEL.to_string());
    let copied_label =
        normalize_optional_text(copied_label).unwrap_or_else(|| DEFAULT_COPIED_LABEL.to_string());
    let aria_label = normalize_optional_text(aria_label).unwrap_or_else(|| label.clone());

    ButtonCopyTextContract {
        label,
        copied_label,
        aria_label,
    }
}

pub fn resolve_view_state(
    text: &str,
    disabled: bool,
    mode: ButtonCopyMode,
    has_custom_label: bool,
    has_custom_copied_label: bool,
    has_custom_aria_label: bool,
    has_custom_class_name: bool,
) -> ButtonCopyViewState {
    let state = resolve_copy_state(ButtonCopyStateInput {
        text,
        is_disabled: disabled,
    });

    ButtonCopyViewState {
        is_copyable: state.is_copyable,
        is_disabled: state.is_disabled,
        is_enabled: state.is_enabled,
        has_text: state.has_text,
        mode,
        mode_attr: mode.as_attr(),
        shows_text: mode.shows_text(),
        shows_icon: mode.shows_icon(),
        is_icon_only: mode.is_icon_only(),
        has_custom_label,
        has_custom_copied_label,
        has_custom_aria_label,
        has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ButtonCopyViewState) -> String {
    let mut classes = vec!["ui-button-copy".to_string()];

    if state.is_copyable {
        classes.push("ui-button-copy--copyable".to_string());
    }
    if state.is_disabled {
        classes.push("ui-button-copy--disabled".to_string());
    }
    if !state.has_text {
        classes.push("ui-button-copy--empty".to_string());
    }
    if state.has_custom_label {
        classes.push("ui-button-copy--custom-label".to_string());
    }
    if state.has_custom_copied_label {
        classes.push("ui-button-copy--custom-copied-label".to_string());
    }
    match state.mode {
        ButtonCopyMode::TextOnly => classes.push("ui-button-copy--text-only".to_string()),
        ButtonCopyMode::IconOnly => classes.push("ui-button-copy--icon-only".to_string()),
        ButtonCopyMode::IconAndText => classes.push("ui-button-copy--icon-and-text".to_string()),
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_trims_and_drops_blank_values() {
        assert_eq!(
            normalize_optional_text(Some("  Copy now  ".to_string())),
            Some("Copy now".to_string())
        );
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
        assert_eq!(normalize_optional_text(None), None);
    }

    #[test]
    fn resolve_text_contract_uses_defaults_when_values_missing() {
        let contract = resolve_text_contract(None, None, None);

        assert_eq!(contract.label, DEFAULT_COPY_LABEL);
        assert_eq!(contract.copied_label, DEFAULT_COPIED_LABEL);
        assert_eq!(contract.aria_label, DEFAULT_COPY_LABEL);
    }

    #[test]
    fn resolve_text_contract_prefers_custom_values_when_present() {
        let contract = resolve_text_contract(
            Some("  Copy URL  ".to_string()),
            Some("  URL copied  ".to_string()),
            Some("  Copy URL to clipboard  ".to_string()),
        );

        assert_eq!(contract.label, "Copy URL");
        assert_eq!(contract.copied_label, "URL copied");
        assert_eq!(contract.aria_label, "Copy URL to clipboard");
    }

    #[test]
    fn resolve_text_contract_falls_back_aria_to_resolved_label() {
        let contract = resolve_text_contract(Some("  Install  ".to_string()), None, None);

        assert_eq!(contract.label, "Install");
        assert_eq!(contract.copied_label, DEFAULT_COPIED_LABEL);
        assert_eq!(contract.aria_label, "Install");
    }

    #[test]
    fn button_copy_mode_contract_exposes_expected_flags() {
        let text_only = resolve_view_state(
            "",
            false,
            ButtonCopyMode::TextOnly,
            false,
            false,
            false,
            false,
        );
        assert_eq!(text_only.mode_attr, "text-only");
        assert!(text_only.shows_text);
        assert!(!text_only.shows_icon);
        assert!(!text_only.is_icon_only);

        let icon_only = resolve_view_state(
            "",
            false,
            ButtonCopyMode::IconOnly,
            false,
            false,
            false,
            false,
        );
        assert_eq!(icon_only.mode_attr, "icon-only");
        assert!(!icon_only.shows_text);
        assert!(icon_only.shows_icon);
        assert!(icon_only.is_icon_only);
    }

    #[test]
    fn empty_text_is_not_copyable() {
        assert!(
            !resolve_view_state(
                "",
                false,
                ButtonCopyMode::IconAndText,
                false,
                false,
                false,
                false
            )
            .is_copyable
        );
        assert!(
            !resolve_view_state(
                "   ",
                false,
                ButtonCopyMode::IconAndText,
                false,
                false,
                false,
                false
            )
            .is_copyable
        );
    }

    #[test]
    fn disabled_is_not_copyable_even_when_text_present() {
        assert!(
            !resolve_view_state(
                "hello",
                true,
                ButtonCopyMode::IconAndText,
                false,
                false,
                false,
                false
            )
            .is_copyable
        );
    }

    #[test]
    fn enabled_with_text_is_copyable() {
        assert!(
            resolve_view_state(
                "hello",
                false,
                ButtonCopyMode::IconAndText,
                false,
                false,
                false,
                false
            )
            .is_copyable
        );
    }

    #[test]
    fn resolve_view_state_tracks_metadata_flags() {
        let state = resolve_view_state(
            "hello",
            false,
            ButtonCopyMode::IconAndText,
            true,
            true,
            true,
            true,
        );
        assert!(state.is_copyable);
        assert!(!state.is_disabled);
        assert!(state.is_enabled);
        assert!(state.has_text);
        assert_eq!(state.mode_attr, "icon-and-text");
        assert!(state.shows_text);
        assert!(state.shows_icon);
        assert!(!state.is_icon_only);
        assert!(state.has_custom_label);
        assert!(state.has_custom_copied_label);
        assert!(state.has_custom_aria_label);
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_view_state(
                "hello",
                false,
                ButtonCopyMode::IconAndText,
                true,
                true,
                false,
                true,
            ),
        );

        for token in [
            "ui-button-copy",
            "ui-button-copy--copyable",
            "ui-button-copy--custom-label",
            "ui-button-copy--custom-copied-label",
            "ui-button-copy--icon-and-text",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
