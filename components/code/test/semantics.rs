fn load_source(path: &str) -> &'static str {
    match path {
        "readme" => include_str!("../src/README.md"),
        "docs_display" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages/display.rs")
        }
        "docs_pages_catalog" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages.rs")
        }
        "heroui_strategy" => include_str!("../../../docs/spec/heroui-parameter-design-strategy.md"),
        "e2e_code_contract" => include_str!("../../../e2e/tests/docs_app_code_contract.spec.mjs"),
        "ui_components_css" => include_str!("../../../crates/ui-components/src/css.rs"),
        "ui_components_manifest" => include_str!("../../../crates/ui-components/Cargo.toml"),
        "ui_components_lib" => include_str!("../../../crates/ui-components/src/lib.rs"),
        "ui_components_root" => include_str!("../../../crates/ui-components/src/root.rs"),
        "ui_headless_manifest" => include_str!("../../../crates/ui-headless/Cargo.toml"),
        "ui_headless_lib" => include_str!("../../../crates/ui-headless/src/lib.rs"),
        "ui_motion_manifest" => include_str!("../../../crates/ui-motion/Cargo.toml"),
        "ui_motion_lib" => include_str!("../../../crates/ui-motion/src/lib.rs"),
        "web_demo_manifest" => include_str!("../../../apps/web-demo/Cargo.toml"),
        "check2" => include_str!("../check2.md"),
        "manifest" => include_str!("../Cargo.toml"),
        "protocol" => include_str!("../src/protocol.rs"),
        "component_toml" => include_str!("../src/Component.toml"),
        "component_rbi" => include_str!("../src/code.rbi"),
        "logic" => include_str!("../src/logic.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "view" => include_str!("../src/view.rs"),
        "mod" => include_str!("../src/mod.rs"),
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn code_view_mounts_headless_locale_contract() {
    let view = load_source("view");

    for required in [
        "use ui_headless::a11y::{A11yDirection, locale_attrs};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang",
        "dir=locale.dir",
    ] {
        assert!(
            view.contains(required),
            "code view should mount ui-headless locale attrs via `{required}`"
        );
    }
}

#[test]
fn code_view_keeps_state_decisions_in_logic() {
    let view = load_source("view");

    for required in [
        "let resolved = logic::resolve_view_state(CodeViewInput {",
        "let state = resolved.state;",
        "let class = resolved.class;",
    ] {
        assert!(
            view.contains(required),
            "code view should keep state derivation in logic via `{required}`"
        );
    }

    for forbidden in ["variant.unwrap_or", "unwrap_or_default()"] {
        assert!(
            !view.contains(forbidden),
            "code view should not perform default fallback logic: `{forbidden}`"
        );
    }
}

#[test]
fn code_module_keeps_public_surface_minimal() {
    let module = load_source("mod");

    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !module.contains(forbidden),
            "code module should keep internals private: `{forbidden}`"
        );
    }
}

#[test]
fn code_motion_contract_is_explicitly_na_without_component_motion_layer() {
    let view = load_source("view");
    let module = load_source("mod");
    let motion_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/motion.rs");

    assert!(
        !motion_file.exists(),
        "Code is a static display component; component-local motion layer should stay N/A."
    );

    for forbidden in ["attach_motion(", "ui_motion::", "use ui_motion"] {
        assert!(
            !view.contains(forbidden),
            "code view should not host motion engine wiring: `{forbidden}`"
        );
    }

    for forbidden in ["mod motion;", "pub mod motion;", "pub use motion::"] {
        assert!(
            !module.contains(forbidden),
            "code module should not export a component-local motion layer: `{forbidden}`"
        );
    }
}

#[test]
fn code_styles_consume_ui_theme_tokens_without_rebuilding_theme_system() {
    let styles = load_source("styles");
    let view = load_source("view");
    let logic = load_source("logic");

    for required in [
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(",
        "--ui-space-3xs,",
        "var(--ui-fallback-space-3xs, var(--ui-fallback-space-2xs))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
    ] {
        assert!(
            styles.contains(required),
            "code styles should consume theme token variable `{required}`"
        );
    }

    for forbidden in [
        "ThemeContext",
        "ThemeSystem",
        "ThemeColor",
        "ThemeScale",
        "ui_theme::",
    ] {
        assert!(
            !view.contains(forbidden),
            "code view should not rebuild theme mapping logic: `{forbidden}`"
        );
        assert!(
            !logic.contains(forbidden),
            "code logic should not rebuild theme mapping logic: `{forbidden}`"
        );
    }
}

#[test]
fn code_component_stays_in_ui_components_assembly_boundary() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::CodeVariant;",
        "pub use view::Code;",
    ] {
        assert!(
            module.contains(required),
            "code module should keep stable ui-components assembly export `{required}`"
        );
    }

    assert!(
        logic.contains("pub use ui_state_primitives::code::{"),
        "code logic should consume ui-state-primitives instead of reimplementing state primitives."
    );
    assert!(
        view.contains("use ui_headless::a11y::{A11yDirection, locale_attrs};"),
        "code view should mount headless a11y contract, not component-local semantics rewrites."
    );

    for forbidden in ["web_sys::", "web-sys", "JsValue", "HtmlElement", "NodeRef"] {
        assert!(
            !module.contains(forbidden),
            "code module public surface must not expose platform detail `{forbidden}`"
        );
        assert!(
            !logic.contains(forbidden),
            "code logic must not leak platform detail `{forbidden}`"
        );
        assert!(
            !view.contains(forbidden),
            "code view must not leak platform detail `{forbidden}`"
        );
    }

    assert!(
        styles.contains("pub const CSS: &str ="),
        "code styles should remain token-first static css contract."
    );
}

#[test]
fn code_file_responsibilities_stay_in_their_own_layers() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let motion_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/motion.rs");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::CodeVariant;",
        "pub use view::Code;",
    ] {
        assert!(
            module.contains(required),
            "mod.rs should keep minimal stable export surface via `{required}`"
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "mod motion;",
        "pub use motion::",
        "pub fn resolve_view_state(",
        "pub const CSS: &str =",
        "#[component]",
    ] {
        assert!(
            !module.contains(forbidden),
            "mod.rs should not carry implementation details: `{forbidden}`"
        );
    }

    for required in [
        "pub struct CodeViewInput",
        "pub struct CodeViewState",
        "pub fn compose_class_name(",
        "pub fn resolve_view_state(",
        "resolve_state(CodeStateInput {",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should own normalization/derivation contract via `{required}`"
        );
    }

    for forbidden in [
        "view! {",
        "<code",
        "data-slot=",
        "data-state=",
        "locale_attrs(",
        "style=",
    ] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs must avoid DOM/headless/style mounting detail: `{forbidden}`"
        );
    }

    for required in [
        "pub const CSS: &str =",
        "var(--ui-fg)",
        "var(--ui-bg)",
        ".ui-code[data-state=\"inline\"]",
        ".ui-code[data-state=\"block\"]",
    ] {
        assert!(
            styles.contains(required),
            "styles.rs should keep token-first static css contract via `{required}`"
        );
    }

    for forbidden in [
        "fn ",
        "pub fn ",
        "struct ",
        "impl ",
        "view! {",
        "#[component]",
        "locale_attrs(",
    ] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should not carry logic/view/headless responsibilities: `{forbidden}`"
        );
    }

    for required in [
        "use ui_headless::a11y::{A11yDirection, locale_attrs};",
        "let resolved = logic::resolve_view_state(CodeViewInput {",
        "let state = resolved.state;",
        "let class = resolved.class;",
        "view! {",
    ] {
        assert!(
            view.contains(required),
            "view.rs should render structure and mount headless contract via `{required}`"
        );
    }

    for forbidden in [
        "unwrap_or(",
        "unwrap_or_default()",
        "resolve_state(CodeStateInput {",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not hide state normalization decisions: `{forbidden}`"
        );
    }

    assert!(
        !motion_file.exists(),
        "Code is a static leaf component; motion.rs contract is N/A and should stay absent."
    );
}

#[test]
fn code_spec_rs_contract_is_explicitly_na_for_simple_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let readme = load_source("readme");
    let spec_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");

    assert!(
        !spec_file.exists(),
        "Code is a simple display component; spec.rs should stay N/A unless a real schema contract appears."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "CodeSpec",
        "Spec::new(",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "simple component should not introduce spec-layer artifacts: `{forbidden}`"
        );
    }

    for required in [
        "## Architecture Layers",
        "- `logic.rs`：`CodeVariant` 归一化与 class/state 派生。",
        "- `view.rs`：渲染 `<code>` 与稳定语义标记。",
        "- `styles.rs`：静态 CSS 契约。",
        "- `mod.rs`：最小导出面（`Code`、`CodeVariant`）。",
    ] {
        assert!(
            readme.contains(required),
            "without spec.rs, documentation should stay in component docs/checklist via `{required}`"
        );
    }
}

#[test]
fn code_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component() {
    let check2 = load_source("check2");
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let readme = load_source("readme");
    let spec_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");

    assert!(
        check2.contains("Hyper-Structure Builder"),
        "checklist should explicitly track hyper-structure builder gate."
    );

    assert!(
        !spec_file.exists(),
        "Code is not a complex schema-driven component; spec.rs should remain N/A."
    );

    for forbidden in [
        "CodeSpec",
        "Spec::new(",
        ".render()",
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !readme.contains(forbidden),
            "non-complex leaf component should not expose hyper-structure builder artifact: `{forbidden}`"
        );
    }
}

#[test]
fn code_context_compression_manifest_and_rbi_projection_are_present_and_aligned() {
    let check2 = load_source("check2");
    let component_toml = load_source("component_toml");
    let component_rbi = load_source("component_rbi");
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    assert!(
        check2.contains("上下文压缩协议（Manifest + RBI）"),
        "checklist should explicitly track context-compression manifest/rbi gate."
    );

    for required_file in ["Component.toml", "code.rbi"] {
        assert!(
            src_dir.join(required_file).exists(),
            "context-compression protocol should keep `{required_file}` in component source directory."
        );
    }

    for required in [
        "schema_version = \"1\"",
        "name = \"Code\"",
        "crate = \"ui-code\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"agent_contract_schema_markers\"",
        "name = \"snapshot_rendering\"",
    ] {
        assert!(
            component_toml.contains(required),
            "Component.toml should declare context-compression capability contract via `{required}`"
        );
    }

    for required in [
        "pub type CodeVariant = ui_state_primitives::code::CodeVariant;",
        "pub fn Code(",
        "variant: Option<CodeVariant>",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<ui_headless::a11y::A11yDirection>",
        "children: leptos::children::Children,",
    ] {
        assert!(
            component_rbi.contains(required),
            "code.rbi should project stable signature contract via `{required}`"
        );
    }
}

