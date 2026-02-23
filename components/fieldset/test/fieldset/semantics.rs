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
fn path_exists(rel_path: &str) -> bool {
    resolve_source_path(rel_path).is_some()
}
#[test]
fn fieldset_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/field_form/fieldset/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Fieldset internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn fieldset_consumes_state_primitives_and_keeps_component_assembly_local() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/fieldset.rs");

    for needle in [
        "pub use ui_state_primitives::fieldset::{",
        "FieldsetOrientation",
        "FieldsetTone",
        "FieldsetState",
        "FieldsetStateInput",
        "normalize_aria_label",
        "normalize_error_message",
        "normalize_optional_text",
        "resolve_state",
        "pub struct FieldsetViewStateInput",
        "pub struct FieldsetViewState",
        "pub fn resolve_view_state(input: FieldsetViewStateInput) -> FieldsetViewState",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Fieldset logic should include `{needle}` to consume ui-state-primitives and keep only assembly logic."
        );
    }

    for needle in [
        "pub enum FieldsetOrientation",
        "pub enum FieldsetTone",
        "pub enum FieldsetMessageKind",
        "pub enum FieldsetDataState",
        "pub struct FieldsetStateInput",
        "pub struct FieldsetState",
        "pub fn normalize_aria_label(",
        "pub fn normalize_error_message(",
        "pub fn resolve_state(input: FieldsetStateInput) -> FieldsetState",
    ] {
        assert!(
            primitives_source.contains(needle),
            "fieldset primitive module should define `{needle}`."
        );
    }

    for needle in [
        "let required_axis = logic::normalize_boolean_axis(logic::FieldsetBooleanAxisInput {",
        "let disabled_axis = logic::normalize_boolean_axis(logic::FieldsetBooleanAxisInput {",
        "let invalid_axis = logic::normalize_boolean_axis(logic::FieldsetBooleanAxisInput {",
        "let required_state = use_controllable_state(",
        "Some(required_axis.initial_value),",
        "on_is_required_change,",
        "let disabled_state = use_controllable_state(",
        "Some(disabled_axis.initial_value),",
        "on_is_disabled_change,",
        "let invalid_state = use_controllable_state(",
        "Some(invalid_axis.initial_value),",
        "on_is_invalid_change,",
        "required: required.get(),",
        "disabled: disabled.get(),",
        "invalid: invalid.get(),",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_view_state(logic::FieldsetViewStateInput {",
        "let state = Memo::new(move |_| state_view_state.get().state);",
        "logic::compose_class_name(resolved.class_name.clone(), resolved.state)",
        "logic::FieldsetMessageKind::Description",
        "logic::FieldsetMessageKind::Error",
    ] {
        assert!(
            view_source.contains(needle),
            "Fieldset view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn fieldset_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/field_form/fieldset/view.rs");

    for attr in [
        "style=move || motion_style.get_value()",
        "lang=move || a11y.get().lang.clone()",
        "dir=move || a11y.get().dir",
        "data-slot=\"fieldset\"",
        "data-orientation=move || state.get().orientation_attr",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-message-kind=move || state.get().message_kind_attr",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-required-source=move || view_state.get().required_source_attr",
        "data-required-control-mode=move || view_state.get().required_control_mode_attr",
        "data-required-change-source=move || view_state.get().required_change_source_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-disabled-source=move || view_state.get().disabled_source_attr",
        "data-disabled-control-mode=move || view_state.get().disabled_control_mode_attr",
        "data-disabled-change-source=move || view_state.get().disabled_change_source_attr",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-invalid-source=move || view_state.get().invalid_source_attr",
        "data-invalid-control-mode=move || view_state.get().invalid_control_mode_attr",
        "data-invalid-change-source=move || view_state.get().invalid_change_source_attr",
        "data-has-legend=move || state.get().has_legend.then_some(\"true\")",
        "data-has-description=move || state.get().has_description.then_some(\"true\")",
        "data-has-error=move || state.get().has_error_message.then_some(\"true\")",
        "data-has-actions=move || state.get().has_actions.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-ui-schema=move || agent_contract.get().schema_attr",
        "data-ui-schema-version=move || agent_contract.get().schema_version_attr",
        "data-ui-intent=move || agent_contract.get().intent_attr",
        "data-ui-action=move || agent_contract.get().action_attr",
        "data-ui-state=move || agent_contract.get().state_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
        "data-slot=\"fieldset-field-group\"",
        "data-slot=\"fieldset-actions\"",
    ] {
        assert!(
            source.contains(attr),
            "Fieldset should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn fieldset_styles_include_state_markers() {
    let source = load_source("src/field_form/fieldset/styles.rs");

    for selector in [
        ".ui-fieldset--orientation-vertical",
        ".ui-fieldset[data-orientation=\"horizontal\"]",
        ".ui-fieldset--tone-default",
        ".ui-fieldset[data-tone=\"muted\"]",
        ".ui-fieldset--required .ui-fieldset__legend",
        ".ui-fieldset[data-required=\"true\"] .ui-fieldset__legend",
        ".ui-fieldset--disabled",
        ".ui-fieldset[data-disabled=\"true\"]",
        ".ui-fieldset--invalid .ui-fieldset__group",
        ".ui-fieldset[data-invalid=\"true\"] .ui-fieldset__group",
        ".ui-fieldset--custom-class",
        ".ui-fieldset[data-custom-class=\"true\"]",
        ".ui-fieldset[data-class-source=\"custom\"]",
        "font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));",
        "font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
        "line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-accent, var(--ui-fallback-accent))",
    ] {
        assert!(
            source.contains(selector),
            "Fieldset styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn fieldset_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn fieldset() -> AnyView",
        "title=\"Fieldset\"",
        "slug=\"fieldset\"",
        "description=\"baseline-style fieldset primitive with centralized orientation/tone/validation/message/action-state modeling and stable data contracts.\"",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Legend + Description\" code_signal=default_code>",
        "<Playground title=\"Horizontal + Invalid + Actions\" code_signal=invalid_code>",
        "title=\"Fieldset Workbench (Display + Config + Code + CSS Test)\"",
        "test_source_path=\"crates/ui/src/field_form/fieldset/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "orientation=FieldsetOrientation::Horizontal",
        "tone=FieldsetTone::Muted",
        "is_invalid=true",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should include `{needle}` for fieldset primary coverage.",
        );
    }
}

#[test]
fn fieldset_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "title=\"Hello World\"",
        "<Fieldset legend=\"Channels\".to_string()>",
        "title=\"Legend + Description\"",
        "legend=\"Notification channels\".to_string()",
        "description=\"Pick every channel you want to receive release updates from.\".to_string()",
        "is_required=true",
        "aria_label=\"Notification channel group\".to_string()",
        "<span>\"Email\"</span>",
        "<span>\"SMS\"</span>",
        "<span>\"Push\"</span>",
        "orientation=FieldsetOrientation::Horizontal",
        "tone=FieldsetTone::Muted",
        "is_invalid=true",
        "error_message=\"Pick at least one channel\".to_string()",
        "class_name=\"docs-fieldset-custom\".to_string()",
        "variant=ui::ButtonVariant::Secondary",
        "size=ui::ButtonSize::Sm",
        "\"Manage channels\"",
        "data-slot=\"fieldset-workbench-controls\"",
        "data-slot=\"fieldset-workbench-compare\"",
        "\"Scenario compare\"",
        "Switch checked=workbench_required set_checked=set_workbench_required",
        "Switch checked=workbench_invalid set_checked=set_workbench_invalid",
        "Switch checked=workbench_show_actions set_checked=set_workbench_show_actions",
    ] {
        assert!(
            source.contains(needle),
            "fieldset docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn fieldset_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_fieldset_contract.spec.mjs");
    let check2_source = load_source("../../components/fieldset/check2.md");
    let check2_source_src = load_source("src/field_form/fieldset/check2.md");

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "title=\"Fieldset Workbench (Display + Config + Code + CSS Test)\"",
        "title=\"Controlled vs Uncontrolled (Snapshot Contrast)\"",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "default_is_invalid=true",
        "is_invalid=controlled_invalid_signal",
        "on_is_invalid_change=on_controlled_invalid_change",
        "Streaming fallback=snapshot: waiting for final validation",
        "Inspect data-ui-stream-support/data-ui-stream-fallback/data-ui-stream-mode.",
        "data-slot=\"fieldset-workbench-compare\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "fieldset docs product surface should include `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "code_imports",
        "CodeBlock",
        "copyable=true",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground source should keep copy-ready import contract marker `{needle}`."
        );
    }

    for needle in [
        "docs-app fieldset docs product surface covers hello/state/controlled/streaming playgrounds",
        "h2:has-text(\"Hello World\")",
        "h2:has-text(\"Controlled vs Uncontrolled (Snapshot Contrast)\")",
        "h2:has-text(\"Streaming Optional (fallback=snapshot)\")",
        "toContainText(\"use leptos::prelude::*;\")",
        "toContainText(\"use ui::*;\")",
        "toContainText(\"<Fieldset\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "fieldset docs e2e should keep copy-paste-ready assertion `{needle}`."
        );
    }

    for source in [&check2_source, &check2_source_src] {
        for needle in [
            "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
            "fieldset_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
            "scripts/check-ui-dx.sh",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 docs-product section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_check2_marks_docs_product_copy_paste_ready_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。"),
            "fieldset check2 should mark docs-product copy-paste-ready item complete."
        );

        for needle in [
            "forms_extra.rs::fieldset",
            "docs_app_fieldset_contract.spec.mjs",
            "DEFAULT_PLAYGROUND_IMPORTS",
            "fieldset_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
            "fieldset_dx_check_script_covers_docs_product_copy_paste_ready_contract",
            "scripts/check-ui-dx.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 docs-product section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_check2_documents_docs_sync_and_state_matrix_rules() {
    for checklist_source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        for required in [
            "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
            "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
            "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
            "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
        ] {
            assert!(
                checklist_source.contains(required),
                "fieldset check2 should keep docs-sync/state-matrix rule `{required}`."
            );
        }
    }
}

#[test]
fn fieldset_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let check2_source = load_source("../../components/fieldset/check2.md");

    for needle in [
        "#[prop(optional)] orientation: FieldsetOrientation,",
        "#[prop(optional)] tone: FieldsetTone,",
        "#[prop(optional)] default_is_required: Option<bool>,",
        "#[prop(optional)] on_is_required_change: Option<Callback<bool>>,",
        "#[prop(optional)] default_is_disabled: Option<bool>,",
        "#[prop(optional)] on_is_disabled_change: Option<Callback<bool>>,",
        "#[prop(optional)] default_is_invalid: Option<bool>,",
        "#[prop(optional)] on_is_invalid_change: Option<Callback<bool>>,",
        "value_source_attr: \"is_invalid\"",
        "default_source_attr: \"default_is_invalid\"",
        "change_source_attr: \"on_is_invalid_change\"",
        "pub fn resolve_view_state(input: FieldsetViewStateInput) -> FieldsetViewState",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "fieldset API/default contract should keep marker `{needle}` for docs sync."
        );
    }

    for needle in [
        "title=\"Legend + Description\"",
        "title=\"Horizontal + Invalid + Actions\"",
        "title=\"Controlled vs Uncontrolled (Snapshot Contrast)\"",
        "title=\"Fieldset Workbench (Display + Config + Code + CSS Test)\"",
        "default_is_invalid=true",
        "is_invalid=controlled_invalid_signal",
        "on_is_invalid_change=on_controlled_invalid_change",
        "Switch checked=workbench_required set_checked=set_workbench_required",
        "Switch checked=workbench_disabled set_checked=set_workbench_disabled",
        "Switch checked=workbench_invalid set_checked=set_workbench_invalid",
        "Switch checked=workbench_show_actions set_checked=set_workbench_show_actions",
        "orientation=orientation",
        "tone=tone",
        "is_required=required",
        "is_disabled=disabled",
        "is_invalid=invalid",
        "FieldsetActualConfig {",
        "orientation: {orientation:?}",
        "tone: {tone:?}",
        "is_required: {required}",
        "is_disabled: {disabled}",
        "is_invalid: {invalid}",
        "data-slot=\"fieldset-workbench-compare\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "fieldset docs should keep synced example/matrix marker `{needle}`."
        );
    }

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "fieldset_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "scripts/check-ui-dx.sh",
        "forms_extra.rs::fieldset",
        "FieldsetActualConfig",
    ] {
        assert!(
            check2_source.contains(needle),
            "components/fieldset/check2.md should keep docs-sync evidence marker `{needle}`."
        );
    }
}

#[test]
fn fieldset_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: fieldset docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include docs-sync/state-matrix marker `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_docs_sync_and_state_matrix_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
            "fieldset check2 should mark docs-sync/state-matrix checklist item complete."
        );

        for needle in [
            "forms_extra.rs::fieldset",
            "Fieldset Workbench (Display + Config + Code + CSS Test)",
            "default_is_invalid",
            "on_is_invalid_change",
            "FieldsetActualConfig",
            "fieldset_check2_documents_docs_sync_and_state_matrix_rules",
            "fieldset_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
            "fieldset_dx_check_script_covers_docs_sync_and_state_matrix_contract",
            "scripts/check-ui-dx.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 docs-sync/state-matrix section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_check2_documents_documentation_as_product_rules() {
    for checklist_source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        for required in [
            "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
            "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
            "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
            "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
        ] {
            assert!(
                checklist_source.contains(required),
                "fieldset check2 should keep documentation-as-product rule `{required}`."
            );
        }
    }
}

#[test]
fn fieldset_documentation_entry_exists_with_beginner_first_progression() {
    let readme = load_source("src/field_form/fieldset/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "# Fieldset",
        "## Hello World",
        "## 常见用法",
        "## 先用起来，再进阶",
        "默认路径：`<Fieldset legend=...>...</Fieldset>`",
        "进阶控制：按需启用 `is_* + default_is_* + on_is_*_change`",
        "is_invalid/default_is_invalid/on_is_invalid_change",
    ] {
        assert!(
            readme.contains(needle),
            "fieldset README should include beginner-friendly marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn fieldset() -> AnyView",
        "title=\"Fieldset\"",
        "slug=\"fieldset\"",
        "title=\"Hello World\"",
        "title=\"Legend + Description\"",
        "title=\"Horizontal + Invalid + Actions\"",
        "title=\"Controlled vs Uncontrolled (Snapshot Contrast)\"",
        "title=\"Fieldset Workbench (Display + Config + Code + CSS Test)\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "fieldset docs entry should include `{needle}`."
        );
    }

    let readme_hello = readme
        .find("## Hello World")
        .expect("fieldset README should include Hello World section");
    let readme_common = readme
        .find("## 常见用法")
        .expect("fieldset README should include common-usage section");
    let readme_progressive = readme
        .find("## 先用起来，再进阶")
        .expect("fieldset README should include beginner-to-advanced section");
    let readme_architecture = readme
        .find("## Architecture Layers")
        .expect("fieldset README should include architecture section");
    assert!(
        readme_hello < readme_common
            && readme_common < readme_progressive
            && readme_progressive < readme_architecture,
        "fieldset README should keep default path before architecture-heavy content."
    );

    let docs_hello = docs_source
        .find("title=\"Hello World\"")
        .expect("fieldset docs should include Hello World playground");
    let docs_common = docs_source
        .find("title=\"Legend + Description\"")
        .expect("fieldset docs should include common-usage playground");
    let docs_controlled = docs_source
        .find("title=\"Controlled vs Uncontrolled (Snapshot Contrast)\"")
        .expect("fieldset docs should include controlled/uncontrolled playground");
    let docs_advanced = docs_source
        .find("title=\"Fieldset Workbench (Display + Config + Code + CSS Test)\"")
        .expect("fieldset docs should include workbench playground");
    assert!(
        docs_hello < docs_common
            && docs_common < docs_controlled
            && docs_controlled < docs_advanced,
        "fieldset docs should keep beginner-first order before advanced controls."
    );
}

#[test]
fn fieldset_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_documentation_as_product_rules",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce documentation-as-product contract `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_documentation_as_product_contract_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
            "fieldset check2 should mark documentation-as-product checklist item complete."
        );

        for needle in [
            "components/fieldset/src/README.md",
            "apps/docs-app/src/pages/components/pages/forms_extra.rs::fieldset",
            "fieldset_check2_documents_documentation_as_product_rules",
            "fieldset_documentation_entry_exists_with_beginner_first_progression",
            "fieldset_dx_check_script_covers_documentation_as_product_contract",
            "scripts/check-ui-dx.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 documentation-as-product section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_check2_documents_interactive_playground_rules() {
    for checklist_source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        for required in [
            "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
            "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
            "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
            "Playground 作为验收面，需可重复复现关键交互路径。",
        ] {
            assert!(
                checklist_source.contains(required),
                "fieldset check2 should keep interactive-playground rule `{required}`."
            );
        }
    }
}

