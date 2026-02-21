pub const DEFAULT_TITLE: &str = "Coachmark";
pub const DEFAULT_ASSET_LABEL: &str = "Coachmark asset";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoachmarkCtaMode {
    None,
    Primary,
    Secondary,
    Dual,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoachmarkAssetSource {
    None,
    Variant,
    Image,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CoachmarkStateInput {
    pub variant_attr: &'static str,
    pub placement_attr: &'static str,
    pub disabled: bool,
    pub is_controlled: bool,
    pub has_footer: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_shortcut: bool,
    pub cta_mode: CoachmarkCtaMode,
    pub has_actions_slot: bool,
    pub has_step_label: bool,
    pub asset_source: CoachmarkAssetSource,
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
        (!trimmed.is_empty()).then(|| trimmed.into())
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
    let title = normalize_optional_text(title).unwrap_or_else(|| DEFAULT_TITLE.into());

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

pub fn resolve_cta_mode(
    primary_cta: Option<&str>,
    secondary_cta: Option<&str>,
) -> CoachmarkCtaMode {
    match (primary_cta.is_some(), secondary_cta.is_some()) {
        (false, false) => CoachmarkCtaMode::None,
        (true, false) => CoachmarkCtaMode::Primary,
        (false, true) => CoachmarkCtaMode::Secondary,
        (true, true) => CoachmarkCtaMode::Dual,
    }
}

pub fn resolve_asset_source(
    asset_variant: Option<crate::asset::AssetVariant>,
    asset_src: Option<&str>,
) -> CoachmarkAssetSource {
    if asset_src.is_some() {
        CoachmarkAssetSource::Image
    } else if asset_variant.is_some() {
        CoachmarkAssetSource::Variant
    } else {
        CoachmarkAssetSource::None
    }
}

pub fn resolve_state(input: CoachmarkStateInput) -> CoachmarkState {
    let has_asset = !matches!(input.asset_source, CoachmarkAssetSource::None);

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
        asset_attr: if has_asset { "present" } else { "absent" },
        cta_attr: match input.cta_mode {
            CoachmarkCtaMode::None => "none",
            CoachmarkCtaMode::Primary | CoachmarkCtaMode::Secondary => "single",
            CoachmarkCtaMode::Dual => "dual",
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
        asset_source_attr: match input.asset_source {
            CoachmarkAssetSource::None => "none",
            CoachmarkAssetSource::Variant => "variant",
            CoachmarkAssetSource::Image => "image",
        },
        has_custom_class_name: input.has_custom_class_name,
        has_asset,
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
#[path = "test/coachmark.rs"]
mod tests;
