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
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join(rel_path).exists()
}

fn contains_hex_color_literal(source: &str) -> bool {
    let bytes = source.as_bytes();
    let mut index = 0usize;

    while index < bytes.len() {
        if bytes[index] != b'#' {
            index += 1;
            continue;
        }

        let mut cursor = index + 1;
        let mut hex_len = 0usize;
        while cursor < bytes.len() && hex_len < 8 && bytes[cursor].is_ascii_hexdigit() {
            hex_len += 1;
            cursor += 1;
        }

        if matches!(hex_len, 3 | 4 | 6 | 8)
            && (cursor == bytes.len() || !bytes[cursor].is_ascii_hexdigit())
        {
            return true;
        }

        index += 1;
    }

    false
}

#[test]
fn form_field_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/form-field/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "FormField internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn form_field_uses_logic_state_model() {
    let mod_source = load_source("../../components/form-field/src/mod.rs");
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");

    for needle in ["struct FormFieldStateInput", "struct FormFieldState"] {
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
        "logic::normalize_error_message(error_message, is_invalid)",
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
    let source = load_source("../../components/form-field/src/view.rs");

    for needle in [
        "FormFieldIndicatorVariant::Switch",
        "FormFieldIndicatorVariant::Checkbox",
        "<Switch",
        "<Checkbox",
        "on_checked_change=on_selected_change",
        "checked=selected",
        "checked=Some(selected)",
        "on_change=Some(on_selected_change)",
    ] {
        assert!(
            source.contains(needle),
            "FormField should compose indicator controls with stable contracts (`{needle}`)."
        );
    }
}