#[test]
fn code_agent_contract_schema_is_typed_and_prevents_dom_guess_or_script_injection() {
    let check2 = load_source("check2");
    let component_toml = load_source("component_toml");
    let component_rbi = load_source("component_rbi");
    let protocol = load_source("protocol");
    let logic = load_source("logic");
    let view = load_source("view");

    assert!(
        check2.contains("语义标记统一升级为 Agent Contract（Schema 化）"),
        "checklist should explicitly track agent-contract schema gate."
    );

    for required in [
        "name = \"agent_contract_schema_markers\"",
        "[[agent_contract]]",
        "schema = \"code.v1\"",
        "intent = \"display\"",
        "action = \"snapshot_render\"",
        "state_axes = [\"variant\", \"state\", \"custom_class\"]",
        "source_axes = [",
        "\"ui_state_primitives::code::resolve_state\"",
        "[[agent_contract_markers]]",
        "attr = \"data-variant\"",
        "attr = \"data-state\"",
        "attr = \"data-custom-class\"",
        "[[agent_contract_whitelist]]",
        "allowed = [\"children()\"]",
        "blocked = [\"inner_html\", \"<script\"]",
    ] {
        assert!(
            component_toml.contains(required),
            "Component.toml should keep typed agent-contract schema fields via `{required}`"
        );
    }

    for required in [
        "pub enum CodeAgentIntent",
        "pub enum CodeAgentAction",
        "pub enum CodeAgentStateAxis",
        "pub enum CodeAgentSourceAxis",
        "pub struct CodeAgentContract",
        "pub agent_contract: CodeAgentContract,",
    ] {
        assert!(
            protocol.contains(required),
            "protocol.rs should keep typed intent/action/state/source schema via `{required}`"
        );
    }

    for required in [
        "pub type CodeVariant = ui_state_primitives::code::CodeVariant;",
        "variant: Option<CodeVariant>",
        "class_name: Option<String>",
    ] {
        assert!(
            component_rbi.contains(required),
            "RBI projection should keep stable typed public contract field `{required}`"
        );
    }

    for required in [
        "let state = resolve_state(CodeStateInput {",
        "variant: input.variant.unwrap_or_default(),",
        "has_custom_class_name: class_name.is_some(),",
        "data-variant=state.variant_attr",
        "data-state=state.state_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "agent contract field should stay traceable to typed state axes and semantic markers: `{required}`"
        );
    }

    for forbidden in [
        "format!(\"data-",
        "concat!(\"data-",
        "data-state=class_name",
        "data-variant=class_name",
        "inner_html=",
        "dangerously_set_inner_html",
        "<script",
        "set_inner_html(",
        "eval(",
        "Function(",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "agent contract render path should remain whitelisted and reject script/guess-based binding: `{forbidden}`"
        );
    }
}

#[test]
fn code_llm_render_modes_are_limited_to_streaming_and_snapshot_display_semantics() {
    let check2 = load_source("check2");
    let readme = load_source("readme");
    let component_toml = load_source("component_toml");
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "流式在这里仅指 LLM 输出渲染（只看两种显示模式）",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            check2.contains(required),
            "checklist should freeze the two-mode LLM output display scope via `{required}`"
        );
    }

    for required in [
        "## LLM 输出显示模式约定（两种）",
        "`Streaming`：LLM 还在生成时，上层按增量内容更新 `children`，组件边收边渲染。",
        "`Snapshot`：LLM 生成完成后，上层一次性提供完整内容，组件一次性渲染。",
        "`Code` 不实现传输协议（SSE/WebSocket），只消费上层提供的文本渲染输入。",
    ] {
        assert!(
            readme.contains(required),
            "component docs should keep rendering-mode boundary explicit via `{required}`"
        );
    }

    assert!(
        component_toml.contains("name = \"snapshot_rendering\""),
        "Component.toml should keep snapshot rendering capability as the stable baseline."
    );

    assert!(
        view.contains("{children()}"),
        "Code view should keep display-only contract by rendering upstream children directly."
    );

    for forbidden in [
        "EventSource",
        "WebSocket",
        "SSE",
        "stream::",
        "tokio_stream",
        "create_resource(",
        "fetch(",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "code component should not embed transport/protocol semantics into display-only mode contract: `{forbidden}`"
        );
    }
}

#[test]
fn code_snapshot_is_foundational_and_accepts_complete_result_and_config() {
    let check2 = load_source("check2");
    let readme = load_source("readme");
    let component_toml = load_source("component_toml");
    let docs_display = load_source("docs_display");
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "`Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep foundational snapshot baseline via `{required}`"
        );
    }

    for required in [
        "`Snapshot`：LLM 生成完成后，上层一次性提供完整内容，组件一次性渲染。",
        "`Snapshot` 是 `Code` 默认基础能力：组件可直接消费完整内容与完整配置并稳定渲染。",
    ] {
        assert!(
            readme.contains(required),
            "readme should keep snapshot baseline contract explicit via `{required}`"
        );
    }

    for required in [
        "name = \"snapshot_rendering\"",
        "action = \"snapshot_render\"",
    ] {
        assert!(
            component_toml.contains(required),
            "Component.toml should keep snapshot capability contract via `{required}`"
        );
    }

    for required in [
        "{children()}",
        "let resolved = logic::resolve_view_state(CodeViewInput {",
        "variant: input.variant.unwrap_or_default(),",
    ] {
        assert!(
            view.contains(required) || logic.contains(required),
            "snapshot render baseline should remain stable and deterministic via `{required}`"
        );
    }

    for required in [
        "pub(super) fn code() -> AnyView {",
        "<Playground title=\"Hello World (Default API)\" code_signal=hello_world_code>",
        "<Code>\"cargo check -p ui-components\"</Code>",
        "test_config_signal=actual_config",
        "<Code variant=variant class_name=class_name.clone()>",
        "{content}",
    ] {
        assert!(
            docs_display.contains(required),
            "docs-app should demonstrate complete snapshot content/config path via `{required}`"
        );
    }
}

#[test]
fn code_streaming_policy_is_optional_with_snapshot_fallback_and_explicit_output_state_markers() {
    let check2 = load_source("check2");
    let readme = load_source("readme");
    let view = load_source("view");
    let logic = load_source("logic");
    let docs_display = load_source("docs_display");

    for required in [
        "`Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
    ] {
        assert!(
            check2.contains(required),
            "checklist should freeze streaming-required-vs-optional policy via `{required}`"
        );
    }

    for required in [
        "`Code` 不是正文阅读面：`Streaming Optional`，并固定 `fallback=snapshot`。",
        "`Code` 不实现传输协议（SSE/WebSocket），只消费上层提供的文本渲染输入。",
        "数据校验、断线恢复、重试策略由上层负责，组件层仅负责稳定渲染。",
    ] {
        assert!(
            readme.contains(required),
            "readme should keep streaming optional boundary explicit via `{required}`"
        );
    }

    for required in [
        "<code",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "data-ui-output-state=\"verified\"",
        "aria-live=\"off\"",
        "aria-busy=\"false\"",
    ] {
        assert!(
            view.contains(required),
            "view should expose stable streaming/output-state markers and aria continuity via `{required}`"
        );
    }

    for forbidden in [
        "EventSource",
        "WebSocket",
        "SSE",
        "stream::",
        "tokio_stream",
        "create_resource(",
        "retry(",
        "reconnect",
        "validate(",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "component layer should not absorb transport/retry/validation ownership: `{forbidden}`"
        );
    }

    assert!(
        docs_display.contains("pub(super) fn code() -> AnyView {"),
        "docs-app should still provide a stable code component page while streaming policy stays optional."
    );
}

#[test]
fn code_rust_hygiene_contract_bans_unwrap_expect_and_let_underscore_and_uses_cow_for_class_parts() {
    let check2 = load_source("check2");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let module = load_source("mod");
    let protocol = load_source("protocol");

    for required in [
        "代码卫生（Rust Hygiene）",
        "非测试代码中完全禁止 `unwrap/expect`",
        "禁止无处理的 `let _ = ...`",
        "字符串复制热点收敛为 `Cow<'static, str>`",
        "./scripts/check-rust-hygiene.sh",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep rust hygiene governance marker `{required}`"
        );
    }

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![",
        "Cow::Borrowed(\"ui-code\")",
        "Cow::Borrowed(\"ui-code--custom-class\")",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should keep class-name string churn under Cow contract via `{required}`"
        );
    }

    for forbidden in [".to_string()", "String::from(", ".to_owned()"] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should avoid string clone hotspots in non-test code: `{forbidden}`"
        );
    }

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !module.contains(forbidden)
                && !protocol.contains(forbidden),
            "non-test component source should not use forbidden hygiene pattern: `{forbidden}`"
        );
    }
}

#[test]
fn code_component_directory_standard_file_layout_is_enforced() {
    let check2 = load_source("check2");
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    assert!(
        check2.contains("组件目录标准文件落点正确"),
        "checklist should keep component-directory standard-file gate explicitly."
    );

    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            src_dir.join(required_file).exists(),
            "component source layout should include required file `{required_file}`"
        );
    }

    for forbidden_file in ["render.rs", "motion.rs", "spec.rs"] {
        assert!(
            !src_dir.join(forbidden_file).exists(),
            "code is a static leaf component; `{forbidden_file}` should stay absent unless scope changes."
        );
    }

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::CodeVariant;",
        "pub use view::Code;",
    ] {
        assert!(
            module.contains(required),
            "mod.rs should keep minimal stable export surface via `{required}`"
        );
    }

    for required in [
        "pub struct CodeViewInput",
        "pub struct CodeViewState",
        "pub fn resolve_view_state(",
        "resolve_state(CodeStateInput {",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should keep normalization/derivation responsibility via `{required}`"
        );
    }
    for forbidden in ["view! {", "<code", "data-slot=", "locale_attrs("] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should not absorb view/headless concerns: `{forbidden}`"
        );
    }

    assert!(
        styles.contains("pub const CSS: &str =")
            && styles.contains("var(--ui-fg, var(--ui-fallback-fg))")
            && styles.contains("var(--ui-bg, var(--ui-fallback-bg))"),
        "styles.rs should remain static token-first CSS contract."
    );
    for forbidden in [
        "ThemeContext",
        "ThemeSystem",
        "ui_theme::",
        "pub fn ",
        "impl ",
    ] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should avoid theme remapping and logic-layer artifacts: `{forbidden}`"
        );
    }

    for required in [
        "#[component]",
        "use ui_headless::a11y::{A11yDirection, locale_attrs};",
        "let resolved = logic::resolve_view_state(CodeViewInput {",
        "view! {",
    ] {
        assert!(
            view.contains(required),
            "view.rs should keep structure rendering + headless mounting via `{required}`"
        );
    }
    for forbidden in [
        "unwrap_or(",
        "unwrap_or_default()",
        "resolve_state(CodeStateInput {",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not hide normalization/state-kernel logic: `{forbidden}`"
        );
    }
}

#[test]
fn code_file_placement_discipline_contract_is_explicit_for_leaf_component_scope() {
    let check2 = load_source("check2");
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");

    assert!(
        check2.contains("文件落点纪律"),
        "checklist should explicitly track file-placement discipline gate."
    );

    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            src_dir.join(required_file).exists(),
            "file-placement discipline requires `{required_file}` in component source directory."
        );
    }

    // Code is a static leaf display component; motion/spec stay N/A unless scope changes.
    for forbidden_file in ["render.rs", "motion.rs", "spec.rs"] {
        assert!(
            !src_dir.join(forbidden_file).exists(),
            "file-placement discipline should keep `{forbidden_file}` absent for current component scope."
        );
    }

    assert!(
        module.contains("mod logic;")
            && module.contains("pub mod styles;")
            && module.contains("mod view;"),
        "mod.rs should keep stable assembly boundary without drifting file layout."
    );

    assert!(
        logic.contains("pub fn resolve_view_state(")
            && styles.contains("pub const CSS: &str =")
            && view.contains("#[component]"),
        "logic/styles/view files should each keep their canonical responsibility anchor."
    );
}

