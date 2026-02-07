#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum KbdSize {
    Sm,
    #[default]
    Md,
}

impl KbdSize {
    pub fn class_name(self) -> &'static str {
        match self {
            KbdSize::Sm => "ui-kbd--size-sm",
            KbdSize::Md => "ui-kbd--size-md",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            KbdSize::Sm => "sm",
            KbdSize::Md => "md",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KbdStateInput {
    pub size: KbdSize,
    pub has_keys: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KbdState {
    pub size: KbdSize,
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub has_keys: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: KbdStateInput) -> KbdState {
    let (state_class, state_attr) = if input.has_keys {
        ("ui-kbd--state-with-keys", "with-keys")
    } else {
        ("ui-kbd--state-label-only", "label-only")
    };

    KbdState {
        size: input.size,
        size_class: input.size.class_name(),
        size_attr: input.size.as_attr(),
        state_class,
        state_attr,
        has_keys: input.has_keys,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: KbdState) -> String {
    let mut classes = vec![
        "ui-kbd".to_string(),
        state.size_class.to_string(),
        state.state_class.to_string(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-kbd--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_class_names_and_attrs_are_stable() {
        assert_eq!(KbdSize::Sm.class_name(), "ui-kbd--size-sm");
        assert_eq!(KbdSize::Md.class_name(), "ui-kbd--size-md");

        assert_eq!(KbdSize::Sm.as_attr(), "sm");
        assert_eq!(KbdSize::Md.as_attr(), "md");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Ctrl+K  ".to_string())),
            Some("Ctrl+K".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_size_keys_and_class_source() {
        let state = resolve_state(KbdStateInput {
            size: KbdSize::Sm,
            has_keys: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.size, KbdSize::Sm);
        assert_eq!(state.size_class, "ui-kbd--size-sm");
        assert_eq!(state.size_attr, "sm");
        assert_eq!(state.state_class, "ui-kbd--state-with-keys");
        assert_eq!(state.state_attr, "with-keys");
        assert!(state.has_keys);
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-kbd".to_string()),
            resolve_state(KbdStateInput {
                size: KbdSize::Md,
                has_keys: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-kbd",
            "ui-kbd--size-md",
            "ui-kbd--state-label-only",
            "ui-kbd--custom-class",
            "docs-kbd",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
