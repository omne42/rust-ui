use super::*;

#[test]
fn state_and_interaction_attrs_follow_contract() {
    assert_eq!(state_attr_for_selected(true), "selected");
    assert_eq!(state_attr_for_selected(false), "unselected");

    assert_eq!(
        interaction_attr(true, false, false, false, false),
        "disabled"
    );
    assert_eq!(interaction_attr(false, true, true, true, true), "pressed");
    assert_eq!(interaction_attr(false, false, true, true, true), "hovered");
    assert_eq!(
        interaction_attr(false, false, false, true, true),
        "focus-visible"
    );
    assert_eq!(
        interaction_attr(false, false, false, false, true),
        "focused"
    );
    assert_eq!(interaction_attr(false, false, false, false, false), "idle");
}

#[test]
fn variant_and_size_attrs_match_contract() {
    assert_eq!(variant_attr(ToggleVariant::Default), "default");
    assert_eq!(variant_attr(ToggleVariant::Accent), "accent");
    assert_eq!(variant_attr(ToggleVariant::Destructive), "destructive");
    assert_eq!(variant_attr(ToggleVariant::Outline), "outline");
    assert_eq!(variant_attr(ToggleVariant::Secondary), "secondary");
    assert_eq!(variant_attr(ToggleVariant::Ghost), "ghost");

    assert_eq!(size_attr(ToggleSize::Xs), "xs");
    assert_eq!(size_attr(ToggleSize::S), "s");
    assert_eq!(size_attr(ToggleSize::M), "m");
    assert_eq!(size_attr(ToggleSize::L), "l");
    assert_eq!(size_attr(ToggleSize::Xl), "xl");
    assert_eq!(size_attr(ToggleSize::IconXs), "icon-xs");
    assert_eq!(size_attr(ToggleSize::IconS), "icon-s");
    assert_eq!(size_attr(ToggleSize::IconM), "icon-m");
    assert_eq!(size_attr(ToggleSize::IconL), "icon-l");
    assert_eq!(size_attr(ToggleSize::IconXl), "icon-xl");

    assert_eq!(size_attr(ToggleSize::Default), "m");
    assert_eq!(size_attr(ToggleSize::Sm), "s");
    assert_eq!(size_attr(ToggleSize::Lg), "l");
    assert_eq!(size_attr(ToggleSize::Icon), "icon-m");
    assert_eq!(size_attr(ToggleSize::IconSm), "icon-s");
    assert_eq!(size_attr(ToggleSize::IconLg), "icon-l");
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("   \n".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-toggle  ".to_string())),
        Some("docs-toggle".to_string())
    );
}

#[test]
fn resolve_state_tracks_source_and_interaction_markers() {
    let state = resolve_state(ToggleStateInput {
        selected: true,
        disabled: false,
        hovered: true,
        pressed_interaction: false,
        focused: true,
        focus_visible: true,
        variant: ToggleVariant::Outline,
        size: ToggleSize::Sm,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_custom_aria_label: true,
        has_on_pressed_change: true,
    });

    assert_eq!(state.state_attr, "selected");
    assert_eq!(state.interaction_attr, "hovered");
    assert_eq!(state.variant_attr, "outline");
    assert_eq!(state.size_attr, "s");
    assert_eq!(state.variant_source_attr, "custom");
    assert_eq!(state.size_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.handler_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-toggle".to_string()),
        resolve_state(ToggleStateInput {
            selected: false,
            disabled: false,
            hovered: false,
            pressed_interaction: false,
            focused: false,
            focus_visible: false,
            variant: ToggleVariant::Outline,
            size: ToggleSize::Sm,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_aria_label: false,
            has_on_pressed_change: false,
        }),
    );

    for token in [
        "ui-toggle",
        "ui-toggle-button",
        "ui-toggle-button--variant-outline",
        "ui-toggle-button--size-s",
        "ui-toggle--custom-motion",
        "ui-toggle--custom-class",
        "docs-toggle",
    ] {
        assert!(
            class_name.contains(token),
            "toggle class name should include `{token}`"
        );
    }
}

