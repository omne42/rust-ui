use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn color_editor_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/color/editor/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorEditor internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_editor_uses_logic_state_model() {
    let logic_source = load_source("src/color/editor/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/color_editor.rs");
    let view_source = load_source("src/color/editor/view.rs");

    for needle in [
        "pub use ui_state_primitives::color_editor::{",
        "DEFAULT_LABEL",
        "sanitize_color",
        "sanitize_hue",
        "sanitize_alpha",
        "sanitize_area",
        "compose_color_from_hsb",
        "format_channel_preview",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorEditor logic should re-export `{needle}` from ui-state-primitives."
        );
    }

    for needle in [
        "pub enum ColorEditorFormat",
        "pub struct ColorEditorStateInput",
        "pub struct ColorEditorState",
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
            primitive_source.contains(needle),
            "ColorEditor primitives should include `{needle}` in ui-state-primitives."
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
fn color_editor_exposes_baseline_style_data_markers() {
    let source = load_source("src/color/editor/view.rs");

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
            "ColorEditor should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn color_editor_styles_include_format_disabled_alpha_and_custom_contracts() {
    let source = load_source("src/color/editor/styles.rs");

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
        "title=\"Interactive Playground\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
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
        "id_base=\"docs-color-editor-workbench\".to_string()",
        "id_base=\"docs-color-editor-workbench-compare\".to_string()",
        "options=workbench_format_options.clone()",
        "<Switch checked=workbench_disabled set_checked=set_workbench_disabled>",
        "<Switch checked=workbench_hide_alpha set_checked=set_workbench_hide_alpha>",
        "<Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>",
        "<Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>",
        "<Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>",
        "Comparison (Disabled + Alpha Hidden)",
        "<Playground title=\"Controlled Color + Controlled Format\" code_signal=basic_code>",
        "id_base=\"docs-color-editor-basic\".to_string()",
        "selected_color=selected_color_signal",
        "on_selected_change=on_selected_change",
        "format=format_signal",
        "on_format_change=on_format_change",
        "<Playground title=\"Disabled + Alpha Hidden + Reduced Motion\" code_signal=states_code>",
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

#[test]
fn color_editor_readme_documents_docs_workbench_contract() {
    let source = load_source("src/color/editor/README.md");

    for needle in [
        "## Docs Playground（展示 / Config / Code / CSS Test）",
        "forms_color.rs` 中 `color_editor()`",
        "展示（Preview）",
        "Config：`test_config_signal`",
        "Code：`code_signal`",
        "CSS Test：`test_css_source`",
        "Controlled Color + Controlled Format",
        "Disabled + Alpha Hidden + Reduced Motion",
    ] {
        assert!(
            source.contains(needle),
            "color_editor README should include docs-playground marker `{needle}`.",
        );
    }
}

#[test]
fn color_editor_feature_dependency_chain_covers_composed_children() {
    let source = load_source("Cargo.toml");

    assert!(
        source.contains("component-color_editor = ["),
        "ColorEditor feature should use an explicit dependency list."
    );

    for dependency in [
        "\"component-color_area\"",
        "\"component-color_field\"",
        "\"component-color_slider\"",
        "\"component-color_swatch\"",
        "\"component-slider\"",
    ] {
        assert!(
            source.contains(dependency),
            "ColorEditor feature dependency chain should include `{dependency}`."
        );
    }
}

#[test]
fn color_editor_component_has_motion_contract_module() {
    let mod_source = load_source("src/color/editor/mod.rs");
    let motion_source = load_source("src/color/editor/motion.rs");
    let view_source = load_source("src/color/editor/view.rs");

    for needle in ["pub mod motion;", "pub use motion::ColorEditorMotion;"] {
        assert!(
            mod_source.contains(needle),
            "ColorEditor mod.rs should export motion contract marker `{needle}`."
        );
    }

    for needle in [
        "pub use crate::color_slider::ColorSliderMotion as ColorEditorMotion;",
        "pub fn sanitize_motion(",
        "pub fn source_attr(",
    ] {
        assert!(
            motion_source.contains(needle),
            "ColorEditor motion contract should contain `{needle}`."
        );
    }

    assert!(
        view_source.contains("motion_contract::sanitize_motion(motion)"),
        "ColorEditor view should consume motion contract sanitize path."
    );
}

#[test]
fn color_editor_check2_marks_all_items_completed() {
    let source = load_source("src/color/editor/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] Tree Shaking 是一等能力",
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            source.contains(needle),
            "color_editor/check2.md should keep completed marker `{needle}`."
        );
    }

    assert!(
        !source.contains("- [ ]"),
        "color_editor/check2.md should not contain unchecked items."
    );
}
