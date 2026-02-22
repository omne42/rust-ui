use ui_test_support::source_contract;

#[test]
fn field_component_layering_contract_is_explicit() {
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let styles_source = include_str!("../src/styles.rs");
    let motion_source = include_str!("../src/motion.rs");

    for needle in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "pub mod motion;",
        "mod view;",
        "pub use view::Field;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Field module boundary should include `{needle}`."
        );
    }

    assert!(
        logic_source.contains("pub use ui_state_primitives::field::*;"),
        "logic.rs should consume state primitives from ui-state-primitives."
    );
    assert!(
        view_source.contains("use_field("),
        "view.rs should mount headless field contract."
    );
    assert!(
        view_source.contains("motion::attach_motion("),
        "view.rs should mount motion contract from motion.rs."
    );
    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "styles.rs should keep token-first static CSS output."
    );
    assert!(
        motion_source.contains("pub fn attach_motion("),
        "motion.rs should provide attach motion contract."
    );
}

#[test]
fn field_component_public_surface_does_not_leak_dom_runtime_types() {
    let mod_source = include_str!("../src/mod.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "js_sys::",
        "HtmlElement",
        "Element",
        "Document",
        "Window",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not leak DOM/runtime detail `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not leak DOM/runtime detail `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not leak DOM/runtime detail `{forbidden}`."
        );
    }
}

#[test]
fn field_api_boolean_props_use_is_prefix_and_keep_legacy_aliases_for_migration() {
    let view_source = include_str!("../src/view.rs");

    for required in [
        "#[prop(optional)] is_required: Option<bool>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] is_invalid: Option<bool>",
        "#[prop(optional)] required: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] invalid: Option<bool>",
        "logic::resolve_is_required(is_required, required)",
        "logic::resolve_is_disabled(is_disabled, disabled)",
        "logic::resolve_is_invalid(is_invalid, invalid)",
    ] {
        assert!(
            view_source.contains(required),
            "Field API naming contract should include `{required}`."
        );
    }
}

#[test]
fn field_readme_documents_api_naming_migration_strategy() {
    let readme_source = include_str!("../src/README.md");
    for required in [
        "## 命名迁移（兼容策略）",
        "新命名：布尔状态统一使用 `is_*`",
        "兼容别名：保留 `required` / `disabled` / `invalid`",
        "优先级：当新旧命名同时传入时，始终以 `is_*` 为准",
    ] {
        assert!(
            readme_source.contains(required),
            "README should keep migration strategy marker `{required}`."
        );
    }
}

#[test]
fn field_controlled_uncontrolled_pair_rule_is_explicitly_na_for_stateless_axes() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let readme_source = include_str!("../src/README.md");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "#[prop(optional)] default_",
        "#[prop(optional)] on_",
        "on_value_change",
        "default_value",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Field should not expose fake controlled-value axis token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "FieldGroup should not expose fake controlled-value axis token `{forbidden}`."
        );
    }

    for required in [
        "## Controlled / Uncontrolled",
        "N/A-by-design：`Field/FieldGroup` 不管理 `value/open/checked/selected` 一类本地状态轴。",
        "不提供 `default_*` 或 `on_*_change` 状态机 API。",
        "N/A-by-design：`Field/FieldGroup` 为字段语义装配组件",
    ] {
        assert!(
            readme_source.contains(required) || check2_source.contains(required),
            "Controlled/uncontrolled N/A evidence should include `{required}`."
        );
    }
}

#[test]
fn field_default_values_are_normalized_in_logic_not_view() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");

    for required in [
        "pub struct FieldContentInput",
        "pub struct FieldContent",
        "pub fn resolve_content(",
        "normalize_error_message_cow(input.error_message, input.is_invalid)",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep default normalization contract `{required}`."
        );
    }

    for forbidden in [
        "normalize_optional_text(",
        "normalize_aria_label(",
        "normalize_error_message(",
        "unwrap_or_default()",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not keep fallback/default token `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("logic::resolve_content(logic::FieldContentInput"),
        "view.rs should consume logic-level normalized content."
    );
}

#[test]
fn field_state_normalization_is_centralized_in_logic_layer() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");

    for required in [
        "pub fn resolve_content(",
        "pub fn resolve_is_required(",
        "pub fn resolve_is_disabled(",
        "pub fn resolve_is_invalid(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization entry `{required}`."
        );
    }

    assert!(
        group_logic_source.contains("pub fn resolve_content("),
        "group/logic.rs should keep group normalization entry."
    );

    for required in [
        "logic::resolve_content(logic::FieldContentInput",
        "logic::resolve_state(FieldStateInput",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should consume centralized logic output `{required}`."
        );
    }

    for required in [
        "logic::resolve_content(logic::FieldGroupContentInput",
        "logic::resolve_state(FieldGroupStateInput",
    ] {
        assert!(
            group_view_source.contains(required),
            "group/view.rs should consume centralized logic output `{required}`."
        );
    }

    assert!(
        !group_view_source.contains("has_custom_class_name: class_name.get_value().is_some()"),
        "group/view.rs should not recompute class source state from raw view values."
    );
}

#[test]
fn field_discrete_state_axes_use_enum_types_not_string_protocols() {
    let primitive_source = include_str!("../../../crates/ui-state-primitives/src/field.rs");
    let group_primitive_source =
        include_str!("../../../crates/ui-state-primitives/src/field_group.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");

    for required in ["pub enum FieldOrientation", "pub enum FieldTone"] {
        assert!(
            primitive_source.contains(required),
            "field primitive should keep enum-typed discrete axis `{required}`."
        );
    }

    for required in [
        "pub enum FieldGroupOrientation",
        "pub enum FieldGroupDensity",
    ] {
        assert!(
            group_primitive_source.contains(required),
            "field-group primitive should keep enum-typed discrete axis `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] orientation: FieldOrientation",
        "#[prop(optional)] tone: FieldTone",
        "#[prop(default = FieldGroupOrientation::Vertical)] orientation: FieldGroupOrientation",
        "#[prop(default = FieldGroupDensity::Comfortable)] density: FieldGroupDensity",
    ] {
        assert!(
            view_source.contains(required) || group_view_source.contains(required),
            "component API should expose typed discrete axis `{required}`."
        );
    }

    for forbidden in [
        "orientation: Option<String>",
        "tone: Option<String>",
        "density: Option<String>",
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view should not expose string-based discrete axis `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should not expose string-based discrete axis `{forbidden}`."
        );
    }
}

#[test]
fn field_state_primitive_source_boundary_is_enforced() {
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");

    for required in [
        "pub use ui_state_primitives::field::*;",
        "pub use ui_state_primitives::field_group::*;",
    ] {
        assert!(
            logic_source.contains(required) || group_logic_source.contains(required),
            "logic layer should consume state primitives `{required}`."
        );
    }

    for forbidden in [
        "ui_state_primitives::field",
        "ui_state_primitives::field_group",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not bypass logic layer to read primitive `{forbidden}` directly."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "group/view.rs should not bypass logic layer to read primitive `{forbidden}` directly."
        );
    }

    for forbidden in [
        "AppStore",
        "GlobalStore",
        "store::",
        "redux",
        "zustand",
        "mobx",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not depend on business store token `{forbidden}`."
        );
        assert!(
            !group_logic_source.contains(forbidden),
            "group/logic.rs should not depend on business store token `{forbidden}`."
        );
    }
}

#[test]
fn field_async_interaction_contract_is_explicitly_na_by_design() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let readme_source = include_str!("../src/README.md");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "aria_busy",
        "retry",
        "use_async_action",
        "on_retry",
        "on_load",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view should not expose async interaction token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should not expose async interaction token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "field logic should not implement async state protocol token `{forbidden}`."
        );
        assert!(
            !group_logic_source.contains(forbidden),
            "field-group logic should not implement async state protocol token `{forbidden}`."
        );
    }

    for required in [
        "非目标：不承载受控 value 状态机，不直接处理业务异步提交流程。",
        "N/A-by-design：`Field/FieldGroup` 当前仅承载同步字段语义装配",
    ] {
        assert!(
            readme_source.contains(required) || check2_source.contains(required),
            "async N/A evidence should include `{required}`."
        );
    }
}

#[test]
fn field_dx_default_api_path_is_minimal_and_no_state_wiring() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let readme_source = include_str!("../src/README.md");
    let docs_field_page_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/forms_extra.rs",
    );

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(into)] state:",
        "state: FieldState",
        "state: FieldStateInput",
        "state: FieldGroupState",
        "state: FieldGroupStateInput",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view API should not require internal state object `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view API should not require internal state object `{forbidden}`."
        );
    }

    let hello_heading = "## Hello World（最小可用）";
    let heading_offset = readme_source
        .find(hello_heading)
        .expect("README should document a hello-world section.");
    let hello_section = &readme_source[heading_offset + hello_heading.len()..];
    let fence_offset = hello_section
        .find("```rust")
        .expect("README hello-world should provide a rust code block.");
    let fenced = &hello_section[fence_offset + "```rust".len()..];
    let end_offset = fenced
        .find("```")
        .expect("README hello-world rust code block should be closed.");
    let hello_code = fenced[..end_offset].trim();

    let hello_lines: Vec<&str> = hello_code
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    assert!(
        hello_lines.len() <= 5,
        "README hello-world should stay within 5 non-empty lines, got {} lines.",
        hello_lines.len()
    );
    assert!(
        hello_code.contains("<Field"),
        "README hello-world should use direct `<Field ...>` default API."
    );
    assert!(
        !hello_code.contains("state="),
        "README hello-world should not require manual `state=` wiring."
    );

    for required in [
        "pub(super) fn field() -> AnyView",
        "title=\"Hello World (Default API)\"",
        "description=\"Minimal path: no manual wiring to ui-state-primitives/ui-headless state machines.\"",
        "let hello_world_code = Signal::derive(move ||",
        "r#\"<Field label=\"Email\".to_string()>",
    ] {
        assert!(
            docs_field_page_source.contains(required),
            "docs-app field page should keep DX default-path marker `{required}`."
        );
    }
}

#[test]
fn field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_field_page_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/forms_extra.rs",
    );
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub(super) fn field() -> AnyView",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix (Required / Invalid / Disabled)\"",
        "title=\"Controlled vs Uncontrolled (Stateless Contract)\"",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "data-slot=\"field-state-matrix\"",
        "data-slot=\"field-controlled-matrix\"",
        "data-slot=\"field-source-first\"",
        "\"Source-first / Copy-Paste Ready\"",
        "compose_copy_ready_code",
        "code_imports=field_imports.clone()",
        "class_name=\"docs-field-source-copy\".to_string()",
    ] {
        assert!(
            docs_field_page_source.contains(required),
            "field docs should keep copy-paste-ready docs-product marker `{required}`."
        );
    }

    for required in [
        "#[prop(optional, into)] code_imports: Option<String>",
        "compose_copy_ready_code",
        "missing_import_lines",
    ] {
        assert!(
            playground_source.contains(required),
            "shared Playground copy pipeline should keep `{required}`."
        );
    }

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "`apps/docs-app/src/pages/components/pages/forms_extra.rs::field` 已提供 `Hello World (Default API)`、`State Matrix (Required / Invalid / Disabled)`、`Controlled vs Uncontrolled (Stateless Contract)` 与 `Streaming Optional (fallback=snapshot)` Playground",
        "`code_imports=field_imports` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 保障一键复制代码自动补全 imports",
        "`data-slot=\"field-source-first\"` 区块提供 Source-first 路径与复制入口（`docs-field-source-copy`）",
        "回归：`components/field/test/semantics.rs::field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot`。",
    ] {
        assert!(
            check2_source.contains(required),
            "field check2 should keep docs-product evidence `{required}`."
        );
    }
}

#[test]
fn field_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(required),
            "field/check2.md should keep source-first copy-paste-ready rule `{required}`."
        );
    }
}

#[test]
fn field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_field_page_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/forms_extra.rs",
    );
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let code_block_view_source = include_str!("../../../components/code-block/src/view.rs");
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");

    for required in [
        "data-slot=\"field-source-first\"",
        "data-slot=\"field-source-first-contract\"",
        "data-slot=\"field-source-first-dependency-baseline\"",
        "\"Source-first / Copy-Paste Ready\"",
        "<code>\"Show code\"</code>",
        "compose_copy_ready_code",
        "code_imports=field_imports.clone()",
        "class_name=\"docs-field-source-copy\".to_string()",
        "data-slot=\"field-source-prerequisites\"",
        "\"component-field\"",
        "\"inject-css\"",
        "Dependency baseline (Cargo.toml)",
        "ui = { default-features = false, features = [\\\"component-field\\\", \\\"inject-css\\\"] }",
        "components/field/src/mod.rs",
        "components/field/src/logic.rs",
        "components/field/src/view.rs",
        "components/field/src/styles.rs",
        "components/field/src/motion.rs",
    ] {
        assert!(
            docs_field_page_source.contains(required),
            "field source-first docs should contain `{required}`."
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "<CodeBlock code=resolved_code.get() />",
        "missing_import_lines(&raw, &imports)",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground copy pipeline should contain `{required}`."
        );
    }

    for required in [
        "class_name=\"ui-code-block__copy-button\"",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view_source.contains(required),
            "CodeBlock copy affordance should contain `{required}`."
        );
    }

    for required in [
        "orientation=orientation",
        "tone=tone",
        "required=required",
        "invalid=invalid",
        "disabled=disabled",
        "motion=motion",
    ] {
        assert!(
            docs_field_page_source.contains(required),
            "field docs code should stay in sync with implementation marker `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] orientation: FieldOrientation",
        "#[prop(optional)] tone: FieldTone",
        "#[prop(optional)] required: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] invalid: Option<bool>",
        "#[prop(optional)] motion: FieldMotion",
    ] {
        assert!(
            view_source.contains(required),
            "field view contract should define `{required}`."
        );
    }

    for required in [
        "resolve_is_required",
        "resolve_is_disabled",
        "resolve_is_invalid",
    ] {
        assert!(
            logic_source.contains(required),
            "field logic should keep normalization marker `{required}`."
        );
    }
}

#[test]
fn field_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let dx_script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "cargo test -p ui-field field_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-field field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            dx_script_source.contains(required),
            "field dx script should cover source-first copy-paste-ready command `{required}`."
        );
    }
}

#[test]
fn field_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "field_check2_documents_source_first_copy_paste_ready_rules",
        "field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "field_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "field/check2.md should keep source-first completion evidence `{required}`."
        );
    }
}

#[test]
fn field_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "field/check2.md should keep heroui-benchmark docs-sync rule `{required}`."
        );
    }
}

#[test]
fn field_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/forms_extra.rs",
    );
    let readme_source = include_str!("../src/README.md");

    for required in [
        "### Field 同步记录（2026-02-20）",
        "参数模型同步：`Field` 参数主轴保持 `orientation/tone/is_required/is_disabled/is_invalid`",
        "component_doc!(\"Field\", \"field\", \"Forms\", forms_extra::field)",
        "forms_extra.rs::field()",
        "title=\"Field\"",
        "slug=\"field\"",
        "`components/field/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(required),
            "heroui strategy doc should include field synchronization marker `{required}`."
        );
    }

    for required in [
        "component_doc!(",
        "\"Field\"",
        "\"field\"",
        "forms_extra::field",
    ] {
        assert!(
            pages_source.contains(required),
            "component docs index should expose field entry marker `{required}`."
        );
    }

    for required in [
        "pub(super) fn field() -> AnyView {",
        "title=\"Field\"",
        "slug=\"field\"",
    ] {
        assert!(
            docs_source.contains(required),
            "docs-app field page should stay indexable via marker `{required}`."
        );
    }

    assert!(
        readme_source.contains("# Field"),
        "field README should remain an equivalent component docs entry."
    );
}

#[test]
fn field_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let dx_script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "cargo test -p ui-field field_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-field field_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            dx_script_source.contains(required),
            "field dx script should cover heroui-benchmark docs-sync command `{required}`."
        );
    }
}

#[test]
fn field_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "field_check2_documents_heroui_benchmark_docs_sync_rules",
        "field_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "field_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "field/check2.md should keep heroui-benchmark docs-sync evidence `{required}`."
        );
    }
}

#[test]
fn field_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs::field",
        "field-api-matrix",
        "State Matrix",
        "Controlled vs Uncontrolled",
        "field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "field_check2_documents_docs_sync_and_state_matrix_rules",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "field/check2.md should keep docs-sync evidence `{required}`."
        );
    }
}