#[test]
fn fieldset_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "pub(super) fn fieldset() -> AnyView",
        "title=\"Fieldset Workbench (Display + Config + Code + CSS Test)\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "controls=move || view!",
        "data-slot=\"fieldset-workbench-controls\"",
        "data-slot=\"fieldset-workbench-compare\"",
        "Switch checked=workbench_required set_checked=set_workbench_required",
        "Switch checked=workbench_disabled set_checked=set_workbench_disabled",
        "Switch checked=workbench_invalid set_checked=set_workbench_invalid",
        "Switch checked=workbench_show_actions set_checked=set_workbench_show_actions",
        "Switch checked=workbench_rtl set_checked=set_workbench_rtl",
        "let (workbench_required, set_workbench_required) = signal(false);",
        "let (workbench_disabled, set_workbench_disabled) = signal(false);",
        "let (workbench_invalid, set_workbench_invalid) = signal(false);",
        "FieldsetActualConfig {",
    ] {
        assert!(
            docs_source.contains(needle),
            "fieldset docs should provide interactive playground marker `{needle}`."
        );
    }

    for needle in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs-app Playground should keep interactive preview contract `{needle}`."
        );
    }
}

#[test]
fn fieldset_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_fieldset_contract.spec.mjs");

    for needle in [
        "docs-app fieldset key flow is repeatable with semantic breakpoints",
        "await page.goto(\"/#/components/fieldset\");",
        "[data-component=\"fieldset\"] [data-slot=\"fieldset\"][data-required=\"true\"]",
        "[data-component=\"fieldset\"] [data-slot=\"fieldset\"][data-invalid=\"true\"]",
        "await page.keyboard.press(\"Enter\");",
        "toHaveAttribute(\"data-invalid-source\", \"is_invalid\")",
        "await page.reload();",
        "toHaveAttribute(\"data-invalid-source\", \"is_invalid\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "fieldset interactive playground should keep repeatable semantic e2e marker `{needle}`."
        );
    }
}

#[test]
fn fieldset_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_interactive_playground_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
            "fieldset check2 should mark interactive-playground checklist item complete."
        );

        for needle in [
            "Fieldset Workbench (Display + Config + Code + CSS Test)",
            "forms_extra.rs::fieldset",
            "docs_app_fieldset_contract.spec.mjs",
            "fieldset_check2_documents_interactive_playground_rules",
            "fieldset_docs_app_provides_interactive_playground_for_props_state_and_preview",
            "fieldset_interactive_playground_reuses_repeatable_semantic_e2e_flow",
            "fieldset_dx_check_script_covers_interactive_playground_contract",
            "scripts/check-ui-dx.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 interactive-playground section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_check2_documents_source_first_copy_paste_ready_rules() {
    for checklist_source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        for required in [
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
            "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
            "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
            "文档代码与当前实现必须同步，防止示例漂移。",
        ] {
            assert!(
                checklist_source.contains(required),
                "fieldset checklist should keep source-first copy-paste-ready rule `{required}`."
            );
        }
    }
}

#[test]
fn fieldset_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let readme_source = load_source("src/field_form/fieldset/README.md");
    let e2e_source = load_source("../../e2e/tests/docs_app_fieldset_contract.spec.mjs");

    for needle in [
        "data-slot=\"fieldset-source-first\"",
        "data-slot=\"fieldset-source-first-contract\"",
        "data-slot=\"fieldset-source-first-dependency-baseline\"",
        "data-slot=\"fieldset-source-prerequisites\"",
        "<code>\"Show code\"</code>",
        "class_name=\"docs-fieldset-source-copy\".to_string()",
        "copyable=true",
        "components/fieldset/src/mod.rs",
        "components/fieldset/src/logic.rs",
        "components/fieldset/src/view.rs",
        "components/fieldset/src/styles.rs",
        "components/fieldset/src/motion.rs",
        "crates/ui/src/field_form/fieldset/{mod,logic,view,styles,motion}.rs",
        "features = [\"component-fieldset\", \"inject-css\"]",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs::fieldset",
    ] {
        assert!(
            docs_source.contains(needle),
            "fieldset source-first docs should contain `{needle}`."
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "code_imports",
        "copyable=true",
        "DEFAULT_PLAYGROUND_IMPORTS",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should contain `{needle}`."
        );
    }

    for needle in [
        "## Source-first",
        "components/fieldset/src/{mod,logic,view,styles,motion}.rs",
        "crates/ui/src/field_form/fieldset/{mod,logic,view,styles,motion}.rs",
        "package feature：`component-fieldset`（可选叠加 `inject-css`）",
        "ui = { default-features = false, features = [\"component-fieldset\", \"inject-css\"] }",
    ] {
        assert!(
            readme_source.contains(needle),
            "fieldset README should document source-first dependency/path marker `{needle}`."
        );
    }

    for needle in [
        "docs-app fieldset source-first section is copy-paste ready and traceable",
        "[data-slot=\"fieldset-source-first\"]",
        "toContainText(\"component-fieldset\")",
        "toContainText(\"inject-css\")",
        "toContainText(\"components/fieldset/src/mod.rs\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "fieldset e2e source-first contract should contain `{needle}`."
        );
    }
}

#[test]
fn fieldset_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: fieldset source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce source-first copy-paste-ready contract `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_source_first_copy_paste_ready_contract_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"),
            "fieldset check2 should mark source-first copy-paste-ready item complete."
        );

        for marker in [
            "apps/docs-app/src/pages/components/pages/forms_extra.rs::fieldset",
            "components/fieldset/src/README.md",
            "fieldset_check2_documents_source_first_copy_paste_ready_rules",
            "fieldset_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
            "fieldset_dx_check_script_covers_source_first_copy_paste_ready_contract",
            "scripts/check-ui-dx.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(marker),
                "fieldset check2 source-first section should reference `{marker}`."
            );
        }
    }
}

#[test]
fn fieldset_check2_documents_heroui_benchmark_docs_sync_rules() {
    for checklist_source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        for required in [
            "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
            "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
            "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
            "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        ] {
            assert!(
                checklist_source.contains(required),
                "fieldset checklist should keep heroui-benchmark docs-sync rule `{required}`."
            );
        }
    }
}

#[test]
fn fieldset_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let readme_source = load_source("src/field_form/fieldset/README.md");

    for needle in [
        "### Fieldset 同步记录（2026-02-20）",
        "参数模型同步：`Fieldset` 参数主轴保持 `orientation/tone/is_required/default_is_required/on_is_required_change/is_disabled/default_is_disabled/on_is_disabled_change/is_invalid/default_is_invalid/on_is_invalid_change`",
        "component_doc!(\"Fieldset\", \"fieldset\", \"Forms\", forms_extra::fieldset)",
        "`apps/docs-app/src/pages/components/pages/forms_extra.rs::fieldset()`",
        "`components/fieldset/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui strategy doc should include fieldset synchronization marker `{needle}`."
        );
    }

    for needle in [
        "component_doc!(",
        "\"Fieldset\"",
        "\"fieldset\"",
        "forms_extra::fieldset",
    ] {
        assert!(
            pages_source.contains(needle),
            "component docs index should expose fieldset entry marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn fieldset() -> AnyView {",
        "title=\"Fieldset\"",
        "slug=\"fieldset\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app fieldset page should stay indexable via marker `{needle}`."
        );
    }

    for needle in ["# Fieldset", "## docs-app 入口"] {
        assert!(
            readme_source.contains(needle),
            "fieldset README should remain an equivalent component doc entry via `{needle}`."
        );
    }
}

#[test]
fn fieldset_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce heroui-benchmark docs-sync contract `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        for marker in [
            "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
            "fieldset_check2_documents_heroui_benchmark_docs_sync_rules",
            "fieldset_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
            "fieldset_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
            "docs/spec/heroui-parameter-design-strategy.md",
            "scripts/check-ui-dx.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(marker),
                "fieldset check2 should keep heroui-benchmark docs-sync evidence marker `{marker}`."
            );
        }
    }
}

#[test]
fn fieldset_api_prefers_explicit_children_over_parallel_array_conventions() {
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for needle in ["pub fn Fieldset(", "children: Children,"] {
        assert!(
            view_source.contains(needle),
            "fieldset API should expose explicit composition marker `{needle}`."
        );
    }

    for forbidden in [
        "labels:",
        "titles:",
        "panels:",
        "items:",
        "item_specs:",
        "ItemSpec",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "fieldset API should not introduce implicit parallel-array/config conventions `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_has_no_dragging_micro_loop_contract() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for forbidden in [
        "Dragging",
        "Action::DragEnd",
        "on:drag",
        "on:pointermove",
        "on:mousemove",
        "touchmove",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "fieldset logic should not carry dragging-loop protocol `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion should not carry dragging-loop protocol `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should not carry dragging-loop protocol `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_has_no_two_pass_geometry_rectification_contract() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for forbidden in [
        "getBoundingClientRect",
        "ResizeObserver",
        "IntersectionObserver",
        "clientWidth",
        "clientHeight",
        "offsetWidth",
        "offsetHeight",
        "Intent",
        "Measure",
        "Rectification",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "fieldset logic should not carry two-pass geometry contract `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion should not carry two-pass geometry contract `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should not carry two-pass geometry contract `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_has_no_registration_protocol_contract() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "fieldset logic should not carry registration protocol `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion should not carry registration protocol `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should not carry registration protocol `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_has_no_slot_projection_lifecycle_contract() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for forbidden in ["Lazy", "KeepAlive", "Eager", "NotifyHidden"] {
        assert!(
            !logic_source.contains(forbidden),
            "fieldset logic should not carry slot projection protocol `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion should not carry slot projection protocol `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should not carry slot projection protocol `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_has_no_env_stream_action_contract() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for forbidden in [
        "BreakpointChanged",
        "ThemeChanged",
        "IntersectionChanged",
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "debounce",
        "throttle",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "fieldset logic should not carry env stream contract `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion should not carry env stream contract `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should not carry env stream contract `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_has_no_event_light_cone_bulk_contract() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for forbidden in [
        "Context Bus",
        "ContextBus",
        "Selector",
        "SelectionState::All",
        "Table",
        "Grid",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "fieldset logic should not carry event light-cone protocol `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion should not carry event light-cone protocol `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should not carry event light-cone protocol `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_has_no_causality_bus_trace_contract() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for forbidden in [
        "TraceId",
        "CausalityBus",
        "CommandBus",
        "broadcast",
        "subscriber",
        "subscribe",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "fieldset logic should not carry causality-bus protocol `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion should not carry causality-bus protocol `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should not carry causality-bus protocol `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_a11y_and_i18n_contracts_are_headless_and_overridable() {
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let headless_source = load_source("../../crates/ui-headless/src/a11y.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/fieldset.rs");

    for needle in [
        "use ui_headless::{A11yDirection, fieldset_attrs, use_controllable_state};",
        "#[prop(optional, into)] legend: Option<String>,",
        "#[prop(optional, into)] description: Option<String>,",
        "#[prop(optional, into)] error_message: Option<String>,",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "lang=move || a11y.get().lang.clone()",
        "dir=move || a11y.get().dir",
        "aria-label=move || a11y.get().aria_label.clone()",
        "aria-disabled=move || a11y.get().aria_disabled",
        "aria-invalid=move || a11y.get().aria_invalid",
        "role=\"alert\"",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset view should preserve a11y/i18n contract marker `{needle}`."
        );
    }

    for forbidden in ["\"Invalid value\"", "\"Fieldset\""] {
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should not hardcode fallback copy `{forbidden}`."
        );
    }

    for needle in [
        "normalize_aria_label",
        "normalize_error_message",
        "pub const DEFAULT_ARIA_LABEL: &str = \"Fieldset\";",
        "pub const DEFAULT_ERROR_MESSAGE: &str = \"Invalid value\";",
        "pub fn fieldset_attrs(",
    ] {
        let present = logic_source.contains(needle)
            || primitives_source.contains(needle)
            || headless_source.contains(needle);
        assert!(
            present,
            "fieldset a11y/i18n stack should expose `{needle}` in primitives/headless/logic."
        );
    }
}

#[test]
fn fieldset_observability_contract_uses_stable_semantic_markers() {
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/fieldset.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_fieldset_contract.spec.mjs");

    for marker in [
        "data-state=move || state.get().data_state_attr",
        "data-message-kind=move || state.get().message_kind_attr",
        "data-required-source=move || view_state.get().required_source_attr",
        "data-required-control-mode=move || view_state.get().required_control_mode_attr",
        "data-required-change-source=move || view_state.get().required_change_source_attr",
        "data-disabled-source=move || view_state.get().disabled_source_attr",
        "data-disabled-control-mode=move || view_state.get().disabled_control_mode_attr",
        "data-disabled-change-source=move || view_state.get().disabled_change_source_attr",
        "data-invalid-source=move || view_state.get().invalid_source_attr",
        "data-invalid-control-mode=move || view_state.get().invalid_control_mode_attr",
        "data-invalid-change-source=move || view_state.get().invalid_change_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "aria-label=move || a11y.get().aria_label.clone()",
        "aria-disabled=move || a11y.get().aria_disabled",
        "aria-invalid=move || a11y.get().aria_invalid",
    ] {
        assert!(
            view_source.contains(marker),
            "fieldset view should expose stable semantic marker `{marker}`."
        );
    }

    for enum_mapping in [
        "FieldsetOrientation::Vertical => \"vertical\"",
        "FieldsetOrientation::Horizontal => \"horizontal\"",
        "FieldsetTone::Default => \"default\"",
        "FieldsetTone::Muted => \"muted\"",
        "FieldsetMessageKind::None => \"none\"",
        "FieldsetMessageKind::Description => \"description\"",
        "FieldsetMessageKind::Error => \"error\"",
        "FieldsetDataState::Default => \"default\"",
        "FieldsetDataState::Required => \"required\"",
        "FieldsetDataState::Disabled => \"disabled\"",
        "FieldsetDataState::Invalid => \"invalid\"",
        "FieldsetDataState::InvalidDisabled => \"invalid-disabled\"",
        "FieldsetDataState::Horizontal => \"horizontal\"",
        "FieldsetDataState::Muted => \"muted\"",
    ] {
        assert!(
            primitives_source.contains(enum_mapping),
            "fieldset primitives should keep closed enum attr mapping `{enum_mapping}`."
        );
    }

    for source_mapping in [
        "let control_mode_attr = if input.value.is_some() {",
        "\"controlled\"",
        "\"uncontrolled\"",
        "let change_source_attr = if input.has_on_change {",
        "\"none\"",
        "let aria_source_attr = if input.has_custom_aria_label {",
        "let error_source_attr = if !input.has_error_message {",
        "let class_source_attr = if input.has_custom_class_name {",
        "\"custom\"",
        "\"default\"",
    ] {
        assert!(
            primitives_source.contains(source_mapping),
            "fieldset primitives should keep stable source marker mapping `{source_mapping}`."
        );
    }

    for selector in [
        "[data-component=\"fieldset\"] [data-slot=\"fieldset\"]",
        "[data-required=\"true\"]",
        "[data-invalid=\"true\"]",
    ] {
        assert!(
            e2e_source.contains(selector),
            "fieldset e2e contract should prefer semantic selector `{selector}`."
        );
    }

    assert!(
        !e2e_source.contains(":nth-child("),
        "fieldset e2e contract should not depend on fragile DOM-order selectors."
    );
}

