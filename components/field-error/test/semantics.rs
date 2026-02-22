use ui_test_support::source_contract;

fn load_source(rel_path: &str) -> &'static str {
    match rel_path {
        "../../components/field-error/src/mod.rs" => include_str!("../src/mod.rs"),
        "../../components/field-error/src/logic.rs" => include_str!("../src/logic.rs"),
        "../../components/field-error/src/view.rs" => include_str!("../src/view.rs"),
        "../../components/field-error/src/styles.rs" => include_str!("../src/styles.rs"),
        "../../components/field-error/src/protocol.rs" => include_str!("../src/protocol.rs"),
        "../../components/field-error/src/README.md" => include_str!("../src/README.md"),
        "../../crates/ui/Cargo.toml" => {
            include_str!("../../../crates/ui/Cargo.toml")
        }
        "../../crates/ui/src/lib.rs" => {
            include_str!("../../../crates/ui/src/lib.rs")
        }
        "../../crates/ui/src/css.rs" => {
            include_str!("../../../crates/ui/src/css.rs")
        }
        "../../crates/ui/src/root.rs" => {
            include_str!("../../../crates/ui/src/root.rs")
        }
        "../../crates/ui-headless/Cargo.toml" => {
            include_str!("../../../crates/ui-headless/Cargo.toml")
        }
        "../../crates/ui-headless/src/lib.rs" => {
            include_str!("../../../crates/ui-headless/src/lib.rs")
        }
        "../../crates/ui-motion/src/lib.rs" => include_str!("../../../crates/ui-motion/src/lib.rs"),
        "../../crates/ui-visual-primitive/src/active_highlight.rs" => {
            include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs")
        }
        "../../apps/web-demo/Cargo.toml" => include_str!("../../../apps/web-demo/Cargo.toml"),
        "../../crates/ui-state-primitives/src/field_error.rs" => {
            include_str!("../../../crates/ui-state-primitives/src/field_error.rs")
        }
        "../../apps/docs-app/src/pages/components/pages/forms_extra.rs" => {
            source_contract::source_from_file_relative(
                file!(),
                "../../../apps/docs-app/src/pages/components/pages/forms_extra.rs",
            )
        }
        "legacy_semantics" => {
            include_str!("../../../components/field-error/test/field_error_semantics.rs")
        }
        _ => panic!("unsupported source path: {rel_path}"),
    }
}

#[test]
fn field_error_semantics_tests_are_migrated_to_component_directory() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let legacy_semantics = load_source("legacy_semantics");
    let local_semantics = include_str!("semantics.rs");

    assert!(
        mod_source.contains("#[path = \"../test/semantics.rs\"]")
            && mod_source.contains("mod semantics_tests;"),
        "field-error should wire `components/field-error/test/semantics.rs` from crate entry.",
    );

    assert!(
        legacy_semantics.contains("../../../components/field-error/test/semantics.rs"),
        "legacy ui semantics entry should include migrated component semantics file.",
    );
    assert!(
        local_semantics.contains("field_error_semantics_tests_are_migrated_to_component_directory"),
        "component-local semantics suite should provide migration coverage.",
    );
}

#[test]
fn field_error_public_surface_does_not_expose_dom_platform_types() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");

    for forbidden in [
        "web_sys::",
        "web-sys",
        "wasm_bindgen",
        "JsValue",
        "HtmlElement",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "field-error public module should not expose `{forbidden}`.",
        );
    }
}

#[test]
fn field_error_component_layer_keeps_file_responsibilities() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");

    for needle in ["pub(crate) mod logic;", "pub mod styles;", "mod view;"] {
        assert!(
            mod_source.contains(needle),
            "field-error module boundary should include `{needle}`.",
        );
    }

    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !mod_source.contains(forbidden),
            "field-error internals should stay private; found `{forbidden}`.",
        );
    }

    for needle in [
        "pub struct FieldErrorLogicInput {",
        "pub struct FieldErrorViewModel {",
        "pub fn normalize_optional_text(",
        "pub fn normalize_control_inputs(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_message(",
        "pub fn resolve_view_model(",
        "pub fn resolve_state(",
        "pub fn resolve_headless_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep centralized normalization/state helpers; missing `{needle}`.",
        );
    }

    for needle in [
        "use ui_headless::{",
        "A11yDirection, CommonStrings, ErrorMessageOptions, use_error_message, use_ui_i18n,",
        "};",
        "logic::resolve_view_model(logic::FieldErrorLogicInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "use_error_message(ErrorMessageOptions {",
        "let i18n = use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "default_message: Some(common.field_error_default_message.as_ref().to_string())",
        "default_aria_label: Some(common.field_error_aria_label.as_ref().to_string())",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should mount logic + headless contracts; missing `{needle}`.",
        );
    }
}

#[test]
fn field_error_api_naming_prefers_is_prefix_with_legacy_alias_compatibility() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let readme_source = load_source("../../components/field-error/src/README.md");

    for needle in [
        "#[prop(optional)] is_visible: Option<bool>",
        "#[prop(optional)] visible: bool",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] is_icon_visible: Option<bool>",
        "#[prop(optional)] show_icon: bool",
        "logic::resolve_view_model(logic::FieldErrorLogicInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "field-error view should keep naming migration contract `{needle}`.",
        );
    }

    for needle in [
        "pub fn normalize_control_inputs(",
        "visible: is_visible.unwrap_or(visible)",
        "disabled: is_disabled.unwrap_or(disabled)",
        "show_icon: is_icon_visible.unwrap_or(show_icon)",
    ] {
        assert!(
            load_source("../../components/field-error/src/logic.rs").contains(needle),
            "field-error logic should keep centralized default-priority contract `{needle}`.",
        );
    }

    for needle in [
        "is_visible=true",
        "is_visible=false",
        "is_disabled=true",
        "is_icon_visible=true",
    ] {
        assert!(
            docs_source.contains(needle),
            "field-error docs should use preferred `is_*` naming via `{needle}`.",
        );
    }

    for needle in [
        "| `is_visible` | `Option<bool>` |",
        "| `is_disabled` | `Option<bool>` |",
        "| `is_icon_visible` | `Option<bool>` |",
        "| `visible` | `bool` (legacy alias) |",
        "Naming migration: `is_*` props are preferred",
    ] {
        assert!(
            readme_source.contains(needle),
            "field-error README should document naming migration via `{needle}`.",
        );
    }
}

#[test]
fn field_error_semantics_contract_is_exposed_via_headless_and_state_markers() {
    let source = load_source("../../components/field-error/src/view.rs");

    for attr in [
        "data-slot=\"field-error\"",
        "role=move || semantics.get().attrs.role",
        "aria-live=move || semantics.get().attrs.aria_live",
        "aria-label=move || semantics.get().attrs.aria_label.clone()",
        "data-tone=move || semantics.get().attrs.data_tone",
        "data-state=move || state.get().data_state.as_attr()",
        "data-disabled=move || semantics.get().attrs.data_disabled",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-message-source=move || state.get().message_source.as_attr()",
        "data-custom-class=move || semantics.get().attrs.data_custom_class",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "data-slot=\"field-error-text\"",
    ] {
        assert!(
            source.contains(attr),
            "field-error should expose `{attr}` for stable semantics contracts.",
        );
    }
}

#[test]
fn field_error_observability_markers_are_stable_and_enumerated() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/field_error.rs");
    let check2_source = include_str!("../check2.md");

    for marker in [
        "role=move || semantics.get().attrs.role",
        "aria-disabled=move || semantics.get().attrs.aria_disabled",
        "aria-hidden=move || (!state.get().is_visible).then_some(\"true\")",
        "data-state=move || state.get().data_state.as_attr()",
        "data-visible=move || state.get().is_visible.then_some(\"true\")",
        "data-disabled=move || semantics.get().attrs.data_disabled",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-message-source=move || state.get().message_source.as_attr()",
        "data-class-source=move || semantics.get().attrs.data_class_source",
    ] {
        assert!(
            view_source.contains(marker),
            "field-error should expose stable observable marker `{marker}`.",
        );
    }

    for closed_set in [
        "pub enum FieldErrorDataState {",
        "pub enum FieldErrorSource {",
        "pub enum FieldErrorMessageSource {",
        "pub const fn as_attr(self) -> &'static str",
        "Self::Hidden => \"hidden\"",
        "Self::Disabled => \"disabled\"",
        "Self::Visible => \"visible\"",
        "Self::Default => \"default\"",
        "Self::Custom => \"custom\"",
        "Self::None => \"none\"",
    ] {
        assert!(
            primitive_source.contains(closed_set),
            "field-error primitive should keep enumerated marker value set `{closed_set}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 状态可观测、可检索、可验证"),
        "field-error check2 should mark observability marker checklist item as complete.",
    );
}

#[test]
fn field_error_tests_prioritize_semantic_contracts_over_visual_snapshots() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let local_semantics = include_str!("semantics.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "field_error_semantics_contract_is_exposed_via_headless_and_state_markers",
        "field_error_observability_markers_are_stable_and_enumerated",
        "field_error_styles_depend_on_explicit_state_markers_not_dom_shape_or_inline_logic",
        "field_error_controlled_uncontrolled_rule_is_explicitly_na_for_stateless_scope",
    ] {
        assert!(
            local_semantics.contains(required),
            "field-error semantics suite should keep semantic-contract coverage `{required}`.",
        );
    }

    for required in [
        "role=move || semantics.get().attrs.role",
        "aria-live=move || semantics.get().attrs.aria_live",
        "aria-label=move || semantics.get().attrs.aria_label.clone()",
        "data-state=move || state.get().data_state.as_attr()",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-message-source=move || state.get().message_source.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "field-error view should expose semantic marker `{required}` for non-snapshot assertions.",
        );
    }

    for forbidden in [
        "\n    assert_snapshot!(",
        "\n    to_match_snapshot(",
        "\n    insta::assert_snapshot!(",
        "\n    snapshot(\"",
    ] {
        assert!(
            !local_semantics.contains(forbidden),
            "field-error semantics suite should not rely on visual snapshot helper `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 测试验证“语义契约”而不只验证视觉快照。"),
        "field-error check2 should mark semantic-contract-testing checklist item as complete.",
    );
}