#[test]
fn field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_field_page_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/forms_extra.rs",
    );
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let primitive_source = include_str!("../../../crates/ui-state-primitives/src/field.rs");

    for required in [
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix (Required / Invalid / Disabled)\"",
        "title=\"Controlled vs Uncontrolled (Stateless Contract)\"",
        "data-slot=\"field-api-matrix\"",
        "data-slot=\"field-api-rows\"",
        "data-slot=\"field-state-matrix\"",
        "data-slot=\"field-controlled-matrix\"",
        "is_required / is_disabled / is_invalid",
        "required / disabled / invalid",
        "motion: FieldMotion",
        "default = false",
        "default = FieldMotion::default()",
        "default = vertical",
        "default = default",
    ] {
        assert!(
            docs_field_page_source.contains(required),
            "field docs page should keep docs-sync marker `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] orientation: FieldOrientation",
        "#[prop(optional)] tone: FieldTone",
        "#[prop(optional)] is_required: Option<bool>",
        "#[prop(optional)] required: Option<bool>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] is_invalid: Option<bool>",
        "#[prop(optional)] invalid: Option<bool>",
        "#[prop(optional)] motion: FieldMotion",
    ] {
        assert!(
            view_source.contains(required),
            "field view API should keep prop marker `{required}`."
        );
    }

    for required in [
        "fn resolve_bool_value(primary: Option<bool>, legacy: Option<bool>) -> bool {",
        "primary.or(legacy).unwrap_or_default()",
        "pub fn resolve_is_required(",
        "pub fn resolve_is_disabled(",
        "pub fn resolve_is_invalid(",
    ] {
        assert!(
            logic_source.contains(required),
            "field logic defaults should keep marker `{required}`."
        );
    }

    for required in [
        "pub enum FieldOrientation",
        "Vertical,",
        "pub enum FieldTone",
        "Default,",
    ] {
        assert!(
            primitive_source.contains(required),
            "field primitive default semantics should keep marker `{required}`."
        );
    }
}

#[test]
fn field_check2_documents_documentation_as_product_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "Hello World",
        "先用起来，再进阶",
        "components/field/src/README.md",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs::field",
        "field_documentation_entry_exists_with_beginner_first_progression",
        "field_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "field/check2.md should keep documentation-as-product evidence `{required}`."
        );
    }
}

#[test]
fn field_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = include_str!("../src/README.md");
    let docs_field_page_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/forms_extra.rs",
    );

    for required in [
        "# Field",
        "## 快速开始（先用起来）",
        "<Field label=\"Email\".to_string() is_required=true>",
        "先用默认 API（`label + children`）完成基础渲染。",
        "再按需开启 `is_required/is_disabled/is_invalid`。",
        "最后再接入 `orientation/tone/motion/class_name` 等进阶能力。",
        "## Hello World（最小可用）",
        "## docs-app 入口",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs",
    ] {
        assert!(
            readme_source.contains(required),
            "field README should keep beginner-friendly marker `{required}`."
        );
    }

    let quick_start_index = readme_source
        .find("## 快速开始（先用起来）")
        .expect("field README should include quick-start section.");
    let architecture_index = readme_source
        .find("## Architecture Layers")
        .expect("field README should include architecture section.");
    let api_table_index = readme_source
        .find("## API (Table)")
        .expect("field README should include api-table section.");
    assert!(
        quick_start_index < architecture_index && quick_start_index < api_table_index,
        "field README should place quick-start/default path before architecture/API details."
    );

    for required in [
        "pub(super) fn field() -> AnyView",
        "title=\"Hello World (Default API)\"",
        "title=\"Required + Description\"",
        "title=\"Horizontal + Invalid + Custom Class\"",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
    ] {
        assert!(
            docs_field_page_source.contains(required),
            "docs-app field entry should keep beginner-to-advanced marker `{required}`."
        );
    }

    let hello_world_index = docs_field_page_source
        .find("title=\"Hello World (Default API)\"")
        .expect("docs field page should include hello-world playground.");
    let workbench_index = docs_field_page_source
        .find("title=\"Workbench (Display + Config + Code + CSS Test)\"")
        .expect("docs field page should include workbench playground.");
    assert!(
        hello_world_index < workbench_index,
        "docs field page should keep default-path hello world before advanced workbench."
    );
}

#[test]
fn field_dx_check_script_covers_documentation_as_product_contract() {
    let dx_script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "cargo test -p ui-field field_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-field field_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            dx_script_source.contains(required),
            "field dx script should cover documentation-as-product command `{required}`."
        );
    }
}

#[test]
fn field_group_api_prefers_explicit_parent_child_composition() {
    let group_view_source = include_str!("../src/group/view.rs");
    let docs_groups_page_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/forms_groups.rs",
    );

    assert!(
        group_view_source.contains("children: Children"),
        "FieldGroup API should model composition through explicit children."
    );
    assert!(
        group_view_source.contains("data-slot=\"field-group-content\""),
        "FieldGroup view should keep explicit child mounting slot."
    );

    for forbidden in [
        "#[prop(optional, into)] labels",
        "#[prop(optional, into)] titles",
        "#[prop(optional, into)] panels",
        "#[prop(optional, into)] items",
        "ItemSpec",
    ] {
        assert!(
            !group_view_source.contains(forbidden),
            "FieldGroup should not expose parallel-array/config sugar token `{forbidden}`."
        );
    }

    let group_fn_start = docs_groups_page_source
        .find("pub(super) fn field_group() -> AnyView {")
        .expect("docs-app should define a field_group page.");
    let group_fn_tail = &docs_groups_page_source[group_fn_start..];
    let group_fn_end = group_fn_tail
        .find("pub(super) fn date_input_group() -> AnyView {")
        .expect("field_group page should end before date_input_group page.");
    let group_fn_source = &group_fn_tail[..group_fn_end];

    for required in [
        "<FieldGroup",
        "<Field label=\"Name\".to_string()>",
        "<Field label=\"Email\".to_string()>",
        "<Field label=\"Purchase Order\".to_string() disabled=true>",
    ] {
        assert!(
            group_fn_source.contains(required),
            "docs-app FieldGroup page should demonstrate explicit parent/child composition `{required}`."
        );
    }

    for forbidden in ["labels=", "titles=", "panels=", "items="] {
        assert!(
            !group_fn_source.contains(forbidden),
            "docs-app FieldGroup default path should avoid implicit parallel-collection prop `{forbidden}`."
        );
    }
}

#[test]
fn field_macro_micro_duality_rule_is_explicitly_na_without_drag_interaction() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let motion_source = include_str!("../src/motion.rs");
    let readme_source = include_str!("../src/README.md");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "Dragging",
        "Action::DragEnd",
        "on:dragstart",
        "on:drag",
        "on:dragend",
        "on:pointermove",
        "requestAnimationFrame",
        "raf",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view should not contain drag micro-loop token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should not contain drag micro-loop token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "field logic should not contain drag macro-state token `{forbidden}`."
        );
        assert!(
            !group_logic_source.contains(forbidden),
            "field-group logic should not contain drag macro-state token `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "field motion should not contain drag runtime token `{forbidden}`."
        );
    }

    for required in [
        "N/A-by-design：`Field/FieldGroup` 不提供拖拽等高频连续交互",
        "非目标：不承载受控 value 状态机，不直接处理业务异步提交流程。",
    ] {
        assert!(
            check2_source.contains(required) || readme_source.contains(required),
            "macro/micro N/A evidence should include `{required}`."
        );
    }
}

#[test]
fn field_two_pass_geometry_rule_is_explicitly_na_without_measurement_cycle() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let motion_source = include_str!("../src/motion.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "Intent -> Measure",
        "Rectification",
        "getBoundingClientRect",
        "offsetWidth",
        "offsetHeight",
        "ResizeObserver",
        "IntersectionObserver",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view should not include geometry two-pass token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should not include geometry two-pass token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "field logic should not include geometry rectification token `{forbidden}`."
        );
        assert!(
            !group_logic_source.contains(forbidden),
            "field-group logic should not include geometry rectification token `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "field motion should not include geometry measure-loop token `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("N/A-by-design：`Field/FieldGroup` 不依赖 DOM 几何测量"),
        "two-pass geometry N/A evidence should be documented in check2."
    );
}

#[test]
fn field_registration_protocol_rule_is_explicitly_na_without_dynamic_item_registry() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view should not contain registration protocol token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should not contain registration protocol token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "field logic should not contain registration protocol token `{forbidden}`."
        );
        assert!(
            !group_logic_source.contains(forbidden),
            "field-group logic should not contain registration protocol token `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("N/A-by-design：`Field/FieldGroup` 不是动态集合容器"),
        "registration protocol N/A evidence should be documented in check2."
    );
}

#[test]
fn field_slot_projection_rule_is_explicitly_na_without_keepalive_contract() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let motion_source = include_str!("../src/motion.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
        "slot_mode",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view should not contain slot projection lifecycle token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should not contain slot projection lifecycle token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "field logic should not contain slot projection lifecycle token `{forbidden}`."
        );
        assert!(
            !group_logic_source.contains(forbidden),
            "field-group logic should not contain slot projection lifecycle token `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "field motion should not contain keep-alive lifecycle token `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("N/A-by-design：`Field/FieldGroup` 不具备容器型内容投影策略"),
        "slot projection N/A evidence should be documented in check2."
    );
}

#[test]
fn field_env_stream_rule_is_explicitly_na_without_environment_subscriptions() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "on:resize",
        "BreakpointChanged",
        "themechange",
        "on:scroll",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view should not contain raw env-stream token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should not contain raw env-stream token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "field logic should not contain env-stream action token `{forbidden}`."
        );
        assert!(
            !group_logic_source.contains(forbidden),
            "field-group logic should not contain env-stream action token `{forbidden}`."
        );
    }

    assert!(
        check2_source
            .contains("N/A-by-design：`Field/FieldGroup` 不订阅 `Resize/Theme/Intersection`"),
        "env stream N/A evidence should be documented in check2."
    );
}

#[test]
fn field_event_light_cone_rule_is_explicitly_na_without_large_collection_bus() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "prop drilling",
        "Table",
        "Grid",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view should not contain event-light-cone token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should not contain event-light-cone token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "field logic should not contain event-light-cone token `{forbidden}`."
        );
        assert!(
            !group_logic_source.contains(forbidden),
            "field-group logic should not contain event-light-cone token `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("N/A-by-design：`Field/FieldGroup` 不属于大集合批量交互容器"),
        "event light cone N/A evidence should be documented in check2."
    );
}

#[test]
fn field_causality_bus_rule_is_explicitly_na_without_trace_chain_bus() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "TraceId",
        "trace_id",
        "Causality Bus",
        "broadcast",
        "subscriber",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view should not contain causality-bus token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should not contain causality-bus token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "field logic should not contain causality-bus token `{forbidden}`."
        );
        assert!(
            !group_logic_source.contains(forbidden),
            "field-group logic should not contain causality-bus token `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("N/A-by-design：`Field/FieldGroup` 不存在复杂派生总线"),
        "causality bus N/A evidence should be documented in check2."
    );
}

#[test]
fn field_focus_stack_rule_is_explicitly_na_without_overlay_focus_restore() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let motion_source = include_str!("../src/motion.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "NodeRef",
        "document.body",
        "FallbackTo",
        "Selector",
        "focus_stack",
        "restore_focus",
        "overlay",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view should not contain overlay focus-stack token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should not contain overlay focus-stack token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "field logic should not contain overlay focus-stack token `{forbidden}`."
        );
        assert!(
            !group_logic_source.contains(forbidden),
            "field-group logic should not contain overlay focus-stack token `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "field motion should not contain overlay focus-stack token `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("N/A-by-design：`Field/FieldGroup` 不创建 overlay layer"),
        "focus stack N/A evidence should be documented in check2."
    );
}

#[test]
fn field_escape_hatch_rule_is_explicitly_na_without_foreign_zone_integration() {
    let mod_source = include_str!("../src/mod.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let motion_source = include_str!("../src/motion.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "GoogleMap",
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
        "foreign_zone",
        "third_party_instance",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "field mod should not expose foreign-zone integration token `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "field view should not contain foreign-zone integration token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should not contain foreign-zone integration token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "field logic should not contain foreign-zone integration token `{forbidden}`."
        );
        assert!(
            !group_logic_source.contains(forbidden),
            "field-group logic should not contain foreign-zone integration token `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "field motion should not contain foreign-zone integration token `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("N/A-by-design：`Field/FieldGroup` 不集成 ECharts/Map"),
        "escape-hatch N/A evidence should be documented in check2."
    );
}

#[test]
fn field_a11y_i18n_l10n_contract_is_mounted_and_text_source_is_not_hardcoded_in_view() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let headless_field_source = include_str!("../../../crates/ui-headless/src/field.rs");
    let headless_group_source = include_str!("../../../crates/ui-headless/src/field_group.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "use_field(",
        "use_field_group(",
        "aria-label=move || headless.get().attrs.aria_label",
        "aria-disabled=move || headless.get().attrs.aria_disabled",
        "aria-invalid=move || headless.get().attrs.aria_invalid",
        "role=move || headless.get().attrs.role",
        "lang=move || headless.get().attrs.lang",
        "dir=move || headless.get().attrs.dir",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
    ] {
        assert!(
            view_source.contains(required) || group_view_source.contains(required),
            "A11y/i18n contract should include `{required}`."
        );
    }

    for required in [
        "use crate::a11y::{A11yDirection, locale_attrs};",
        "lang: locale.lang",
        "dir: locale.dir",
    ] {
        assert!(
            headless_field_source.contains(required) || headless_group_source.contains(required),
            "headless field contracts should consume shared a11y utility `{required}`."
        );
    }

    for required in [
        "normalize_error_message_cow(input.error_message, input.is_invalid)",
        "normalize_aria_label_cow(input.aria_label)",
    ] {
        assert!(
            logic_source.contains(required) || group_logic_source.contains(required),
            "fallback text normalization should stay in logic/primitives path `{required}`."
        );
    }

    for forbidden in ["\"Invalid value\"", "\"Field group\""] {
        assert!(
            !view_source.contains(forbidden),
            "field view should not hardcode user-visible fallback text token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should not hardcode user-visible fallback text token `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("role=\"alert\""),
        "field error message should expose accessible live error role."
    );

    assert!(
        check2_source.contains("本组件落地：`Field`/`FieldGroup` 通过 `ui-headless` 的 `use_field`/`use_field_group` 挂载语义契约"),
        "A11y/i18n/l10n evidence should be documented in check2."
    );
}

#[test]
fn field_state_is_observable_and_source_markers_are_stable_and_enumerable() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let primitive_field_source = include_str!("../../../crates/ui-state-primitives/src/field.rs");
    let primitive_group_source =
        include_str!("../../../crates/ui-state-primitives/src/field_group.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "data-state=move || headless.get().attrs.data_state",
        "data-required=move || headless.get().attrs.data_required",
        "data-disabled=move || headless.get().attrs.data_disabled",
        "data-invalid=move || headless.get().attrs.data_invalid",
        "data-required-source=required_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-invalid-source=invalid_source_attr",
        "data-aria-source=move || headless.get().attrs.data_aria_source",
        "data-error-source=move || headless.get().attrs.data_error_source",
        "data-class-source=move || headless.get().attrs.data_class_source",
        "aria-disabled=move || headless.get().attrs.aria_disabled",
        "aria-invalid=move || headless.get().attrs.aria_invalid",
    ] {
        assert!(
            view_source.contains(required),
            "field view should expose stable observability marker `{required}`."
        );
    }

    for required in [
        "data-state=move || headless.get().attrs.data_state",
        "data-disabled=move || headless.get().attrs.data_disabled",
        "data-invalid=move || headless.get().attrs.data_invalid",
        "data-disabled-source=disabled_source_attr",
        "data-invalid-source=invalid_source_attr",
        "data-aria-source=move || headless.get().attrs.data_aria_source",
        "data-class-source=move || headless.get().attrs.data_class_source",
        "aria-disabled=move || headless.get().attrs.aria_disabled",
        "aria-invalid=move || headless.get().attrs.aria_invalid",
    ] {
        assert!(
            group_view_source.contains(required),
            "field-group view should expose stable observability marker `{required}`."
        );
    }

    for required in [
        "pub enum FieldBoolPropSource",
        "FieldBoolPropSource::IsProp => \"is-prop\"",
        "FieldBoolPropSource::LegacyProp => \"legacy-prop\"",
        "FieldBoolPropSource::DefaultValue => \"default\"",
        "pub fn resolve_required_source(",
        "pub fn resolve_disabled_source(",
        "pub fn resolve_invalid_source(",
    ] {
        assert!(
            logic_source.contains(required),
            "field logic should expose closed-set source contract `{required}`."
        );
    }

    for required in [
        "pub enum FieldGroupBoolPropSource",
        "FieldGroupBoolPropSource::IsProp => \"is-prop\"",
        "FieldGroupBoolPropSource::LegacyProp => \"legacy-prop\"",
        "FieldGroupBoolPropSource::DefaultValue => \"default\"",
        "pub fn resolve_disabled_source(",
        "pub fn resolve_invalid_source(",
    ] {
        assert!(
            group_logic_source.contains(required),
            "field-group logic should expose closed-set source contract `{required}`."
        );
    }

    for required in [
        "\"invalid-disabled\"",
        "\"invalid\"",
        "\"disabled\"",
        "\"required\"",
        "\"horizontal\"",
        "\"muted\"",
        "\"default\"",
    ] {
        assert!(
            primitive_field_source.contains(required),
            "field primitive state attr should remain enumerable, missing `{required}`."
        );
    }

    for required in [
        "\"invalid-disabled\"",
        "\"invalid\"",
        "\"disabled\"",
        "\"default\"",
        "\"present\"",
        "\"absent\"",
        "\"custom\"",
        "\"label\"",
    ] {
        assert!(
            primitive_group_source.contains(required),
            "field-group primitive state attr should remain enumerable, missing `{required}`."
        );
    }

    assert!(
        check2_source.contains("自动化选择器优先使用 `data-*` / `aria-*` 语义标记"),
        "check2 should document semantic-marker-first selector strategy."
    );
}

