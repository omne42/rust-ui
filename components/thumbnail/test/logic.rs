use super::*;
use ui_state_primitives::thumbnail::ThumbnailDataState;

#[test]
fn normalize_input_filters_background_and_class_name() {
    let normalized = normalize_input(
        Some("  #111827 ".to_string()),
        Some(" docs-thumbnail ".to_string()),
    );
    assert_eq!(normalized.background.as_deref(), Some("#111827"));
    assert_eq!(normalized.class_name.as_deref(), Some("docs-thumbnail"));

    let normalized = normalize_input(
        Some("javascript:alert(1)".to_string()),
        Some("   ".to_string()),
    );
    assert_eq!(normalized.background, None);
    assert_eq!(normalized.class_name, None);
}

#[test]
fn normalize_lang_filters_blank_values() {
    assert_eq!(
        normalize_lang(Some("  zh-CN ".to_string())),
        Some("zh-CN".to_string())
    );
    assert_eq!(normalize_lang(Some("   ".to_string())), None);
    assert_eq!(normalize_lang(None), None);
}

#[test]
fn normalize_bool_alias_prefers_is_prefix_and_falls_back_to_legacy() {
    assert_eq!(normalize_bool_alias(Some(true), Some(false)), Some(true));
    assert_eq!(normalize_bool_alias(Some(false), Some(true)), Some(false));
    assert_eq!(normalize_bool_alias(None, Some(true)), Some(true));
    assert_eq!(normalize_bool_alias(None, None), None);
}

#[test]
fn normalize_view_state_input_centralizes_input_boundary_mapping() {
    let normalized = normalize_view_state_input(ThumbnailRawViewStateInput {
        size: ThumbnailSize::Size600,
        is_cover: Some(true),
        is_layer: None,
        is_selected: Some(false),
        is_focused: None,
        cover: Some(false),
        layer: Some(true),
        selected: Some(true),
        focused: Some(true),
        motion: ThumbnailMotion {
            active_scale: 1.08,
            ..ThumbnailMotion::default()
        },
    });

    assert_eq!(normalized.size, ThumbnailSize::Size600);
    assert_eq!(normalized.cover, Some(true));
    assert_eq!(normalized.layer, Some(true));
    assert_eq!(normalized.selected, Some(false));
    assert_eq!(normalized.focused, Some(true));
    assert_eq!(normalized.motion_source, ThumbnailMotionSource::Custom);
}

