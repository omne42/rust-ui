use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn labeled_value_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/labeled_value/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "LabeledValue internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn labeled_value_uses_logic_state_model() {
    let logic_source = load_source("src/labeled_value/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/labeled_value.rs");
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");
    let headless_source = load_source("../ui-headless/src/labeled_value.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let view_source = load_source("src/labeled_value/view.rs");

    for needle in [
        "pub use ui_state_primitives::labeled_value::{",
        "LabeledValueOrientation",
        "LabeledValueTone",
        "LabeledValueState",
        "LabeledValueStateInput",
        "normalize_label_text",
        "normalize_value_text",
        "normalize_aria_label",
        "resolve_state",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "LabeledValue logic should bridge ui-state-primitives and include `{needle}`."
        );
    }

    for needle in [
        "pub enum LabeledValueOrientation",
        "pub enum LabeledValueTone",
        "pub struct LabeledValueStateInput",
        "pub struct LabeledValueState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_label_text(",
        "pub fn normalize_value_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "LabeledValue state primitive should define `{needle}` in ui-state-primitives."
        );
    }

    assert!(
        primitive_lib_source.contains("pub mod labeled_value;"),
        "ui-state-primitives should export `labeled_value` module."
    );

    for needle in [
        "pub struct LabeledValueOptions",
        "pub struct LabeledValueContract",
        "pub fn use_labeled_value(options: LabeledValueOptions) -> LabeledValueContract",
        "labeled_group_attrs(options.aria_label, options.lang, options.dir)",
    ] {
        assert!(
            headless_source.contains(needle),
            "LabeledValue headless contract should include `{needle}`."
        );
    }

    assert!(
        headless_lib_source.contains("pub mod labeled_value;"),
        "ui-headless should export `labeled_value` module."
    );
    assert!(
        headless_lib_source.contains("use_labeled_value"),
        "ui-headless should re-export `use_labeled_value` contract."
    );

    for needle in [
        "use ui_headless::{A11yDirection, LabeledValueOptions, use_labeled_value};",
        "logic::normalize_label_text(label)",
        "logic::normalize_value_text(value)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(LabeledValueStateInput {",
        "let semantics = Signal::derive(move || {",
        "use_labeled_value(LabeledValueOptions {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "LabeledValue view should derive state via logic/headless helpers; missing `{needle}`."
        );
    }
}

#[test]
fn labeled_value_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/labeled_value/view.rs");

    for attr in [
        "data-slot=\"labeled-value\"",
        "data-orientation=move || semantics.get().attrs.data_orientation",
        "data-tone=move || semantics.get().attrs.data_tone",
        "data-state=move || semantics.get().attrs.data_state",
        "data-has-description=move || semantics.get().attrs.data_has_description",
        "data-label-source=move || semantics.get().attrs.data_label_source",
        "data-value-source=move || semantics.get().attrs.data_value_source",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-custom-class=move || semantics.get().attrs.data_custom_class",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "data-motion-source=motion_source",
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "lang=move || semantics.get().attrs.lang",
        "dir=move || semantics.get().attrs.dir",
        "data-slot=\"labeled-value-label\"",
        "data-slot=\"labeled-value-value\"",
        "data-slot=\"labeled-value-description\"",
    ] {
        assert!(
            source.contains(attr),
            "LabeledValue should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn labeled_value_styles_include_orientation_tone_and_source_markers() {
    let source = load_source("src/labeled_value/styles.rs");

    for selector in [
        ".ui-labeled-value--orientation-stacked",
        ".ui-labeled-value[data-orientation=\"stacked\"]",
        ".ui-labeled-value--orientation-inline",
        ".ui-labeled-value[data-orientation=\"inline\"]",
        ".ui-labeled-value--tone-default",
        ".ui-labeled-value--tone-subtle",
        ".ui-labeled-value--tone-strong",
        ".ui-labeled-value--with-description",
        ".ui-labeled-value[data-has-description=\"true\"]",
        ".ui-labeled-value--label-custom",
        ".ui-labeled-value[data-label-source=\"custom\"]",
        ".ui-labeled-value--value-custom",
        ".ui-labeled-value[data-value-source=\"custom\"]",
        ".ui-labeled-value--aria-custom",
        ".ui-labeled-value[data-aria-source=\"custom\"]",
        ".ui-labeled-value--custom-class",
        ".ui-labeled-value[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "LabeledValue styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn labeled_value_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn labeled_value() -> AnyView",
        "title=\"LabeledValue\"",
        "slug=\"labeled-value\"",
        "description=\"Label-value pair primitive with centralized orientation/tone/source state contracts and baseline-style data markers.\"",
        "<Playground title=\"Orientation + Tone\" code_signal=orientation_code>",
        "<Playground title=\"Description + Custom Aria/Class\" code_signal=custom_code>",
        "<LabeledValue",
    ] {
        assert!(
            source.contains(needle),
            "display_extra docs should include `{needle}` for labeled_value primary playground coverage.",
        );
    }
}

#[test]
fn labeled_value_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Orientation + Tone\"",
        "label=\"Project\".to_string()",
        "value=\"Omne\".to_string()",
        "orientation=LabeledValueOrientation::Inline",
        "tone=LabeledValueTone::Subtle",
        "title=\"Description + Custom Aria/Class\"",
        "label=\"Build\".to_string()",
        "description=\"Updated 2 minutes ago\".to_string()",
        "aria_label=\"Build status\".to_string()",
        "class_name=\"docs-labeled-value-custom\".to_string()",
        "tone=LabeledValueTone::Strong",
    ] {
        assert!(
            source.contains(needle),
            "labeled_value docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn labeled_value_check2_marks_core_sections_complete() {
    let source = load_source("src/labeled_value/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui-components` 定义",
        "- [x] API 命名契约统一",
        "- [x] 状态归一化集中",
        "- [x] 状态可观测、可检索、可验证",
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "ui-state-primitives/src/labeled_value.rs",
        "ui-headless/src/labeled_value.rs",
        "crates/ui-components/tests/labeled_value_semantics.rs",
    ] {
        assert!(
            source.contains(needle),
            "LabeledValue check2 should contain completion evidence `{needle}`."
        );
    }
}

#[test]
fn labeled_value_check2_has_no_unchecked_checklist_items() {
    let source = load_source("src/labeled_value/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "labeled_value check2 should not keep unchecked checklist items"
    );
}
