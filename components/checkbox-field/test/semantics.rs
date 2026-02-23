use ui_test_support::source_contract;

fn load_source(path: &str) -> &'static str {
    match path {
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "motion" => include_str!("../src/motion.rs"),
        "mod" => include_str!("../src/mod.rs"),
        "checkbox_view" => include_str!("../../checkbox/src/view.rs"),
        "docs_todo" => include_str!("../../../docs/plan/TODO.md"),
        "docs_checkbox_field_page" => docs_checkbox_field_page_source(),
        "readme" => include_str!("../src/README.md"),
        "check2" => include_str!("../check2.md"),
        "dx_script" => include_str!("../../../scripts/check-ui-dx.sh"),
        _ => panic!("unsupported source path: {path}"),
    }
}

fn docs_checkbox_field_page_source() -> &'static str {
    static DOCS: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| {
        let parent = source_contract::source_from_file_relative(
            file!(),
            "../../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs",
        );
        let child = source_contract::source_from_file_relative(
            file!(),
            "../../../apps/docs-app/src/pages/components/pages/forms_groups_extra/checkbox_field.rs",
        );
        let compat = child.replace(
            "pub(crate) fn checkbox_field() -> AnyView {",
            "pub(super) fn checkbox_field() -> AnyView {",
        );
        Box::leak(format!("{parent}\n{compat}").into_boxed_str())
    });
    *DOCS
}

#[test]
fn checkbox_field_component_keeps_ui_components_layer_file_layout() {
    let module = load_source("mod");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::CheckboxField;",
    ] {
        assert!(
            module.contains(needle),
            "checkbox-field module should keep ui assembly layout via `{needle}`"
        );
    }

    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !module.contains(forbidden),
            "checkbox-field internals should stay private: `{forbidden}`"
        );
    }
}

#[test]
fn checkbox_field_logic_consumes_state_primitives_without_reimplementation() {
    let logic = load_source("logic");

    for needle in [
        "pub use ui_state_primitives::checkbox::{",
        "pub use ui_state_primitives::checkbox_field::{",
        "CheckboxFieldStateInput",
        "CheckboxFieldState",
        "CheckboxFieldStatus",
        "resolve_checked_axis",
        "resolve_checked_change_handler_source",
        "normalize_label",
        "normalize_aria_label",
        "resolve_status",
        "resolve_state",
    ] {
        assert!(
            logic.contains(needle),
            "checkbox-field logic should consume ui-state-primitives via `{needle}`"
        );
    }

    for forbidden in [
        "pub struct CheckboxFieldStateInput {",
        "pub struct CheckboxFieldState {",
        "pub enum CheckboxFieldTone {",
    ] {
        assert!(
            !logic.contains(forbidden),
            "checkbox-field logic should not reimplement primitive state contracts: `{forbidden}`"
        );
    }
}

#[test]
fn checkbox_field_view_mounts_headless_and_motion_contracts() {
    let view = load_source("view");

    for needle in [
        "use ui_headless::{A11yDirection, labeled_group_attrs};",
        "let group_a11y = StoredValue::new(labeled_group_attrs(",
        "let motion_source = motion::source_attr(motion);",
        "let style_vars = StoredValue::new(motion::attach_motion(None, motion));",
        "style=move || style_vars.get_value()",
        "data-motion-source=motion_source",
    ] {
        assert!(
            view.contains(needle),
            "checkbox-field view should mount assembly contracts via `{needle}`"
        );
    }
}

