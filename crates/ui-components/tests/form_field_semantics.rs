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
fn form_field_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/form_field/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "FormField internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn form_field_uses_logic_state_model() {
    let mod_source = load_source("src/form_field/mod.rs");
    let logic_source = load_source("src/form_field/logic.rs");
    let view_source = load_source("src/form_field/view.rs");

    for needle in [
        "pub struct FormFieldStateInput",
        "pub struct FormFieldState",
    ] {
        assert!(
            mod_source.contains(needle),
            "FormField module should include `{needle}` state contracts."
        );
    }

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_label(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_error_message(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "FormField logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_label(label)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_error_message(error_message, invalid)",
        "logic::resolve_state(FormFieldStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "FormField view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn form_field_composes_switch_and_checkbox_indicators() {
    let source = load_source("src/form_field/view.rs");

    for needle in [
        "FormFieldIndicatorVariant::Switch",
        "FormFieldIndicatorVariant::Checkbox",
        "<Switch",
        "<Checkbox",
        "on_change=on_selected_change",
        "checked=selected",
        "set_checked=set_selected",
    ] {
        assert!(
            source.contains(needle),
            "FormField should compose indicator controls with stable contracts (`{needle}`)."
        );
    }
}

#[test]
fn form_field_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/form_field/view.rs");

    for attr in [
        "data-slot=\"form-field\"",
        "data-state=move || state.get().state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-indicator-variant=move || state.get().indicator_variant_attr",
        "data-indicator-placement=move || state.get().indicator_placement_attr",
        "data-message-kind=move || state.get().message_kind_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-slot=\"form-field-content\"",
        "data-slot=\"form-field-indicator\"",
        "data-slot=\"form-field-label\"",
        "data-slot=\"form-field-description\"",
        "data-slot=\"form-field-error\"",
    ] {
        assert!(
            source.contains(attr),
            "FormField should expose `{attr}` for baseline-style state inspection and styling."
        );
    }
}

#[test]
fn form_field_styles_include_state_marker_contracts() {
    let source = load_source("src/form_field/styles.rs");

    for selector in [
        ".ui-form-field--placement-end",
        ".ui-form-field[data-indicator-placement=\"start\"]",
        ".ui-form-field--tone-quiet",
        ".ui-form-field[data-tone=\"default\"]",
        ".ui-form-field--invalid .ui-form-field__label",
        ".ui-form-field[data-disabled=\"true\"]",
        ".ui-form-field__control.ui-switch .ui-switch__label",
        ".ui-form-field--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "FormField styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn form_field_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "pub(super) fn form_field() -> AnyView",
        "title=\"FormField\"",
        "slug=\"form-field\"",
        "description=\"baseline-style form field primitive that composes switch/checkbox indicators with centralized tone/placement/message state derivation and stable slot/data-state markers.\"",
        "<Playground title=\"Switch Indicator + Description\" code_signal=code>",
        "<Playground title=\"Checkbox Indicator + Quiet + Invalid/Disabled\" code_signal=states_code>",
        "<FormField",
    ] {
        assert!(
            source.contains(needle),
            "forms_groups_extra form_field docs page should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn form_field_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "title=\"Switch Indicator + Description\"",
        "id_base=\"docs-form-field-marketing\".to_string()",
        "label=\"Subscribe to product updates\".to_string()",
        "description=\"Receive release notes and occasional best-practice tips.\".to_string()",
        "indicator_placement=FormFieldIndicatorPlacement::Start",
        "title=\"Checkbox Indicator + Quiet + Invalid/Disabled\"",
        "id_base=\"docs-form-field-tos\".to_string()",
        "indicator_variant=FormFieldIndicatorVariant::Checkbox",
        "indicator_placement=FormFieldIndicatorPlacement::End",
        "tone=FormFieldTone::Quiet",
        "invalid=true",
        "error_message=\"Please accept terms to continue.\".to_string()",
        "class_name=\"docs-form-field-custom\".to_string()",
        "id_base=\"docs-form-field-read-only\".to_string()",
        "disabled=true",
        "aria_label=\"Maintenance alerts (read only)\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "form_field docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn form_field_feature_dependency_chain_supports_minimal_component_builds() {
    let cargo_toml = load_source("Cargo.toml");

    assert!(
        cargo_toml
            .contains("component-form_field = [\"component-switch\", \"component-checkbox\"]"),
        "FormField feature dependency chain should include switch/checkbox for minimal-feature builds."
    );
}

#[test]
fn form_field_view_mounts_locale_and_headless_a11y_contracts() {
    let source = load_source("src/form_field/view.rs");

    for needle in [
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "role=\"group\"",
        "aria-describedby=move || describedby.get()",
        "aria-invalid=move || state.get().is_invalid.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "FormField view should include `{needle}` for locale/a11y contract coverage."
        );
    }
}

#[test]
fn form_field_tree_shaking_boundaries_stay_feature_gated() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "#[cfg(feature = \"component-form_field\")]",
        "pub mod form_field;",
        "pub use form_field::{",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib boundary should include `{needle}` for FormField feature gating."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-form_field\")]",
        "out.push_str(crate::form_field::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css boundary should include `{needle}` for FormField feature gating."
        );
    }
}