#[test]
fn form_field_emits_baseline_style_state_data_attributes() {
    let source = load_source("../../components/form-field/src/view.rs");

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
    let source = load_source("../../components/form-field/src/styles.rs");

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
fn form_field_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals() {
    let styles = load_source("../../components/form-field/src/styles.rs");
    let theme_css = load_source("../ui-theme/src/css.rs");
    let check2 = load_source("../../components/form-field/check2.md");
    let local_semantics = load_source("../../components/form-field/test/semantics.rs");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-space-3xs, var(--ui-fallback-space-3xs))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity))",
        "var(--ui-button-size-l-font-size, var(--ui-fallback-button-size-l-font-size))",
        "var(--ui-button-size-l-line-height, var(--ui-fallback-button-size-l-line-height))",
    ] {
        assert!(
            styles.contains(required),
            "form-field styles should keep defensive variable fallback chain `{required}`."
        );
    }

    for forbidden in [
        ", 1px)", ", 2px)", ", 12px)", ", 15px)", ", 16px)", ", 22px)", ", 0.72)",
    ] {
        assert!(
            !styles.contains(forbidden),
            "form-field styles should not keep local hardcoded fallback terminal `{forbidden}`."
        );
    }

    assert!(
        !contains_hex_color_literal(&styles),
        "form-field styles should not include hardcoded hex color literals."
    );

    for required in [
        "--ui-fallback-space-3xs:",
        "--ui-fallback-disabled-opacity:",
        "--ui-fallback-button-size-l-font-size:",
        "--ui-fallback-button-size-l-line-height:",
    ] {
        assert!(
            theme_css.contains(required),
            "ui-theme SSOT fallback output should provide `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "fn form_field_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals() {",
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "components/form-field/test/semantics.rs::form_field_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
        "components/form-field/test/form_field/semantics.rs::form_field_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
    ] {
        assert!(
            local_semantics.contains(required) || check2.contains(required),
            "form-field defensive-variable contract evidence should include `{required}`."
        );
    }
}

#[test]
fn form_field_cascade_layer_and_runtime_style_contract_is_enforced() {
    let module_source = load_source("../../components/form-field/src/mod.rs");
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");
    let css_entry_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let check2 = load_source("../../components/form-field/check2.md");
    let local_semantics = load_source("../../components/form-field/test/semantics.rs");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-form_field\")]",
        "out.push_str(crate::field_form::form_field::styles::CSS);",
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

    for source in [
        module_source.as_str(),
        logic_source.as_str(),
        view_source.as_str(),
    ] {
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
            "style:width=",
            "style:height=",
        ] {
            assert!(
                !source.contains(forbidden),
                "form-field runtime should not include plain inline style token `{forbidden}`."
            );
        }
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "fn form_field_cascade_layer_and_runtime_style_contract_is_enforced() {",
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "components/form-field/test/semantics.rs::form_field_cascade_layer_and_runtime_style_contract_is_enforced",
        "components/form-field/test/form_field/semantics.rs::form_field_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            local_semantics.contains(required) || check2.contains(required),
            "form-field cascade-layer contract evidence should include `{required}`."
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
        "is_selected=Some(marketing.into())",
        "on_selected_change=Some(on_marketing_selected_change)",
        "id_base=\"docs-form-field-marketing\".to_string()",
        "label=\"Subscribe to product updates\".to_string()",
        "description=\"Receive release notes and occasional best-practice tips.\".to_string()",
        "indicator_placement=FormFieldIndicatorPlacement::Start",
        "title=\"Checkbox Indicator + Quiet + Invalid/Disabled\"",
        "is_selected=Some(tos.into())",
        "on_selected_change=Some(on_tos_selected_change)",
        "id_base=\"docs-form-field-tos\".to_string()",
        "indicator_variant=FormFieldIndicatorVariant::Checkbox",
        "indicator_placement=FormFieldIndicatorPlacement::End",
        "tone=FormFieldTone::Quiet",
        "is_invalid=true",
        "error_message=\"Please accept terms to continue.\".to_string()",
        "class_name=\"docs-form-field-custom\".to_string()",
        "id_base=\"docs-form-field-read-only\".to_string()",
        "default_selected=Some(true)",
        "is_disabled=true",
        "aria_label=\"Maintenance alerts (read only)\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "form_field docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn form_field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let check2_source = load_source("../../components/form-field/check2.md");
    let e2e_source = load_source("../../e2e/tests/docs_app_form_field_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "<Playground title=\"Hello World（默认路径）\" code_signal=hello_code>",
        "<Playground title=\"Switch Indicator + Description\" code_signal=code>",
        "<Playground title=\"Checkbox Indicator + Quiet + Invalid/Disabled\" code_signal=states_code>",
        "<Playground title=\"Controlled vs Default (Comparison)\" code_signal=comparison_code>",
        "data-slot=\"form-field-state-matrix-note\"",
        "data-slot=\"form-field-controlled-uncontrolled-note\"",
        "data-slot=\"form-field-streaming-policy\"",
        "data-slot=\"form-field-streaming-modes\"",
        "data-slot=\"form-field-copy-ready\"",
        "data-slot=\"form-field-source-paths\"",
        "data-slot=\"form-field-source-prerequisites\"",
        "Streaming Optional; fallback=snapshot.",
        "Snapshot mode renders verified full output for form-field semantics.",
        "Copy-ready snippets prepend imports automatically: use leptos::prelude::*; use ui::*.",
    ] {
        assert!(
            docs_source.contains(needle),
            "form-field docs should keep docs-product marker `{needle}`.",
        );
    }

    for needle in [
        "docs-app form-field playground source is copy-paste ready",
        "data-copyable",
        "use leptos::prelude::*;",
        "use ui::*;",
        "<FormField",
    ] {
        assert!(
            e2e_source.contains(needle),
            "form-field e2e docs contract should keep copy-ready marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "components/form-field/test/semantics.rs::form_field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "components/form-field/test/form_field/semantics.rs::form_field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "e2e/tests/docs_app_form_field_contract.spec.mjs::docs-app form-field playground source is copy-paste ready",
        "bash scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "form_field/check2.md should record docs-product evidence `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        script_source.contains(script_needle),
        "dx gate script should include `{script_needle}`.",
    );
}

#[test]
fn form_field_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_source("../../components/form-field/check2.md");

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 docs-sync/state-matrix section should include `{needle}`.",
        );
    }
}

#[test]
fn form_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let logic_source = load_source("../../components/form-field/src/logic.rs");

    for needle in [
        "<Playground title=\"Hello World（默认路径）\" code_signal=hello_code>",
        "<Playground title=\"Switch Indicator + Description\" code_signal=code>",
        "<Playground title=\"Checkbox Indicator + Quiet + Invalid/Disabled\" code_signal=states_code>",
        "<Playground title=\"Controlled vs Default (Comparison)\" code_signal=comparison_code>",
        "data-slot=\"form-field-state-matrix-note\"",
        "data-slot=\"form-field-controlled-uncontrolled-note\"",
        "is_selected=Some(marketing.into())",
        "on_selected_change=Some(on_marketing_selected_change)",
        "is_selected=Some(tos.into())",
        "on_selected_change=Some(on_tos_selected_change)",
        "default_selected=Some(true)",
        "is_disabled=true",
        "is_invalid=true",
        "tone=FormFieldTone::Quiet",
        "indicator_variant=FormFieldIndicatorVariant::Checkbox",
        "indicator_placement=FormFieldIndicatorPlacement::End",
    ] {
        assert!(
            docs_source.contains(needle),
            "form-field docs matrix/examples should include `{needle}`.",
        );
    }

    for needle in [
        "pub const DEFAULT_SELECTED: bool = ui_state_primitives::radio::DEFAULT_CHECKED;",
        "default_selected: input.default_selected.unwrap_or(DEFAULT_SELECTED),",
        "pub struct FormFieldSelectedAxisInput {",
        "pub struct FormFieldSelectedAxisState {",
        "pub fn normalize_selected_axis(input: FormFieldSelectedAxisInput) -> FormFieldSelectedAxisState",
    ] {
        assert!(
            logic_source.contains(needle),
            "form-field logic should keep API/default normalization marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "dx script should include docs-sync/state-matrix gate `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let check2_source = load_source("../../components/form-field/check2.md");

    assert!(
        check2_source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "form-field check2 should mark docs-sync/state-matrix item complete.",
    );

    for needle in [
        "components/form-field/test/semantics.rs::form_field_check2_documents_docs_sync_and_state_matrix_rules",
        "components/form-field/test/semantics.rs::form_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "components/form-field/test/form_field/semantics.rs::form_field_check2_documents_docs_sync_and_state_matrix_rules",
        "components/form-field/test/form_field/semantics.rs::form_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "components/form-field/test/form_field/semantics.rs::form_field_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 docs-sync/state-matrix section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("../../components/form-field/check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 documentation-as-product section should include `{needle}`.",
        );
    }
}

#[test]
fn form_field_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = load_source("../../components/form-field/src/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "# FormField",
        "## Hello World",
        "## 常见用法",
        "## 先用起来，再进阶",
        "默认路径：`<FormField label=... />`",
        "进阶控制：按需启用 `is_selected + default_selected + on_selected_change`。",
    ] {
        assert!(
            readme_source.contains(needle),
            "form-field README should include beginner marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn form_field() -> AnyView",
        "title=\"FormField\"",
        "slug=\"form-field\"",
        "title=\"Hello World（默认路径）\"",
        "title=\"Switch Indicator + Description\"",
        "title=\"Checkbox Indicator + Quiet + Invalid/Disabled\"",
        "title=\"Controlled vs Default (Comparison)\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "form-field docs entry should include `{needle}`.",
        );
    }

    let readme_hello = readme_source
        .find("## Hello World")
        .expect("form-field README should include Hello World section");
    let readme_common = readme_source
        .find("## 常见用法")
        .expect("form-field README should include common-usage section");
    let readme_progressive = readme_source
        .find("## 先用起来，再进阶")
        .expect("form-field README should include beginner-to-advanced section");
    let readme_architecture = readme_source
        .find("## Architecture Layers")
        .expect("form-field README should include architecture section");
    assert!(
        readme_hello < readme_common
            && readme_common < readme_progressive
            && readme_progressive < readme_architecture,
        "form-field README should keep default path before architecture-heavy content.",
    );

    let docs_hello = docs_source
        .find("title=\"Hello World（默认路径）\"")
        .expect("form-field docs should include Hello World playground");
    let docs_common = docs_source
        .find("title=\"Switch Indicator + Description\"")
        .expect("form-field docs should include common-usage playground");
    let docs_advanced = docs_source
        .find("title=\"Checkbox Indicator + Quiet + Invalid/Disabled\"")
        .expect("form-field docs should include advanced-state playground");
    let docs_controlled = docs_source
        .find("title=\"Controlled vs Default (Comparison)\"")
        .expect("form-field docs should include controlled/uncontrolled playground");
    assert!(
        docs_hello < docs_common && docs_common < docs_advanced && docs_advanced < docs_controlled,
        "form-field docs should keep beginner-first order before controlled comparison.",
    );
}

#[test]
fn form_field_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_documentation_as_product_rules",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "dx script should include documentation-as-product gate `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_marks_documentation_as_product_contract_complete() {
    let check2_source = load_source("../../components/form-field/check2.md");

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "form-field check2 should mark documentation-as-product item complete.",
    );

    for needle in [
        "components/form-field/src/README.md",
        "apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::form_field",
        "form_field_check2_documents_documentation_as_product_rules",
        "form_field_documentation_entry_exists_with_beginner_first_progression",
        "form_field_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 documentation-as-product section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("../../components/form-field/check2.md");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 interactive-playground section should include `{needle}`.",
        );
    }
}

#[test]
fn form_field_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "title=\"FormField Workbench (Display + Config + Code + CSS Test)\"",
        "test_css_source=interactive_test_css_source",
        "test_config_signal=interactive_actual_config",
        "controls=move || view!",
        "data-slot=\"form-field-workbench-controls\"",
        "data-slot=\"form-field-workbench-compare\"",
        "Switch checked=interactive_selected set_checked=set_interactive_selected",
        "Switch checked=interactive_disabled set_checked=set_interactive_disabled",
        "Switch checked=interactive_invalid set_checked=set_interactive_invalid",
        "let (interactive_selected, set_interactive_selected) = signal(true);",
        "let (interactive_disabled, set_interactive_disabled) = signal(false);",
        "let (interactive_invalid, set_interactive_invalid) = signal(false);",
        "FormFieldActualConfig {",
    ] {
        assert!(
            docs_source.contains(needle),
            "form-field docs should provide interactive marker `{needle}`.",
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
            "docs-app playground should keep interactive preview marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_form_field_contract.spec.mjs");

    for needle in [
        "docs-app form-field key flow is repeatable with semantic breakpoints",
        "await page.goto(\"/#/components/form-field\");",
        "body:not(:has(#boot))",
        "await tosCheckbox.focus();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(tos).toHaveAttribute(\"data-state\", \"selected-invalid\");",
        "await page.reload();",
        "await expect(reloadedTos).toHaveAttribute(\"data-state\", \"invalid\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "form-field interactive playground should keep repeatable e2e marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(needle),
            "dx script should include interactive-playground gate `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_marks_interactive_playground_item_complete() {
    let check2_source = load_source("../../components/form-field/check2.md");

    assert!(
        check2_source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "form-field check2 should mark interactive-playground item complete.",
    );

    for needle in [
        "FormField Workbench (Display + Config + Code + CSS Test)",
        "forms_groups_extra.rs::form_field",
        "docs_app_form_field_contract.spec.mjs",
        "form_field_check2_documents_interactive_playground_rules",
        "form_field_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "form_field_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "form_field_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 interactive-playground section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("../../components/form-field/check2.md");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 source-first section should include `{needle}`.",
        );
    }
}

#[test]
fn form_field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_form_field_contract.spec.mjs");

    for needle in [
        "data-slot=\"form-field-copy-ready\"",
        "data-slot=\"form-field-source-paths\"",
        "data-slot=\"form-field-source-prerequisites\"",
        "Copy-ready snippets prepend imports automatically: use leptos::prelude::*; use ui::*.",
        "Source paths: components/form-field/src/mod.rs, components/form-field/src/logic.rs, components/form-field/src/view.rs, components/form-field/src/styles.rs.",
        "Feature prerequisites: component-form_field (inject-css optional for runtime style injection).",
        "title=\"Switch Indicator + Description\" code_signal=code",
        "title=\"FormField Workbench (Display + Config + Code + CSS Test)\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "form-field source-first docs should contain `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "code_imports",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should contain `{needle}`.",
        );
    }

    for needle in [
        "docs-app form-field playground source is copy-paste ready",
        "data-copyable",
        "use leptos::prelude::*;",
        "use ui::*;",
        "data-slot=\"form-field-source-paths\"",
        "data-slot=\"form-field-source-prerequisites\"",
        "toContainText(\"components/form-field/src/mod.rs\")",
        "toContainText(\"component-form_field\")",
        "toContainText(\"inject-css\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "form-field e2e source-first contract should contain `{needle}`.",
        );
    }
}

#[test]
fn form_field_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: form-field source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "dx script should include source-first copy-paste-ready gate `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_source("../../components/form-field/check2.md");

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "form-field check2 should mark source-first copy-paste-ready item complete.",
    );

    for needle in [
        "forms_groups_extra.rs::form_field",
        "docs_app_form_field_contract.spec.mjs::docs-app form-field playground source is copy-paste ready",
        "form_field_check2_documents_source_first_copy_paste_ready_rules",
        "form_field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "form_field_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 source-first section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_source("../../components/form-field/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 heroui-benchmark docs-sync section should include `{needle}`.",
        );
    }
}

