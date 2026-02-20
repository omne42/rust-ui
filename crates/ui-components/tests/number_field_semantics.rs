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
fn number_field_uses_headless_spinbutton_semantics() {
    let source = load_source("src/text_input/number_field/view.rs");

    assert!(
        source.contains("use_number_field"),
        "NumberField should delegate keyboard/editing behavior to `ui_headless::use_number_field`."
    );
    assert!(
        source.contains("role=number_field.input.role"),
        "NumberField input should use spinbutton role from the headless hook."
    );
    assert!(
        source.contains("aria-valuenow"),
        "NumberField input should expose `aria-valuenow` for spinbutton semantics."
    );
    assert!(
        source.contains("aria-valuemin"),
        "NumberField input should expose `aria-valuemin` when min is configured."
    );
    assert!(
        source.contains("aria-valuemax"),
        "NumberField input should expose `aria-valuemax` when max is configured."
    );
    assert!(
        source.contains("on:keydown=on_key_down"),
        "NumberField should handle ArrowUp/ArrowDown/PageUp/PageDown keys for stepping."
    );
}

#[test]
fn number_field_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/text_input/number_field/view.rs");

    for attr in [
        "data-focused",
        "data-focus-visible",
        "data-invalid",
        "data-disabled",
        "data-required",
    ] {
        assert!(
            source.contains(attr),
            "NumberField should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn number_field_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn number_field() -> AnyView",
        "title=\"NumberField\"",
        "slug=\"number-field\"",
        "description=\"Numeric input with steppers and keyboard control.\"",
        "<Playground title=\"Stepper\" code_signal=code>",
        "<NumberField",
    ] {
        assert!(
            source.contains(needle),
            "forms docs should include `{needle}` for number-field primary playground coverage.",
        );
    }
}

#[test]
fn number_field_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Stepper\"",
        "id=\"docs-number-field\".to_string()",
        "label=\"Quantity\".to_string()",
        "min=0",
        "max=100",
        "value: ",
    ] {
        assert!(
            source.contains(needle),
            "number-field docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn number_field_docs_includes_state_matrix_comparison_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"State Matrix\"",
        "id=\"docs-number-field-default\".to_string()",
        "id=\"docs-number-field-required\".to_string()",
        "id=\"docs-number-field-invalid\".to_string()",
        "id=\"docs-number-field-disabled\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "number-field state matrix playground should contain `{needle}`.",
        );
    }
}

#[test]
fn number_field_docs_interactive_playground_exposes_config_code_css_test_sections() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Interactive Playground\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "controls=move || view! {",
        "data-slot=\"number-field-workbench-controls\"",
        "id_base=\"docs-number-field-workbench-bounds\".to_string()",
        "id_base=\"docs-number-field-workbench-step\".to_string()",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/text-input/src/number_field/styles.rs\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "number-field interactive docs playground should include `{needle}`.",
        );
    }
}

#[test]
fn number_field_check2_marks_core_sections_complete() {
    let source = load_source("src/text_input/number_field/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
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
        "### 10. NumberField 本轮验收证据",
        "component-number_field -> component-button",
        "components/text-input/src/number_field/view.rs",
    ] {
        assert!(
            source.contains(needle),
            "NumberField check2 should contain completion evidence `{needle}`."
        );
    }
}

#[test]
fn number_field_check2_has_no_unchecked_checklist_items() {
    let source = load_source("src/text_input/number_field/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "number_field check2 should not keep unchecked checklist items"
    );
}

#[test]
fn number_field_text_metrics_use_typography_tokens() {
    let source = load_source("src/text_input/number_field/styles.rs");

    for needle in [
        "--ui-number-field-label-font-size: var(--ui-font-size-150);",
        "--ui-number-field-label-line-height: var(--ui-line-height-150);",
        "--ui-number-field-input-font-size: var(--ui-font-size-150);",
        "--ui-number-field-input-line-height: var(--ui-line-height-150);",
        "--ui-number-field-meta-font-size: var(--ui-font-size-100);",
        "--ui-number-field-meta-line-height: var(--ui-line-height-100);",
    ] {
        assert!(
            source.contains(needle),
            "NumberField styles should include tokenized text metric `{needle}`."
        );
    }

    for forbidden in ["font-size: 14px;", "font-size: 13px;", "font-size: 12px;"] {
        assert!(
            !source.contains(forbidden),
            "NumberField styles should not hardcode legacy text size `{forbidden}`."
        );
    }
}
