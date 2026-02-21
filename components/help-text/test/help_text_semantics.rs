use std::fs;
use std::path::Path;

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_help_text_component_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir.join("components/help-text").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ui_components_reexports_help_text_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("pub mod field_form {")
            && lib_source.contains("#[cfg(feature = \"component-help_text\")]")
            && lib_source.contains("pub use ui_help_text as help_text;"),
        "ui-components field_form module should re-export the external ui-help-text crate as `help_text`.",
    );
    assert!(
        cargo_source.contains("component-help_text = [\"dep:ui-help-text\"]"),
        "component-help_text feature should depend on dep:ui-help-text after extraction.",
    );
    assert!(
        cargo_source
            .contains("ui-help-text = { path = \"../../components/help-text\", optional = true }"),
        "ui-components Cargo.toml should include the optional ui-help-text dependency.",
    );
}

#[test]
fn help_text_does_not_expose_logic_or_render_modules() {
    let source = load_help_text_component_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod render"] {
        assert!(
            !source.contains(needle),
            "HelpText internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn help_text_uses_layered_logic_and_headless_mount_model() {
    let logic_source = load_help_text_component_source("src/logic.rs");
    let render_source = load_help_text_component_source("src/view.rs");

    for needle in [
        "pub use ui_state_primitives::help_text::",
        "pub use ui_headless::A11yDirection;",
        "pub fn resolve_locale_attrs(",
        "pub fn resolve_error_live_region_attrs(",
        "pub fn resolve_agent_contract_attrs(",
        "pub fn resolve_render_model(",
        "pub struct HelpTextLogicInput",
        "HelpTextMessageKind",
        "HelpTextDataState",
        "HELP_TEXT_AGENT_SCHEMA",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "HelpText logic should include `{needle}` for layered state/headless composition."
        );
    }

    for needle in [
        "logic::resolve_render_model(logic::HelpTextLogicInput {",
        "let logic::HelpTextRenderModel {",
        "let class = StoredValue::new(logic::compose_class_name(class_name, resolved_state));",
        "let agent_contract = StoredValue::new(logic::resolve_agent_contract_attrs(resolved_state));",
        "state.get_value().message_kind.as_attr()",
        "state.get_value().data_state.as_attr()",
        "state.get_value().aria_source.as_attr()",
        "state.get_value().error_source.as_attr()",
        "state.get_value().class_source.as_attr()",
        "logic::resolve_locale_attrs(lang, dir)",
        "logic::resolve_error_live_region_attrs()",
        "#[prop(optional)] is_invalid: bool,",
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional)] is_error_icon_visible: bool,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
    ] {
        assert!(
            render_source.contains(needle),
            "HelpText view should mount logic/headless helpers; missing `{needle}`."
        );
    }

    assert!(
        !render_source.contains("unwrap_or_default")
            && !render_source.contains("logic::resolve_state(HelpTextStateInput {")
            && !render_source.contains("normalize_aria_label(aria_label)")
            && !render_source.contains("has_description = description.is_some()")
            && !render_source.contains("message_kind_attr")
            && !render_source.contains("data_state_attr")
            && !render_source.contains("aria_source_attr")
            && !render_source.contains("error_source_attr")
            && !render_source.contains("class_source_attr"),
        "HelpText view should not rebuild state normalization or rely on string-typed discrete status fields."
    );
}

