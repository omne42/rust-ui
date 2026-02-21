use super::super::{ButtonType, logic as button_logic};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchInputButtonStateInput {
    pub is_disabled: bool,
    pub has_shortcut: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_compact_placeholder: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchInputButtonState {
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub state_attr: &'static str,
    pub state_class: &'static str,
    pub has_shortcut: bool,
    pub shortcut_attr: &'static str,
    pub shortcut_class: &'static str,
    pub has_custom_placeholder: bool,
    pub placeholder_source_attr: &'static str,
    pub placeholder_source_class: &'static str,
    pub has_custom_compact_placeholder: bool,
    pub compact_placeholder_source_attr: &'static str,
    pub compact_placeholder_source_class: &'static str,
    pub has_custom_aria_label: bool,
    pub aria_label_source_attr: &'static str,
    pub aria_label_source_class: &'static str,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchInputButtonViewState {
    pub placeholder: String,
    pub compact_placeholder: String,
    pub meta_key_label: Option<String>,
    pub key_label: Option<String>,
    pub show_shortcut: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchInputButtonAriaLabelResolution {
    pub aria_label: String,
    pub has_custom_aria_label: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    button_logic::normalize_optional_text(value)
}

pub fn resolve_button_type(button_type: Option<ButtonType>) -> ButtonType {
    button_type.unwrap_or_default()
}

pub fn resolve_effective_aria_label(
    aria_label: Option<String>,
    placeholder: &str,
) -> SearchInputButtonAriaLabelResolution {
    match normalize_optional_text(aria_label) {
        Some(aria_label) => SearchInputButtonAriaLabelResolution {
            aria_label,
            has_custom_aria_label: true,
        },
        None => SearchInputButtonAriaLabelResolution {
            aria_label: placeholder.into(),
            has_custom_aria_label: false,
        },
    }
}

pub fn resolve_state(input: SearchInputButtonStateInput) -> SearchInputButtonState {
    let is_disabled = input.is_disabled;

    let (state_attr, state_class) = if is_disabled {
        ("disabled", "ui-search-input-button--disabled")
    } else {
        ("enabled", "ui-search-input-button--enabled")
    };

    let (shortcut_attr, shortcut_class) = if input.has_shortcut {
        ("visible", "ui-search-input-button--with-shortcut")
    } else {
        ("hidden", "ui-search-input-button--without-shortcut")
    };

    let (placeholder_source_attr, placeholder_source_class) = if input.has_custom_placeholder {
        ("custom", "ui-search-input-button--custom-placeholder")
    } else {
        ("default", "ui-search-input-button--default-placeholder")
    };

    let (compact_placeholder_source_attr, compact_placeholder_source_class) =
        if input.has_custom_compact_placeholder {
            (
                "custom",
                "ui-search-input-button--custom-compact-placeholder",
            )
        } else {
            (
                "default",
                "ui-search-input-button--default-compact-placeholder",
            )
        };

    let (aria_label_source_attr, aria_label_source_class) = if input.has_custom_aria_label {
        ("custom", "ui-search-input-button--custom-aria-label")
    } else {
        (
            "placeholder",
            "ui-search-input-button--placeholder-aria-label",
        )
    };

    SearchInputButtonState {
        is_disabled,
        is_enabled: !is_disabled,
        state_attr,
        state_class,
        has_shortcut: input.has_shortcut,
        shortcut_attr,
        shortcut_class,
        has_custom_placeholder: input.has_custom_placeholder,
        placeholder_source_attr,
        placeholder_source_class,
        has_custom_compact_placeholder: input.has_custom_compact_placeholder,
        compact_placeholder_source_attr,
        compact_placeholder_source_class,
        has_custom_aria_label: input.has_custom_aria_label,
        aria_label_source_attr,
        aria_label_source_class,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn resolve_view_state(
    placeholder: Option<&str>,
    compact_placeholder: Option<&str>,
    meta_key_label: Option<&str>,
    key_label: Option<&str>,
    fallback_placeholder: &str,
) -> SearchInputButtonViewState {
    let placeholder = placeholder
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(fallback_placeholder)
        .to_string();

    let compact_placeholder = compact_placeholder
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(placeholder.as_str())
        .to_string();

    let meta_key_label = meta_key_label
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(Into::into);

    let key_label = key_label
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(Into::into);

    let show_shortcut = meta_key_label.is_some() && key_label.is_some();

    SearchInputButtonViewState {
        placeholder,
        compact_placeholder,
        meta_key_label,
        key_label,
        show_shortcut,
    }
}

pub fn resolve_shortcut_labels(
    meta_key_label: Option<String>,
    key_label: Option<String>,
) -> (String, String) {
    (
        meta_key_label.unwrap_or_default(),
        key_label.unwrap_or_default(),
    )
}

pub fn compose_class_name(
    base_class_name: Option<String>,
    state: SearchInputButtonState,
) -> String {
    let mut classes = vec![
        "ui-search-input-button".to_string(),
        state.state_class.into(),
        state.shortcut_class.into(),
        state.placeholder_source_class.into(),
        state.compact_placeholder_source_class.into(),
        state.aria_label_source_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-search-input-button--custom-class".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/search_input/logic.rs"]
mod tests;