#[test]
fn field_styles_depend_on_explicit_semantic_markers_not_fragile_dom_structure() {
    let styles_source = include_str!("../src/styles.rs");
    let group_styles_source = include_str!("../src/group/styles.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        ".ui-field[data-orientation=\"horizontal\"] .ui-field__label",
        ".ui-field[data-tone=\"muted\"]",
        ".ui-field[data-required=\"true\"] .ui-field__label",
        ".ui-field[data-disabled=\"true\"]",
        ".ui-field[data-invalid=\"true\"] .ui-field__control",
        ".ui-field[data-message-kind=\"description\"] .ui-field__description",
        ".ui-field[data-message-kind=\"error\"] .ui-field__error",
        ".ui-field[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "field styles should branch on explicit semantic marker `{required}`."
        );
    }

    for required in [
        ".ui-field-group[data-density=\"comfortable\"]",
        ".ui-field-group[data-density=\"compact\"]",
        ".ui-field-group[data-orientation=\"vertical\"] .ui-field-group__content",
        ".ui-field-group[data-orientation=\"horizontal\"] .ui-field-group__content",
        ".ui-field-group[data-invalid=\"true\"]",
        ".ui-field-group[data-disabled=\"true\"]",
        ".ui-field-group[data-custom-class=\"true\"]",
    ] {
        assert!(
            group_styles_source.contains(required),
            "field-group styles should branch on explicit semantic marker `{required}`."
        );
    }

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        ":first-child",
        ":last-child",
        ":only-child",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "field styles should not rely on fragile DOM-structure selector `{forbidden}`."
        );
        assert!(
            !group_styles_source.contains(forbidden),
            "field-group styles should not rely on fragile DOM-structure selector `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("data-message-kind=move || headless.get().attrs.data_message_kind"),
        "field view should expose message semantic marker for style interpretation."
    );
    assert!(
        view_source.contains("style=move || motion_style.get_value()"),
        "field runtime style should be centralized in motion contract output."
    );
    assert!(
        !group_view_source.contains("style="),
        "field-group view should avoid runtime inline style logic."
    );

    assert!(
        motion_source.contains("format!(\"--ui-field-motion-duration: {duration_ms}ms;\")"),
        "field motion runtime style should emit CSS custom-property payload."
    );
    for forbidden in ["top:", "left:", "width:", "height:", "display:", "color:"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion inline payload should not carry business visual style token `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("样式状态分支仅基于 `data-*`/稳定 class"),
        "check2 should document semantic-marker-first style selector strategy."
    );
}

#[test]
fn field_semantic_contract_tests_cover_matrix_and_do_not_rely_on_visual_snapshots() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let semantics_source = include_str!("../test/semantics.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "role=move || headless.get().attrs.role",
        "aria-disabled=move || headless.get().attrs.aria_disabled",
        "aria-invalid=move || headless.get().attrs.aria_invalid",
        "data-state=move || headless.get().attrs.data_state",
        "data-disabled=move || headless.get().attrs.data_disabled",
        "data-invalid=move || headless.get().attrs.data_invalid",
        "data-required-source=required_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-invalid-source=invalid_source_attr",
    ] {
        assert!(
            view_source.contains(required) || group_view_source.contains(required),
            "semantic contract surface should include `{required}`."
        );
    }

    for required in [
        "fn field_controlled_uncontrolled_pair_rule_is_explicitly_na_for_stateless_axes()",
        "fn field_state_is_observable_and_source_markers_are_stable_and_enumerable()",
        "fn field_a11y_i18n_l10n_contract_is_mounted_and_text_source_is_not_hardcoded_in_view()",
    ] {
        assert!(
            semantics_source.contains(required),
            "semantic matrix should keep regression test `{required}`."
        );
    }

    for forbidden in [
        "on:keydown",
        "on:keyup",
        "on:keypress",
        "on:pointerdown",
        "on:pointerup",
        "on:pointerenter",
        "on:pointerleave",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Field should not introduce unmodeled keyboard/pointer path token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "FieldGroup should not introduce unmodeled keyboard/pointer path token `{forbidden}`."
        );
    }

    for required in [
        "fn field_macro_micro_duality_rule_is_explicitly_na_without_drag_interaction()",
        "fn field_component_public_surface_does_not_leak_dom_runtime_types()",
    ] {
        assert!(
            semantics_source.contains(required),
            "keyboard/pointer/SSR-wasm applicability guard should include `{required}`."
        );
    }

    for forbidden in [
        "insta::assert",
        "assert_snapshot!(",
        "assert_debug_snapshot!(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !group_view_source.contains(forbidden),
            "semantic contract suite should not depend on visual snapshot token `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("视觉快照只作为补充，不作为语义契约通过条件"),
        "check2 should document semantics-first over visual snapshot."
    );
}

#[test]
fn field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let mod_source = include_str!("../src/mod.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let semantics_source = include_str!("../test/semantics.rs");
    let check2_source = include_str!("../check2.md");
    let perf_script_source = include_str!("../../../scripts/check-ui-performance.sh");

    for required in [
        "role=move || headless.get().attrs.role",
        "aria-disabled=move || headless.get().attrs.aria_disabled",
        "aria-invalid=move || headless.get().attrs.aria_invalid",
        "data-state=move || headless.get().attrs.data_state",
        "data-required-source=required_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-invalid-source=invalid_source_attr",
        "data-aria-source=move || headless.get().attrs.data_aria_source",
        "data-error-source=move || headless.get().attrs.data_error_source",
        "data-class-source=move || headless.get().attrs.data_class_source",
    ] {
        assert!(
            view_source.contains(required) || group_view_source.contains(required),
            "field semantic-priority contract should keep aria/data/source marker `{required}`."
        );
    }

    for required in [
        "#[path = \"../test/semantics.rs\"]",
        "mod semantics;",
        "fn field_state_is_observable_and_source_markers_are_stable_and_enumerable()",
        "fn field_a11y_i18n_l10n_contract_is_mounted_and_text_source_is_not_hardcoded_in_view()",
        "fn field_agent_contract_schema_is_machine_readable_and_whitelisted()",
    ] {
        assert!(
            mod_source.contains(required) || semantics_source.contains(required),
            "field should keep local *_semantics.rs contract coverage marker `{required}`."
        );
    }

    for forbidden in [
        "assert_snapshot!(",
        "assert_debug_snapshot!(",
        "insta::assert",
        "pixelmatch",
    ] {
        assert!(
            !view_source.contains(forbidden) && !group_view_source.contains(forbidden),
            "field semantic-priority contract should avoid snapshot-only assertion marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-field field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance gate script should include semantic-priority command `{script_needle}`."
    );

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "components/field/test/semantics.rs",
        "field_state_is_observable_and_source_markers_are_stable_and_enumerable",
        "field_a11y_i18n_l10n_contract_is_mounted_and_text_source_is_not_hardcoded_in_view",
        "field_agent_contract_schema_is_machine_readable_and_whitelisted",
        "field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "scripts/check-ui-performance.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "field/check2.md should keep semantic-test-priority evidence marker `{required}`."
        );
    }
}

#[test]
fn field_file_responsibility_boundaries_are_enforced() {
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let styles_source = include_str!("../src/styles.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");

    let group_mod_source = include_str!("../src/group/mod.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let group_styles_source = include_str!("../src/group/styles.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "#[cfg(feature = \"field-group\")]",
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::FieldMotion;",
        "pub use view::Field;",
    ] {
        assert!(
            mod_source.contains(required),
            "mod.rs should keep minimal export/feature boundary marker `{required}`."
        );
    }

    for forbidden in ["#[component]", "view!", "pub fn resolve_", "pub const CSS"] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not carry implementation token `{forbidden}`."
        );
    }

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::FieldGroup;",
    ] {
        assert!(
            group_mod_source.contains(required),
            "group/mod.rs should keep minimal export boundary marker `{required}`."
        );
    }

    for forbidden in ["#[component]", "view!", "pub fn resolve_", "pub const CSS"] {
        assert!(
            !group_mod_source.contains(forbidden),
            "group/mod.rs should not carry implementation token `{forbidden}`."
        );
    }

    for required in [
        "pub use ui_state_primitives::field::*;",
        "pub fn resolve_content(",
        "pub fn resolve_is_required(",
        "pub fn resolve_is_disabled(",
        "pub fn resolve_is_invalid(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization/derivation marker `{required}`."
        );
    }

    for forbidden in [
        "view!",
        "class=",
        "style=",
        ".ui-field",
        "use_field(",
        "attach_motion(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not carry DOM/style/headless/motion token `{forbidden}`."
        );
    }

    for required in [
        "pub use ui_state_primitives::field_group::*;",
        "pub fn resolve_content(",
        "pub fn resolve_is_disabled(",
        "pub fn resolve_is_invalid(",
    ] {
        assert!(
            group_logic_source.contains(required),
            "group/logic.rs should keep normalization/derivation marker `{required}`."
        );
    }

    for forbidden in [
        "view!",
        "class=",
        "style=",
        ".ui-field-group",
        "use_field_group(",
    ] {
        assert!(
            !group_logic_source.contains(forbidden),
            "group/logic.rs should not carry DOM/style/headless token `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-field",
        "var(--ui-",
        "data-orientation",
        "data-tone",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first static CSS marker `{required}`."
        );
    }

    for forbidden in [
        "#[component]",
        "view!",
        "use_field(",
        "attach_motion(",
        "DEFAULT_ARIA_LABEL",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not carry view/headless/motion/content token `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-field-group",
        "var(--ui-",
        "data-density",
    ] {
        assert!(
            group_styles_source.contains(required),
            "group/styles.rs should keep token-first static CSS marker `{required}`."
        );
    }

    for forbidden in [
        "#[component]",
        "view!",
        "use_field_group(",
        "DEFAULT_ARIA_LABEL",
    ] {
        assert!(
            !group_styles_source.contains(forbidden),
            "group/styles.rs should not carry view/headless/content token `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "logic::resolve_content(logic::FieldContentInput",
        "logic::resolve_state(FieldStateInput",
        "use_field(",
        "motion::attach_motion(",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep structure rendering + headless/motion mount marker `{required}`."
        );
    }

    for forbidden in [
        "normalize_optional_text(",
        "normalize_aria_label(",
        "normalize_error_message(",
        "pub fn resolve_is_required(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not hide logic-level decision token `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "logic::resolve_content(logic::FieldGroupContentInput",
        "logic::resolve_state(FieldGroupStateInput",
        "use_field_group(",
    ] {
        assert!(
            group_view_source.contains(required),
            "group/view.rs should keep structure rendering + headless mount marker `{required}`."
        );
    }

    for forbidden in [
        "normalize_optional_text(",
        "normalize_aria_label(",
        "pub fn resolve_is_disabled(",
    ] {
        assert!(
            !group_view_source.contains(forbidden),
            "group/view.rs should not hide logic-level decision token `{forbidden}`."
        );
    }

    for required in [
        "pub struct FieldMotion",
        "pub spring: ui_motion::spring::SpringConfig,",
        "pub fn sanitize_motion(",
        "pub fn source_attr(",
        "pub fn attach_motion(",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should keep semantic->motion contract mapping marker `{required}`."
        );
    }

    for forbidden in [
        "SpringAnimator",
        "Keyframe",
        "requestAnimationFrame",
        "raf",
        "web_sys::",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not embed shared engine/runtime token `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("本组件落地：`components/field/src/{mod,logic,styles,view,motion}.rs` 与 `components/field/src/group/{mod,logic,styles,view}.rs` 分工单一"),
        "check2 should document file-responsibility boundary evidence."
    );
}

#[test]
fn field_spec_file_scope_is_restricted_for_simple_component() {
    let mod_source = include_str!("../src/mod.rs");
    let group_mod_source = include_str!("../src/group/mod.rs");
    let protocol_source = include_str!("../src/protocol.rs");
    let group_protocol_source = include_str!("../src/group/protocol.rs");
    let check2_source = include_str!("../check2.md");
    let readme_source = include_str!("../src/README.md");

    for forbidden in ["mod spec;", "pub mod spec;", "spec::", "use crate::spec"] {
        assert!(
            !mod_source.contains(forbidden),
            "field mod.rs should not expose spec module token `{forbidden}`."
        );
        assert!(
            !group_mod_source.contains(forbidden),
            "field-group mod.rs should not expose spec module token `{forbidden}`."
        );
    }

    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for relative in ["src/spec.rs", "src/group/spec.rs"] {
        assert!(
            !manifest_dir.join(relative).exists(),
            "simple field component should not introduce `{relative}`."
        );
    }

    for required in [
        "pub enum FieldComponentSchemaVersion",
        "pub struct FieldComponentSpec",
        "schema_version: FieldComponentSchemaVersion",
        "V1",
    ] {
        assert!(
            protocol_source.contains(required),
            "field protocol contract should stay minimal/versioned marker `{required}`."
        );
    }

    for required in [
        "pub enum GroupComponentSchemaVersion",
        "pub struct GroupComponentSpec",
        "schema_version: GroupComponentSchemaVersion",
        "V1",
    ] {
        assert!(
            group_protocol_source.contains(required),
            "field-group protocol contract should stay minimal/versioned marker `{required}`."
        );
    }

    assert!(
        check2_source
            .contains("N/A-by-design：`Field/FieldGroup` 为简单语义装配组件，不引入 `spec.rs`"),
        "check2 should document why spec.rs is intentionally not introduced."
    );
    assert!(
        readme_source.contains("## 快速开始（先用起来）"),
        "simple-component rationale should stay in component docs/checklist, not spec.rs."
    );
}

#[test]
fn field_hyper_structure_builder_spec_is_explicitly_na_for_simple_component() {
    let check2_source = include_str!("../check2.md");
    let mod_source = include_str!("../src/mod.rs");
    let group_mod_source = include_str!("../src/group/mod.rs");
    let protocol_source = include_str!("../src/protocol.rs");
    let group_protocol_source = include_str!("../src/group/protocol.rs");
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    for relative in ["src/spec.rs", "src/group/spec.rs"] {
        assert!(
            !manifest_dir.join(relative).exists(),
            "simple field component should not add hyper-structure builder file `{relative}`."
        );
    }

    for forbidden in [
        "Spec::new(",
        "FieldComponentSpec::new(",
        "GroupComponentSpec::new(",
        ".render()",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "field mod surface should not expose builder token `{forbidden}`."
        );
        assert!(
            !group_mod_source.contains(forbidden),
            "field-group mod surface should not expose builder token `{forbidden}`."
        );
        assert!(
            !protocol_source.contains(forbidden),
            "field protocol should remain schema-only without builder token `{forbidden}`."
        );
        assert!(
            !group_protocol_source.contains(forbidden),
            "field-group protocol should remain schema-only without builder token `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains(
            "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"
        ),
        "field check2 should mark hyper-structure builder checklist item complete."
    );
    assert!(
        check2_source.contains("N/A-by-design"),
        "field check2 should explicitly record N/A-by-design rationale for simple component."
    );
    assert!(
        check2_source
            .contains("field_hyper_structure_builder_spec_is_explicitly_na_for_simple_component"),
        "field check2 should reference hyper-structure builder regression test."
    );
}