#[test]
fn help_text_emits_baseline_style_state_data_attributes() {
    let source = load_help_text_component_source("src/view.rs");

    for attr in [
        "data-slot=\"help-text\"",
        "data-tone=move || state.get_value().tone_attr",
        "data-state=move || state.get_value().data_state.as_attr()",
        "data-message-kind=move || state.get_value().message_kind.as_attr()",
        "data-invalid=move || state.get_value().is_invalid.then_some(\"true\")",
        "data-disabled=move || state.get_value().is_disabled.then_some(\"true\")",
        "data-show-error-icon=move || state.get_value().show_error_icon.then_some(\"true\")",
        "data-has-description=move || state.get_value().has_description.then_some(\"true\")",
        "data-has-error=move || state.get_value().has_error_message.then_some(\"true\")",
        "data-aria-source=move || state.get_value().aria_source.as_attr()",
        "data-error-source=move || state.get_value().error_source.as_attr()",
        "data-custom-class=move || state.get_value().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get_value().class_source.as_attr()",
        "data-ui-schema=move || agent_contract.get_value().data_ui_schema",
        "data-ui-schema-version=move || agent_contract.get_value().data_ui_schema_version",
        "data-ui-intent=move || agent_contract.get_value().data_ui_intent",
        "data-ui-action=move || agent_contract.get_value().data_ui_action",
        "data-ui-state=move || agent_contract.get_value().data_ui_state",
        "data-ui-source=move || agent_contract.get_value().data_ui_source",
        "data-ui-stream-support=move || agent_contract.get_value().data_ui_stream_support",
        "data-ui-stream-mode=move || agent_contract.get_value().data_ui_stream_mode",
        "data-ui-stream-fallback=move || agent_contract.get_value().data_ui_stream_fallback",
        "data-ui-output-status=move || agent_contract.get_value().data_ui_output_status",
        "aria-label=aria_label",
        "aria-disabled=move || state.get_value().is_disabled.then_some(\"true\")",
        "aria-invalid=move || state.get_value().is_invalid.then_some(\"true\")",
        "role=error_live_region.role",
        "aria-live=error_live_region.aria_live",
    ] {
        assert!(
            source.contains(attr),
            "HelpText should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn help_text_styles_include_state_markers() {
    let source = load_help_text_component_source("src/styles.rs");

    for selector in [
        ".ui-help-text--tone-auto",
        ".ui-help-text[data-tone=\"negative\"]",
        ".ui-help-text--invalid",
        ".ui-help-text[data-invalid=\"true\"]",
        ".ui-help-text--disabled",
        ".ui-help-text[data-disabled=\"true\"]",
        ".ui-help-text__icon",
        ".ui-help-text__text",
        ".ui-help-text--custom-class",
        ".ui-help-text[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "HelpText styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn help_text_theme_contract_consumes_ui_theme_tokens_without_private_token_namespace() {
    let styles_source = load_help_text_component_source("src/styles.rs");
    let motion_source = load_help_text_component_source("src/motion.rs");

    for token_var in [
        "var(--ui-fg-muted)",
        "var(--ui-danger)",
        "var(--ui-accent)",
        "var(--ui-font-size-100",
        "var(--ui-line-height-100",
    ] {
        assert!(
            styles_source.contains(token_var),
            "HelpText styles should consume shared ui-theme token variables; missing `{token_var}`.",
        );
    }

    assert!(
        motion_source.contains("default_text_field_motion_tokens"),
        "HelpText motion contract should source duration/easing from ui-theme tokens."
    );
    assert!(
        !styles_source.contains("var(--help-text-")
            && !styles_source.contains("var(--ui-help-text-"),
        "HelpText should not introduce a private parallel token namespace in component styles.",
    );
}

#[test]
fn help_text_docs_page_covers_primary_playgrounds() {
    let source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn help_text() -> AnyView",
        "title=\"HelpText\"",
        "slug=\"help-text\"",
        "description=\"baseline-style form assistance primitive that resolves description vs error message and tone/icon state through centralized logic contracts.\"",
        "let help_text_imports =",
        "<Playground\n                title=\"Hello World (Default API)\"",
        "<Playground\n                title=\"State Matrix (Description / Error / Disabled)\"",
        "<Playground\n                title=\"Controlled vs Uncontrolled (Stateless Contract)\"",
        "<Playground\n                title=\"Streaming Optional (fallback=snapshot)\"",
        "<Playground title=\"Description (Neutral)\" code_signal=description_code>",
        "<Playground title=\"Invalid + Error Icon\" code_signal=error_code>",
        "code_imports=help_text_imports.clone()",
        "test_source_path=\"components/help-text/src/styles.rs\".to_string()",
        "data-slot=\"help-text-source-first\"",
        "docs-help-text-source-copy",
        "compose_copy_ready_code",
        "<HelpText",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra help_text docs page should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn help_text_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "title=\"Hello World (Default API)\"",
        "description=\"Use at least 12 characters.\".to_string()",
        "title=\"State Matrix (Description / Error / Disabled)\"",
        "title=\"Description (Neutral)\"",
        "aria_label=\"Password hint\".to_string()",
        "tone=HelpTextTone::Neutral",
        "description=\"This value is visible to project admins only.\".to_string()",
        "title=\"Invalid + Error Icon\"",
        "is_invalid=true",
        "is_error_icon_visible=true",
        "error_message=\"Password does not meet complexity requirements.\".to_string()",
        "class_name=\"docs-help-text-custom\".to_string()",
        "tone=HelpTextTone::Negative",
        "error_message=\"Two-factor token expired. Request a new code.\".to_string()",
        "is_disabled=true",
        "title=\"Controlled vs Uncontrolled (Stateless Contract)\"",
        "Uncontrolled snapshot: email must include @",
        "Controlled snapshot: email format is invalid",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "Streaming fallback=snapshot: waiting for final validation",
    ] {
        assert!(
            source.contains(needle),
            "help_text docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn help_text_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_help_text_component_source("check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2_source.contains(required),
            "help-text checklist should preserve e2e selector/stable-wait rule `{required}`."
        );
    }
}

#[test]
fn help_text_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source =
        load_ui_components_source("../../e2e/tests/docs_app_help_text_contract.spec.mjs");

    for needle in [
        "/#/components/help-text",
        "body:not(:has(#boot))",
        "[data-component=\"help-text\"]",
        "[data-slot=\"help-text\"][data-message-kind=\"description\"][data-state=\"description\"]",
        "[data-slot=\"help-text-workbench-controls\"]",
        "[data-slot=\"help-text-workbench-canvas\"]",
        "toHaveAttribute(\"data-ui-stream-support\", \"optional\")",
        "toHaveAttribute(\"data-ui-stream-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "[data-slot=\"help-text\"][data-message-kind=\"error\"][data-state=\"error\"][data-show-error-icon=\"true\"]",
        "toHaveAttribute(\"data-ui-action\", \"announce-error\")",
        "toHaveAttribute(\"role\", \"alert\")",
        "toHaveAttribute(\"aria-live\", \"assertive\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "help-text e2e selector/wait contract should include `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "help-text e2e should avoid fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn help_text_check2_documents_repeatable_key_flow_rules() {
    let check2_source = load_help_text_component_source("check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(required),
            "help-text checklist should preserve repeatable key-flow governance rule `{required}`."
        );
    }
}

#[test]
fn help_text_e2e_repeatable_key_flow_covers_semantic_breakpoints() {
    let e2e_source =
        load_ui_components_source("../../e2e/tests/docs_app_help_text_contract.spec.mjs");

    for needle in [
        "docs-app help-text key flow is repeatable with semantic contract breakpoints",
        "runRepeatableStateCycle(",
        "[data-slot=\"help-text-toggle-invalid\"] [data-slot=\"switch\"]",
        "[data-slot=\"help-text-toggle-disabled\"] [data-slot=\"switch\"]",
        "toHaveAttribute(\"data-state\", \"error\")",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-state\", \"error-disabled\")",
        "toHaveAttribute(\"data-message-kind\", \"error\")",
        "toHaveAttribute(\"data-ui-action\", \"announce-error\")",
        "toHaveAttribute(\"aria-live\", \"assertive\")",
        "await toggleInvalid.focus();",
        "await expect(toggleInvalid).toBeFocused();",
        "await toggleInvalid.press(\"Enter\");",
        "await toggleDisabled.focus();",
        "await expect(toggleDisabled).toBeFocused();",
        "await toggleDisabled.press(\"Enter\");",
        "await page.goto(\"/#/components/badge\");",
        "await expect(page.locator(\".docs-page-title\")).toHaveText(\"Badge\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "help-text e2e repeatable key-flow contract should include `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "help-text repeatable key-flow e2e should avoid brittle assertion/wait API `{forbidden}`."
        );
    }
}

#[test]
fn help_text_docs_examples_and_state_matrix_stay_synced_with_logic_api_defaults() {
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let view_source = load_help_text_component_source("src/view.rs");
    let logic_source = load_help_text_component_source("src/logic.rs");

    let section_start = docs_source
        .find("pub(super) fn help_text() -> AnyView")
        .unwrap_or_else(|| panic!("forms_extra.rs should contain help_text docs section."));
    let section_end = docs_source[section_start..]
        .find("pub(super) fn textarea() -> AnyView")
        .map(|offset| section_start + offset)
        .unwrap_or_else(|| panic!("help_text docs section should end before textarea section."));
    let help_text_docs = &docs_source[section_start..section_end];

    for required in [
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix (Description / Error / Disabled)\"",
        "title=\"Controlled vs Uncontrolled (Stateless Contract)\"",
        "title=\"Interactive Playground\"",
        "is_invalid=true",
        "is_disabled=true",
        "is_error_icon_visible=true",
        "tone=HelpTextTone::Neutral",
        "tone=HelpTextTone::Negative",
    ] {
        assert!(
            help_text_docs.contains(required),
            "help-text docs should keep synced example/matrix marker `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] tone: HelpTextTone,",
        "#[prop(optional)] is_invalid: bool,",
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional)] is_error_icon_visible: bool,",
    ] {
        assert!(
            view_source.contains(required),
            "help-text view API should expose `{required}` and docs must stay aligned."
        );
    }

    assert!(
        logic_source.contains("HelpTextTone::Auto"),
        "help-text logic should keep `HelpTextTone::Auto` as default normalization entrypoint."
    );
    assert!(
        help_text_docs
            .contains("<HelpText description=\"Use at least 12 characters.\".to_string() />"),
        "help-text docs hello-world should preserve minimal default path aligned with optional prop defaults."
    );

    for legacy in ["invalid=", "disabled=", "show_error_icon="] {
        assert!(
            !help_text_docs.contains(legacy),
            "help-text docs must not drift back to legacy prop alias `{legacy}`."
        );
    }
}

#[test]
fn help_text_documentation_is_beginner_friendly_with_readme_or_equivalent_entry() {
    let readme = load_help_text_component_source("src/README.md");
    let docs =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2 = load_help_text_component_source("check2.md");

    for needle in [
        "# HelpText",
        "## 先用起来（默认路径）",
        "### Hello World（最小可用）",
        "<HelpText description=\"Use at least 12 characters.\".to_string() />",
        "## 常见用法",
        "## 再进阶（高级控制）",
        "默认 API 路径优先",
        "不需要用户手动接线 `ui-state-primitives` / `ui-headless`",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs",
    ] {
        assert!(
            readme.contains(needle),
            "help-text README should include beginner-friendly marker `{needle}`."
        );
    }

    let hello_idx = readme
        .find("### Hello World（最小可用）")
        .expect("help-text README should contain Hello World section");
    let advanced_idx = readme
        .find("## 再进阶（高级控制）")
        .expect("help-text README should contain advanced section");
    assert!(
        hello_idx < advanced_idx,
        "help-text README should keep default path before advanced path."
    );

    for needle in [
        "pub(super) fn help_text() -> AnyView",
        "title=\"HelpText\"",
        "<Playground\n                title=\"Hello World (Default API)\"",
        "<Playground\n                title=\"State Matrix (Description / Error / Disabled)\"",
        "<Playground\n                title=\"Controlled vs Uncontrolled (Stateless Contract)\"",
        "<Playground\n                title=\"Interactive Playground\"",
    ] {
        assert!(
            docs.contains(needle),
            "docs-app equivalent entry should include `{needle}`."
        );
    }

    assert!(
        check2.contains("组件文档必须对新手友好（Documentation as Product）"),
        "help-text checklist should keep documentation-as-product governance item."
    );
}

#[test]
fn help_text_interactive_playground_contract_is_documented_and_traceable() {
    let docs =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let e2e = load_ui_components_source("../../e2e/tests/docs_app_help_text_contract.spec.mjs");
    let check2 = load_help_text_component_source("check2.md");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
        "AI Spec 联动示例 N/A（有依据）",
    ] {
        assert!(
            check2.contains(needle),
            "help-text checklist should keep interactive-playground governance marker `{needle}`."
        );
    }

    for needle in [
        "<Playground\n                title=\"Interactive Playground\"",
        "data-slot=\"help-text-workbench-controls\"",
        "data-slot=\"help-text-workbench-canvas\"",
        "Switch checked=is_invalid set_checked=set_is_invalid",
        "Switch checked=is_disabled set_checked=set_is_disabled",
        "Switch checked=is_error_icon_visible set_checked=set_is_error_icon_visible",
        "Switch checked=use_error_message set_checked=set_use_error_message",
    ] {
        assert!(
            docs.contains(needle),
            "help-text docs interactive playground should include `{needle}`."
        );
    }

    for needle in [
        "docs-app help-text key flow is repeatable with semantic contract breakpoints",
        "runRepeatableStateCycle(",
        "[data-slot=\"help-text-workbench-controls\"]",
        "[data-slot=\"help-text-workbench-canvas\"]",
    ] {
        assert!(
            e2e.contains(needle),
            "help-text e2e should keep repeatable interactive-playground marker `{needle}`."
        );
    }
}

