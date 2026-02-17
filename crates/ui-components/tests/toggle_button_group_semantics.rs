use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn toggle_button_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button/toggle_button_group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ToggleButtonGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn toggle_button_group_uses_logic_state_model() {
    let view_source = load_source("src/button/toggle_button_group/view.rs");
    let logic_source = load_source("src/button/toggle_button_group/logic.rs");

    for needle in [
        "pub struct ToggleButtonGroupState",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub is_attached: bool",
        "pub has_explicit_label: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "ToggleButtonGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let (aria_label, has_explicit_label) = logic::normalize_aria_label(aria_label);",
        "let state = Memo::new(move |_|",
        "logic::resolve_state(orientation, attached, has_explicit_label)",
    ] {
        assert!(
            view_source.contains(needle),
            "ToggleButtonGroup view should derive root state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn toggle_button_group_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/button/toggle_button_group/view.rs");

    for attr in [
        "data-slot=\"toggle-button-group\"",
        "data-orientation=orientation.data_orientation()",
        "data-horizontal=move || state.get().is_horizontal.then_some(\"true\")",
        "data-vertical=move || state.get().is_vertical.then_some(\"true\")",
        "data-attached=move || state.get().is_attached.then_some(\"true\")",
        "data-detached=move || state.get().is_detached.then_some(\"true\")",
        "data-has-explicit-label=move || state.get().has_explicit_label.then_some(\"true\")",
        "data-has-fallback-label=move || state.get().has_fallback_label.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "ToggleButtonGroup should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn toggle_button_group_defaults_accessible_group_label() {
    let source = load_source("src/button/toggle_button_group/logic.rs");

    assert!(
        source.contains("\"Toggle group\".to_string()"),
        "ToggleButtonGroup should provide a stable fallback aria label when none is passed."
    );
}

#[test]
fn toggle_button_group_styles_define_attached_overlap_rule() {
    let source = load_source("src/button/toggle_button_group/styles.rs");

    assert!(
        source.contains("--ui-toggle-button-group-border-overlap"),
        "ToggleButtonGroup styles should define overlap CSS vars for attached layout behavior."
    );
}

#[test]
fn toggle_button_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn toggle_button_group() -> AnyView",
        "title=\"ToggleButtonGroup\"",
        "slug=\"toggle-button-group\"",
        "description=\"Layout wrapper with baseline-style root state attrs for orientation, attachment, and accessible labeling.\"",
        "<Playground",
        "title=\"Attached horizontal\"",
        "code_signal=code",
        "id_base=\"docs-toggle-button-group-orientation\".to_string()",
        "id_base=\"docs-toggle-button-group-variant\".to_string()",
        "id_base=\"docs-toggle-button-group-size\".to_string()",
        "aria_label=\"ToggleButtonGroup orientation\".to_string()",
        "aria_label=\"ToggleButtonGroup variant\".to_string()",
        "aria_label=\"ToggleButtonGroup size\".to_string()",
        "<Switch checked=attached set_checked=set_attached>",
        "title=\"Vertical + detached\"",
        "<ToggleButtonGroup",
        "orientation=ToggleButtonGroupOrientation::Vertical",
        "attached=false",
        "aria_label=\"Alignment controls\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for toggle-button-group coverage.",
        );
    }
}

#[test]
fn toggle_button_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "code_signal=code",
        "let mut toggle_props = String::new();",
        "if orientation != ToggleButtonGroupOrientation::Horizontal {",
        "orientation=ToggleButtonGroupOrientation::{orientation:?}",
        "if attached {",
        "variant=ToggleButtonVariant::{variant:?}",
        "size=ToggleButtonSize::{size:?}",
        "selected=a",
        "set_selected=set_a",
        "selected=b",
        "set_selected=set_b",
        "selected=c",
        "set_selected=set_c",
        "variant=variant",
        "size=size",
        "\"attached selected count: \"",
        "\"detached selected count: \"",
        "selected=left",
        "set_selected=set_left",
        "selected=center",
        "set_selected=set_center",
        "selected=right",
        "set_selected=set_right",
        "variant=ToggleButtonVariant::Secondary",
        "{move || detached_selected_count.get().to_string()}",
    ] {
        assert!(
            source.contains(needle),
            "toggle-button-group docs playgrounds should contain `{needle}`.",
        );
    }
}