#[test]
fn form_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let readme_source = load_source("../../components/form-field/src/README.md");

    for needle in [
        "### FormField 同步记录（2026-02-20）",
        "参数模型同步：`FormField` 参数主轴保持 `is_selected/default_selected/on_selected_change`",
        "component_doc!(\"FormField\", \"form-field\", \"Forms\", forms_groups_extra::form_field)",
        "`apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::form_field()`",
        "`components/form-field/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui strategy doc should include form-field synchronization marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"FormField\"",
        "\"form-field\"",
        "forms_groups_extra::form_field",
    ] {
        assert!(
            pages_source.contains(needle),
            "component docs index should expose form-field entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn form_field() -> AnyView {",
        "title=\"FormField\"",
        "slug=\"form-field\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app form-field page should stay indexable via marker `{needle}`.",
        );
    }

    for needle in ["# FormField", "## docs-app 入口"] {
        assert!(
            readme_source.contains(needle),
            "form-field README should remain an equivalent component doc entry via `{needle}`.",
        );
    }
}

#[test]
fn form_field_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: form-field heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce heroui-benchmark docs-sync contract `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("../../components/form-field/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "form_field_check2_documents_heroui_benchmark_docs_sync_rules",
        "form_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "form_field_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 should keep heroui-benchmark docs-sync evidence marker `{needle}`.",
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
    let source = load_source("../../components/form-field/src/view.rs");

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

    assert!(
        lib_source.contains("pub mod field_form {"),
        "ui lib boundary should expose `field_form` domain module."
    );
    assert!(
        lib_source.contains("pub use field_form::form_field::{"),
        "ui lib boundary should re-export FormField from field_form domain."
    );
    assert!(
        lib_source.contains("#[cfg(feature = \"component-form_field\")]")
            && lib_source.contains("pub mod form_field {")
            && lib_source.contains("pub use crate::field_form_form_field::*;"),
        "inline field_form module should feature-gate `form_field`."
    );

    for needle in [
        "#[cfg(feature = \"component-form_field\")]",
        "out.push_str(crate::field_form::form_field::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui css boundary should include `{needle}` for FormField feature gating."
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

    for forbidden in [
        "getByText(",
        "locator(\"div > div >",
        "nth-child(",
        "waitForTimeout(",
        "setTimeout(",
    ] {
        assert!(
            !source.contains(forbidden),
            "form-field E2E selector contract should avoid brittle selector/wait token `{forbidden}`.",
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

    for needle in [
        "await marketingSwitch.click();",
        "await expect(marketing).toHaveAttribute(\"data-state\", \"unselected\");",
        "await page.keyboard.press(\"Enter\");",
        "await expect(tos).toHaveAttribute(\"data-state\", \"selected-invalid\");",
        "await expect(tos).toHaveAttribute(\"data-state\", \"invalid\");",
    ] {
        assert!(
            source.contains(needle),
            "form-field E2E flow should keep semantic ready/settled breakpoint `{needle}`.",
        );
    }
}

#[test]
fn form_field_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script_source = load_source("../../components/form-field/scripts/check-ui-e2e-form-field.sh");

    for needle in [
        "echo \"[e2e-form-field] contract: checklist e2e-selector/stable-wait governance\"",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_e2e_selector_and_stable_wait_rules",
        "echo \"[e2e-form-field] contract: semantic selectors + settled waits\"",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_e2e_contract_uses_semantic_selectors_and_settled_waits",
        "echo \"[e2e-form-field] contract: checklist repeatable-key-flow governance\"",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_e2e_repeatable_key_flow_rules",
        "echo \"[e2e-form-field] contract: repeatable key flow with semantic ready/settled breakpoints\"",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "echo \"[e2e-form-field] contract: repeatable key flow + copy-ready source coverage\"",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_e2e_contract_covers_repeatable_key_flow_and_copy_ready_source",
    ] {
        assert!(
            script_source.contains(needle),
            "form-field e2e check script should include `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("../../components/form-field/check2.md");

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        "components/form-field/test/semantics.rs::form_field_e2e_selector_stability_prefers_semantic_markers_and_settled_waits",
        "components/form-field/test/form_field/semantics.rs::form_field_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/form-field/test/form_field/semantics.rs::form_field_e2e_contract_uses_semantic_selectors_and_settled_waits",
        "components/form-field/test/form_field/semantics.rs::form_field_e2e_contract_covers_repeatable_key_flow_and_copy_ready_source",
        "components/form-field/scripts/check-ui-e2e-form-field.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 e2e-selector/stable-wait section should reference `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_marks_e2e_selector_stability_item_complete() {
    let check2_source = load_source("../../components/form-field/check2.md");

    assert!(
        check2_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "form-field check2 should mark e2e selector stability item complete.",
    );

    for needle in [
        "form_field_check2_documents_e2e_selector_and_stable_wait_rules",
        "form_field_e2e_contract_uses_semantic_selectors_and_settled_waits",
        "form_field_e2e_contract_covers_repeatable_key_flow_and_copy_ready_source",
        "components/form-field/scripts/check-ui-e2e-form-field.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 should retain e2e selector stability marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = load_source("../../components/form-field/check2.md");

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 repeatable-key-flow section should include `{needle}`.",
        );
    }
}

#[test]
fn form_field_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let source = load_source("../../e2e/tests/docs_app_form_field_contract.spec.mjs");

    for needle in [
        "docs-app form-field key flow is repeatable with semantic breakpoints",
        "body:not(:has(#boot))",
        "#docs-form-field-tos[data-slot=\"form-field\"]",
        "const tosCheckbox = tos.locator('[data-slot=\"checkbox\"]').first();",
        "await tosCheckbox.focus();",
        "await expect(tosCheckbox).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(tos).toHaveAttribute(\"data-state\", \"selected-invalid\");",
        "await expect(tos).toHaveAttribute(\"data-state\", \"invalid\");",
        "await page.reload();",
        "await expect(reloadedTos).toHaveAttribute(\"data-state\", \"invalid\");",
    ] {
        assert!(
            source.contains(needle),
            "form-field e2e repeatable key-flow contract should include `{needle}`.",
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !source.contains(forbidden),
            "form-field repeatable key flow should avoid non-semantic/flaky token `{forbidden}`.",
        );
    }
}

