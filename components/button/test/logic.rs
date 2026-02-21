use super::*;
use crate::ButtonIntent;

fn base_state_input() -> ButtonStateInput {
    ButtonStateInput {
        is_disabled: false,
        is_loading: false,
        variant: ButtonVariant::Default,
        color: ButtonColor::Primary,
        radius: ButtonRadius::Md,
        size: ButtonSize::M,
        loading_placement: ButtonLoadingPlacement::Start,
        is_full_width: false,
        has_custom_class_name: false,
        has_custom_motion: false,
    }
}

fn base_normalization_input() -> ButtonInputNormalizationInput {
    ButtonInputNormalizationInput {
        is_disabled: false,
        is_full_width: false,
        class_name: None,
        aria_label: None,
        button_type: ButtonType::Button,
    }
}

#[test]
fn variant_class_names_are_stable() {
    assert_eq!(
        ButtonVariant::Default.class_name(),
        "ui-button--variant-default"
    );
    assert_eq!(
        ButtonVariant::Solid.class_name(),
        "ui-button--variant-solid"
    );
    assert_eq!(
        ButtonVariant::Faded.class_name(),
        "ui-button--variant-faded"
    );
    assert_eq!(
        ButtonVariant::Bordered.class_name(),
        "ui-button--variant-bordered"
    );
    assert_eq!(
        ButtonVariant::Light.class_name(),
        "ui-button--variant-light"
    );
    assert_eq!(ButtonVariant::Flat.class_name(), "ui-button--variant-flat");
    assert_eq!(
        ButtonVariant::Shadow.class_name(),
        "ui-button--variant-shadow"
    );
    assert_eq!(
        ButtonVariant::Accent.class_name(),
        "ui-button--variant-accent"
    );
    assert_eq!(
        ButtonVariant::Destructive.class_name(),
        "ui-button--variant-destructive"
    );
    assert_eq!(
        ButtonVariant::Outline.class_name(),
        "ui-button--variant-outline"
    );
    assert_eq!(
        ButtonVariant::Secondary.class_name(),
        "ui-button--variant-secondary"
    );
    assert_eq!(
        ButtonVariant::Ghost.class_name(),
        "ui-button--variant-ghost"
    );
    assert_eq!(ButtonVariant::Link.class_name(), "ui-button--variant-link");
}

#[test]
fn color_class_and_attr_names_are_stable() {
    assert_eq!(
        ButtonColor::Default.class_name(),
        "ui-button--color-default"
    );
    assert_eq!(
        ButtonColor::Primary.class_name(),
        "ui-button--color-primary"
    );
    assert_eq!(
        ButtonColor::Secondary.class_name(),
        "ui-button--color-secondary"
    );
    assert_eq!(
        ButtonColor::Success.class_name(),
        "ui-button--color-success"
    );
    assert_eq!(
        ButtonColor::Warning.class_name(),
        "ui-button--color-warning"
    );
    assert_eq!(ButtonColor::Danger.class_name(), "ui-button--color-danger");

    assert_eq!(ButtonColor::Default.as_attr(), "default");
    assert_eq!(ButtonColor::Primary.as_attr(), "primary");
    assert_eq!(ButtonColor::Secondary.as_attr(), "secondary");
    assert_eq!(ButtonColor::Success.as_attr(), "success");
    assert_eq!(ButtonColor::Warning.as_attr(), "warning");
    assert_eq!(ButtonColor::Danger.as_attr(), "danger");
}

#[test]
fn radius_class_and_attr_names_are_stable() {
    assert_eq!(ButtonRadius::None.class_name(), "ui-button--radius-none");
    assert_eq!(ButtonRadius::Sm.class_name(), "ui-button--radius-sm");
    assert_eq!(ButtonRadius::Md.class_name(), "ui-button--radius-md");
    assert_eq!(ButtonRadius::Lg.class_name(), "ui-button--radius-lg");
    assert_eq!(ButtonRadius::Full.class_name(), "ui-button--radius-full");

    assert_eq!(ButtonRadius::None.as_attr(), "none");
    assert_eq!(ButtonRadius::Sm.as_attr(), "sm");
    assert_eq!(ButtonRadius::Md.as_attr(), "md");
    assert_eq!(ButtonRadius::Lg.as_attr(), "lg");
    assert_eq!(ButtonRadius::Full.as_attr(), "full");
}

