use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn color_handle_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/color_handle/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorHandle internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_handle_uses_logic_state_model() {
    let logic_source = load_source("src/color_handle/logic.rs");
    let view_source = load_source("src/color_handle/view.rs");

    for needle in [
        "pub const DEFAULT_ARIA_LABEL",
        "pub fn sanitize_color(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorHandle logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::resolve_state(ColorHandleStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "<ColorThumb",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorHandle view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_handle_exposes_baseline_style_data_markers() {
    let source = load_source("src/color_handle/view.rs");

    for attr in [
        "data-slot=\"color-handle\"",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-dragging=move || state.get().is_dragging.then_some(\"true\")",
        "data-slot=\"color-handle-surface\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorHandle should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn color_handle_styles_include_focus_drag_disabled_and_custom_contracts() {
    let source = load_source("src/color_handle/styles.rs");

    for selector in [
        ".ui-color-handle",
        ".ui-color-handle__surface",
        ".ui-color-handle__thumb.ui-color-thumb",
        ".ui-color-handle--focused .ui-color-handle__surface",
        ".ui-color-handle[data-focused=\"true\"] .ui-color-handle__surface",
        ".ui-color-handle--dragging .ui-color-handle__surface",
        ".ui-color-handle[data-dragging=\"true\"] .ui-color-handle__surface",
        ".ui-color-handle--disabled",
        ".ui-color-handle[data-disabled=\"true\"]",
        ".ui-color-handle--custom-class",
        ".ui-color-handle[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorHandle styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn color_handle_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "pub(super) fn color_handle() -> AnyView",
        "title=\"ColorHandle\"",
        "slug=\"color-handle\"",
        "title=\"Focused + Dragging + Position\"",
        "title=\"Disabled + Custom Class + Loupe Off\"",
    ] {
        assert!(
            source.contains(needle),
            "color-handle docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn color_handle_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "<Playground title=\"Focused + Dragging + Position\" code_signal=basic_code>",
        "id_base=\"docs-color-handle-idle\".to_string()",
        "id_base=\"docs-color-handle-focused\".to_string()",
        "focused=true",
        "id_base=\"docs-color-handle-dragging\".to_string()",
        "dragging=true",
        "<Playground title=\"Disabled + Custom Class + Loupe Off\" code_signal=states_code>",
        "id_base=\"docs-color-handle-disabled\".to_string()",
        "disabled=true",
        "id_base=\"docs-color-handle-custom\".to_string()",
        "show_loupe=false",
        "class_name=\"docs-color-handle-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "color-handle docs playground should contain `{needle}`.",
        );
    }
}