#[test]
fn fieldset_styles_depend_on_semantic_markers_not_fragile_structure() {
    let styles_source = load_source("src/field_form/fieldset/styles.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for selector in [
        ".ui-fieldset[data-orientation=\"horizontal\"]",
        ".ui-fieldset[data-tone=\"muted\"]",
        ".ui-fieldset[data-required=\"true\"] .ui-fieldset__legend",
        ".ui-fieldset[data-disabled=\"true\"]",
        ".ui-fieldset[data-invalid=\"true\"] .ui-fieldset__group",
        ".ui-fieldset[data-class-source=\"custom\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "fieldset styles should branch via semantic selector `{selector}`."
        );
    }

    for forbidden in [":nth-child(", ":first-child", ":last-child", "> *", "* + *"] {
        assert!(
            !styles_source.contains(forbidden),
            "fieldset styles should not rely on fragile structural selector `{forbidden}`."
        );
    }

    for marker in [
        "style=move || motion_style.get_value()",
        "pub fn attach_motion(motion: FieldsetMotion) -> String {",
        "--ui-fieldset-motion-duration",
        "--ui-fieldset-motion-distance",
        "--ui-fieldset-motion-stiffness",
        "--ui-fieldset-motion-damping",
    ] {
        let present = view_source.contains(marker) || motion_source.contains(marker);
        assert!(
            present,
            "fieldset runtime style path should stay in motion custom properties `{marker}`."
        );
    }

    for forbidden in [
        "background:",
        "color:",
        "display:",
        "grid-template-columns:",
        "padding:",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion inline style generator should not carry business style rule `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_token_first_static_style_contract_is_enforced() {
    let styles_source = load_source("src/field_form/fieldset/styles.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-danger, var(--ui-fallback-danger))",
    ] {
        assert!(
            styles_source.contains(needle),
            "fieldset styles should stay token-first via `{needle}`."
        );
    }

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-fieldset\")]",
        "out.push_str(crate::field_form::fieldset::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui css aggregation should include `{needle}` for fieldset."
        );
    }

    assert!(
        view_source.contains("style=move || motion_style.get_value()"),
        "fieldset view should only inject runtime style through motion custom properties."
    );

    for needle in [
        "pub fn attach_motion(motion: FieldsetMotion) -> String {",
        "--ui-fieldset-motion-duration: {:.3}ms;",
        "--ui-fieldset-motion-distance: {:.3}px;",
        "--ui-fieldset-motion-stiffness: {:.3};",
        "--ui-fieldset-motion-damping: {:.3};",
    ] {
        assert!(
            motion_source.contains(needle),
            "fieldset motion should expose only custom-property motion contract `{needle}`."
        );
    }

    for forbidden in [
        "tailwind",
        "class_variance_authority",
        "cva(",
        "styled_components",
        "emotion::",
        "stylex",
        "styled!(",
    ] {
        let leaked = styles_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden);
        assert!(
            !leaked,
            "fieldset component layer should not leak utility-first/CSS-in-Rust marker `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_visual_desire_reuses_theme_visual_baseline_and_form_quality_contracts() {
    let fieldset_styles_source = load_source("src/field_form/fieldset/styles.rs");
    let fieldset_docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        ".ui-fieldset__legend",
        ".ui-fieldset__description",
        ".ui-fieldset__error",
        ".ui-fieldset--invalid .ui-fieldset__group",
        ".ui-fieldset[data-disabled=\"true\"]",
    ] {
        assert!(
            fieldset_styles_source.contains(needle),
            "fieldset default styles should keep hierarchy/feedback marker `{needle}`."
        );
    }

    for needle in [
        "title=\"Fieldset\"",
        "slug=\"fieldset\"",
        "title=\"Legend + Description\"",
        "title=\"Horizontal + Invalid + Actions\"",
        "variant=ui::ButtonVariant::Secondary",
    ] {
        assert!(
            fieldset_docs_source.contains(needle),
            "fieldset docs page should keep visual-quality token `{needle}`."
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            baseline_page_source.contains(needle),
            "theme visual baseline docs page should include `{needle}`."
        );
    }

    for needle in ["\"ThemeVisualBaseline\"", "\"theme-visual-baseline\""] {
        assert!(
            pages_source.contains(needle),
            "docs pages registry should keep visual baseline route token `{needle}`."
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "theme visual baseline renders button/input/overlay",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "theme visual baseline e2e gate should include `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_visual_desire_gate_complete() {
    let source = load_source("../../components/fieldset/check2.md");
    assert!(
        source.contains("- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。"),
        "fieldset check2 should mark visual desire gate complete."
    );
    assert!(
        source.contains(
            "fieldset_visual_desire_reuses_theme_visual_baseline_and_form_quality_contracts"
        ),
        "fieldset check2 should reference executable visual-desire regression evidence."
    );
}

#[test]
fn fieldset_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "component-fieldset = [\"dep:ui-fieldset\"]",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui tree-shaking feature map should include `{needle}`."
        );
    }

    assert!(
        lib_source.contains("feature = \"component-fieldset\"")
            && lib_source.contains("pub mod field_form {"),
        "lib.rs should keep field_form module behind component feature gates including component-fieldset."
    );
    assert!(
        lib_source.contains("#[cfg(feature = \"component-fieldset\")]")
            && lib_source.contains("pub use ui_fieldset as fieldset;"),
        "inline field_form module in lib.rs should feature-gate fieldset export for tree-shaking."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-fieldset\")]")
            && css_source.contains("out.push_str(crate::field_form::fieldset::styles::CSS);"),
        "css.rs should gate fieldset CSS aggregation behind component-fieldset feature."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)"),
        "css.rs should keep inject-css top-level gate for component CSS injection."
    );

    for forbidden in ["component_registry", "ALL_COMPONENTS_MAP", "lazy_static!"] {
        assert!(
            !lib_source.contains(forbidden) && !css_source.contains(forbidden),
            "tree-shaking contract should avoid global keep-alive registries `{forbidden}`."
        );
    }

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("web-demo-components")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components for full docs coverage."
    );
}

#[test]
fn fieldset_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features",
        "cargo tree -e features -i ui -p web-demo",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "size regression",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking gate script should include `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_tree_shaking_contract_complete() {
    let source = load_source("../../components/fieldset/check2.md");

    assert!(
        source.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "fieldset check2 should mark tree-shaking item complete."
    );

    for needle in [
        "fieldset_tree_shaking_keeps_component_feature_and_css_boundaries",
        "fieldset_tree_shaking_check_script_covers_feature_tree_wasm_and_budget",
        "cargo tree -e features -i ui -p ui --no-default-features --features component-fieldset,inject-css",
        "cargo tree -e features -i ui -p web-demo",
        "bash ./scripts/check-ui-tree-shaking.sh",
    ] {
        assert!(
            source.contains(needle),
            "fieldset check2 tree-shaking section should reference `{needle}`."
        );
    }
}

#[test]
fn fieldset_tree_shaking_script_enforces_component_minimal_feature_tree() {
    let script_source = load_source("../../scripts/check-ui-tree-shaking.sh");

    for needle in [
        "FIELDSET_MIN_FEATURES=\"component-fieldset,inject-css\"",
        "echo \"[tree-shaking] fieldset feature registration + gated aggregation contract\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_tree_shaking_keeps_component_feature_and_css_boundaries",
        "echo \"[tree-shaking] fieldset minimal feature tree\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$FIELDSET_MIN_FEATURES\"",
        "if ! grep -q 'feature \"component-fieldset\" (command-line)' <<<\"$FIELDSET_TREE_OUTPUT\";",
        "if ! grep -q 'feature \"inject-css\" (command-line)' <<<\"$FIELDSET_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$FIELDSET_TREE_OUTPUT\";",
        "echo \"[tree-shaking] fieldset minimal wasm check\"",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$FIELDSET_MIN_FEATURES\"",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking script should enforce fieldset minimal feature contract `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
            "fieldset check2 should mark tree-shaking feature-pruning checklist item complete."
        );

        for needle in [
            "component-fieldset = [\"dep:ui-fieldset\"]",
            "#[cfg(feature = \"component-fieldset\")]",
            "out.push_str(crate::field_form::fieldset::styles::CSS);",
            "fieldset_tree_shaking_keeps_component_feature_and_css_boundaries",
            "fieldset_tree_shaking_script_enforces_component_minimal_feature_tree",
            "scripts/check-ui-tree-shaking.sh",
            "FIELDSET_MIN_FEATURES=\"component-fieldset,inject-css\"",
            "cargo tree -e features -i ui -p ui --no-default-features --features \"$FIELDSET_MIN_FEATURES\"",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 tree-shaking feature-pruning section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_type_system_and_semantic_markers_keep_machine_readable_contract() {
    let primitives_source = load_source("../../crates/ui-state-primitives/src/fieldset.rs");
    let primitives_test_source =
        load_source("../../crates/ui-state-primitives/src/test/fieldset.rs");
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_fieldset_contract.spec.mjs");

    for needle in [
        "pub enum FieldsetOrientation",
        "pub enum FieldsetTone",
        "pub enum FieldsetMessageKind",
        "pub enum FieldsetDataState",
        "pub struct FieldsetStateInput",
        "pub orientation: FieldsetOrientation,",
        "pub tone: FieldsetTone,",
        "pub fn resolve_state(input: FieldsetStateInput) -> FieldsetState",
        "FieldsetDataState::InvalidDisabled",
        "FieldsetDataState::Invalid",
        "FieldsetDataState::Disabled",
        "FieldsetDataState::Required",
        "FieldsetDataState::Horizontal",
        "FieldsetDataState::Muted",
        "FieldsetDataState::Default",
    ] {
        assert!(
            primitives_source.contains(needle),
            "fieldset primitives should keep type-constrained state modeling `{needle}`."
        );
    }

    for needle in [
        "pub struct FieldsetViewStateInput",
        "pub orientation: FieldsetOrientation,",
        "pub tone: FieldsetTone,",
        "pub fn resolve_view_state(input: FieldsetViewStateInput) -> FieldsetViewState",
    ] {
        assert!(
            logic_source.contains(needle),
            "fieldset logic should keep typed normalization boundary `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] orientation: FieldsetOrientation,",
        "#[prop(optional)] tone: FieldsetTone,",
        "data-state=move || state.get().data_state_attr",
        "data-message-kind=move || state.get().message_kind_attr",
        "data-ui-state=move || agent_contract.get().state_attr",
        "data-required-source=move || view_state.get().required_source_attr",
        "data-disabled-source=move || view_state.get().disabled_source_attr",
        "data-invalid-source=move || view_state.get().invalid_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset view should expose machine-readable semantic marker `{needle}`."
        );
    }

    for forbidden in [
        "orientation: String",
        "tone: String",
        "variant: String",
        "status: String",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "fieldset should avoid string protocol drift for key discrete axes `{forbidden}`."
        );
    }

    for needle in [
        "resolve_state_tracks_sources_and_priorities",
        "normalize_boolean_axis_prefers_controlled_value_source",
        "normalize_boolean_axis_uses_default_source_for_uncontrolled_axis",
        "normalize_boolean_axis_falls_back_to_builtin_default_false",
    ] {
        assert!(
            primitives_test_source.contains(needle),
            "fieldset primitive tests should keep normalization regression case `{needle}`."
        );
    }

    for selector in [
        "[data-component=\"fieldset\"] [data-slot=\"fieldset\"]",
        "[data-required=\"true\"]",
        "[data-invalid=\"true\"]",
    ] {
        assert!(
            e2e_source.contains(selector),
            "fieldset e2e contract should consume machine-readable semantic selector `{selector}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_type_system_semantic_contract_complete() {
    let source = load_source("../../components/fieldset/check2.md");

    assert!(
        source.contains("- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。"),
        "fieldset check2 should mark type-system + semantic-marker item complete."
    );

    for needle in [
        "fieldset_type_system_and_semantic_markers_keep_machine_readable_contract",
        "FieldsetOrientation",
        "FieldsetTone",
        "FieldsetMessageKind",
        "FieldsetDataState",
        "data-state",
        "data-message-kind",
        "data-ui-state",
    ] {
        assert!(
            source.contains(needle),
            "fieldset check2 type-system section should reference `{needle}`."
        );
    }
}

#[test]
fn fieldset_has_no_focus_stack_overlay_restore_contract() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");

    for forbidden in [
        "NodeRef",
        "FallbackTo",
        "Selector",
        "FocusManager",
        "focus_stack",
        "document.body",
        "active_element",
        "restore_focus",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "fieldset logic should not carry overlay focus-stack contract `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should not carry overlay focus-stack contract `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion should not carry overlay focus-stack contract `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_focus_stack_item_complete_as_na() {
    let source = load_source("../../components/fieldset/check2.md");

    assert!(
        source.contains("- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。"),
        "fieldset check2 should mark focus stack item complete."
    );
    assert!(
        source.contains("N/A（Fieldset 非 Overlay）"),
        "fieldset check2 should state explicit N/A reason for focus-stack rule."
    );
    assert!(
        source.contains("fieldset_has_no_focus_stack_overlay_restore_contract"),
        "fieldset check2 should reference executable focus-stack regression evidence."
    );
}

#[test]
fn fieldset_has_no_foreign_zone_escape_hatch_contract() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");

    for forbidden in [
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "OpenLayers",
        "third-party instance",
        "wasm_bindgen::JsValue",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "fieldset logic should not carry foreign-zone escape-hatch contract `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should not carry foreign-zone escape-hatch contract `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion should not carry foreign-zone escape-hatch contract `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_escape_hatch_item_complete_as_na() {
    let source = load_source("../../components/fieldset/check2.md");

    assert!(
        source.contains("- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。"),
        "fieldset check2 should mark escape-hatch item complete."
    );
    assert!(
        source.contains("N/A（Fieldset 非命令式第三方承载组件）"),
        "fieldset check2 should state explicit N/A reason for escape-hatch rule."
    );
    assert!(
        source.contains("fieldset_has_no_foreign_zone_escape_hatch_contract"),
        "fieldset check2 should reference executable escape-hatch regression evidence."
    );
}

#[test]
fn fieldset_semantics_matrix_prefers_contract_assertions_over_visual_snapshots() {
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_fieldset_contract.spec.mjs");

    for marker in [
        "data-state=move || state.get().data_state_attr",
        "data-required-source=move || view_state.get().required_source_attr",
        "data-required-control-mode=move || view_state.get().required_control_mode_attr",
        "data-disabled-source=move || view_state.get().disabled_source_attr",
        "data-disabled-control-mode=move || view_state.get().disabled_control_mode_attr",
        "data-invalid-source=move || view_state.get().invalid_source_attr",
        "data-invalid-control-mode=move || view_state.get().invalid_control_mode_attr",
        "aria-disabled=move || a11y.get().aria_disabled",
        "aria-invalid=move || a11y.get().aria_invalid",
        "role=\"alert\"",
    ] {
        assert!(
            view_source.contains(marker),
            "fieldset semantics matrix should expose contract marker `{marker}`."
        );
    }

    for matrix_toggle in [
        "Switch checked=workbench_required set_checked=set_workbench_required",
        "Switch checked=workbench_disabled set_checked=set_workbench_disabled",
        "Switch checked=workbench_invalid set_checked=set_workbench_invalid",
    ] {
        assert!(
            docs_source.contains(matrix_toggle),
            "fieldset docs workbench should include matrix toggle `{matrix_toggle}`."
        );
    }

    for semantic_assertion in [
        "toHaveAttribute(\"data-required-source\"",
        "toHaveAttribute(\"data-invalid-source\"",
        "[data-component=\"fieldset\"] [data-slot=\"fieldset\"]",
    ] {
        assert!(
            e2e_source.contains(semantic_assertion),
            "fieldset e2e contract should include semantic assertion `{semantic_assertion}`."
        );
    }

    for forbidden_snapshot in ["toHaveScreenshot", "toMatchSnapshot", "screenshot("] {
        assert!(
            !e2e_source.contains(forbidden_snapshot),
            "fieldset e2e contract should not depend on visual snapshot assertion `{forbidden_snapshot}`."
        );
    }

    for forbidden_handler in [
        "on:keydown",
        "on:keyup",
        "on:pointerdown",
        "on:pointerup",
        "on:pointermove",
    ] {
        assert!(
            !view_source.contains(forbidden_handler),
            "fieldset view should not claim keyboard/pointer control path `{forbidden_handler}`."
        );
    }
}

#[test]
fn fieldset_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let local_semantics_source = load_source("../../components/fieldset/test/semantics.rs");
    let semantics_source = load_source("tests/fieldset/semantics.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_fieldset_contract.spec.mjs");
    let perf_script_source = load_source("../../scripts/check-ui-performance.sh");

    for needle in [
        "aria-label=move || a11y.get().aria_label.clone()",
        "aria-disabled=move || a11y.get().aria_disabled",
        "aria-invalid=move || a11y.get().aria_invalid",
        "data-state=move || state.get().data_state_attr",
        "data-required-source=move || view_state.get().required_source_attr",
        "data-disabled-source=move || view_state.get().disabled_source_attr",
        "data-invalid-source=move || view_state.get().invalid_source_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
        "role=ROLE_ALERT",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset semantic-priority contract should keep marker `{needle}`."
        );
    }

    for needle in [
        "fn fieldset_component_depends_on_layered_kernel_shell_crates()",
        "fn fieldset_public_api_surface_is_stable_and_not_dom_leaky()",
        "fn fieldset_view_assembles_logic_headless_and_motion_without_reimplementing_kernels()",
        "fn fieldset_component_has_local_semantics_test_file()",
    ] {
        assert!(
            local_semantics_source.contains(needle),
            "fieldset local semantics suite should keep marker `{needle}`."
        );
    }

    for needle in [
        "fn fieldset_semantics_matrix_prefers_contract_assertions_over_visual_snapshots()",
        "toHaveAttribute(\"data-required-source\"",
        "toHaveAttribute(\"data-invalid-source\"",
        "[data-component=\"fieldset\"] [data-slot=\"fieldset\"]",
    ] {
        assert!(
            semantics_source.contains(needle) || e2e_source.contains(needle),
            "fieldset semantic-priority path should keep marker `{needle}`."
        );
    }

    for forbidden in ["assert_snapshot!", "insta::assert"] {
        assert!(
            !local_semantics_source.contains(forbidden) && !semantics_source.contains(forbidden),
            "fieldset semantic-priority path should avoid snapshot-only assertion `{forbidden}`."
        );
    }

    for forbidden_snapshot in ["toHaveScreenshot(", "toMatchSnapshot(", "screenshot("] {
        assert!(
            !e2e_source.contains(forbidden_snapshot),
            "fieldset e2e should avoid snapshot-only assertion `{forbidden_snapshot}`."
        );
    }

    let script_needle = "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance script should include semantic-priority gate `{script_needle}`."
    );
}