#[test]
fn form_field_check2_marks_e2e_repeatable_key_flow_item_complete() {
    let check2_source = load_source("../../components/form-field/check2.md");

    assert!(
        check2_source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "form-field check2 should mark repeatable-key-flow item complete.",
    );

    for needle in [
        "docs_app_form_field_contract.spec.mjs",
        "docs-app form-field key flow is repeatable with semantic breakpoints",
        "form_field_check2_documents_e2e_repeatable_key_flow_rules",
        "form_field_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "components/form-field/scripts/check-ui-e2e-form-field.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 repeatable-key-flow section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_performance_governance_budget_is_defined_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("../../components/form-field/check2.md");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let view_source = load_source("../../components/form-field/src/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"form-field\" => UiPerfBudget {",
        "max_mount_ms: 26.0,",
        "max_update_ms: Some(8.0),",
        "max_heap_kb: Some(384.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep form-field perf budget token `{needle}`."
        );
    }

    for needle in [
        "component_doc!(",
        "\"FormField\"",
        "\"form-field\"",
        "forms_groups_extra::form_field",
    ] {
        assert!(
            pages_source.contains(needle),
            "FormField docs page should remain in coverage traversal via `{needle}`.",
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose performance regression marker `{needle}`."
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
            "docs coverage e2e should enforce repeatable perf regression guard `{needle}`.",
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace-based perf attribution token `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`.",
        );
    }

    for needle in [
        "性能治理：关键路径有预算（首次渲染/更新耗时/内存）",
        "关键交互组件需定义最小预算项（首渲染、关键更新、内存/分配趋势）",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
        "form_field_performance_governance_budget_is_defined_traceable_and_blocking",
    ] {
        assert!(
            check2_source.contains(needle),
            "FormField checklist should keep perf governance baseline/follow-up token `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().state_attr",
        "data-selected-control-mode=selected_control_mode_attr",
        "data-default-selected-source=default_selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "FormField view should expose attribution marker `{needle}` for perf triage.",
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_performance_governance_budget_is_defined_traceable_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`.",
    );
}

#[test]
fn form_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view_source = load_source("../../components/form-field/src/view.rs");
    let local_semantics_source = load_source("../../components/form-field/test/semantics.rs");
    let semantics_source = load_source("tests/form_field/semantics.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_form_field_contract.spec.mjs");
    let perf_script_source = load_source("../../scripts/check-ui-performance.sh");

    for needle in [
        "role=\"group\"",
        "aria-label=move || control_aria_label.get_value()",
        "aria-describedby=move || describedby.get()",
        "aria-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "aria-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-state=move || state.get().state_attr",
        "data-selected-control-mode=selected_control_mode_attr",
        "data-default-selected-source=default_selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "form-field semantic-priority contract should keep marker `{needle}`.",
        );
    }

    for needle in [
        "fn form_field_semantic_contract_tests_cover_branch_matrix_without_snapshot_dependency()",
        "fn form_field_state_markers_are_observable_queryable_and_enumerable()",
        "fn form_field_a11y_i18n_contract_is_mounted_without_hardcoded_view_text()",
        "for forbidden in [\"toMatchSnapshot\", \"assert_snapshot!\", \"snapshot_diff\"]",
        "semantic tests should not depend on visual snapshot assertion",
    ] {
        assert!(
            local_semantics_source.contains(needle),
            "form-field local semantics suite should keep semantic-priority marker `{needle}`.",
        );
    }

    for needle in [
        "page.keyboard.press(\"Enter\")",
        "toHaveAttribute(\"data-state\", \"unselected\")",
        "toHaveAttribute(\"data-disabled\", \"true\")",
        "[data-component=\"form-field\"]",
        "data-copyable",
    ] {
        assert!(
            semantics_source.contains(needle) || e2e_source.contains(needle),
            "form-field semantic-priority path should keep marker `{needle}`.",
        );
    }

    for forbidden_snapshot in ["toHaveScreenshot(", "toMatchSnapshot(", "screenshot("] {
        assert!(
            !e2e_source.contains(forbidden_snapshot),
            "form-field e2e should avoid snapshot-only assertion `{forbidden_snapshot}`.",
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance script should include semantic-priority gate `{script_needle}`.",
    );
}

#[test]
fn form_field_performance_script_covers_semantic_test_priority_contract() {
    let script_source = load_source("../../scripts/check-ui-performance.sh");

    for needle in [
        "echo \"[perf] contract: form-field semantic test priority\"",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "performance script should include form-field semantic-priority marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_view_macro_complexity_is_controlled_by_semantic_subview_split() {
    let view_source = load_source("../../components/form-field/src/view.rs");
    let check2_source = load_source("../../components/form-field/check2.md");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    for needle in [
        "fn render_indicator_view(",
        "fn render_content_view(",
        "{render_indicator_view(",
        "{render_content_view(",
        "data-slot=\"form-field\"",
        "data-slot=\"form-field-content\"",
        "data-slot=\"form-field-indicator\"",
    ] {
        assert!(
            view_source.contains(needle),
            "FormField view should keep semantic split marker `{needle}` to bound macro expansion."
        );
    }

    let switch_branch_count = view_source
        .matches("FormFieldIndicatorVariant::Switch =>")
        .count();
    assert_eq!(
        switch_branch_count, 1,
        "FormField indicator switch branch should be centralized once; found {switch_branch_count}."
    );
    let checkbox_branch_count = view_source
        .matches("FormFieldIndicatorVariant::Checkbox =>")
        .count();
    assert_eq!(
        checkbox_branch_count, 1,
        "FormField indicator checkbox branch should be centralized once; found {checkbox_branch_count}."
    );

    for needle in [
        "`view!` 宏复杂度受控",
        "form_field_view_macro_complexity_is_bounded_via_semantic_subview_split",
        "form_field_view_macro_complexity_is_controlled_by_semantic_subview_split",
    ] {
        assert!(
            check2_source.contains(needle),
            "FormField checklist should keep view-macro complexity governance marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_view_macro_complexity_is_controlled_by_semantic_subview_split";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn form_field_view_functional_split_prefers_plain_functions_over_extra_local_components() {
    let view_source = load_source("../../components/form-field/src/view.rs");
    let check2_source = load_source("../../components/form-field/check2.md");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    for needle in [
        "fn render_indicator_view(",
        "fn render_content_view(",
        "{render_indicator_view(",
        "{render_content_view(",
        "#[component]\nfn Switch(",
        "#[component]\npub fn FormField(",
    ] {
        assert!(
            view_source.contains(needle),
            "FormField view should keep function-first split marker `{needle}`.",
        );
    }

    let component_attr_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_attr_count, 2,
        "FormField view should keep exactly 2 component declarations (FormField + Switch); found {component_attr_count}."
    );

    for needle in [
        "函数式拆分优先",
        "form_field_view_functional_split_prefers_plain_helpers_without_component_noise",
        "form_field_view_functional_split_prefers_plain_functions_over_extra_local_components",
    ] {
        assert!(
            check2_source.contains(needle),
            "FormField checklist should keep function-first governance marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_view_functional_split_prefers_plain_functions_over_extra_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn form_field_static_fragments_are_constantized_or_absent_for_simple_layout() {
    let view_source = load_source("../../components/form-field/src/view.rs");
    let check2_source = load_source("../../components/form-field/check2.md");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    for forbidden in [
        "inner_html=",
        "include_str!(",
        "markdown_to_html",
        "svg path d=\"",
        "</footer>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FormField view should avoid heavy static fragment token `{forbidden}`.",
        );
    }

    assert_eq!(
        view_source.matches("ui-switch__track").count(),
        1,
        "FormField switch track static fragment should stay single-source."
    );
    assert_eq!(
        view_source.matches("ui-switch__thumb").count(),
        1,
        "FormField switch thumb static fragment should stay single-source."
    );

    for needle in [
        "data-slot=\"switch-track\"",
        "data-slot=\"switch-thumb\"",
        "role=aria.attrs.role",
        "aria-checked=move || aria.attrs.aria_checked.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "FormField static micro fragment should keep semantic/a11y marker `{needle}`.",
        );
    }

    for needle in [
        "静态片段常量化",
        "static fragments are constantized or absent for simple layout",
        "form_field_static_fragments_are_constantized_or_absent_for_simple_layout",
    ] {
        assert!(
            check2_source.contains(needle),
            "FormField checklist should keep static fragment governance marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_static_fragments_are_constantized_or_absent_for_simple_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn form_field_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let mod_source = load_source("../../components/form-field/src/mod.rs");
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let styles_source = load_source("../../components/form-field/src/styles.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let check2_source = load_source("../../components/form-field/check2.md");
    let script_source = load_source("../../scripts/check-ui-inner-html.sh");

    for source in [
        mod_source.as_str(),
        logic_source.as_str(),
        styles_source.as_str(),
        view_source.as_str(),
        docs_source.as_str(),
    ] {
        let normalized = source.to_ascii_lowercase();
        for forbidden in [
            "inner_html=",
            "set_inner_html(",
            "dangerously_set_inner_html",
            "markdown_to_html(",
            "format!(\"<",
            "<script",
            "javascript:",
        ] {
            assert!(
                !normalized.contains(forbidden),
                "FormField component/docs source should forbid html injection marker `{forbidden}`.",
            );
        }
    }

    for required in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "该项按“零注入面”通过",
        "components/form-field/test/semantics.rs::form_field_inner_html_usage_is_forbidden_in_component_and_docs_examples",
        "components/form-field/test/form_field/semantics.rs::form_field_inner_html_usage_is_forbidden_in_component_and_docs_examples",
    ] {
        assert!(
            check2_source.contains(required),
            "FormField checklist should keep inner_html safety governance marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(script_needle),
        "inner-html check script should include `{script_needle}`.",
    );
}

#[test]
fn form_field_wasm_debug_contract_is_na_and_feature_isolated() {
    let form_field_cargo = load_source("../../components/form-field/Cargo.toml");
    let ui_components_cargo = load_source("Cargo.toml");
    let ui_components_lib = load_source("src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let mod_source = load_source("../../components/form-field/src/mod.rs");
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let styles_source = load_source("../../components/form-field/src/styles.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");
    let check2_source = load_source("../../components/form-field/check2.md");
    let script_source = load_source("../../scripts/check-ui-wasm-debug.sh");

    for needle in ["[features]", "default = []"] {
        assert!(
            form_field_cargo.contains(needle),
            "form-field crate feature boundary should include `{needle}`.",
        );
    }

    for forbidden in ["wasm-debug", "wasm_debug", "dep:tracing"] {
        assert!(
            !form_field_cargo.contains(forbidden),
            "form-field crate should not leak wasm-debug feature `{forbidden}`.",
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui should keep shared wasm-debug feature marker `{needle}`.",
        );
    }

    for forbidden in [
        "form-field-wasm-debug =",
        "form_field_wasm_debug =",
        "component-form_field\", \"dep:tracing",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "ui feature graph should not leak form-field-specific debug toggle `{forbidden}`.",
        );
    }

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui root should keep shared wasm-debug isolation marker `{needle}`.",
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(needle),
            "docs app should keep dev-only debug overlay entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "events.into_iter().rev().take(40)",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            trace_source.contains(needle) || debug_overlay_source.contains(needle),
            "global trace/debug-overlay contract should keep marker `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || state.get().state_attr",
        "data-selected-control-mode=selected_control_mode_attr",
        "data-default-selected-source=default_selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "form-field should keep state/source marker `{needle}` for debug traceability.",
        );
    }

    for source in [
        mod_source.as_str(),
        logic_source.as_str(),
        styles_source.as_str(),
        view_source.as_str(),
    ] {
        for forbidden in [
            "use_ui_trace(",
            "provide_ui_trace(",
            "trace.emit(",
            "debug_overlay",
            "request_replay",
            "replay",
            "trace_id",
            "wasm_debug_proxy!",
            "observability::",
            "#[prop(optional)] debug",
        ] {
            assert!(
                !source.contains(forbidden),
                "form-field runtime/public contract should not leak wasm-debug internals `{forbidden}`.",
            );
        }
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_wasm_debug_contract_is_na_and_feature_isolated";
    assert!(
        script_source.contains(script_needle),
        "wasm-debug check script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "本组件判定：N/A（组件级不自建 wasm 调试/回放管线）",
        "components/form-field/test/semantics.rs::form_field_wasm_debug_contract_is_na_and_feature_isolated",
        "components/form-field/test/form_field/semantics.rs::form_field_wasm_debug_contract_is_na_and_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field checklist should keep wasm-debug governance marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let check2_source = load_source("../../components/form-field/check2.md");
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "data-playground-scope=scope_id.clone()",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "\"Restore original CSS\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground should keep DX hot-reload/isolated-canvas marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn form_field() -> AnyView",
        "let (marketing, set_marketing) = signal(true);",
        "let (tos, set_tos) = signal(false);",
        "let on_marketing_selected_change = Callback::new(move |next| set_marketing.set(next));",
        "let on_tos_selected_change = Callback::new(move |next| set_tos.set(next));",
        "<Playground title=\"Hello World（默认路径）\" code_signal=hello_code>",
        "<Playground title=\"Switch Indicator + Description\" code_signal=code>",
        "<Playground title=\"Checkbox Indicator + Quiet + Invalid/Disabled\" code_signal=states_code>",
        "\"marketing: \" {move || marketing.get()}",
        "\"tos: \" {move || tos.get()}",
    ] {
        assert!(
            docs_source.contains(needle),
            "form-field docs should keep DX workbench/context marker `{needle}`.",
        );
    }

    for forbidden in [
        "FORM_FIELD_WORKBENCH_STORAGE_KEY",
        "load_form_field_workbench_state(",
        "save_form_field_workbench_state(",
        "clear_form_field_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "form-field keeps optional persisted state as N/A in current scope; `{forbidden}` should remain absent.",
        );
    }

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "components/form-field/test/semantics.rs::form_field_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
        "components/form-field/test/form_field/semantics.rs::form_field_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field checklist should keep DX governance marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na";
    assert!(
        script_source.contains(script_needle),
        "dx check script should include `{script_needle}`.",
    );
}