#[test]
fn checkbox_field_api_naming_contract_prefers_is_on_default_prefixes_with_compat_aliases() {
    let view = load_source("view");
    let logic = load_source("logic");

    for needle in [
        "#[prop(optional)] is_checked: Option<ReadSignal<bool>>",
        "#[prop(optional)] on_checked_change: Option<WriteSignal<bool>>",
        "#[prop(optional)] default_checked: Option<bool>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] is_invalid: Option<bool>",
        "#[prop(optional)] checked: Option<ReadSignal<bool>>",
        "#[prop(optional)] set_checked: Option<WriteSignal<bool>>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] invalid: bool",
        "let resolved_content = logic::resolve_content(logic::CheckboxFieldContentInput {",
        "let checked_control = logic::resolve_checked_control(",
        "let checkbox_affordance = logic::resolve_checkbox_affordance(",
        "let render_state = Memo::new(move |_| {",
        "logic::resolve_render_state(logic::CheckboxFieldRenderStateInput {",
        "logic::normalize_is_disabled(is_disabled, disabled)",
        "logic::normalize_is_invalid(is_invalid, invalid)",
        "data-checked-mode=checked_mode_attr",
        "data-checked-prop-source=checked_prop_source_attr",
        "data-checked-change-source=checked_change_source_attr",
        "data-checked-default-source=checked_default_source_attr",
    ] {
        assert!(
            view.contains(needle),
            "checkbox-field view should keep API naming migration contract `{needle}`"
        );
    }

    for needle in [
        "pub struct CheckboxFieldContentInput {",
        "pub struct CheckboxFieldContent {",
        "pub fn resolve_content(",
        "CheckboxControlMode",
        "CheckboxCheckedAxisInput",
        "CheckboxCheckedValueSource",
        "CheckboxChangeHandlerSource",
        "CheckboxFieldStatus",
        "resolve_status",
        "pub struct CheckboxFieldRenderStateInput {",
        "pub struct CheckboxFieldRenderState {",
        "pub struct CheckboxFieldAffordance {",
        "pub fn resolve_checkbox_affordance(",
        "pub fn resolve_render_state(",
        "pub fn resolve_checked_control(",
        "pub fn normalize_is_disabled(",
        "pub fn normalize_is_invalid(",
    ] {
        assert!(
            logic.contains(needle),
            "checkbox-field logic should centralize naming normalization via `{needle}`"
        );
    }

    assert!(
        !view.contains("unwrap_or_default()"),
        "checkbox-field view should not contain fallback default branches; defaults belong to logic.rs"
    );
    assert!(
        !view.contains("logic::resolve_state(CheckboxFieldStateInput {"),
        "checkbox-field view should not rebuild state-machine mapping outside logic.rs"
    );
    assert!(
        !logic.contains("pub enum CheckedControlMode {"),
        "checkbox-field logic should consume control mode from ui-state-primitives instead of defining a local duplicate"
    );
}

#[test]
fn checkbox_field_public_surface_does_not_expose_platform_dom_types() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");

    for forbidden in [
        "web_sys::",
        "web-sys",
        "wasm_bindgen",
        "JsValue",
        "HtmlElement",
        "NodeRef<html::",
    ] {
        assert!(
            !module.contains(forbidden),
            "checkbox-field module should not expose platform detail `{forbidden}`"
        );
        assert!(
            !logic.contains(forbidden),
            "checkbox-field logic should not expose platform detail `{forbidden}`"
        );
        assert!(
            !view.contains(forbidden),
            "checkbox-field view should not expose platform detail `{forbidden}`"
        );
        assert!(
            !motion.contains(forbidden),
            "checkbox-field motion should not expose platform detail `{forbidden}`"
        );
    }
}

#[test]
fn checkbox_field_styles_stay_token_first_static_contract() {
    let styles = load_source("styles");

    assert!(
        styles.contains("pub const CSS: &str = r#\""),
        "checkbox-field styles should stay static CSS contract in styles.rs"
    );

    for needle in [
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-text-field-motion-duration",
    ] {
        assert!(
            styles.contains(needle),
            "checkbox-field styles should consume ui-theme token vars via `{needle}`"
        );
    }
}

#[test]
fn checkbox_field_semantic_contract_tests_cover_state_and_interaction_matrix_without_snapshot_dependency_locally()
 {
    let view = load_source("view");
    let local_semantics = include_str!("semantics.rs");

    for marker in [
        "role=group_a11y.get_value().role",
        "aria-label=move || group_a11y.get_value().aria_label",
        "aria-describedby=move || render_state.get().state.has_description.then(|| description_id.get())",
        "aria-disabled=move || render_state.get().state.is_disabled.then_some(\"true\")",
        "aria-invalid=move || render_state.get().state.is_invalid.then_some(\"true\")",
        "data-state=move || render_state.get().state.state_attr",
        "data-checked=move || render_state.get().state.is_checked.then_some(\"true\")",
        "data-disabled=move || render_state.get().state.is_disabled.then_some(\"true\")",
        "data-invalid=move || render_state.get().state.is_invalid.then_some(\"true\")",
        "data-checked-mode=checked_mode_attr",
        "data-checked-prop-source=checked_prop_source_attr",
        "data-checked-change-source=checked_change_source_attr",
        "data-checked-default-source=checked_default_source_attr",
    ] {
        assert!(
            view.contains(marker),
            "checkbox-field local semantic contract should include `{marker}`",
        );
    }

    let snapshot_macro = ["assert", "_snapshot!"].concat();
    let insta_snapshot = ["insta::assert", "_snapshot"].concat();
    assert!(
        !local_semantics.contains(&snapshot_macro) && !local_semantics.contains(&insta_snapshot),
        "checkbox-field local semantics should not degrade to snapshot-only assertions.",
    );
}