#[test]
fn fieldset_performance_script_covers_semantic_test_priority_contract() {
    let script_source = load_source("../../scripts/check-ui-performance.sh");

    for needle in [
        "echo \"[perf] contract: fieldset semantic test priority\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "performance script should include fieldset semantic-priority marker `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_semantic_test_priority_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains(
                "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。"
            ),
            "fieldset check2 should mark semantic-test-priority item complete."
        );

        for needle in [
            "components/fieldset/test/semantics.rs",
            "fieldset_semantics_matrix_prefers_contract_assertions_over_visual_snapshots",
            "fieldset_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
            "fieldset_performance_script_covers_semantic_test_priority_contract",
            "scripts/check-ui-performance.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 semantic-test-priority section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_file_role_boundaries_stay_separated() {
    let mod_source = load_source("src/field_form/fieldset/mod.rs");
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let styles_source = load_source("src/field_form/fieldset/styles.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");

    for needle in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Fieldset;",
    ] {
        assert!(
            mod_source.contains(needle),
            "fieldset mod.rs should keep stable export boundary `{needle}`."
        );
    }

    for forbidden in [
        "#[component]",
        "view! {",
        "data-slot=",
        "on:",
        "--ui-",
        "web_sys",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "fieldset logic should not carry view/css/dom concern `{forbidden}`."
        );
    }

    for forbidden in [
        "#[component]",
        "Signal<",
        "use_controllable_state",
        "resolve_state(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "fieldset styles should stay static and not carry runtime/state concern `{forbidden}`."
        );
    }

    for required in [
        "logic::resolve_view_state(logic::FieldsetViewStateInput {",
        "use ui_headless::{A11yDirection, fieldset_attrs, use_controllable_state};",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(required),
            "fieldset view should keep explicit rendering/semantic mount contract `{required}`."
        );
    }

    for forbidden in [
        "Theme::new(",
        "default_text_field_motion_tokens()",
        "clamp(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should not carry motion/theme engine concern `{forbidden}`."
        );
    }

    for required in [
        "pub fn sanitize_motion(motion: FieldsetMotion) -> FieldsetMotion {",
        "pub fn attach_motion(motion: FieldsetMotion) -> String {",
        "--ui-fieldset-motion-duration",
        "--ui-fieldset-motion-distance",
        "--ui-fieldset-motion-stiffness",
        "--ui-fieldset-motion-damping",
    ] {
        assert!(
            motion_source.contains(required),
            "fieldset motion should keep attach/sanitize contract `{required}`."
        );
    }

    for forbidden in ["#[component]", "view! {", "data-slot=", "fieldset_attrs("] {
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion should not carry view/headless concern `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_does_not_introduce_spec_module_for_simple_component_scope() {
    assert!(
        !path_exists("src/field_form/fieldset/spec.rs"),
        "fieldset should not introduce `spec.rs` for simple component scope."
    );

    let mod_source = load_source("src/field_form/fieldset/mod.rs");
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for forbidden in ["mod spec", "pub mod spec", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "fieldset mod.rs should not surface spec module marker `{forbidden}`."
        );
    }

    for forbidden in ["Spec::new(", "FieldsetSpec", "render()"] {
        let leaked = mod_source.contains(forbidden)
            || logic_source.contains(forbidden)
            || view_source.contains(forbidden);
        assert!(
            !leaked,
            "fieldset simple component scope should not leak spec-builder pattern `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_logic_resolves_agent_contract_and_locale_helpers() {
    let source = load_source("src/field_form/fieldset/logic.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for needle in [
        "pub struct FieldsetBooleanAxisInput",
        "pub struct FieldsetBooleanAxis",
        "pub fn normalize_boolean_axis(input: FieldsetBooleanAxisInput) -> FieldsetBooleanAxis",
        "pub struct FieldsetViewStateInput",
        "pub struct FieldsetViewState",
        "pub fn resolve_view_state(input: FieldsetViewStateInput) -> FieldsetViewState",
        "pub struct FieldsetAgentContract",
        "pub fn resolve_agent_contract(state: FieldsetState) -> FieldsetAgentContract",
        "schema_attr: \"ui.fieldset.agent-contract\"",
        "stream_support_attr: \"unsupported\"",
        "stream_fallback_attr: \"snapshot\"",
        "stream_mode_attr: \"snapshot\"",
        "output_status_attr: \"verified\"",
    ] {
        assert!(
            source.contains(needle),
            "fieldset logic should expose `{needle}` machine-readable contract marker."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, fieldset_attrs, use_controllable_state};",
        "let a11y = Memo::new(move |_| {",
        "fieldset_attrs(",
        "aria-disabled=move || a11y.get().aria_disabled",
        "aria-invalid=move || a11y.get().aria_invalid",
        "let agent_contract = Memo::new(move |_| logic::resolve_agent_contract(state.get()));",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset view should compose locale + agent contract via `{needle}`."
        );
    }
}

#[test]
fn fieldset_e2e_contract_file_exists_and_uses_semantic_selectors() {
    let rel = "../../e2e/tests/docs_app_fieldset_contract.spec.mjs";
    assert!(
        path_exists(rel),
        "fieldset should provide docs-app e2e contract file: `{rel}`."
    );

    let source = load_source(rel);
    for needle in [
        "body:not(:has(#boot))",
        "data-component=\"fieldset\"",
        "data-slot=\"fieldset\"",
        "data-ui-schema",
        "data-ui-stream-mode",
        "data-ui-output-status",
        "data-required-source",
        "Show code|Hide code",
    ] {
        assert!(
            source.contains(needle),
            "fieldset e2e contract should include `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_documents_e2e_selector_and_stable_wait_rules() {
    for checklist_source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        for required in [
            "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
            "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
            "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
            "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        ] {
            assert!(
                checklist_source.contains(required),
                "fieldset check2 should keep e2e selector/stable-wait rule `{required}`."
            );
        }
    }
}

#[test]
fn fieldset_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_fieldset_contract.spec.mjs");

    for needle in [
        "/#/components/fieldset",
        "body:not(:has(#boot))",
        "[data-component=\"fieldset\"] [data-slot=\"fieldset\"]",
        "[data-slot=\"ui-perf-probe\"]",
        "[data-required=\"true\"]",
        "[data-invalid=\"true\"]",
        "toHaveAttribute(\"data-ui-schema\", \"ui.fieldset.agent-contract\")",
        "toHaveAttribute(\"data-required-source\", \"required\")",
        "toHaveAttribute(\"data-invalid-source\", \"is_invalid\")",
        "toHaveAttribute(\"data-perf-observability\", \"mount-plus-budget\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "fieldset e2e selector/stable-wait contract should include `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        ":nth-child(",
        ":nth-of-type(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "fieldset e2e selector contract should avoid flaky selector/wait token `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_e2e_check_script_covers_selector_contract() {
    let script_source = load_source("../../components/fieldset/scripts/check-ui-e2e-fieldset.sh");

    for needle in [
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
    ] {
        assert!(
            script_source.contains(needle),
            "fieldset e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_e2e_selector_stability_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
            "fieldset check2 should mark e2e-selector-stability checklist item complete."
        );

        for needle in [
            "docs_app_fieldset_contract.spec.mjs",
            "body:not(:has(#boot))",
            "fieldset_check2_documents_e2e_selector_and_stable_wait_rules",
            "fieldset_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
            "components/fieldset/scripts/check-ui-e2e-fieldset.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 e2e-selector-stability section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_check2_documents_e2e_repeatable_key_flow_rules() {
    for checklist_source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        for required in [
            "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
            "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
            "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
            "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
        ] {
            assert!(
                checklist_source.contains(required),
                "fieldset check2 should keep repeatable-key-flow rule `{required}`."
            );
        }
    }
}

#[test]
fn fieldset_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_fieldset_contract.spec.mjs");

    for needle in [
        "docs-app fieldset key flow is repeatable with semantic breakpoints",
        "[data-component=\"fieldset\"] [data-slot=\"fieldset\"][data-required=\"true\"]",
        "[data-component=\"fieldset\"] [data-slot=\"fieldset\"][data-invalid=\"true\"]",
        "toHaveAttribute(\"data-required-source\", \"required\")",
        "toHaveAttribute(\"data-invalid-source\", \"is_invalid\")",
        "toHaveAttribute(\"data-error-source\", \"custom\")",
        "const actionButton = invalidFieldset.locator('[data-slot=\"button\"]').first();",
        "await actionButton.focus();",
        "await expect(actionButton).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await actionButton.click();",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "fieldset e2e repeatable-flow contract should include `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "fieldset e2e key flow should avoid non-semantic/flaky token `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_e2e_check_script_covers_selector_and_key_flow_contracts() {
    let script_source = load_source("../../components/fieldset/scripts/check-ui-e2e-fieldset.sh");

    for needle in [
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
    ] {
        assert!(
            script_source.contains(needle),
            "fieldset e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_e2e_repeatable_key_flow_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
            "fieldset check2 should mark repeatable-key-flow checklist item complete."
        );

        for needle in [
            "docs_app_fieldset_contract.spec.mjs",
            "docs-app fieldset key flow is repeatable with semantic breakpoints",
            "fieldset_check2_documents_e2e_repeatable_key_flow_rules",
            "fieldset_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
            "components/fieldset/scripts/check-ui-e2e-fieldset.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 repeatable-key-flow section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_hydration_discontinuity_contract_is_explicitly_na_without_time_or_random_id_init() {
    let check_source = load_source("src/field_form/fieldset/check2.md");
    assert!(
        check_source.contains("N/A（Fieldset 当前无本地 ID 生成需求）"),
        "fieldset check2 should state explicit N/A reason for hydration discontinuity rule."
    );

    for rel in [
        "src/field_form/fieldset/mod.rs",
        "src/field_form/fieldset/logic.rs",
        "src/field_form/fieldset/view.rs",
        "src/field_form/fieldset/motion.rs",
    ] {
        let source = load_source(rel);
        for forbidden in [
            "SystemTime::now",
            "Instant::now",
            "js_sys::Date::now",
            "Date::now",
            "now(",
            "Uuid::new_v4",
            "uuid::",
            "rand::",
            "thread_rng",
            "random::<",
            "random_uuid",
            "nanoid",
            "use_id(",
        ] {
            assert!(
                !source.contains(forbidden),
                "fieldset source `{rel}` should not use non-deterministic hydration initializer `{forbidden}`."
            );
        }
    }

    let root_source = load_source("../../crates/ui/src/root.rs");
    for required in [
        "#[prop(optional, default = 1)] id_seed: u64",
        "provide_ui_id_provider(id_seed);",
    ] {
        assert!(
            root_source.contains(required),
            "ui root should keep deterministic id-provider seed wiring `{required}`."
        );
    }

    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    assert!(
        headless_lib_source.contains(
            "pub use id_provider::{UiIdProvider, provide_ui_id_provider, use_ui_id_provider};"
        ),
        "ui-headless should expose deterministic id-provider bridge for hydration-safe ids."
    );

    let headless_id_provider_source = load_source("../../crates/ui-headless/src/id_provider.rs");
    for required in [
        "pub struct UiIdProvider",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider>",
    ] {
        assert!(
            headless_id_provider_source.contains(required),
            "ui-headless id-provider module should keep deterministic API `{required}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_hydration_discontinuity_item_complete_as_na() {
    let source = load_source("src/field_form/fieldset/check2.md");
    assert!(
        source.contains("- [x] SSR 时空断裂治理（Hydration Discontinuity）"),
        "fieldset check2 must mark hydration discontinuity item as complete."
    );
    assert!(
        source.contains(
            "fieldset_hydration_discontinuity_contract_is_explicitly_na_without_time_or_random_id_init"
        ),
        "fieldset check2 should link hydration discontinuity item to executable regression evidence."
    );
}

#[test]
fn fieldset_ssr_cross_platform_contract_uses_explicit_cfg_and_keeps_non_wasm_clean() {
    let ui_components_manifest = load_source("../../crates/ui/Cargo.toml");
    assert!(
        ui_components_manifest.contains("component-fieldset = [\"dep:ui-fieldset\"]"),
        "ui feature graph should keep explicit component-fieldset gating."
    );

    let ui_motion_source = load_source("../../crates/ui-motion/src/lib.rs");
    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web;",
        "pub fn prefers_reduced_motion() -> bool",
    ] {
        assert!(
            ui_motion_source.contains(required),
            "ui-motion should keep explicit cross-platform cfg branch `{required}`."
        );
    }

    for rel in [
        "src/field_form/fieldset/mod.rs",
        "src/field_form/fieldset/logic.rs",
        "src/field_form/fieldset/styles.rs",
        "src/field_form/fieldset/view.rs",
        "src/field_form/fieldset/motion.rs",
        "src/field_form/fieldset/protocol.rs",
    ] {
        let source = load_source(rel);
        for forbidden in [
            "web_sys",
            "web-sys",
            "js_sys",
            "wasm_bindgen",
            "window(",
            "document(",
        ] {
            assert!(
                !source.contains(forbidden),
                "fieldset non-wasm source `{rel}` should not reference browser object `{forbidden}`."
            );
        }
    }
}

#[test]
fn fieldset_platform_check_script_covers_native_ssr_wasm_paths_and_source_guards() {
    let source = load_source("../../scripts/check-ui-platforms.sh");

    for required in [
        "echo \"[platform] compile-only: fieldset native path\"",
        "cargo check -p ui --no-default-features --features component-fieldset,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "echo \"[platform] compile-only: fieldset wasm path\"",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-fieldset,inject-css",
        "echo \"[platform] source guard: non-wasm fieldset files must not reference web_sys\"",
        "components/fieldset/src/view.rs",
        "components/fieldset/src/motion.rs",
        "web_sys|web-sys|js_sys|wasm_bindgen|window\\\\(|document\\\\(",
    ] {
        assert!(
            source.contains(required),
            "platform check script should include fieldset cross-platform guard `{required}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_ssr_cross_platform_item_complete() {
    let source = load_source("src/field_form/fieldset/check2.md");

    assert!(
        source
            .contains("- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。"),
        "fieldset check2 should mark SSR/cross-platform item complete."
    );

    for required in [
        "cargo check -p ui --no-default-features --features component-fieldset,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-fieldset,inject-css",
        "fieldset_ssr_cross_platform_contract_uses_explicit_cfg_and_keeps_non_wasm_clean",
        "fieldset_platform_check_script_covers_native_ssr_wasm_paths_and_source_guards",
    ] {
        assert!(
            source.contains(required),
            "fieldset check2 SSR/cross-platform section should reference `{required}`."
        );
    }
}

#[test]
fn fieldset_ui_headless_web_ssr_feature_mutex_contract_is_enforced() {
    let headless_lib = load_source("../../crates/ui-headless/src/lib.rs");
    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            headless_lib.contains(required),
            "ui-headless should keep web/ssr mutex compile_error contract `{required}`."
        );
    }

    let fieldset_manifest = load_source("src/field_form/fieldset/Cargo.toml");
    assert!(
        fieldset_manifest.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "fieldset should consume headless contract via dependency boundary."
    );

    let fieldset_view = load_source("src/field_form/fieldset/view.rs");
    assert!(
        fieldset_view
            .contains("use ui_headless::{A11yDirection, fieldset_attrs, use_controllable_state};"),
        "fieldset view should consume ui-headless contract without bypassing feature gates."
    );
}

#[test]
fn fieldset_platform_script_covers_ui_headless_web_ssr_mutex_checks() {
    let source = load_source("../../scripts/check-ui-platforms.sh");
    for required in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "echo \"[platform] compile guard: ui-headless web+ssr must fail\"",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "rg -n \"mutually exclusive\" \"$MUTEX_LOG\"",
    ] {
        assert!(
            source.contains(required),
            "platform script should keep ui-headless web/ssr mutex verification `{required}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_ui_headless_mutex_item_complete() {
    let source = load_source("src/field_form/fieldset/check2.md");
    assert!(
        source.contains("- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。"),
        "fieldset check2 should mark ui-headless web/ssr mutex item complete."
    );
    for required in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "fieldset_ui_headless_web_ssr_feature_mutex_contract_is_enforced",
        "fieldset_platform_script_covers_ui_headless_web_ssr_mutex_checks",
    ] {
        assert!(
            source.contains(required),
            "fieldset check2 ui-headless mutex section should reference `{required}`."
        );
    }
}

#[test]
fn fieldset_ui_motion_non_wasm_noop_stub_contract_is_enforced() {
    let motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop() {",
    ] {
        assert!(
            motion_lib.contains(required),
            "ui-motion should keep non-wasm no-op stub contract `{required}`."
        );
    }
}

#[test]
fn fieldset_motion_module_degrades_safely_without_animation_handle_assumption() {
    let source = load_source("src/field_form/fieldset/motion.rs");
    for required in [
        "pub fn resolve_effective_motion(",
        "pub fn attach_motion(motion: FieldsetMotion) -> String {",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-fieldset-motion-duration",
        "--ui-fieldset-motion-distance",
        "--ui-fieldset-motion-stiffness",
        "--ui-fieldset-motion-damping",
    ] {
        assert!(
            source.contains(required),
            "fieldset motion module should keep safe reduced-motion/no-handle contract `{required}`."
        );
    }

    for forbidden in [
        "ui_motion::web::animate(",
        "web_sys::Animation",
        "wasm_bindgen",
        "JsValue",
        "unwrap(",
        "expect(",
        "panic!(",
    ] {
        assert!(
            !source.contains(forbidden),
            "fieldset motion module should not assume runtime animation handle or panic path `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_platform_script_covers_ui_motion_non_wasm_stub_checks() {
    let source = load_source("../../scripts/check-ui-platforms.sh");
    for required in [
        "echo \"[platform] compile-only: ui-motion native path\"",
        "cargo check -p ui-motion",
        "echo \"[platform] compile-only: ui-motion wasm path\"",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "echo \"[platform] ui-motion non-wasm stub tests\"",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            source.contains(required),
            "platform script should keep ui-motion non-wasm stub verification `{required}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_ui_motion_non_wasm_stub_item_complete() {
    let source = load_source("src/field_form/fieldset/check2.md");
    assert!(
        source.contains(
            "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。"
        ),
        "fieldset check2 should mark ui-motion non-wasm stub item complete."
    );
    for required in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
        "fieldset_ui_motion_non_wasm_noop_stub_contract_is_enforced",
        "fieldset_motion_module_degrades_safely_without_animation_handle_assumption",
        "fieldset_platform_script_covers_ui_motion_non_wasm_stub_checks",
    ] {
        assert!(
            source.contains(required),
            "fieldset check2 ui-motion non-wasm section should reference `{required}`."
        );
    }
}

#[test]
fn fieldset_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let ui_motion_source = load_source("../../crates/ui-motion/src/lib.rs");

    for required in [
        "pub fn resolve_effective_motion(",
        "if prefers_reduced_motion {",
        "duration_ms: MIN_DURATION_MS,",
        "distance_px: 0.0,",
        "stiffness: motion.stiffness,",
        "damping: motion.damping,",
        "let motion = resolve_effective_motion(motion, ui_motion::web::prefers_reduced_motion());",
        "pub fn attach_motion(motion: FieldsetMotion) -> String {",
    ] {
        assert!(
            motion_source.contains(required),
            "fieldset motion should keep reduced-motion downgrade contract `{required}`."
        );
    }

    for forbidden in ["data-state", "aria-", "role=", "set_attribute(\"aria-\")"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion layer should not mutate semantics token `{forbidden}`."
        );
    }

    for required in [
        "data-state=move || state.get().data_state_attr",
        "data-ui-state=move || agent_contract.get().state_attr",
        "data-message-kind=move || state.get().message_kind_attr",
        "aria-disabled=move || a11y.get().aria_disabled",
        "aria-invalid=move || a11y.get().aria_invalid",
        "lang=move || a11y.get().lang.clone()",
        "dir=move || a11y.get().dir",
    ] {
        assert!(
            view_source.contains(required),
            "fieldset view should keep SSR/hydration semantic markers stable via `{required}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "web_sys",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "fieldset view semantics should not split by platform token `{forbidden}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web;",
        "pub fn prefers_reduced_motion() -> bool {",
    ] {
        assert!(
            ui_motion_source.contains(required),
            "ui-motion backend should keep explicit wasm/non-wasm branch `{required}`."
        );
    }
}

#[test]
fn fieldset_platform_script_covers_reduced_motion_ssr_wasm_contract() {
    let script_source = load_source("../../scripts/check-ui-platforms.sh");
    for required in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-fieldset,inject-css",
        "echo \"[platform] fieldset reduced-motion/ssr/wasm contract\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
    ] {
        assert!(
            script_source.contains(required),
            "platform script should keep fieldset reduced-motion/SSR/wasm guard `{required}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_reduced_motion_ssr_wasm_item_complete() {
    let source = load_source("src/field_form/fieldset/check2.md");
    assert!(
        source.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。"),
        "fieldset check2 should mark reduced-motion/SSR/wasm item complete."
    );
    for required in [
        "fieldset_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
        "fieldset_platform_script_covers_reduced_motion_ssr_wasm_contract",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
    ] {
        assert!(
            source.contains(required),
            "fieldset check2 reduced-motion/SSR/wasm section should reference `{required}`."
        );
    }
}

#[test]
fn fieldset_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let fieldset_e2e_source = load_source("../../e2e/tests/docs_app_fieldset_contract.spec.mjs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/field_form/fieldset/check2.md");
    let root_check2_source = load_source("../../components/fieldset/check2.md");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "max_update_ms: Some(10.0),",
        "\"fieldset\" => UiPerfBudget {",
        "max_heap_kb: Some(384.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep performance budget contract token `{needle}`."
        );
    }

    assert!(
        pages_source.contains(
            "component_doc!(\"Fieldset\", \"fieldset\", \"Forms\", forms_extra::fieldset)"
        ),
        "Fieldset docs page should stay in component coverage traversal."
    );

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose repeatable perf regression marker `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"ui-perf-probe\"",
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-update-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-heap-kb\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", \"mount-plus-budget\");",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            fieldset_e2e_source.contains(needle),
            "fieldset e2e contract should keep perf observability guard `{needle}`."
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_source.contains(needle),
            "global docs coverage e2e should keep perf regression guard `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`."
        );
    }

    for source in [&check2_source, &root_check2_source] {
        for needle in [
            "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
            "渲染次数预算为 `1`",
            "render_count",
            "若当前测试框架暂不支持精确渲染计数",
            "等价证据",
            "fieldset_performance_governance_contract_is_budgeted_traceable_and_blocking",
        ] {
            assert!(
                source.contains(needle),
                "fieldset checklist should keep performance governance marker `{needle}`."
            );
        }
    }

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
        "data-required-source=move || view_state.get().required_source_attr",
        "data-disabled-source=move || view_state.get().disabled_source_attr",
        "data-invalid-source=move || view_state.get().invalid_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset view should expose perf-attribution marker `{needle}`."
        );
    }

    for needle in [
        "pub fn sanitize_motion(motion: FieldsetMotion) -> FieldsetMotion {",
        "pub fn resolve_effective_motion(",
        "pub fn attach_motion(motion: FieldsetMotion) -> String {",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-fieldset-motion-stiffness",
        "--ui-fieldset-motion-damping",
    ] {
        assert!(
            motion_source.contains(needle),
            "fieldset motion module should keep bounded perf path token `{needle}`."
        );
    }
    for forbidden in [
        "Effect::new(",
        "request_animation_frame",
        "set_interval",
        "set_timeout",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion should avoid high-frequency runtime loop token `{forbidden}`."
        );
    }

    for needle in [
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }
}

#[test]
fn fieldset_performance_check_script_covers_budget_and_follow_up_gates() {
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    for needle in [
        "echo \"[perf] contract: fieldset performance governance\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_performance_governance_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。"),
            "fieldset check2 should mark performance governance item complete."
        );
        for required in [
            "fieldset_performance_governance_contract_is_budgeted_traceable_and_blocking",
            "fieldset_performance_check_script_covers_budget_and_follow_up_gates",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_performance_governance_contract_is_budgeted_traceable_and_blocking",
            "scripts/check-ui-performance.sh",
            "render_count",
            "等价证据",
        ] {
            assert!(
                source.contains(required),
                "fieldset check2 performance section should reference `{required}`."
            );
        }
    }
}

#[test]
fn fieldset_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let local_semantics_source = load_source("../../components/fieldset/test/semantics.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_fieldset_contract.spec.mjs");
    let check2_source = load_source("../../components/fieldset/check2.md");
    let check2_source_src = load_source("src/field_form/fieldset/check2.md");
    let perf_script_source = load_source("../../scripts/check-ui-performance.sh");
    let semantics_source = load_source("tests/fieldset/semantics.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for needle in [
        "aria-label=move || a11y.get().aria_label.clone()",
        "aria-disabled=move || a11y.get().aria_disabled",
        "aria-invalid=move || a11y.get().aria_invalid",
        "data-state=move || state.get().data_state_attr",
        "data-required-source=move || view_state.get().required_source_attr",
        "data-disabled-source=move || view_state.get().disabled_source_attr",
        "data-invalid-source=move || view_state.get().invalid_source_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset view should keep aria/data contract marker `{needle}`."
        );
    }

    for forbidden in [
        "on:focus",
        "on:blur",
        "on:keydown",
        "on:keyup",
        "on:pointerdown",
        "on:pointerup",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "fieldset should not introduce focus-navigation runtime handler `{forbidden}` in view."
        );
    }

    for needle in [
        "fn fieldset_component_depends_on_layered_kernel_shell_crates()",
        "fn fieldset_public_api_surface_is_stable_and_not_dom_leaky()",
        "fn fieldset_view_assembles_logic_headless_and_motion_without_reimplementing_kernels()",
        "fn fieldset_component_has_local_semantics_test_file()",
    ] {
        assert!(
            local_semantics_source.contains(needle),
            "fieldset local semantics suite should keep contract test `{needle}`."
        );
    }

    for needle in [
        "body:not(:has(#boot))",
        "[data-component=\"fieldset\"] [data-slot=\"fieldset\"]",
        "toHaveAttribute(\"data-required-source\", \"required\")",
        "toHaveAttribute(\"data-invalid-source\", \"is_invalid\")",
        "toHaveAttribute(\"data-ui-stream-mode\", \"snapshot\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "fieldset e2e should keep semantic contract assertion `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "screenshot("] {
        assert!(
            !e2e_source.contains(forbidden),
            "fieldset e2e should avoid snapshot-only assertion token `{forbidden}`."
        );
    }

    for needle in [
        "fn fieldset_has_no_focus_stack_overlay_restore_contract()",
        "fn fieldset_check2_marks_focus_stack_item_complete_as_na()",
        "fn fieldset_performance_governance_contract_is_budgeted_traceable_and_blocking()",
    ] {
        assert!(
            semantics_source.contains(needle),
            "fieldset semantics suite should keep focus/perf contract test `{needle}`."
        );
    }

    let perf_gate_needle = "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_performance_governance_contract_is_budgeted_traceable_and_blocking";
    assert!(
        perf_script_source.contains(perf_gate_needle),
        "performance gate script should include `{perf_gate_needle}`."
    );

    let matrix_gate_needle = "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement";
    assert!(
        perf_script_source.contains(matrix_gate_needle),
        "performance gate script should include `{matrix_gate_needle}`."
    );

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance follow-up should keep `{needle}` marker."
        );
    }

    for source in [&check2_source, &check2_source_src] {
        for needle in [
            "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
            "fieldset_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
            "fieldset_performance_script_covers_semantics_and_performance_regression_matrix",
            "scripts/check-ui-performance.sh",
            "`render_count` 自动化回归仍在仓库统一 follow-up",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 semantics+performance section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_performance_script_covers_semantics_and_performance_regression_matrix() {
    let script_source = load_source("../../scripts/check-ui-performance.sh");

    for needle in [
        "echo \"[perf] contract: fieldset semantics/perf matrix\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            script_source.contains(needle),
            "performance check script should include `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_semantics_and_performance_regression_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。"),
            "fieldset check2 should mark semantics+performance item complete."
        );
        for needle in [
            "fieldset_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
            "fieldset_performance_script_covers_semantics_and_performance_regression_matrix",
            "fieldset_performance_governance_contract_is_budgeted_traceable_and_blocking",
            "fieldset_has_no_focus_stack_overlay_restore_contract",
            "scripts/check-ui-performance.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 semantics+performance item should include `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let manifest_source = load_source("../../components/fieldset/src/Component.toml");
    let protocol_source = load_source("src/field_form/fieldset/protocol.rs");
    let mod_source = load_source("src/field_form/fieldset/mod.rs");
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let styles_source = load_source("src/field_form/fieldset/styles.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let check2_source = load_source("../../components/fieldset/check2.md");
    let check2_source_src = load_source("src/field_form/fieldset/check2.md");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Fieldset\"",
        "crate = \"ui-fieldset\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "fieldset manifest should keep stable v1 marker `{needle}`."
        );
    }

    for needle in [
        "pub enum FieldsetComponentSchemaVersion",
        "V1,",
        "pub struct FieldsetComponentSpec",
        "pub schema_version: FieldsetComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "fieldset protocol should keep v1-only schema contract marker `{needle}`."
        );
    }

    let combined = format!(
        "{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}\n{protocol_source}"
    );
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "contract.v2",
        "SchemaRegistry",
        "V2,",
    ] {
        assert!(
            !manifest_source.contains(forbidden) && !combined.contains(forbidden),
            "fieldset should not introduce major-version migration marker `{forbidden}` in current scope."
        );
    }

    for source in [&check2_source, &check2_source_src] {
        for needle in [
            "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
            "N/A：本次 `Fieldset` 未发生跨大版本 API 破坏升级",
            "schema_version = \"1\"",
            "migrate_v1_to_v2",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 should keep version-migration governance marker `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_check2_marks_version_deprecation_migration_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。"),
            "fieldset check2 should mark version-migration item complete."
        );

        for needle in [
            "N/A：本次 `Fieldset` 未发生跨大版本 API 破坏升级",
            "schema_version = \"1\"",
            "fieldset_version_deprecation_migration_is_na_without_major_breaking_upgrade",
            "scripts/check-ui-engineering.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 version-migration section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_view_macro_complexity_is_split_into_semantic_subblocks() {
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for needle in [
        "fn render_legend_block(",
        "fn render_actions_block(",
        "fn render_message_block(",
        "render_legend_block(state.clone(), view_state.clone())",
        "render_actions_block(state.clone(), actions)",
        "render_message_block(state, view_state)",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset view should split macro-heavy structure into semantic helper token `{needle}`."
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 4,
        "fieldset view macro complexity regression: expected <= 4 `view!` blocks after semantic split, found {view_macro_count}."
    );

    let component_macro_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_macro_count, 1,
        "fieldset should keep one public component entry and avoid local subcomponent noise; found {component_macro_count}."
    );

    assert!(
        view_source.lines().count() <= 320,
        "fieldset view.rs grew too large; split semantic subrenders further if this regresses."
    );

    for forbidden in [
        "for item in",
        ".map(|item|",
        "collect::<Vec<_>>()",
        "#[component]\nfn render_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should avoid loop-heavy macro patterns or nested local components `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_view_macro_check_script_covers_complexity_gate() {
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");
    for needle in [
        "echo \"[view-macro] contract: fieldset view macro split\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_view_macro_complexity_is_split_into_semantic_subblocks",
    ] {
        assert!(
            script_source.contains(needle),
            "view-macro gate script should include `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_view_macro_complexity_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。"),
            "fieldset check2 should mark view-macro complexity item complete."
        );
        for required in [
            "render_legend_block/render_actions_block/render_message_block",
            "fieldset_view_macro_complexity_is_split_into_semantic_subblocks",
            "fieldset_view_macro_check_script_covers_complexity_gate",
            "scripts/check-ui-view-macro.sh",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_view_macro_complexity_is_split_into_semantic_subblocks",
        ] {
            assert!(
                source.contains(required),
                "fieldset check2 view-macro section should reference `{required}`."
            );
        }
    }
}

#[test]
fn fieldset_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for needle in [
        "fn render_legend_block(",
        "fn render_actions_block(",
        "fn render_message_block(",
        "render_legend_block(state.clone(), view_state.clone())",
        "render_actions_block(state.clone(), actions)",
        "render_message_block(state, view_state)",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset view should keep function-first split marker `{needle}`."
        );
    }

    let component_macro_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_macro_count, 1,
        "fieldset should keep one component boundary after function split; found {component_macro_count}."
    );

    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\nfn fieldset_",
        "pub fn render_",
        "pub fn section_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "fieldset should avoid local component abstraction noise `{forbidden}`."
        );
    }

    for semantic_marker in [
        "data-slot=\"fieldset\"",
        "data-slot=\"fieldset-legend\"",
        "data-slot=\"fieldset-field-group\"",
        "data-slot=\"fieldset-actions\"",
        "data-slot=\"fieldset-description\"",
        "data-slot=\"fieldset-error\"",
    ] {
        assert!(
            view_source.contains(semantic_marker),
            "fieldset function split should keep stable semantic marker `{semantic_marker}`."
        );
    }
}