#[test]
fn field_error_semantics_first_rule_is_checked_with_contract_focused_assertions() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let local_semantics = include_str!("semantics.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "field_error_semantics_contract_is_exposed_via_headless_and_state_markers",
        "field_error_observability_markers_are_stable_and_enumerated",
        "field_error_tests_prioritize_semantic_contracts_over_visual_snapshots",
    ] {
        assert!(
            local_semantics.contains(required),
            "field-error semantics-first rule should keep contract-focused test `{required}`.",
        );
    }

    for required in [
        "role=move || semantics.get().attrs.role",
        "aria-live=move || semantics.get().attrs.aria_live",
        "aria-label=move || semantics.get().attrs.aria_label.clone()",
        "data-state=move || state.get().data_state.as_attr()",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-message-source=move || state.get().message_source.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "field-error view should keep semantic contract marker `{required}`.",
        );
    }

    for forbidden in [
        "\n    assert_snapshot!(",
        "\n    to_match_snapshot(",
        "\n    insta::assert_snapshot!(",
        "\n    snapshot(\"",
    ] {
        assert!(
            !local_semantics.contains(forbidden),
            "field-error semantics-first rule should not regress to visual snapshot assertions `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains(
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。"
        ),
        "field-error check2 should mark semantics-first checklist item as complete.",
    );
    assert!(
        check2_source
            .contains("`FieldError` 为非交互单节点组件，无独立键盘路径，按组件范围记为 N/A"),
        "field-error check2 should record keyboard-path N/A rationale for this static component.",
    );
}

#[test]
fn field_error_e2e_selectors_rule_uses_semantic_markers_and_wasm_stable_waits() {
    let check2_source = include_str!("../check2.md");
    let e2e_source = include_str!("../../../e2e/tests/docs_app_field_error_contract.spec.mjs");

    for required in [
        "body:not(:has(#boot))",
        "[data-component=\"field-error\"]",
        "[data-slot=\"field-error\"][data-state=\"visible\"]",
        "[data-slot=\"field-error\"][data-state=\"disabled\"][data-disabled=\"true\"]",
        "[data-slot=\"field-error\"][data-state=\"hidden\"][aria-hidden=\"true\"]",
        "toHaveAttribute(\"role\", \"alert\")",
        "toHaveAttribute(\"aria-live\", \"assertive\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "field-error e2e should keep semantic selector/wait contract `{required}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "locator(\".docs-stack >",
        "nth-child(",
        "getByText(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "field-error e2e should avoid brittle selector/sleep pattern `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "field-error check2 should mark e2e-selector-stability checklist item as complete.",
    );
    assert!(
        check2_source.contains("`e2e/tests/docs_app_field_error_contract.spec.mjs`")
            && check2_source.contains("N/A（组件级）：`FieldError` 无异步请求与组件级动画"),
        "field-error check2 should document e2e evidence and async/motion N/A rationale.",
    );
}

#[test]
fn field_error_key_flow_is_in_repeatable_e2e_regression_set_with_semantic_breakpoints() {
    let check2_source = include_str!("../check2.md");
    let e2e_source = include_str!("../../../e2e/tests/docs_app_field_error_contract.spec.mjs");

    for required in [
        "docs-app field-error key flow remains repeatable via semantic breakpoints",
        "await page.goto(\"/#/components/error-message\");",
        "await page.goto(\"/#/components/field-error\");",
        "const baseSelector =",
        "data-slot=\"field-error\"",
        "data-state=\"visible\"",
        "data-tone=\"negative\"",
        "data-message-source=\"custom\"",
        "toHaveAttribute(\"role\", \"alert\")",
        "toHaveAttribute(\"aria-live\", \"assertive\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "field-error e2e key-flow regression should keep semantic breakpoint `{required}`.",
        );
    }

    for forbidden in [
        "toHaveScreenshot(",
        "page.screenshot(",
        "视觉快照",
        "page content mismatch",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "field-error key-flow regression should not rely on non-semantic failure mode `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "field-error check2 should mark key-flow regression checklist item as complete.",
    );
    assert!(
        check2_source.contains("`e2e/tests/docs_app_field_error_contract.spec.mjs`")
            && check2_source.contains("field-error -> error-message -> field-error")
            && check2_source
                .contains("N/A（组件级）：`FieldError` 为非 overlay、非异步、非键盘交互组件"),
        "field-error check2 should document repeatable-flow evidence and high-risk-path N/A boundary.",
    );
}

#[test]
fn field_error_component_file_responsibilities_are_explicit_with_motion_na() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let styles_source = load_source("../../components/field-error/src/styles.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    for required in ["pub(crate) mod logic;", "pub mod styles;", "mod view;"] {
        assert!(
            mod_source.contains(required),
            "field-error mod.rs should keep minimal module boundary `{required}`.",
        );
    }

    for forbidden in [
        "mod motion;",
        "pub mod motion;",
        "motion::",
        "attach_motion",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "field-error static component should keep motion contract as N/A `{forbidden}`.",
        );
    }

    for forbidden in ["view! {", "on:click", "on:pointer", "style=", "web_sys::"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not carry DOM/rendering concern `{forbidden}`.",
        );
    }

    for required in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles_source.contains(required),
            "styles.rs should remain static token-first css contract `{required}`.",
        );
    }
    for forbidden in ["fn ", "FieldError(", "use_error_message", "Invalid value"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not include logic/view/business copy concern `{forbidden}`.",
        );
    }

    for required in [
        "view! {",
        "use_error_message(ErrorMessageOptions {",
        "logic::resolve_view_model(logic::FieldErrorLogicInput {",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should focus on structure + headless mount `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 组件文件职责正确：`mod.rs`"),
        "field-error check2 should mark file-responsibility checklist item as complete.",
    );
}

#[test]
fn field_error_spec_rs_is_not_introduced_for_simple_component() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let readme_source = load_source("../../components/field-error/src/README.md");
    let check2_source = include_str!("../check2.md");

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "field-error is a simple component and must not introduce `src/spec.rs`.",
    );

    for forbidden in ["mod spec;", "pub mod spec;", "spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "field-error module boundary should not wire complex-component spec entry `{forbidden}`.",
        );
    }

    assert!(
        readme_source.contains("FieldError"),
        "field-error documentation should stay in README/check2 rather than a forced spec.rs.",
    );

    assert!(
        check2_source.contains("- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。"),
        "field-error check2 should mark spec.rs-governance checklist item as complete.",
    );
    assert!(
        check2_source.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
        "field-error check2 should mark hyper-structure-builder checklist item as complete.",
    );
    assert!(
        check2_source.contains("`FieldError` 为静态错误提示组件，不承载复杂配置编排与结构投影，不应引入 `*Spec::new()...render()` 建造者层"),
        "field-error check2 should document hyper-structure-builder N/A rationale for simple component.",
    );
}

#[test]
fn field_error_manifest_rbi_requirement_is_tracked_as_component_stage_na() {
    let check2_source = include_str!("../check2.md");
    let component_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    assert!(
        check2_source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "field-error check2 should mark manifest-rbi checklist item as complete.",
    );
    assert!(
        check2_source
            .contains("`FieldError` 当前为存量静态组件的语义契约对齐，不属于新增组件或大改范围"),
        "field-error check2 should document manifest-rbi N/A rationale for current stage.",
    );
    assert!(
        !component_root.join("Component.toml").exists(),
        "field-error should not introduce Component.toml unless it becomes a new/majorly changed component.",
    );
    assert!(
        !component_root.join(".rbi").exists(),
        "field-error should not introduce .rbi unless it becomes a new/majorly changed component.",
    );
}

#[test]
fn field_error_styles_remain_token_first_and_state_marker_driven() {
    let source = load_source("../../components/field-error/src/styles.rs");

    for selector in [
        ".ui-field-error--tone-auto",
        ".ui-field-error[data-tone=\"auto\"]",
        ".ui-field-error--tone-negative",
        ".ui-field-error[data-tone=\"negative\"]",
        ".ui-field-error[data-state=\"hidden\"]",
        ".ui-field-error[data-disabled=\"true\"]",
        ".ui-field-error[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "field-error styles should include `{selector}` marker contract.",
        );
    }

    assert!(
        source.contains("var(--ui-"),
        "field-error styles should remain token-first and consume `--ui-*` variables.",
    );
}

#[test]
fn field_error_token_first_styles_are_aggregated_via_ui_components_css_contract() {
    let styles_source = load_source("../../components/field-error/src/styles.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let css_aggregate_source = load_source("../../crates/ui/src/css.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-field-error {",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
    ] {
        assert!(
            styles_source.contains(required),
            "field-error styles should keep token-first static contract `{required}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"component-field_error\")]",
        "out.push_str(crate::field_form::field_error::styles::CSS);",
        "@layer ui",
    ] {
        assert!(
            css_aggregate_source.contains(required),
            "ui css aggregation should include field_error via `{required}`.",
        );
    }

    for forbidden in ["style=", "style:\"", "style='"] {
        assert!(
            !view_source.contains(forbidden),
            "field-error view should avoid runtime inline style branch `{forbidden}`.",
        );
    }

    for forbidden in ["stylist::", "styled_components", "emotion::", "tailwind"] {
        assert!(
            !styles_source.contains(forbidden) && !view_source.contains(forbidden),
            "field-error should not default to utility/css-in-rust style path `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。"),
        "field-error check2 should mark token-first static-style checklist item as complete.",
    );
}