#[test]
fn size_class_names_and_string_conversions_are_stable() {
    assert_eq!(ButtonSize::Xs.class_name(), "ui-button--size-xs");
    assert_eq!(ButtonSize::S.class_name(), "ui-button--size-s");
    assert_eq!(ButtonSize::M.class_name(), "ui-button--size-m");
    assert_eq!(ButtonSize::L.class_name(), "ui-button--size-l");
    assert_eq!(ButtonSize::Xl.class_name(), "ui-button--size-xl");
    assert_eq!(ButtonSize::IconXs.class_name(), "ui-button--size-icon-xs");
    assert_eq!(ButtonSize::IconS.class_name(), "ui-button--size-icon-s");
    assert_eq!(ButtonSize::IconM.class_name(), "ui-button--size-icon-m");
    assert_eq!(ButtonSize::IconL.class_name(), "ui-button--size-icon-l");
    assert_eq!(ButtonSize::IconXl.class_name(), "ui-button--size-icon-xl");

    assert_eq!(ButtonSize::from("xs"), ButtonSize::Xs);
    assert_eq!(ButtonSize::from("s"), ButtonSize::S);
    assert_eq!(ButtonSize::from("m"), ButtonSize::M);
    assert_eq!(ButtonSize::from("l"), ButtonSize::L);
    assert_eq!(ButtonSize::from("xl"), ButtonSize::Xl);
}

#[test]
fn loading_placement_and_button_type_attrs_are_stable() {
    assert_eq!(ButtonLoadingPlacement::Start.as_attr(), "start");
    assert_eq!(ButtonLoadingPlacement::End.as_attr(), "end");
    assert_eq!(ButtonLoadingPlacement::Center.as_attr(), "center");

    assert_eq!(ButtonType::Button.as_attr(), "button");
    assert_eq!(ButtonType::Submit.as_attr(), "submit");
    assert_eq!(ButtonType::Reset.as_attr(), "reset");

    assert_eq!(ButtonType::from("button"), ButtonType::Button);
    assert_eq!(ButtonType::from("submit"), ButtonType::Submit);
    assert_eq!(ButtonType::from("reset"), ButtonType::Reset);
}

#[test]
fn label_source_attrs_are_stable() {
    assert_eq!(ButtonLabelSource::Explicit.as_attr(), "explicit");
    assert_eq!(ButtonLabelSource::None.as_attr(), "none");
}

#[test]
fn normalize_optional_text_and_aria_label_follow_new_contract() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Button  ".to_string())),
        Some("Button".to_string())
    );

    assert_eq!(
        resolve_aria_label(Some(" Save ".to_string())),
        (Some("Save".to_string()), ButtonLabelSource::Explicit)
    );
    assert_eq!(resolve_aria_label(None), (None, ButtonLabelSource::None));
}

#[test]
fn normalize_input_and_state_resolution_match_flags() {
    let normalized = normalize_input(ButtonInputNormalizationInput {
        is_disabled: true,
        is_full_width: true,
        class_name: Some("  docs-btn  ".to_string()),
        aria_label: Some(" Save ".to_string()),
        button_type: ButtonType::Submit,
    });

    assert!(normalized.is_disabled);
    assert!(normalized.is_full_width);
    assert_eq!(
        normalized.disabled_input_source,
        ButtonBooleanInputSource::IsProp
    );
    assert_eq!(
        normalized.full_width_input_source,
        ButtonBooleanInputSource::IsProp
    );
    assert_eq!(normalized.class_name, Some("docs-btn".to_string()));
    assert!(normalized.has_custom_class_name);
    assert_eq!(normalized.aria_label, Some("Save".to_string()));
    assert_eq!(normalized.aria_label_source, ButtonLabelSource::Explicit);
    assert_eq!(normalized.button_type, ButtonType::Submit);

    let state = resolve_state(ButtonStateInput {
        is_disabled: false,
        is_loading: true,
        variant: ButtonVariant::Secondary,
        color: ButtonColor::Success,
        radius: ButtonRadius::Full,
        size: ButtonSize::Icon,
        loading_placement: ButtonLoadingPlacement::End,
        is_full_width: true,
        has_custom_class_name: true,
        has_custom_motion: true,
    });

    assert!(state.is_disabled);
    assert_eq!(state.state_attr, "loading");
    assert_eq!(state.color_attr, "success");
    assert_eq!(state.radius_attr, "full");
    assert_eq!(state.loading_placement_attr, "end");
    assert!(state.is_full_width);
    assert!(state.has_custom_class_name);
    assert!(state.has_custom_motion);
}