#[test]
fn fieldset_view_macro_check_script_covers_functional_split_gate() {
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");
    for needle in [
        "echo \"[view-macro] contract: fieldset function-first split\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_view_functional_split_prefers_plain_functions_over_local_components",
    ] {
        assert!(
            script_source.contains(needle),
            "view-macro gate script should include `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_functional_split_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。"),
            "fieldset check2 should mark functional-split item complete."
        );
        for required in [
            "render_legend_block/render_actions_block/render_message_block",
            "fieldset_view_functional_split_prefers_plain_functions_over_local_components",
            "fieldset_view_macro_check_script_covers_functional_split_gate",
            "scripts/check-ui-view-macro.sh",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_view_functional_split_prefers_plain_functions_over_local_components",
        ] {
            assert!(
                source.contains(required),
                "fieldset check2 functional-split section should reference `{required}`."
            );
        }
    }
}

#[test]
fn fieldset_static_fragments_are_constantized_or_absent_for_simple_layout() {
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for needle in [
        "const SLOT_FIELDSET: &str = \"fieldset\";",
        "const SLOT_FIELDSET_LEGEND: &str = \"fieldset-legend\";",
        "const SLOT_FIELDSET_REQUIRED: &str = \"fieldset-required\";",
        "const SLOT_FIELDSET_FIELD_GROUP: &str = \"fieldset-field-group\";",
        "const SLOT_FIELDSET_ACTIONS: &str = \"fieldset-actions\";",
        "const SLOT_FIELDSET_DESCRIPTION: &str = \"fieldset-description\";",
        "const SLOT_FIELDSET_ERROR: &str = \"fieldset-error\";",
        "const FIELDSET_REQUIRED_INDICATOR_TEXT: &str = \"*\";",
        "const ROLE_ALERT: &str = \"alert\";",
        "data-slot=SLOT_FIELDSET",
        "data-slot=SLOT_FIELDSET_LEGEND",
        "data-slot=SLOT_FIELDSET_REQUIRED",
        "data-slot=SLOT_FIELDSET_FIELD_GROUP",
        "data-slot=SLOT_FIELDSET_ACTIONS",
        "data-slot=SLOT_FIELDSET_DESCRIPTION",
        "data-slot=SLOT_FIELDSET_ERROR",
        "{FIELDSET_REQUIRED_INDICATOR_TEXT}",
        "role=ROLE_ALERT",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset view should keep static-fragment constantization marker `{needle}`."
        );
    }

    for forbidden in [
        "data-slot=\"fieldset\"",
        "data-slot=\"fieldset-legend\"",
        "data-slot=\"fieldset-required\"",
        "data-slot=\"fieldset-field-group\"",
        "data-slot=\"fieldset-actions\"",
        "data-slot=\"fieldset-description\"",
        "data-slot=\"fieldset-error\"",
        "role=\"alert\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should avoid scattered inline static fragment literal `{forbidden}`."
        );
    }

    for semantic_marker in [
        "aria-hidden=\"true\"",
        "aria-label=move || a11y.get().aria_label.clone()",
        "aria-disabled=move || a11y.get().aria_disabled",
        "aria-invalid=move || a11y.get().aria_invalid",
    ] {
        assert!(
            view_source.contains(semantic_marker),
            "fieldset static-fragment constantization should preserve a11y marker `{semantic_marker}`."
        );
    }
}