#[test]
fn field_error_styles_depend_on_explicit_state_markers_not_dom_shape_or_inline_logic() {
    let styles_source = load_source("../../components/field-error/src/styles.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        ".ui-field-error[data-tone=\"auto\"]",
        ".ui-field-error[data-tone=\"neutral\"]",
        ".ui-field-error[data-tone=\"negative\"]",
        ".ui-field-error[data-state=\"hidden\"]",
        ".ui-field-error[data-disabled=\"true\"]",
        ".ui-field-error[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "field-error styles should branch on explicit semantic marker `{required}`.",
        );
    }

    for forbidden in [
        ":nth-child",
        "nth-of-type",
        ".ui-field-error > ",
        ".ui-field-error .",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "field-error styles should not guess state from brittle selector `{forbidden}`.",
        );
    }

    for forbidden in ["style=", "style:\"", "style='"] {
        assert!(
            !view_source.contains(forbidden),
            "field-error view should avoid inline style business logic `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 样式依赖显式状态（`data-*`/class）"),
        "field-error check2 should mark explicit-state-style checklist item as complete.",
    );
}

#[test]
fn field_error_component_tests_live_in_neighbor_test_directory() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let protocol_source = load_source("../../components/field-error/src/protocol.rs");

    for needle in [
        "../test/logic.rs",
        "../test/protocol.rs",
        "../test/semantics.rs",
    ] {
        assert!(
            mod_source.contains(needle)
                || logic_source.contains(needle)
                || protocol_source.contains(needle),
            "field-error should keep tests in `components/field-error/test`; missing `{needle}`.",
        );
    }
}

#[test]
fn field_error_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn field_error() -> AnyView",
        "title=\"FieldError\"",
        "slug=\"field-error\"",
        "description=\"baseline-style field error primitive with centralized visibility/tone/message normalization and stable data contracts.\"",
        "<Playground title=\"Visible + Tone\" code_signal=default_code>",
        "<Playground title=\"Hidden + Disabled + Custom Class\" code_signal=hidden_code>",
        "<FieldError",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should include `{needle}` for field_error coverage.",
        );
    }
}

#[test]
fn field_error_docs_copy_paste_ready_contract_covers_hello_matrix_controlled_and_streaming_modes() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "let field_error_imports =",
        "use ui::{FieldError, FieldErrorTone};",
        "Hello World (Snapshot Baseline)",
        "State Matrix (Visible / Hidden / Disabled)",
        "Controlled vs Uncontrolled (Stateless Contract)",
        "Streaming Optional (fallback=snapshot)",
        "let hello_world_code = Signal::derive",
        "let state_matrix_code = Signal::derive",
        "let controlled_uncontrolled_code = Signal::derive",
        "let stream_snapshot_code = Signal::derive",
        "fallback=snapshot",
    ] {
        assert!(
            docs_source.contains(required),
            "field-error docs should keep copy-paste-ready playground coverage `{required}`.",
        );
    }

    let import_injection_count = docs_source
        .matches("code_imports=field_error_imports.clone()")
        .count();
    assert!(
        import_injection_count >= 4,
        "field-error docs should attach copy-ready imports to each required playground; found {import_injection_count}.",
    );

    assert!(
        check2_source.contains("- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。"),
        "field-error check2 should mark docs-as-product copy-paste-ready checklist item as complete.",
    );
    assert!(
        check2_source.contains("`Hello World (Snapshot Baseline)`")
            && check2_source.contains("`State Matrix (Visible / Hidden / Disabled)`")
            && check2_source.contains("`Controlled vs Uncontrolled (Stateless Contract)`")
            && check2_source.contains("`Streaming Optional (fallback=snapshot)`"),
        "field-error check2 should document the four required playground entries as evidence.",
    );
    assert!(
        check2_source.contains("code_imports")
            && check2_source.contains("use ui::{FieldError, FieldErrorTone};"),
        "field-error check2 should document source-first import completion evidence.",
    );
}

#[test]
fn field_error_docs_examples_and_matrices_are_synced_with_logic_api_contract() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub(super) fn field_error() -> AnyView",
        "Hello World (Snapshot Baseline)",
        "State Matrix (Visible / Hidden / Disabled)",
        "Controlled vs Uncontrolled (Stateless Contract)",
        "Visible + Tone",
        "Hidden + Disabled + Custom Class",
        "is_visible=true",
        "is_visible=false",
        "is_disabled=true",
        "is_icon_visible=true",
        "tone=FieldErrorTone::Neutral",
        "tone=FieldErrorTone::Negative",
    ] {
        assert!(
            docs_source.contains(required),
            "field-error docs should keep synced example/matrix contract `{required}`.",
        );
    }

    for required in [
        "pub fn normalize_control_inputs(",
        "visible: is_visible.unwrap_or(visible)",
        "disabled: is_disabled.unwrap_or(disabled)",
        "show_icon: is_icon_visible.unwrap_or(show_icon)",
        "pub const DEFAULT_ARIA_LABEL: &str = \"FieldError\";",
        "pub const DEFAULT_MESSAGE: &str = \"Invalid value\";",
    ] {
        assert!(
            logic_source.contains(required),
            "field-error logic should keep authoritative api/default contract `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "field-error check2 should mark docs/examples/matrix-sync checklist item as complete.",
    );
    assert!(
        check2_source
            .contains("`apps/docs-app/src/pages/components/pages/forms_extra.rs::field_error()`")
            && check2_source.contains("`is_visible/is_disabled/is_icon_visible`")
            && check2_source.contains("`logic.rs::normalize_control_inputs`"),
        "field-error check2 should document docs-to-logic sync evidence and naming/default ownership.",
    );
}

#[test]
fn field_error_docs_interactive_playground_supports_realtime_state_preview_and_repeatable_flow() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let e2e_source = include_str!("../../../e2e/tests/docs_app_field_error_contract.spec.mjs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "Interactive Playground (Props + State + Source Markers)",
        "test_config_signal=workbench_actual_config",
        "controls=move || {",
        "data-slot=\"field-error-config-controls\"",
        "data-action=\"cycle-tone-config\"",
        "data-action=\"toggle-visible-config\"",
        "data-action=\"toggle-disabled-config\"",
        "data-action=\"toggle-icon-config\"",
        "data-action=\"toggle-message-config\"",
        "data-action=\"toggle-aria-config\"",
        "data-action=\"toggle-class-config\"",
        "data-slot=\"field-error-config-summary\"",
        "data-slot=\"field-error-interactive-stage\"",
        "data-slot=\"field-error-interactive-hint\"",
    ] {
        assert!(
            docs_source.contains(required),
            "field-error docs interactive playground should keep `{required}`.",
        );
    }

    for required in [
        "interactive playground supports realtime props/state preview",
        "toggle-message-config",
        "toggle-aria-config",
        "toggle-class-config",
        "toggle-visible-config",
        "toHaveAttribute(\"data-state\", \"hidden\")",
        "toHaveAttribute(\"data-message-source\", \"custom\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "field-error e2e should keep repeatable interactive flow assertion `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "field-error check2 should mark interactive-playground checklist item as complete.",
    );
    assert!(
        check2_source.contains("`Interactive Playground (Props + State + Source Markers)`")
            && check2_source.contains("`e2e/tests/docs_app_field_error_contract.spec.mjs`")
            && check2_source.contains("N/A（组件级）：`FieldError` 非 AI Spec 组件"),
        "field-error check2 should document interactive flow evidence and spec N/A boundary.",
    );
}

#[test]
fn field_error_source_first_docs_are_copy_paste_ready_with_real_source_paths() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let e2e_source = include_str!("../../../e2e/tests/docs_app_field_error_contract.spec.mjs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "data-slot=\"field-error-source-first\"",
        "Source-first / Copy-Paste Ready",
        "Copy starter",
        "use ui::{FieldError, FieldErrorTone};",
        "components/field-error/src/mod.rs",
        "components/field-error/src/logic.rs",
        "components/field-error/src/view.rs",
        "components/field-error/src/styles.rs",
        "component-field_error",
        "inject-css",
    ] {
        assert!(
            docs_source.contains(required),
            "field-error docs should keep source-first copy-ready evidence `{required}`.",
        );
    }

    let import_injection_count = docs_source
        .matches("code_imports=field_error_imports.clone()")
        .count();
    assert!(
        import_injection_count >= 5,
        "field-error docs should keep import-ready code snippets across playgrounds; found {import_injection_count}.",
    );

    for required in [
        "source-first docs are copy-paste ready",
        "toHaveAttribute(\"data-copyable\", \"true\")",
        "toHaveAttribute(\"aria-label\", /Copy to clipboard/i)",
        "field-error-source-first",
        "components/field-error/src/mod.rs",
    ] {
        assert!(
            e2e_source.contains(required),
            "field-error e2e should verify copy-paste/source-first contract `{required}`.",
        );
    }

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "field-error check2 should mark source-first copy-paste checklist item as complete.",
    );
    assert!(
        check2_source.contains("`components/field-error/src/{mod,logic,view,styles}.rs`")
            && check2_source.contains("`e2e/tests/docs_app_field_error_contract.spec.mjs`"),
        "field-error check2 should document real source paths and e2e evidence for source-first contract.",
    );
}

#[test]
fn field_error_heroui_benchmark_docs_and_component_docs_are_synced_for_parameter_changes() {
    let strategy_source = include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_catalog_source = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let readme_source = load_source("../../components/field-error/src/README.md");
    let check2_source = include_str!("../check2.md");

    for required in [
        "### FieldError 同步记录（2026-02-20）",
        "- 参数模型同步：`FieldError` 维持 form feedback primitive 定位",
        "`tone/is_visible/is_disabled/is_icon_visible/message/aria_label/class_name/lang/dir`",
        "`is_visible > visible`、`is_disabled > disabled`、`is_icon_visible > show_icon`",
        "component_doc!(\"FieldError\", \"field-error\", \"Forms\", forms_extra::field_error)",
        "`#/components/field-error` 可索引访问",
        "`components/field-error/src/README.md` 提供等价入门文档入口",
        "研究文档补充判定：本轮为参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。",
        "HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。",
    ] {
        assert!(
            strategy_source.contains(required),
            "heroUI strategy doc should keep field-error synchronization evidence `{required}`.",
        );
    }

    assert!(
        pages_catalog_source.contains("\"FieldError\"")
            && pages_catalog_source.contains("\"field-error\"")
            && pages_catalog_source.contains("forms_extra::field_error"),
        "docs-app catalog should keep field-error entry discoverable.",
    );
    assert!(
        readme_source.contains("# FieldError")
            && readme_source.contains("## Start Here (先用起来，再进阶)"),
        "field-error README should remain accessible as component-document entry.",
    );

    assert!(
        check2_source.contains("- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。"),
        "field-error check2 should mark heroui benchmark/doc sync checklist item as complete.",
    );
    assert!(
        check2_source.contains("`FieldError 同步记录（2026-02-20）`")
            && check2_source.contains("`#/components/field-error`")
            && check2_source
                .contains("不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`"),
        "field-error check2 should include strategy-doc, docs entry and research-doc synchronization evidence.",
    );
}