#[test]
fn render_state_and_class_composition_follow_current_model() {
    let state = resolve_state(ButtonStateInput {
        is_loading: true,
        variant: ButtonVariant::Outline,
        color: ButtonColor::Danger,
        radius: ButtonRadius::Sm,
        size: ButtonSize::S,
        loading_placement: ButtonLoadingPlacement::Center,
        is_full_width: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        ..base_state_input()
    });

    let class_name = compose_class_name(Some("docs-button".to_string()), state);
    for needle in [
        "ui-button",
        "ui-button--variant-outline",
        "ui-button--color-danger",
        "ui-button--radius-sm",
        "ui-button--size-s",
        "ui-button--loading-center",
        "ui-button--full-width",
        "ui-button--loading",
        "ui-button--custom-motion",
        "docs-button",
    ] {
        assert!(class_name.contains(needle));
    }

    let start = derive_render_state(resolve_state(ButtonStateInput {
        is_loading: true,
        loading_placement: ButtonLoadingPlacement::Start,
        ..base_state_input()
    }));
    assert!(start.show_start_inline_spinner);
    assert!(!start.show_end_spinner);
    assert!(!start.show_center_spinner);

    let end = derive_render_state(resolve_state(ButtonStateInput {
        is_loading: true,
        loading_placement: ButtonLoadingPlacement::End,
        ..base_state_input()
    }));
    assert!(!end.show_start_inline_spinner);
    assert!(end.show_end_spinner);
    assert!(!end.show_center_spinner);

    let center = derive_render_state(resolve_state(ButtonStateInput {
        is_loading: true,
        loading_placement: ButtonLoadingPlacement::Center,
        ..base_state_input()
    }));
    assert!(!center.show_start_inline_spinner);
    assert!(!center.show_end_spinner);
    assert!(center.show_center_spinner);
}

#[test]
fn resolve_view_state_and_status_and_agent_contract_are_consistent() {
    let view_state = resolve_view_state(ButtonLogicInput {
        normalized: normalize_input(ButtonInputNormalizationInput {
            is_disabled: true,
            is_full_width: true,
            class_name: Some("  docs-btn  ".to_string()),
            aria_label: None,
            button_type: ButtonType::Button,
        }),
        is_loading: true,
        variant: ButtonVariant::Outline,
        color: ButtonColor::Danger,
        radius: ButtonRadius::Sm,
        size: ButtonSize::S,
        loading_placement: ButtonLoadingPlacement::Center,
        has_custom_motion: true,
    });

    assert!(view_state.state.is_disabled);
    assert!(view_state.state.is_full_width);
    assert!(view_state.state.has_custom_motion);
    assert_eq!(view_state.source.disabled_source_attr, "loading");
    assert_eq!(view_state.source.loading_source_attr, "prop");
    assert_eq!(view_state.source.disabled_input_source_attr, "is-prop");
    assert_eq!(view_state.source.full_width_input_source_attr, "is-prop");
    assert!(view_state.class_name.contains("ui-button--variant-outline"));
    assert!(view_state.class_name.contains("docs-btn"));
    assert!(view_state.render.show_center_spinner);

    let ready = resolve_state(ButtonStateInput {
        is_loading: false,
        ..base_state_input()
    });
    let disabled = resolve_state(ButtonStateInput {
        is_disabled: true,
        ..base_state_input()
    });

    assert_eq!(
        resolve_output_status(ready, ButtonType::Submit).as_attr(),
        "submittable"
    );
    assert_eq!(
        resolve_output_status(ready, ButtonType::Button).as_attr(),
        "verified"
    );
    assert_eq!(
        resolve_output_status(view_state.state, ButtonType::Submit).as_attr(),
        "draft"
    );

    let interactive_contract = resolve_agent_contract(ready, true);
    assert_eq!(interactive_contract.schema_name, BUTTON_AGENT_SCHEMA);
    assert_eq!(interactive_contract.schema_version.as_str(), "1");
    assert_eq!(interactive_contract.intent.as_str(), "trigger");
    assert_eq!(interactive_contract.action.as_str(), "press");
    assert_eq!(interactive_contract.state.as_str(), "ready");
    assert_eq!(interactive_contract.source.as_str(), "state-primitives");
    assert!(interactive_contract.capabilities.can_press);
    assert!(interactive_contract.capabilities.can_focus);
    assert!(interactive_contract.capabilities.can_hover);
    assert!(interactive_contract.capabilities.can_popup_trigger);

    let disabled_contract = resolve_agent_contract(disabled, false);
    assert_eq!(disabled_contract.state.as_str(), "disabled");
    assert!(!disabled_contract.capabilities.can_press);
    assert!(!disabled_contract.capabilities.can_focus);
    assert!(!disabled_contract.capabilities.can_hover);
    assert!(!disabled_contract.capabilities.can_popup_trigger);
}