#[cfg(feature = "component-toggle_group")]
#[test]
fn toggle_group_orientation_and_selection_mode_attrs_are_stable() {
    assert_eq!(
        ToggleGroupOrientation::Horizontal.class_name(),
        "ui-toggle-group--horizontal"
    );
    assert_eq!(
        ToggleGroupOrientation::Vertical.class_name(),
        "ui-toggle-group--vertical"
    );
    assert_eq!(ToggleGroupOrientation::Horizontal.as_attr(), "horizontal");
    assert_eq!(ToggleGroupOrientation::Vertical.as_attr(), "vertical");

    assert_eq!(
        ToggleGroupSelectionMode::Multiple.class_name(),
        "ui-toggle-group--mode-multiple"
    );
    assert_eq!(
        ToggleGroupSelectionMode::Single.class_name(),
        "ui-toggle-group--mode-single"
    );
    assert_eq!(ToggleGroupSelectionMode::Multiple.as_attr(), "multiple");
    assert_eq!(ToggleGroupSelectionMode::Single.as_attr(), "single");
}

#[cfg(feature = "component-toggle_group")]
#[test]
fn normalize_and_sanitize_toggle_group_selected_ids_filter_unknown_and_disabled() {
    let items = normalize_toggle_group_items(vec![
        ToggleGroupItem::new("bold", "Bold"),
        ToggleGroupItem::new("italic", "Italic").disabled(true),
    ]);
    let item_ids = collect_toggle_group_item_ids(&items);

    let selected = BTreeSet::from([
        "bold".to_string(),
        "italic".to_string(),
        "missing".to_string(),
    ]);

    let selected = sanitize_toggle_group_selected_ids(
        selected,
        &item_ids,
        &items,
        ToggleGroupSelectionMode::Multiple,
    );

    assert_eq!(selected, BTreeSet::from(["bold".to_string()]));
}

#[cfg(feature = "component-toggle_group")]
#[test]
fn toggle_toggle_group_selected_id_respects_selection_mode() {
    let items = normalize_toggle_group_items(vec![
        ToggleGroupItem::new("bold", "Bold"),
        ToggleGroupItem::new("italic", "Italic"),
    ]);
    let item_ids = collect_toggle_group_item_ids(&items);

    let selected = toggle_toggle_group_selected_id(
        BTreeSet::from(["bold".to_string()]),
        "italic",
        &item_ids,
        &items,
        ToggleGroupSelectionMode::Single,
        true,
    );
    assert_eq!(selected, BTreeSet::from(["italic".to_string()]));

    let selected = toggle_toggle_group_selected_id(
        BTreeSet::from(["bold".to_string()]),
        "italic",
        &item_ids,
        &items,
        ToggleGroupSelectionMode::Multiple,
        true,
    );
    assert_eq!(
        selected,
        BTreeSet::from(["bold".to_string(), "italic".to_string()])
    );

    let selected = toggle_toggle_group_selected_id(
        BTreeSet::from(["bold".to_string(), "italic".to_string()]),
        "bold",
        &item_ids,
        &items,
        ToggleGroupSelectionMode::Multiple,
        false,
    );
    assert_eq!(selected, BTreeSet::from(["italic".to_string()]));
}

#[cfg(feature = "component-toggle_group")]
#[test]
fn compose_toggle_group_class_name_includes_state_and_custom_markers() {
    let state = resolve_toggle_group_state(ToggleGroupStateInput {
        orientation: ToggleGroupOrientation::Vertical,
        selection_mode: ToggleGroupSelectionMode::Single,
        disabled: false,
        attached: true,
        item_count: 3,
        selected_count: 1,
        disabled_item_count: 1,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    let class_name = compose_toggle_group_class_name(Some("docs-toggle-group".to_string()), state);
    assert!(class_name.contains("ui-toggle-group"));
    assert!(class_name.contains("ui-toggle-group--vertical"));
    assert!(class_name.contains("ui-toggle-group--mode-single"));
    assert!(class_name.contains("ui-toggle-group--attached"));
    assert!(class_name.contains("ui-toggle-group--has-selection"));
    assert!(class_name.contains("ui-toggle-group--custom-class"));
    assert!(class_name.contains("docs-toggle-group"));
}