#[test]
fn field_error_documentation_as_product_is_beginner_friendly_and_progressive() {
    let readme_source = load_source("../../components/field-error/src/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "# FieldError",
        "## Start Here (先用起来，再进阶)",
        "## Hello World (Minimum Viable)",
        "## Common Usage",
        "## Advanced Controls (When Needed)",
    ] {
        assert!(
            readme_source.contains(required),
            "field-error README should keep beginner-friendly progressive structure `{required}`.",
        );
    }

    for required in [
        "<FieldError",
        "is_visible=true",
        "message=\"Email is required\".to_string()",
        "tone=FieldErrorTone::Neutral",
        "is_disabled=true",
    ] {
        assert!(
            readme_source.contains(required),
            "field-error README should keep zero-threshold hello/common usage snippet `{required}`.",
        );
    }

    for required in [
        "pub(super) fn field_error() -> AnyView",
        "title=\"FieldError\"",
        "slug=\"field-error\"",
        "Hello World (Snapshot Baseline)",
        "State Matrix (Visible / Hidden / Disabled)",
    ] {
        assert!(
            docs_source.contains(required),
            "field-error docs-app entry should remain discoverable and runnable via `{required}`.",
        );
    }

    assert!(
        check2_source.contains(
            "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"
        ),
        "field-error check2 should mark documentation-as-product checklist item as complete.",
    );
    assert!(
        check2_source.contains("`components/field-error/src/README.md`")
            && check2_source.contains("`Start Here (先用起来，再进阶)`")
            && check2_source.contains(
                "`apps/docs-app/src/pages/components/pages/forms_extra.rs::field_error()`"
            ),
        "field-error check2 should document beginner path and docs entry evidence.",
    );
}

#[test]
fn field_error_visual_desire_check_is_resolved_with_component_scope_and_repo_escalation() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub(super) fn field_error() -> AnyView",
        "title=\"FieldError\"",
        "Playground title=\"Visible + Tone\"",
        "Playground title=\"Hidden + Disabled + Custom Class\"",
    ] {
        assert!(
            docs_source.contains(required),
            "field-error docs should keep default-theme visual baseline entry `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 默认主题美学质量达标（Visual Desire）"),
        "field-error check2 should mark visual-desire checklist item as complete.",
    );
    assert!(
        check2_source.contains("仓库级视觉回归基线（Button/Input/Overlay 截图对比）属于跨组件治理"),
        "visual-desire item should explicitly escalate cross-component screenshot baseline to repo-level task.",
    );
}

#[test]
fn field_error_tree_shaking_contract_uses_feature_gates_without_implicit_all_components() {
    let ui_components_cargo = load_source("../../crates/ui/Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui/src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let check2_source = include_str!("../check2.md");

    for required in [
        "component-field_error = [\"dep:ui-field-error\"]",
        "all-components = [",
        "web-demo-components = [",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui feature map should keep tree-shaking contract `{required}`.",
        );
    }

    for required in [
        "#[cfg(any(",
        "feature = \"component-field_error\",",
        "pub mod field_form {",
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]",
        "pub use web_demo_components::*;",
        "#[cfg(feature = \"all-components\")]",
        "pub use all_components::*;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib should keep feature-gated export boundary `{required}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"component-field_error\")]",
        "out.push_str(crate::field_form::field_error::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css aggregation should keep feature-gated field_error entry `{required}`.",
        );
    }

    let web_demo_line = web_demo_cargo
        .lines()
        .find(|line| line.trim_start().starts_with("ui ="))
        .expect("web-demo Cargo.toml should declare ui dependency line.");
    for required in [
        "default-features = false",
        "inject-css",
        "web-demo-components",
    ] {
        assert!(
            web_demo_line.contains(required),
            "web-demo should keep explicit non-default feature dependency `{required}`.",
        );
    }
    assert!(
        !web_demo_line.contains("all-components"),
        "web-demo should not implicitly enable `all-components`.",
    );

    assert!(
        check2_source.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "field-error check2 should mark tree-shaking checklist item as complete.",
    );
    assert!(
        check2_source.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
        "field-error check2 should mark tree-shaking-and-feature-pruning checklist item as complete.",
    );
    assert!(
        check2_source.contains("component-field_error = [\"dep:ui-field-error\"]")
            && check2_source.contains(
                "cargo tree -e features -p ui --no-default-features --features component-field_error,inject-css"
            )
            && check2_source.contains("cargo tree -e features -i ui -p web-demo")
            && check2_source.contains("未见 `all-components` 被隐式拉起"),
        "field-error tree-shaking note should include feature-map and cargo-tree evidence for non-all-components dependency.",
    );
    assert!(
        check2_source.contains("cargo tree -e features -i ui -p ui --no-default-features --features component-accordion,inject-css"),
        "tree-shaking checklist note should preserve minimal feature-tree verification command.",
    );
    assert!(
        check2_source.contains("cargo tree -e features -i ui -p web-demo"),
        "tree-shaking checklist note should preserve reverse dependency verification command.",
    );
}

#[test]
fn field_error_controlled_uncontrolled_rule_is_explicitly_na_for_stateless_scope() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "value:",
        "on_value_change",
        "default_value",
        "on_visible_change",
        "default_visible",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field-error should not expose pseudo-controlled API `{forbidden}` for stateless scope.",
        );
    }

    assert!(
        check2_source.contains("N/A：`FieldError` 为纯展示错误提示组件"),
        "field-error check2 should document why controlled/uncontrolled triplet is N/A.",
    );
}

#[test]
fn field_error_async_contract_is_explicitly_na_for_sync_display_scope() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "on_retry",
        "retry",
        "use_async_action",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field-error should not expose async protocol surface `{forbidden}` for sync display scope.",
        );
    }

    assert!(
        check2_source.contains("N/A：`FieldError` 为静态错误展示组件"),
        "field-error check2 should document why async contract is N/A.",
    );
}

#[test]
fn field_error_dx_paradox_keeps_hello_world_short_and_default_path_obvious() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let readme_source = load_source("../../components/field-error/src/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "#[prop(optional)] state",
        "#[prop(optional, into)] state",
        "state: FieldErrorState",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field-error should not expose internal state as required public API `{forbidden}`.",
        );
    }

    let hello_prefix = "## Hello World (Minimum Viable)\n\n```rust\n";
    let hello_start = readme_source
        .find(hello_prefix)
        .expect("README should contain hello world code fence");
    let hello_body_start = hello_start + hello_prefix.len();
    let hello_tail = &readme_source[hello_body_start..];
    let hello_end = hello_tail
        .find("\n```")
        .expect("README hello world code fence should terminate");
    let hello_block = &hello_tail[..hello_end];
    let hello_line_count = hello_block.lines().count();

    assert!(
        hello_line_count <= 5,
        "README hello world should be <=5 lines, got {hello_line_count} lines.",
    );
    assert!(
        !hello_block.contains("ui_state_primitives::") && !hello_block.contains("ui_headless::"),
        "README hello world should not require manual primitive/headless wiring.",
    );
    assert!(
        hello_block.contains("<FieldError")
            && hello_block.contains("is_visible=true")
            && hello_block.contains("message="),
        "README hello world should keep a copy-paste default call path.",
    );

    assert!(
        docs_source.contains("title=\"FieldError\"")
            && docs_source
                .contains("<Playground title=\"Visible + Tone\" code_signal=default_code>")
            && docs_source.contains("<FieldError")
            && docs_source.contains("is_visible=true"),
        "docs-app should expose an obvious minimum default path for field-error.",
    );

    assert!(
        check2_source.contains("API 易用性验收标准（DX Paradox）"),
        "field-error check2 should keep DX paradox governance entry.",
    );
}

#[test]
fn field_error_composite_parent_item_rule_is_explicitly_na_for_single_node_scope() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "#[prop(optional, into)] labels",
        "#[prop(optional, into)] titles",
        "#[prop(optional, into)] panels",
        "#[prop(optional)] items",
        "ItemSpec",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field-error should not expose composite/parallel-array API surface `{forbidden}`.",
        );
    }

    for forbidden in ["labels + children", "titles + panels", "<Parent><Item"] {
        assert!(
            !docs_source.contains(forbidden),
            "field-error docs should not recommend implicit composite pattern `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("N/A：`FieldError` 为单节点错误提示原语"),
        "field-error check2 should document why composite parent/item rule is N/A.",
    );
}

#[test]
fn field_error_macro_micro_drag_state_machine_rule_is_explicitly_na() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "on:pointermove",
        "on:pointerdown",
        "on:pointerup",
        "request_animation_frame",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "field-error should not implement drag macro/micro state machine contract `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("N/A：`FieldError` 为静态错误展示组件"),
        "field-error check2 should document why macro/micro drag state machine rule is N/A.",
    );
}