#[test]
fn field_token_first_static_styles_are_aggregated_and_not_polluted_by_utility_or_css_in_rust() {
    let styles_source = include_str!("../src/styles.rs");
    let group_styles_source = include_str!("../src/group/styles.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let ui_components_css_source = include_str!("../../../crates/ui/src/css.rs");
    let ui_components_lib_source = include_str!("../../../crates/ui/src/lib.rs");
    let ui_components_cargo = include_str!("../../../crates/ui/Cargo.toml");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        "var(--ui-fallback-",
        ".ui-field",
    ] {
        assert!(
            styles_source.contains(required),
            "field styles should keep token-first static CSS marker `{required}`."
        );
    }

    for required in ["pub const CSS: &str = r#\"", "var(--ui-", ".ui-field-group"] {
        assert!(
            group_styles_source.contains(required),
            "field-group styles should keep token-first static CSS marker `{required}`."
        );
    }

    for forbidden in [
        "styled_components",
        "stylist",
        "emotion",
        "linaria",
        "stylex",
        "css!(",
        "StyleSheet",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "field styles should not adopt css-in-rust token `{forbidden}`."
        );
        assert!(
            !group_styles_source.contains(forbidden),
            "field-group styles should not adopt css-in-rust token `{forbidden}`."
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"text-",
        "class=\"bg-",
        "class=\"p-",
        "class=\"m-",
        "class=\"gap-",
        "class=\"rounded-",
        "class=\"shadow-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view should not be polluted by utility-first class token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should not be polluted by utility-first class token `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("style=move || motion_style.get_value()"),
        "field view should keep runtime style payload delegated to motion contract."
    );
    assert!(
        !group_view_source.contains("style="),
        "field-group view should not introduce runtime inline style logic."
    );
    assert!(
        motion_source.contains("format!(\"--ui-field-motion-duration: {duration_ms}ms;\")"),
        "motion runtime payload should remain CSS custom property only."
    );

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "out.push_str(crate::field_form::field::styles::CSS);",
        "out.push_str(crate::field_form::field::group::styles::CSS);",
    ] {
        assert!(
            ui_components_css_source.contains(required),
            "ui css aggregation should include `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            ui_components_lib_source.contains(required),
            "ui public css injection path should include `{required}`."
        );
    }

    for required in [
        "inject-css = []",
        "component-field = [\"dep:ui-field\"]",
        "component-field_group = [\"component-field\", \"ui-field/field-group\"]",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui feature graph should include `{required}`."
        );
    }

    assert!(
        check2_source.contains(
            "token-first 静态样式由 `styles.rs`/`group/styles.rs` 定义，并经 `crates/ui/src/css.rs` 在 `inject-css` 路径聚合到 `@layer ui`"
        ),
        "check2 should document token-first css aggregation evidence."
    );
}

#[test]
fn field_defensive_variables_use_dual_fallback_chain_without_hex_or_naked_size_literals() {
    let styles_source = include_str!("../src/styles.rs");
    let group_styles_source = include_str!("../src/group/styles.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
    ] {
        assert!(
            styles_source.contains(required) || group_styles_source.contains(required),
            "field styles should keep dual-fallback defensive variable marker `{required}`."
        );
    }

    for forbidden in [
        "#fff",
        "#ffffff",
        "#000",
        "#000000",
        "8rem",
        "14rem",
        "1px solid color-mix(in oklab, var(--ui-accent)",
        "1px solid color-mix(in oklab, var(--ui-border)",
        "outline-offset: 2px;",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "field styles should not keep hardcoded style literal `{forbidden}`."
        );
        assert!(
            !group_styles_source.contains(forbidden),
            "field-group styles should not keep hardcoded style literal `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains(
            "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"
        ),
        "field check2 should mark defensive-variable checklist item complete."
    );
    assert!(
        check2_source.contains(
            "field_defensive_variables_use_dual_fallback_chain_without_hex_or_naked_size_literals"
        ),
        "field check2 should reference defensive-variable regression test."
    );
}

#[test]
fn field_cascade_layer_contract_uses_ui_layer_and_css_variable_only_runtime_updates() {
    let check2_source = include_str!("../check2.md");
    let ui_components_css_source = include_str!("../../../crates/ui/src/css.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let motion_source = include_str!("../src/motion.rs");

    for marker in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "field_cascade_layer_contract_uses_ui_layer_and_css_variable_only_runtime_updates",
    ] {
        assert!(
            check2_source.contains(marker),
            "field checklist should keep cascade-layer contract marker `{marker}`."
        );
    }

    for marker in [
        "#[cfg(feature = \"inject-css\")]",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-field\")]",
        "out.push_str(crate::field_form::field::styles::CSS);",
        "#[cfg(feature = \"component-field_group\")]",
        "out.push_str(crate::field_form::field::group::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            ui_components_css_source.contains(marker),
            "ui css aggregation should keep ui-layer marker `{marker}`."
        );
    }

    assert!(
        view_source.contains("style=move || motion_style.get_value()"),
        "field view should keep runtime style channel bound to motion custom-property payload."
    );
    for forbidden in [
        "style:top",
        "style:left",
        "style:width",
        "style:height",
        "style:position",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view should not set raw inline style token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should not set raw inline style token `{forbidden}`."
        );
    }
    assert!(
        !group_view_source.contains("style="),
        "field-group view should not introduce runtime inline style payload."
    );

    assert!(
        motion_source.contains("format!(\"--ui-field-motion-duration: {duration_ms}ms;\")"),
        "field motion runtime payload should remain CSS custom property only."
    );
    for forbidden in ["top:", "left:", "width:", "height:", "position:"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion payload should not include raw style literal `{forbidden}`."
        );
    }
}

#[test]
fn field_motion_contract_is_component_scoped_and_respects_reduced_motion_with_non_wasm_noop() {
    let check2_source = include_str!("../check2.md");
    let motion_source = include_str!("../src/motion.rs");
    let motion_lib_source = include_str!("../../../crates/ui-motion/src/lib.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_test_source = include_str!("../test/motion.rs");

    for marker in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "field_motion_contract_is_component_scoped_and_respects_reduced_motion_with_non_wasm_noop",
    ] {
        assert!(
            check2_source.contains(marker),
            "field checklist should keep motion-contract marker `{marker}`."
        );
    }

    for marker in [
        "pub struct FieldMotion {",
        "pub duration_ms: f64,",
        "pub spring: ui_motion::spring::SpringConfig,",
        "spring: ui_motion::presets::spring_soft(),",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {",
        "ui_motion::spring::sanitize_config(value, default)",
        "pub fn attach_motion(motion: FieldMotion) -> String {",
        "ui_motion::web::prefers_reduced_motion()",
        "\"--ui-field-motion-stiffness: {};\"",
        "\"--ui-field-motion-damping: {};\"",
        "motion.spring.stiffness",
        "motion.spring.damping",
    ] {
        assert!(
            motion_source.contains(marker),
            "field motion contract should keep marker `{marker}`."
        );
    }

    for forbidden in [
        "SpringAnimator::new(",
        "web_sys::",
        "wasm_bindgen::",
        "js_sys::",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "field motion contract should not embed runtime engine token `{forbidden}`."
        );
    }

    for marker in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib_source.contains(marker),
            "ui-motion should keep non-wasm noop backend marker `{marker}`."
        );
    }

    for marker in [
        "let motion = motion::sanitize_motion(motion);",
        "let motion_source_attr = motion::source_attr(motion);",
        "let motion_style = StoredValue::new(motion::attach_motion(motion));",
        "style=move || motion_style.get_value()",
    ] {
        assert!(
            view_source.contains(marker),
            "field view should mount motion contract marker `{marker}`."
        );
    }

    for marker in [
        "fn attach_motion_outputs_css_variable()",
        "let expected_suffix =",
        "format!(\"--ui-field-motion-duration: 200ms;{expected_suffix}\")",
        "format!(\"--ui-field-motion-duration: 1ms;{expected_suffix}\")",
    ] {
        assert!(
            motion_test_source.contains(marker),
            "field motion regression should keep marker `{marker}`."
        );
    }
}

#[test]
fn field_visual_desire_baseline_is_documented_and_has_interaction_cues() {
    let field_styles_source = include_str!("../src/styles.rs");
    let group_styles_source = include_str!("../src/group/styles.rs");
    let docs_baseline_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs",
    );
    let docs_pages_source = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "Checks first-impression quality: hierarchy, spacing rhythm, contrast layers, and interactive feedback (hover/active/focus).",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            docs_baseline_source.contains(required),
            "docs baseline page should keep visual quality marker `{required}`."
        );
    }

    for required in [
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            docs_pages_source.contains(required),
            "docs routing should include visual baseline entry `{required}`."
        );
    }

    for required in [
        ".ui-field[data-disabled=\"false\"][data-invalid=\"false\"]:hover .ui-field__label",
        ".ui-field[data-disabled=\"false\"][data-invalid=\"false\"]:hover .ui-field__control",
        ".ui-field[data-disabled=\"false\"]:focus-within .ui-field__label",
        ".ui-field[data-disabled=\"false\"][data-invalid=\"false\"]:focus-within .ui-field__control",
        "outline: var(--ui-field-border-width) solid",
        "color-mix(in oklab, var(--ui-field-accent) 52%, var(--ui-field-border) 48%);",
    ] {
        assert!(
            field_styles_source.contains(required),
            "field styles should keep visual desire interaction cue `{required}`."
        );
    }

    for required in [
        ".ui-field-group[data-disabled=\"false\"][data-invalid=\"false\"]:hover",
        ".ui-field-group[data-disabled=\"false\"]:focus-within",
        ".ui-field-group[data-disabled=\"false\"]:focus-within .ui-field-group__label",
        "box-shadow: 0 0 0 var(--ui-field-group-border-width)",
        "color-mix(in oklab, var(--ui-field-group-accent) 26%, transparent);",
    ] {
        assert!(
            group_styles_source.contains(required),
            "field-group styles should keep visual desire interaction cue `{required}`."
        );
    }

    assert!(
        check2_source
            .contains("field_visual_desire_baseline_is_documented_and_has_interaction_cues"),
        "check2 should reference visual desire regression test evidence."
    );
}

#[test]
fn field_tree_shaking_contract_is_feature_gated_and_budget_guarded() {
    let check2_source = include_str!("../check2.md");
    let ui_components_cargo = include_str!("../../../crates/ui/Cargo.toml");
    let ui_components_lib = include_str!("../../../crates/ui/src/lib.rs");
    let ui_components_css = include_str!("../../../crates/ui/src/css.rs");
    let web_demo_cargo = include_str!("../../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = include_str!("../../../apps/docs-app/Cargo.toml");
    let tree_shaking_script = include_str!("../../../scripts/check-ui-tree-shaking.sh");
    let tree_shaking_budget = include_str!("../../../scripts/tree_shaking_budget.env");
    let ci_source = include_str!("../../../.github/workflows/ci.yml");

    for required in [
        "default = [\"inject-css\", \"all-components\"]",
        "web-demo-components = [",
        "all-components = [",
        "component-field = [\"dep:ui-field\"]",
        "component-field_group = [\"component-field\", \"ui-field/field-group\"]",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui feature table should keep tree-shaking marker `{required}`."
        );
    }

    for required in [
        "#[cfg(any(",
        "feature = \"component-field\",",
        "pub mod field_form {",
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]",
        "pub use web_demo_components::*;",
        "#[cfg(feature = \"all-components\")]",
        "pub use all_components::*;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib export boundary should keep `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-field\")]",
        "out.push_str(crate::field_form::field::styles::CSS);",
        "#[cfg(feature = \"component-field_group\")]",
        "out.push_str(crate::field_form::field::group::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css aggregation should stay feature-gated via `{required}`."
        );
    }

    assert!(
        web_demo_cargo.contains(
            "ui = { path = \"../../crates/ui\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }"
        ),
        "web-demo should consume ui in source-mode bundle without default features."
    );
    assert!(
        !web_demo_cargo.contains("\"all-components\""),
        "web-demo should not implicitly pull all-components."
    );
    assert!(
        docs_app_cargo.contains(
            "ui = { path = \"../../crates/ui\", default-features = false, features = [\"inject-css\", \"all-components\"] }"
        ),
        "docs-app should explicitly opt into all-components as full showcase surface."
    );

    for required in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "cargo tree -e features -i ui -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
    ] {
        assert!(
            tree_shaking_script.contains(required),
            "tree-shaking gate script should keep `{required}`."
        );
    }

    for required in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            tree_shaking_budget.contains(required),
            "tree-shaking budget file should keep `{required}`."
        );
    }

    assert!(
        ci_source.contains("Tree Shaking Budget")
            && ci_source.contains("./scripts/check-ui-tree-shaking.sh"),
        "CI should run tree-shaking budget gate script."
    );

    assert!(
        check2_source.contains(
            "- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"
        ),
        "field check2 should mark tree-shaking checklist item complete."
    );
    assert!(
        check2_source.contains("field_tree_shaking_contract_is_feature_gated_and_budget_guarded"),
        "field check2 should reference tree-shaking regression test."
    );
}

#[test]
fn field_ui_components_entry_files_follow_fixed_layered_contract() {
    let check2_source = include_str!("../check2.md");
    let ui_components_lib_source = include_str!("../../../crates/ui/src/lib.rs");
    let ui_components_css_source = include_str!("../../../crates/ui/src/css.rs");
    let ui_components_root_source = include_str!("../../../crates/ui/src/root.rs");
    let active_highlight_source =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let ui_components_src_root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui/src");
    let ui_headless_src_root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui-headless/src");

    for marker in [
        "- [x] `ui` 固定入口文件落点正确。",
        "field_ui_components_entry_files_follow_fixed_layered_contract",
    ] {
        assert!(
            check2_source.contains(marker),
            "field checklist should keep ui entry contract marker `{marker}`."
        );
    }

    for marker in [
        "#[cfg(any(",
        "feature = \"component-field\",",
        "pub mod field_form {",
        "#[cfg(feature = \"component-field\")]",
        "pub use ui_field as field;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "css::push_components_css(out);",
    ] {
        assert!(
            ui_components_lib_source.contains(marker),
            "ui lib entry should keep marker `{marker}`."
        );
    }
    for forbidden in [
        "pub use web_sys",
        "pub use leptos::web_sys",
        "pub use wasm_bindgen",
    ] {
        assert!(
            !ui_components_lib_source.contains(forbidden),
            "ui public API should not expose platform token `{forbidden}`."
        );
    }

    for marker in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-field\")]",
        "out.push_str(crate::field_form::field::styles::CSS);",
        "#[cfg(feature = \"component-field_group\")]",
        "out.push_str(crate::field_form::field::group::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css_source.contains(marker),
            "ui css entry should keep marker `{marker}`."
        );
    }

    for marker in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root_source.contains(marker),
            "UiRoot entry should keep centralized injection marker `{marker}`."
        );
    }

    for marker in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion {",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(marker),
            "active_highlight shared primitive should keep marker `{marker}`."
        );
    }
    for forbidden in ["Field", "Accordion", "Popover", "Tooltip"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight primitive should avoid component-business token `{forbidden}`."
        );
    }

    for forbidden_path in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src_root.join(forbidden_path).exists(),
            "ui src should not host duplicated headless primitive `{forbidden_path}`."
        );
    }

    for required_path in ["controllable_state.rs", "presence.rs", "a11y.rs"] {
        assert!(
            ui_headless_src_root.join(required_path).exists(),
            "ui-headless should host shared primitive `{required_path}`."
        );
    }
}

