use super::*;
use leptos::prelude::{GetUntracked, Set};

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Newsletter  ".to_string())),
        Some("Newsletter".to_string())
    );

    assert_eq!(normalize_id_base(None), "ui-checkbox-field");
    assert_eq!(
        normalize_id_base(Some("  docs-checkbox-field  ".to_string())),
        "docs-checkbox-field"
    );

    assert_eq!(
        normalize_label(Some("  Accept terms  ".to_string())),
        ("Accept terms".to_string(), true)
    );
    assert_eq!(normalize_label(None), (DEFAULT_LABEL.into(), false));

    assert_eq!(
        normalize_aria_label(Some("  Custom aria  ".to_string()), "Ignored"),
        ("Custom aria".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None, "Fallback label"),
        ("Fallback label".to_string(), false)
    );
}

#[test]
fn resolve_content_centralizes_default_value_priority() {
    let resolved = resolve_content(CheckboxFieldContentInput {
        id_base: Some("  docs-checkbox-field  ".to_string()),
        label: Some("  Accept terms  ".to_string()),
        description: Some("  ".to_string()),
        aria_label: None,
        class_name: Some("  docs-checkbox-field  ".to_string()),
    });

    assert_eq!(resolved.id_base, "docs-checkbox-field");
    assert_eq!(resolved.label, "Accept terms");
    assert_eq!(resolved.description_text, "");
    assert_eq!(resolved.aria_label, "Accept terms");
    assert_eq!(resolved.class_name.as_deref(), Some("docs-checkbox-field"));
    assert!(!resolved.has_description);
    assert!(resolved.has_custom_label);
    assert!(!resolved.has_custom_aria_label);
    assert!(resolved.has_custom_class_name);
}