#[test]
fn field_error_two_pass_rendering_rule_is_explicitly_na_for_non_measured_component() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "getBoundingClientRect",
        "get_bounding_client_rect",
        "ClientRect",
        "ResizeObserver",
        "offsetWidth",
        "offsetHeight",
        "Rectification",
        "Placement",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "field-error should not implement geometry two-pass measurement contract `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("N/A：`FieldError` 为静态文本语义组件"),
        "field-error check2 should document why two-pass rendering rule is N/A.",
    );
}

#[test]
fn field_error_registration_protocol_rule_is_explicitly_na_for_single_node_component() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "field-error should not implement collection registration protocol `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("N/A：`FieldError` 为单节点错误提示组件"),
        "field-error check2 should document why registration protocol rule is N/A.",
    );
}

#[test]
fn field_error_slot_projection_rule_is_explicitly_na_for_non_container_component() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "keep_alive",
        "notify_hidden",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "field-error should not implement slot projection/lifecycle policy `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("N/A：`FieldError` 为单节点错误提示组件，不承载容器插槽投影"),
        "field-error check2 should document why slot projection rule is N/A.",
    );
}

#[test]
fn field_error_env_streams_rule_is_explicitly_na_for_static_component() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "IntersectionObserver",
        "matchMedia",
        "match_media",
        "on:resize",
        "BreakpointChanged",
        "debounce",
        "throttle",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "field-error should not implement env-stream sampling/action contract `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains(
            "N/A：`FieldError` 为静态错误提示组件，不订阅 `Resize/Theme/Intersection` 环境流"
        ),
        "field-error check2 should document why env-streams rule is N/A.",
    );
}

#[test]
fn field_error_event_light_cone_rule_is_explicitly_na_for_single_node_component() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "selection_state",
        "prop drilling",
        "provide_context",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "field-error should not implement event-light-cone collection protocol `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("N/A：`FieldError` 为单节点错误提示组件，不涉及集合批量选择与广播"),
        "field-error check2 should document why event light cone rule is N/A.",
    );
}

#[test]
fn field_error_causality_bus_rule_is_explicitly_na_for_static_component() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "TraceId",
        "trace_id",
        "CommandBus",
        "broadcast",
        "subscribe",
        "publish",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "field-error should not implement causality-bus trace pipeline `{forbidden}`.",
        );
    }

    assert!(
        check2_source
            .contains("N/A：`FieldError` 为单节点静态语义组件，不承载跨模块派生命令或总线广播流程"),
        "field-error check2 should document why causality bus rule is N/A.",
    );
}

#[test]
fn field_error_a11y_and_i18n_contract_is_mounted_with_headless_and_i18n_fallback_chain() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "use ui_headless::{",
        "A11yDirection, CommonStrings, ErrorMessageOptions, use_error_message, use_ui_i18n,",
        "};",
        "let i18n = use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "default_message: Some(common.field_error_default_message.as_ref().to_string())",
        "default_aria_label: Some(common.field_error_aria_label.as_ref().to_string())",
        "role=move || semantics.get().attrs.role",
        "aria-live=move || semantics.get().attrs.aria_live",
        "aria-label=move || semantics.get().attrs.aria_label.clone()",
        "lang=move || semantics.get().attrs.lang.clone()",
        "dir=move || semantics.get().attrs.dir",
        "use_error_message(ErrorMessageOptions {",
    ] {
        assert!(
            view_source.contains(required),
            "field-error should mount a11y + i18n contract `{required}`.",
        );
    }

    for required in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"FieldError\";",
        "pub const DEFAULT_MESSAGE: &str = \"Invalid value\";",
        "pub fn normalize_aria_label_with_default(",
        "pub fn normalize_message_with_default(",
    ] {
        assert!(
            logic_source.contains(required),
            "field-error logic should keep fallback text normalization contract `{required}`.",
        );
    }

    {
        let forbidden = "\"Invalid value\"";
        assert!(
            !view_source.contains(forbidden),
            "field-error view should not hardcode user-visible fallback text `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 存在 A11y 实现、国际化与本地化实现"),
        "field-error check2 should mark a11y + i18n checklist item as complete.",
    );
}

#[test]
fn field_error_default_priority_is_centralized_in_logic_layer() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    assert!(
        view_source.contains("logic::resolve_view_model(logic::FieldErrorLogicInput {"),
        "view.rs should consume centralized logic output.",
    );

    for forbidden in [
        "is_visible.unwrap_or(visible)",
        "is_disabled.unwrap_or(disabled)",
        "is_icon_visible.unwrap_or(show_icon)",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs must not own default-priority fallback branch `{forbidden}`.",
        );
    }

    for required in [
        "pub fn normalize_control_inputs(",
        "visible: is_visible.unwrap_or(visible)",
        "disabled: is_disabled.unwrap_or(disabled)",
        "show_icon: is_icon_visible.unwrap_or(show_icon)",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should centralize default-priority rule `{required}`.",
        );
    }

    assert!(
        check2_source.contains("默认值单一来源"),
        "field-error check2 should keep default single-source governance entry.",
    );
}

#[test]
fn field_error_state_normalization_is_centralized_in_logic_layer() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub struct FieldErrorLogicInput {",
        "pub struct FieldErrorViewModel {",
        "pub fn resolve_view_model(",
        "resolve_primitive_state(input)",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should centralize state normalization via `{required}`.",
        );
    }

    for forbidden in [
        "logic::normalize_aria_label(",
        "logic::normalize_message(",
        "logic::resolve_state(FieldErrorStateInput {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not rebuild state machine branch `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("状态归一化集中"),
        "field-error check2 should keep state-normalization governance entry.",
    );
}

#[test]
fn field_error_discrete_states_are_type_constrained_by_enums() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/field_error.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub enum FieldErrorDataState {",
        "pub enum FieldErrorSource {",
        "pub enum FieldErrorMessageSource {",
        "pub data_state: FieldErrorDataState,",
        "pub aria_source: FieldErrorSource,",
        "pub message_source: FieldErrorMessageSource,",
        "pub class_source: FieldErrorSource,",
        "FieldErrorDataState::Hidden",
        "FieldErrorDataState::Disabled",
        "FieldErrorDataState::Visible",
        "FieldErrorMessageSource::None",
        "FieldErrorMessageSource::Custom",
        "FieldErrorMessageSource::Default",
    ] {
        assert!(
            primitive_source.contains(required),
            "field-error should encode discrete state space via enums `{required}`.",
        );
    }

    for forbidden in [
        "pub data_state_attr: &'static str",
        "pub aria_source_attr: &'static str",
        "pub message_source_attr: &'static str",
        "pub class_source_attr: &'static str",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "field-error should not keep stringly discrete state field `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("离散状态必须类型约束"),
        "field-error check2 should keep discrete-state-type governance entry.",
    );
}

#[test]
fn field_error_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/field_error.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub enum FieldErrorDataState {",
        "pub enum FieldErrorSource {",
        "pub enum FieldErrorMessageSource {",
        "pub struct FieldErrorStateInput {",
        "pub struct FieldErrorState {",
    ] {
        assert!(
            primitive_source.contains(required),
            "field-error primitive should keep typed machine-readable state contract `{required}`.",
        );
    }

    for required in [
        "pub fn resolve_view_model(",
        "resolve_state(FieldErrorStateInput {",
        "resolve_primitive_state(input)",
    ] {
        assert!(
            logic_source.contains(required),
            "field-error logic should centralize invalid-state normalization via `{required}`.",
        );
    }

    for required in [
        "data-state=move || state.get().data_state.as_attr()",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-message-source=move || state.get().message_source.as_attr()",
        "data-class-source=move || semantics.get().attrs.data_class_source",
    ] {
        assert!(
            view_source.contains(required),
            "field-error view should expose stable semantic marker `{required}`.",
        );
    }

    assert!(
        check2_source
            .contains("- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。"),
        "field-error check2 should mark type-system + semantic-marker governance item as complete.",
    );
}

#[test]
fn field_error_agent_contract_schema_is_machine_readable_without_dom_guessing() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/field_error.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub enum FieldErrorDataState {",
        "pub enum FieldErrorSource {",
        "pub enum FieldErrorMessageSource {",
        "pub const fn as_attr(self) -> &'static str {",
        "data-state=move || state.get().data_state.as_attr()",
        "data-tone=move || semantics.get().attrs.data_tone",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-message-source=move || state.get().message_source.as_attr()",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "resolve_state(FieldErrorStateInput {",
    ] {
        assert!(
            primitive_source.contains(required)
                || logic_source.contains(required)
                || view_source.contains(required),
            "field-error agent contract should remain typed and machine-readable via `{required}`.",
        );
    }

    for forbidden in [
        "data-ui-schema=",
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "field-error render chain should keep whitelist-only safe boundary and avoid `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。"),
        "field-error check2 should mark agent-contract schema checklist item as complete.",
    );
    assert!(
        check2_source.contains(
            "N/A（组件级增强）：`FieldError` 为非复杂交互组件，当前不强制 `data-ui-schema`"
        ),
        "field-error check2 should document why `data-ui-schema` is not required for simple component scope.",
    );
}

#[test]
fn field_error_llm_streaming_snapshot_scope_is_component_level_na() {
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    assert!(
        check2_source.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
        "field-error check2 should mark llm streaming/snapshot-definition checklist item as complete.",
    );
    assert!(
        check2_source.contains(
            "`FieldError` 为静态错误提示组件，不承载 LLM 文本输出渲染面；组件职责是稳定呈现已归一化的错误语义状态，不负责 token 流接入与分段拼接"
        ),
        "field-error check2 should document why streaming/snapshot definition is N/A at component scope.",
    );

    for forbidden in [
        "Streaming",
        "Snapshot",
        "LLM",
        "streaming",
        "snapshot",
        "token 流",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "field-error should not mix llm rendering responsibility into component implementation `{forbidden}`.",
        );
    }
}