#[test]
fn field_component_directory_standard_file_layout_is_correct() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let check2_source = include_str!("../check2.md");
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let styles_source = include_str!("../src/styles.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let group_mod_source = include_str!("../src/group/mod.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let group_styles_source = include_str!("../src/group/styles.rs");
    let group_view_source = include_str!("../src/group/view.rs");

    for marker in [
        "- [x] 组件目录标准文件落点正确。",
        "field_component_directory_standard_file_layout_is_correct",
    ] {
        assert!(
            check2_source.contains(marker),
            "field checklist should keep component-directory marker `{marker}`."
        );
    }

    for relative in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
        "src/group/mod.rs",
        "src/group/logic.rs",
        "src/group/styles.rs",
        "src/group/view.rs",
    ] {
        assert!(
            root.join(relative).exists(),
            "field component directory should include `{relative}`."
        );
    }

    for forbidden in [
        "src/render.rs",
        "src/spec.rs",
        "src/group/render.rs",
        "src/group/spec.rs",
    ] {
        assert!(
            !root.join(forbidden).exists(),
            "field component directory should not include `{forbidden}`."
        );
    }

    for required in [
        "#[cfg(feature = \"field-group\")]",
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::FieldMotion;",
        "pub use view::Field;",
    ] {
        assert!(
            mod_source.contains(required),
            "field mod.rs should keep minimal export marker `{required}`."
        );
    }
    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub use view::*;",
        "pub use logic::*;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "field mod.rs should avoid over-export token `{forbidden}`."
        );
    }

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::FieldGroup;",
    ] {
        assert!(
            group_mod_source.contains(required),
            "field-group mod.rs should keep minimal export marker `{required}`."
        );
    }
    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub use view::*;",
        "pub use logic::*;",
        "pub mod motion;",
    ] {
        assert!(
            !group_mod_source.contains(forbidden),
            "field-group mod.rs should avoid over-export or invalid surface `{forbidden}`."
        );
    }

    for required in [
        "pub fn resolve_content(",
        "pub fn resolve_is_required(",
        "pub fn resolve_is_disabled(",
        "pub fn resolve_is_invalid(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization/derivation marker `{required}`."
        );
    }
    for forbidden in ["view! {", "use_field(", ".ui-field", "attach_motion("] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should avoid view/headless/style/motion token `{forbidden}`."
        );
    }

    for required in ["pub const CSS: &str = r#\"", "var(--ui-", ".ui-field"] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep static token-first css marker `{required}`."
        );
    }
    for forbidden in ["#[component]", "view! {", "use_field("] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should avoid view/headless token `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn Field(",
        "use_field(",
        "motion::attach_motion(",
        "logic::resolve_content(logic::FieldContentInput",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep structure render + semantic mount marker `{required}`."
        );
    }
    for forbidden in ["pub fn resolve_is_required(", "pub fn resolve_is_disabled("] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not hide logic decision token `{forbidden}`."
        );
    }

    for required in [
        "pub struct FieldMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should keep component motion contract marker `{required}`."
        );
    }
    for forbidden in ["use_field(", "view! {", "SpringAnimator::new("] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should avoid view/headless/engine token `{forbidden}`."
        );
    }

    for required in [
        "pub fn resolve_content(",
        "pub fn resolve_is_disabled(",
        "pub fn resolve_is_invalid(",
    ] {
        assert!(
            group_logic_source.contains(required),
            "group/logic.rs should keep normalization/derivation marker `{required}`."
        );
    }
    for required in ["pub const CSS: &str = r#\"", "var(--ui-", ".ui-field-group"] {
        assert!(
            group_styles_source.contains(required),
            "group/styles.rs should keep static token-first css marker `{required}`."
        );
    }
    for required in [
        "#[component]",
        "pub fn FieldGroup(",
        "use_field_group(",
        "logic::resolve_content(logic::FieldGroupContentInput",
    ] {
        assert!(
            group_view_source.contains(required),
            "group/view.rs should keep structure render + semantic mount marker `{required}`."
        );
    }
}

#[test]
fn field_file_placement_discipline_contract_is_enforced() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let check2_source = include_str!("../check2.md");
    let semantics_source = include_str!("../test/semantics.rs");
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let styles_source = include_str!("../src/styles.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");

    for marker in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "field_file_placement_discipline_contract_is_enforced",
    ] {
        assert!(
            check2_source.contains(marker),
            "field checklist should keep file-placement discipline marker `{marker}`."
        );
    }

    for relative in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        assert!(
            root.join(relative).exists(),
            "field file-placement contract should include `{relative}`."
        );
    }
    for forbidden in [
        "src/render.rs",
        "src/spec.rs",
        "src/group/render.rs",
        "src/group/spec.rs",
    ] {
        assert!(
            !root.join(forbidden).exists(),
            "field file-placement contract should forbid `{forbidden}`."
        );
    }

    assert!(
        semantics_source.contains("field_component_directory_standard_file_layout_is_correct"),
        "file-placement discipline should be anchored by directory responsibility regression."
    );

    for required in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Field;",
    ] {
        assert!(
            mod_source.contains(required),
            "field mod.rs should keep constrained export marker `{required}`."
        );
    }
    for required in [
        "pub fn resolve_content(",
        "pub use ui_state_primitives::field::*;",
    ] {
        assert!(
            logic_source.contains(required),
            "field logic.rs should keep normalization marker `{required}`."
        );
    }
    assert!(
        styles_source.contains("pub const CSS: &str = r#\"") && styles_source.contains("var(--ui-"),
        "field styles.rs should keep token-first static CSS contract."
    );
    for required in ["#[component]", "pub fn Field(", "use_field("] {
        assert!(
            view_source.contains(required),
            "field view.rs should keep render + headless mount marker `{required}`."
        );
    }
    for required in ["pub struct FieldMotion", "pub fn attach_motion("] {
        assert!(
            motion_source.contains(required),
            "field motion.rs should keep motion-contract marker `{required}`."
        );
    }
}

#[test]
fn field_type_system_and_semantic_markers_form_machine_readable_contract() {
    let check2_source = include_str!("../check2.md");
    let primitive_field_source = include_str!("../../../crates/ui-state-primitives/src/field.rs");
    let primitive_group_source =
        include_str!("../../../crates/ui-state-primitives/src/field_group.rs");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let semantics_source = include_str!("../test/semantics.rs");

    for required in [
        "pub enum FieldOrientation",
        "pub enum FieldTone",
        "pub enum FieldGroupOrientation",
        "pub enum FieldGroupDensity",
    ] {
        assert!(
            primitive_field_source.contains(required) || primitive_group_source.contains(required),
            "discrete state axis should remain enum-typed `{required}`."
        );
    }

    for required in [
        "pub fn resolve_content(",
        "pub use ui_state_primitives::field::*;",
        "pub enum FieldBoolPropSource",
        "pub enum FieldGroupBoolPropSource",
    ] {
        assert!(
            logic_source.contains(required) || group_logic_source.contains(required),
            "logic layer should keep typed normalization/state markers `{required}`."
        );
    }

    for required in [
        "data-state=move || headless.get().attrs.data_state",
        "data-required-source=required_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-invalid-source=invalid_source_attr",
    ] {
        assert!(
            view_source.contains(required) || group_view_source.contains(required),
            "view layer should expose machine-readable semantic marker `{required}`."
        );
    }

    for required in [
        "fn field_discrete_state_axes_use_enum_types_not_string_protocols()",
        "fn field_state_is_observable_and_source_markers_are_stable_and_enumerable()",
    ] {
        assert!(
            semantics_source.contains(required),
            "semantic suite should keep contract regression test `{required}`."
        );
    }

    assert!(
        check2_source
            .contains("- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。"),
        "field check2 should mark type-system + semantic-marker checklist item complete."
    );
    assert!(
        check2_source
            .contains("field_type_system_and_semantic_markers_form_machine_readable_contract"),
        "field check2 should reference this regression test."
    );
}

#[test]
fn field_hydration_discontinuity_contract_uses_id_provider_without_time_or_random_sources() {
    let group_view_source = include_str!("../src/group/view.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let group_primitive_source =
        include_str!("../../../crates/ui-state-primitives/src/field_group.rs");
    let id_provider_source = include_str!("../../../crates/ui-headless/src/id_provider.rs");
    let ui_root_source = include_str!("../../../crates/ui/src/root.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "use_ui_id_provider",
        "let id_base = id_base.or_else(||",
        "next_prefixed_id(logic::DEFAULT_ID_BASE)",
    ] {
        assert!(
            group_view_source.contains(required),
            "field-group view should keep deterministic id-provider contract `{required}`."
        );
    }

    assert!(
        group_primitive_source.contains("pub const DEFAULT_ID_BASE: &str = \"ui-field-group\";"),
        "field-group primitive should keep a stable default id base constant."
    );
    assert!(
        id_provider_source.contains("pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider")
            && id_provider_source.contains("pub fn use_ui_id_provider() -> Option<UiIdProvider>"),
        "ui-headless should expose id-provider injection + consumption contract."
    );
    assert!(
        ui_root_source.contains("provide_ui_id_provider(id_seed);"),
        "UiRoot should inject deterministic id seed into id-provider."
    );

    for forbidden in [
        "now(",
        "Date::now",
        "Math::random",
        "randomUUID",
        "Uuid::new",
        "uuid::",
        "rand::",
    ] {
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view must not derive hydration ids from time/random source `{forbidden}`."
        );
        assert!(
            !group_logic_source.contains(forbidden),
            "field-group logic must not derive hydration ids from time/random source `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains(
            "- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。"
        ),
        "field check2 should mark hydration discontinuity checklist item complete."
    );
    assert!(
        check2_source.contains(
            "field_hydration_discontinuity_contract_uses_id_provider_without_time_or_random_sources"
        ),
        "field check2 should reference hydration discontinuity regression test."
    );
}

#[test]
fn field_ssr_and_cross_platform_compile_contract_is_explicit_and_non_wasm_safe() {
    let check_script_source = include_str!("../../../scripts/check.sh");
    let headless_lib_source = include_str!("../../../crates/ui-headless/src/lib.rs");
    let motion_lib_source = include_str!("../../../crates/ui-motion/src/lib.rs");
    let mod_source = include_str!("../src/mod.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "echo \"[check] full feature matrix (native, dev)\"",
        "cargo check -p ui --no-default-features --features inject-css,dev-all-components",
        "echo \"[check] ssr (compile-only)\"",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "echo \"[check] wasm\"",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features inject-css,dev-all-components",
    ] {
        assert!(
            check_script_source.contains(required),
            "compile-only gate script should keep `{required}`."
        );
    }

    assert!(
        mod_source.contains("#[cfg(feature = \"field-group\")]"),
        "field component module should keep explicit feature-gated platform boundary."
    );
    assert!(
        motion_lib_source.contains("#[cfg(target_arch = \"wasm32\")]")
            && motion_lib_source.contains("#[cfg(not(target_arch = \"wasm32\"))]"),
        "ui-motion backend should keep explicit wasm/non-wasm cfg split."
    );
    assert!(
        headless_lib_source.contains("#[cfg(all(feature = \"web\", feature = \"ssr\"))]")
            && headless_lib_source
                .contains("compile_error!(\"features `web` and `ssr` are mutually exclusive;"),
        "ui-headless should keep explicit web/ssr feature exclusivity guard."
    );

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "js_sys::",
        "window.",
        "document.",
        "Document",
        "Window",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view should remain non-wasm safe and avoid browser token `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should remain non-wasm safe and avoid browser token `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "field motion should remain non-wasm safe and avoid browser token `{forbidden}`."
        );
    }

    assert!(
        check2_source
            .contains("- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。"),
        "field check2 should mark SSR/cross-platform checklist item complete."
    );
    assert!(
        check2_source.contains(
            "field_ssr_and_cross_platform_compile_contract_is_explicit_and_non_wasm_safe"
        ),
        "field check2 should reference SSR/cross-platform regression test."
    );
}

#[test]
fn field_headless_web_ssr_feature_mutex_is_compile_error_guarded() {
    let headless_lib_source = include_str!("../../../crates/ui-headless/src/lib.rs");
    let check_script_source = include_str!("../../../scripts/check.sh");
    let check2_source = include_str!("../check2.md");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            headless_lib_source.contains(required),
            "ui-headless web/ssr mutex guard should keep `{required}`."
        );
    }

    for required in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            check_script_source.contains(required),
            "check script should keep separate web/ssr compile-only path `{required}`."
        );
    }

    for forbidden in [
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "cargo check -p ui-headless --no-default-features --features ssr,web",
    ] {
        assert!(
            !check_script_source.contains(forbidden),
            "check script must not treat simultaneous web+ssr as valid path `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains(
            "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。"
        ),
        "field check2 should mark ui-headless web/ssr mutex checklist item complete."
    );
    assert!(
        check2_source.contains("field_headless_web_ssr_feature_mutex_is_compile_error_guarded"),
        "field check2 should reference ui-headless web/ssr mutex regression test."
    );
}

#[test]
fn field_motion_non_wasm_noop_stub_contract_is_predictable_and_tooling_safe() {
    let motion_lib_source = include_str!("../../../crates/ui-motion/src/lib.rs");
    let field_motion_source = include_str!("../src/motion.rs");
    let check_script_source = include_str!("../../../scripts/check.sh");
    let check2_source = include_str!("../check2.md");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib_source.contains(required),
            "ui-motion should keep non-wasm predictable noop marker `{required}`."
        );
    }

    for required in [
        "pub spring: ui_motion::spring::SpringConfig,",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {",
        "ui_motion::spring::sanitize_config(value, default)",
        "pub fn attach_motion(motion: FieldMotion) -> String {",
        "ui_motion::web::prefers_reduced_motion()",
        "format!(\"--ui-field-motion-duration: {duration_ms}ms;\")",
        "\"--ui-field-motion-stiffness: {};\"",
        "\"--ui-field-motion-damping: {};\"",
        "motion.spring.stiffness",
        "motion.spring.damping",
    ] {
        assert!(
            field_motion_source.contains(required),
            "field motion mapping should keep non-wasm-safe attach contract `{required}`."
        );
    }

    for forbidden in [
        "panic!",
        "unwrap(",
        "expect(",
        "web_sys::",
        "wasm_bindgen::",
        "js_sys::",
    ] {
        assert!(
            !field_motion_source.contains(forbidden),
            "field motion should avoid runtime-coupled or panic-prone token `{forbidden}`."
        );
    }

    for required in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui --no-default-features --features inject-css,dev-all-components",
    ] {
        assert!(
            check_script_source.contains(required),
            "tooling/ssr compile-only gate should keep `{required}`."
        );
    }

    assert!(
        check2_source.contains(
            "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。"
        ),
        "field check2 should mark ui-motion non-wasm noop/stub checklist item complete."
    );
    assert!(
        check2_source
            .contains("field_motion_non_wasm_noop_stub_contract_is_predictable_and_tooling_safe"),
        "field check2 should reference ui-motion non-wasm noop/stub regression test."
    );
}

#[test]
fn field_reduced_motion_ssr_and_wasm_branch_contract_keeps_semantics_consistent() {
    let field_motion_source = include_str!("../src/motion.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let motion_test_source = include_str!("../test/motion.rs");
    let check_script_source = include_str!("../../../scripts/check.sh");
    let check2_source = include_str!("../check2.md");
    let semantics_source = include_str!("../test/semantics.rs");

    for required in [
        "ui_motion::web::prefers_reduced_motion()",
        "let duration_ms = if ui_motion::web::prefers_reduced_motion() {",
        "1.0",
        "format!(\"--ui-field-motion-duration: {duration_ms}ms;\")",
        "\"--ui-field-motion-stiffness: {};\"",
        "\"--ui-field-motion-damping: {};\"",
        "motion.spring.stiffness",
        "motion.spring.damping",
    ] {
        assert!(
            field_motion_source.contains(required),
            "field motion should keep reduced-motion downgrade marker `{required}`."
        );
    }

    for required in [
        "fn attach_motion_outputs_css_variable()",
        "if cfg!(target_arch = \"wasm32\")",
        "let expected_suffix =",
        "--ui-field-motion-stiffness: 200;--ui-field-motion-damping: 16;",
        "format!(\"--ui-field-motion-duration: 200ms;{expected_suffix}\")",
        "format!(\"--ui-field-motion-duration: 1ms;{expected_suffix}\")",
    ] {
        assert!(
            motion_test_source.contains(required),
            "field motion test should lock wasm/non-wasm branch behavior `{required}`."
        );
    }

    for required in [
        "data-state=move || headless.get().attrs.data_state",
        "data-required=move || headless.get().attrs.data_required",
        "data-disabled=move || headless.get().attrs.data_disabled",
        "data-invalid=move || headless.get().attrs.data_invalid",
    ] {
        assert!(
            view_source.contains(required) || group_view_source.contains(required),
            "field semantic surface should remain headless-driven across platforms `{required}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "#[cfg(feature = \"ssr\")]",
        "#[cfg(feature = \"web\")]",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view should avoid platform-splitting semantic branch `{forbidden}`."
        );
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view should avoid platform-splitting semantic branch `{forbidden}`."
        );
    }

    for required in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            check_script_source.contains(required),
            "compile-only gate should keep SSR/wasm branch check `{required}`."
        );
    }

    for required in [
        "fn field_hydration_discontinuity_contract_uses_id_provider_without_time_or_random_sources()",
        "fn field_ssr_and_cross_platform_compile_contract_is_explicit_and_non_wasm_safe()",
        "fn field_motion_non_wasm_noop_stub_contract_is_predictable_and_tooling_safe()",
    ] {
        assert!(
            semantics_source.contains(required),
            "branch contract should stay connected with regression suite `{required}`."
        );
    }

    assert!(
        check2_source.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。"),
        "field check2 should mark reduced-motion/SSR/wasm checklist item complete."
    );
    assert!(
        check2_source.contains(
            "field_reduced_motion_ssr_and_wasm_branch_contract_keeps_semantics_consistent"
        ),
        "field check2 should reference reduced-motion/SSR/wasm regression test."
    );
}