#[test]
fn fieldset_view_macro_check_script_covers_static_fragment_gate() {
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");
    for needle in [
        "echo \"[view-macro] contract: fieldset static fragment constantization\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_static_fragments_are_constantized_or_absent_for_simple_layout",
    ] {
        assert!(
            script_source.contains(needle),
            "view-macro gate script should include `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_static_fragment_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。"),
            "fieldset check2 should mark static-fragment constantization item complete."
        );
        for required in [
            "SLOT_FIELDSET/SLOT_FIELDSET_LEGEND/SLOT_FIELDSET_REQUIRED/SLOT_FIELDSET_FIELD_GROUP/SLOT_FIELDSET_ACTIONS/SLOT_FIELDSET_DESCRIPTION/SLOT_FIELDSET_ERROR",
            "FIELDSET_REQUIRED_INDICATOR_TEXT",
            "fieldset_static_fragments_are_constantized_or_absent_for_simple_layout",
            "fieldset_view_macro_check_script_covers_static_fragment_gate",
            "scripts/check-ui-view-macro.sh",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_static_fragments_are_constantized_or_absent_for_simple_layout",
        ] {
            assert!(
                source.contains(required),
                "fieldset check2 static-fragment section should reference `{required}`."
            );
        }
    }
}

#[test]
fn fieldset_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    for rel_path in [
        "src/field_form/fieldset/mod.rs",
        "src/field_form/fieldset/logic.rs",
        "src/field_form/fieldset/styles.rs",
        "src/field_form/fieldset/view.rs",
        "src/field_form/fieldset/motion.rs",
        "src/field_form/fieldset/protocol.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in [
            "inner_html=",
            "set_inner_html(",
            "dangerously_set_inner_html",
            "markdown_to_html(",
            "format!(\"<",
            "<script",
            "javascript:",
            "onerror=",
            "onload=",
        ] {
            assert!(
                !source.contains(forbidden),
                "fieldset source `{rel_path}` must not contain raw-html injection token `{forbidden}`."
            );
        }
    }

    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "fieldset docs examples must not contain raw-html injection token `{forbidden}`."
        );
    }

    for checklist_source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        for required in [
            "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
            "仅允许编译期常量或明确白名单内容进入 `inner_html`。",
            "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。",
            "使用 `inner_html` 的节点必须补语义测试与安全回归说明。",
        ] {
            assert!(
                checklist_source.contains(required),
                "fieldset checklist should keep inner_html safety governance rule `{required}`."
            );
        }
    }
}

#[test]
fn fieldset_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-inner-html.sh");
    let needle = "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce `{needle}`."
    );
}

#[test]
fn fieldset_check2_marks_inner_html_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。"),
            "fieldset check2 should mark inner_html item complete."
        );
        for required in [
            "fieldset_inner_html_usage_is_forbidden_in_component_and_docs_examples",
            "fieldset_inner_html_check_script_covers_security_contract",
            "scripts/check-ui-inner-html.sh",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_inner_html_usage_is_forbidden_in_component_and_docs_examples",
        ] {
            assert!(
                source.contains(required),
                "fieldset check2 inner_html section should reference `{required}`."
            );
        }
    }
}

#[test]
fn fieldset_wasm_debug_contract_reuses_global_trace_overlay_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let check2_source = load_source("src/field_form/fieldset/check2.md");

    assert!(
        cargo_source.contains("component-fieldset = [\"dep:ui-fieldset\"]"),
        "fieldset should stay on baseline component feature gate."
    );
    assert!(
        !cargo_source.contains("fieldset-wasm-debug")
            && !cargo_source.contains("fieldset_wasm_debug"),
        "fieldset should not introduce component-specific wasm debug feature."
    );

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(needle),
            "docs app should keep shared wasm-debug visual entry `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "data-slot=\"ui-debug-overlay-events\"",
        ".into_iter()",
        ".rev()",
        ".take(40)",
        "let ts_ms = event.ts_ms;",
        "data-kind=kind_attr",
    ] {
        let present = debug_overlay_source.contains(needle) || trace_source.contains(needle);
        assert!(
            present,
            "shared wasm-debug trace timeline should include `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub fn provide_ui_trace(enabled: bool) -> UiTrace",
        "pub fn use_ui_trace() -> Option<UiTrace>",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace primitive should expose `{needle}`."
        );
    }

    for needle in [
        "data-required-source=move || view_state.get().required_source_attr",
        "data-disabled-source=move || view_state.get().disabled_source_attr",
        "data-invalid-source=move || view_state.get().invalid_source_attr",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
        "data-ui-source=move || agent_contract.get().source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset should keep machine-readable debug attribution marker `{needle}`."
        );
    }

    for forbidden in [
        "fieldset-wasm-debug",
        "fieldset_wasm_debug",
        "wasm_debug",
        "trace.emit(",
        "use_ui_trace(",
        "UiDebugOverlay",
        "data-debug-source",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "fieldset local implementation should not leak wasm debug runtime token `{forbidden}`."
        );
    }

    for needle in [
        "WASM 调试要求：关键状态可追踪",
        "开发模式下至少能追踪关键状态变更来源与前后值",
        "关键交互链路应支持最小可复现记录",
        "调试开关默认不进入生产包体与公共 API",
    ] {
        assert!(
            check2_source.contains(needle),
            "fieldset checklist should keep wasm-debug governance marker `{needle}`."
        );
    }
}

#[test]
fn fieldset_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-wasm-debug.sh");
    let needle = "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_wasm_debug_contract_reuses_global_trace_overlay_and_stays_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm-debug check script should enforce `{needle}`."
    );
}

#[test]
fn fieldset_check2_marks_wasm_debug_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。"),
            "fieldset check2 should mark wasm-debug item complete."
        );
        for required in [
            "fieldset_wasm_debug_contract_reuses_global_trace_overlay_and_stays_feature_isolated",
            "fieldset_wasm_debug_check_script_covers_shared_contract",
            "scripts/check-ui-wasm-debug.sh",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_wasm_debug_contract_reuses_global_trace_overlay_and_stays_feature_isolated",
        ] {
            assert!(
                source.contains(required),
                "fieldset check2 wasm-debug section should reference `{required}`."
            );
        }
    }
}

#[test]
fn fieldset_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn fieldset() -> AnyView",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Legend + Description\" code_signal=default_code>",
        "<Playground title=\"Horizontal + Invalid + Actions\" code_signal=invalid_code>",
        "title=\"Fieldset Workbench (Display + Config + Code + CSS Test)\"",
        "test_source_path=\"crates/ui/src/field_form/fieldset/styles.rs\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "Fieldset docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