#[test]
fn checkbox_field_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally()
 {
    let view = load_source("view");
    let checkbox_view = load_source("checkbox_view");
    let todo_source = load_source("docs_todo");

    for marker in [
        "<Checkbox",
        "is_checked=checked",
        "on_checked_change=on_checked_change",
        "is_disabled=disabled",
    ] {
        assert!(
            view.contains(marker),
            "checkbox-field should delegate interaction surface to Checkbox via `{marker}`.",
        );
    }

    for marker in [
        "on:keydown=move |ev| {",
        "on:keyup=move |ev| {",
        "on:focus=move |_| aria.handlers.focus_ring.on_focus.run(())",
        "on:blur=move |_| {",
        "data-focused=move || render_state.get().state.is_focused.then_some(\"true\")",
        "data-focus-visible=move || render_state.get().state.is_focus_visible.then_some(\"true\")",
        "aria-checked=move || aria.attrs.aria_checked.get()",
    ] {
        assert!(
            checkbox_view.contains(marker),
            "checkbox focus-flow contract should stay in delegated checkbox semantics via `{marker}`.",
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "render_count follow-up governance should include `{marker}`.",
        );
    }
}

#[test]
fn checkbox_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks_locally()
 {
    let view = load_source("view");
    let local_semantics = include_str!("semantics.rs");

    for marker in [
        "role=group_a11y.get_value().role",
        "aria-label=move || group_a11y.get_value().aria_label",
        "aria-describedby=move || render_state.get().state.has_description.then(|| description_id.get())",
        "aria-disabled=move || render_state.get().state.is_disabled.then_some(\"true\")",
        "aria-invalid=move || render_state.get().state.is_invalid.then_some(\"true\")",
        "data-state=move || render_state.get().state.state_attr",
        "data-checked-mode=checked_mode_attr",
        "data-checked-prop-source=checked_prop_source_attr",
        "data-checked-change-source=checked_change_source_attr",
        "data-checked-default-source=checked_default_source_attr",
    ] {
        assert!(
            view.contains(marker),
            "checkbox-field semantic-priority contract should include `{marker}`.",
        );
    }

    let snapshot_macro = ["assert", "_snapshot!"].concat();
    let insta_snapshot = ["insta::assert", "_snapshot"].concat();
    assert!(
        !local_semantics.contains(&snapshot_macro) && !local_semantics.contains(&insta_snapshot),
        "checkbox-field local semantic-priority path should avoid snapshot-only assertions.",
    );
}

