use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(component_path) = rel_path.strip_prefix("src/") {
        let mut parts = component_path.splitn(2, '/');
        let component = parts.next().unwrap_or_default();
        let Some(suffix) = parts.next() else {
            return fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        };

        let component_dir = component.replace('_', "-");
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/{component_dir}/src/{suffix}"));

        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn color_thumb_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/color-thumb/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorThumb internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_thumb_uses_logic_state_model() {
    let logic_source = load_source("../../components/color-thumb/src/logic.rs");
    let view_source = load_source("../../components/color-thumb/src/view.rs");

    for needle in [
        "pub const DEFAULT_COLOR",
        "pub const DEFAULT_ARIA_LABEL",
        "pub fn sanitize_percent(",
        "pub fn sanitize_color(",
        "pub fn normalize_aria_label(",
        "pub fn position_bucket(",
        "pub fn vertical_bucket(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorThumb logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::resolve_state(ColorThumbStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "<ColorSwatch",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorThumb view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_thumb_exposes_baseline_style_data_markers() {
    let source = load_source("../../components/color-thumb/src/view.rs");

    for attr in [
        "data-slot=\"color-thumb\"",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-dragging=move || state.get().is_dragging.then_some(\"true\")",
        "data-x-bucket=move || state.get().x_bucket_attr",
        "data-y-bucket=move || state.get().y_bucket_attr",
        "data-slot=\"color-thumb-handle\"",
        "data-slot=\"color-thumb-fill\"",
        "data-slot=\"color-thumb-loupe\"",
        "data-slot=\"color-thumb-loupe-fill\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorThumb should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn color_thumb_styles_include_focus_drag_disabled_and_custom_contracts() {
    let source = load_source("../../components/color-thumb/src/styles.rs");

    for selector in [
        ".ui-color-thumb",
        ".ui-color-thumb__handle",
        ".ui-color-thumb__fill",
        ".ui-color-thumb__loupe",
        ".ui-color-thumb--x-start",
        ".ui-color-thumb--x-center",
        ".ui-color-thumb--x-end",
        ".ui-color-thumb--y-start",
        ".ui-color-thumb--y-center",
        ".ui-color-thumb--y-end",
        ".ui-color-thumb--focused .ui-color-thumb__handle",
        ".ui-color-thumb[data-focused=\"true\"] .ui-color-thumb__handle",
        ".ui-color-thumb--dragging .ui-color-thumb__handle",
        ".ui-color-thumb[data-dragging=\"true\"] .ui-color-thumb__handle",
        ".ui-color-thumb--disabled",
        ".ui-color-thumb[data-disabled=\"true\"]",
        ".ui-color-thumb--custom-class",
        ".ui-color-thumb[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorThumb styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn color_thumb_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "pub(super) fn color_thumb() -> AnyView",
        "title=\"ColorThumb\"",
        "slug=\"color-thumb\"",
        "title=\"Focused + Dragging + Position\"",
        "title=\"Disabled + Custom Class + Loupe Off\"",
    ] {
        assert!(
            source.contains(needle),
            "color-thumb docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn color_thumb_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "<Playground title=\"Focused + Dragging + Position\" code_signal=basic_code>",
        "id_base=\"docs-color-thumb-idle\".to_string()",
        "id_base=\"docs-color-thumb-focused\".to_string()",
        "focused=true",
        "id_base=\"docs-color-thumb-dragging\".to_string()",
        "dragging=true",
        "<Playground title=\"Disabled + Custom Class + Loupe Off\" code_signal=states_code>",
        "id_base=\"docs-color-thumb-disabled\".to_string()",
        "disabled=true",
        "id_base=\"docs-color-thumb-custom\".to_string()",
        "show_loupe=false",
        "class_name=\"docs-color-thumb-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "color-thumb docs playground should contain `{needle}`.",
        );
    }
}