#[test]
fn field_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = include_str!("../../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = include_str!("../../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = include_str!("../../../docs/plan/TODO.md");
    let script_source = include_str!("../../../scripts/check-ui-performance.sh");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"field\" => UiPerfBudget {",
        "\"field-group\" => UiPerfBudget {",
        "max_update_ms: Some(8.0),",
        "max_update_ms: Some(10.0),",
        "max_heap_kb: Some(384.0),",
        "max_heap_kb: Some(512.0),",
    ] {
        assert!(
            shell_source.contains(required),
            "docs shell should keep field perf budget token `{required}`."
        );
    }

    for required in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
    ] {
        assert!(
            perf_probe_source.contains(required),
            "UiPerfProbe should expose performance regression marker `{required}`."
        );
    }

    for required in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_source.contains(required),
            "docs coverage e2e should enforce repeatable perf regression guard `{required}`."
        );
    }

    for required in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(required),
            "performance governance should keep render_count follow-up marker `{required}`."
        );
    }

    for required in [
        "cargo test -p ui-field field_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(required),
            "performance gate script should include `{required}`."
        );
    }

    for required in [
        "data-state=move || headless.get().attrs.data_state",
        "data-motion-source=motion_source_attr",
        "data-required-source=required_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-invalid-source=invalid_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "Field view should expose attribution marker `{required}` for perf triage."
        );
    }

    for required in [
        "data-state=move || headless.get().attrs.data_state",
        "data-disabled-source=disabled_source_attr",
        "data-invalid-source=invalid_source_attr",
        "data-class-source=move || headless.get().attrs.data_class_source",
    ] {
        assert!(
            group_view_source.contains(required),
            "FieldGroup view should expose attribution marker `{required}` for perf triage."
        );
    }

    for required in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
        "field_performance_governance_contract_is_budgeted_traceable_and_blocking",
    ] {
        assert!(
            check2_source.contains(required),
            "Field checklist should keep performance governance marker `{required}`."
        );
    }
}

#[test]
fn field_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement() {
    let semantics_source = include_str!("../test/semantics.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let field_styles_source = include_str!("../src/styles.rs");
    let group_styles_source = include_str!("../src/group/styles.rs");
    let perf_script_source = include_str!("../../../scripts/check-ui-performance.sh");
    let todo_source = include_str!("../../../docs/plan/TODO.md");
    let check2_source = include_str!("../check2.md");

    for required in [
        "fn field_semantic_contract_tests_cover_matrix_and_do_not_rely_on_visual_snapshots()",
        "fn field_state_is_observable_and_source_markers_are_stable_and_enumerable()",
        "fn field_a11y_i18n_l10n_contract_is_mounted_and_text_source_is_not_hardcoded_in_view()",
        "fn field_performance_governance_contract_is_budgeted_traceable_and_blocking()",
    ] {
        assert!(
            semantics_source.contains(required),
            "semantic/perf suite should retain `{required}`.",
        );
    }

    for required in [
        "aria-disabled=move || headless.get().attrs.aria_disabled",
        "aria-invalid=move || headless.get().attrs.aria_invalid",
        "data-state=move || headless.get().attrs.data_state",
        "data-required-source=required_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-invalid-source=invalid_source_attr",
    ] {
        assert!(
            view_source.contains(required) || group_view_source.contains(required),
            "view contract should keep aria/data semantic marker `{required}`.",
        );
    }

    for required in [
        ".ui-field[data-disabled=\"false\"]:focus-within .ui-field__label",
        ".ui-field[data-disabled=\"false\"][data-invalid=\"false\"]:focus-within .ui-field__control",
    ] {
        assert!(
            field_styles_source.contains(required),
            "Field focus-flow style contract should keep `{required}`.",
        );
    }

    for required in [
        ".ui-field-group[data-disabled=\"false\"]:focus-within",
        ".ui-field-group[data-disabled=\"false\"]:focus-within .ui-field-group__label",
    ] {
        assert!(
            group_styles_source.contains(required),
            "FieldGroup focus-flow style contract should keep `{required}`.",
        );
    }

    for forbidden in [
        "insta::assert",
        "assert_snapshot!(",
        "assert_debug_snapshot!(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !group_view_source.contains(forbidden),
            "semantic/perf regression gate should not rely on snapshot token `{forbidden}`.",
        );
    }

    for required in [
        "cargo test -p ui-field field_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-field field_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script_source.contains(required),
            "performance gate script should keep `{required}`.",
        );
    }

    for required in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(required) || check2_source.contains(required),
            "render_count evidence should keep marker `{required}`.",
        );
    }

    for required in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "field_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            check2_source.contains(required),
            "Field checklist should keep semantics/performance marker `{required}`.",
        );
    }
}

#[test]
fn field_version_deprecation_migration_is_not_required_without_major_breaking_upgrade() {
    let check2_source = include_str!("../check2.md");
    let module_source = include_str!("../src/mod.rs");
    let group_mod_source = include_str!("../src/group/mod.rs");
    let component_toml_source = include_str!("../src/Component.toml");
    let rbi_source = include_str!("../src/field.rbi");
    let protocol_source = include_str!("../src/protocol.rs");
    let group_protocol_source = include_str!("../src/group/protocol.rs");
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let styles_source = include_str!("../src/styles.rs");
    let engineering_script_source = include_str!("../../../scripts/check-ui-engineering.sh");

    for required in [
        "pub use motion::FieldMotion;",
        "pub use view::Field;",
        "pub use view::FieldGroup;",
    ] {
        assert!(
            module_source.contains(required) || group_mod_source.contains(required),
            "public API export should remain stable without major-version migration `{required}`.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "pub enum FieldComponentSchemaVersion",
        "V1",
        "pub enum GroupComponentSchemaVersion",
        "V1",
    ] {
        assert!(
            component_toml_source.contains(required)
                || protocol_source.contains(required)
                || group_protocol_source.contains(required),
            "field protocol should remain on v1 contract marker `{required}`.",
        );
    }

    for source in [
        component_toml_source,
        rbi_source,
        protocol_source,
        group_protocol_source,
        logic_source,
        view_source,
        motion_source,
        styles_source,
    ] {
        for forbidden in [
            "migrate_v1_to_v2",
            "deprecation_window",
            "schema_registry",
            "Schema Registry",
        ] {
            assert!(
                !source.contains(forbidden),
                "component sources should not introduce migration scaffolding without major-break trigger (`{forbidden}`).",
            );
        }
    }

    {
        let required = "cargo test -p ui-field field_version_deprecation_migration_is_not_required_without_major_breaking_upgrade";
        assert!(
            engineering_script_source.contains(required),
            "engineering gate script should include `{required}`.",
        );
    }

    for required in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A-by-scope：本次 `components/field` 变更未引入跨大版本 API 破坏升级",
        "`components/field/src/mod.rs` 与 `components/field/src/group/mod.rs` 公共导出面保持稳定",
        "`components/field/src/Component.toml` 仍为 `schema_version = \"1\"`；`components/field/src/{protocol.rs,group/protocol.rs}` 仍保持 `FieldComponentSchemaVersion::V1` / `GroupComponentSchemaVersion::V1`",
        "因此无需登记 Schema Registry 弃用窗口，也无需新增 `migrate_v1_to_v2` 迁移函数",
        "回归：`components/field/test/semantics.rs::field_version_deprecation_migration_is_not_required_without_major_breaking_upgrade`。",
    ] {
        assert!(
            check2_source.contains(required),
            "field check2 should retain version deprecation/migration evidence `{required}`.",
        );
    }
}

#[test]
fn field_view_macro_complexity_is_split_into_semantic_blocks() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "fn render_field_label(",
        "fn render_field_description(",
        "fn render_field_error(",
        "render_field_label(",
        "render_field_description(",
        "render_field_error(",
    ] {
        assert!(
            view_source.contains(required),
            "field view should split semantic subrenders via `{required}`."
        );
    }

    for required in [
        "fn render_group_label(",
        "fn render_group_description(",
        "render_group_label(",
        "render_group_description(",
    ] {
        assert!(
            group_view_source.contains(required),
            "field-group view should split semantic subrenders via `{required}`."
        );
    }

    assert_eq!(
        view_source.matches("view! {").count(),
        5,
        "field view should keep one root render block plus semantic subrender blocks."
    );
    assert_eq!(
        group_view_source.matches("view! {").count(),
        3,
        "field-group view should keep one root render block plus two semantic subrender blocks."
    );

    assert!(
        check2_source.contains(
            "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。"
        ),
        "field check2 should mark view-macro complexity checklist item complete."
    );
    assert!(
        check2_source.contains("field_view_macro_complexity_is_split_into_semantic_blocks"),
        "field check2 should reference view-macro complexity regression test."
    );
}

#[test]
fn field_prefers_functional_subview_splitting_over_extra_components() {
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "fn render_field_label(",
        "fn render_field_description(",
        "fn render_field_error(",
        ") -> impl IntoView {",
    ] {
        assert!(
            view_source.contains(required),
            "field view should keep lightweight subfragments as plain functions via `{required}`."
        );
    }

    for required in [
        "fn render_group_label(",
        "fn render_group_description(",
        ") -> impl IntoView {",
    ] {
        assert!(
            group_view_source.contains(required),
            "field-group view should keep lightweight subfragments as plain functions via `{required}`."
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "field view should keep only the root Field as #[component]."
    );
    assert_eq!(
        group_view_source.matches("#[component]").count(),
        1,
        "field-group view should keep only the root FieldGroup as #[component]."
    );

    for forbidden in [
        "#[component]\nfn render_field_label(",
        "#[component]\nfn render_field_description(",
        "#[component]\nfn render_field_error(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field view helper subfragments should not be upgraded to #[component]: `{forbidden}`."
        );
    }

    for forbidden in [
        "#[component]\nfn render_group_label(",
        "#[component]\nfn render_group_description(",
    ] {
        assert!(
            !group_view_source.contains(forbidden),
            "field-group view helper subfragments should not be upgraded to #[component]: `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains(
            "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。"
        ),
        "field check2 should mark functional subview split checklist item complete."
    );
    assert!(
        check2_source.contains("field_prefers_functional_subview_splitting_over_extra_components"),
        "field check2 should reference functional subview split regression test."
    );
}

#[test]
fn field_static_fragments_are_constantized_with_stable_a11y_markup() {
    let view_source = include_str!("../src/view.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "const REQUIRED_INDICATOR_TEXT: &str = \"*\";",
        "const REQUIRED_INDICATOR_ARIA_HIDDEN: &str = \"true\";",
        "fn render_required_indicator() -> impl IntoView {",
        "aria-hidden=REQUIRED_INDICATOR_ARIA_HIDDEN",
        "{REQUIRED_INDICATOR_TEXT}",
        "{render_required_indicator()}",
    ] {
        assert!(
            view_source.contains(required),
            "field view should keep static fragment constantization marker `{required}`."
        );
    }

    assert!(
        !view_source.contains("aria-hidden=\"true\">\n                        \"*\""),
        "field view should avoid inlined required-indicator literal fragment."
    );

    assert!(
        check2_source.contains(
            "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。"
        ),
        "field check2 should mark static-fragment constantization checklist item complete."
    );
    assert!(
        check2_source.contains("field_static_fragments_are_constantized_with_stable_a11y_markup"),
        "field check2 should reference static-fragment constantization regression test."
    );
}

#[test]
fn field_inner_html_usage_is_forbidden_without_explicit_static_whitelist_contract() {
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let motion_source = include_str!("../src/motion.rs");
    let styles_source = include_str!("../src/styles.rs");
    let mod_source = include_str!("../src/mod.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let check2_source = include_str!("../check2.md");

    let component_sources = [
        ("view.rs", view_source),
        ("logic.rs", logic_source),
        ("motion.rs", motion_source),
        ("styles.rs", styles_source),
        ("mod.rs", mod_source),
        ("group/view.rs", group_view_source),
        ("group/logic.rs", group_logic_source),
    ];

    for (name, source) in component_sources {
        for forbidden in [
            "inner_html",
            "innerHTML",
            "set_inner_html(",
            "setInnerHTML(",
            "dangerously_set_inner_html",
            "from_html_unchecked",
            "HtmlElement::set_inner_html",
            "Element::set_inner_html",
        ] {
            assert!(
                !source.contains(forbidden),
                "{name} should not include unsafe raw-html injection token `{forbidden}`."
            );
        }
    }

    assert!(
        check2_source.contains(
            "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。"
        ),
        "field check2 should mark inner_html safety checklist item complete."
    );
    assert!(
        check2_source.contains("N/A-by-design"),
        "field check2 should explicitly record N/A-by-design rationale when inner_html is unused."
    );
    assert!(
        check2_source.contains(
            "field_inner_html_usage_is_forbidden_without_explicit_static_whitelist_contract"
        ),
        "field check2 should reference inner_html safety regression test."
    );
}

#[test]
fn field_wasm_debug_contract_is_explicitly_na_with_dev_only_observability_entry() {
    let field_cargo_source = include_str!("../Cargo.toml");
    let mod_source = include_str!("../src/mod.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let docs_app_source = include_str!("../../../apps/docs-app/src/lib.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "data-state=move || headless.get().attrs.data_state",
        "data-required-source=required_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-invalid-source=invalid_source_attr",
    ] {
        assert!(
            view_source.contains(required) || group_view_source.contains(required),
            "field wasm-debug observability baseline should keep marker `{required}`."
        );
    }

    for required in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(required),
            "docs-app should keep dev-only wasm debug observability entry `{required}`."
        );
    }

    assert!(
        field_cargo_source.contains("[features]\ndefault = []\nfield-group = []"),
        "ui-field crate should keep minimal feature surface without forced debug runtime in production."
    );
    assert!(
        !field_cargo_source.contains("debug"),
        "ui-field crate should not leak debug feature/runtime into public package surface."
    );

    for forbidden in [
        "pub fn enable_debug",
        "pub fn replay_trace",
        "pub fn debug_trace",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "field public API must not expose debug-only entry `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains(
            "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。"
        ),
        "field check2 should mark wasm debug checklist item complete."
    );
    assert!(
        check2_source.contains("N/A-by-design"),
        "field check2 should keep N/A-by-design rationale for non-interactive replay path."
    );
    assert!(
        check2_source.contains(
            "field_wasm_debug_contract_is_explicitly_na_with_dev_only_observability_entry"
        ),
        "field check2 should reference wasm debug regression test."
    );
}

#[test]
fn field_dx_contract_keeps_css_fast_feedback_context_persistence_and_workbench_canvas() {
    let forms_extra_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/forms_extra.rs",
    );
    let check2_source = include_str!("../check2.md");

    for required in [
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "slug=\"field\"",
        "test_css_source=workbench_test_css",
        "let workbench_test_css = Signal::derive(move || {",
        "data-slot=\"field-workbench-controls\"",
        "data-slot=\"field-workbench-summary\"",
    ] {
        assert!(
            forms_extra_source.contains(required),
            "field docs workbench should keep DX fast-feedback marker `{required}`."
        );
    }

    for required in [
        "const FIELD_WORKBENCH_STORAGE_KEY: &str = \"docs:field:workbench:v1\";",
        "fn load_field_workbench_state() -> Option<FieldWorkbenchState>",
        "fn save_field_workbench_state(state: FieldWorkbenchState)",
        "fn clear_field_workbench_state()",
        "let (workbench_persist_state, set_workbench_persist_state) =",
        "if workbench_persist_state.get() {",
        "<Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>",
        "\"Persist workbench state\"",
    ] {
        assert!(
            forms_extra_source.contains(required),
            "field docs workbench should keep context-persistence marker `{required}`."
        );
    }

    assert!(
        check2_source.contains(
            "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。"
        ),
        "field check2 should mark DX checklist item complete."
    );
    assert!(
        check2_source.contains(
            "field_dx_contract_keeps_css_fast_feedback_context_persistence_and_workbench_canvas"
        ),
        "field check2 should reference DX regression test."
    );
}

