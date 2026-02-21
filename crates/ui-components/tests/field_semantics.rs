use std::fs;
use std::path::Path;

fn resolve_source_path(rel_path: &str) -> Option<std::path::PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let mut candidates = vec![manifest_dir.join(rel_path)];

    if let Some(component_rel_path) = rel_path.strip_prefix("../../components/") {
        let direct = workspace_dir.join("components").join(component_rel_path);
        candidates.push(direct.clone());

        let parts: Vec<&str> = component_rel_path.split('/').collect();
        if parts.len() > 3 && parts.get(1) == Some(&"src") && parts.get(2) == parts.first() {
            let collapsed = workspace_dir
                .join("components")
                .join(parts[0])
                .join("src")
                .join(parts[3..].join("/"));
            candidates.push(collapsed);
        }
    }

    if let Some(src_rel_path) = rel_path.strip_prefix("src/") {
        let segments: Vec<&str> = src_rel_path.split('/').collect();
        let components_root = workspace_dir.join("components");

        if let Ok(entries) = fs::read_dir(&components_root) {
            let component_dirs: Vec<String> = entries
                .flatten()
                .filter_map(|entry| {
                    let path = entry.path();
                    path.is_dir()
                        .then(|| entry.file_name().to_string_lossy().to_string())
                })
                .collect();

            for component_dir in component_dirs {
                for start in 0..segments.len() {
                    for end in start..segments.len() {
                        let name = segments[start..=end]
                            .iter()
                            .map(|segment| segment.replace('_', "-"))
                            .collect::<Vec<_>>()
                            .join("-");

                        if name != component_dir {
                            continue;
                        }

                        if end + 1 >= segments.len() {
                            candidates
                                .push(components_root.join(&component_dir).join("src/mod.rs"));
                            candidates
                                .push(components_root.join(&component_dir).join("src/check2.md"));
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                            continue;
                        }

                        let suffix = segments[end + 1..].join("/");
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("src")
                                .join(&suffix),
                        );
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("test")
                                .join(&suffix),
                        );

                        if suffix == "check2.md" {
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                        }
                    }
                }
            }
        }
    }

    candidates.into_iter().find(|path| path.exists())
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_source_path(rel_path)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path));

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}
#[test]
fn field_does_not_expose_logic_or_render_modules() {
    let source = load_source("src/field_form/field/mod.rs");

    for needle in ["pub mod logic", "pub mod render"] {
        assert!(
            !source.contains(needle),
            "Field internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn field_uses_logic_state_model() {
    let logic_source = load_source("src/field_form/field/logic.rs");
    let render_source = load_source("src/field_form/field/view.rs");

    let needle = "pub use ui_state_primitives::field::*;";
    assert!(
        logic_source.contains(needle),
        "Field logic should consume shared state primitives; missing `{needle}`."
    );

    for needle in [
        "use ui_headless::{A11yDirection, FieldOptions, use_field};",
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_error_message(error_message, invalid)",
        "logic::resolve_state(FieldStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "use_field(FieldOptions {",
    ] {
        assert!(
            render_source.contains(needle),
            "Field render should derive state via logic/helpers + headless contract; missing `{needle}`."
        );
    }
}

#[test]
fn field_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/field_form/field/view.rs");

    for attr in [
        "data-slot=\"field\"",
        "data-orientation=move || headless.get().attrs.data_orientation",
        "data-tone=move || headless.get().attrs.data_tone",
        "data-state=move || headless.get().attrs.data_state",
        "data-message-kind=move || headless.get().attrs.data_message_kind",
        "data-required=move || headless.get().attrs.data_required",
        "data-disabled=move || headless.get().attrs.data_disabled",
        "data-invalid=move || headless.get().attrs.data_invalid",
        "data-has-label=move || headless.get().attrs.data_has_label",
        "data-has-description=move || headless.get().attrs.data_has_description",
        "data-has-error=move || headless.get().attrs.data_has_error",
        "data-aria-source=move || headless.get().attrs.data_aria_source",
        "data-error-source=move || headless.get().attrs.data_error_source",
        "data-custom-class=move || headless.get().attrs.data_custom_class",
        "data-class-source=move || headless.get().attrs.data_class_source",
        "lang=move || headless.get().attrs.lang",
        "dir=move || headless.get().attrs.dir",
    ] {
        assert!(
            source.contains(attr),
            "Field should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn field_styles_include_state_markers() {
    let source = load_source("src/field_form/field/styles.rs");

    for selector in [
        ".ui-field--orientation-vertical",
        ".ui-field[data-orientation=\"horizontal\"]",
        ".ui-field--tone-default",
        ".ui-field[data-tone=\"muted\"]",
        ".ui-field--required .ui-field__label",
        ".ui-field[data-required=\"true\"] .ui-field__label",
        ".ui-field--disabled",
        ".ui-field[data-disabled=\"true\"]",
        ".ui-field--invalid .ui-field__control",
        ".ui-field[data-invalid=\"true\"] .ui-field__control",
        ".ui-field--custom-class",
        ".ui-field[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Field styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn field_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn field() -> AnyView",
        "title=\"Field\"",
        "slug=\"field\"",
        "description=\"Form field wrapper with centralized orientation/tone/validation/message-state modeling and stable data contracts.\"",
        "<Playground title=\"Required + Description\" code_signal=required_code>",
        "<Playground title=\"Horizontal + Invalid + Custom Class\" code_signal=invalid_code>",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "<Field",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra field docs page should include `{needle}` for field primary playground coverage.",
        );
    }
}

#[test]
fn field_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "title=\"Required + Description\"",
        "label=\"Email\".to_string()",
        "required=true",
        "description=\"We'll only use this for release notes.\".to_string()",
        "aria_label=\"Email field\".to_string()",
        "placeholder=\"name@example.com\"",
        "title=\"Horizontal + Invalid + Custom Class\"",
        "orientation=FieldOrientation::Horizontal",
        "tone=FieldTone::Muted",
        "invalid=true",
        "error_message=\"A valid email is required\".to_string()",
        "class_name=\"docs-field-custom\".to_string()",
        "placeholder=\"owner@company.com\"",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "test_css_source=workbench_test_css",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"field-workbench-controls\"",
        "FieldActualConfig",
    ] {
        assert!(
            source.contains(needle),
            "field docs playgrounds should contain `{needle}`."
        );
    }
}

#[test]
fn field_check2_keeps_field_scope_and_na_rationale_explicit() {
    let source = load_source("src/field_form/field/check2.md");

    for needle in [
        "已核验（field，2026-02-18）：本组件是表单字段容器",
        "不承载受控 value 轴、远程异步流程、overlay 焦点链路与流式正文渲染职责",
        "相关条目按 N/A 语义核验并保持契约可追溯。",
    ] {
        assert!(
            source.contains(needle),
            "field check2 should keep scoped rationale marker `{needle}`."
        );
    }
}

#[test]
fn field_check2_marks_semantics_streaming_and_docs_contract_complete() {
    let source = load_source("src/field_form/field/check2.md");

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
    ] {
        assert!(
            source.contains(needle),
            "field check2 should mark semantics/docs governance item `{needle}` as complete."
        );
    }
}

#[test]
fn field_check2_marks_final_merge_gates_complete() {
    let source = load_source("src/field_form/field/check2.md");

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
            source.contains(needle),
            "field check2 should keep final merge gate marker `{needle}`."
        );
    }
}

#[test]
fn field_check2_has_no_remaining_unchecked_items() {
    let source = load_source("src/field_form/field/check2.md");

    assert!(
        !source.contains("- [ ]"),
        "field/check2.md should not keep unchecked checklist items once governance is complete."
    );
}

#[test]
fn field_readme_covers_display_config_code_css_test_and_comparisons() {
    let source = load_source("src/field_form/field/README.md");

    for needle in [
        "## Playground 展示区（Display / Config / Code / CSS Test）",
        "## 多场景对比展示",
        "Workbench (Display + Config + Code + CSS Test)",
    ] {
        assert!(
            source.contains(needle),
            "field README should include `{needle}`."
        );
    }
}