#[test]
fn form_field_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries() {
    let mod_source = load_source("../../components/form-field/src/mod.rs");
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");
    let styles_source = load_source("../../components/form-field/src/styles.rs");
    let protocol_source = load_source("../../components/form-field/src/protocol.rs");
    let protocol_test_source = load_source("../../components/form-field/test/protocol.rs");
    let form_field_cargo = load_source("../../components/form-field/Cargo.toml");
    let ui_components_cargo = load_source("Cargo.toml");
    let button_view_source = load_source("../../components/button/src/view.rs");
    let check2_source = load_source("../../components/form-field/check2.md");
    let script_source = load_source("../../scripts/check-ui-engineering.sh");

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "pub enum FormFieldComponentSchemaVersion",
        "#[serde(rename_all = \"snake_case\")]",
        "pub struct FormFieldComponentSpec",
        "pub schema_version: FormFieldComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "form-field protocol should keep structured serde marker `{needle}`.",
        );
    }

    for needle in [
        "fn protocol_types_implement_serde_contract()",
        "T: Serialize + DeserializeOwned",
    ] {
        assert!(
            protocol_test_source.contains(needle),
            "form-field protocol tests should keep serde regression marker `{needle}`.",
        );
    }

    assert!(
        button_view_source.contains("target: \"ui::button::state_change\""),
        "engineering baseline should keep canonical tracing target `ui::button::state_change`.",
    );
    assert!(
        ui_components_cargo.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "ui feature surface should keep shared tracing/debug baseline marker.",
    );

    for source in [
        mod_source.as_str(),
        logic_source.as_str(),
        view_source.as_str(),
        styles_source.as_str(),
        protocol_source.as_str(),
    ] {
        for forbidden in [
            "tracing::span!(",
            "tracing::event!(",
            "#[tracing::instrument]",
            "target: \"ui::form_field::",
            "const FORM_FIELD_TRACE_TARGET",
            "tokio",
            "tokio::",
            "async_std",
            "async_std::",
            "async-std",
            "smol::",
            "runtime::Handle",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "form-field engineering contract should avoid tracing/runtime leak marker `{forbidden}`.",
            );
        }
    }

    for forbidden in ["tokio", "async-std", "async_std", "smol", "runtime::Handle"] {
        assert!(
            !form_field_cargo.contains(forbidden),
            "form-field Cargo.toml should not leak runtime binding `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries";
    assert!(
        script_source.contains(script_needle),
        "engineering check script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
        "components/form-field/test/semantics.rs::form_field_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
        "components/form-field/test/form_field/semantics.rs::form_field_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field checklist should keep engineering governance marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let check2_source = load_source("../../components/form-field/check2.md");
    let script_source = load_source("../../scripts/check-ui-engineering.sh");
    let protocol_source = load_source("../../components/form-field/src/protocol.rs");
    let component_manifest = load_source("../../components/form-field/src/Component.toml");
    let rbi_source = load_source("../../components/form-field/src/form_field.rbi");

    for required in [
        "pub enum FormFieldComponentSchemaVersion",
        "V1",
        "pub struct FormFieldComponentSpec",
        "pub schema_version: FormFieldComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(required),
            "form-field protocol should keep stable v1 marker `{required}` in non-breaking scope.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "schema = \"ui.form_field.agent-contract.v1\"",
        "values = [\"v1\"]",
    ] {
        assert!(
            component_manifest.contains(required),
            "form-field Component.toml should keep v1 registration marker `{required}` in current scope.",
        );
    }

    for forbidden in [
        "V2",
        "migrate_v1_to_v2",
        "SchemaRegistry",
        "deprecation_window",
        "codemod",
        "schema_version = \"2\"",
        "agent-contract.v2",
    ] {
        assert!(
            !protocol_source.contains(forbidden)
                && !component_manifest.contains(forbidden)
                && !rbi_source.contains(forbidden),
            "without major breaking upgrade, form-field should not claim migration surface token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `FormField` 改动未引入跨大版本 API 破坏升级，组件协议与 Agent Contract 仍保持 `v1`（`components/form-field/src/protocol.rs` 的 `FormFieldComponentSchemaVersion::V1`、`components/form-field/src/Component.toml` 的 `schema_version = \"1\"` 与 `ui.form_field.agent-contract.v1`），因此不触发 Codemod/Schema Registry 弃用窗口与 `migrate_v1_to_v2` 迁移层要求。回归：`components/form-field/test/semantics.rs::form_field_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade`、`components/form-field/test/form_field/semantics.rs::form_field_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade`；门禁脚本：`scripts/check-ui-engineering.sh` 已接入对应 `cargo test` 目标。）",
        "form_field_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep codemod/registry migration marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards()
 {
    let module_source = load_source("../../components/form-field/src/mod.rs");
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");
    let styles_source = load_source("../../components/form-field/src/styles.rs");
    let motion_lib = load_source("../ui-motion/src/lib.rs");
    let motion_web = load_source("../ui-motion/src/web.rs");
    let check2_source = load_source("../../components/form-field/check2.md");
    let local_semantics = load_source("../../components/form-field/test/semantics.rs");
    let script_source = load_source("../../scripts/check-ui-platforms.sh");

    for source in [
        module_source.as_str(),
        logic_source.as_str(),
        view_source.as_str(),
        styles_source.as_str(),
    ] {
        for forbidden in [
            "mod motion;",
            "pub mod motion;",
            "pub use motion::",
            "attach_motion(",
            "stiffness",
            "damping",
            "MotionOptions",
        ] {
            assert!(
                !source.contains(forbidden),
                "form-field should not leak component-local motion contract token `{forbidden}`.",
            );
        }
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib.contains(required),
            "ui-motion should keep non-wasm no-op branch for N/A component motion via `{required}`.",
        );
    }

    for required in [
        "w.match_media(\"(prefers-reduced-motion: reduce)\")",
        "if prefers_reduced_motion() {",
        "return;",
    ] {
        assert!(
            motion_web.contains(required),
            "ui-motion wasm backend should keep reduced-motion guard `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards";
    assert!(
        script_source.contains(script_needle),
        "platform check script should include `{script_needle}`.",
    );

    for required in [
        "fn form_field_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards(",
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "本组件判定：N/A（`FormField` 无独立组件级动效状态轴，不定义 `src/motion.rs` 与 `attach_motion`）",
        "components/form-field/test/semantics.rs::form_field_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards",
        "components/form-field/test/form_field/semantics.rs::form_field_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards",
    ] {
        assert!(
            local_semantics.contains(required) || check2_source.contains(required),
            "form-field motion contractualization evidence should include `{required}`.",
        );
    }
}

#[test]
fn form_field_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let ui_components_lib = load_source("src/lib.rs");
    let ui_components_css = load_source("src/css.rs");
    let ui_components_root = load_source("src/root.rs");
    let active_highlight = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let check2_source = load_source("../../components/form-field/check2.md");
    let local_semantics = load_source("../../components/form-field/test/semantics.rs");
    let script_source = load_source("../../scripts/check-ui-entrypoints.sh");

    for required in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(any(",
        "feature = \"component-form_field\",",
        "pub mod field_form {",
        "pub use field_form::form_field::{",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib.rs should keep fixed entrypoint marker `{required}`."
        );
    }

    for forbidden in [
        "pub use web_sys",
        "pub use leptos::web_sys",
        "pub use wasm_bindgen",
    ] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui public API should not leak platform detail `{forbidden}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-form_field\")]",
        "pub mod form_field {",
        "pub use crate::field_form_form_field::*;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib.rs inline field_form module should keep form-field feature-gated entry `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-form_field\")]",
        "out.push_str(crate::field_form::form_field::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css.rs should keep fixed entrypoint marker `{required}`."
        );
    }

    for required in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root.contains(required),
            "UiRoot should keep centralized injection marker `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion {",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight shared primitive should keep marker `{required}`."
        );
    }

    for forbidden in ["FormField", "ui-form-field", "form_field"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight shared primitive should avoid form-field business token `{forbidden}`."
        );
    }

    for forbidden_path in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden_path),
            "ui src should not host duplicated headless primitive `{forbidden_path}`."
        );
    }

    for required_path in [
        "../ui-headless/src/controllable_state.rs",
        "../ui-headless/src/presence.rs",
        "../ui-headless/src/a11y.rs",
    ] {
        assert!(
            path_exists(required_path),
            "ui-headless should host shared primitive `{required_path}`."
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(script_needle),
        "entrypoints script should include `{script_needle}`."
    );

    for required in [
        "fn form_field_ui_components_fixed_entry_files_follow_layered_boundaries() {",
        "- [x] `ui` 固定入口文件落点正确。",
        "components/form-field/test/semantics.rs::form_field_ui_components_fixed_entry_files_follow_layered_boundaries",
        "components/form-field/test/form_field/semantics.rs::form_field_ui_components_fixed_entry_files_follow_layered_boundaries",
        "scripts/check-ui-entrypoints.sh",
    ] {
        assert!(
            local_semantics.contains(required) || check2_source.contains(required),
            "form-field fixed-entrypoint evidence should include `{required}`."
        );
    }
}