#[test]
fn code_token_first_static_style_contract_is_enforced() {
    let styles = load_source("styles");
    let view = load_source("view");
    let logic = load_source("logic");
    let ui_components_css = load_source("ui_components_css");
    let ui_components_root = load_source("ui_components_root");

    for required in [
        "pub const CSS: &str =",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "--ui-space-3xs,",
        "var(--ui-fallback-space-3xs, var(--ui-fallback-space-2xs))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
    ] {
        assert!(
            styles.contains(required),
            "code styles should stay token-first via `{required}`"
        );
    }

    for required in [
        "#[cfg(feature = \"component-code\")]",
        "out.push_str(crate::code::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui-components css aggregator should include code styles under feature gate: `{required}`"
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root.contains(required),
            "UiRoot should inject aggregated component css via `{required}`"
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view.contains(forbidden),
            "code view should avoid runtime inline style logic: `{forbidden}`"
        );
    }

    for forbidden in [
        "@apply ",
        "tailwind",
        "tw-",
        "stylist::",
        "stylex",
        "emotion",
        "styled(",
        "css!(",
        "css! {",
    ] {
        assert!(
            !styles.contains(forbidden) && !view.contains(forbidden) && !logic.contains(forbidden),
            "component layer should not default to utility-first/CSS-in-Rust patterns: `{forbidden}`"
        );
    }
}

#[test]
fn code_visual_desire_gate_is_explicitly_na_for_leaf_display_scope() {
    let readme = load_source("readme");
    let styles = load_source("styles");
    let semantics_suite = include_str!("semantics.rs");

    // Visual Desire baseline for default theme and key interactive components
    // (Button/Input/Overlay) is a repository-level gate. Code is a leaf display component.
    assert!(
        readme.contains("`Code` 是一个轻量的内联/块级代码展示组件"),
        "code docs should make component scope explicit, avoiding cross-component visual ownership drift."
    );

    for required in [
        "fn code_styles_consume_ui_theme_tokens_without_rebuilding_theme_system()",
        "fn code_token_first_static_style_contract_is_enforced()",
    ] {
        assert!(
            semantics_suite.contains(required),
            "code should enforce visual non-regression through token/theme contracts: `{required}`"
        );
    }

    for forbidden in [
        "color: #",
        "background: #",
        "border: 1px solid #",
        "rgb(",
        "hsl(",
        "bootstrap",
        "btn-",
        "form-control",
    ] {
        assert!(
            !styles.to_ascii_lowercase().contains(forbidden),
            "code styles should avoid rough hardcoded visual language drift: `{forbidden}`"
        );
    }
}

#[test]
fn code_tree_shaking_contract_is_feature_gated_end_to_end() {
    let check2 = load_source("check2");
    let ui_components_manifest = load_source("ui_components_manifest");
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");
    let web_demo_manifest = load_source("web_demo_manifest");

    for required in [
        "Tree Shaking & 特性剪裁",
        "组件必须注册到 `ui-components` 特性树",
        "`css.rs` 和 `lib.rs` 聚合必须受 feature 门控",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep tree-shaking gate explicit via `{required}`"
        );
    }

    for required in [
        "component-code = [\"dep:ui-code\"]",
        "ui-code = { path = \"../../components/code\", optional = true }",
        "inject-css = []",
    ] {
        assert!(
            ui_components_manifest.contains(required),
            "package mode should keep component-level feature gate for code: `{required}`"
        );
    }

    for required in [
        "#[cfg(feature = \"component-code\")]",
        "pub use ui_code as code;",
        "#[cfg(feature = \"component-code\")]\n    out.push_str(crate::code::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
    ] {
        assert!(
            ui_components_lib.contains(required) || ui_components_css.contains(required),
            "lib.rs/css.rs should only export or aggregate code under feature gates: `{required}`"
        );
    }

    for required in [
        "#[cfg(feature = \"all-components\")]\npub use all_components::*;",
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]\npub use web_demo_components::*;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "central re-export registries must stay behind explicit cfg gates: `{required}`"
        );
    }

    assert!(
        web_demo_manifest.contains("default-features = false")
            && web_demo_manifest.contains("features = [\"inject-css\", \"web-demo-components\"]"),
        "reverse dependency should use explicit demo feature bundle without implicitly enabling all-components."
    );
}

#[test]
fn code_type_system_and_semantic_markers_form_machine_readable_contract() {
    let logic = load_source("logic");
    let view = load_source("view");
    let readme = load_source("readme");
    let semantics_suite = include_str!("semantics.rs");

    for required in [
        "CodeVariant",
        "pub variant: Option<CodeVariant>,",
        "pub fn resolve_view_state(input: CodeViewInput) -> CodeViewState",
        "let state = resolve_state(CodeStateInput {",
        "variant: input.variant.unwrap_or_default()",
    ] {
        assert!(
            logic.contains(required),
            "type-level state space and normalization entry should stay explicit in logic.rs: `{required}`"
        );
    }

    for forbidden in [
        "variant: Option<String>",
        "variant: String",
        "mode: Option<String>",
        "status: Option<String>",
        "Option<bool>",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "invalid free-form/bool-explosion state space should remain inexpressible: `{forbidden}`"
        );
    }

    for required in [
        "data-variant=state.variant_attr",
        "data-state=state.state_attr",
        "data-inline=state.is_inline.then_some(\"true\")",
        "data-block=state.is_block.then_some(\"true\")",
    ] {
        assert!(
            view.contains(required),
            "critical state should stay machine-readable through stable semantic markers: `{required}`"
        );
    }

    assert!(
        readme.contains("`data-variant`（`inline` / `block`）")
            && readme.contains("`data-state`（`inline` / `block`）"),
        "docs should keep enumerable marker contract for automated consumers."
    );

    for required in [
        "fn code_discrete_state_contract_is_enum_typed_without_bool_or_string_state_machine()",
        "fn code_state_observability_uses_stable_enumerable_data_markers()",
    ] {
        assert!(
            semantics_suite.contains(required),
            "contract breakpoints should stay directly locatable via targeted tests: `{required}`"
        );
    }
}

#[test]
fn code_api_naming_contract_uses_stable_semantics_without_alias_drift() {
    let view = load_source("view");

    for required in [
        "pub fn Code(",
        "#[prop(optional, into)] variant: Option<CodeVariant>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "children: Children,",
    ] {
        assert!(
            view.contains(required),
            "code public api should keep stable naming contract via `{required}`"
        );
    }

    // Code is a display-only component: no controllable bool/event/default axis.
    for forbidden in [
        "is_inline:",
        "is_block:",
        "is_disabled:",
        "on_change:",
        "on_open_change:",
        "default_open:",
        "default_variant:",
    ] {
        assert!(
            !view.contains(forbidden),
            "code api should avoid semantic alias drift for naming contract: `{forbidden}`"
        );
    }
}

#[test]
fn code_controllable_contract_is_explicitly_na_without_half_controlled_api() {
    let view = load_source("view");
    let logic = load_source("logic");

    // Code has no controllable state axis; avoid introducing partial value/default/on triplets.
    for forbidden in [
        "value:",
        "default_value:",
        "on_value_change:",
        "open:",
        "default_open:",
        "on_open_change:",
    ] {
        assert!(
            !view.contains(forbidden),
            "code api should stay display-only and avoid half-controlled props: `{forbidden}`"
        );
    }

    // No component-local signal state writes that could create implicit controlled/uncontrolled split.
    for forbidden in [
        "RwSignal<",
        "ReadSignal<",
        "WriteSignal<",
        "create_signal(",
        ".set(",
    ] {
        assert!(
            !logic.contains(forbidden),
            "code logic should not introduce implicit local state authority: `{forbidden}`"
        );
    }
}

#[test]
fn code_state_normalization_is_centralized_in_logic_layer() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    for required in [
        "pub struct CodeViewInput",
        "pub struct CodeViewState",
        "pub fn resolve_view_state(input: CodeViewInput) -> CodeViewState",
        "let class_name = normalize_optional_text(input.class_name);",
        "let state = resolve_state(CodeStateInput {",
        "variant: input.variant.unwrap_or_default()",
    ] {
        assert!(
            logic.contains(required),
            "code normalization pipeline should stay in logic via `{required}`"
        );
    }

    for required in [
        "let resolved = logic::resolve_view_state(CodeViewInput {",
        "let state = resolved.state;",
        "let class = resolved.class;",
    ] {
        assert!(
            view.contains(required),
            "code view should only consume normalized state via `{required}`"
        );
    }

    for forbidden in [
        "resolve_state(CodeStateInput",
        "normalize_optional_text(",
        "unwrap_or_default()",
    ] {
        assert!(
            !view.contains(forbidden),
            "code view must not rebuild normalization/state machine rule: `{forbidden}`"
        );
    }

    // styles consumes semantic markers only, no runtime normalization/state machine logic.
    for forbidden in ["if ", "match ", "unwrap_or", "resolve_state"] {
        assert!(
            !styles.contains(forbidden),
            "code styles should consume markers only, not state logic: `{forbidden}`"
        );
    }
}

#[test]
fn code_discrete_state_contract_is_enum_typed_without_bool_or_string_state_machine() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "pub use ui_state_primitives::code::{",
        "CodeVariant",
        "pub variant: Option<CodeVariant>,",
        "#[prop(optional, into)] variant: Option<CodeVariant>,",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "code discrete state should stay enum-typed via `{required}`"
        );
    }

    for forbidden in [
        "variant: Option<String>",
        "variant: String",
        "mode: Option<String>",
        "status: Option<String>",
        "size: Option<String>",
        "Option<bool>",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "code should not model mutually exclusive states with string/free bool combinations: `{forbidden}`"
        );
    }
}

#[test]
fn code_state_primitives_source_boundary_is_kept_in_logic_mapping_only() {
    let manifest = load_source("manifest");
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "ui-state-primitives = { path = \"../../crates/ui-state-primitives\" }",
        "pub use ui_state_primitives::code::{",
        "resolve_state(CodeStateInput {",
    ] {
        assert!(
            manifest.contains(required) || logic.contains(required),
            "code should consume state primitives via `{required}`"
        );
    }

    for forbidden in [
        "pub enum CodeVariant",
        "pub struct CodeStateInput",
        "pub struct CodeState",
        "pub fn resolve_state(",
    ] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should map primitives output only, not reimplement state primitive `{forbidden}`"
        );
    }

    for forbidden in [
        "ui-web-demo",
        "docs-app",
        "tauri-demo",
        "AppState",
        "GlobalStore",
        "use crate::store",
        "use app::store",
    ] {
        assert!(
            !manifest.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden),
            "component layer should not bind business store types directly: `{forbidden}`"
        );
    }
}

#[test]
fn code_async_interaction_contract_is_explicitly_na_for_static_component() {
    let manifest = load_source("manifest");
    let logic = load_source("logic");
    let view = load_source("view");

    // Code is a static display component; it should not define its own async loading/error/retry protocol.
    for forbidden in [
        "is_loading",
        "on_retry",
        "retry",
        "aria-busy",
        "disabled:",
        "use_async_action",
        "create_resource(",
        "Resource<",
        "Suspense",
        "Transition",
        "spawn_local",
        "tokio::",
        "async_std::",
    ] {
        assert!(
            !manifest.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden),
            "code component should keep async interaction semantics N/A and avoid `{forbidden}`"
        );
    }
}

