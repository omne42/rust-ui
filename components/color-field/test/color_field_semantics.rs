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

fn path_exists(rel_path: &str) -> bool {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(rel_path)
        .exists()
}

#[test]
fn color_field_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/color-field/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorField internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_field_consumes_state_primitives_and_centralized_logic() {
    let logic_source = load_source("../../components/color-field/src/logic.rs");
    let view_source = load_source("../../components/color-field/src/view.rs");
    let primitives_source = load_source("../ui-state-primitives/src/color_field.rs");

    for needle in [
        "pub use ui_state_primitives::color_field::{",
        "ColorFieldState",
        "ColorFieldStateInput",
        "resolve_state",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorField logic should re-export `{needle}` from ui-state-primitives."
        );
    }

    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] value: Option<Signal<Option<String>>>",
        "#[prop(optional)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] is_preview_visible: Option<bool>",
        "#[prop(optional)] show_preview: Option<bool>",
        "let default_value = logic::normalize_color_value(default_value);",
        "let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);",
        "let is_preview_visible = logic::resolve_is_preview_visible(is_preview_visible, show_preview);",
        "prop:value=move || logic::resolve_input_value(value.get())",
        "logic::resolve_preview_color(value.get())",
        "logic::resolve_derived_state(logic::ColorFieldDerivedStateInput {",
        "let next = logic::resolve_next_value(event_target_value(&ev));",
        "data-invalid=move || logic::is_invalid_state(state.get()).then_some(\"true\")",
        "aria-invalid=move || logic::is_invalid_state(state.get()).then_some(\"true\")",
        "use_controllable_state(value, Some(default_value), on_value_change)",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "let i18n = use_ui_i18n();",
        "let locale = locale_attrs(logic::normalize_optional_text(lang), dir);",
        "common.clear_aria_label",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorField view should derive state via logic helpers; missing `{needle}`."
        );
    }

    for forbidden in [
        "let (value, set_value) = signal(",
        "set_value.set(",
        "set_value.update(",
        "prop:value=move || value.get().unwrap_or_default()",
        "unwrap_or_default()",
        "let has_value = raw_value.is_some();",
        "let has_valid_value = preview_color.get().is_some();",
        "(state.get().has_value && !state.get().has_valid_value).then_some(\"true\")",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ColorField view should avoid ad-hoc local value state in controlled mode; found `{forbidden}`."
        );
    }

    for needle in [
        "pub const DEFAULT_LABEL",
        "pub const DEFAULT_PLACEHOLDER",
        "pub const DEFAULT_ARIA_LABEL",
        "pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool",
        "pub fn resolve_is_preview_visible(",
        "pub fn resolve_input_value(value: Option<String>) -> String",
        "pub struct ColorFieldDerivedStateInput",
        "pub fn resolve_preview_color(value: Option<String>) -> Option<String>",
        "pub fn resolve_next_value(raw_value: String) -> Option<String>",
        "pub fn resolve_derived_state(input: ColorFieldDerivedStateInput) -> ColorFieldState",
        "pub fn is_invalid_state(state: ColorFieldState) -> bool",
        "pub fn normalize_label(",
        "pub fn normalize_placeholder(",
        "pub fn normalize_aria_label(",
        "pub fn sanitize_preview_color(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            primitives_source.contains(needle),
            "ColorField primitive module should define `{needle}`."
        );
    }
}