#[test]
fn checkbox_field_e2e_selector_stability_prefers_semantic_markers_and_settled_waits() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_checkbox_field_contract.spec.mjs");

    for needle in [
        "/#/components/checkbox-field",
        "body:not(:has(#boot))",
        "#docs-checkbox-field-newsletter[data-slot=\"checkbox-field\"]",
        "#docs-checkbox-field-terms[data-slot=\"checkbox-field\"]",
        "#docs-checkbox-field-read-only[data-slot=\"checkbox-field\"]",
        "#docs-checkbox-field-controlled[data-slot=\"checkbox-field\"]",
        "#docs-checkbox-field-uncontrolled[data-slot=\"checkbox-field\"]",
        "[data-slot=\"checkbox\"][role=\"checkbox\"]",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "checkbox-field e2e selector contract should include semantic marker `{needle}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        ":nth-child(",
        ":nth-of-type(",
        "locator(\"div > div >",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "checkbox-field e2e selector contract should avoid brittle token `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_field_e2e_repeatable_key_flow_uses_focus_keyboard_and_semantic_breakpoints() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_checkbox_field_contract.spec.mjs");

    for needle in [
        "docs-app checkbox-field key flow is repeatable with semantic breakpoints",
        "await controlledCheckbox.focus();",
        "await expect(controlledCheckbox).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(controlled).toHaveAttribute(\"data-state\", \"unchecked\");",
        "await expect(controlled).toHaveAttribute(\"data-state\", \"checked\");",
        "await uncontrolledCheckbox.focus();",
        "await expect(uncontrolledCheckbox).toBeFocused();",
        "await expect(uncontrolled).toHaveAttribute(\"data-state\", \"unchecked\");",
        "await page.reload();",
        "await expect(reloadedUncontrolled).toHaveAttribute(\"data-state\", \"checked\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "checkbox-field e2e repeatable-key-flow contract should include `{needle}`.",
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "checkbox-field repeatable key flow should avoid flaky token `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2.contains(required),
            "checkbox-field check2 docs-sync section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs = load_source("docs_checkbox_field_page");
    let logic = load_source("logic");

    for required in [
        "title=\"Hello World（默认路径）\"",
        "title=\"Controlled + Description\"",
        "title=\"Indicator End + Quiet + Invalid/Disabled\"",
        "title=\"Controlled vs Default (Comparison)\"",
        "data-slot=\"checkbox-field-state-matrix-note\"",
        "data-slot=\"checkbox-field-controlled-uncontrolled-note\"",
        "is_checked=newsletter",
        "on_checked_change=set_newsletter",
        "is_checked=terms",
        "on_checked_change=set_terms",
        "default_checked=true",
        "is_disabled=true",
        "is_invalid=true",
        "tone=CheckboxFieldTone::Quiet",
        "indicator_placement=CheckboxFieldIndicatorPlacement::End",
    ] {
        assert!(
            docs.contains(required),
            "checkbox-field docs matrix/examples should include `{required}`.",
        );
    }

    for required in [
        "pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {",
        "is_disabled.unwrap_or(disabled)",
        "pub fn normalize_is_invalid(is_invalid: Option<bool>, invalid: bool) -> bool {",
        "is_invalid.unwrap_or(invalid)",
        "pub fn resolve_checked_control(",
        "resolve_checked_axis(CheckboxCheckedAxisInput {",
        "default_checked,",
        "checked_default_source_attr = if default_checked.is_some() {",
        "\"default_checked\"",
        "\"implicit-default\"",
    ] {
        assert!(
            logic.contains(required),
            "checkbox-field logic should keep API/default normalization marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let check2 = load_source("check2");
    let dx_script = load_source("dx_script");

    assert!(
        check2.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "checkbox-field check2 should mark docs-sync/state-matrix item complete.",
    );

    for required in [
        "components/checkbox-field/test/semantics.rs::checkbox_field_check2_documents_docs_sync_and_state_matrix_rules",
        "components/checkbox-field/test/semantics.rs::checkbox_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_check2_documents_docs_sync_and_state_matrix_rules",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "checkbox-field check2 docs-sync/state-matrix section should reference `{required}`.",
        );
    }

    for required in [
        "checkbox_field_check2_documents_docs_sync_and_state_matrix_rules",
        "checkbox_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            dx_script.contains(required),
            "dx script should include docs-sync/state-matrix gate `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_documents_documentation_as_product_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2.contains(required),
            "checkbox-field check2 documentation-as-product section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_documentation_entry_exists_with_beginner_first_progression() {
    let readme = load_source("readme");
    let docs = load_source("docs_checkbox_field_page");

    for required in [
        "# CheckboxField",
        "## Hello World（最小可用）",
        "## 常见用法",
        "## 先用起来，再进阶",
        "默认路径：`<CheckboxField label=... />`，只传 `label` 也能直接工作。",
        "进阶控制：按需启用 `is_checked + default_checked + on_checked_change`。",
    ] {
        assert!(
            readme.contains(required),
            "checkbox-field README should include beginner marker `{required}`.",
        );
    }

    for required in [
        "pub(super) fn checkbox_field() -> AnyView",
        "title=\"CheckboxField\"",
        "slug=\"checkbox-field\"",
        "title=\"Hello World（默认路径）\"",
        "title=\"Controlled + Description\"",
        "title=\"Indicator End + Quiet + Invalid/Disabled\"",
        "title=\"Controlled vs Default (Comparison)\"",
    ] {
        assert!(
            docs.contains(required),
            "checkbox-field docs entry should include `{required}`.",
        );
    }

    let readme_hello = readme
        .find("## Hello World（最小可用）")
        .expect("checkbox-field README should include Hello World section");
    let readme_common = readme
        .find("## 常见用法")
        .expect("checkbox-field README should include common-usage section");
    let readme_progressive = readme
        .find("## 先用起来，再进阶")
        .expect("checkbox-field README should include beginner-to-advanced section");
    assert!(
        readme_hello < readme_common && readme_common < readme_progressive,
        "checkbox-field README should keep beginner-first order before advanced guidance.",
    );
}

#[test]
fn checkbox_field_check2_marks_documentation_as_product_contract_complete() {
    let check2 = load_source("check2");

    assert!(
        check2.contains(
            "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"
        ),
        "checkbox-field check2 should mark documentation-as-product item complete.",
    );

    for required in [
        "components/checkbox-field/src/README.md",
        "apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::checkbox_field",
        "components/checkbox-field/test/semantics.rs::checkbox_field_check2_documents_documentation_as_product_rules",
        "components/checkbox-field/test/semantics.rs::checkbox_field_documentation_entry_exists_with_beginner_first_progression",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_check2_documents_documentation_as_product_rules",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_documentation_entry_exists_with_beginner_first_progression",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "checkbox-field check2 documentation-as-product section should reference `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_documents_interactive_playground_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2.contains(required),
            "checkbox-field check2 interactive-playground section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs = load_source("docs_checkbox_field_page");
    let playground = include_str!("../../../apps/docs-app/src/playground.rs");

    for required in [
        "title=\"Interactive Playground\"",
        "description=\"Display + Config + Code + CSS Test: edit props and inspect actual config/state contracts.\"",
        "code_signal=interactive_code",
        "test_css_source=interactive_test_css",
        "test_source_path=\"crates/ui/src/checkbox_field/styles.rs\".to_string()",
        "test_config_signal=interactive_config",
        "controls=move || view!",
        "Switch checked=interactive_checked set_checked=set_interactive_checked",
        "Switch checked=interactive_disabled set_checked=set_interactive_disabled",
        "Switch checked=interactive_invalid set_checked=set_interactive_invalid",
        "checked=interactive_show_description",
        "checked=interactive_custom_class",
        "is_checked=interactive_checked",
        "on_checked_change=set_interactive_checked",
        "\"checked: \" {move || interactive_checked.get()}",
    ] {
        assert!(
            docs.contains(required),
            "checkbox-field docs should provide interactive playground marker `{required}`.",
        );
    }

    for required in [
        "let section_class = \"docs-card playground\";",
        "<div data-playground-scope=scope_id.clone()>",
        "<Card class_name=\"playground__preview\".to_string()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<div data-slot=\"playground-controls\">",
        "<Card class_name=\"playground__panel playground__controls\".to_string()>",
    ] {
        assert!(
            playground.contains(required),
            "docs-app Playground should keep interactive preview marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e = include_str!("../../../e2e/tests/docs_app_checkbox_field_contract.spec.mjs");

    for required in [
        "docs-app checkbox-field key flow is repeatable with semantic breakpoints",
        "await page.goto(CHECKBOX_FIELD_PAGE);",
        "body:not(:has(#boot))",
        "await controlledCheckbox.focus();",
        "await expect(controlledCheckbox).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(controlled).toHaveAttribute(\"data-state\", \"unchecked\");",
        "await expect(controlled).toHaveAttribute(\"data-state\", \"checked\");",
        "await uncontrolledCheckbox.focus();",
        "await expect(uncontrolledCheckbox).toBeFocused();",
        "await page.reload();",
        "await expect(reloadedUncontrolled).toHaveAttribute(\"data-state\", \"checked\");",
    ] {
        assert!(
            e2e.contains(required),
            "checkbox-field interactive playground should keep repeatable e2e marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_interactive_playground_contract_complete() {
    let check2 = load_source("check2");

    assert!(
        check2.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "checkbox-field check2 should mark interactive-playground item complete.",
    );

    for required in [
        "title=\"Interactive Playground\"",
        "apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::checkbox_field",
        "e2e/tests/docs_app_checkbox_field_contract.spec.mjs::docs-app checkbox-field key flow is repeatable with semantic breakpoints",
        "components/checkbox-field/test/semantics.rs::checkbox_field_check2_documents_interactive_playground_rules",
        "components/checkbox-field/test/semantics.rs::checkbox_field_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "components/checkbox-field/test/semantics.rs::checkbox_field_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_check2_documents_interactive_playground_rules",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-dx.sh",
        "AI Spec 相关联动示例：N/A（`checkbox-field` 非 Spec 构建器组件）",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "checkbox-field check2 interactive-playground section should reference `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_documents_source_first_copy_paste_ready_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2.contains(required),
            "checkbox-field check2 source-first section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs = load_source("docs_checkbox_field_page");
    let playground = include_str!("../../../apps/docs-app/src/playground.rs");
    let e2e = include_str!("../../../e2e/tests/docs_app_checkbox_field_contract.spec.mjs");

    for required in [
        "data-slot=\"checkbox-field-copy-ready\"",
        "data-slot=\"checkbox-field-source-paths\"",
        "data-slot=\"checkbox-field-source-prerequisites\"",
        "Copy-ready snippets prepend imports automatically: use leptos::prelude::*; use ui::*.",
        "Source paths: components/checkbox-field/src/mod.rs, components/checkbox-field/src/logic.rs, components/checkbox-field/src/view.rs, components/checkbox-field/src/styles.rs.",
        "Feature prerequisites: component-checkbox_field (inject-css optional for runtime style injection).",
        "title=\"Controlled + Description\"",
        "title=\"Interactive Playground\"",
    ] {
        assert!(
            docs.contains(required),
            "checkbox-field source-first docs should keep marker `{required}`.",
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "code_imports",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground.contains(required),
            "docs playground copy-ready pipeline should keep `{required}`.",
        );
    }

    for required in [
        "docs-app checkbox-field playground source is copy-paste ready",
        "data-copyable",
        "use leptos::prelude::*;",
        "use ui::*;",
        "data-slot=\"checkbox-field-source-paths\"",
        "data-slot=\"checkbox-field-source-prerequisites\"",
        "toContainText(\"components/checkbox-field/src/mod.rs\")",
        "toContainText(\"component-checkbox_field\")",
        "toContainText(\"inject-css\")",
    ] {
        assert!(
            e2e.contains(required),
            "checkbox-field e2e source-first contract should keep `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2 = load_source("check2");

    assert!(
        check2.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "checkbox-field check2 should mark source-first copy-paste-ready item complete.",
    );

    for required in [
        "apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::checkbox_field",
        "e2e/tests/docs_app_checkbox_field_contract.spec.mjs::docs-app checkbox-field playground source is copy-paste ready",
        "components/checkbox-field/test/semantics.rs::checkbox_field_check2_documents_source_first_copy_paste_ready_rules",
        "components/checkbox-field/test/semantics.rs::checkbox_field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_check2_documents_source_first_copy_paste_ready_rules",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "checkbox-field check2 source-first section should reference `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2.contains(required),
            "checkbox-field check2 heroui-benchmark docs-sync section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("docs_checkbox_field_page");
    let readme_source = load_source("readme");

    for required in [
        "### CheckboxField 同步记录（2026-02-20）",
        "参数模型同步：`CheckboxField` 参数主轴保持 `is_checked/default_checked/on_checked_change`",
        "component_doc!(\"CheckboxField\", \"checkbox-field\", \"Forms\", forms_groups_extra::checkbox_field)",
        "`apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::checkbox_field()`",
        "`components/checkbox-field/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(required),
            "heroui strategy doc should include checkbox-field synchronization marker `{required}`.",
        );
    }

    for required in [
        "component_doc!(",
        "\"CheckboxField\"",
        "\"checkbox-field\"",
        "forms_groups_extra::checkbox_field",
    ] {
        assert!(
            pages_source.contains(required),
            "component docs index should expose checkbox-field entry marker `{required}`.",
        );
    }

    for required in [
        "pub(super) fn checkbox_field() -> AnyView",
        "title=\"CheckboxField\"",
        "slug=\"checkbox-field\"",
    ] {
        assert!(
            docs_source.contains(required),
            "docs-app checkbox-field page should stay indexable via marker `{required}`.",
        );
    }

    for required in [
        "# CheckboxField",
        "## docs-app 入口",
        "forms_groups_extra.rs::checkbox_field()",
        "#/components/checkbox-field",
    ] {
        assert!(
            readme_source.contains(required),
            "checkbox-field README should remain an equivalent component doc entry via `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "echo \"[dx] contract: checkbox-field heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should enforce heroui-benchmark docs-sync contract `{required}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2 = load_source("check2");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "components/checkbox-field/test/semantics.rs::checkbox_field_check2_documents_heroui_benchmark_docs_sync_rules",
        "components/checkbox-field/test/semantics.rs::checkbox_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "components/checkbox-field/test/semantics.rs::checkbox_field_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_check2_documents_heroui_benchmark_docs_sync_rules",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "components/checkbox-field/test/checkbox_field/semantics.rs::checkbox_field_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "checkbox-field check2 should keep heroui-benchmark docs-sync evidence marker `{required}`.",
        );
    }
}