#[test]
fn compose_class_name_tracks_state_markers() {
    let state = resolve_state(ThumbnailStateInput {
        size: ThumbnailSize::Size600,
        cover: true,
        layer: true,
        selected: true,
        focused: false,
        has_background: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.data_state, ThumbnailDataState::Selected);
    assert_eq!(state.data_state.as_attr(), "selected");

    let class_name = compose_class_name(Some("docs-thumbnail".to_string()), state);
    for token in [
        "ui-thumbnail",
        "ui-thumbnail--size-600",
        "ui-thumbnail--cover",
        "ui-thumbnail--layer",
        "ui-thumbnail--selected",
        "ui-thumbnail--background",
        "ui-thumbnail--custom-class",
        "docs-thumbnail",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn compose_inline_style_maps_background_to_css_variable() {
    assert_eq!(
        compose_inline_style(Some("#111827")),
        Some("--ui-thumbnail-background: #111827;".to_string())
    );
}

#[test]
fn resolve_inline_css_vars_centralizes_default_fallback() {
    assert_eq!(
        resolve_inline_css_vars(Some("#111827")),
        "--ui-thumbnail-background: #111827;"
    );
    assert_eq!(resolve_inline_css_vars(None), "");
}

#[test]
fn resolve_motion_source_tracks_default_vs_custom_motion() {
    assert_eq!(
        resolve_motion_source(ThumbnailMotion::default()),
        ThumbnailMotionSource::Default
    );
    assert_eq!(
        resolve_motion_source(ThumbnailMotion {
            active_scale: 1.08,
            ..ThumbnailMotion::default()
        }),
        ThumbnailMotionSource::Custom
    );
}

#[test]
fn boolean_source_resolves_default_and_prop_inputs() {
    assert_eq!(
        ThumbnailBooleanSource::resolve(None),
        (false, ThumbnailBooleanSource::Default)
    );
    assert_eq!(
        ThumbnailBooleanSource::resolve(Some(true)),
        (true, ThumbnailBooleanSource::Prop)
    );
    assert_eq!(
        ThumbnailBooleanSource::resolve(Some(false)),
        (false, ThumbnailBooleanSource::Prop)
    );
    assert_eq!(ThumbnailBooleanSource::Default.as_attr(), "default");
    assert_eq!(ThumbnailBooleanSource::Prop.as_attr(), "prop");
}

#[test]
fn value_source_uses_closed_default_custom_set() {
    assert_eq!(
        ThumbnailValueSource::from_has_custom_value(false),
        ThumbnailValueSource::Default
    );
    assert_eq!(
        ThumbnailValueSource::from_has_custom_value(true),
        ThumbnailValueSource::Custom
    );
    assert_eq!(ThumbnailValueSource::Default.as_attr(), "default");
    assert_eq!(ThumbnailValueSource::Custom.as_attr(), "custom");
}

#[test]
fn resolve_view_state_centralizes_defaults_and_markers() {
    let view_state = resolve_view_state(
        ThumbnailViewStateInput {
            size: ThumbnailSize::Size600,
            cover: Some(true),
            layer: Some(false),
            selected: Some(true),
            focused: Some(false),
            motion_source: ThumbnailMotionSource::Custom,
        },
        normalize_input(
            Some("#0f172a".to_string()),
            Some("docs-thumbnail-custom".to_string()),
        ),
    );

    assert_eq!(view_state.state.data_state, ThumbnailDataState::Selected);
    assert_eq!(view_state.state.data_state.as_attr(), "selected");
    assert_eq!(view_state.motion_source, ThumbnailMotionSource::Custom);
    assert_eq!(view_state.motion_source.as_attr(), "custom");
    assert_eq!(view_state.motion_source.custom_motion_attr(), Some("true"));
    assert!(view_state.motion_active);
    assert_eq!(view_state.cover_source, ThumbnailBooleanSource::Prop);
    assert_eq!(view_state.layer_source, ThumbnailBooleanSource::Prop);
    assert_eq!(view_state.selected_source, ThumbnailBooleanSource::Prop);
    assert_eq!(view_state.focused_source, ThumbnailBooleanSource::Prop);
    assert_eq!(view_state.background_source, ThumbnailValueSource::Custom);
    assert_eq!(view_state.class_name_source, ThumbnailValueSource::Custom);
    assert_eq!(
        view_state.inline_css_vars,
        "--ui-thumbnail-background: #0f172a;"
    );
    assert!(view_state.class_name.contains("docs-thumbnail-custom"));

    let defaults = resolve_view_state(
        ThumbnailViewStateInput {
            size: ThumbnailSize::Size500,
            cover: None,
            layer: None,
            selected: None,
            focused: None,
            motion_source: ThumbnailMotionSource::Default,
        },
        normalize_input(None, None),
    );
    assert_eq!(defaults.inline_css_vars, "");
    assert_eq!(defaults.motion_source, ThumbnailMotionSource::Default);
    assert_eq!(defaults.motion_source.as_attr(), "default");
    assert_eq!(defaults.motion_source.custom_motion_attr(), None);
    assert!(!defaults.motion_active);
    assert_eq!(defaults.cover_source, ThumbnailBooleanSource::Default);
    assert_eq!(defaults.layer_source, ThumbnailBooleanSource::Default);
    assert_eq!(defaults.selected_source, ThumbnailBooleanSource::Default);
    assert_eq!(defaults.focused_source, ThumbnailBooleanSource::Default);
    assert_eq!(defaults.background_source, ThumbnailValueSource::Default);
    assert_eq!(defaults.class_name_source, ThumbnailValueSource::Default);
}

#[test]
fn resolve_agent_contract_is_schema_typed_and_traceable() {
    let defaults = resolve_view_state(
        ThumbnailViewStateInput {
            size: ThumbnailSize::Size500,
            cover: None,
            layer: None,
            selected: None,
            focused: None,
            motion_source: ThumbnailMotionSource::Default,
        },
        normalize_input(None, None),
    );
    let default_contract = resolve_agent_contract(&defaults);
    assert_eq!(default_contract.schema_name, "ui.thumbnail.agent-contract");
    assert_eq!(default_contract.schema_version.as_str(), "1");
    assert_eq!(default_contract.intent.as_str(), "media-preview");
    assert_eq!(default_contract.action.as_str(), "inspect");
    assert_eq!(default_contract.state.as_str(), "default");
    assert_eq!(default_contract.source.as_str(), "default-only");

    let customized = resolve_view_state(
        ThumbnailViewStateInput {
            size: ThumbnailSize::Size600,
            cover: Some(true),
            layer: Some(true),
            selected: Some(true),
            focused: Some(false),
            motion_source: ThumbnailMotionSource::Custom,
        },
        normalize_input(
            Some("#0f172a".to_string()),
            Some("docs-thumbnail-custom".to_string()),
        ),
    );
    let customized_contract = resolve_agent_contract(&customized);
    assert_eq!(customized_contract.state.as_str(), "selected");
    assert_eq!(customized_contract.source.as_str(), "prop-or-custom");
}
