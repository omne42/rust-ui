use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn color_editor_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/color_editor/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorEditor internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_editor_uses_logic_state_model() {
    let logic_source = load_source("src/color_editor/logic.rs");
    let view_source = load_source("src/color_editor/view.rs");

    for needle in [
        "pub const DEFAULT_LABEL",
        "pub const DEFAULT_ARIA_LABEL",
        "pub fn sanitize_color(",
        "pub fn sanitize_hue(",
        "pub fn sanitize_alpha(",
        "pub fn sanitize_area(",
        "pub fn hsb_to_rgb(",
        "pub fn hsb_to_hsl(",
        "pub fn compose_color_from_hsb(",
        "pub fn format_channel_preview(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorEditor logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "let selected_state =",
        "let format_state =",
        "logic::resolve_state(ColorEditorStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "<ColorArea",
        "<ColorSlider",
        "<ColorField",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorEditor view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_editor_exposes_spectrum_style_data_markers() {
    let source = load_source("src/color_editor/view.rs");

    for attr in [
        "data-slot=\"color-editor\"",
        "data-state=move || state.get().data_state_attr",
        "data-format=move || state.get().format_attr",
        "data-alpha=move || state.get().alpha_visibility_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-slot=\"color-editor-canvas\"",
        "data-slot=\"color-editor-sliders\"",
        "data-slot=\"color-editor-formats\"",
        "data-slot=\"color-editor-format-button\"",
        "data-slot=\"color-editor-channels\"",
        "data-slot=\"color-editor-channel-row\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorEditor should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn color_editor_styles_include_format_disabled_alpha_and_custom_contracts() {
    let source = load_source("src/color_editor/styles.rs");

    for selector in [
        ".ui-color-editor",
        ".ui-color-editor__canvas",
        ".ui-color-editor__sliders",
        ".ui-color-editor__format-button",
        ".ui-color-editor__channels",
        ".ui-color-editor--format-hex .ui-color-editor__channels",
        ".ui-color-editor--disabled",
        ".ui-color-editor[data-disabled=\"true\"]",
        ".ui-color-editor--alpha-hidden .ui-color-editor__slider--alpha",
        ".ui-color-editor[data-alpha=\"hidden\"] .ui-color-editor__slider--alpha",
        ".ui-color-editor--custom-class",
        ".ui-color-editor[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorEditor styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn color_editor_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "pub(super) fn color_editor() -> AnyView",
        "title=\"ColorEditor\"",
        "slug=\"color-editor\"",
        "title=\"Controlled Color + Controlled Format\"",
        "title=\"Disabled + Alpha Hidden + Reduced Motion\"",
    ] {
        assert!(
            source.contains(needle),
            "color-editor docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn color_editor_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "<Playground title=\"Controlled Color + Controlled Format\" code=basic_code>",
        "id_base=\"docs-color-editor-basic\".to_string()",
        "selected_color=selected_color_signal",
        "on_selected_change=on_selected_change",
        "format=format_signal",
        "on_format_change=on_format_change",
        "<Playground title=\"Disabled + Alpha Hidden + Reduced Motion\" code=states_code>",
        "id_base=\"docs-color-editor-disabled\".to_string()",
        "default_selected_color=\"#0ea5e9\".to_string()",
        "default_format=ColorEditorFormat::Rgb",
        "hide_alpha_channel=true",
        "disabled=true",
        "class_name=\"docs-color-editor-custom\".to_string()",
        "id_base=\"docs-color-editor-motion\".to_string()",
        "default_format=ColorEditorFormat::Hsb",
        "default_hue=282.0",
        "default_alpha=64.0",
        "default_area=(0.46, 0.88)",
        "motion=reduced_motion",
    ] {
        assert!(
            source.contains(needle),
            "color-editor docs playground should contain `{needle}`.",
        );
    }
}
