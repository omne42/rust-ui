use ui_state_primitives::breadcrumb as breadcrumb_state;

pub use ui_state_primitives::breadcrumb::{
    BreadcrumbLinkState, BreadcrumbLinkStateInput, BreadcrumbRootState, BreadcrumbRootStateInput,
    BreadcrumbSeparatorState, BreadcrumbSeparatorStateInput, BreadcrumbSlotState,
    BreadcrumbSlotStateInput,
};
pub const DEFAULT_ARIA_LABEL: &str = breadcrumb_state::DEFAULT_ARIA_LABEL;

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    breadcrumb_state::normalize_optional_text(value)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    let (label, custom) = breadcrumb_state::normalize_aria_label(value);
    if custom {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn normalize_href(value: Option<String>) -> Option<String> {
    breadcrumb_state::normalize_href(value)
}

pub fn resolve_root_state(input: BreadcrumbRootStateInput) -> BreadcrumbRootState {
    breadcrumb_state::resolve_root_state(input)
}

pub fn resolve_slot_state(input: BreadcrumbSlotStateInput) -> BreadcrumbSlotState {
    breadcrumb_state::resolve_slot_state(input)
}

pub fn resolve_link_state(input: BreadcrumbLinkStateInput) -> BreadcrumbLinkState {
    breadcrumb_state::resolve_link_state(input)
}

pub fn resolve_separator_state(input: BreadcrumbSeparatorStateInput) -> BreadcrumbSeparatorState {
    breadcrumb_state::resolve_separator_state(input)
}

pub fn compose_class_name(
    base_class_name: &'static str,
    class_name: Option<String>,
    has_custom_class_name: bool,
) -> String {
    let mut classes = vec![base_class_name.into()];

    if has_custom_class_name {
        classes.push(format!("{base_class_name}--custom-class"));

        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

pub fn compose_link_class_name(class_name: Option<String>, state: BreadcrumbLinkState) -> String {
    let mut classes = vec!["ui-breadcrumb__link".to_string()];

    if !state.interactive {
        classes.push("ui-breadcrumb__link--placeholder".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-breadcrumb__link--custom-class".to_string());

        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

pub fn compose_separator_class_name(
    class_name: Option<String>,
    state: BreadcrumbSeparatorState,
) -> String {
    let mut classes = vec!["ui-breadcrumb__separator".to_string()];

    if state.content_source_attr == "custom" {
        classes.push("ui-breadcrumb__separator--custom-content".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-breadcrumb__separator--custom-class".to_string());

        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_aria_label_tracks_default_and_custom_sources() {
        assert_eq!(
            normalize_aria_label(Some("  Site navigation  ".to_string())),
            ("Site navigation".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
    }

    #[test]
    fn normalize_href_trims_blank_values() {
        assert_eq!(
            normalize_href(Some("  /components  ".to_string())),
            Some("/components".to_string())
        );
        assert_eq!(normalize_href(Some("  ".to_string())), None);
        assert_eq!(normalize_href(None), None);
    }

    #[test]
    fn resolve_root_and_slot_states_track_source_contracts() {
        let root = resolve_root_state(BreadcrumbRootStateInput {
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(root.state_attr, "customized");
        assert_eq!(root.aria_source_attr, "custom");
        assert_eq!(root.class_source_attr, "default");

        let slot = resolve_slot_state(BreadcrumbSlotStateInput {
            has_custom_class_name: true,
        });

        assert_eq!(slot.state_attr, "customized");
        assert_eq!(slot.class_source_attr, "custom");
    }

    #[test]
    fn resolve_link_and_separator_states_cover_customized_paths() {
        let link = resolve_link_state(BreadcrumbLinkStateInput {
            has_href: false,
            has_custom_class_name: true,
        });

        assert_eq!(link.state_attr, "placeholder-customized");
        assert_eq!(link.href_state_attr, "absent");
        assert!(!link.interactive);
        assert_eq!(link.class_source_attr, "custom");

        let separator = resolve_separator_state(BreadcrumbSeparatorStateInput {
            has_custom_content: true,
            has_custom_class_name: true,
        });

        assert_eq!(separator.state_attr, "customized");
        assert_eq!(separator.content_source_attr, "custom");
        assert_eq!(separator.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_helpers_include_custom_markers() {
        let root_class =
            compose_class_name("ui-breadcrumb", Some("docs-breadcrumb".to_string()), true);

        for token in [
            "ui-breadcrumb",
            "ui-breadcrumb--custom-class",
            "docs-breadcrumb",
        ] {
            assert!(
                root_class.contains(token),
                "composed root class name should include `{token}`"
            );
        }

        let link_state = resolve_link_state(BreadcrumbLinkStateInput {
            has_href: false,
            has_custom_class_name: true,
        });

        let link_class =
            compose_link_class_name(Some("docs-breadcrumb-link".to_string()), link_state);

        for token in [
            "ui-breadcrumb__link",
            "ui-breadcrumb__link--placeholder",
            "ui-breadcrumb__link--custom-class",
            "docs-breadcrumb-link",
        ] {
            assert!(
                link_class.contains(token),
                "composed link class name should include `{token}`"
            );
        }

        let separator_state = resolve_separator_state(BreadcrumbSeparatorStateInput {
            has_custom_content: true,
            has_custom_class_name: true,
        });

        let separator_class = compose_separator_class_name(
            Some("docs-breadcrumb-separator".to_string()),
            separator_state,
        );

        for token in [
            "ui-breadcrumb__separator",
            "ui-breadcrumb__separator--custom-content",
            "ui-breadcrumb__separator--custom-class",
            "docs-breadcrumb-separator",
        ] {
            assert!(
                separator_class.contains(token),
                "composed separator class name should include `{token}`"
            );
        }
    }
}
