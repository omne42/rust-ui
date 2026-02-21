use super::*;

#[test]
fn normalize_disable_state_uses_is_prefix_first() {
    let from_is = normalize_disable_state(ColorAreaDisableInput {
        is_disabled: Some(true),
    });
    assert!(from_is.is_disabled);
    assert_eq!(
        from_is.disabled_source_attr,
        ColorAreaDisabledSourceAttr::IsProp
    );

    let from_default = normalize_disable_state(ColorAreaDisableInput { is_disabled: None });
    assert!(!from_default.is_disabled);
    assert_eq!(
        from_default.disabled_source_attr,
        ColorAreaDisabledSourceAttr::Default
    );
}

#[test]
fn normalize_value_axis_tracks_control_mode() {
    let controlled = normalize_value_axis(true);
    assert_eq!(
        controlled.control_mode,
        ColorAreaValueControlMode::Controlled
    );
    assert_eq!(controlled.value_source, ColorAreaValueSourceAttr::External);

    let uncontrolled = normalize_value_axis(false);
    assert_eq!(
        uncontrolled.control_mode,
        ColorAreaValueControlMode::Uncontrolled
    );
    assert_eq!(uncontrolled.value_source, ColorAreaValueSourceAttr::Default);
}

#[test]
fn normalize_default_value_uses_single_fallback_source() {
    assert_eq!(normalize_default_value(None), (1.0, 1.0));
    assert_eq!(normalize_default_value(Some((1.3, -0.1))), (1.0, 0.0));
}

#[test]
fn normalize_step_and_grid_size_use_single_fallback_sources() {
    assert_eq!(normalize_step(None), DEFAULT_STEP);
    assert_eq!(normalize_step(Some(0.2)), 0.2);

    assert_eq!(normalize_grid_size(None), DEFAULT_GRID_SIZE);
    assert_eq!(normalize_grid_size(Some(17)), 17);
}

#[test]
fn reduce_axis_input_keeps_axis_rules_inside_logic() {
    let next_x = reduce_axis_input(
        ColorAreaInteractivity::Active,
        (0.2, 0.8),
        ColorAreaAxis::X,
        Some(0.4),
    );
    assert_eq!(next_x.next_value, Some((0.4, 0.8)));
    assert!(!next_x.prevent_default);

    let next_y = reduce_axis_input(
        ColorAreaInteractivity::Active,
        (0.2, 0.8),
        ColorAreaAxis::Y,
        Some(0.3),
    );
    assert_eq!(next_y.next_value, Some((0.2, 0.3)));
    assert!(!next_y.prevent_default);

    assert_eq!(
        reduce_axis_input(
            ColorAreaInteractivity::Disabled,
            (0.2, 0.8),
            ColorAreaAxis::X,
            Some(0.9)
        ),
        ColorAreaEventOutcome::noop()
    );
    assert_eq!(
        reduce_axis_input(
            ColorAreaInteractivity::Active,
            (0.2, 0.8),
            ColorAreaAxis::X,
            None
        ),
        ColorAreaEventOutcome::noop()
    );
}

#[test]
fn reduce_cell_select_and_keyboard_result_keep_handler_logic_minimal() {
    assert_eq!(
        reduce_cell_select(ColorAreaInteractivity::Active, (1.2, -0.3)).next_value,
        Some((1.0, 0.0))
    );
    assert_eq!(
        reduce_cell_select(ColorAreaInteractivity::Disabled, (0.4, 0.6)),
        ColorAreaEventOutcome::noop()
    );

    let keyboard = reduce_keyboard_result(ColorAreaInteractivity::Active, Some((0.7, 0.3)), true);
    assert_eq!(keyboard.next_value, Some((0.7, 0.3)));
    assert!(keyboard.prevent_default);

    let keyboard_ignored = reduce_keyboard_result(ColorAreaInteractivity::Active, None, true);
    assert_eq!(keyboard_ignored.next_value, None);
    assert!(!keyboard_ignored.prevent_default);

    assert_eq!(
        reduce_keyboard_result(ColorAreaInteractivity::Disabled, Some((0.7, 0.3)), true),
        ColorAreaEventOutcome::noop()
    );
}