fn fieldset_dx_workbench_keeps_context_and_isolated_canvas_with_optional_persist_na() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = load_source("src/field_form/fieldset/check2.md");

    for needle in [
        "<div data-playground-scope=scope_id.clone()>",
        "<Card class_name=\"playground__preview\".to_string()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "data-slot=\"playground-controls\"",
        "class_name=\"playground__panel playground__controls\".to_string()",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`."
        );
    }

    for needle in [
        "let (workbench_orientation_index, set_workbench_orientation_index) = signal(Some(0_usize));",
        "let (workbench_required, set_workbench_required) = signal(false);",
        "let (workbench_disabled, set_workbench_disabled) = signal(false);",
        "let (workbench_invalid, set_workbench_invalid) = signal(false);",
        "let (workbench_show_actions, set_workbench_show_actions) = signal(false);",
        "let (workbench_rtl, set_workbench_rtl) = signal(false);",
        "data-slot=\"fieldset-workbench-controls\"",
        "data-slot=\"fieldset-workbench-compare\"",
        "\"Scenario compare\"",
        "Switch checked=workbench_required set_checked=set_workbench_required",
        "Switch checked=workbench_disabled set_checked=set_workbench_disabled",
        "Switch checked=workbench_invalid set_checked=set_workbench_invalid",
    ] {
        assert!(
            docs_source.contains(needle),
            "Fieldset docs should keep context-preserving interactive marker `{needle}`."
        );
    }

    for forbidden in [
        "FIELDSET_WORKBENCH_STORAGE_KEY",
        "load_fieldset_workbench_state(",
        "save_fieldset_workbench_state(",
        "clear_fieldset_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Fieldset keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
        );
    }

    for required in [
        "DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
    ] {
        assert!(
            check2_source.contains(required),
            "Fieldset checklist should keep DX governance rule `{required}`."
        );
    }
}

#[test]
fn fieldset_dx_check_script_covers_hot_reload_and_workbench_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_dx_workbench_keeps_context_and_isolated_canvas_with_optional_persist_na",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn fieldset_dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: fieldset docs product copy-paste-ready + streaming/snapshot contract\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include docs-product contract marker `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_dx_contract_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。"),
            "fieldset check2 should mark DX gate complete."
        );

        for needle in [
            "playground.rs",
            "compose_scoped_css",
            "Fieldset Workbench (Display + Config + Code + CSS Test)",
            "optional persisted workbench state as N/A",
            "scripts/check-ui-dx.sh",
            "fieldset_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
            "fieldset_dx_workbench_keeps_context_and_isolated_canvas_with_optional_persist_na",
            "fieldset_dx_check_script_covers_hot_reload_and_workbench_contract",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 DX section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_engineering_contract_uses_serde_protocol_and_structured_schema_defaults() {
    let mod_source = load_source("src/field_form/fieldset/mod.rs");
    let protocol_source = load_source("src/field_form/fieldset/protocol.rs");

    assert!(
        !mod_source.contains("pub mod protocol;"),
        "fieldset public API should not expose protocol internals by default."
    );

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "pub enum FieldsetComponentSchemaVersion",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(rename_all = \"snake_case\")]",
        "pub struct FieldsetComponentSpec",
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(default)]",
        "pub schema_version: FieldsetComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "fieldset protocol should keep serde/schema contract marker `{needle}`."
        );
    }

    for forbidden in [
        "serde_json::",
        "from_json(",
        "to_json_result(",
        "SchemaError",
    ] {
        assert!(
            !protocol_source.contains(forbidden),
            "fieldset protocol should avoid ad-hoc serde drift token `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("src/field_form/fieldset/mod.rs"),
        load_source("src/field_form/fieldset/logic.rs"),
        load_source("src/field_form/fieldset/view.rs"),
        load_source("src/field_form/fieldset/styles.rs"),
        load_source("src/field_form/fieldset/motion.rs"),
        load_source("src/field_form/fieldset/protocol.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing marker `{required}`."
        );
    }

    for forbidden_feature in [
        "fieldset-wasm-debug",
        "fieldset_wasm_debug",
        "component-fieldset-wasm-debug",
    ] {
        assert!(
            !cargo_source.contains(forbidden_feature),
            "fieldset should not define component-local tracing feature `{forbidden_feature}`."
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::fieldset::",
        "const FIELDSET_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "fieldset should avoid tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/field_form/fieldset/mod.rs");
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let styles_source = load_source("src/field_form/fieldset/styles.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let protocol_source = load_source("src/field_form/fieldset/protocol.rs");
    let manifest_source = load_source("../../components/fieldset/Cargo.toml");

    let sources = [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
        &protocol_source,
        &manifest_source,
    ];
    for source in sources {
        for forbidden in [
            "tokio",
            "tokio::",
            "async_std",
            "async_std::",
            "async-std",
            "runtime::Handle",
            "smol::",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "fieldset engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    for forbidden in ["web_sys", "wasm_bindgen", "js_sys"] {
        assert!(
            !mod_source.contains(forbidden),
            "fieldset public module boundary should not leak browser runtime type `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");

    for needle in [
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_engineering_contract_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。"),
            "fieldset check2 should mark engineering gate complete."
        );

        for needle in [
            "components/fieldset/src/protocol.rs",
            "FieldsetComponentSchemaVersion",
            "FieldsetComponentSpec",
            "button-wasm-debug",
            "target: \"ui::button::state_change\"",
            "scripts/check-ui-engineering.sh",
            "fieldset_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
            "fieldset_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
            "fieldset_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
            "fieldset_engineering_check_script_covers_serde_tracing_and_runtime_boundaries",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 engineering section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("src/field_form/fieldset/styles.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");

    for needle in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-fieldset-horizontal-legend-min-inline-size, var(--ui-fallback-fieldset-horizontal-legend-min-inline-size))",
        "var(--ui-fieldset-horizontal-legend-max-inline-size, var(--ui-fallback-fieldset-horizontal-legend-max-inline-size))",
    ] {
        assert!(
            styles_source.contains(needle),
            "fieldset styles should keep defensive variable chain marker `{needle}`."
        );
    }

    for forbidden in [
        "#",
        "8rem",
        "14rem",
        "outline: 1px solid",
        "outline-offset: 2px",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "fieldset styles should not keep hard-coded size/color terminal `{forbidden}`."
        );
    }

    for needle in [
        "--ui-fieldset-horizontal-legend-min-inline-size: 128px;",
        "--ui-fallback-fieldset-horizontal-legend-min-inline-size: 128px;",
        "--ui-fieldset-horizontal-legend-max-inline-size: 224px;",
        "--ui-fallback-fieldset-horizontal-legend-max-inline-size: 224px;",
    ] {
        assert!(
            theme_css_source.contains(needle),
            "ui-theme css should provide fieldset fallback SSOT terminal `{needle}`."
        );
    }
}

#[test]
fn fieldset_defensive_variables_check_script_covers_style_fallback_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");
    let needle = "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_styles_use_defensive_variable_fallback_chain";

    assert!(
        script_source.contains(needle),
        "contract-hygiene script should enforce `{needle}`."
    );
}

#[test]
fn fieldset_check2_marks_defensive_variables_contract_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
            "fieldset check2 should mark defensive variables gate complete."
        );

        for needle in [
            "components/fieldset/src/styles.rs",
            "crates/ui-theme/src/css.rs",
            "fieldset_styles_use_defensive_variable_fallback_chain",
            "fieldset_defensive_variables_check_script_covers_style_fallback_contract",
            "scripts/check-ui-contract-hygiene.sh",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_styles_use_defensive_variable_fallback_chain",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 defensive-variables section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-fieldset\")]",
        "out.push_str(crate::field_form::fieldset::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(needle),
            "ui css entry should enforce cascade-layer contract `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized css injection contract `{needle}`."
        );
    }

    assert!(
        view_source.contains("style=move || motion_style.get_value()"),
        "fieldset view should keep a single runtime style injection path from motion css custom properties."
    );

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
        "style:top=",
        "style:left=",
        "style:right=",
        "style:bottom=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should not include plain inline style token `{forbidden}`."
        );
    }

    for (line_index, line) in motion_source.lines().enumerate() {
        if let Some(pos) = line.find("--ui-fieldset-motion-") {
            assert!(
                line[pos..].contains(':'),
                "fieldset motion css variable assignment should stay explicit at line {}.",
                line_index + 1
            );
        }
    }

    for forbidden in ["top:", "left:", "right:", "bottom:", "width:", "height:"] {
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion runtime style should only set css custom properties; found `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_cascade_layer_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");
    let needle = "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_cascade_layer_and_runtime_style_contract_is_enforced";

    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn fieldset_check2_marks_cascade_layer_contract_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
            "fieldset check2 should mark cascade-layer gate complete."
        );

        for needle in [
            "fieldset_cascade_layer_and_runtime_style_contract_is_enforced",
            "fieldset_cascade_layer_check_script_covers_contract",
            "scripts/check-ui-contract-hygiene.sh",
            "crates/ui/src/css.rs",
            "crates/ui/src/root.rs",
            "components/fieldset/src/view.rs",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 cascade-layer section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let ui_motion_source = load_source("../../crates/ui-motion/src/lib.rs");

    for required in [
        "pub struct FieldsetMotion {",
        "pub stiffness: f64,",
        "pub damping: f64,",
        "pub fn sanitize_motion(motion: FieldsetMotion) -> FieldsetMotion {",
        "stiffness: if motion.stiffness.is_finite() {",
        "damping: if motion.damping.is_finite() {",
        "pub fn resolve_effective_motion(",
        "if prefers_reduced_motion {",
        "duration_ms: MIN_DURATION_MS,",
        "distance_px: 0.0,",
        "stiffness: motion.stiffness,",
        "damping: motion.damping,",
        "pub fn attach_motion(motion: FieldsetMotion) -> String {",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-fieldset-motion-stiffness",
        "--ui-fieldset-motion-damping",
    ] {
        assert!(
            motion_source.contains(required),
            "fieldset motion contract should keep `{required}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
    ] {
        assert!(
            ui_motion_source.contains(required),
            "ui-motion should keep cross-platform reduced-motion/no-op backend marker `{required}`."
        );
    }

    for forbidden in [
        "ui_motion::web::animate(",
        "web_sys::Animation",
        "wasm_bindgen",
        "JsValue",
        "unwrap(",
        "expect(",
        "panic!(",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion contract should avoid runtime-handle/panic path `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_platform_script_covers_motion_contractualization() {
    let script_source = load_source("../../scripts/check-ui-platforms.sh");
    for needle in [
        "echo \"[platform] fieldset motion contractualization (component contract + reduced-motion + non-wasm no-op)\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
    ] {
        assert!(
            script_source.contains(needle),
            "platform script should keep fieldset motion contractualization gate `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_motion_contractualization_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。"),
            "fieldset check2 should mark motion contractualization item complete."
        );

        for needle in [
            "components/fieldset/src/motion.rs",
            "stiffness",
            "damping",
            "fieldset_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
            "fieldset_platform_script_covers_motion_contractualization",
            "scripts/check-ui-platforms.sh",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 motion-contract section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");
    let headless_controllable_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let headless_presence_source = load_source("../../crates/ui-headless/src/presence.rs");

    for required in ["feature = \"component-fieldset\"", "pub mod field_form {"] {
        assert!(
            lib_source.contains(required),
            "ui lib entry should keep feature-gated public surface marker `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-fieldset\")]",
        "out.push_str(crate::field_form::fieldset::styles::CSS);",
        "pub fn push_components_css(out: &mut String)",
    ] {
        assert!(
            css_source.contains(required),
            "ui css entry should keep feature-gated aggregation marker `{required}`."
        );
    }

    for required in [
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should keep centralized theme/css/i18n entry marker `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-fieldset\")]",
        "pub use ui_fieldset as fieldset;",
    ] {
        assert!(
            lib_source.contains(required),
            "fieldset inline field_form export should stay feature-gated via `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringAnimator",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "active_highlight should stay a shared visual primitive marker `{required}`."
        );
    }

    for forbidden in [
        "pub use web_sys",
        "pub mod web_sys",
        "pub use wasm_bindgen",
        "Fieldset",
        "data-required-source",
        "aria-controls",
    ] {
        let leaked = lib_source.contains(forbidden) || active_highlight_source.contains(forbidden);
        assert!(
            !leaked,
            "ui fixed entry boundary should reject platform/component semantic leak `{forbidden}`."
        );
    }

    assert!(
        !path_exists("src/overlay_open.rs"),
        "ui should not carry legacy entry `src/overlay_open.rs`."
    );
    assert!(
        !path_exists("src/presence.rs"),
        "ui should not carry legacy entry `src/presence.rs`."
    );
    assert!(
        !path_exists("src/a11y.rs"),
        "ui should not carry legacy entry `src/a11y.rs`."
    );

    assert!(
        path_exists("../../crates/ui-headless/src/controllable_state.rs"),
        "headless canonical controllable-state path should exist."
    );
    assert!(
        path_exists("../../crates/ui-headless/src/presence.rs"),
        "headless canonical presence path should exist."
    );
    assert!(
        headless_a11y_source.contains("pub fn aria_controls_when_open("),
        "headless a11y shared utility should keep canonical helper."
    );
    assert!(
        headless_controllable_source.contains("pub fn use_controllable_state"),
        "headless controllable-state primitive should remain canonical."
    );
    assert!(
        headless_presence_source.contains("pub fn use_presence"),
        "headless presence primitive should remain canonical."
    );
}