#[test]
fn form_field_component_directory_standard_files_follow_contract_and_na_paths() {
    let module_source = load_source("../../components/form-field/src/mod.rs");
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let styles_source = load_source("../../components/form-field/src/styles.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");
    let protocol_source = load_source("../../components/form-field/src/protocol.rs");
    let check2_source = load_source("../../components/form-field/check2.md");
    let local_semantics = load_source("../../components/form-field/test/semantics.rs");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    for required_path in [
        "../../components/form-field/src/mod.rs",
        "../../components/form-field/src/logic.rs",
        "../../components/form-field/src/styles.rs",
        "../../components/form-field/src/view.rs",
        "../../components/form-field/src/protocol.rs",
    ] {
        assert!(
            path_exists(required_path),
            "form-field component should include standard file `{required_path}`."
        );
    }

    for forbidden_path in [
        "../../components/form-field/src/render.rs",
        "../../components/form-field/src/motion.rs",
        "../../components/form-field/src/spec.rs",
    ] {
        assert!(
            !path_exists(forbidden_path),
            "form-field component should keep N/A path absent `{forbidden_path}`."
        );
    }

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::FormField;",
    ] {
        assert!(
            module_source.contains(required),
            "form-field mod.rs should keep minimal export boundary `{required}`."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;", "mod render;"] {
        assert!(
            !module_source.contains(forbidden),
            "form-field mod.rs should avoid over-export/render drift `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_selected_axis(",
        "pub fn resolve_state(input: FormFieldStateInput) -> FormFieldState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: FormFieldState)",
    ] {
        assert!(
            logic_source.contains(required),
            "form-field logic.rs should keep normalization/derivation marker `{required}`."
        );
    }

    for forbidden in ["view!", "data-slot=", "role=", "on:click="] {
        assert!(
            !logic_source.contains(forbidden),
            "form-field logic.rs should not include view/headless mount token `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
    ] {
        assert!(
            styles_source.contains(required),
            "form-field styles.rs should keep token-first static css marker `{required}`."
        );
    }

    for forbidden in ["#[component]", "view!", "style=\"top:"] {
        assert!(
            !styles_source.contains(forbidden),
            "form-field styles.rs should avoid runtime/view marker `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn FormField(",
        "let aria = use_switch(SwitchOptions {",
        "data-slot=\"form-field\"",
        "data-state=move || state.get().state_attr",
    ] {
        assert!(
            view_source.contains(required),
            "form-field view.rs should keep structure + headless mount marker `{required}`."
        );
    }

    for required in [
        "pub enum FormFieldComponentSchemaVersion",
        "pub struct FormFieldComponentSpec",
    ] {
        assert!(
            protocol_source.contains(required),
            "form-field protocol.rs should remain minimal schema file `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`."
    );

    for required in [
        "fn form_field_component_directory_standard_files_follow_contract_and_na_paths() {",
        "- [x] 组件目录标准文件落点正确。",
        "本组件判定：`src/motion.rs` N/A",
        "本组件判定：`src/spec.rs` N/A",
        "components/form-field/test/semantics.rs::form_field_component_directory_standard_files_follow_contract_and_na_paths",
        "components/form-field/test/form_field/semantics.rs::form_field_component_directory_standard_files_follow_contract_and_na_paths",
        "scripts/check-ui-component-files.sh",
    ] {
        assert!(
            local_semantics.contains(required) || check2_source.contains(required),
            "form-field standard file-layout evidence should include `{required}`."
        );
    }
}

#[test]
fn form_field_file_placement_discipline_is_strict_for_component_scope() {
    let module_source = load_source("../../components/form-field/src/mod.rs");
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let styles_source = load_source("../../components/form-field/src/styles.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");
    let check2_source = load_source("../../components/form-field/check2.md");
    let local_semantics = load_source("../../components/form-field/test/semantics.rs");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    for required_path in [
        "../../components/form-field/src/mod.rs",
        "../../components/form-field/src/logic.rs",
        "../../components/form-field/src/styles.rs",
        "../../components/form-field/src/view.rs",
    ] {
        assert!(
            path_exists(required_path),
            "form-field file-placement discipline requires `{required_path}`."
        );
    }

    let forbidden_path = "../../components/form-field/src/render.rs";
    assert!(
        !path_exists(forbidden_path),
        "form-field should not include forbidden file `{forbidden_path}`."
    );

    assert!(
        !path_exists("../../components/form-field/src/motion.rs"),
        "form-field keeps `src/motion.rs` as N/A without component-local motion axis."
    );
    assert!(
        !path_exists("../../components/form-field/src/spec.rs"),
        "form-field keeps `src/spec.rs` as N/A for simple component scope."
    );

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::FormField;",
    ] {
        assert!(
            module_source.contains(required),
            "form-field mod.rs should keep export boundary marker `{required}`."
        );
    }

    for forbidden in ["mod render;", "pub mod view;", "pub mod logic;"] {
        assert!(
            !module_source.contains(forbidden),
            "form-field mod.rs should avoid over-export/render drift `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_selected_axis(",
        "pub fn resolve_state(input: FormFieldStateInput) -> FormFieldState",
        "pub const CSS: &str = r#\"",
        "pub fn FormField(",
    ] {
        assert!(
            logic_source.contains(required)
                || styles_source.contains(required)
                || view_source.contains(required),
            "form-field file-placement marker missing `{required}`."
        );
    }

    for forbidden in ["view!", "#[component]", "on:click="] {
        assert!(
            !logic_source.contains(forbidden),
            "form-field logic.rs should remain pure for file-placement contract `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_file_placement_discipline_is_strict_for_component_scope";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`."
    );

    for required in [
        "fn form_field_file_placement_discipline_is_strict_for_component_scope() {",
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "本组件判定：`src/motion.rs` N/A",
        "本组件判定：`src/spec.rs` N/A",
        "components/form-field/test/semantics.rs::form_field_file_placement_discipline_is_strict_for_component_scope",
        "components/form-field/test/form_field/semantics.rs::form_field_file_placement_discipline_is_strict_for_component_scope",
        "scripts/check-ui-component-files.sh",
    ] {
        assert!(
            local_semantics.contains(required) || check2_source.contains(required),
            "form-field file-placement discipline evidence should include `{required}`."
        );
    }
}

#[test]
fn form_field_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let module_source = load_source("../../components/form-field/src/mod.rs");
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");
    let protocol_source = load_source("../../components/form-field/src/protocol.rs");
    let check2_source = load_source("../../components/form-field/check2.md");
    let local_semantics = load_source("../../components/form-field/test/semantics.rs");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    assert!(
        !path_exists("../../components/form-field/src/spec.rs"),
        "form-field should keep hyper-structure builder as N/A and avoid `src/spec.rs`."
    );
    assert!(
        path_exists("../../components/form-field/src/protocol.rs"),
        "form-field should keep `src/protocol.rs` as schema fallback for N/A builder path."
    );

    for forbidden in [
        "pub struct FormFieldSpec",
        "impl FormFieldSpec",
        "fn new(",
        "fn render(",
        "FormFieldSpec::new",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "form-field should not expose hyper-structure builder token `{forbidden}`."
        );
    }

    for required in [
        "pub enum FormFieldComponentSchemaVersion",
        "pub struct FormFieldComponentSpec",
        "pub schema_version: FormFieldComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(required),
            "form-field protocol fallback should include `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`."
    );

    for required in [
        "fn form_field_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {",
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "本组件判定：N/A（`FormField` 为单字段基础组件，不存在复杂多槽位组合与可编排 DSL 输入，不引入 `*Spec::new()...render()` builder）",
        "components/form-field/test/semantics.rs::form_field_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "components/form-field/test/form_field/semantics.rs::form_field_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "scripts/check-ui-component-files.sh",
    ] {
        assert!(
            local_semantics.contains(required) || check2_source.contains(required),
            "form-field Hyper-Structure Builder evidence should include `{required}`."
        );
    }
}

#[test]
fn form_field_context_compression_manifest_and_rbi_projection_are_present_and_synced() {
    let check2_source = load_source("../../components/form-field/check2.md");
    let local_semantics = load_source("../../components/form-field/test/semantics.rs");
    let manifest_source = load_source("../../components/form-field/src/Component.toml");
    let rbi_source = load_source("../../components/form-field/src/form_field.rbi");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    assert!(
        path_exists("../../components/form-field/src/Component.toml"),
        "form-field should provide context-compression manifest at src/Component.toml."
    );
    assert!(
        path_exists("../../components/form-field/src/form_field.rbi"),
        "form-field should provide RBI projection at src/form_field.rbi."
    );

    for required in [
        "schema_version = \"1\"",
        "name = \"FormField\"",
        "crate = \"ui-form-field\"",
        "rbi = \"form_field.rbi\"",
        "name = \"is_selected\"",
        "name = \"default_selected\"",
        "name = \"on_selected_change\"",
        "name = \"indicator_variant\"",
        "name = \"indicator_placement\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "form-field Component.toml should include marker `{required}`."
        );
    }

    for required in [
        "pub type FormFieldTone = ui_form_field::FormFieldTone;",
        "pub type FormFieldIndicatorVariant = ui_form_field::FormFieldIndicatorVariant;",
        "pub type FormFieldIndicatorPlacement = ui_form_field::FormFieldIndicatorPlacement;",
        "pub fn FormField(",
        "is_selected: Option<leptos::prelude::Signal<bool>>",
        "default_selected: Option<bool>",
        "on_selected_change: Option<leptos::prelude::Callback<bool>>",
        "tone: FormFieldTone",
        "indicator_variant: FormFieldIndicatorVariant",
        "indicator_placement: FormFieldIndicatorPlacement",
        "label: Option<String>",
        "description: Option<String>",
        "error_message: Option<String>",
        "aria_label: Option<String>",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
        "class_name: Option<String>",
    ] {
        assert!(
            rbi_source.contains(required),
            "form-field RBI projection should include marker `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_context_compression_manifest_and_rbi_projection_are_present_and_synced";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`."
    );

    for required in [
        "fn form_field_context_compression_manifest_and_rbi_projection_are_present_and_synced() {",
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "已满足（Manifest 落位）：`components/form-field/src/Component.toml`",
        "已满足（RBI 投影落位）：`components/form-field/src/form_field.rbi`",
        "components/form-field/test/semantics.rs::form_field_context_compression_manifest_and_rbi_projection_are_present_and_synced",
        "components/form-field/test/form_field/semantics.rs::form_field_context_compression_manifest_and_rbi_projection_are_present_and_synced",
        "scripts/check-ui-component-files.sh",
    ] {
        assert!(
            local_semantics.contains(required) || check2_source.contains(required),
            "form-field context-compression evidence should include `{required}`."
        );
    }
}

#[test]
fn form_field_check2_documents_agent_contract_schema_governance_rules() {
    let check2_source = load_source("../../components/form-field/check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            check2_source.contains(required),
            "form-field checklist should keep agent-contract governance rule `{required}`."
        );
    }
}

#[test]
fn form_field_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");
    let manifest_source = load_source("../../components/form-field/src/Component.toml");
    let rbi_source = load_source("../../components/form-field/src/form_field.rbi");

    for required in [
        "pub const FORM_FIELD_AGENT_SCHEMA: &str = \"ui.form_field.agent-contract.v1\";",
        "pub const FORM_FIELD_AGENT_SCHEMA_VERSION: &str = \"v1\";",
        "pub enum FormFieldAgentIntent",
        "pub enum FormFieldAgentAction",
        "pub enum FormFieldAgentStateAxis",
        "pub enum FormFieldAgentSourceAxis",
        "pub enum FormFieldAgentStreamSupport",
        "pub enum FormFieldAgentStreamFallback",
        "pub enum FormFieldAgentOutputStatus",
        "pub struct FormFieldAgentContractAttrs",
        "pub fn resolve_agent_contract_attrs(",
    ] {
        assert!(
            logic_source.contains(required),
            "form-field logic should include typed agent-contract marker `{required}`."
        );
    }

    for required in [
        "let agent_contract = Memo::new(move |_| {",
        "logic::resolve_agent_contract_attrs(state.get(), selected_control_mode_attr)",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version",
        "data-ui-intent=move || agent_contract.get().intent_attr",
        "data-ui-action=move || agent_contract.get().action_attr",
        "data-ui-state=move || agent_contract.get().state_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
    ] {
        assert!(
            view_source.contains(required),
            "form-field view should mount schemaized agent-contract marker `{required}`."
        );
    }

    for required in [
        "[agent_contract]",
        "schema = \"ui.form_field.agent-contract.v1\"",
        "intent = \"selection-control\"",
        "action = \"render-snapshot\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "[[agent_contract_whitelist]]",
    ] {
        assert!(
            manifest_source.contains(required),
            "form-field Component.toml should include agent-contract marker `{required}`."
        );
    }

    for required in [
        "pub const FORM_FIELD_AGENT_SCHEMA: &str;",
        "pub const FORM_FIELD_AGENT_SCHEMA_VERSION: &str;",
        "pub enum FormFieldAgentIntent",
        "pub enum FormFieldAgentAction",
        "pub enum FormFieldAgentStateAxis",
        "pub enum FormFieldAgentSourceAxis",
        "pub struct FormFieldAgentContractAttrs",
    ] {
        assert!(
            rbi_source.contains(required),
            "form-field RBI should include typed agent-contract projection `{required}`."
        );
    }
}

#[test]
fn form_field_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
        "format!(\"data-ui-",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "form-field agent-contract fields should avoid free-form splicing `{forbidden}`."
        );
    }
}