#[test]
fn code_dx_paradox_keeps_default_usage_simple_and_internal_complexity_hidden() {
    let readme = load_source("readme");
    let docs_display = load_source("docs_display");
    let view = load_source("view");

    for required in [
        "## Hello World",
        "<Code>\"cargo test -p ui-components\"</Code>",
        "pub fn Code(",
        "children: Children,",
    ] {
        assert!(
            readme.contains(required) || view.contains(required),
            "code dx should keep default usage simple via `{required}`"
        );
    }

    assert!(
        docs_display.contains(
            "<Playground title=\"Hello World (Default API)\" code_signal=hello_world_code>"
        ),
        "docs-app should expose a copy-paste default Code path for zero-threshold onboarding."
    );
    assert!(
        docs_display.contains("<Code>\"cargo check -p ui-components\"</Code>"),
        "docs-app hello world should use default api path without forcing internal wiring."
    );

    for forbidden in [
        "state:",
        "state=",
        "headless_state:",
        "primitive_state:",
        "state: CodeState",
        "state: CodeStateInput",
    ] {
        assert!(
            !view.contains(forbidden),
            "code default api should not require internal state wiring: `{forbidden}`"
        );
    }
}

#[test]
fn code_documentation_as_product_readme_is_beginner_friendly_with_default_first_path() {
    let check2 = load_source("check2");
    let readme = load_source("readme");

    assert!(
        check2.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "checklist should record documentation-as-product gate completion."
    );

    for required in [
        "# Code",
        "## 快速开始（先用起来）",
        "## Hello World",
        "## 常见用法",
        "### Inline（默认）",
        "### Block（常见）",
        "## 进阶用法（可选）",
        "## docs-app Playground（展示区 / Config 区 / Code 区 / CSS Test 区）",
        "<Code>\"cargo test -p ui-components\"</Code>",
    ] {
        assert!(
            readme.contains(required),
            "readme should keep beginner-friendly onboarding marker `{required}`"
        );
    }

    let hello_world_idx = readme
        .find("## Hello World")
        .expect("README should contain Hello World section.");
    let common_idx = readme
        .find("## 常见用法")
        .expect("README should contain common usage section.");
    let advanced_idx = readme
        .find("## 进阶用法（可选）")
        .expect("README should contain advanced section.");

    assert!(
        hello_world_idx < common_idx && common_idx < advanced_idx,
        "README should keep progressive disclosure: default API first, common usage second, advanced path last."
    );

    for forbidden in [
        "必须先理解底层分层架构",
        "仅供架构师",
        "只有源码没有文档",
        "machine-only",
    ] {
        assert!(
            !readme.contains(forbidden),
            "beginner docs should avoid exclusionary wording: `{forbidden}`"
        );
    }
}

#[test]
fn code_dx_workbench_flow_supports_fast_style_iteration_and_context_retention() {
    let manifest = load_source("manifest");
    let docs_display = load_source("docs_display");
    let check2 = load_source("check2");

    for required in [
        "./scripts/dev-docs-app.sh",
        "./scripts/smoke-docs-app.sh",
        "cargo check -p ui-code --quiet",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep explicit DX verification command `{required}`"
        );
    }

    assert!(
        docs_display.contains("pub(super) fn code() -> AnyView {"),
        "docs app should provide isolated Code demo/workbench entry."
    );
    for required in [
        "<Playground\n                title=\"Interactive Playground\"",
        "description=\"展示区 + Config 区 + Code 区 + CSS Test 区；包含 inline/block 与 custom class 的对比展示。\"",
        "test_css_source=test_css_source",
        "test_source_path=\"components/code/src/styles.rs\".to_string()",
        "test_config_signal=actual_config",
        "let (variant_index, set_variant_index) = signal(Some(0_usize));",
        "let (custom_class, set_custom_class) = signal(false);",
        "let (long_content, set_long_content) = signal(false);",
        "let (show_compare, set_show_compare) = signal(true);",
        "<ui_components::SegmentedControl",
        "<ui_components::Switch checked=show_compare set_checked=set_show_compare>",
    ] {
        assert!(
            docs_display.contains(required),
            "code DX workbench should keep fast iteration/state-retention contract marker `{required}`"
        );
    }

    for forbidden in ["wasm-debug", "wasm_debug", "debug_overlay", "UiTrace"] {
        assert!(
            !manifest.contains(forbidden),
            "ui-code should not leak dev-only debug capability into public feature/API surface: `{forbidden}`"
        );
    }
}

#[test]
fn code_docs_product_copy_paste_ready_playground_covers_hello_matrix_control_streaming_and_source_first()
 {
    let check2 = load_source("check2");
    let docs_display = load_source("docs_display");

    assert!(
        check2.contains("- [x] 文档即产品（Copy-Paste Ready）"),
        "checklist should record docs-as-product copy-paste-ready gate as completed."
    );

    for required in [
        "<Playground title=\"Hello World (Default API)\" code_signal=hello_world_code>",
        "<Playground title=\"Variant Matrix\" code_signal=variants_code>",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=\"use leptos::prelude::*;\\nuse ui_components::{Code, CodeVariant};\".to_string()",
        "data-slot=\"code-state-matrix\"",
        "data-slot=\"code-streaming-modes\"",
        "data-slot=\"code-source-first\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "label=\"Copy code starter\".to_string()",
        "data-slot=\"code-source-paths\"",
        "component-code",
        "inject-css",
    ] {
        assert!(
            docs_display.contains(required),
            "docs-app code page should keep docs-product/copy-paste-ready contract marker `{required}`"
        );
    }
}

#[test]
fn code_source_first_docs_are_copy_paste_ready_with_imports_prerequisites_and_synced_example() {
    let check2 = load_source("check2");
    let docs_display = load_source("docs_display");
    let module = load_source("mod");
    let readme = load_source("readme");

    assert!(
        check2.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "checklist should record source-first copy-paste-ready gate completion."
    );

    for required in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=\"use leptos::prelude::*;\\nuse ui_components::{Code, CodeVariant};\".to_string()",
        "data-slot=\"code-source-first\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "label=\"Copy code starter\".to_string()",
        "copyable=true",
        "data-slot=\"code-source-prerequisites\"",
        "component-code",
        "inject-css",
        "data-slot=\"code-source-paths\"",
        "components/code/src/mod.rs",
        "components/code/src/logic.rs",
        "components/code/src/view.rs",
        "components/code/src/styles.rs",
        "use ui_components::{Code, CodeVariant};",
        "<Code variant=CodeVariant::Block>",
    ] {
        assert!(
            docs_display.contains(required),
            "source-first docs contract should include `{required}`"
        );
    }

    for required in ["pub use logic::CodeVariant;", "pub use view::Code;"] {
        assert!(
            module.contains(required),
            "source-first copied snippet should stay synchronized with exported public API `{required}`"
        );
    }

    assert!(
        readme.contains("Source-first：`Source-first Starter (Copy-Paste Ready)`"),
        "README should provide discoverable entry to source-first docs path."
    );
}

#[test]
fn code_heroui_benchmark_docs_and_component_docs_remain_synced() {
    let check2 = load_source("check2");
    let heroui = load_source("heroui_strategy");
    let docs_pages = load_source("docs_pages_catalog");
    let docs_display = load_source("docs_display");
    let readme = load_source("readme");
    let logic = load_source("logic");

    assert!(
        check2.contains("- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。"),
        "checklist should record HeroUI benchmark and docs-sync gate completion."
    );

    for required in [
        "### Code 同步记录（2026-02-20）",
        "components/code/src/logic.rs::resolve_view_state",
        "variant.unwrap_or_default()",
        "normalize_optional_text(input.class_name)",
        "component_doc!(\"Code\", \"code\", \"Display\", display::code)",
        "apps/docs-app/src/pages/components/pages/display.rs::code()",
        "Source-first Starter (Copy-Paste Ready)",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入。",
    ] {
        assert!(
            heroui.contains(required),
            "HeroUI strategy doc should keep code sync marker `{required}`"
        );
    }

    for required in [
        "component_doc!(\"Code\", \"code\", \"Display\", display::code)",
        "mod display;",
        "pub(super) const CATALOG: &[ComponentDoc] = &[",
    ] {
        assert!(
            docs_pages.contains(required),
            "docs catalog should expose code component entry `{required}`"
        );
    }

    for required in [
        "pub(super) fn code() -> AnyView {",
        "title=\"Code\"",
        "slug=\"code\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
    ] {
        assert!(
            docs_display.contains(required),
            "docs page should remain indexable and accessible via `{required}`"
        );
    }

    for required in [
        "docs-app entry: `apps/docs-app/src/pages/components/pages/display.rs::code()`",
        "feature: `component-code`（可选 `inject-css`）",
    ] {
        assert!(
            readme.contains(required),
            "component README should keep accessible docs entry marker `{required}`"
        );
    }

    for required in [
        "variant: input.variant.unwrap_or_default(),",
        "let class_name = normalize_optional_text(input.class_name);",
    ] {
        assert!(
            logic.contains(required),
            "logic parameter semantics should stay synchronized with HeroUI strategy docs via `{required}`"
        );
    }
}

#[test]
fn code_docs_app_interactive_playground_supports_live_prop_state_preview_and_repeatable_flow() {
    let check2 = load_source("check2");
    let docs_display = load_source("docs_display");
    let e2e = load_source("e2e_code_contract");

    assert!(
        check2.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "checklist should record interactive-playground gate completion."
    );

    for required in [
        "<Playground\n                title=\"Interactive Playground\"",
        "data-slot=\"code-workbench-controls\"",
        "data-slot=\"code-workbench-preview\"",
        "data-slot=\"code-workbench-primary\"",
        "data-slot=\"code-workbench-compare\"",
        "let (variant_index, set_variant_index) = signal(Some(0_usize));",
        "let (custom_class, set_custom_class) = signal(false);",
        "let (long_content, set_long_content) = signal(false);",
        "let (show_compare, set_show_compare) = signal(true);",
        "<ui_components::SegmentedControl",
        "<ui_components::Switch checked=custom_class set_checked=set_custom_class>",
        "<ui_components::Switch checked=long_content set_checked=set_long_content>",
        "<ui_components::Switch checked=show_compare set_checked=set_show_compare>",
        "test_config_signal=actual_config",
    ] {
        assert!(
            docs_display.contains(required),
            "docs-app should keep interactive playground prop/state live-preview contract via `{required}`"
        );
    }

    for required in [
        "async function runCodeCriticalFlow(page, docsRoot) {",
        "[data-slot=\"code-workbench-controls\"]",
        "[data-slot=\"code-workbench-primary\"] [data-slot=\"code\"]",
        "await expect(primaryCode).toHaveAttribute(\"data-variant\", \"block\");",
        "await expect(primaryCode).toHaveAttribute(\"data-custom-class\", \"true\");",
        "await page.reload();",
        "await runCodeCriticalFlow(page, reloadedDocsRoot);",
    ] {
        assert!(
            e2e.contains(required),
            "interactive playground acceptance flow should stay repeatable in E2E via `{required}`"
        );
    }
}