#[test]
fn color_field_exposes_baseline_style_data_markers() {
    let source = load_source("../../components/color-field/src/view.rs");

    for attr in [
        "data-slot=\"color-field\"",
        "data-state=move || state.get().visual_state.as_attr()",
        "data-valid=move || state.get().has_valid_value.then_some(\"true\")",
        "data-invalid=move ||",
        "data-has-preview=move || state.get().has_preview.then_some(\"true\")",
        "data-label-source=move || state.get().label_source_attr",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-slot=\"color-field-preview\"",
        "data-slot=\"color-field-input\"",
        "slot_name=\"color-field-clear\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorField should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn color_field_styles_include_valid_invalid_disabled_and_custom_contracts() {
    let source = load_source("../../components/color-field/src/styles.rs");

    for selector in [
        ".ui-color-field",
        ".ui-color-field__control",
        ".ui-color-field__preview",
        ".ui-color-field__input",
        ".ui-color-field[data-state=\"valid\"] .ui-color-field__input",
        ".ui-color-field[data-state=\"invalid\"] .ui-color-field__input",
        ".ui-color-field--disabled",
        ".ui-color-field[data-disabled=\"true\"]",
        ".ui-color-field--custom-class",
        ".ui-color-field[data-custom-class=\"true\"]",
        ".ui-color-field[data-class-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorField styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn color_field_discrete_state_axis_is_modeled_as_typed_enum() {
    let primitives_source = load_source("../ui-state-primitives/src/color_field.rs");
    let view_source = load_source("../../components/color-field/src/view.rs");

    for needle in [
        "pub enum ColorFieldVisualState",
        "pub fn as_attr(self) -> &'static str",
        "pub fn resolve_visual_state(input: ColorFieldStateInput) -> ColorFieldVisualState",
        "pub visual_state: ColorFieldVisualState",
    ] {
        assert!(
            primitives_source.contains(needle),
            "ColorField primitive should keep typed discrete visual state marker `{needle}`."
        );
    }

    assert!(
        view_source.contains("data-state=move || state.get().visual_state.as_attr()"),
        "ColorField view should mount data-state from typed enum mapping."
    );
}

#[test]
fn color_field_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "pub(super) fn color_field() -> AnyView",
        "title=\"ColorField\"",
        "slug=\"color-field\"",
        "title=\"Hello World\"",
        "title=\"Controlled Value\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Invalid + Disabled + Custom Class\"",
        "title=\"Streaming Optional / Snapshot\"",
    ] {
        assert!(
            source.contains(needle),
            "color-field docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn color_field_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "id_base=\"docs-color-field-hello\".to_string()",
        "<Playground title=\"Controlled Value\" code_signal=basic_code>",
        "id_base=\"docs-color-field-basic\".to_string()",
        "label=\"Fill color\".to_string()",
        "value=value.into()",
        "on_value_change=on_value_change",
        "<Playground\n                title=\"Controlled vs Uncontrolled\"",
        "id_base=\"docs-color-field-compare-controlled\".to_string()",
        "label=\"Controlled\".to_string()",
        "id_base=\"docs-color-field-compare-uncontrolled\".to_string()",
        "label=\"Uncontrolled\".to_string()",
        "<Playground title=\"Invalid + Disabled + Custom Class\" code_signal=states_code>",
        "id_base=\"docs-color-field-invalid\".to_string()",
        "default_value=\"javascript:alert(1)\".to_string()",
        "class_name=\"docs-color-field-custom\".to_string()",
        "id_base=\"docs-color-field-disabled\".to_string()",
        "default_value=\"#0ea5e9\".to_string()",
        "is_disabled=true",
        "<Playground title=\"Streaming Optional / Snapshot\" code_signal=output_mode_code>",
        "data-slot=\"color-field-output-mode\"",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "data-ui-output-state=\"snapshot\"",
        "id_base=\"docs-color-field-snapshot\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "color-field docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn color_field_feature_chain_includes_color_swatch_for_preview_contract() {
    let cargo_toml = load_source("Cargo.toml");
    assert!(
        cargo_toml.contains(
            "component-color_field = [\"component-color_swatch\", \"component-clear_button\"]"
        ),
        "component-color_field must depend on component-color_swatch to keep minimal feature compilation green."
    );
}

#[test]
fn color_field_e2e_contract_uses_semantic_selectors_and_settled_waits() {
    let rel = "../../e2e/tests/docs_app_color_field_contract.spec.mjs";
    assert!(
        path_exists(rel),
        "color-field E2E contract file should exist at `{rel}`."
    );

    let source = load_source(rel);
    for needle in [
        "body:not(:has(#boot))",
        "[data-component=\"color-field\"]",
        "#docs-color-field-basic",
        "data-slot=\"color-field\"",
        "data-slot=\"color-field-input\"",
        "data-slot=\"color-field-clear\"",
        "#docs-color-field-disabled",
        "aria-labelledby",
        "aria-label",
    ] {
        assert!(
            source.contains(needle),
            "color-field E2E contract should include semantic selector/wait marker `{needle}`.",
        );
    }
}

#[test]
fn color_field_e2e_contract_covers_repeatable_flow_and_copy_ready_source() {
    let source = load_source("../../e2e/tests/docs_app_color_field_contract.spec.mjs");

    for needle in [
        "await input.focus();",
        "await expect(input).toBeFocused();",
        "input.fill(\"javascript:alert(1)\")",
        "toHaveAttribute(\"aria-invalid\", \"true\")",
        "await clear.focus();",
        "await expect(clear).toBeFocused();",
        "await clear.press(\"Shift+Tab\");",
        "await clear.click();",
        "await page.reload();",
        "Show code|Hide code",
        "data-copyable",
        "Copy to clipboard",
    ] {
        assert!(
            source.contains(needle),
            "color-field E2E contract should include `{needle}` for repeatable flow and copy-ready source coverage.",
        );
    }
}

#[test]
fn color_field_check2_marks_component_governance_complete() {
    let check2_source = load_source("../../components/color-field/src/check2.md");

    assert!(
        !check2_source.contains("- [ ]"),
        "color_field/check2.md should not contain unchecked governance items."
    );

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui` 定义",
        "- [x] 语义测试优先",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "### 0.1 本次执行记录（2026-02-19）",
        "component-color_field` 增加特性链依赖 `component-color_swatch`",
    ] {
        assert!(
            check2_source.contains(needle),
            "color_field/check2.md should pin completion marker `{needle}`.",
        );
    }
}