#[test]
fn form_field_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let mod_source = load_source("../../components/form-field/src/mod.rs");
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");
    let styles_source = load_source("../../components/form-field/src/styles.rs");
    let combined = format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}");

    for forbidden in [
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !combined.contains(forbidden),
            "form-field agent-contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn form_field_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(required),
            "contract-hygiene script should enforce `{required}`."
        );
    }
}

#[test]
fn form_field_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("../../components/form-field/check2.md");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "N/A（职责边界）：`FormField` 不是 LLM 正文渲染组件",
        "components/form-field/test/semantics.rs::form_field_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "components/form-field/test/form_field/semantics.rs::form_field_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
    ] {
        assert!(
            check2_source.contains(needle),
            "form_field/check2.md should pin two-mode LLM streaming definition marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_streaming_check_script_covers_two_mode_definition_guard() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");
    let needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";

    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`.",
    );
}

#[test]
fn form_field_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("../../components/form-field/check2.md");
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "components/form-field/test/semantics.rs::form_field_check2_documents_snapshot_as_default_baseline_capability",
        "components/form-field/test/form_field/semantics.rs::form_field_check2_documents_snapshot_as_default_baseline_capability",
    ] {
        assert!(
            check2_source.contains(needle),
            "form_field/check2.md should pin snapshot baseline marker `{needle}`.",
        );
    }

    for needle in [
        "FormFieldAgentAction::RenderSnapshot",
        "FormFieldAgentStreamFallback::Snapshot",
        "FormFieldAgentOutputStatus::Verified",
        "data-ui-action=move || agent_contract.get().action_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
        "logic::resolve_state(FormFieldStateInput {",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "form-field snapshot baseline implementation should keep marker `{needle}`.",
        );
    }

    for needle in [
        "slug=\"form-field\"",
        "title=\"Hello World（默认路径）\"",
        "title=\"Switch Indicator + Description\"",
        "title=\"Checkbox Indicator + Quiet + Invalid/Disabled\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs form-field page should keep complete snapshot consumption path `{needle}`.",
        );
    }
}

#[test]
fn form_field_streaming_check_script_covers_snapshot_baseline_guard() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");
    let needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_snapshot_as_default_baseline_capability";

    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`.",
    );
}