#[test]
fn code_e2e_selector_contract_uses_semantic_markers_and_wasm_stable_ready_waits() {
    let check2 = load_source("check2");
    let e2e = load_source("e2e_code_contract");

    for required in [
        "E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep E2E selector stability gate explicit via `{required}`"
        );
    }

    for required in [
        "page.goto(\"/#/components/code\")",
        "body:not(:has(#boot))",
        "[data-component=\"code\"]",
        "[data-slot=\"code\"]",
        "[data-ui-streaming=\"optional\"]",
        "[data-ui-fallback=\"snapshot\"]",
        "[data-ui-output-state=\"verified\"]",
        "[data-slot=\"code-state-matrix\"]",
        "[data-slot=\"code-streaming-modes\"]",
        "[data-slot=\"code-source-first\"]",
        "await page.reload();",
    ] {
        assert!(
            e2e.contains(required),
            "code E2E contract should use semantic ready/selector checkpoints via `{required}`"
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        ":nth-child",
        ":nth-of-type",
        "getByText(",
    ] {
        assert!(
            !e2e.contains(forbidden),
            "code E2E contract should avoid fragile selector or fixed-delay path: `{forbidden}`"
        );
    }
}

#[test]
fn code_e2e_critical_flow_is_repeatable_and_locates_semantic_breakpoints() {
    let check2 = load_source("check2");
    let e2e = load_source("e2e_code_contract");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep E2E critical-flow gate explicit via `{required}`"
        );
    }

    for required in [
        "async function runCodeCriticalFlow(page, docsRoot) {",
        "[data-slot=\"code-workbench-controls\"]",
        "[data-slot=\"code-workbench-primary\"] [data-slot=\"code\"]",
        "[data-slot=\"segmented-control-option\"][data-index=\"1\"]",
        "await page.keyboard.press(\"Space\");",
        "await expect(primaryCode).toHaveAttribute(\"data-variant\", \"block\");",
        "await expect(primaryCode).toHaveAttribute(\"data-custom-class\", \"true\");",
        "await expect(playground.locator('[data-slot=\"code-workbench-compare\"]')).toHaveCount(0);",
        "await page.reload();",
        "await runCodeCriticalFlow(page, reloadedDocsRoot);",
        "Code has no overlay/async path; prioritize focus + keyboard regression branch.",
    ] {
        assert!(
            e2e.contains(required),
            "code E2E critical flow should keep semantic breakpoint contract via `{required}`"
        );
    }
}

#[test]
fn code_docs_examples_and_parameter_state_matrix_stay_synced_with_logic_defaults() {
    let check2 = load_source("check2");
    let docs_display = load_source("docs_display");
    let logic = load_source("logic");
    let view = load_source("view");

    assert!(
        check2.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "checklist should record docs-app docs/examples/matrix sync gate as completed."
    );

    for required in [
        "<Playground title=\"Hello World (Default API)\" code_signal=hello_world_code>",
        "<Playground title=\"Variant Matrix\" code_signal=variants_code>",
        "<Playground\n                title=\"Interactive Playground\"",
        "data-slot=\"code-state-matrix\"",
        "data-slot=\"code-parameter-matrix\"",
        "data-slot=\"code-state-rows\"",
        "data-slot=\"code-parameter-rows\"",
        "variant: Option&lt;CodeVariant&gt;",
        "default = None -> normalize to inline",
        "class_name: Option&lt;String&gt;",
        "normalize_optional_text",
        "lang: Option&lt;String&gt;, dir: Option&lt;A11yDirection&gt;",
        "locale_attrs",
    ] {
        assert!(
            docs_display.contains(required),
            "docs-app code page should keep synced docs/example/matrix marker `{required}`"
        );
    }

    for required in [
        "variant: input.variant.unwrap_or_default(),",
        "let class_name = normalize_optional_text(input.class_name);",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should keep default/normalization behavior documented by docs-app via `{required}`"
        );
    }

    for required in [
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang",
        "dir=locale.dir",
    ] {
        assert!(
            view.contains(required),
            "view.rs should keep locale default behavior documented by docs-app via `{required}`"
        );
    }
}

#[test]
fn code_composition_api_contract_is_explicitly_na_without_parallel_array_inputs() {
    let view = load_source("view");
    let readme = load_source("readme");

    for required in ["pub fn Code(", "children: Children,"] {
        assert!(
            view.contains(required),
            "code should keep explicit leaf composition surface via `{required}`"
        );
    }

    // Code is a leaf display component; Parent/Item composition contract is N/A.
    // Guard against introducing parallel-array or index-pairing APIs.
    for forbidden in [
        "labels:",
        "titles:",
        "panels:",
        "items:",
        "item_specs:",
        "labels=",
        "titles=",
        "panels=",
        "items=",
        "item_specs=",
    ] {
        assert!(
            !view.contains(forbidden) && !readme.contains(forbidden),
            "code leaf api should not introduce parallel-array composition input: `{forbidden}`"
        );
    }
}

#[test]
fn code_macro_micro_duality_is_explicitly_na_without_dragging_feedback_loop() {
    let logic = load_source("logic");
    let view = load_source("view");
    let module = load_source("mod");
    let motion_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/motion.rs");

    // Code is a static display component; Dragging macro/micro dual-state contract is N/A.
    assert!(
        !motion_file.exists(),
        "Code should not have component-local motion loop for drag interactions."
    );

    for forbidden in [
        "Dragging",
        "Action::DragEnd",
        "DragEnd",
        "on:pointermove",
        "on:mousemove",
        "on:drag",
        "ondrag",
        "requestAnimationFrame",
        "request_animation_frame",
        "raf(",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "code should not introduce drag per-frame feedback loop contract: `{forbidden}`"
        );
    }

    for forbidden in ["mod motion;", "pub mod motion;", "pub use motion::"] {
        assert!(
            !module.contains(forbidden),
            "code module should keep drag/motion orchestration out of the leaf component: `{forbidden}`"
        );
    }
}

#[test]
fn code_two_pass_rendering_is_explicitly_na_without_dom_measure_rectification_loop() {
    let logic = load_source("logic");
    let view = load_source("view");
    let module = load_source("mod");

    // Code is a static display component; geometry two-pass rendering is N/A.
    for forbidden in [
        "getBoundingClientRect",
        "getClientRects",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "ResizeObserver",
        "IntersectionObserver",
        "NodeRef",
        "Intent",
        "Rectification",
        "Action::Measure",
        "Action::Rectify",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "code should not introduce geometry measure/rectification loop primitive: `{forbidden}`"
        );
    }

    for forbidden in ["mod motion;", "pub mod motion;", "pub use motion::"] {
        assert!(
            !module.contains(forbidden),
            "code module should not expose component-local geometry loop orchestration: `{forbidden}`"
        );
    }
}

#[test]
fn code_registration_protocol_is_explicitly_na_without_dynamic_item_registry() {
    let logic = load_source("logic");
    let view = load_source("view");
    let readme = load_source("readme");

    // Code is a leaf display component; dynamic item registration contract is N/A.
    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "Accordion",
        "Tabs",
        "Menu",
        "item_ids",
        "register_item",
        "unregister_item",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !readme.contains(forbidden),
            "code should not introduce dynamic collection registration protocol: `{forbidden}`"
        );
    }
}

#[test]
fn code_slot_projection_strategy_is_explicitly_na_for_leaf_display_component() {
    let logic = load_source("logic");
    let view = load_source("view");
    let readme = load_source("readme");

    // Code is a leaf display component; container slot projection strategy is N/A.
    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
        "slot strategy",
        "suspend_effects",
        "pause_effects",
        "resume_effects",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !readme.contains(forbidden),
            "code should not introduce container slot projection lifecycle contract: `{forbidden}`"
        );
    }
}

#[test]
fn code_env_streams_contract_is_explicitly_na_without_resize_theme_intersection_sampling() {
    let logic = load_source("logic");
    let view = load_source("view");
    let readme = load_source("readme");

    // Code is a static leaf component; Env Streams contract is N/A.
    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "match_media",
        "prefers-color-scheme",
        "BreakpointChanged",
        "Action::BreakpointChanged",
        "Action::ThemeChanged",
        "Action::IntersectionChanged",
        "debounce",
        "throttle",
        "on:resize",
        "add_event_listener",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !readme.contains(forbidden),
            "code should not introduce env stream sampling/flood-control contract: `{forbidden}`"
        );
    }
}

#[test]
fn code_event_light_cone_is_explicitly_na_without_collection_bus_and_selector_contract() {
    let logic = load_source("logic");
    let view = load_source("view");
    let readme = load_source("readme");

    // Code is a static leaf component; large-collection event light cone is N/A.
    for forbidden in [
        "Context Bus",
        "ContextBus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "Table",
        "Grid",
        "batch_select",
        "broadcast_selection",
        "prop drilling",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !readme.contains(forbidden),
            "code should not introduce collection-scale event cone contract: `{forbidden}`"
        );
    }
}

#[test]
fn code_causality_bus_is_explicitly_na_without_traceid_broadcast_chain() {
    let logic = load_source("logic");
    let view = load_source("view");
    let readme = load_source("readme");

    // Code is a static leaf component; complex causality bus chain is N/A.
    for forbidden in [
        "TraceId",
        "trace_id",
        "causality",
        "command_bus",
        "event_bus",
        "broadcast",
        "subscriber",
        "dispatch_command",
        "derived_command",
        "correlation_id",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !readme.contains(forbidden),
            "code should not introduce causality bus trace propagation contract: `{forbidden}`"
        );
    }
}

#[test]
fn code_focus_stack_contract_is_explicitly_na_without_overlay_recovery_flow() {
    let logic = load_source("logic");
    let view = load_source("view");
    let readme = load_source("readme");
    let module = load_source("mod");

    // Code is a static leaf display component; overlay focus stack/GC contract is N/A.
    for forbidden in [
        "NodeRef",
        "Overlay",
        "overlay",
        "FallbackTo",
        "Selector",
        "FocusManager",
        "focus_manager",
        "focus_stack",
        "restore_focus",
        "document.body",
        "activeElement",
        "on:focus",
        "on:blur",
    ] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !readme.contains(forbidden)
                && !module.contains(forbidden),
            "code should not introduce overlay focus recovery stack contract: `{forbidden}`"
        );
    }
}

#[test]
fn code_escape_hatches_contract_is_explicitly_na_without_foreign_zone_bridge() {
    let logic = load_source("logic");
    let view = load_source("view");
    let readme = load_source("readme");
    let module = load_source("mod");

    // Code is a static leaf display component; imperative third-party Foreign Zone bridge is N/A.
    for forbidden in [
        "Foreign Zone",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "yield_control",
        "cleanup_foreign",
        "foreign_zone",
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "GoogleMap",
        "AMap",
        "BMap",
        "chart_instance",
        "map_instance",
        "imperative_instance",
    ] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !readme.contains(forbidden)
                && !module.contains(forbidden),
            "code should not introduce imperative third-party escape hatch contract: `{forbidden}`"
        );
    }
}