#[test]
fn normalize_schema_json_input_enforces_typed_whitelist_boundary() {
    let valid_schema = ButtonSchema {
        schema_version: 1,
        element_id: "btn_primary".to_string(),
        intent: ButtonIntent::Primary,
        action_signature: "submit()".to_string(),
        requires_confirmation: false,
    };
    let valid_json = valid_schema.to_json();
    let normalized_valid = normalize_schema_json_input(Some(valid_json.clone()));
    assert_eq!(
        normalized_valid.source,
        ButtonSchemaInputSource::PropValidated
    );
    assert_eq!(normalized_valid.schema_json, Some(valid_json));

    let normalized_missing = normalize_schema_json_input(None);
    assert_eq!(normalized_missing.source, ButtonSchemaInputSource::Missing);
    assert_eq!(normalized_missing.schema_json, None);

    let normalized_invalid = normalize_schema_json_input(Some("<script>alert(1)</script>".into()));
    assert_eq!(
        normalized_invalid.source,
        ButtonSchemaInputSource::PropRejected
    );
    assert_eq!(normalized_invalid.schema_json, None);
}

#[test]
fn defaults_are_kept_when_inputs_are_missing() {
    let normalized = normalize_input(base_normalization_input());
    assert!(!normalized.is_disabled);
    assert!(!normalized.is_full_width);
    assert_eq!(normalized.aria_label, None);
    assert_eq!(normalized.aria_label_source, ButtonLabelSource::None);
    assert_eq!(normalized.button_type, ButtonType::Button);
}

#[cfg(feature = "component-button_group")]
#[test]
fn button_group_orientation_class_and_data_values_are_stable() {
    assert_eq!(
        ButtonGroupOrientation::Horizontal.class_name(),
        "ui-button-group--horizontal"
    );
    assert_eq!(
        ButtonGroupOrientation::Vertical.class_name(),
        "ui-button-group--vertical"
    );
    assert_eq!(
        ButtonGroupOrientation::Horizontal.data_orientation(),
        "horizontal"
    );
    assert_eq!(
        ButtonGroupOrientation::Vertical.data_orientation(),
        "vertical"
    );
}

#[cfg(feature = "component-button_group")]
#[test]
fn button_group_aria_label_uses_trimmed_label_or_fallback() {
    let (label, explicit) = normalize_button_group_aria_label(Some("  Text align  ".to_string()));
    assert_eq!(label, "Text align");
    assert!(explicit);

    let (label, explicit) = normalize_button_group_aria_label(Some("   ".to_string()));
    assert_eq!(label, "Button group");
    assert!(!explicit);

    let (label, explicit) = normalize_button_group_aria_label(None);
    assert_eq!(label, "Button group");
    assert!(!explicit);
}

#[cfg(feature = "component-button_group")]
#[test]
fn resolve_button_group_state_tracks_orientation_attachment_and_label_source() {
    let state = resolve_button_group_state(ButtonGroupOrientation::Vertical, true, true);
    assert!(!state.is_horizontal);
    assert!(state.is_vertical);
    assert!(state.is_attached);
    assert!(!state.is_detached);
    assert!(state.has_explicit_label);
    assert!(!state.has_fallback_label);

    let state = resolve_button_group_state(ButtonGroupOrientation::Horizontal, false, false);
    assert!(state.is_horizontal);
    assert!(!state.is_vertical);
    assert!(!state.is_attached);
    assert!(state.is_detached);
    assert!(!state.has_explicit_label);
    assert!(state.has_fallback_label);
}
