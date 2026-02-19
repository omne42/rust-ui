use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join(rel_path).exists()
}

#[test]
fn field_label_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/field_form/field_label/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "FieldLabel internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn field_label_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/field_form/field_label/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::FieldLabel;"),
        "field_label module should export `FieldLabel`."
    );
    assert!(
        crate_source.contains("pub use field_form::field_label::{FieldLabel, FieldLabelTone};"),
        "crate root should re-export FieldLabel contract."
    );
}

#[test]
fn field_label_uses_primitives_and_headless_contract_model() {
    let logic_source = load_source("src/field_form/field_label/logic.rs");
    let view_source = load_source("src/field_form/field_label/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/field_label.rs");
    let headless_source = load_source("../ui-headless/src/field_label.rs");
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");

    for needle in [
        "pub use ui_state_primitives::field_label::{",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_REQUIRED_INDICATOR",
        "DEFAULT_TEXT",
        "FieldLabelState",
        "FieldLabelStateInput",
        "FieldLabelTone",
        "normalize_aria_label",
        "normalize_optional_text",
        "normalize_required_indicator",
        "normalize_text",
        "resolve_state",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldLabel logic should bridge primitives and include `{needle}`."
        );
    }

    for needle in [
        "pub enum FieldLabelTone",
        "pub struct FieldLabelStateInput",
        "pub struct FieldLabelState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_text(",
        "pub fn normalize_required_indicator(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "FieldLabel state primitive should define `{needle}` in ui-state-primitives."
        );
    }

    for needle in [
        "pub struct FieldLabelOptions",
        "pub struct FieldLabelAttrs",
        "pub struct FieldLabelContract",
        "pub fn use_field_label(options: FieldLabelOptions) -> FieldLabelContract",
        "locale_attrs(options.lang, options.dir)",
    ] {
        assert!(
            headless_source.contains(needle),
            "FieldLabel headless contract should include `{needle}`."
        );
    }

    assert!(
        primitive_lib_source.contains("pub mod field_label;"),
        "ui-state-primitives should export `field_label` module."
    );
    assert!(
        headless_lib_source.contains("pub mod field_label;"),
        "ui-headless should export `field_label` module."
    );
    assert!(
        headless_lib_source.contains("use_field_label"),
        "ui-headless should re-export `use_field_label` contract."
    );

    for needle in [
        "logic::normalize_text(text)",
        "logic::normalize_required_indicator(required_indicator)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(FieldLabelStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "use_field_label(FieldLabelOptions {",
        "state: state.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "FieldLabel view should compose logic/headless contracts; missing `{needle}`."
        );
    }
}