#[test]
fn field_error_supports_snapshot_baseline_with_complete_config_input() {
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub struct FieldErrorLogicInput {",
        "pub fn resolve_view_model(input: FieldErrorLogicInput) -> FieldErrorViewModel",
        "default_message: Some(common.field_error_default_message.as_ref().to_string())",
        "default_aria_label: Some(common.field_error_aria_label.as_ref().to_string())",
        "let view_model = logic::resolve_view_model(logic::FieldErrorLogicInput {",
    ] {
        assert!(
            logic_source.contains(required) || view_source.contains(required),
            "field-error should consume complete snapshot config and normalize it via `{required}`.",
        );
    }

    for required in [
        "data-state=move || state.get().data_state.as_attr()",
        "aria-label=move || semantics.get().attrs.aria_label.clone()",
        "role=move || semantics.get().attrs.role",
    ] {
        assert!(
            view_source.contains(required),
            "field-error snapshot rendering should stably expose semantic output `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。"),
        "field-error check2 should mark snapshot-baseline checklist item as complete.",
    );
    assert!(
        check2_source.contains("`FieldError` 只消费上层提供的完整 props 配置"),
        "field-error check2 should document snapshot baseline rationale for complete config input.",
    );
}

#[test]
fn field_error_streaming_requirement_is_optional_and_stays_in_upper_layer() {
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    assert!(
        check2_source.contains("- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。"),
        "field-error check2 should mark streaming-required-vs-optional checklist item as complete.",
    );
    assert!(
        check2_source.contains(
            "`FieldError` 非正文阅读面组件，按 `Streaming Optional` 处理并固定 `fallback=snapshot`"
        ),
        "field-error check2 should document streaming-optional fallback rationale.",
    );
    assert!(
        check2_source
            .contains("`草稿/已验证/可提交` 状态标识与数据校验/断线恢复/重试由上层流程负责"),
        "field-error check2 should keep upper-layer ownership for llm workflow status/retry semantics.",
    );

    for required in [
        "role=move || semantics.get().attrs.role",
        "aria-live=move || semantics.get().attrs.aria_live",
        "data-state=move || state.get().data_state.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "field-error should keep continuous semantic output marker `{required}`.",
        );
    }

    for forbidden in [
        "draft",
        "verified",
        "submittable",
        "retry",
        "reconnect",
        "stream",
        "token",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "field-error should not absorb upper-layer llm workflow responsibility `{forbidden}`.",
        );
    }
}

#[test]
fn field_error_rust_hygiene_forbids_unwrap_expect_and_uses_cow_for_string_hotspot() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let styles_source = load_source("../../components/field-error/src/styles.rs");
    let protocol_source = load_source("../../components/field-error/src/protocol.rs");
    let check2_source = include_str!("../check2.md");

    for source in [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &protocol_source,
    ] {
        for forbidden in ["unwrap(", "expect(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "field-error non-test code should forbid hygiene anti-pattern `{forbidden}`.",
            );
        }
    }

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![",
        "Cow::Borrowed(\"ui-field-error\")",
        "classes.push(Cow::Owned(base_class_name));",
    ] {
        assert!(
            logic_source.contains(required),
            "field-error logic should use Cow for class-name string hotspot `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。"),
        "field-error check2 should mark rust-hygiene checklist item as complete.",
    );
    assert!(
        check2_source.contains(
            "`logic.rs::compose_class_name` 已使用 `Cow<'static, str>` 收敛静态类名片段分配热点"
        ),
        "field-error check2 should document Cow hotspot rationale for rust hygiene entry.",
    );
}

#[test]
fn field_error_consumes_state_primitive_from_ui_state_primitives() {
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "use ui_state_primitives::field_error::{",
        "use ui_state_primitives::error_message::{",
        "resolve_state as resolve_error_message_state",
        "resolve_state as resolve_primitive_state",
        "pub use ui_state_primitives::field_error::FieldErrorTone;",
        "pub fn to_error_message_tone(tone: FieldErrorTone) -> ErrorMessageTone {",
        "resolve_primitive_state(input)",
    ] {
        assert!(
            logic_source.contains(required),
            "field-error logic should consume state primitive contract `{required}`.",
        );
    }

    for forbidden in [
        "FieldErrorDataState::Hidden",
        "FieldErrorSource::Custom",
        "FieldErrorMessageSource::None",
        "use leptos::",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "field-error logic should avoid local primitive/state-store implementation `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("状态原语来源正确"),
        "field-error check2 should keep state primitive source governance entry.",
    );
    assert!(
        check2_source.contains("- [x] `status-primitives` 定义：纯状态原语层"),
        "field-error check2 should mark status-primitives architecture checklist item as complete.",
    );
}

#[test]
fn field_error_focus_stack_requirement_is_na_for_non_overlay_component() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "NodeRef",
        "FallbackTo",
        "FocusManager",
        "focus_stack",
        "restore_focus",
        "document.body",
        "aria-modal",
        "role=\"dialog\"",
        "role=\"menu\"",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "field-error non-overlay component should not own focus-stack/overlay contract `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 焦点全局栈（Focus Stack & GC）"),
        "field-error check2 should mark focus-stack checklist item as complete.",
    );
    assert!(
        check2_source.contains("N/A：`FieldError` 为非 Overlay 的静态错误提示组件"),
        "field-error check2 should explicitly record non-overlay N/A rationale for focus-stack governance.",
    );
}

#[test]
fn field_error_foreign_zone_escape_hatch_is_na_without_imperative_third_party_instances() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "YieldControl",
        "CleanupForeign",
        "ForeignZone",
        "foreign_zone",
        "JsValue",
        "wasm_bindgen",
        "web_sys::",
        "extern \"C\"",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "field-error should not couple imperative third-party/foreign-zone contract `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 受控外交特区（Escape Hatches）"),
        "field-error check2 should mark escape-hatch checklist item as complete.",
    );
    assert!(
        check2_source.contains(
            "N/A：`FieldError` 为静态错误提示组件，不集成 ECharts/Map 等命令式第三方实例"
        ),
        "field-error check2 should explicitly record N/A rationale for foreign-zone governance.",
    );
}

#[test]
fn field_error_hydration_discontinuity_rule_is_na_without_time_or_random_id_initialization() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "now()",
        "SystemTime::now",
        "Instant::now",
        "Date::now",
        "Uuid::",
        "new_v4",
        "rand::",
        "getrandom",
        "nanoid",
        "IdProvider",
        "id_provider",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "field-error should not introduce hydration-unstable initializer `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] SSR 时空断裂治理（Hydration Discontinuity）"),
        "field-error check2 should mark hydration-discontinuity checklist item as complete.",
    );
    assert!(
        check2_source
            .contains("N/A：`FieldError` 不生成运行时 ID、无时间戳驱动初始化，也不依赖随机源"),
        "field-error check2 should explicitly record hydration-discontinuity N/A rationale.",
    );
}

#[test]
fn field_error_ssr_cross_platform_rule_is_na_with_explicit_non_web_constraints() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "web_sys::",
        "window(",
        "window().",
        "document.",
        "navigator.",
        "HtmlElement",
        "JsValue",
        "wasm_bindgen",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "field-error non-wasm path should not depend on browser object `{forbidden}`.",
        );
    }

    for forbidden in [
        "cfg(target_arch = \"wasm32\")",
        "cfg(feature = \"ssr\")",
        "cfg(feature = \"web\")",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "field-error should not hide platform difference behind ad-hoc cfg branch `{forbidden}`.",
        );
    }

    assert!(
        check2_source
            .contains("- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。"),
        "field-error check2 should mark ssr-cross-platform checklist item as complete.",
    );
    assert!(
        check2_source.contains("N/A：`FieldError` 组件自身不含平台分支与浏览器专有 API"),
        "field-error check2 should explicitly record ssr-cross-platform N/A rationale.",
    );
}

#[test]
fn field_error_respects_ui_headless_web_ssr_mutex_compile_error_contract() {
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let headless_cargo_source = load_source("../../crates/ui-headless/Cargo.toml");
    let field_error_cargo_source = include_str!("../Cargo.toml");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(required),
            "ui-headless should keep web/ssr mutual-exclusion guard `{required}`.",
        );
    }

    for required in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            headless_cargo_source.contains(required),
            "ui-headless feature table should preserve `{required}`.",
        );
    }

    assert!(
        field_error_cargo_source.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "field-error should consume ui-headless without overriding mutual-exclusion feature contract.",
    );

    for required in [
        "use_error_message(ErrorMessageOptions {",
        "let i18n = use_ui_i18n();",
    ] {
        assert!(
            view_source.contains(required),
            "field-error should use ui-headless APIs via `{required}` without redefining feature rules.",
        );
    }

    assert!(
        check2_source
            .contains("- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。"),
        "field-error check2 should mark ui-headless web/ssr mutex checklist item as complete.",
    );
}

#[test]
fn field_error_motion_non_wasm_noop_contract_is_satisfied_without_component_motion_dependency() {
    let motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib_source.contains(required),
            "ui-motion should keep non-wasm predictable no-op contract `{required}`.",
        );
    }

    for forbidden in [
        "mod motion;",
        "pub mod motion;",
        "attach_motion",
        "ui_motion::",
        "motion::",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "field-error should not assume motion handle/contract in non-wasm path `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。"),
        "field-error check2 should mark ui-motion non-wasm stub checklist item as complete.",
    );
}

#[test]
fn field_error_reduced_motion_ssr_wasm_branch_coverage_is_satisfied_by_static_non_motion_design() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let local_semantics = include_str!("semantics.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "cfg(target_arch = \"wasm32\")",
        "cfg(feature = \"ssr\")",
        "prefers_reduced_motion",
        "attach_motion",
        "ui_motion::",
        "motion::",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field-error static component should not split semantics by platform/motion branch `{forbidden}`.",
        );
    }

    for required in [
        "field_error_motion_non_wasm_noop_contract_is_satisfied_without_component_motion_dependency",
        "field_error_ssr_cross_platform_rule_is_na_with_explicit_non_web_constraints",
        "field_error_hydration_discontinuity_rule_is_na_without_time_or_random_id_initialization",
    ] {
        assert!(
            local_semantics.contains(required),
            "field-error semantics suite should keep branch-coverage guard `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。"),
        "field-error check2 should mark reduced-motion/SSR/wasm coverage item as complete.",
    );
    assert!(
        check2_source.contains("N/A：`FieldError` 不承载组件级动效实现"),
        "field-error check2 should explicitly record reduced-motion/SSR/wasm N/A rationale.",
    );
}