#[test]
fn interactivity_is_modeled_as_enum_not_boolean_protocol() {
    assert_eq!(
        ColorAreaInteractivity::from_disabled(false),
        ColorAreaInteractivity::Active
    );
    assert_eq!(
        ColorAreaInteractivity::from_disabled(true),
        ColorAreaInteractivity::Disabled
    );
    assert!(!ColorAreaInteractivity::Active.is_disabled());
    assert!(ColorAreaInteractivity::Disabled.is_disabled());
}

#[test]
fn normalize_root_state_uses_i18n_fallback_and_tracks_sources() {
    let root = normalize_root_state(ColorAreaRootInput {
        class_name: Some(" docs-color-area ".to_string()),
        label: None,
        fallback_label: "Color area i18n".to_string(),
        aria_label: None,
        fallback_aria_label: "Color area aria i18n".to_string(),
        x_axis_label: None,
        fallback_x_axis_label: "Saturation i18n".to_string(),
        y_axis_label: None,
        fallback_y_axis_label: "Lightness i18n".to_string(),
        preview_color: Some("#09f".to_string()),
        value: (0.4, 0.6),
        step: 0.1,
        grid_size: 11,
        disabled: ColorAreaDisableInput {
            is_disabled: Some(false),
        },
    });

    assert_eq!(root.label, "Color area i18n");
    assert_eq!(root.aria_label, "Color area aria i18n");
    assert_eq!(root.x_axis_label, "Saturation i18n");
    assert_eq!(root.y_axis_label, "Lightness i18n");
    assert_eq!(root.preview_color.as_deref(), Some("#09f"));
    assert_eq!(
        root.state.label_source_attr,
        ui_state_primitives::color_area::ColorAreaSourceAttr::Default
    );
    assert_eq!(
        root.state.aria_source_attr,
        ui_state_primitives::color_area::ColorAreaSourceAttr::Default
    );
    assert_eq!(
        root.state.x_axis_source_attr,
        ui_state_primitives::color_area::ColorAreaSourceAttr::Default
    );
    assert_eq!(
        root.state.y_axis_source_attr,
        ui_state_primitives::color_area::ColorAreaSourceAttr::Default
    );
    assert_eq!(
        root.disabled_source_attr,
        ColorAreaDisabledSourceAttr::IsProp
    );
}

#[test]
fn compose_class_name_supports_stable_markers() {
    let state = resolve_state(ColorAreaStateInput {
        disabled: true,
        step: 0.1,
        value: (0.2, 0.8),
        grid_size: 11,
        has_preview_color: true,
        has_custom_label: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
        has_custom_x_axis_label: false,
        has_custom_y_axis_label: false,
    });

    let class_name = compose_class_name(Some("docs-color-area".to_string()), state);
    assert!(class_name.contains("ui-color-area"));
    assert!(class_name.contains("ui-color-area--disabled"));
    assert!(class_name.contains("ui-color-area--with-preview"));
    assert!(class_name.contains("ui-color-area--custom-class"));
    assert!(class_name.contains("docs-color-area"));
}

#[test]
fn resolve_agent_contract_uses_closed_schema_markers() {
    let state = resolve_state(ColorAreaStateInput {
        disabled: false,
        step: 0.1,
        value: (0.2, 0.8),
        grid_size: 11,
        has_preview_color: false,
        has_custom_label: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
        has_custom_x_axis_label: false,
        has_custom_y_axis_label: false,
    });
    let value_axis = normalize_value_axis(true);
    let agent = resolve_agent_contract(state, value_axis);

    assert_eq!(agent.schema_attr, "ui.color-area.agent-contract.v1");
    assert_eq!(agent.stream_support_attr, "optional");
    assert_eq!(agent.stream_fallback_attr, "snapshot");
    assert_eq!(agent.stream_mode_attr, "snapshot");
    assert_eq!(agent.output_status_attr, "verified");
    assert_eq!(agent.intent_attr, "select-color-point");
    assert_eq!(agent.action_attr, "select");
    assert_eq!(agent.state_attr, "active");
    assert_eq!(agent.source_attr, "external");
}