#[test]
fn code_hydration_discontinuity_contract_is_explicitly_na_without_time_or_random_id_init() {
    let logic = load_source("logic");
    let view = load_source("view");
    let module = load_source("mod");
    let readme = load_source("readme");
    let ui_components_root = load_source("ui_components_root");

    // Code does not allocate runtime IDs; keep SSR/hydration discontinuity axis N/A.
    for forbidden in [
        "Date::now",
        "Date.now",
        "SystemTime::now",
        "Instant::now",
        "now(",
        "Uuid",
        "uuid",
        "rand::",
        "thread_rng",
        "random::<",
        "random_uuid",
        "use_id(",
        "id_seed",
    ] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !module.contains(forbidden)
                && !readme.contains(forbidden),
            "code should not introduce nondeterministic init that can split SSR/hydration contract: `{forbidden}`"
        );
    }

    assert!(
        ui_components_root.contains("provide_ui_id_provider(id_seed);"),
        "id determinism entrypoint should stay in UiRoot, not in leaf component local init."
    );
}

#[test]
fn code_ssr_cross_platform_contract_uses_explicit_cfg_and_keeps_non_wasm_clean() {
    let manifest = load_source("manifest");
    let logic = load_source("logic");
    let view = load_source("view");
    let module = load_source("mod");
    let ui_headless_manifest = load_source("ui_headless_manifest");
    let ui_headless_lib = load_source("ui_headless_lib");
    let ui_components_manifest = load_source("ui_components_manifest");
    let check2 = load_source("check2");

    for required in [
        "cargo check -p ui-code --quiet",
        "cargo check -p ui-headless --no-default-features --features ssr --quiet",
        "cargo check -p ui-code --target wasm32-unknown-unknown --quiet",
    ] {
        assert!(
            check2.contains(required),
            "checklist should preserve explicit compile-only evidence command `{required}`"
        );
    }

    assert!(
        manifest.contains(
            "leptos = { version = \"0.8.15\", default-features = false, features = [\"csr\"] }"
        ),
        "ui-code should keep explicit platform behavior under feature cfg, not implicit runtime branching."
    );
    assert!(
        ui_headless_manifest.contains("default = [\"web\"]")
            && ui_headless_manifest.contains("web = [\"leptos/csr\"]")
            && ui_headless_manifest.contains("ssr = [\"leptos/ssr\"]"),
        "ui-headless should keep web/ssr split under explicit feature management."
    );
    assert!(
        ui_headless_lib.contains("compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"),
        "ui-headless should guard invalid web+ssr co-enable path via compile_error."
    );
    assert!(
        ui_components_manifest.contains("[target.'cfg(target_arch = \"wasm32\")'.dependencies]")
            && ui_components_manifest.contains("web-sys = { version = \"0.3.85\""),
        "wasm-only browser dependency should stay behind explicit target cfg."
    );

    for forbidden in [
        "web_sys::",
        "web-sys",
        "window.",
        "document.",
        "HtmlElement",
        "NodeRef",
        "js_sys::",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !module.contains(forbidden),
            "non-wasm code path should not depend on browser object `{forbidden}`"
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(feature = \"ssr\")]",
        "cfg!(target_arch = \"wasm32\")",
        "cfg!(feature = \"ssr\")",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "code semantic contract should not split by runtime accidental target branch: `{forbidden}`"
        );
    }
}

#[test]
fn code_headless_web_ssr_mutex_is_guarded_by_compile_error_contract() {
    let manifest = load_source("manifest");
    let view = load_source("view");
    let ui_headless_manifest = load_source("ui_headless_manifest");
    let ui_headless_lib = load_source("ui_headless_lib");
    let check2 = load_source("check2");

    for required in [
        "cargo check -p ui-headless --quiet",
        "cargo check -p ui-headless --no-default-features --features ssr --quiet",
        "cargo check -p ui-headless --no-default-features --features web,ssr --quiet",
    ] {
        assert!(
            check2.contains(required),
            "checklist should preserve ui-headless web/ssr verification command `{required}`"
        );
    }

    assert!(
        manifest.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "code component should consume shared ui-headless contract from workspace crate."
    );
    assert!(
        view.contains("use ui_headless::a11y::{A11yDirection, locale_attrs};"),
        "code view should mount headless a11y contract instead of bypassing ui-headless."
    );

    assert!(
        ui_headless_manifest.contains("default = [\"web\"]")
            && ui_headless_manifest.contains("web = [\"leptos/csr\"]")
            && ui_headless_manifest.contains("ssr = [\"leptos/ssr\"]"),
        "ui-headless should keep web/ssr split behind explicit feature declarations."
    );

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_lib.contains(required),
            "ui-headless should guard web+ssr co-enable regression with `{required}`"
        );
    }
}

#[test]
fn code_motion_non_wasm_stub_keeps_ssr_tooling_compile_path_predictable() {
    let logic = load_source("logic");
    let view = load_source("view");
    let module = load_source("mod");
    let ui_motion_manifest = load_source("ui_motion_manifest");
    let ui_motion_lib = load_source("ui_motion_lib");
    let check2 = load_source("check2");

    for required in [
        "cargo check -p ui-motion --quiet",
        "cargo check -p ui-motion --target wasm32-unknown-unknown --quiet",
        "cargo test -p ui-motion non_wasm_web_backend_is_predictable_noop --quiet",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep explicit ui-motion verification command `{required}`"
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "pub mod web;",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should keep explicit non-wasm noop/stub branch with `{required}`"
        );
    }

    assert!(
        ui_motion_manifest.contains("[target.'cfg(target_arch = \"wasm32\")'.dependencies]")
            && ui_motion_manifest.contains("web-sys = { version = \"0.3.85\""),
        "ui-motion browser dependencies should stay wasm-target gated."
    );

    // Code is a static leaf component and must not assume motion handles exist.
    for forbidden in [
        "ui_motion::",
        "attach_motion(",
        "AnimationHandle",
        "animation_handle",
        "unwrap()",
        ".expect(",
        "panic!(",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !module.contains(forbidden),
            "code should not depend on component-local motion handle semantics: `{forbidden}`"
        );
    }
}

#[test]
fn code_reduced_motion_ssr_wasm_contract_stays_semantically_single_track() {
    let logic = load_source("logic");
    let view = load_source("view");
    let module = load_source("mod");
    let ui_motion_lib = load_source("ui_motion_lib");
    let check2 = load_source("check2");

    for required in [
        "cargo check -p ui-code --quiet",
        "cargo check -p ui-code --target wasm32-unknown-unknown --quiet",
        "cargo test -p ui-motion non_wasm_web_backend_is_predictable_noop --quiet",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep reduced-motion/SSR/wasm verification command `{required}`"
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should provide deterministic reduced-motion + non-wasm noop fallback via `{required}`"
        );
    }

    // Static leaf component keeps one semantic contract across SSR/wasm; no branch split in component layer.
    for forbidden in [
        "ui_motion::",
        "attach_motion(",
        "prefers_reduced_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(feature = \"ssr\")]",
        "cfg!(target_arch = \"wasm32\")",
        "cfg!(feature = \"ssr\")",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !module.contains(forbidden),
            "code component should not diverge semantics across reduced-motion/SSR/wasm: `{forbidden}`"
        );
    }

    for required in [
        "data-variant=state.variant_attr",
        "data-state=state.state_attr",
        "lang=locale.lang",
        "dir=locale.dir",
    ] {
        assert!(
            view.contains(required),
            "code view should keep stable semantic output across runtime branches via `{required}`"
        );
    }
}

#[test]
fn code_a11y_i18n_contract_is_mounted_without_hardcoded_user_visible_copy() {
    let view = load_source("view");
    let readme = load_source("readme");

    for required in [
        "use ui_headless::a11y::{A11yDirection, locale_attrs};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang",
        "dir=locale.dir",
        "{children()}",
    ] {
        assert!(
            view.contains(required),
            "code should mount shared a11y/i18n contract and render consumer-provided copy via `{required}`"
        );
    }

    assert!(
        readme.contains("`lang` / `dir`（由 `ui_headless::a11y::locale_attrs` 归一）"),
        "code docs should expose lang/dir localization entrypoint."
    );

    // Code is non-interactive text display; interactive keyboard/role mapping axis is N/A.
    for forbidden in [
        "on:click",
        "on:keydown",
        "on:keyup",
        "on:pointerdown",
        "on:pointerup",
        "on:mouseenter",
    ] {
        assert!(
            !view.contains(forbidden),
            "code should not introduce interactive handler path in leaf display component: `{forbidden}`"
        );
    }
}

#[test]
fn code_state_observability_uses_stable_enumerable_data_markers() {
    let logic = load_source("logic");
    let view = load_source("view");
    let readme = load_source("readme");

    for required in [
        "data-slot=\"code\"",
        "data-variant=state.variant_attr",
        "data-state=state.state_attr",
        "data-inline=state.is_inline.then_some(\"true\")",
        "data-block=state.is_block.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "let state = resolve_state(CodeStateInput {",
        "variant: input.variant.unwrap_or_default()",
    ] {
        assert!(
            view.contains(required) || logic.contains(required),
            "code should expose stable observable state markers via `{required}`"
        );
    }

    assert!(
        readme.contains("`data-variant`（`inline` / `block`）")
            && readme.contains("`data-state`（`inline` / `block`）"),
        "docs should describe closed-set data marker values for reliable selectors."
    );

    // Non-interactive leaf component: dynamic aria state axes are N/A.
    for forbidden in [
        "aria-expanded",
        "aria-selected",
        "aria-disabled",
        "aria-busy",
        "data-state=class_name",
    ] {
        assert!(
            !view.contains(forbidden),
            "code should avoid drifting into free-form/dynamic aria state contract: `{forbidden}`"
        );
    }
}

#[test]
fn code_performance_governance_uses_repeatable_equivalent_baseline_for_static_leaf() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "cargo test -p ui-code --quiet",
        "cargo check -p ui-code --quiet",
        "cargo check -p ui-code --target wasm32-unknown-unknown --quiet",
        "render_count",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep explicit performance verification/follow-up token `{required}`"
        );
    }

    let resolve_state_count = logic.matches("resolve_state(CodeStateInput {").count();
    assert_eq!(
        resolve_state_count, 1,
        "static leaf should keep single normalized state-derivation path; found {resolve_state_count} resolve_state invocations."
    );

    for forbidden in [
        "create_signal(",
        "RwSignal",
        "Memo::new(",
        "Signal::derive(",
        "requestAnimationFrame",
        "request_animation_frame",
        "attach_motion(",
        "on:click",
        "on:keydown",
        "on:pointerdown",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "code performance baseline should avoid extra reactive/event/motion hot paths: `{forbidden}`"
        );
    }

    for required in [
        "data-variant=state.variant_attr",
        "data-state=state.state_attr",
        "data-inline=state.is_inline.then_some(\"true\")",
        "data-block=state.is_block.then_some(\"true\")",
    ] {
        assert!(
            view.contains(required),
            "performance-attribution contract should keep stable semantic markers via `{required}`"
        );
    }
}