#[test]
fn field_error_performance_governance_is_na_with_static_render_contract() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    assert!(
        mod_source.contains("mod view;") && !mod_source.contains("mod motion;"),
        "field-error should keep static non-motion module shape for predictable performance baseline.",
    );

    for required in [
        "let view_model = logic::resolve_view_model(",
        "let state = Memo::new(move |_| resolved_state);",
        "let semantics = Memo::new(move |_| {",
    ] {
        assert!(
            view_source.contains(required),
            "field-error render path should stay deterministic for performance budgeting; missing `{required}`.",
        );
    }

    for forbidden in [
        "create_effect",
        "create_resource",
        "create_action",
        "spawn_local",
        "set_timeout",
        "set_interval",
        "request_animation_frame",
        "on:click",
        "on:keydown",
        "attach_motion",
        "ui_motion::",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "field-error should not introduce extra runtime update drivers for static performance contract `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains(
            "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。"
        ),
        "field-error check2 should mark performance governance checklist item as complete.",
    );
    assert!(
        check2_source.contains("N/A（组件级）：`FieldError` 为静态错误提示组件"),
        "field-error check2 should explicitly document component-level N/A performance rationale.",
    );
    assert!(
        check2_source
            .contains("`Button`/`Input` 的 `render_count=1` 预算与自动化基线属于仓库级任务"),
        "field-error check2 should pin repository-level render_count baseline ownership.",
    );
}

#[test]
fn field_error_semantic_and_performance_regression_item_is_satisfied_with_component_scope_na() {
    let local_semantics = include_str!("semantics.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "field_error_semantics_contract_is_exposed_via_headless_and_state_markers",
        "field_error_tests_prioritize_semantic_contracts_over_visual_snapshots",
        "field_error_performance_governance_is_na_with_static_render_contract",
    ] {
        assert!(
            local_semantics.contains(required),
            "field-error should keep semantic/performance regression guard `{required}`.",
        );
    }

    for required in [
        "role=move || semantics.get().attrs.role",
        "aria-live=move || semantics.get().attrs.aria_live",
        "data-state=move || state.get().data_state.as_attr()",
        "data-message-source=move || state.get().message_source.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "field-error should expose semantic marker `{required}` for regression coverage.",
        );
    }

    for forbidden in [
        "tabindex=",
        "on:focus",
        "on:blur",
        "on:keydown",
        "focusin",
        "focusout",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "field-error static component should not introduce standalone focus-flow contract `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。"),
        "field-error check2 should mark semantic-test-and-performance-regression checklist item as complete.",
    );
    assert!(
        check2_source.contains(
            "N/A（组件级）：`FieldError` 为非交互静态错误提示节点，不存在可独立验证的焦点流转路径"
        ),
        "field-error check2 should document focus-flow N/A rationale for static component scope.",
    );
    assert!(
        check2_source.contains("`render_count` 强约束面向高频/重型组件"),
        "field-error check2 should document render_count ownership for heavy-component scope.",
    );
}

#[test]
fn field_error_version_deprecation_migration_rule_is_component_stage_na() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let protocol_source = load_source("../../components/field-error/src/protocol.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_MESSAGE, FieldErrorTone};",
        "pub use view::FieldError;",
    ] {
        assert!(
            mod_source.contains(required),
            "field-error public export should remain stable for non-breaking component stage `{required}`.",
        );
    }

    for required in ["schema_version", "V1"] {
        assert!(
            protocol_source.contains(required),
            "field-error protocol should keep current v1 schema marker `{required}`.",
        );
    }

    for forbidden in ["migrate_v1_to_v2", "deprecated_window", "schema_registry"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "field-error should not introduce fake deprecation migration surface `{forbidden}` without real major break.",
        );
    }

    assert!(
        check2_source.contains("- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。"),
        "field-error check2 should mark version-deprecation-migration checklist item as complete.",
    );
    assert!(
        check2_source.contains("N/A（组件级阶段）：`FieldError` 本轮未发生跨大版本 API 破坏升级"),
        "field-error check2 should document no-major-break N/A rationale for migration rule.",
    );
}

#[test]
fn field_error_view_macro_complexity_is_bounded_with_single_shallow_template() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        (1..=3).contains(&view_macro_count),
        "field-error `view!` usage should stay bounded (1..=3), found {view_macro_count}.",
    );

    let show_count = view_source.matches("<Show ").count();
    assert!(
        show_count <= 2,
        "field-error should keep shallow conditional nesting in `view!`; found {show_count} `<Show>` blocks.",
    );

    for forbidden in ["<For ", "<Index ", "<Suspense ", "<Transition "] {
        assert!(
            !view_source.contains(forbidden),
            "field-error should avoid collection/heavy macro expansion in view template `{forbidden}`.",
        );
    }

    for (slot, expected) in [
        ("data-slot=\"field-error\"", 1usize),
        ("data-slot=\"field-error-icon\"", 1usize),
        ("data-slot=\"field-error-text\"", 1usize),
    ] {
        let actual = view_source.matches(slot).count();
        assert_eq!(
            actual, expected,
            "field-error should keep non-duplicated semantic fragment `{slot}`; expected {expected}, got {actual}.",
        );
    }

    assert!(
        check2_source.contains("- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。"),
        "field-error check2 should mark view macro complexity checklist item as complete.",
    );
    assert!(
        check2_source.contains("已满足：`FieldError` 当前仅 1 个 `view!` 宏块"),
        "field-error check2 should document explicit bounded-macro rationale.",
    );
}

#[test]
fn field_error_prefers_functional_subviews_without_component_noise() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "field-error should keep a single `#[component]` boundary for public API.",
    );

    for required in [
        "fn field_error_icon_view() -> impl IntoView {",
        "fn field_error_text_view(message: StoredValue<Option<String>>) -> impl IntoView {",
        "{field_error_icon_view()}",
        "{field_error_text_view(message)}",
    ] {
        assert!(
            view_source.contains(required),
            "field-error should extract light UI fragments into plain functions; missing `{required}`.",
        );
    }

    for forbidden in [
        "#[component]\nfn FieldErrorIcon",
        "#[component]\nfn FieldErrorText",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field-error should not promote light fragments into extra components `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。"),
        "field-error check2 should mark functional-splitting checklist item as complete.",
    );
    assert!(
        check2_source.contains(
            "`field_error_icon_view()` 与 `field_error_text_view(..)` 已从主 `view!` 提取为普通函数"
        ),
        "field-error check2 should document functional-splitting rationale.",
    );
}

#[test]
fn field_error_static_fragments_are_centralized_with_single_icon_asset_path() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    assert!(
        view_source.contains("const FIELD_ERROR_ICON_GLYPH: &str = \"⚠\";"),
        "field-error should centralize static icon asset as a named constant.",
    );
    assert!(
        view_source.contains("fn field_error_icon_view() -> impl IntoView {"),
        "field-error should keep static icon template in a single function entry.",
    );
    assert!(
        view_source.contains("{FIELD_ERROR_ICON_GLYPH}"),
        "field-error icon view should consume centralized static icon constant.",
    );
    assert!(
        view_source.contains("data-slot=\"field-error-icon\"")
            && view_source.contains("aria-hidden=\"true\""),
        "field-error static icon fragment should preserve accessibility semantics.",
    );

    assert!(
        check2_source.contains("- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。"),
        "field-error check2 should mark static-fragment centralization checklist item as complete.",
    );
    assert!(
        check2_source.contains(
            "已集中到 `FIELD_ERROR_ICON_GLYPH` 并通过 `field_error_icon_view()` 单点挂载"
        ),
        "field-error check2 should document centralized static-fragment rationale.",
    );
}

#[test]
fn field_error_disallows_inner_html_injection_paths() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "insert_adjacent_html",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "field-error should not expose html injection sinks `{forbidden}`.",
        );
    }

    assert!(
        view_source.contains("{move || message.get_value().unwrap_or_default()}"),
        "field-error should keep user-facing message rendering as text node, not HTML injection.",
    );

    assert!(
        check2_source.contains("- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。"),
        "field-error check2 should mark inner_html safety checklist item as complete.",
    );
    assert!(
        check2_source.contains("`FieldError` 未使用 `inner_html`/`set_inner_html` 注入路径"),
        "field-error check2 should explicitly document no-inner_html rationale.",
    );
}

#[test]
fn field_error_wasm_debug_contract_is_na_and_feature_isolation_is_preserved() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let cargo_toml = include_str!("../Cargo.toml");
    let check2_source = include_str!("../check2.md");

    assert!(
        cargo_toml.contains("[features]\ndefault = []")
            && !cargo_toml.contains("debug")
            && !cargo_toml.contains("devtools")
            && !cargo_toml.contains("replay"),
        "field-error crate should not leak wasm debug toggles into package features.",
    );

    for forbidden in [
        "trace",
        "TraceId",
        "replay",
        "timeline",
        "devtools",
        "debug_panel",
        "console_log",
        "console_error",
        "instrument(",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "field-error should not expose wasm debug/replay runtime hooks `{forbidden}`.",
        );
    }

    assert!(
        mod_source.contains("pub use view::FieldError;"),
        "field-error should keep stable public export for component entry.",
    );
    for forbidden in [
        "pub use debug",
        "pub use Debug",
        "pub use WasmDebug",
        "pub use Replay",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "field-error public module surface should not expose debug entry `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。"),
        "field-error check2 should mark wasm debug checklist item as complete.",
    );
    assert!(
        check2_source.contains("N/A（组件级）：`FieldError` 为静态错误提示组件"),
        "field-error check2 should explain why wasm debug requirement is N/A at component scope.",
    );
}