#[test]
fn form_field_e2e_contract_uses_semantic_selectors_and_settled_waits() {
    let rel = "../../e2e/tests/docs_app_form_field_contract.spec.mjs";
    assert!(
        path_exists(rel),
        "form-field E2E contract file should exist at `{rel}`."
    );

    let source = load_source(rel);
    for needle in [
        "body:not(:has(#boot))",
        "[data-component=\"form-field\"]",
        "#docs-form-field-marketing",
        "data-slot=\"form-field\"",
        "data-slot=\"switch\"",
        "data-slot=\"checkbox\"",
    ] {
        assert!(
            source.contains(needle),
            "form-field E2E contract should include semantic selector/wait marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_e2e_contract_covers_repeatable_key_flow_and_copy_ready_source() {
    let source = load_source("../../e2e/tests/docs_app_form_field_contract.spec.mjs");

    for needle in [
        "page.keyboard.press(\"Enter\")",
        "await page.reload();",
        "Show code|Hide code",
        "data-copyable",
        "Copy to clipboard",
    ] {
        assert!(
            source.contains(needle),
            "form-field E2E contract should include `{needle}` for key-flow and source-copy coverage.",
        );
    }
}

#[test]
fn form_field_check2_marks_component_governance_complete() {
    let check2_source = load_source("src/form_field/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui-components` 定义",
        "- [x] API 命名契约统一",
        "- [x] 如果无异步相关，直接打勾。",
        "- [x] 语义测试优先",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "N/A：`FormField` 当前仅处理同步选择状态与语义标记",
        "`FormField` 归类为 `Streaming Optional`",
        "fallback=snapshot",
    ] {
        assert!(
            check2_source.contains(needle),
            "form_field/check2.md should pin completion marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_marks_forbidden_anti_patterns_complete() {
    let check2_source = load_source("src/form_field/check2.md");

    for needle in [
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
    ] {
        assert!(
            check2_source.contains(needle),
            "form_field/check2.md should mark anti-pattern guard `{needle}` as complete.",
        );
    }
}

#[test]
fn form_field_check2_marks_final_merge_gates_complete() {
    let check2_source = load_source("src/form_field/check2.md");

    for needle in [
        "- [x] 架构正确（边界不破）。",
        "- [x] 行为正确（状态与交互语义成立）。",
        "- [x] 可访问性达标（默认可用）。",
        "- [x] 默认主题美学质量达标（与可访问性同级门禁）。",
        "- [x] 可测试（契约可断言）。",
        "- [x] 可维护（命名和模式一致）。",
        "- [x] 可解释（人和自动化都能读懂）。",
        "- [x] 改动在正确层。",
        "- [x] 命名与全库一致。",
        "- [x] 无效状态被限制或归一化。",
        "- [x] 暴露必要语义标记。",
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
        "- [x] 文档与示例同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "form_field/check2.md should keep final merge-gate marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_has_no_unchecked_checklist_items() {
    let check2_source = load_source("src/form_field/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "FormField check2.md should not keep unchecked checklist items after completion."
    );
}