#[test]
fn fieldset_entrypoints_script_covers_fixed_entry_file_boundaries() {
    let script_source = load_source("../../scripts/check-ui-entrypoints.sh");
    for needle in [
        "echo \"[entrypoints] contract: fieldset fixed entry files and forbidden file guards\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_ui_components_fixed_entry_files_follow_layered_boundaries",
    ] {
        assert!(
            script_source.contains(needle),
            "entrypoints script should enforce fieldset boundary gate `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_ui_components_fixed_entry_files_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] `ui` 固定入口文件落点正确。"),
            "fieldset check2 should mark ui fixed-entry item complete."
        );

        for needle in [
            "crates/ui/src/lib.rs",
            "crates/ui/src/css.rs",
            "crates/ui/src/root.rs",
            "crates/ui-visual-primitive/src/active_highlight.rs",
            "crates/ui/src/overlay_open.rs",
            "crates/ui/src/presence.rs",
            "crates/ui/src/a11y.rs",
            "fieldset_ui_components_fixed_entry_files_follow_layered_boundaries",
            "fieldset_entrypoints_script_covers_fixed_entry_file_boundaries",
            "scripts/check-ui-entrypoints.sh",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_ui_components_fixed_entry_files_follow_layered_boundaries",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 fixed-entry section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_component_directory_standard_files_follow_contract_and_na_paths() {
    for required in [
        "src/field_form/fieldset/mod.rs",
        "src/field_form/fieldset/logic.rs",
        "src/field_form/fieldset/styles.rs",
        "src/field_form/fieldset/view.rs",
        "src/field_form/fieldset/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "fieldset component directory should include required file `{required}`."
        );
    }

    for forbidden in [
        "src/field_form/fieldset/render.rs",
        "src/field_form/fieldset/spec.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "fieldset simple component scope should not include `{forbidden}`."
        );
    }

    let mod_source = load_source("src/field_form/fieldset/mod.rs");
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let styles_source = load_source("src/field_form/fieldset/styles.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");

    for required in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Fieldset;",
        "pub use motion::FieldsetMotion;",
    ] {
        assert!(
            mod_source.contains(required),
            "fieldset mod.rs should keep minimal stable export marker `{required}`."
        );
    }
    for forbidden in ["pub mod view;", "pub mod logic;", "pub mod protocol;"] {
        assert!(
            !mod_source.contains(forbidden),
            "fieldset mod.rs should avoid over-export marker `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_boolean_axis(input: FieldsetBooleanAxisInput) -> FieldsetBooleanAxis",
        "pub fn resolve_view_state(input: FieldsetViewStateInput) -> FieldsetViewState",
        "pub fn resolve_agent_contract(state: FieldsetState) -> FieldsetAgentContract",
    ] {
        assert!(
            logic_source.contains(required),
            "fieldset logic.rs should keep normalization/derivation marker `{required}`."
        );
    }
    for forbidden in ["#[component]", "view! {", "pub const CSS", "attach_motion("] {
        assert!(
            !logic_source.contains(forbidden),
            "fieldset logic.rs should not leak render/style/motion concern `{forbidden}`."
        );
    }

    for required in ["pub const CSS: &str = r#\"", "var(--ui-", ".ui-fieldset"] {
        assert!(
            styles_source.contains(required),
            "fieldset styles.rs should keep token-first static css marker `{required}`."
        );
    }
    for forbidden in [
        "#[component]",
        "Signal<",
        "resolve_state(",
        "use_controllable_state",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "fieldset styles.rs should not leak runtime/state concern `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "use ui_headless::{A11yDirection, fieldset_attrs, use_controllable_state};",
        "logic::resolve_view_state(logic::FieldsetViewStateInput {",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(required),
            "fieldset view.rs should keep structure + headless mount marker `{required}`."
        );
    }
    for forbidden in [
        "pub const CSS",
        "Theme::new(",
        "default_text_field_motion_tokens()",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "fieldset view.rs should not leak style/theme engine concern `{forbidden}`."
        );
    }

    for required in [
        "pub struct FieldsetMotion {",
        "pub fn sanitize_motion(motion: FieldsetMotion) -> FieldsetMotion {",
        "pub fn attach_motion(motion: FieldsetMotion) -> String {",
        "--ui-fieldset-motion-",
    ] {
        assert!(
            motion_source.contains(required),
            "fieldset motion.rs should keep motion-contract mapping marker `{required}`."
        );
    }
    for forbidden in ["#[component]", "view! {", "fieldset_attrs(", "data-slot="] {
        assert!(
            !motion_source.contains(forbidden),
            "fieldset motion.rs should not leak view/headless semantic concern `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_component_files_script_covers_standard_file_layout_contract() {
    let script_source = load_source("../../scripts/check-ui-component-files.sh");
    for needle in [
        "echo \"[component-files] contract: fieldset standard file layout + scoped responsibilities\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_component_directory_standard_files_follow_contract_and_na_paths",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files script should enforce fieldset layout gate `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_component_directory_layout_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 组件目录标准文件落点正确。"),
            "fieldset check2 should mark component-directory layout item complete."
        );
        for needle in [
            "components/fieldset/src/mod.rs",
            "components/fieldset/src/logic.rs",
            "components/fieldset/src/styles.rs",
            "components/fieldset/src/view.rs",
            "components/fieldset/src/motion.rs",
            "components/fieldset/src/spec.rs",
            "components/fieldset/src/render.rs",
            "fieldset_component_directory_standard_files_follow_contract_and_na_paths",
            "fieldset_component_files_script_covers_standard_file_layout_contract",
            "scripts/check-ui-component-files.sh",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_component_directory_standard_files_follow_contract_and_na_paths",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 component-directory section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_file_placement_discipline_is_strict_for_component_scope() {
    for required in [
        "src/field_form/fieldset/mod.rs",
        "src/field_form/fieldset/logic.rs",
        "src/field_form/fieldset/styles.rs",
        "src/field_form/fieldset/view.rs",
        "src/field_form/fieldset/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "fieldset file-placement discipline should keep required component file `{required}`."
        );
    }

    let forbidden = "src/field_form/fieldset/render.rs";
    assert!(
        !path_exists(forbidden),
        "fieldset file-placement discipline forbids file `{forbidden}`."
    );

    assert!(
        !path_exists("src/field_form/fieldset/spec.rs"),
        "fieldset simple component scope should keep `spec.rs` absent unless complexity requires it."
    );

    let mod_source = load_source("src/field_form/fieldset/mod.rs");
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let styles_source = load_source("src/field_form/fieldset/styles.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");

    for required in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
    ] {
        assert!(
            mod_source.contains(required),
            "fieldset file-placement discipline expects mod.rs export boundary `{required}`."
        );
    }

    for forbidden in [
        "pub mod view;",
        "pub mod logic;",
        "mod render;",
        "pub mod render;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "fieldset mod.rs should not leak implementation boundary `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_boolean_axis(input: FieldsetBooleanAxisInput) -> FieldsetBooleanAxis",
        "pub fn resolve_view_state(input: FieldsetViewStateInput) -> FieldsetViewState",
        "pub fn resolve_agent_contract(state: FieldsetState) -> FieldsetAgentContract",
    ] {
        assert!(
            logic_source.contains(required),
            "fieldset logic.rs should keep normalization/derivation concern `{required}`."
        );
    }

    for required in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles_source.contains(required),
            "fieldset styles.rs should keep token-first static concern `{required}`."
        );
    }

    for required in [
        "#[component]",
        "use ui_headless::{A11yDirection, fieldset_attrs, use_controllable_state};",
        "logic::resolve_view_state(logic::FieldsetViewStateInput {",
    ] {
        assert!(
            view_source.contains(required),
            "fieldset view.rs should keep rendering + headless mount concern `{required}`."
        );
    }

    for required in [
        "pub struct FieldsetMotion {",
        "pub fn sanitize_motion(motion: FieldsetMotion) -> FieldsetMotion {",
        "pub fn attach_motion(motion: FieldsetMotion) -> String {",
    ] {
        assert!(
            motion_source.contains(required),
            "fieldset motion.rs should keep motion-contract mapping concern `{required}`."
        );
    }

    for forbidden in [
        "render.rs",
        "mod render",
        "pub mod render",
        "pub struct FieldsetSpec",
        "FieldsetSpec::new(",
        "Spec::new(",
    ] {
        let leaked = mod_source.contains(forbidden)
            || logic_source.contains(forbidden)
            || styles_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden);
        assert!(
            !leaked,
            "fieldset file-placement discipline should reject leak marker `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_component_files_script_covers_file_placement_discipline_contract() {
    let script_source = load_source("../../scripts/check-ui-component-files.sh");
    for needle in [
        "echo \"[component-files] contract: fieldset file-placement discipline in AI struct-first section\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_file_placement_discipline_is_strict_for_component_scope",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files script should enforce fieldset file-placement discipline gate `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_file_placement_discipline_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
            "fieldset check2 should mark file-placement discipline item complete."
        );
        for needle in [
            "components/fieldset/src/mod.rs",
            "components/fieldset/src/logic.rs",
            "components/fieldset/src/styles.rs",
            "components/fieldset/src/view.rs",
            "components/fieldset/src/motion.rs",
            "components/fieldset/src/spec.rs",
            "components/fieldset/src/render.rs",
            "fieldset_file_placement_discipline_is_strict_for_component_scope",
            "fieldset_component_files_script_covers_file_placement_discipline_contract",
            "scripts/check-ui-component-files.sh",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_file_placement_discipline_is_strict_for_component_scope",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 file-placement discipline section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    assert!(
        !path_exists("src/field_form/fieldset/spec.rs"),
        "fieldset should keep `spec.rs` absent because it is not a complex component."
    );

    let mod_source = load_source("src/field_form/fieldset/mod.rs");
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let styles_source = load_source("src/field_form/fieldset/styles.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "pub struct FieldsetSpec",
        "FieldsetSpec::new(",
        "Spec::new(",
    ] {
        let leaked = mod_source.contains(forbidden)
            || logic_source.contains(forbidden)
            || styles_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden);
        assert!(
            !leaked,
            "fieldset hyper-structure builder is N/A for simple component scope; found `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_component_files_script_covers_hyper_structure_builder_na_contract() {
    let script_source = load_source("../../scripts/check-ui-component-files.sh");
    for needle in [
        "echo \"[component-files] contract: fieldset hyper-structure builder spec contract is explicitly N/A\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files script should enforce fieldset hyper-structure builder N/A gate `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_hyper_structure_builder_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
            "fieldset check2 should mark hyper-structure builder item complete."
        );
        for needle in [
            "N/A（Fieldset 复杂度）",
            "components/fieldset/src/spec.rs",
            "fieldset_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
            "fieldset_component_files_script_covers_hyper_structure_builder_na_contract",
            "scripts/check-ui-component-files.sh",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 hyper-structure section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    for required_file in [
        "src/field_form/fieldset/Component.toml",
        "src/field_form/fieldset/fieldset.rbi",
    ] {
        assert!(
            path_exists(required_file),
            "fieldset context-compression artifact should exist: `{required_file}`."
        );
    }

    let manifest_source = load_source("src/field_form/fieldset/Component.toml");
    let rbi_source = load_source("src/field_form/fieldset/fieldset.rbi");

    for needle in [
        "schema_version = \"1\"",
        "[component]",
        "name = \"Fieldset\"",
        "crate = \"ui-fieldset\"",
        "name = \"is_required\"",
        "name = \"default_is_required\"",
        "name = \"on_is_required_change\"",
        "name = \"is_disabled\"",
        "name = \"default_is_disabled\"",
        "name = \"on_is_disabled_change\"",
        "name = \"is_invalid\"",
        "name = \"default_is_invalid\"",
        "name = \"on_is_invalid_change\"",
        "name = \"data-ui-schema\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "fieldset Component.toml should keep context-compression marker `{needle}`."
        );
    }

    for needle in [
        "pub type FieldsetOrientation = ui_state_primitives::fieldset::FieldsetOrientation;",
        "pub type FieldsetTone = ui_state_primitives::fieldset::FieldsetTone;",
        "pub struct FieldsetMotion {",
        "pub fn Fieldset(",
        "is_required: Option<bool>",
        "default_is_required: Option<bool>",
        "on_is_required_change: Option<leptos::prelude::Callback<bool>>",
        "is_disabled: Option<bool>",
        "default_is_disabled: Option<bool>",
        "on_is_disabled_change: Option<leptos::prelude::Callback<bool>>",
        "is_invalid: Option<bool>",
        "default_is_invalid: Option<bool>",
        "on_is_invalid_change: Option<leptos::prelude::Callback<bool>>",
        "dir: Option<ui_headless::A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "fieldset RBI projection should keep signature marker `{needle}`."
        );
    }
}

#[test]
fn fieldset_component_files_script_covers_context_compression_manifest_contract() {
    let script_source = load_source("../../scripts/check-ui-component-files.sh");
    for needle in [
        "echo \"[component-files] contract: fieldset context-compression manifest + rbi projection\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files script should enforce fieldset context-compression gate `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_context_compression_manifest_and_rbi_contract_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
            "fieldset check2 should mark context-compression manifest/rbi gate complete."
        );

        for needle in [
            "components/fieldset/src/Component.toml",
            "components/fieldset/src/fieldset.rbi",
            "fieldset_context_compression_manifest_and_rbi_projection_are_present_and_current",
            "fieldset_component_files_script_covers_context_compression_manifest_contract",
            "scripts/check-ui-component-files.sh",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_context_compression_manifest_and_rbi_projection_are_present_and_current",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 context-compression section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let manifest_source = load_source("src/field_form/fieldset/Component.toml");
    let e2e_source = load_source("../../e2e/tests/docs_app_fieldset_contract.spec.mjs");

    for needle in [
        "pub struct FieldsetAgentContract {",
        "pub schema_attr: &'static str,",
        "pub schema_version_attr: &'static str,",
        "pub intent_attr: &'static str,",
        "pub action_attr: &'static str,",
        "pub state_attr: &'static str,",
        "pub source_attr: &'static str,",
        "pub stream_support_attr: &'static str,",
        "pub stream_fallback_attr: &'static str,",
        "pub stream_mode_attr: &'static str,",
        "pub output_status_attr: &'static str,",
        "pub fn resolve_agent_contract(state: FieldsetState) -> FieldsetAgentContract {",
    ] {
        assert!(
            logic_source.contains(needle),
            "fieldset logic should keep typed agent-contract schema marker `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema=move || agent_contract.get().schema_attr",
        "data-ui-schema-version=move || agent_contract.get().schema_version_attr",
        "data-ui-intent=move || agent_contract.get().intent_attr",
        "data-ui-action=move || agent_contract.get().action_attr",
        "data-ui-state=move || agent_contract.get().state_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset view should expose machine-readable agent marker `{needle}`."
        );
    }

    for needle in [
        "schema = \"ui.fieldset.agent-contract/v1\"",
        "\"intent\"",
        "\"action\"",
        "\"state\"",
        "\"source\"",
        "\"stream.support\"",
        "\"stream.fallback\"",
        "\"stream.mode\"",
        "\"output.status\"",
        "intent = \"form-grouping\"",
        "action = \"initialize\"",
        "state = \"data-ui-state\"",
        "source = \"data-ui-source\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "fieldset Component.toml should keep agent-contract schema manifest marker `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema",
        "data-ui-state",
        "data-ui-source",
        "data-ui-stream-mode",
        "data-ui-output-status",
    ] {
        assert!(
            e2e_source.contains(needle),
            "fieldset e2e contract should consume agent marker `{needle}`."
        );
    }
}

#[test]
fn fieldset_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for needle in [
        "schema_attr: \"ui.fieldset.agent-contract\"",
        "schema_version_attr: \"1\"",
        "intent_attr: \"form-grouping\"",
        "action_attr: \"initialize\"",
        "state_attr: state.data_state.as_attr(),",
        "source_attr: state.class_source_attr,",
        "stream_support_attr: \"unsupported\"",
        "stream_fallback_attr: \"snapshot\"",
        "stream_mode_attr: \"snapshot\"",
        "output_status_attr: \"verified\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "fieldset agent-contract field should be type-derived and traceable `{needle}`."
        );
    }

    for needle in [
        "let agent_contract = Memo::new(move |_| logic::resolve_agent_contract(state.get()));",
        "data-ui-schema=move || agent_contract.get().schema_attr",
        "data-ui-intent=move || agent_contract.get().intent_attr",
        "data-ui-action=move || agent_contract.get().action_attr",
        "data-ui-state=move || agent_contract.get().state_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset view should bind agent contract via typed struct field `{needle}`."
        );
    }

    for forbidden in [
        "schema_attr: format!(",
        "intent_attr: format!(",
        "action_attr: format!(",
        "state_attr: format!(",
        "source_attr: format!(",
        "data-ui-schema=move || format!(",
        "data-ui-intent=move || format!(",
        "data-ui-action=move || format!(",
        "data-ui-state=move || format!(",
        "data-ui-source=move || format!(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "fieldset should not free-form splice agent schema fields via `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let manifest_source = load_source("src/field_form/fieldset/Component.toml");
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for needle in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "\"logic::resolve_view_state\"",
        "\"logic::resolve_agent_contract\"",
        "\"view::Fieldset\"",
        "\"view::render_legend_block\"",
        "\"view::render_actions_block\"",
        "\"view::render_message_block\"",
        "\"motion::attach_motion\"",
        "blocked = [\"inner_html\", \"dangerously_set_inner_html\", \"<script\", \"javascript:\"]",
    ] {
        assert!(
            manifest_source.contains(needle),
            "fieldset Component.toml should keep whitelist render-path marker `{needle}`."
        );
    }

    for forbidden in [
        "inner_html=",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "fieldset view render path should remain script-injection safe; found `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_contract_hygiene_script_covers_agent_contract_schema_governance() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");
    for needle in [
        "echo \"[contract-hygiene] contract: fieldset agent-contract schema-like markers + whitelist-safe render path\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_agent_contract_schema_governance_rules",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce fieldset agent-contract gate `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_documents_agent_contract_schema_governance_rules() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。"),
            "fieldset check2 should mark agent-contract schema governance gate complete."
        );

        for needle in [
            "data-ui-schema",
            "data-ui-intent",
            "data-ui-action",
            "data-ui-state",
            "data-ui-source",
            "components/fieldset/src/Component.toml",
            "components/fieldset/src/logic.rs",
            "components/fieldset/src/view.rs",
            "fieldset_agent_contract_is_schema_typed_and_machine_readable",
            "fieldset_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
            "fieldset_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
            "fieldset_contract_hygiene_script_covers_agent_contract_schema_governance",
            "scripts/check-ui-contract-hygiene.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 agent-contract section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_streaming_definition_is_llm_output_only_with_two_modes() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");

    for needle in [
        "stream_support_attr: \"unsupported\"",
        "stream_fallback_attr: \"snapshot\"",
        "stream_mode_attr: \"snapshot\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "fieldset streaming definition should stay typed and snapshot-pinned marker `{needle}`."
        );
    }

    for needle in [
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset view should expose stream/snapshot semantic marker `{needle}`."
        );
    }
}

#[test]
fn fieldset_streaming_script_covers_two_mode_definition_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");
    for needle in [
        "echo \"[streaming] contract: fieldset checklist pins two-mode streaming definition (LLM-only scope)\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_streaming_definition_is_llm_output_only_with_two_modes",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming script should enforce fieldset two-mode definition gate `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_streaming_definition_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
            "fieldset check2 should mark two-mode streaming-definition gate complete."
        );

        for needle in [
            "`Streaming`：LLM 还在生成，界面边生成边显示。",
            "`Snapshot`：LLM 全部生成完成后，一次性显示。",
            "data-ui-stream-support",
            "data-ui-stream-fallback",
            "data-ui-stream-mode",
            "stream_support_attr: \"unsupported\"",
            "stream_fallback_attr: \"snapshot\"",
            "stream_mode_attr: \"snapshot\"",
            "fieldset_streaming_definition_is_llm_output_only_with_two_modes",
            "fieldset_streaming_script_covers_two_mode_definition_contract",
            "scripts/check-ui-streaming.sh",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_streaming_definition_is_llm_output_only_with_two_modes",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 streaming-definition section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let manifest_source = load_source("src/field_form/fieldset/Component.toml");
    let e2e_source = load_source("../../e2e/tests/docs_app_fieldset_contract.spec.mjs");

    for needle in [
        "stream_support_attr: \"unsupported\"",
        "stream_fallback_attr: \"snapshot\"",
        "stream_mode_attr: \"snapshot\"",
        "output_status_attr: \"verified\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "fieldset logic should keep snapshot baseline marker `{needle}`."
        );
    }

    for needle in [
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset view should expose snapshot baseline marker `{needle}`."
        );
    }

    for needle in [
        "name = \"snapshot_rendering\"",
        "enabled = true",
        "name = \"context_compression_manifest\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "fieldset component manifest should keep snapshot-capability marker `{needle}`."
        );
    }

    for needle in [
        "data-ui-stream-mode",
        "data-ui-output-status",
        "data-ui-stream-fallback",
    ] {
        assert!(
            e2e_source.contains(needle),
            "fieldset e2e contract should verify snapshot baseline selector `{needle}`."
        );
    }
}

#[test]
fn fieldset_streaming_script_covers_snapshot_baseline_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");
    for needle in [
        "echo \"[streaming] contract: fieldset snapshot baseline stays default capability\"",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming script should enforce fieldset snapshot-baseline gate `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_snapshot_baseline_item_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        assert!(
            source.contains("- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。"),
            "fieldset check2 should mark snapshot-baseline gate complete."
        );

        for needle in [
            "所有组件都应能消费“完整生成结果”并稳定渲染。",
            "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
            "stream_support_attr: \"unsupported\"",
            "stream_fallback_attr: \"snapshot\"",
            "stream_mode_attr: \"snapshot\"",
            "output_status_attr: \"verified\"",
            "data-ui-stream-mode",
            "fieldset_snapshot_baseline_consumes_complete_result_and_renders_stably",
            "fieldset_streaming_script_covers_snapshot_baseline_contract",
            "scripts/check-ui-streaming.sh",
            "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_snapshot_baseline_consumes_complete_result_and_renders_stably",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 snapshot-baseline section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn fieldset_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("src/field_form/fieldset/check2.md");
    let checklist_source_root = load_source("../../components/fieldset/check2.md");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for source in [checklist_source, checklist_source_root] {
        for required in [
            "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
            "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
            "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
            "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
            "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
            "`Fieldset` 归类为 `Streaming Optional`",
        ] {
            assert!(
                source.contains(required),
                "fieldset checklist should keep streaming responsibility marker `{required}`."
            );
        }
    }

    for script_needle in [
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming check script should enforce `{script_needle}`."
        );
    }
}

#[test]
fn fieldset_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let logic_source = load_source("src/field_form/fieldset/logic.rs");

    for required in [
        "<fieldset",
        "data-slot=SLOT_FIELDSET",
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
        "aria-label=move || a11y.get().aria_label.clone()",
        "aria-disabled=move || a11y.get().aria_disabled",
        "aria-invalid=move || a11y.get().aria_invalid",
        "data-state=move || state.get().data_state_attr",
        "data-message-kind=move || state.get().message_kind_attr",
    ] {
        assert!(
            view_source.contains(required),
            "fieldset should keep continuous aria/data semantics via `{required}` in optional-streaming scope."
        );
    }

    assert!(
        logic_source.contains("output_status_attr: \"verified\""),
        "fieldset should explicitly expose snapshot output status marker `verified`."
    );
}

#[test]
fn fieldset_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let mod_source = load_source("src/field_form/fieldset/mod.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let styles_source = load_source("src/field_form/fieldset/styles.rs");
    let combined =
        format!("{mod_source}\n{view_source}\n{logic_source}\n{motion_source}\n{styles_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "network_error",
        "transport_error",
        "abort_controller",
        "exponential_backoff",
    ] {
        assert!(
            !combined.contains(forbidden),
            "fieldset should keep validation/retry/resilience orchestration out of component layer; found `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_streaming_script_covers_streaming_responsibility_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn fieldset_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let mod_source = load_source("src/field_form/fieldset/mod.rs");
    let logic_source = load_source("src/field_form/fieldset/logic.rs");
    let styles_source = load_source("src/field_form/fieldset/styles.rs");
    let view_source = load_source("src/field_form/fieldset/view.rs");
    let motion_source = load_source("src/field_form/fieldset/motion.rs");
    let protocol_source = load_source("src/field_form/fieldset/protocol.rs");
    let combined = format!(
        "{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}\n{protocol_source}"
    );

    for forbidden in ["unwrap(", "expect(", "unwrap_err(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "fieldset non-test sources should forbid rust-hygiene violation `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("src/field_form/fieldset/logic.rs");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> =",
        "Cow::Borrowed(\"ui-fieldset\")",
        "Cow::Owned(base_class_name)",
        ".map(|class_name| class_name.as_ref())",
    ] {
        assert!(
            logic_source.contains(required),
            "fieldset logic should keep Cow-based string hotspot mitigation marker `{required}`."
        );
    }

    for forbidden in [
        "\"ui-fieldset\".to_string()",
        "\"ui-fieldset--required\".to_string()",
        "\"ui-fieldset--disabled\".to_string()",
        "\"ui-fieldset--invalid\".to_string()",
        "\"ui-fieldset--custom-class\".to_string()",
        "String::from(\"ui-fieldset\")",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "fieldset fallback normalization should avoid string clone hotspot `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = load_source("../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("../../scripts/check-ui-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            script_source.contains(required),
            "rust-hygiene gate script should enforce `{required}`."
        );
    }

    for needle in [
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn fieldset_check2_marks_rust_hygiene_contract_complete() {
    for source in [
        load_source("src/field_form/fieldset/check2.md"),
        load_source("../../components/fieldset/check2.md"),
    ] {
        for needle in [
            "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
            "Cow<'static, str>",
            "./scripts/check-rust-hygiene.sh",
            "fieldset_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
            "fieldset_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
            "fieldset_rust_hygiene_script_enforces_repo_level_hygiene_guards",
            "scripts/check-ui-engineering.sh",
        ] {
            assert!(
                source.contains(needle),
                "fieldset check2 rust-hygiene section should reference `{needle}`."
            );
        }
    }
}