#[test]
fn resolve_state_tracks_state_markers() {
    let state = resolve_state(CheckboxFieldStateInput {
        checked: true,
        disabled: false,
        invalid: true,
        tone: CheckboxFieldTone::Quiet,
        indicator_placement: CheckboxFieldIndicatorPlacement::End,
        has_description: true,
        has_custom_label: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    assert_eq!(state.status, CheckboxFieldStatus::CheckedInvalid);
    assert!(state.is_checked);
    assert!(!state.is_unchecked);
    assert!(state.is_invalid);
    assert!(!state.is_disabled);
    assert_eq!(state.tone_class, "ui-checkbox-field--tone-quiet");
    assert_eq!(state.tone_attr, "quiet");
    assert_eq!(
        state.indicator_placement_class,
        "ui-checkbox-field--indicator-end"
    );
    assert_eq!(state.indicator_placement_attr, "end");
    assert_eq!(state.description_attr, "present");
    assert_eq!(state.label_source_attr, "default");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.state_attr, "checked-invalid");
}

#[test]
fn resolve_status_models_mutually_exclusive_state_with_enum() {
    assert_eq!(
        resolve_status(false, false, false),
        CheckboxFieldStatus::Unchecked
    );
    assert_eq!(
        resolve_status(true, false, false),
        CheckboxFieldStatus::Checked
    );
    assert_eq!(
        resolve_status(false, true, false),
        CheckboxFieldStatus::Disabled
    );
    assert_eq!(
        resolve_status(false, false, true),
        CheckboxFieldStatus::Invalid
    );
    assert_eq!(
        resolve_status(true, false, true),
        CheckboxFieldStatus::CheckedInvalid
    );
}

#[test]
fn compose_class_name_includes_state_classes() {
    let state = resolve_state(CheckboxFieldStateInput {
        checked: false,
        disabled: true,
        invalid: false,
        tone: CheckboxFieldTone::Default,
        indicator_placement: CheckboxFieldIndicatorPlacement::Start,
        has_description: false,
        has_custom_label: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-checkbox-field".to_string()), state);

    for expected in [
        "ui-checkbox-field",
        "ui-checkbox-field--tone-default",
        "ui-checkbox-field--indicator-start",
        "ui-checkbox-field--unchecked",
        "ui-checkbox-field--disabled",
        "ui-checkbox-field--no-description",
        "ui-checkbox-field--custom-class",
        "docs-checkbox-field",
    ] {
        assert!(class_name.contains(expected));
    }
}

#[test]
fn normalize_boolean_aliases_prefers_is_prefix_when_present() {
    assert!(normalize_is_disabled(Some(true), false));
    assert!(!normalize_is_disabled(Some(false), true));
    assert!(normalize_is_disabled(None, true));

    assert!(normalize_is_invalid(Some(true), false));
    assert!(!normalize_is_invalid(Some(false), true));
    assert!(normalize_is_invalid(None, true));
}

#[test]
fn resolve_checked_control_uncontrolled_uses_default_and_internal_writer() {
    let resolved = resolve_checked_control(None, None, None, None, Some(true));
    assert_eq!(resolved.mode, CheckboxControlMode::Uncontrolled);
    assert!(resolved.checked.get_untracked());
    assert_eq!(resolved.checked_prop_source_attr, "none");
    assert_eq!(resolved.checked_change_source_attr, "internal");
    assert_eq!(resolved.checked_default_source_attr, "default_checked");

    let writer = resolved.on_checked_change.expect("internal writer");
    writer.set(false);
    assert!(!resolved.checked.get_untracked());
}

#[test]
fn resolve_checked_control_controlled_without_writer_stays_read_only() {
    let (external_checked, _) = leptos::prelude::signal(true);
    let resolved = resolve_checked_control(Some(external_checked), None, None, None, Some(false));

    assert_eq!(resolved.mode, CheckboxControlMode::Controlled);
    assert!(resolved.checked.get_untracked());
    assert!(resolved.on_checked_change.is_none());
    assert_eq!(resolved.checked_prop_source_attr, "is_checked");
    assert_eq!(resolved.checked_change_source_attr, "none");
}

#[test]
fn resolve_checked_control_controlled_prefers_primary_writer_over_alias() {
    let (external_checked, _) = leptos::prelude::signal(false);
    let (primary_value, primary_set) = leptos::prelude::signal(false);
    let (alias_value, alias_set) = leptos::prelude::signal(false);

    let resolved = resolve_checked_control(
        Some(external_checked),
        None,
        Some(primary_set),
        Some(alias_set),
        None,
    );

    assert_eq!(resolved.mode, CheckboxControlMode::Controlled);
    assert_eq!(resolved.checked_change_source_attr, "on_checked_change");

    let writer = resolved.on_checked_change.expect("writer");
    writer.set(true);
    assert!(primary_value.get_untracked());
    assert!(!alias_value.get_untracked());
}

#[test]
fn resolve_render_state_centralizes_state_derivation_and_class_mapping() {
    let render_state = resolve_render_state(CheckboxFieldRenderStateInput {
        checked: true,
        disabled: false,
        invalid: true,
        tone: CheckboxFieldTone::Quiet,
        indicator_placement: CheckboxFieldIndicatorPlacement::End,
        has_description: true,
        has_custom_label: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
        class_name: Some("docs-checkbox-field".to_string()),
    });

    assert_eq!(render_state.state.state_attr, "checked-invalid");
    assert_eq!(render_state.state.indicator_placement_attr, "end");
    assert!(
        render_state
            .root_class_name
            .contains("ui-checkbox-field--checked")
    );
    assert!(
        render_state
            .root_class_name
            .contains("ui-checkbox-field--indicator-end")
    );
}

#[test]
fn resolve_checkbox_affordance_maps_variant_and_class_without_view_branching() {
    let start_default = resolve_checkbox_affordance(CheckboxFieldIndicatorPlacement::Start, false);
    assert_eq!(start_default.class_name, "ui-checkbox-field__checkbox");
    assert_eq!(start_default.variant, ui_checkbox::CheckboxVariant::Default);

    let end_invalid = resolve_checkbox_affordance(CheckboxFieldIndicatorPlacement::End, true);
    assert_eq!(
        end_invalid.class_name,
        "ui-checkbox-field__checkbox ui-checkbox-field__checkbox--indicator-end"
    );
    assert_eq!(end_invalid.variant, ui_checkbox::CheckboxVariant::Accent);
}

#[test]
fn resolve_agent_contract_models_schema_state_action_and_source_axes() {
    let controlled = resolve_agent_contract(CheckboxFieldAgentContractInput {
        status: CheckboxFieldStatus::CheckedInvalid,
        checked_mode: CheckboxControlMode::Controlled,
        checked_prop_source_attr: "is_checked",
        checked_change_source_attr: "on_checked_change",
        checked_default_source_attr: "default_checked",
    });

    assert_eq!(controlled.schema_name, "ui.checkbox-field.agent-contract");
    assert_eq!(
        controlled.schema_version,
        CheckboxFieldAgentSchemaVersion::V1
    );
    assert_eq!(controlled.intent, CheckboxFieldAgentIntent::BooleanField);
    assert_eq!(
        controlled.action,
        CheckboxFieldAgentAction::ToggleControlled
    );
    assert_eq!(
        controlled.state,
        CheckboxFieldAgentStateAxis::CheckedInvalid
    );
    assert_eq!(controlled.source, CheckboxFieldAgentSource::IsCheckedProp);
    assert_eq!(
        controlled.output_status,
        CheckboxFieldAgentOutputStatus::Submittable
    );

    let uncontrolled_implicit = resolve_agent_contract(CheckboxFieldAgentContractInput {
        status: CheckboxFieldStatus::Unchecked,
        checked_mode: CheckboxControlMode::Uncontrolled,
        checked_prop_source_attr: "none",
        checked_change_source_attr: "internal",
        checked_default_source_attr: "implicit-default",
    });

    assert_eq!(
        uncontrolled_implicit.action,
        CheckboxFieldAgentAction::ToggleUncontrolled
    );
    assert_eq!(
        uncontrolled_implicit.source,
        CheckboxFieldAgentSource::ImplicitDefault
    );
    assert_eq!(
        uncontrolled_implicit.state,
        CheckboxFieldAgentStateAxis::Unchecked
    );
    assert_eq!(
        uncontrolled_implicit.output_status,
        CheckboxFieldAgentOutputStatus::Submittable
    );
}

#[test]
fn resolve_agent_contract_marks_controlled_without_writer_as_read_only() {
    let read_only = resolve_agent_contract(CheckboxFieldAgentContractInput {
        status: CheckboxFieldStatus::Disabled,
        checked_mode: CheckboxControlMode::Controlled,
        checked_prop_source_attr: "checked",
        checked_change_source_attr: "none",
        checked_default_source_attr: "default_checked",
    });

    assert_eq!(
        read_only.action,
        CheckboxFieldAgentAction::ReadOnlyControlled
    );
    assert_eq!(read_only.source, CheckboxFieldAgentSource::CheckedAliasProp);
    assert_eq!(read_only.state, CheckboxFieldAgentStateAxis::Disabled);
    assert_eq!(
        read_only.output_status,
        CheckboxFieldAgentOutputStatus::Verified
    );
}