#[test]
fn code_semantic_and_performance_regression_contract_covers_aria_data_and_focus_flow_scope() {
    let check2 = load_source("check2");
    let semantics_suite = include_str!("semantics.rs");
    let view = load_source("view");
    let logic = load_source("logic");

    for required in [
        "语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照",
        "render_count",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep semantic+performance regression baseline via `{required}`"
        );
    }

    for required in [
        "fn code_state_observability_uses_stable_enumerable_data_markers()",
        "fn code_a11y_i18n_contract_is_mounted_without_hardcoded_user_visible_copy()",
        "fn code_performance_governance_uses_repeatable_equivalent_baseline_for_static_leaf()",
        "fn code_semantics_contract_tests_prioritize_semantics_over_visual_snapshots()",
    ] {
        assert!(
            semantics_suite.contains(required),
            "semantics suite should keep this regression branch via `{required}`"
        );
    }

    for required in [
        "data-slot=\"code\"",
        "data-variant=state.variant_attr",
        "data-state=state.state_attr",
        "aria-live=\"off\"",
        "aria-busy=\"false\"",
    ] {
        assert!(
            view.contains(required),
            "view should keep stable aria/data semantics for machine-readable regression checks: `{required}`"
        );
    }

    // Leaf display component has no interactive focus-flow state machine; keep this axis explicitly N/A.
    for forbidden in [
        "on:focus",
        "on:blur",
        "on:focusin",
        "on:focusout",
        "on:keydown",
        "on:pointerdown",
        "tabindex=",
        "aria-activedescendant",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "focus-flow path should remain N/A for static leaf without interactive handlers: `{forbidden}`"
        );
    }
}

#[test]
fn code_view_macro_complexity_is_controlled_for_leaf_component() {
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "cargo check -p ui-code --quiet",
        "cargo check -p ui-code --target wasm32-unknown-unknown --quiet",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep view-macro governance verification command `{required}`"
        );
    }

    let view_macro_count = view.matches("view! {").count();
    assert_eq!(
        view_macro_count, 1,
        "Code view should keep a single small view! block; found {view_macro_count} blocks."
    );

    let non_empty_line_count = view.lines().filter(|line| !line.trim().is_empty()).count();
    assert!(
        non_empty_line_count <= 60,
        "Code view should stay compact for macro-expansion safety; found {non_empty_line_count} non-empty lines."
    );

    // Leaf component should not grow into repeated deep template branches.
    for forbidden in [
        "for item in",
        ".map(|",
        "collect::<Vec",
        "<For",
        "render_header(",
        "render_body(",
        "render_item(",
    ] {
        assert!(
            !view.contains(forbidden),
            "Code view macro complexity regressed with repeated/deep template marker `{forbidden}`"
        );
    }
}

#[test]
fn code_functional_split_prefers_plain_functions_over_extra_components() {
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "cargo check -p ui-code --quiet",
        "cargo test -p ui-code --quiet",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep functional-split verification command `{required}`"
        );
    }

    let component_attr_count = view.matches("#[component]").count();
    assert_eq!(
        component_attr_count, 1,
        "Code should expose only one public component entry for this leaf; found {component_attr_count} #[component] markers."
    );
    assert!(
        view.contains("pub fn Code("),
        "Code leaf entrypoint should remain explicit as `pub fn Code`."
    );

    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\nfn section_",
        "#[component]\nfn item_",
    ] {
        assert!(
            !view.contains(forbidden),
            "lightweight local UI fragments should not be promoted to nested components: `{forbidden}`"
        );
    }

    // If future refactoring introduces local fragment helpers, they should remain plain functions.
    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\nfn section_",
        "#[component]\nfn item_",
    ] {
        assert!(
            !view.contains(forbidden),
            "local helper should remain plain function and must not be a component: `{forbidden}`"
        );
    }
}

#[test]
fn code_static_fragment_constantization_is_explicitly_na_for_leaf_component() {
    let view = load_source("view");
    let styles = load_source("styles");
    let readme = load_source("readme");
    let check2 = load_source("check2");

    for required in [
        "cargo check -p ui-code --quiet",
        "cargo test -p ui-code --quiet",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep static-fragment governance verification command `{required}`"
        );
    }

    // Code is a leaf text presenter: no complex static svg/footer/long copy fragment to constantize in view.
    for forbidden in [
        "<svg",
        "<footer",
        "inner_html=",
        "inner_html:",
        "dangerously_set_inner_html",
        "lorem ipsum",
        "版权所有",
    ] {
        assert!(
            !view.contains(forbidden),
            "code view should not introduce heavyweight static fragment in render path: `{forbidden}`"
        );
    }

    assert!(
        view.contains("{children()}"),
        "code view should consume caller-provided text payload rather than embedding long static copy."
    );
    assert!(
        styles.contains("pub const CSS: &str ="),
        "static style fragment path should stay centralized in styles.rs const CSS."
    );
    assert!(
        readme.contains("source: `components/code/src/{mod,logic,view,styles}.rs`"),
        "static resource modification path should remain explicit and traceable in README."
    );
}

#[test]
fn code_inner_html_contract_is_explicitly_na_and_blocks_untrusted_html_paths() {
    let logic = load_source("logic");
    let view = load_source("view");
    let readme = load_source("readme");
    let check2 = load_source("check2");

    for required in [
        "cargo check -p ui-code --quiet",
        "cargo test -p ui-code --quiet",
        "inner_html",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep inner_html governance marker `{required}`"
        );
    }

    // Code has no trusted static inner_html use-case; keep this axis explicitly N/A with hard guardrails.
    for forbidden in [
        "inner_html=",
        "inner_html:",
        "dangerously_set_inner_html",
        ".set_inner_html(",
        "set_inner_html(",
        "HtmlElement::set_inner_html",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "code component should not introduce inner_html surface: `{forbidden}`"
        );
    }

    // Guard against direct/indirect unsanitized HTML construction pipelines.
    for forbidden in [
        "format!(\"<",
        "push_str(\"<",
        "String::from(\"<",
        "serde_json::from_str",
        "reqwest::",
        "fetch(",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "code component should not construct or ingest untrusted html payloads: `{forbidden}`"
        );
    }

    assert!(
        view.contains("{children()}") && !readme.contains("inner_html"),
        "code should keep explicit child text rendering path and avoid documenting inner_html usage."
    );
}

#[test]
fn code_wasm_debug_contract_is_explicitly_na_and_feature_isolated() {
    let manifest = load_source("manifest");
    let logic = load_source("logic");
    let view = load_source("view");
    let module = load_source("mod");
    let readme = load_source("readme");
    let ui_components_manifest = load_source("ui_components_manifest");
    let check2 = load_source("check2");

    for required in [
        "cargo check -p ui-code --target wasm32-unknown-unknown --quiet",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-code,inject-css",
        "cargo tree -e features -p ui-components --no-default-features --features component-code,inject-css",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep explicit wasm-debug governance verification command `{required}`"
        );
    }

    assert!(
        manifest.contains("[features]\ndefault = []"),
        "ui-code should keep debug capability out of default/public feature surface."
    );
    for forbidden in [
        "wasm-debug",
        "wasm_debug",
        "debug-overlay",
        "trace-overlay",
        "replay",
        "timeline",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "ui-code manifest should not expose wasm debug/replay feature token `{forbidden}`"
        );
    }

    for forbidden in [
        "code-wasm-debug",
        "component-code-wasm-debug",
        "component-code\", \"dep:tracing",
    ] {
        assert!(
            !ui_components_manifest.contains(forbidden),
            "ui-components feature graph should not introduce code-specific wasm debug public toggle `{forbidden}`"
        );
    }

    for forbidden in [
        "UiTrace",
        "use_ui_trace",
        "provide_ui_trace",
        "trace.emit(",
        "TraceId",
        "debug_overlay",
        "wasm_debug_proxy",
        "replay",
        "timeline",
        "event_log",
    ] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !module.contains(forbidden)
                && !readme.contains(forbidden),
            "code component should keep wasm-debug capability out of runtime/public contract: `{forbidden}`"
        );
    }
}

#[test]
fn code_engineering_infra_contract_keeps_spec_structured_and_runtime_agnostic() {
    let manifest = load_source("manifest");
    let protocol = load_source("protocol");
    let logic = load_source("logic");
    let view = load_source("view");
    let module = load_source("mod");
    let readme = load_source("readme");
    let check2 = load_source("check2");

    for required in [
        "cargo check -p ui-code --quiet",
        "cargo test -p ui-code --quiet",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep engineering-infra governance verification command `{required}`"
        );
    }

    for required in [
        "use serde::{Deserialize, Serialize};",
        "#[serde(rename_all = \"snake_case\")]",
        "pub enum CodeComponentSchemaVersion",
        "pub struct CodeComponentSpec",
        "#[serde(default)]",
        "pub schema_version: CodeComponentSchemaVersion,",
    ] {
        assert!(
            protocol.contains(required),
            "code protocol should keep structured serde spec contract via `{required}`"
        );
    }

    assert!(
        manifest.contains("serde = { version = \"1.0\", features = [\"derive\"] }"),
        "ui-code manifest should keep serde as the structured spec serialization contract."
    );

    for forbidden in [
        "tracing",
        "tokio",
        "async-std",
        "async_std",
        "Runtime",
        "JoinHandle",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "ui-code manifest should not bind component public surface to runtime/tracing detail: `{forbidden}`"
        );
    }

    for forbidden in [
        "tracing::",
        "span!(",
        "event!(",
        "#[instrument]",
        "tokio::",
        "async_std::",
        "Runtime",
        "JoinHandle",
    ] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !module.contains(forbidden)
                && !readme.contains(forbidden),
            "code component should keep runtime/tracing details out of component-level API and flow: `{forbidden}`"
        );
    }
}

#[test]
fn code_version_deprecation_migration_contract_registers_window_and_pure_upgrade_function() {
    let check2 = load_source("check2");
    let protocol = load_source("protocol");
    let component_toml = load_source("component_toml");

    assert!(
        check2.contains("- [x] 版本弃用迁移（Codemod/Registry）"),
        "checklist should record version-deprecation migration contract as completed."
    );

    for required in [
        "pub enum CodeComponentSchemaVersion",
        "V1,",
        "V2,",
        "pub struct CodeComponentSpecV2",
        "pub fn migrate_v1_to_v2(v1: CodeComponentSpec) -> CodeComponentSpecV2 {",
        "render_mode: CodeRenderMode::Snapshot",
        "pub const CODE_SCHEMA_REGISTRY: [CodeSchemaRegistryEntry; 2] = [",
        "CodeSchemaStatus::Deprecated",
        "starts_on: \"2026-02-20\"",
        "ends_on: \"2026-08-31\"",
        "successor: Some(CodeComponentSchemaVersion::V2)",
        "migration: Some(migrate_v1_to_v2)",
    ] {
        assert!(
            protocol.contains(required),
            "protocol should keep breaking-upgrade migration registry contract via `{required}`"
        );
    }

    for required in [
        "[[schema_registry]]",
        "schema = \"code.v1\"",
        "status = \"deprecated\"",
        "successor = \"code.v2\"",
        "migration = \"migrate_v1_to_v2\"",
        "deprecation_start = \"2026-02-20\"",
        "deprecation_end = \"2026-08-31\"",
        "schema = \"code.v2\"",
        "status = \"active\"",
    ] {
        assert!(
            component_toml.contains(required),
            "component manifest should publish schema-registry metadata via `{required}`"
        );
    }

    for forbidden in [
        "SystemTime::now",
        "std::time::SystemTime",
        "rand::",
        "thread_rng",
        "Uuid::new_v4",
    ] {
        assert!(
            !protocol.contains(forbidden),
            "migration layer should stay pure and deterministic, found forbidden token `{forbidden}`"
        );
    }
}

