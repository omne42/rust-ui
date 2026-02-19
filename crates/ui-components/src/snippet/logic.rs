use leptos::prelude::*;
use ui_headless::{A11yDirection, SnippetCopyOptions, use_snippet_copy};

pub use ui_state_primitives::snippet::{SnippetStateInput, normalize_optional_text, resolve_state};
pub type SnippetViewState = ui_state_primitives::snippet::SnippetState;

pub const DEFAULT_IS_COPYABLE: bool = true;
pub const DEFAULT_COPY_LABEL: &str = "Copy";
pub const DEFAULT_COPIED_LABEL: &str = "Copied";
pub const DEFAULT_COPY_ARIA_LABEL: &str = "Copy to clipboard";
pub const DEFAULT_COPY_ERROR_LABEL: &str = "Copy failed. Activate again to retry.";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnippetTextContract {
    pub copy_label: String,
    pub copied_label: String,
    pub copy_aria_label: String,
    pub copy_error_label: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SnippetTextFallbacks {
    pub copy_label: Option<String>,
    pub copied_label: Option<String>,
    pub copy_aria_label: Option<String>,
    pub copy_error_label: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnippetCopyableSource {
    Default,
    IsCopyableProp,
    LegacyCopyableProp,
}

impl SnippetCopyableSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::IsCopyableProp => "is_copyable",
            Self::LegacyCopyableProp => "copyable_legacy",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnippetCopyableContract {
    pub is_copyable: bool,
    pub source: SnippetCopyableSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnippetCopiedSource {
    Controlled,
    Uncontrolled,
}

impl SnippetCopiedSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone)]
pub struct SnippetControlledCopied {
    pub value: Option<Signal<bool>>,
    pub source: SnippetCopiedSource,
}

#[derive(Clone)]
pub struct SnippetLogicOptions {
    pub text: String,
    pub is_copyable: bool,
    pub is_copied: Option<Signal<bool>>,
    pub default_copied: Option<bool>,
    pub on_copied_change: Option<Callback<bool>>,
    pub on_copy_error: Option<Callback<()>>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct SnippetLogic {
    pub copied: Signal<bool>,
    pub is_loading: ReadSignal<bool>,
    pub has_error: ReadSignal<bool>,
    pub is_copying: ReadSignal<bool>,
    pub has_copy_error: ReadSignal<bool>,
    pub copy: Callback<()>,
    pub retry_copy: Callback<()>,
    pub aria_busy: Signal<Option<&'static str>>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

pub fn resolve_copyable_contract(
    is_copyable: Option<bool>,
    copyable: Option<bool>,
) -> SnippetCopyableContract {
    if let Some(value) = is_copyable {
        return SnippetCopyableContract {
            is_copyable: value,
            source: SnippetCopyableSource::IsCopyableProp,
        };
    }

    if let Some(value) = copyable {
        return SnippetCopyableContract {
            is_copyable: value,
            source: SnippetCopyableSource::LegacyCopyableProp,
        };
    }

    SnippetCopyableContract {
        is_copyable: DEFAULT_IS_COPYABLE,
        source: SnippetCopyableSource::Default,
    }
}

pub fn resolve_controlled_copied(
    is_copied: Option<Signal<bool>>,
    copied: Option<Signal<bool>>,
) -> SnippetControlledCopied {
    let value = is_copied.or(copied);
    let source = if value.is_some() {
        SnippetCopiedSource::Controlled
    } else {
        SnippetCopiedSource::Uncontrolled
    };
    SnippetControlledCopied { value, source }
}

pub fn resolve_text_contract(
    copy_label: Option<String>,
    copied_label: Option<String>,
    copy_aria_label: Option<String>,
    copy_error_label: Option<String>,
    fallbacks: SnippetTextFallbacks,
) -> SnippetTextContract {
    let SnippetTextFallbacks {
        copy_label: fallback_copy_label,
        copied_label: fallback_copied_label,
        copy_aria_label: fallback_copy_aria_label,
        copy_error_label: fallback_copy_error_label,
    } = fallbacks;

    let copy_label = normalize_optional_text(copy_label)
        .or_else(|| normalize_optional_text(fallback_copy_label))
        .unwrap_or_else(|| DEFAULT_COPY_LABEL.into());
    let copied_label = normalize_optional_text(copied_label)
        .or_else(|| normalize_optional_text(fallback_copied_label))
        .unwrap_or_else(|| DEFAULT_COPIED_LABEL.into());
    let copy_aria_label = normalize_optional_text(copy_aria_label)
        .or_else(|| normalize_optional_text(fallback_copy_aria_label))
        .unwrap_or_else(|| DEFAULT_COPY_ARIA_LABEL.into());
    let copy_error_label = normalize_optional_text(copy_error_label)
        .or_else(|| normalize_optional_text(fallback_copy_error_label))
        .unwrap_or_else(|| DEFAULT_COPY_ERROR_LABEL.into());

    SnippetTextContract {
        copy_label,
        copied_label,
        copy_aria_label,
        copy_error_label,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SnippetViewState) -> String {
    let mut classes = vec![
        "ui-snippet".to_string(),
        state.state_class.into(),
        state.copy_state_class.into(),
        state.copied_label_source_class.into(),
    ];

    if state.has_label {
        classes.push("ui-snippet--with-label".to_string());
    }
    if state.is_empty {
        classes.push("ui-snippet--empty".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-snippet--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn use_snippet_logic_with_options(options: SnippetLogicOptions) -> SnippetLogic {
    let contract = use_snippet_copy(SnippetCopyOptions {
        text: options.text,
        is_copyable: options.is_copyable,
        is_copied: options.is_copied,
        default_copied: options.default_copied,
        on_copied_change: options.on_copied_change,
        on_copy_error: options.on_copy_error,
        lang: options.lang,
        dir: options.dir,
    });

    SnippetLogic {
        copied: contract.state.copied,
        is_loading: contract.state.is_loading,
        has_error: contract.state.has_error,
        is_copying: contract.state.is_loading,
        has_copy_error: contract.state.has_error,
        copy: contract.handlers.on_copy,
        retry_copy: contract.handlers.on_retry,
        aria_busy: contract.attrs.aria_busy,
        lang: contract.attrs.lang,
        dir: contract.attrs.dir,
    }
}

#[cfg(any(feature = "component-button_copy", feature = "component-code_block"))]
pub fn use_snippet_logic(text: String) -> SnippetLogic {
    use_snippet_logic_with_options(SnippetLogicOptions {
        text,
        is_copyable: true,
        is_copied: None,
        default_copied: Some(false),
        on_copied_change: None,
        on_copy_error: None,
        lang: None,
        dir: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-snippet".to_string()),
            resolve_state(SnippetStateInput {
                is_multiline: false,
                has_text: false,
                has_label: true,
                is_copyable: true,
                has_custom_copied_label: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-snippet",
            "ui-snippet--state-single-line",
            "ui-snippet--copy-disabled",
            "ui-snippet--default-copied-label",
            "ui-snippet--with-label",
            "ui-snippet--empty",
            "ui-snippet--custom-class",
            "docs-snippet",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn snippet_logic_supports_controlled_copied_axis() {
        let (is_copied, set_is_copied) = signal(false);

        let logic = use_snippet_logic_with_options(SnippetLogicOptions {
            text: "cargo test".to_string(),
            is_copyable: true,
            is_copied: Some(is_copied.into()),
            default_copied: Some(true),
            on_copied_change: None,
            on_copy_error: None,
            lang: None,
            dir: None,
        });

        assert!(!logic.copied.get_untracked());
        set_is_copied.set(true);
        assert!(logic.copied.get_untracked());
    }

    #[test]
    fn default_snippet_logic_is_uncontrolled_and_not_busy() {
        let logic = use_snippet_logic_with_options(SnippetLogicOptions {
            text: "cargo fmt --all".to_string(),
            is_copyable: true,
            is_copied: None,
            default_copied: Some(false),
            on_copied_change: None,
            on_copy_error: None,
            lang: None,
            dir: None,
        });
        assert!(!logic.copied.get_untracked());
        assert!(!logic.is_loading.get_untracked());
        assert!(!logic.has_error.get_untracked());
        assert_eq!(logic.aria_busy.get_untracked(), None);
    }

    #[test]
    fn resolve_text_contract_centralizes_defaults() {
        let contract =
            resolve_text_contract(None, None, None, None, SnippetTextFallbacks::default());
        assert_eq!(contract.copy_label, DEFAULT_COPY_LABEL);
        assert_eq!(contract.copied_label, DEFAULT_COPIED_LABEL);
        assert_eq!(contract.copy_aria_label, DEFAULT_COPY_ARIA_LABEL);
        assert_eq!(contract.copy_error_label, DEFAULT_COPY_ERROR_LABEL);
    }

    #[test]
    fn resolve_text_contract_prefers_props_then_i18n_then_defaults() {
        let contract = resolve_text_contract(
            None,
            Some("Copied now".to_string()),
            None,
            None,
            SnippetTextFallbacks {
                copy_label: Some("复制".to_string()),
                copied_label: Some("已复制".to_string()),
                copy_aria_label: Some("复制到剪贴板".to_string()),
                copy_error_label: Some("复制失败，请重试".to_string()),
            },
        );

        assert_eq!(contract.copy_label, "复制");
        assert_eq!(contract.copied_label, "Copied now");
        assert_eq!(contract.copy_aria_label, "复制到剪贴板");
        assert_eq!(contract.copy_error_label, "复制失败，请重试");
    }

    #[test]
    fn resolve_copyable_contract_tracks_source() {
        let from_default = resolve_copyable_contract(None, None);
        assert_eq!(from_default.source, SnippetCopyableSource::Default);
        assert!(from_default.is_copyable);

        let from_new_prop = resolve_copyable_contract(Some(false), Some(true));
        assert_eq!(from_new_prop.source, SnippetCopyableSource::IsCopyableProp);
        assert!(!from_new_prop.is_copyable);

        let from_legacy_prop = resolve_copyable_contract(None, Some(false));
        assert_eq!(
            from_legacy_prop.source,
            SnippetCopyableSource::LegacyCopyableProp
        );
        assert!(!from_legacy_prop.is_copyable);
    }
}