#[test]
fn field_engineering_capability_contract_is_serde_versioned_trace_aligned_and_runtime_agnostic() {
    let field_cargo_source = include_str!("../Cargo.toml");
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let protocol_source = include_str!("../src/protocol.rs");
    let group_protocol_source = include_str!("../src/group/protocol.rs");
    let protocol_test_source = include_str!("../test/protocol.rs");
    let group_protocol_test_source = include_str!("../test/group/protocol.rs");
    let forms_extra_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/forms_extra.rs",
    );
    let docs_app_source = include_str!("../../../apps/docs-app/src/lib.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "serde = { version = \"1.0\", features = [\"derive\"] }",
        "pub enum FieldComponentSchemaVersion",
        "pub struct FieldComponentSpec",
        "schema_version: FieldComponentSchemaVersion",
        "pub enum GroupComponentSchemaVersion",
        "pub struct GroupComponentSpec",
        "schema_version: GroupComponentSchemaVersion",
    ] {
        assert!(
            field_cargo_source.contains(required)
                || protocol_source.contains(required)
                || group_protocol_source.contains(required),
            "engineering capability should keep serde/versioned protocol marker `{required}`."
        );
    }

    for required in [
        "assert_serde::<FieldComponentSchemaVersion>();",
        "assert_serde::<FieldComponentSpec>();",
        "assert_serde::<GroupComponentSchemaVersion>();",
        "assert_serde::<GroupComponentSpec>();",
    ] {
        assert!(
            protocol_test_source.contains(required)
                || group_protocol_test_source.contains(required),
            "protocol regression should keep serde trait contract marker `{required}`."
        );
    }

    for required in [
        "const FIELD_WORKBENCH_STORAGE_VERSION: u8 = 1;",
        "struct FieldWorkbenchStorage {",
        "version: u8,",
        "enum FieldWorkbenchStorageError {",
        "Serialize(serde_json::Error)",
        "Deserialize(serde_json::Error)",
        "UnsupportedVersion(u8)",
        "fn as_code(&self) -> &'static str {",
        "field workbench decode failed: code={} error={error:?}",
        "field workbench encode failed: code={} error={error:?}",
    ] {
        assert!(
            forms_extra_source.contains(required),
            "field workbench config path should keep structured serde/error marker `{required}`."
        );
    }

    for required in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
    ] {
        assert!(
            docs_app_source.contains(required),
            "trace semantics should stay aligned with shared ui_trace contract `{required}`."
        );
    }

    let component_sources = [
        mod_source,
        logic_source,
        view_source,
        motion_source,
        group_logic_source,
        group_view_source,
    ];

    for source in component_sources {
        for forbidden in [
            "tokio::",
            "tokio_",
            "async_std::",
            "async-std",
            "tracing::",
            "#[instrument",
            "tracing_subscriber",
            "Runtime",
            "JoinHandle",
        ] {
            assert!(
                !source.contains(forbidden),
                "field component should avoid leaking runtime-specific engine token `{forbidden}`."
            );
        }
    }

    assert!(
        !field_cargo_source.contains("tokio") && !field_cargo_source.contains("async-std"),
        "ui-field crate should not bind async boundary to a specific runtime dependency."
    );

    assert!(
        check2_source.contains(
            "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。"
        ),
        "field check2 should mark engineering capability checklist item complete."
    );
    assert!(
        check2_source.contains("N/A-by-design"),
        "field check2 should explicitly keep N/A-by-design rationale for async runtime boundary."
    );
    assert!(
        check2_source.contains(
            "field_engineering_capability_contract_is_serde_versioned_trace_aligned_and_runtime_agnostic"
        ),
        "field check2 should reference engineering capability regression test."
    );
}

#[test]
fn field_context_compression_manifest_and_rbi_projection_are_present_and_consistent() {
    let manifest_source = include_str!("../src/Component.toml");
    let rbi_source = include_str!("../src/field.rbi");
    let check2_source = include_str!("../check2.md");

    for required in [
        "id = \"ui-field\"",
        "name = \"Field\"",
        "crate = \"ui-field\"",
        "rbi = \"field.rbi\"",
        "context_compression_manifest = true",
        "rbi_signature_projection = true",
        "fallback = \"snapshot\"",
        "owner = \"upstream\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should keep context-compression marker `{required}`."
        );
    }

    for required in [
        "component \"ui-field\" {",
        "crate: \"ui-field\"",
        "mode: \"snapshot\"",
        "\"Field\"",
        "\"FieldMotion\"",
        "\"FieldOrientation\"",
        "\"FieldTone\"",
        "\"A11yDirection\"",
        "\"FieldComponentSchemaVersion\"",
        "\"FieldComponentSpec\"",
        "signature Field(",
        "signature FieldGroup(",
        "[feature=\"field-group\"]",
        "agent_contract_schema \"ui.field.agent-contract/v1\"",
    ] {
        assert!(
            rbi_source.contains(required),
            "field.rbi should keep signature projection marker `{required}`."
        );
    }

    assert!(
        check2_source.contains(
            "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"
        ),
        "field check2 should mark context-compression checklist item complete."
    );
    assert!(
        check2_source.contains(
            "field_context_compression_manifest_and_rbi_projection_are_present_and_consistent"
        ),
        "field check2 should reference context-compression regression test."
    );
}

#[test]
fn field_agent_contract_schema_is_machine_readable_and_whitelisted() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let manifest_source = include_str!("../src/Component.toml");
    let rbi_source = include_str!("../src/field.rbi");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub const FIELD_AGENT_SCHEMA: &str = \"ui.field.agent-contract/v1\";",
        "pub struct FieldAgentContract {",
        "pub fn resolve_agent_contract(",
        "pub enum FieldAgentIntent",
        "pub enum FieldAgentAction",
        "pub enum FieldAgentState",
        "pub enum FieldAgentSource",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep typed field agent-contract marker `{required}`."
        );
    }

    for required in [
        "pub const FIELD_GROUP_AGENT_SCHEMA: &str = \"ui.field.agent-contract/v1\";",
        "pub struct FieldGroupAgentContract {",
        "pub fn resolve_agent_contract(",
        "pub enum FieldGroupAgentIntent",
        "pub enum FieldGroupAgentAction",
        "pub enum FieldGroupAgentState",
        "pub enum FieldGroupAgentSource",
    ] {
        assert!(
            group_logic_source.contains(required),
            "group/logic.rs should keep typed field-group agent-contract marker `{required}`."
        );
    }

    for required in [
        "logic::resolve_agent_contract(",
        "data-ui-schema=move || agent_contract.get().schema",
        "data-ui-intent=move || agent_contract.get().intent",
        "data-ui-action=move || agent_contract.get().action",
        "data-ui-state=move || agent_contract.get().state",
        "data-ui-source=move || agent_contract.get().source",
        "data-ui-source-required=move || agent_contract.get().source_required",
        "data-ui-source-disabled=move || agent_contract.get().source_disabled",
        "data-ui-source-invalid=move || agent_contract.get().source_invalid",
        "data-ui-source-motion=move || agent_contract.get().source_motion",
        "data-ui-source-aria=move || agent_contract.get().source_aria",
        "data-ui-source-error=move || agent_contract.get().source_error",
        "data-ui-source-class=move || agent_contract.get().source_class",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode",
        "data-ui-stream-support=move || agent_contract.get().stream_support",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback",
        "data-ui-output-mode=move || agent_contract.get().output_mode",
        "data-ui-output-status=move || agent_contract.get().output_status",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should mount typed field agent marker `{required}`."
        );
    }

    for required in [
        "logic::resolve_agent_contract(state.get(), disabled_source, invalid_source)",
        "data-ui-schema=move || agent_contract.get().schema",
        "data-ui-intent=move || agent_contract.get().intent",
        "data-ui-action=move || agent_contract.get().action",
        "data-ui-state=move || agent_contract.get().state",
        "data-ui-source=move || agent_contract.get().source",
        "data-ui-source-disabled=move || agent_contract.get().source_disabled",
        "data-ui-source-invalid=move || agent_contract.get().source_invalid",
        "data-ui-source-aria=move || agent_contract.get().source_aria",
        "data-ui-source-class=move || agent_contract.get().source_class",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode",
        "data-ui-stream-support=move || agent_contract.get().stream_support",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback",
        "data-ui-output-mode=move || agent_contract.get().output_mode",
        "data-ui-output-status=move || agent_contract.get().output_status",
    ] {
        assert!(
            group_view_source.contains(required),
            "group/view.rs should mount typed field-group agent marker `{required}`."
        );
    }

    for required in [
        "schema = \"ui.field.agent-contract/v1\"",
        "\"data-ui-schema\"",
        "\"data-ui-intent\"",
        "\"data-ui-action\"",
        "\"data-ui-state\"",
        "\"data-ui-source\"",
        "\"data-ui-stream-mode\"",
        "\"data-ui-stream-support\"",
        "\"data-ui-stream-fallback\"",
        "\"data-ui-output-mode\"",
        "\"data-ui-output-status\"",
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "allowed = [",
        "blocked = [\"inner_html\", \"dangerously_set_inner_html\", \"<script\", \"javascript:\"]",
    ] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should keep agent-contract schema/whitelist marker `{required}`."
        );
    }

    for required in [
        "agent_contract_schema \"ui.field.agent-contract/v1\"",
        "\"data-ui-schema\"",
        "\"data-ui-intent\"",
        "\"data-ui-action\"",
        "\"data-ui-state\"",
        "\"data-ui-source\"",
        "\"data-ui-stream-mode\"",
        "\"data-ui-stream-support\"",
        "\"data-ui-stream-fallback\"",
        "\"data-ui-output-mode\"",
        "\"data-ui-output-status\"",
        "whitelist \"render_path\" {",
        "blocked: [\"inner_html\", \"dangerously_set_inner_html\", \"<script\", \"javascript:\"]",
    ] {
        assert!(
            rbi_source.contains(required),
            "field.rbi should keep agent-contract projection/whitelist marker `{required}`."
        );
    }

    for source in [view_source, group_view_source] {
        for forbidden in [
            "inner_html",
            "dangerously_set_inner_html",
            "<script",
            "javascript:",
        ] {
            assert!(
                !source.contains(forbidden),
                "agent contract mount path should keep script injection boundary `{forbidden}`."
            );
        }
    }

    assert!(
        check2_source.contains(
            "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。"
        ),
        "field check2 should mark agent-contract schema checklist item complete."
    );
    assert!(
        check2_source.contains("field_agent_contract_schema_is_machine_readable_and_whitelisted"),
        "field check2 should reference agent-contract schema regression test."
    );
}

#[test]
fn field_llm_rendering_modes_are_strictly_streaming_or_snapshot_only() {
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let manifest_source = include_str!("../src/Component.toml");
    let rbi_source = include_str!("../src/field.rbi");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub const FIELD_LLM_RENDER_MODES: [&str; 2] = [\"streaming\", \"snapshot\"];",
        "pub const FIELD_DEFAULT_RENDER_MODE: &str = \"snapshot\";",
        "pub stream_mode: &'static str,",
        "pub output_mode: &'static str,",
        "stream_mode: FIELD_DEFAULT_RENDER_MODE,",
        "stream_fallback: FIELD_DEFAULT_RENDER_MODE,",
        "output_mode: FIELD_DEFAULT_RENDER_MODE,",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep strict two-mode render marker `{required}`."
        );
    }

    for required in [
        "pub const FIELD_GROUP_LLM_RENDER_MODES: [&str; 2] = [\"streaming\", \"snapshot\"];",
        "pub const FIELD_GROUP_DEFAULT_RENDER_MODE: &str = \"snapshot\";",
        "pub stream_mode: &'static str,",
        "pub output_mode: &'static str,",
        "stream_mode: FIELD_GROUP_DEFAULT_RENDER_MODE,",
        "stream_fallback: FIELD_GROUP_DEFAULT_RENDER_MODE,",
        "output_mode: FIELD_GROUP_DEFAULT_RENDER_MODE,",
    ] {
        assert!(
            group_logic_source.contains(required),
            "group/logic.rs should keep strict two-mode render marker `{required}`."
        );
    }

    for required in [
        "data-ui-stream-mode=move || agent_contract.get().stream_mode",
        "data-ui-output-mode=move || agent_contract.get().output_mode",
    ] {
        assert!(
            view_source.contains(required) && group_view_source.contains(required),
            "view layers should expose strict stream/output mode marker `{required}`."
        );
    }

    for required in [
        "[llm_render_modes]",
        "allowed = [\"streaming\", \"snapshot\"]",
        "default = \"snapshot\"",
        "\"data-ui-stream-mode\"",
        "\"data-ui-output-mode\"",
        "\"stream.mode\"",
        "\"output.mode\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should keep two-mode LLM render contract marker `{required}`."
        );
    }

    for required in [
        "llm_render_modes {",
        "allowed: [\"streaming\", \"snapshot\"]",
        "default: \"snapshot\"",
        "\"data-ui-stream-mode\"",
        "\"data-ui-output-mode\"",
        "\"stream.mode\"",
        "\"output.mode\"",
    ] {
        assert!(
            rbi_source.contains(required),
            "field.rbi should keep two-mode LLM render contract marker `{required}`."
        );
    }

    assert!(
        check2_source.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
        "field check2 should mark two-mode LLM rendering checklist item complete."
    );
    assert!(
        check2_source.contains("field_llm_rendering_modes_are_strictly_streaming_or_snapshot_only"),
        "field check2 should reference two-mode LLM rendering regression test."
    );
}

#[test]
fn field_snapshot_is_foundational_default_and_consumes_complete_results_stably() {
    let manifest_source = include_str!("../src/Component.toml");
    let rbi_source = include_str!("../src/field.rbi");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let protocol_source = include_str!("../src/protocol.rs");
    let group_protocol_source = include_str!("../src/group/protocol.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "kind = \"snapshot\"",
        "snapshot = true",
        "streaming = false",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should keep snapshot-foundation marker `{required}`."
        );
    }

    for required in [
        "mode: \"snapshot\"",
        "fallback: \"snapshot\"",
        "default: \"snapshot\"",
        "signature Field(",
        "signature FieldGroup(",
    ] {
        assert!(
            rbi_source.contains(required),
            "field.rbi should keep snapshot projection marker `{required}`."
        );
    }

    for required in [
        "pub const FIELD_DEFAULT_RENDER_MODE: &str = \"snapshot\";",
        "stream_mode: FIELD_DEFAULT_RENDER_MODE,",
        "stream_fallback: FIELD_DEFAULT_RENDER_MODE,",
        "output_mode: FIELD_DEFAULT_RENDER_MODE,",
        "pub fn resolve_content(input: FieldContentInput) -> FieldContent",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep snapshot-default complete-config marker `{required}`."
        );
    }

    for required in [
        "pub const FIELD_GROUP_DEFAULT_RENDER_MODE: &str = \"snapshot\";",
        "stream_mode: FIELD_GROUP_DEFAULT_RENDER_MODE,",
        "stream_fallback: FIELD_GROUP_DEFAULT_RENDER_MODE,",
        "output_mode: FIELD_GROUP_DEFAULT_RENDER_MODE,",
        "pub fn resolve_content(input: FieldGroupContentInput) -> FieldGroupContent",
    ] {
        assert!(
            group_logic_source.contains(required),
            "group/logic.rs should keep snapshot-default complete-config marker `{required}`."
        );
    }

    for required in [
        "data-ui-output-mode=move || agent_contract.get().output_mode",
        "data-ui-output-status=move || agent_contract.get().output_status",
    ] {
        assert!(
            view_source.contains(required) && group_view_source.contains(required),
            "view layers should expose stable snapshot output marker `{required}`."
        );
    }

    for required in [
        "pub struct FieldComponentSpec",
        "pub schema_version: FieldComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(required),
            "protocol.rs should keep versioned complete-config snapshot contract `{required}`."
        );
    }

    for required in [
        "pub struct GroupComponentSpec",
        "pub schema_version: GroupComponentSchemaVersion",
    ] {
        assert!(
            group_protocol_source.contains(required),
            "group/protocol.rs should keep versioned complete-config snapshot contract `{required}`."
        );
    }

    for source in [
        view_source,
        group_view_source,
        logic_source,
        group_logic_source,
    ] {
        for forbidden in ["delta", "chunk", "partial", "token_stream"] {
            assert!(
                !source.contains(forbidden),
                "snapshot baseline path should not depend on partial-stream token `{forbidden}`."
            );
        }
    }

    assert!(
        check2_source.contains("- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。"),
        "field check2 should mark snapshot-foundation checklist item complete."
    );
    assert!(
        check2_source.contains(
            "field_snapshot_is_foundational_default_and_consumes_complete_results_stably"
        ),
        "field check2 should reference snapshot-foundation regression test."
    );
}

