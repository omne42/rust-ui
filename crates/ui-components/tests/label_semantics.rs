use std::fs;
use std::path::Path;

fn resolve_path(rel_path: &str) -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    if let Some(suffix) = rel_path.strip_prefix("src/label/") {
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        return workspace_dir.join("components/label/src").join(suffix);
    }

    manifest_dir.join(rel_path)
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_path(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn label_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/label/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Label internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn label_uses_logic_state_model() {
    let logic_source = load_source("src/label/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/label.rs");
    let view_source = load_source("src/label/view.rs");

    for needle in [
        "pub use ui_state_primitives::label::{",
        "LabelEmphasis",
        "LabelStateInput",
        "normalize_label_text",
        "normalize_required_indicator",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "Label logic should consume ui-state-primitives; missing `{needle}`."
        );
    }

    for needle in [
        "pub enum LabelEmphasis",
        "pub struct LabelStateInput",
        "pub struct LabelState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_label_text(",
        "pub fn normalize_required_indicator(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_attr",
        "indicator_source_attr",
        "class_source_attr",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Label primitive should include `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_label_text(text)",
        "logic::normalize_required_indicator(required_indicator)",
        "logic::normalize_optional_text(for_id)",
        "logic::resolve_state(LabelStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "locale_attrs(logic::normalize_optional_text(lang), dir)",
    ] {
        assert!(
            view_source.contains(needle),
            "Label view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn label_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/label/view.rs");

    for attr in [
        "data-slot=\"label\"",
        "data-emphasis=move || state.get().emphasis_attr",
        "data-state=move || if state.get().is_required { \"required\" } else { \"optional\" }",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-for=move || state.get().has_for_id.then_some(\"true\")",
        "data-label-source=move || state.get().label_source_attr",
        "data-indicator-source=move || state.get().indicator_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"label-text\"",
        "data-slot=\"label-required\"",
    ] {
        assert!(
            source.contains(attr),
            "Label should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn label_styles_include_emphasis_required_and_source_markers() {
    let source = load_source("src/label/styles.rs");

    for selector in [
        ".ui-label--emphasis-default",
        ".ui-label[data-emphasis=\"default\"]",
        ".ui-label--emphasis-subtle",
        ".ui-label--emphasis-strong",
        ".ui-label--required",
        ".ui-label[data-required=\"true\"]",
        ".ui-label--disabled",
        ".ui-label[data-disabled=\"true\"]",
        ".ui-label--for",
        ".ui-label[data-has-for=\"true\"]",
        ".ui-label--text-custom",
        ".ui-label[data-label-source=\"custom\"]",
        ".ui-label--indicator-custom",
        ".ui-label[data-indicator-source=\"custom\"]",
        ".ui-label--custom-class",
        ".ui-label[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Label styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn label_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn label() -> AnyView",
        "title=\"Label\"",
        "slug=\"label\"",
        "description=\"Form label primitive with centralized required/emphasis/source state contracts.\"",
        "title=\"Interactive Playground\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "<Playground title=\"Emphasis + Required\" code_signal=emphasis_code>",
        "<Playground title=\"Custom Indicator + Class\" code_signal=custom_code>",
        "<Label",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs should include `{needle}` for label primary playground coverage.",
        );
    }
}

#[test]
fn label_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "id_base=\"docs-label-workbench-emphasis\".to_string()",
        "options=emphasis_options.clone()",
        "<Switch checked=is_required set_checked=set_is_required>",
        "<Switch checked=is_disabled set_checked=set_is_disabled>",
        "<Switch checked=has_for_id set_checked=set_has_for_id>",
        "<Switch checked=custom_text set_checked=set_custom_text>",
        "<Switch checked=custom_indicator set_checked=set_custom_indicator>",
        "<Switch checked=custom_class set_checked=set_custom_class>",
        "Comparison (Strong + Required + Custom Indicator)",
        "title=\"Emphasis + Required\"",
        "text=\"Name\".to_string()",
        "required=true",
        "emphasis=LabelEmphasis::Subtle",
        "text=\"Critical\".to_string()",
        "emphasis=LabelEmphasis::Strong",
        "title=\"Custom Indicator + Class\"",
        "text=\"Assignee\".to_string()",
        "required_indicator=\"(required)\".to_string()",
        "class_name=\"docs-label-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "label docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn label_readme_documents_docs_workbench_contract() {
    let source = load_source("src/label/README.md");

    for needle in [
        "## Docs Playground（展示 / Config / Code / CSS Test）",
        "forms_extra.rs` 中 `label()`",
        "展示（Preview）",
        "Config：`test_config_signal`",
        "Code：`code_signal`",
        "CSS Test：`test_css_source`",
        "Emphasis + Required",
        "Custom Indicator + Class",
    ] {
        assert!(
            source.contains(needle),
            "label README should include docs-playground marker `{needle}`.",
        );
    }
}

#[test]
fn label_check2_marks_all_items_completed() {
    let source = load_source("src/label/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            source.contains(needle),
            "label/check2.md should keep completed marker `{needle}`.",
        );
    }

    assert!(
        !source.contains("- [ ]"),
        "label/check2.md should not contain unchecked items.",
    );
}