#[test]
fn field_error_dx_requirement_is_satisfied_by_static_css_and_playground_isolation() {
    let view_source = load_source("../../components/field-error/src/view.rs");
    let styles_source = load_source("../../components/field-error/src/styles.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = include_str!("../check2.md");

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "field-error should keep static css source for fast style iteration path.",
    );
    for forbidden in ["style=", ".set_property(", "web_sys::CssStyleDeclaration"] {
        assert!(
            !view_source.contains(forbidden) && !styles_source.contains(forbidden),
            "field-error should avoid runtime style mutation path `{forbidden}`.",
        );
    }

    assert!(
        !view_source.contains("signal(") && !view_source.contains("RwSignal"),
        "field-error should not introduce local interactive state that requires persisted hot-context recovery.",
    );

    for required in [
        "pub(super) fn field_error() -> AnyView {",
        "title=\"FieldError\"",
        "slug=\"field-error\"",
        "<Playground title=\"Visible + Tone\"",
        "<Playground title=\"Hidden + Disabled + Custom Class\"",
    ] {
        assert!(
            docs_source.contains(required),
            "field-error docs should provide isolated playground path `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。"),
        "field-error check2 should mark DX checklist item as complete.",
    );
    assert!(
        check2_source.contains("N/A（组件级）：`FieldError` 为静态错误提示组件"),
        "field-error check2 should document component-level DX N/A rationale.",
    );
    assert!(
        check2_source.contains("docs-app 已提供 `FieldError` 专属 `Playground` 隔离演练入口"),
        "field-error check2 should document playground isolation evidence for DX entry.",
    );
}

#[test]
fn field_error_engineering_contract_uses_serde_protocol_and_avoids_runtime_leaks() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let protocol_source = load_source("../../components/field-error/src/protocol.rs");
    let cargo_toml = include_str!("../Cargo.toml");
    let check2_source = include_str!("../check2.md");

    for required in [
        "use serde::{Deserialize, Serialize};",
        "Serialize, Deserialize",
        "#[serde(rename_all = \"snake_case\")]",
        "#[serde(default)]",
        "schema_version",
    ] {
        assert!(
            protocol_source.contains(required),
            "field-error protocol should keep serde/versioned schema contract `{required}`.",
        );
    }

    for forbidden in [
        "tokio",
        "async-std",
        "tokio::",
        "async_std::",
        "Runtime",
        "Handle",
    ] {
        assert!(
            !cargo_toml.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "field-error should not leak async runtime implementation detail `{forbidden}`.",
        );
    }

    for forbidden in [
        "tracing::",
        "tracing_subscriber",
        "span!",
        "event!",
        "#[instrument",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !mod_source.contains(forbidden),
            "field-error should not define custom tracing contract drift `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。"),
        "field-error check2 should mark engineering-contract checklist item as complete.",
    );
    assert!(
        check2_source
            .contains("`protocol.rs` 提供 `serde` 协议结构（含 `schema_version` 与默认值）"),
        "field-error check2 should document serde protocol evidence.",
    );
}

#[test]
fn field_error_styles_use_defensive_dual_fallback_variables_without_hardcoded_terminal_values() {
    let styles_source = load_source("../../components/field-error/src/styles.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-opacity-disabled, var(--ui-fallback-opacity-disabled))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-border-width-thin, var(--ui-fallback-border-width-thin))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-transparent, var(--ui-fallback-transparent))",
    ] {
        assert!(
            styles_source.contains(required),
            "field-error styles should keep defensive dual-fallback token chain `{required}`.",
        );
    }

    for forbidden in [
        "color: #",
        "background: #",
        "border-color: #",
        "outline-color: #",
        "12px",
        "14px",
        "16px",
        "20px",
        "1px",
        "2px",
        "0.0625rem",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "field-error styles should not keep hardcoded terminal literal `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "field-error check2 should mark defensive-variables checklist item as complete.",
    );
    assert!(
        check2_source.contains("终值统一落到 `--ui-fallback-*` 命名"),
        "field-error check2 should document defensive-variable rationale.",
    );
}

#[test]
fn field_error_css_is_aggregated_under_layer_ui_without_inline_style_paths() {
    let css_aggregate_source = load_source("../../crates/ui/src/css.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-field_error\")]",
        "out.push_str(crate::field_form::field_error::styles::CSS);",
    ] {
        assert!(
            css_aggregate_source.contains(required),
            "field-error css should be injected through @layer ui aggregation contract `{required}`.",
        );
    }

    for forbidden in [
        "style=",
        "style:\"",
        "style='",
        "style=\"top:",
        "style=\"left:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "field-error view should not contain ordinary inline style path `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "field-error check2 should mark @layer ui checklist item as complete.",
    );
    assert!(
        check2_source.contains("以 `@layer ui` 聚合并按 `component-field_error` 条件注入"),
        "field-error check2 should document @layer ui aggregation rationale.",
    );
}

#[test]
fn field_error_motion_contract_rule_is_na_with_zero_component_motion_surface() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let local_semantics = include_str!("semantics.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "mod motion;",
        "pub mod motion;",
        "attach_motion",
        "stiffness",
        "damping",
        "ui_motion::",
        "motion::",
        "prefers_reduced_motion",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "field-error should keep zero motion-surface in component layer `{forbidden}`.",
        );
    }

    for required in [
        "field_error_motion_non_wasm_noop_contract_is_satisfied_without_component_motion_dependency",
        "field_error_reduced_motion_ssr_wasm_branch_coverage_is_satisfied_by_static_non_motion_design",
    ] {
        assert!(
            local_semantics.contains(required),
            "field-error semantics should keep upstream motion/noop coverage guard `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。"),
        "field-error check2 should mark motion-contract checklist item as complete.",
    );
    assert!(
        check2_source.contains("N/A（组件级）：`FieldError` 为静态错误提示组件"),
        "field-error check2 should document component-level motion-contract N/A rationale.",
    );
}

#[test]
fn field_error_ui_components_fixed_entry_files_contract_is_satisfied() {
    let lib_source = load_source("../../crates/ui/src/lib.rs");
    let css_source = load_source("../../crates/ui/src/css.rs");
    let root_source = load_source("../../crates/ui/src/root.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "feature = \"component-field_error\",",
        "pub mod field_form {",
        "pub use field_form::field_error::{FieldError, FieldErrorTone};",
    ] {
        assert!(
            lib_source.contains(required),
            "ui lib entry should preserve field-error public gated export `{required}`.",
        );
    }

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-field_error\")]",
        "out.push_str(crate::field_form::field_error::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "ui css entry should keep conditional layer-injection contract `{required}`.",
        );
    }

    for required in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "ui root entry should keep centralized root-injection/i18n contract `{required}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion {",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "ui-visual-primitive active_highlight should keep shared highlight motion capability `{required}`.",
        );
    }

    let ui_components_src =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui/src");
    for missing in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src.join(missing).exists(),
            "ui should not reintroduce forbidden entry file `{missing}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] `ui` 固定入口文件落点正确。"),
        "field-error check2 should mark ui fixed-entry checklist item as complete.",
    );
    assert!(
        check2_source.contains(
            "`crates/ui/src/overlay_open.rs`、`presence.rs`、`a11y.rs` 在当前仓库均不存在"
        ),
        "field-error check2 should document forbidden-entry absence rationale.",
    );
}

#[test]
fn field_error_component_directory_standard_file_layout_is_satisfied() {
    let mod_source = load_source("../../components/field-error/src/mod.rs");
    let logic_source = load_source("../../components/field-error/src/logic.rs");
    let styles_source = load_source("../../components/field-error/src/styles.rs");
    let view_source = load_source("../../components/field-error/src/view.rs");
    let check2_source = include_str!("../check2.md");

    let component_src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            component_src.join(required).exists(),
            "field-error component directory should contain required file `{required}`.",
        );
    }
    for forbidden in ["render.rs", "motion.rs", "spec.rs"] {
        assert!(
            !component_src.join(forbidden).exists(),
            "field-error component directory should not introduce unnecessary file `{forbidden}`.",
        );
    }

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::FieldError;",
    ] {
        assert!(
            mod_source.contains(required),
            "field-error module entry should keep minimal stable exports via `{required}`.",
        );
    }
    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !mod_source.contains(forbidden),
            "field-error module entry should avoid over-export `{forbidden}`.",
        );
    }

    assert!(
        logic_source.contains("pub struct FieldErrorLogicInput {")
            && !logic_source.contains("web_sys::")
            && !logic_source.contains("NodeRef<"),
        "field-error logic should stay in normalization/derivation responsibility boundary.",
    );
    assert!(
        styles_source.contains("pub const CSS: &str = r#\"") && styles_source.contains("var(--ui-"),
        "field-error styles should remain static token-driven css contract.",
    );
    for required in [
        "use ui_headless::{",
        "A11yDirection, CommonStrings, ErrorMessageOptions, use_error_message, use_ui_i18n,",
        "};",
        "view! {",
    ] {
        assert!(
            view_source.contains(required),
            "field-error view should keep leptos structure + headless mounting contract `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 组件目录标准文件落点正确。"),
        "field-error check2 should mark component-directory-layout checklist item as complete.",
    );
    assert!(
        check2_source.contains("N/A（组件级）：`FieldError` 为静态错误提示组件"),
        "field-error check2 should document motion/spec N/A rationale for simple component.",
    );
    assert!(
        check2_source.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
        "field-error check2 should mark file-placement-discipline checklist item as complete.",
    );
    assert!(
        check2_source.contains("`FieldError` 是静态错误提示组件，不承载动效编排与复杂 schema，`motion.rs/spec.rs` 不应为“形式统一”强行引入"),
        "field-error check2 should document file-placement-discipline N/A rationale for motion/spec.",
    );
}