#[test]
fn code_styles_depend_on_explicit_state_markers_not_dom_structure_guessing() {
    let styles = load_source("styles");
    let view = load_source("view");

    for required in [
        ".ui-code--variant-inline,",
        ".ui-code[data-variant=\"inline\"]",
        ".ui-code--variant-block,",
        ".ui-code[data-variant=\"block\"]",
        ".ui-code--state-inline,",
        ".ui-code[data-state=\"inline\"]",
        ".ui-code[data-inline=\"true\"]",
        ".ui-code--state-block,",
        ".ui-code[data-state=\"block\"]",
        ".ui-code[data-block=\"true\"]",
        ".ui-code--custom-class,",
        ".ui-code[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles.contains(required),
            "code styles should branch only on stable semantic marker selector `{required}`"
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":first-child", ":last-child"] {
        assert!(
            !styles.contains(forbidden),
            "code styles should not infer state from DOM structure: `{forbidden}`"
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view.contains(forbidden),
            "code view should not carry business style logic via inline style: `{forbidden}`"
        );
    }
}

#[test]
fn code_defensive_variables_use_ui_theme_fallback_chain_without_raw_terminal_values() {
    let styles = load_source("styles");
    let check2 = load_source("check2");

    assert!(
        check2.contains("样式孤岛防御（Defensive Variables）"),
        "checklist should track defensive variables gate explicitly."
    );

    for required in [
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "--ui-space-3xs,",
        "var(--ui-fallback-space-3xs, var(--ui-fallback-space-2xs))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
    ] {
        assert!(
            styles.contains(required),
            "defensive variable chain should be enforced via `{required}`"
        );
    }

    for forbidden in [
        ", 12px)", ", 16px)", ", 20px)", ", 6px)", ", 2px)", "#", "rgb(", "hsl(",
    ] {
        assert!(
            !styles.to_ascii_lowercase().contains(forbidden),
            "component styles should avoid hardcoded terminal values and color literals: `{forbidden}`"
        );
    }
}

#[test]
fn code_cascade_layer_coverage_is_routed_via_ui_layer_and_blocks_plain_inline_style() {
    let ui_components_css = load_source("ui_components_css");
    let view = load_source("view");
    let check2 = load_source("check2");

    assert!(
        check2.contains("级联层覆盖（`@layer ui`）"),
        "checklist should keep cascade-layer governance marker."
    );

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-code\")]",
        "out.push_str(crate::code::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            ui_components_css.contains(required),
            "component css aggregation should keep `{required}` in @layer ui pipeline."
        );
    }

    for forbidden in ["style=", "style:", "style=\"top:", "style:top"] {
        assert!(
            !view.contains(forbidden),
            "code view should not introduce plain inline style path: `{forbidden}`"
        );
    }
}

#[test]
fn code_motion_contractualization_is_explicitly_na_and_keeps_shared_noop_motion_path() {
    let check2 = load_source("check2");
    let logic = load_source("logic");
    let view = load_source("view");
    let module = load_source("mod");
    let ui_motion_lib = load_source("ui_motion_lib");
    let motion_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/motion.rs");

    assert!(
        check2.contains("Motion 合同化"),
        "checklist should track motion-contract gate explicitly."
    );

    assert!(
        !motion_file.exists(),
        "Code is a static leaf component; motion.rs contract mapping is N/A and should stay absent."
    );

    for forbidden in [
        "attach_motion(",
        "ui_motion::",
        "stiffness",
        "damping",
        "MotionContract",
        "AnimationHandle",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !module.contains(forbidden),
            "code component should not host component-local motion contract detail: `{forbidden}`"
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "shared ui-motion layer should keep reduced-motion + non-wasm noop path via `{required}`"
        );
    }
}

#[test]
fn code_ui_components_fixed_entry_files_follow_layering_contract() {
    let check2 = load_source("check2");
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");
    let ui_components_root = load_source("ui_components_root");
    let ui_headless_lib = load_source("ui_headless_lib");
    let ui_components_src =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui-components/src");
    let active_highlight_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let active_highlight = std::fs::read_to_string(&active_highlight_path)
        .expect("should read ui-visual-primitive active_highlight.rs");

    assert!(
        check2.contains("`ui-components` 固定入口文件落点正确"),
        "checklist should keep ui-components fixed-entry gate explicitly."
    );

    for required in [
        "mod css;",
        "#[cfg(feature = \"component-code\")]",
        "pub use ui_code as code;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui-components lib.rs should keep feature-gated public export surface via `{required}`"
        );
    }

    for forbidden in [
        "pub use web_sys::",
        "pub use leptos::web_sys::",
        "pub use wasm_bindgen::",
    ] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui-components lib.rs should not expose platform internals through public API: `{forbidden}`"
        );
    }

    for required in [
        "pub fn push_components_css(out: &mut String)",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-code\")]",
        "out.push_str(crate::code::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui-components css.rs should aggregate component css with feature-gated @layer ui path via `{required}`"
        );
    }

    for required in [
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root.contains(required),
            "ui-components root.rs should keep UiRoot theme/css/i18n centralization via `{required}`"
        );
    }

    assert!(
        active_highlight_path.exists(),
        "shared active_highlight entry should live in crates/ui-visual-primitive/src/active_highlight.rs"
    );
    for required in [
        "pub struct ActiveHighlightMotion",
        "pub spring: ui_motion::spring::SpringConfig,",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight should stay a shared visual-primitive motion capability via `{required}`"
        );
    }

    for forbidden_file in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src.join(forbidden_file).exists(),
            "ui-components/src should not host forbidden shared primitive file `{forbidden_file}`"
        );
    }

    for required in [
        "pub mod controllable_state;",
        "pub use presence::{Presence, use_presence};",
        "aria_controls_when_open",
    ] {
        assert!(
            ui_headless_lib.contains(required),
            "shared interactive/a11y primitives should stay in ui-headless via `{required}`"
        );
    }
}

#[test]
fn code_semantics_contract_tests_prioritize_semantics_over_visual_snapshots() {
    let check2 = load_source("check2");
    let semantics_suite = include_str!("semantics.rs");
    let logic_suite = include_str!("logic.rs");
    let view = load_source("view");
    let logic = load_source("logic");

    for required in [
        "语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep semantic-first gate explicit via `{required}`"
        );
    }

    for required in [
        "fn code_state_observability_uses_stable_enumerable_data_markers()",
        "fn code_a11y_i18n_contract_is_mounted_without_hardcoded_user_visible_copy()",
        "fn code_controllable_contract_is_explicitly_na_without_half_controlled_api()",
        "fn code_async_interaction_contract_is_explicitly_na_for_static_component()",
        "fn code_ssr_cross_platform_contract_uses_explicit_cfg_and_keeps_non_wasm_clean()",
        "fn code_headless_web_ssr_mutex_is_guarded_by_compile_error_contract()",
        "fn code_motion_non_wasm_stub_keeps_ssr_tooling_compile_path_predictable()",
        "fn code_reduced_motion_ssr_wasm_contract_stays_semantically_single_track()",
        "fn code_dx_workbench_flow_supports_fast_style_iteration_and_context_retention()",
        "fn code_documentation_as_product_readme_is_beginner_friendly_with_default_first_path()",
        "fn code_docs_product_copy_paste_ready_playground_covers_hello_matrix_control_streaming_and_source_first()",
        "fn code_source_first_docs_are_copy_paste_ready_with_imports_prerequisites_and_synced_example()",
        "fn code_heroui_benchmark_docs_and_component_docs_remain_synced()",
        "fn code_docs_app_interactive_playground_supports_live_prop_state_preview_and_repeatable_flow()",
        "fn code_docs_examples_and_parameter_state_matrix_stay_synced_with_logic_defaults()",
        "fn code_e2e_selector_contract_uses_semantic_markers_and_wasm_stable_ready_waits()",
        "fn code_e2e_critical_flow_is_repeatable_and_locates_semantic_breakpoints()",
        "fn code_performance_governance_uses_repeatable_equivalent_baseline_for_static_leaf()",
        "fn code_semantic_and_performance_regression_contract_covers_aria_data_and_focus_flow_scope()",
        "fn code_view_macro_complexity_is_controlled_for_leaf_component()",
        "fn code_functional_split_prefers_plain_functions_over_extra_components()",
        "fn code_static_fragment_constantization_is_explicitly_na_for_leaf_component()",
        "fn code_inner_html_contract_is_explicitly_na_and_blocks_untrusted_html_paths()",
        "fn code_wasm_debug_contract_is_explicitly_na_and_feature_isolated()",
        "fn code_engineering_infra_contract_keeps_spec_structured_and_runtime_agnostic()",
        "fn code_tree_shaking_contract_is_feature_gated_end_to_end()",
        "fn code_defensive_variables_use_ui_theme_fallback_chain_without_raw_terminal_values()",
        "fn code_cascade_layer_coverage_is_routed_via_ui_layer_and_blocks_plain_inline_style()",
        "fn code_motion_contractualization_is_explicitly_na_and_keeps_shared_noop_motion_path()",
        "fn code_ui_components_fixed_entry_files_follow_layering_contract()",
        "fn code_version_deprecation_migration_contract_registers_window_and_pure_upgrade_function()",
        "fn code_component_directory_standard_file_layout_is_enforced()",
        "fn code_file_placement_discipline_contract_is_explicit_for_leaf_component_scope()",
        "fn code_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component()",
        "fn code_context_compression_manifest_and_rbi_projection_are_present_and_aligned()",
        "fn code_agent_contract_schema_is_typed_and_prevents_dom_guess_or_script_injection()",
        "fn code_llm_render_modes_are_limited_to_streaming_and_snapshot_display_semantics()",
        "fn code_snapshot_is_foundational_and_accepts_complete_result_and_config()",
        "fn code_streaming_policy_is_optional_with_snapshot_fallback_and_explicit_output_state_markers()",
        "fn code_rust_hygiene_contract_bans_unwrap_expect_and_let_underscore_and_uses_cow_for_class_parts()",
    ] {
        assert!(
            semantics_suite.contains(required),
            "semantic test matrix should include contract branch `{required}`"
        );
    }

    for required in [
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "lang=locale.lang",
        "dir=locale.dir",
        "on:keydown",
        "on:pointerdown",
        "disabled:",
        "aria-disabled",
        "value:",
        "default_value:",
        "on_value_change",
    ] {
        assert!(
            semantics_suite.contains(required),
            "semantic matrix should explicitly cover this axis (or N/A guard): `{required}`"
        );
    }

    assert!(
        semantics_suite.contains("for forbidden in [\"web_sys::\", \"web-sys\", \"JsValue\", \"HtmlElement\", \"NodeRef\"]"),
        "semantic tests should guard SSR/wasm applicability by enforcing no platform-specific contract split."
    );

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(feature = \"ssr\")]",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "code component should keep single semantic contract without per-target branch divergence: `{forbidden}`"
        );
    }

    for forbidden in [
        "insta::",
        "assert_snapshot!",
        "assert_debug_snapshot!",
        "assert_json_snapshot!",
    ] {
        assert!(
            !logic_suite.contains(forbidden)
                && !view.contains(forbidden)
                && !logic.contains(forbidden),
            "snapshot-only assertions must not replace semantic contract tests: `{forbidden}`"
        );
    }
}
