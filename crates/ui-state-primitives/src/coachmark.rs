pub const DEFAULT_TITLE: &str = "Coachmark";
pub const DEFAULT_ASSET_LABEL: &str = "Coachmark asset";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CoachmarkStateInput {
    pub variant_attr: &'static str,
    pub placement_attr: &'static str,
    pub disabled: bool,
    pub is_controlled: bool,
    pub has_footer: bool,
    pub has_asset: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_shortcut: bool,
    pub has_primary_cta: bool,
    pub has_secondary_cta: bool,
    pub has_actions_slot: bool,
    pub has_step_label: bool,
    pub has_asset_variant: bool,
    pub has_asset_src: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CoachmarkState {
    pub variant_attr: &'static str,
    pub placement_attr: &'static str,
    pub state_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub footer_attr: &'static str,
    pub asset_attr: &'static str,
    pub cta_attr: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub shortcut_attr: &'static str,
    pub actions_attr: &'static str,
    pub steps_attr: &'static str,
    pub asset_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_asset: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_modifier_keys(modifier_keys: Vec<String>) -> Vec<String> {
    modifier_keys
        .into_iter()
        .filter_map(|key| normalize_optional_text(Some(key)))
        .collect()
}

pub fn compose_heading(
    title: Option<String>,
    modifier_keys: Vec<String>,
    shortcut_key: Option<String>,
) -> String {
    let title = normalize_optional_text(title).unwrap_or_else(|| DEFAULT_TITLE.to_string());

    let mut keys = normalize_modifier_keys(modifier_keys);

    if let Some(shortcut_key) = normalize_optional_text(shortcut_key) {
        keys.push(shortcut_key);
    }

    if keys.is_empty() {
        title
    } else {
        format!("{title} ({})", keys.join(" + "))
    }
}

pub fn compose_step_label(
    current_step: Option<usize>,
    total_steps: Option<usize>,
) -> Option<String> {
    match (current_step, total_steps) {
        (Some(current_step), Some(total_steps)) if current_step > 0 && total_steps > 1 => {
            Some(format!("{current_step} of {total_steps}"))
        }
        _ => None,
    }
}

pub fn resolve_state(input: CoachmarkStateInput) -> CoachmarkState {
    let cta_count = usize::from(input.has_primary_cta) + usize::from(input.has_secondary_cta);

    CoachmarkState {
        variant_attr: input.variant_attr,
        placement_attr: input.placement_attr,
        state_attr: if input.disabled {
            "disabled"
        } else {
            "enabled"
        },
        open_mode_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        footer_attr: if input.has_footer {
            "present"
        } else {
            "absent"
        },
        asset_attr: if input.has_asset { "present" } else { "absent" },
        cta_attr: match cta_count {
            0 => "none",
            1 => "single",
            _ => "dual",
        },
        label_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        shortcut_attr: if input.has_shortcut {
            "present"
        } else {
            "absent"
        },
        actions_attr: if input.has_actions_slot {
            "present"
        } else {
            "absent"
        },
        steps_attr: if input.has_step_label {
            "present"
        } else {
            "absent"
        },
        asset_source_attr: if input.has_asset_src {
            "image"
        } else if input.has_asset_variant {
            "variant"
        } else {
            "none"
        },
        has_custom_class_name: input.has_custom_class_name,
        has_asset: input.has_asset,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: CoachmarkState) -> String {
    let mut classes = vec![
        "ui-coachmark".to_string(),
        format!("ui-coachmark--variant-{}", state.variant_attr),
        format!("ui-coachmark--placement-{}", state.placement_attr),
        format!("ui-coachmark--state-{}", state.state_attr),
        format!("ui-coachmark--mode-{}", state.open_mode_attr),
    ];

    if state.footer_attr == "present" {
        classes.push("ui-coachmark--with-footer".to_string());
    }

    if state.asset_attr == "present" {
        classes.push("ui-coachmark--with-asset".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-coachmark--custom-class".to_string());
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
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-coachmark ".to_string())),
            Some("docs-coachmark".to_string())
        );
    }

    #[test]
    fn compose_heading_appends_shortcut_keys() {
        assert_eq!(
            compose_heading(
                Some("Keyboard shortcuts".to_string()),
                vec!["Ctrl".to_string(), "K".to_string()],
                None,
            ),
            "Keyboard shortcuts (Ctrl + K)"
        );
        assert_eq!(compose_heading(None, vec![], None), DEFAULT_TITLE);
    }

    #[test]
    fn compose_step_label_requires_multi_step_context() {
        assert_eq!(
            compose_step_label(Some(2), Some(5)),
            Some("2 of 5".to_string())
        );
        assert_eq!(compose_step_label(Some(1), Some(1)), None);
        assert_eq!(compose_step_label(Some(0), Some(8)), None);
    }

    #[test]
    fn resolve_state_tracks_sources_and_markers() {
        let state = resolve_state(CoachmarkStateInput {
            variant_attr: "info",
            placement_attr: "top-end",
            disabled: false,
            is_controlled: true,
            has_footer: true,
            has_asset: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_shortcut: true,
            has_primary_cta: true,
            has_secondary_cta: true,
            has_actions_slot: true,
            has_step_label: true,
            has_asset_variant: false,
            has_asset_src: true,
        });

        assert_eq!(state.variant_attr, "info");
        assert_eq!(state.placement_attr, "top-end");
        assert_eq!(state.open_mode_attr, "controlled");
        assert_eq!(state.footer_attr, "present");
        assert_eq!(state.cta_attr, "dual");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.asset_source_attr, "image");
    }

    #[test]
    fn compose_class_name_exposes_state_markers() {
        let state = resolve_state(CoachmarkStateInput {
            variant_attr: "help",
            placement_attr: "bottom-start",
            disabled: true,
            is_controlled: false,
            has_footer: false,
            has_asset: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_shortcut: false,
            has_primary_cta: false,
            has_secondary_cta: false,
            has_actions_slot: false,
            has_step_label: false,
            has_asset_variant: false,
            has_asset_src: false,
        });

        let class_name = compose_class_name(Some("docs-coachmark".to_string()), state);
        for token in [
            "ui-coachmark",
            "ui-coachmark--variant-help",
            "ui-coachmark--placement-bottom-start",
            "ui-coachmark--state-disabled",
            "ui-coachmark--mode-uncontrolled",
            "ui-coachmark--custom-class",
            "docs-coachmark",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