#[test]
fn field_streaming_requirement_is_role_based_optional_with_snapshot_fallback_and_continuous_status_markers()
 {
    let manifest_source = include_str!("../src/Component.toml");
    let rbi_source = include_str!("../src/field.rbi");
    let logic_source = include_str!("../src/logic.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "[streaming_policy]",
        "required = false",
        "fallback = \"snapshot\"",
        "owner = \"upstream\"",
        "streaming = false",
        "[output_state]",
        "allowed = [\"draft\", \"verified\", \"committable\"]",
    ] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should keep role-based streaming-optional marker `{required}`."
        );
    }

    for required in [
        "streaming_policy {",
        "required: false",
        "fallback: \"snapshot\"",
        "owner: \"upstream\"",
        "\"stream.support\"",
        "\"stream.fallback\"",
        "\"output.status\"",
    ] {
        assert!(
            rbi_source.contains(required),
            "field.rbi should keep streaming policy/output status marker `{required}`."
        );
    }

    for required in [
        "pub enum FieldAgentStreamSupport",
        "FieldAgentStreamSupport::Optional => \"optional\"",
        "pub enum FieldAgentOutputStatus",
        "FieldAgentOutputStatus::Verified => \"verified\"",
        "stream_support: stream_support.as_data_attr()",
        "stream_fallback: FIELD_DEFAULT_RENDER_MODE",
        "output_status: output_status.as_data_attr()",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep streaming-optional/status marker `{required}`."
        );
    }

    for required in [
        "pub enum FieldGroupAgentStreamSupport",
        "FieldGroupAgentStreamSupport::Optional => \"optional\"",
        "pub enum FieldGroupAgentOutputStatus",
        "FieldGroupAgentOutputStatus::Verified => \"verified\"",
        "stream_support: stream_support.as_data_attr()",
        "stream_fallback: FIELD_GROUP_DEFAULT_RENDER_MODE",
        "output_status: output_status.as_data_attr()",
    ] {
        assert!(
            group_logic_source.contains(required),
            "group/logic.rs should keep streaming-optional/status marker `{required}`."
        );
    }

    for required in [
        "role=\"alert\"",
        "data-ui-stream-support=move || agent_contract.get().stream_support",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback",
        "data-ui-output-status=move || agent_contract.get().output_status",
        "aria-label=move || headless.get().attrs.aria_label",
        "aria-disabled=move || headless.get().attrs.aria_disabled",
        "aria-invalid=move || headless.get().attrs.aria_invalid",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep continuous role/aria/data marker `{required}`."
        );
    }

    for required in [
        "role=move || headless.get().attrs.role",
        "data-ui-stream-support=move || agent_contract.get().stream_support",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback",
        "data-ui-output-status=move || agent_contract.get().output_status",
        "aria-label=move || aria_label_value.get()",
        "aria-disabled=move || headless.get().attrs.aria_disabled",
        "aria-invalid=move || headless.get().attrs.aria_invalid",
    ] {
        assert!(
            group_view_source.contains(required),
            "group/view.rs should keep continuous role/aria/data marker `{required}`."
        );
    }

    for source in [
        view_source,
        group_view_source,
        logic_source,
        group_logic_source,
    ] {
        for forbidden in [
            "retry",
            "reconnect",
            "backoff",
            "resume",
            "network_error",
            "transport_error",
        ] {
            assert!(
                !source.contains(forbidden),
                "component layer should not own retry/disconnect policy token `{forbidden}`."
            );
        }
    }

    assert!(
        check2_source.contains("- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。"),
        "field check2 should mark role-based streaming requirement checklist item complete."
    );
    assert!(
        check2_source.contains(
            "field_streaming_requirement_is_role_based_optional_with_snapshot_fallback_and_continuous_status_markers"
        ),
        "field check2 should reference streaming requirement regression test."
    );
}

#[test]
fn field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_with_cow_string_hotspots() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let group_logic_source = include_str!("../src/group/logic.rs");
    let group_view_source = include_str!("../src/group/view.rs");
    let styles_source = include_str!("../src/styles.rs");
    let mod_source = include_str!("../src/mod.rs");
    let check2_source = include_str!("../check2.md");

    let component_sources = [
        ("logic.rs", logic_source),
        ("view.rs", view_source),
        ("motion.rs", motion_source),
        ("group/logic.rs", group_logic_source),
        ("group/view.rs", group_view_source),
        ("styles.rs", styles_source),
        ("mod.rs", mod_source),
    ];

    for (name, source) in component_sources {
        for forbidden in [".unwrap(", ".unwrap_err(", ".expect(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "{name} should not contain rust-hygiene forbidden token `{forbidden}`."
            );
        }
    }

    for required in [
        "use std::borrow::Cow;",
        "type FieldCowStr = Cow<'static, str>;",
        "fn normalize_optional_cow(",
        "fn normalize_aria_label_cow(",
        "fn normalize_error_message_cow(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep Cow-based string hotspot marker `{required}`."
        );
    }

    for required in [
        "use std::borrow::Cow;",
        "type FieldGroupCowStr = Cow<'static, str>;",
        "fn normalize_optional_cow(",
        "fn normalize_id_base_cow(",
        "fn normalize_aria_label_cow(",
    ] {
        assert!(
            group_logic_source.contains(required),
            "group/logic.rs should keep Cow-based string hotspot marker `{required}`."
        );
    }

    for required in [
        "StoredValue<Cow<'static, str>>",
        "content.class_name.map(std::borrow::Cow::into_owned)",
        "content.lang.map(std::borrow::Cow::into_owned)",
    ] {
        assert!(
            view_source.contains(required) || group_view_source.contains(required),
            "view layers should keep Cow bridge marker `{required}`."
        );
    }

    assert!(
        check2_source.contains(
            "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。"
        ),
        "field check2 should mark rust hygiene checklist item complete."
    );
    assert!(
        check2_source.contains(
            "field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_with_cow_string_hotspots"
        ),
        "field check2 should reference rust hygiene regression test."
    );
}

#[test]
fn field_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "e2e/tests/docs_app_field_contract.spec.mjs",
        "body:not(:has(#boot))",
        "data-slot=\"field\"",
        "ready/settled",
        "field_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "field_e2e_flow_covers_ready_and_settled_semantic_breakpoints",
        "components/field/scripts/check-ui-e2e-field.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "field/check2.md should keep e2e selector and stable wait marker `{required}`."
        );
    }
}

#[test]
fn field_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_field_contract.spec.mjs");
    let docs_page_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/forms_extra.rs",
    );

    for required in [
        "/#/components/field",
        "body:not(:has(#boot))",
        "[data-component=\"field\"]",
        "[data-slot=\"field-state-matrix\"]",
        "[data-slot=\"field\"][data-required=\"true\"][data-required-source=\"legacy-prop\"][data-message-kind=\"description\"]",
        "[data-slot=\"field\"][data-invalid=\"true\"][data-invalid-source=\"legacy-prop\"][data-message-kind=\"error\"]",
        "[data-slot=\"field\"][data-disabled=\"true\"][data-disabled-source=\"legacy-prop\"]",
    ] {
        assert!(
            e2e_source.contains(required),
            "field e2e selector contract should include semantic marker `{required}`."
        );
    }

    for required in [
        "data-action=\"field-workbench-toggle-invalid\"",
        "data-action=\"field-workbench-toggle-disabled\"",
        "data-action=\"field-workbench-motion-ms\"",
    ] {
        assert!(
            docs_page_source.contains(required),
            "docs field workbench should expose stable semantic control marker `{required}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
        "nth-child(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "field e2e selector contract should avoid unstable token `{forbidden}`."
        );
    }
}

#[test]
fn field_e2e_flow_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_field_contract.spec.mjs");
    let check_script_source =
        include_str!("../../../components/field/scripts/check-ui-e2e-field.sh");

    for required in [
        "docs-app field motion path uses semantic ready/settled breakpoints",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "toHaveAttribute(\"data-state\", \"invalid\")",
        "toHaveAttribute(\"data-state\", \"invalid-disabled\")",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-state\", \"required\")",
        "toHaveAttribute(\"data-motion-source\", \"custom\")",
        "toHaveAttribute(\"style\", /--ui-field-motion-duration:",
    ] {
        assert!(
            e2e_source.contains(required),
            "field e2e flow should keep semantic ready/settled marker `{required}`."
        );
    }

    for required in [
        "cargo test -p ui-field field_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-field field_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "cargo test -p ui-field field_e2e_flow_covers_ready_and_settled_semantic_breakpoints",
    ] {
        assert!(
            check_script_source.contains(required),
            "field e2e check script should include gate command `{required}`."
        );
    }
}

#[test]
fn field_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "e2e/tests/docs_app_field_contract.spec.mjs",
        "docs-app field key flow is repeatable with semantic failure breakpoints",
        "focus/keyboard",
        "本组件无 overlay 与 async 分支（当前 N/A）",
        "field_e2e_flow_is_repeatable_and_failure_points_are_semantic",
        "field_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "components/field/scripts/check-ui-e2e-field.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "field/check2.md should keep repeatable e2e flow marker `{required}`."
        );
    }
}

#[test]
fn field_e2e_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_field_contract.spec.mjs");

    for required in [
        "docs-app field key flow is repeatable with semantic failure breakpoints",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "await expect(fieldInput).toBeFocused();",
        "await expect(invalidToggle).toBeFocused();",
        "await expect(disabledToggle).toBeFocused();",
        "await page.keyboard.press(\"Space\");",
        "toHaveAttribute(\"data-state\", \"invalid\")",
        "toHaveAttribute(\"data-state\", \"invalid-disabled\")",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-state\", \"required\")",
        "await page.reload();",
        "await runRepeatableFieldKeyboardFlow(page, docsRootAfterReload);",
    ] {
        assert!(
            e2e_source.contains(required),
            "field repeatable e2e flow should include semantic breakpoint `{required}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep(", "nth-child("] {
        assert!(
            !e2e_source.contains(forbidden),
            "field repeatable e2e flow should avoid unstable token `{forbidden}`."
        );
    }
}

#[test]
fn field_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_field_contract.spec.mjs");
    let check2_source = include_str!("../check2.md");
    let check_script_source =
        include_str!("../../../components/field/scripts/check-ui-e2e-field.sh");

    for required in [
        "await expect(fieldInput).toBeFocused();",
        "await expect(invalidToggle).toBeFocused();",
        "await expect(disabledToggle).toBeFocused();",
        "await page.keyboard.press(\"Space\");",
        "toHaveAttribute(\"data-state\", \"invalid-disabled\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(required),
            "field high-risk e2e path should include semantic breakpoint `{required}`."
        );
    }

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "本组件无 overlay 与 async 分支（当前 N/A）",
        "field_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "components/field/scripts/check-ui-e2e-field.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "field/check2.md should keep high-risk e2e evidence marker `{required}`."
        );
    }

    for required in [
        "cargo test -p ui-field field_e2e_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui-field field_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            check_script_source.contains(required),
            "field e2e check script should include `{required}`."
        );
    }
}

#[test]
fn field_check2_documents_interactive_playground_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs::field",
        "field-workbench-controls",
        "field_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "field_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "field_dx_check_script_covers_interactive_playground_contract",
        "field_e2e_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-dx.sh",
        "components/field/scripts/check-ui-e2e-field.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "field/check2.md should keep interactive-playground evidence `{required}`."
        );
    }
}

#[test]
fn field_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_field_page_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/forms_extra.rs",
    );
    let e2e_source = include_str!("../../../e2e/tests/docs_app_field_contract.spec.mjs");

    for required in [
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "data-slot=\"field-workbench-controls\"",
        "data-action=\"field-workbench-orientation\"",
        "data-action=\"field-workbench-tone\"",
        "data-action=\"field-workbench-motion-ms\"",
        "data-action=\"field-workbench-toggle-required\"",
        "data-action=\"field-workbench-toggle-invalid\"",
        "data-action=\"field-workbench-toggle-disabled\"",
        "data-action=\"field-workbench-toggle-custom-class\"",
        "data-slot=\"field-workbench-summary\"",
        "test_config_signal=workbench_actual_config",
    ] {
        assert!(
            docs_field_page_source.contains(required),
            "field docs page should keep interactive-playground marker `{required}`."
        );
    }

    for required in [
        "docs-app field interactive playground supports realtime props/state preview",
        "await orientationSelect.selectOption(\"horizontal\");",
        "await toneSelect.selectOption(\"muted\");",
        "await customClassToggle.check();",
        "await expect(summary).toContainText(\"orientation=horizontal\");",
        "await expect(summary).toContainText(\"tone=muted\");",
        "await expect(summary).toContainText(\"custom_class=true\");",
    ] {
        assert!(
            e2e_source.contains(required),
            "field interactive playground e2e should include `{required}`."
        );
    }
}

#[test]
fn field_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_field_contract.spec.mjs");

    for required in [
        "async function runRepeatableFieldKeyboardFlow(page, docsRoot)",
        "docs-app field key flow is repeatable with semantic failure breakpoints",
        "docs-app field interactive playground supports realtime props/state preview",
        "await runRepeatableFieldKeyboardFlow(page, docsRoot);",
        "await runRepeatableFieldKeyboardFlow(page, docsRootAfterReload);",
        "await runRepeatableFieldKeyboardFlow(page, docsRoot);",
    ] {
        assert!(
            e2e_source.contains(required),
            "field interactive playground should reuse repeatable semantic e2e flow `{required}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep(", "nth-child("] {
        assert!(
            !e2e_source.contains(forbidden),
            "field interactive playground e2e should avoid unstable token `{forbidden}`."
        );
    }
}

#[test]
fn field_dx_check_script_covers_interactive_playground_contract() {
    let dx_script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "cargo test -p ui-field field_check2_documents_interactive_playground_rules",
        "cargo test -p ui-field field_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-field field_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            dx_script_source.contains(required),
            "field dx script should cover interactive-playground command `{required}`."
        );
    }
}

#[test]
fn field_e2e_check_script_covers_interactive_playground_contract() {
    let e2e_script_source = include_str!("../../../components/field/scripts/check-ui-e2e-field.sh");

    for required in [
        "cargo test -p ui-field field_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "cargo test -p ui-field field_e2e_check_script_covers_interactive_playground_contract",
    ] {
        assert!(
            e2e_script_source.contains(required),
            "field e2e script should cover interactive-playground command `{required}`."
        );
    }
}

#[cfg(feature = "field-group")]
#[test]
fn field_group_mounts_headless_contract_with_component_local_assembly_only() {
    let group_logic_source = include_str!("../src/group/logic.rs");
    let group_view_source = include_str!("../src/group/view.rs");

    assert!(
        group_logic_source.contains("pub use ui_state_primitives::field_group::*;"),
        "group/logic.rs should consume field_group primitives from ui-state-primitives."
    );
    assert!(
        group_view_source.contains("use_field_group("),
        "group/view.rs should mount headless group contract in view layer."
    );
    assert!(
        !group_view_source.contains("ui_state_primitives::field_group"),
        "group/view.rs should avoid bypassing logic layer to consume primitives directly."
    );
    for required in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] is_invalid: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] invalid: Option<bool>",
        "logic::resolve_is_disabled(is_disabled, disabled)",
        "logic::resolve_is_invalid(is_invalid, invalid)",
        "logic::resolve_content(logic::FieldGroupContentInput",
    ] {
        assert!(
            group_view_source.contains(required),
            "FieldGroup API naming contract should include `{required}`."
        );
    }

    for forbidden in [
        "normalize_optional_text(",
        "normalize_aria_label(",
        "normalize_id_base(",
        "unwrap_or_default()",
    ] {
        assert!(
            !group_view_source.contains(forbidden),
            "group/view.rs should not keep fallback/default token `{forbidden}`."
        );
    }
}