#[test]
fn form_field_check2_documents_streaming_required_optional_classification_rules() {
    let check2_source = load_source("../../components/form-field/check2.md");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "`FormField` 归类为 `Streaming Optional`",
        "fallback=snapshot",
        "components/form-field/test/semantics.rs::form_field_check2_documents_streaming_required_optional_classification_rules",
        "components/form-field/test/form_field/semantics.rs::form_field_check2_documents_streaming_required_optional_classification_rules",
    ] {
        assert!(
            check2_source.contains(needle),
            "form_field/check2.md should keep streaming required/optional marker `{needle}`.",
        );
    }
}

#[test]
fn form_field_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("../../components/form-field/src/view.rs");

    for needle in [
        "role=\"group\"",
        "aria-label=move || control_aria_label.get_value()",
        "aria-describedby=move || describedby.get()",
        "aria-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "aria-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-state=move || state.get().state_attr",
        "data-selected-control-mode=selected_control_mode_attr",
        "data-selected-change-source=selected_change_source_attr",
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "form-field should keep continuous role/aria/data semantics marker `{needle}` in optional-streaming scope.",
        );
    }
}

#[test]
fn form_field_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("../../components/form-field/src/view.rs");
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let combined = format!("{view_source}\n{logic_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "form-field should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`.",
        );
    }
}

#[test]
fn form_field_streaming_check_script_covers_required_optional_classification_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn form_field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let mod_source = load_source("../../components/form-field/src/mod.rs");
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let styles_source = load_source("../../components/form-field/src/styles.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");
    let protocol_source = load_source("../../components/form-field/src/protocol.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{protocol_source}");

    for forbidden in ["unwrap(", "expect(", "unwrap_err(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "form-field non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn form_field_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("../../components/form-field/src/logic.rs");
    let view_source = load_source("../../components/form-field/src/view.rs");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> =",
        "Cow::Borrowed(\"ui-form-field\")",
        "Cow::Borrowed(\"ui-form-field--custom-class\")",
        "Cow::Owned(base_class_name)",
        ".map(|class_name| class_name.as_ref())",
    ] {
        assert!(
            logic_source.contains(required),
            "form-field logic should keep Cow-based string hotspot mitigation marker `{required}`.",
        );
    }

    for forbidden in [
        "DEFAULT_ID_BASE.to_string()",
        "\"ui-form-field\".to_string()",
        "\"ui-form-field--selected\".to_string()",
        "\"ui-form-field--unselected\".to_string()",
        "\"ui-form-field--invalid\".to_string()",
        "\"ui-form-field--disabled\".to_string()",
        "\"ui-form-field--with-description\".to_string()",
        "\"ui-form-field--with-error\".to_string()",
        "\"ui-form-field--custom-class\".to_string()",
        "\"ui-switch\".to_string()",
        "\"ui-form-field__control\".to_string()",
        "String::from(\"ui-form-field\")",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "form-field string hotspot contract should avoid `{forbidden}`.",
        );
    }
}

#[test]
fn form_field_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
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
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }

    for needle in [
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_marks_semantic_and_performance_regression_contract_complete() {
    let check2_source = load_source("../../components/form-field/check2.md");

    for needle in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "components/form-field/test/semantics.rs::form_field_semantic_contract_tests_cover_branch_matrix_without_snapshot_dependency",
        "components/form-field/test/semantics.rs::form_field_state_markers_are_observable_queryable_and_enumerable",
        "components/form-field/test/semantics.rs::form_field_performance_governance_budget_is_defined_traceable_and_blocking",
        "components/form-field/test/form_field/semantics.rs::form_field_e2e_contract_uses_semantic_selectors_and_settled_waits",
        "components/form-field/test/form_field/semantics.rs::form_field_performance_governance_budget_is_defined_traceable_and_blocking",
        "render_count",
        "bash scripts/check-ui-performance.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "form_field/check2.md semantic+performance section should reference `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_marks_semantic_test_priority_item_complete() {
    let check2_source = load_source("../../components/form-field/check2.md");

    assert!(
        check2_source.contains(
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。"
        ),
        "form-field check2 should mark semantic-test-priority item complete.",
    );

    for needle in [
        "components/form-field/test/semantics.rs::form_field_semantic_contract_tests_cover_branch_matrix_without_snapshot_dependency",
        "components/form-field/test/semantics.rs::form_field_state_markers_are_observable_queryable_and_enumerable",
        "components/form-field/test/semantics.rs::form_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "components/form-field/test/form_field/semantics.rs::form_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "scripts/check-ui-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "form-field check2 semantic-test-priority section should reference `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("../../components/form-field/check2.md");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "Cow<'static, str>",
        "./scripts/check-rust-hygiene.sh",
        "components/form-field/test/semantics.rs::form_field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "components/form-field/test/semantics.rs::form_field_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "components/form-field/test/semantics.rs::form_field_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "components/form-field/test/form_field/semantics.rs::form_field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "components/form-field/test/form_field/semantics.rs::form_field_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "components/form-field/test/form_field/semantics.rs::form_field_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "scripts/check-ui-engineering.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "form_field/check2.md rust-hygiene section should reference `{needle}`.",
        );
    }
}

#[test]
fn form_field_check2_marks_component_governance_complete() {
    let check2_source = load_source("../../components/form-field/src/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui` 定义",
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
    let check2_source = load_source("../../components/form-field/src/check2.md");

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
    let check2_source = load_source("../../components/form-field/src/check2.md");

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
    let check2_source = load_source("../../components/form-field/src/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "FormField check2.md should not keep unchecked checklist items after completion."
    );
}