#[test]
fn help_text_source_first_docs_are_copy_paste_ready_and_traceable() {
    let docs =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2 = load_help_text_component_source("check2.md");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
        "help_text_source_first_docs_are_copy_paste_ready_and_traceable",
    ] {
        assert!(
            check2.contains(needle),
            "help-text checklist should keep source-first copy-paste governance marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn help_text() -> AnyView",
        "let help_text_imports =",
        "code_imports=help_text_imports.clone()",
        "data-slot=\"help-text-source-first\"",
        "Source-first / Copy-Paste Ready",
        "Copied snippets are import-ready via ",
        "compose_copy_ready_code",
        "Snippet",
        "copyable=true",
        "docs-help-text-source-copy",
        "use ui_components::{HelpText, HelpTextTone};",
        "data-slot=\"help-text-source-paths\"",
        "\"components/help-text/src/mod.rs\"",
        "\"components/help-text/src/logic.rs\"",
        "\"components/help-text/src/view.rs\"",
        "\"components/help-text/src/styles.rs\"",
        "\"components/help-text/src/motion.rs\"",
    ] {
        assert!(
            docs.contains(needle),
            "help-text source-first docs should include `{needle}`."
        );
    }
}

#[test]
fn help_text_heroui_strategy_and_component_docs_stay_synced() {
    let strategy = load_ui_components_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index = load_ui_components_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let readme = load_help_text_component_source("src/README.md");
    let check2 = load_help_text_component_source("check2.md");

    for needle in [
        "### HelpText 同步记录（2026-02-20）",
        "`HelpText` 参数主轴保持 `tone/is_invalid/is_disabled/is_error_icon_visible/description/error_message/aria_label/motion/class_name/lang/dir`",
        "component_doc!(\"HelpText\", \"help-text\", \"Forms\", forms_extra::help_text)",
        "研究文档补充判定：本轮为参数语义命名与文档验收面对齐，不引入新的 Spectrum/HeroUI 风格结论",
        "HeroUI 对齐结论：保持“默认路径简洁、进阶参数按需开启”的体验目标",
    ] {
        assert!(
            strategy.contains(needle),
            "HelpText HeroUI strategy sync should include `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"HelpText\", \"help-text\", \"Forms\", forms_extra::help_text)",
        "pub(super) fn help_text() -> AnyView",
        "slug=\"help-text\"",
        "data-slot=\"help-text-source-first\"",
    ] {
        assert!(
            docs_index.contains(needle) || docs_page.contains(needle),
            "HelpText docs entry/index should include `{needle}`."
        );
    }

    assert!(
        readme.contains("# HelpText")
            && readme
                .contains("apps/docs-app/src/pages/components/pages/forms_extra.rs::help_text()"),
        "HelpText README should stay accessible and point to docs-app entry."
    );

    assert!(
        check2.contains("HeroUI 对标文档与组件文档同步"),
        "HelpText checklist should keep HeroUI/doc sync governance item."
    );
}
