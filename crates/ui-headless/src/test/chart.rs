use super::*;
use ui_state_primitives::chart::{ChartKind, ChartStateInput, resolve_state};

#[test]
fn use_chart_maps_region_locale_and_state_markers() {
    let state = resolve_state(ChartStateInput {
        kind: ChartKind::Line,
        point_count: 4,
        active_index: 2,
        disabled: false,
        show_grid: true,
        is_controlled: true,
        has_custom_class_name: true,
    });

    let contract = use_chart(ChartOptions {
        state,
        aria_label: " Quarterly chart ".to_string(),
        lang: Some(" zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, "region");
    assert_eq!(contract.attrs.aria_label, " Quarterly chart ");
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_kind, "line");
    assert_eq!(contract.attrs.data_state, "ready");
    assert_eq!(contract.attrs.data_controlled, Some("true"));
    assert_eq!(contract.attrs.data_class_source, "custom");
    assert_eq!(contract.state.active_index, 2);
}

#[test]
fn chart_handlers_map_keyboard_to_semantic_intents() {
    let handlers = ChartHandlers;

    assert_eq!(
        handlers.on_key_down("ArrowRight", 0, 3, false),
        ChartKeyAction::MoveTo(1)
    );
    assert_eq!(
        handlers.on_key_down("Enter", 1, 3, false),
        ChartKeyAction::Activate(1)
    );
    assert_eq!(
        handlers.on_key_down("ArrowLeft", 1, 3, true),
        ChartKeyAction::Noop
    );
    assert_eq!(
        handlers.on_key_down("Escape", 1, 3, false),
        ChartKeyAction::Noop
    );
    assert_eq!(
        handlers.on_key_down("Enter", 0, 0, false),
        ChartKeyAction::Noop
    );
}

#[test]
fn chart_handlers_normalize_pointer_focus_and_click_interactions() {
    let handlers = ChartHandlers;

    assert_eq!(
        handlers.on_pointer_enter(2, 4, false),
        ChartKeyAction::MoveTo(2)
    );
    assert_eq!(handlers.on_focus(7, 4, false), ChartKeyAction::MoveTo(3));
    assert_eq!(handlers.on_click(1, 4, false), ChartKeyAction::Activate(1));
    assert_eq!(handlers.on_pointer_enter(1, 0, false), ChartKeyAction::Noop);
    assert_eq!(handlers.on_click(1, 4, true), ChartKeyAction::Noop);
}

#[test]
fn chart_handlers_build_accessible_point_labels() {
    let handlers = ChartHandlers;
    assert_eq!(handlers.point_aria_label("Q1", 42.0), "Q1 42.00");
}