#[test]
fn field_label_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/field_form/field_label/view.rs");

    for attr in [
        "data-slot=\"field-label\"",
        "data-tone=move || semantics.get().attrs.data_tone",
        "data-state=move || semantics.get().attrs.data_state",
        "data-required=move || semantics.get().attrs.data_required",
        "data-disabled=move || semantics.get().attrs.data_disabled",
        "data-has-for=move || semantics.get().attrs.data_has_for",
        "data-text-source=move || semantics.get().attrs.data_text_source",
        "data-indicator-source=move || semantics.get().attrs.data_indicator_source",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-custom-class=move || semantics.get().attrs.data_custom_class",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-disabled=move || semantics.get().attrs.aria_disabled",
        "lang=move || semantics.get().attrs.lang",
        "dir=move || semantics.get().attrs.dir",
        "data-slot=\"field-label-text\"",
        "data-slot=\"field-label-required\"",
    ] {
        assert!(
            source.contains(attr),
            "FieldLabel should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn field_label_styles_include_tone_state_and_markers() {
    let source = load_source("src/field_form/field_label/styles.rs");

    for selector in [
        ".ui-field-label {",
        ".ui-field-label--tone-default",
        ".ui-field-label[data-tone=\"default\"]",
        ".ui-field-label--tone-muted",
        ".ui-field-label[data-tone=\"muted\"]",
        ".ui-field-label--tone-strong",
        ".ui-field-label[data-tone=\"strong\"]",
        ".ui-field-label--required",
        ".ui-field-label[data-required=\"true\"]",
        ".ui-field-label--disabled",
        ".ui-field-label[data-disabled=\"true\"]",
        ".ui-field-label--for",
        ".ui-field-label[data-has-for=\"true\"]",
        ".ui-field-label--text-custom",
        ".ui-field-label[data-text-source=\"custom\"]",
        ".ui-field-label--indicator-custom",
        ".ui-field-label[data-indicator-source=\"custom\"]",
        ".ui-field-label--aria-custom",
        ".ui-field-label[data-aria-source=\"custom\"]",
        ".ui-field-label--custom-class",
        ".ui-field-label[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "FieldLabel styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn field_label_docs_page_exists_in_forms_extra() {
    let forms_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_field_label.rs");

    for needle in [
        "pub(super) fn field_label() -> AnyView",
        "title=\"FieldLabel\"",
        "slug=\"field-label\"",
        "<FieldLabel",
    ] {
        assert!(
            forms_extra.contains(needle),
            "forms_extra docs page should contain `{needle}`."
        );
    }
}

#[test]
fn field_label_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_field_label.rs");

    for needle in [
        "pub(super) fn field_label() -> AnyView",
        "title=\"FieldLabel\"",
        "slug=\"field-label\"",
        "description=\"baseline-compatible field label primitive with centralized tone/required/source-state modeling and stable data contracts.\"",
        "<Playground title=\"Tone + Required\" code_signal=tone_code>",
        "<Playground title=\"Custom Indicator + Aria + Class\" code_signal=custom_code>",
        "<FieldLabel",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra_field_label docs page should include `{needle}` for field_label primary playground coverage.",
        );
    }
}

#[test]
fn field_label_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_field_label.rs");

    for needle in [
        "title=\"Tone + Required\"",
        "text=\"Email\".to_string()",
        "for_id=\"docs-field-label-email\".to_string()",
        "required=true",
        "placeholder=\"name@example.com\"",
        "text=\"Helper\".to_string()",
        "tone=FieldLabelTone::Muted",
        "text=\"Critical\".to_string()",
        "tone=FieldLabelTone::Strong",
        "title=\"Custom Indicator + Aria + Class\"",
        "text=\"Assignee\".to_string()",
        "for_id=\"docs-field-label-assignee\".to_string()",
        "required_indicator=\"(required)\".to_string()",
        "aria_label=\"Assignee field label\".to_string()",
        "class_name=\"docs-field-label-custom\".to_string()",
        "placeholder=\"Owner\"",
    ] {
        assert!(
            source.contains(needle),
            "field_label docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn field_label_component_files_follow_expected_layout_and_no_spec_file() {
    let mod_source = load_source("src/field_form/field_label/mod.rs");

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::FieldLabel;",
    ] {
        assert!(
            mod_source.contains(needle),
            "field_label module should include `{needle}`."
        );
    }

    assert!(
        !path_exists("src/field_form/field_label/spec.rs"),
        "field_label should not introduce spec.rs for a simple form primitive."
    );
}

#[test]
fn field_label_check2_marks_core_sections_complete() {
    let source = load_source("src/field_form/field_label/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui-components` 定义",
        "- [x] API 命名契约统一",
        "- [x] 状态归一化集中",
        "- [x] 存在 A11y 实现、国际化与本地化实现",
        "- [x] 状态可观测、可检索、可验证",
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "ui-state-primitives/src/field_label.rs",
        "ui-headless/src/field_label.rs",
        "crates/ui-components/tests/field_label_semantics.rs",
    ] {
        assert!(
            source.contains(needle),
            "FieldLabel check2 should contain completion evidence `{needle}`."
        );
    }
}

#[test]
fn field_label_check2_has_no_unchecked_checklist_items() {
    let source = load_source("src/field_form/field_label/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "field_label check2 should not keep unchecked checklist items"
    );
}
