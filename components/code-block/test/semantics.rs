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
fn code_block_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/code-block/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "CodeBlock internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn code_block_does_not_introduce_component_spec_module() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("../../components/code-block/src/spec.rs");
    let mod_source = load_source("../../components/code-block/src/mod.rs");
    let protocol_source = load_source("../../components/code-block/src/protocol.rs");

    assert!(
        !spec_path.exists(),
        "CodeBlock is a simple component and should not introduce `spec.rs`."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "CodeBlock mod.rs should not expose spec module token `{forbidden}`."
        );
    }

    for needle in [
        "pub enum CodeBlockComponentSchemaVersion",
        "V1",
        "pub struct CodeBlockComponentSpec",
        "#[path = \"../test/protocol.rs\"]",
    ] {
        assert!(
            protocol_source.contains(needle),
            "CodeBlock protocol contract should keep versioned schema marker `{needle}`."
        );
    }
}

#[test]
fn code_block_file_responsibilities_stay_layered() {
    let mod_source = load_source("../../components/code-block/src/mod.rs");
    let logic_source = load_source("../../components/code-block/src/logic.rs");
    let styles_source = load_source("../../components/code-block/src/styles.rs");
    let view_source = load_source("../../components/code-block/src/view.rs");
    let motion_source = load_source("../../components/code-block/src/motion.rs");

    for marker in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::CodeBlockMotion;",
        "pub use view::CodeBlock;",
    ] {
        assert!(
            mod_source.contains(marker),
            "code_block/mod.rs should keep export boundary marker `{marker}`."
        );
    }
    for forbidden in ["pub mod logic", "pub mod view", "resolve_view_state("] {
        assert!(
            !mod_source.contains(forbidden),
            "code_block/mod.rs should avoid implementation detail export `{forbidden}`."
        );
    }

    for marker in [
        "pub fn resolve_copyable_contract(",
        "pub fn resolve_copied_contract(",
        "pub fn resolve_render_model(",
    ] {
        assert!(
            logic_source.contains(marker),
            "code_block/logic.rs should keep normalization or derivation contract `{marker}`."
        );
    }
    for forbidden in ["view! {", "NodeRef<", "web_sys", "set_property("] {
        assert!(
            !logic_source.contains(forbidden),
            "code_block/logic.rs should avoid view or DOM concern `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("var(--ui-"),
        "code_block/styles.rs should be token-first and consume `var(--ui-*)`."
    );
    for forbidden in ["Copy to clipboard", "Copied", "view! {", "on:click"] {
        assert!(
            !styles_source.contains(forbidden),
            "code_block/styles.rs should avoid interaction/view/text concern `{forbidden}`."
        );
    }

    for marker in [
        "#[component]",
        "logic::resolve_render_model(",
        "on_press=on_copy_press",
    ] {
        assert!(
            view_source.contains(marker),
            "code_block/view.rs should render structure and mount headless contract `{marker}`."
        );
    }
    for forbidden in [
        "logic::resolve_state_from_content(",
        "logic::normalize_optional_text(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "code_block/view.rs should avoid hidden normalization decision `{forbidden}`."
        );
    }

    for marker in [
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator",
    ] {
        assert!(
            motion_source.contains(marker),
            "code_block/motion.rs should map semantic motion contracts via `{marker}`."
        );
    }
    for forbidden in ["view! {", "on:click"] {
        assert!(
            !motion_source.contains(forbidden),
            "code_block/motion.rs should avoid view interaction concern `{forbidden}`."
        );
    }
}

#[test]
fn code_block_component_directory_layout_matches_standard_contract() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_src_dir = manifest_dir.join("../../components/code-block/src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            component_src_dir.join(required).exists(),
            "CodeBlock component directory should contain required file `{required}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !component_src_dir.join(forbidden).exists(),
            "CodeBlock component directory should not contain forbidden file `{forbidden}`."
        );
    }

    let mod_source = load_source("../../components/code-block/src/mod.rs");
    for marker in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::CodeBlockMotion;",
        "pub use view::CodeBlock;",
    ] {
        assert!(
            mod_source.contains(marker),
            "CodeBlock mod.rs should keep minimal stable export marker `{marker}`."
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "pub mod snippet",
        "pub mod button",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "CodeBlock mod.rs should avoid over-export marker `{forbidden}`."
        );
    }
}

#[test]
fn code_block_manifest_and_rbi_contracts_are_present_and_aligned() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_src_dir = manifest_dir.join("../../components/code-block/src");
    let manifest_path = component_src_dir.join("Component.toml");
    let rbi_path = component_src_dir.join("code_block.rbi");

    assert!(
        manifest_path.exists(),
        "CodeBlock should provide Component.toml for context compression."
    );
    assert!(
        rbi_path.exists(),
        "CodeBlock should provide code_block.rbi for API signature projection."
    );

    let manifest_source = load_source("../../components/code-block/src/Component.toml");
    let rbi_source = load_source("../../components/code-block/src/code_block.rbi");
    let view_source = load_source("../../components/code-block/src/view.rs");
    let protocol_source = load_source("../../components/code-block/src/protocol.rs");

    for needle in [
        "schema_version = \"1\"",
        "name = \"CodeBlock\"",
        "crate = \"ui-code-block\"",
        "name = \"code\"",
        "name = \"is_copyable\"",
        "name = \"copyable\"",
        "name = \"is_copied\"",
        "name = \"copied\"",
        "name = \"default_copied\"",
        "name = \"on_copied_change\"",
        "name = \"motion\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "CodeBlock Component.toml should include `{needle}`."
        );
    }

    for needle in [
        "pub struct CodeBlockStrings",
        "pub struct CodeBlockMotion",
        "pub enum CodeBlockComponentSchemaVersion",
        "pub struct CodeBlockComponentSpec",
        "pub fn CodeBlock(",
        "dir: Option<ui_headless::a11y::A11yDirection>",
        "is_copyable: Option<bool>",
        "copyable: Option<bool>",
        "is_copied: Option<leptos::prelude::Signal<bool>>",
        "copied: Option<leptos::prelude::Signal<bool>>",
        "on_copied_change: Option<leptos::prelude::Callback<bool>>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "CodeBlock RBI signature projection should include `{needle}`."
        );
    }

    for needle in [
        "pub fn CodeBlock(",
        "is_copyable: Option<bool>",
        "copyable: Option<bool>",
        "is_copied: Option<Signal<bool>>",
        "copied: Option<Signal<bool>>",
        "on_copied_change: Option<Callback<bool>>",
    ] {
        assert!(
            view_source.contains(needle),
            "CodeBlock view API should include `{needle}` for manifest/RBI alignment."
        );
    }

    for needle in [
        "pub enum CodeBlockComponentSchemaVersion",
        "pub struct CodeBlockComponentSpec",
    ] {
        assert!(
            protocol_source.contains(needle),
            "CodeBlock protocol surface should include `{needle}` for RBI alignment."
        );
    }
}

#[test]
fn code_block_uses_logic_state_model() {
    let view_source = load_source("../../components/code-block/src/view.rs");
    let logic_source = load_source("../../components/code-block/src/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/code_block.rs");

    for needle in [
        "pub use crate::button::normalize_optional_text;",
        "pub struct CodeBlockStateInput",
        "pub struct CodeBlockViewState",
        "pub fn resolve_state(input: CodeBlockStateInput)",
        "pub fn resolve_view_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "CodeBlock state primitive should include `{needle}` in ui-state-primitives."
        );
    }

    assert!(
        logic_source.contains("pub use ui_state_primitives::code_block::{"),
        "CodeBlock logic should consume state primitives from ui-state-primitives."
    );
    assert!(
        !logic_source.contains("pub struct CodeBlockStateInput"),
        "CodeBlock logic should not re-define primitive structs."
    );
    assert!(
        logic_source.contains("pub fn compose_class_name("),
        "CodeBlock logic should keep component assembly helpers such as class composition."
    );
    assert!(
        logic_source.contains(
            "pub fn resolve_render_model(input: CodeBlockLogicInput) -> CodeBlockRenderModel"
        ),
        "CodeBlock logic should centralize prop normalization and state derivation."
    );

    for needle in [
        "logic::resolve_render_model(logic::CodeBlockLogicInput {",
        "let state = model.state;",
        "let class_name = model.class_name;",
    ] {
        assert!(
            view_source.contains(needle),
            "CodeBlock view should consume logic output rather than re-deriving state; missing `{needle}`."
        );
    }
    assert!(
        !view_source.contains("logic::normalize_optional_text("),
        "CodeBlock view should not own prop normalization."
    );
    assert!(
        !view_source.contains("logic::resolve_state_from_content("),
        "CodeBlock view should not directly invoke state primitives."
    );
}

#[test]
fn code_block_emits_baseline_style_state_data_attributes() {
    let source = load_source("../../components/code-block/src/view.rs");

    for attr in [
        "data-slot=\"code-block\"",
        "data-state=state.state_attr",
        "data-header=state.header_attr",
        "data-multiline=state.is_multiline.then_some(\"true\")",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-label=state.has_label.then_some(\"true\")",
        "data-language=state.has_language.then_some(\"true\")",
        "data-copyable=state.copyable.then_some(\"true\")",
        "data-copied=move || copy_logic.copied.get().then_some(\"true\")",
        "data-copy-loading=move || copy_logic.is_loading.get().then_some(\"true\")",
        "data-copy-error=move || copy_logic.has_error.get().then_some(\"true\")",
        "data-copyable-source=copyable_contract.source.as_attr()",
        "data-copied-source=copy_logic.copied_source.as_attr()",
        "data-motion-source=state.motion_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "aria-busy=move || copy_logic.aria_busy.get()",
        "data-slot=\"code-block-status\"",
    ] {
        assert!(
            source.contains(attr),
            "CodeBlock should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn code_block_agent_contract_schema_is_typed_and_whitelisted() {
    let view_source = load_source("../../components/code-block/src/view.rs");
    let protocol_source = load_source("../../components/code-block/src/protocol.rs");
    let manifest_source = load_source("../../components/code-block/src/Component.toml");
    let rbi_source = load_source("../../components/code-block/src/code_block.rbi");

    for needle in [
        "let render_policy = protocol::render_policy();",
        "debug_assert!(!render_policy.allow_inner_html);",
        "debug_assert!(!render_policy.allow_script_injection);",
        "protocol::resolve_agent_data_attrs(protocol::CodeBlockAgentInput {",
        "data-ui-schema=move || agent_data.get().schema.as_attr()",
        "data-ui-intent=move || agent_data.get().intent.as_attr()",
        "data-ui-action=move || agent_data.get().action.as_attr()",
        "data-ui-state=move || agent_data.get().state.as_attr()",
        "data-ui-source=move || agent_data.get().source.as_attr()",
        "data-ui-source-copyable=move || agent_data.get().source_copyable.as_attr()",
        "data-ui-source-copied=move || agent_data.get().source_copied.as_attr()",
        "data-ui-source-motion=move || agent_data.get().source_motion.as_attr()",
        "data-ui-output-mode=move || agent_data.get().output_mode.as_attr()",
        "data-ui-output-status=move || agent_data.get().output_status.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "CodeBlock view should emit typed agent-contract marker `{needle}`."
        );
    }

    for needle in [
        "pub const CODE_BLOCK_AGENT_SCHEMA: &str = \"ui.code-block.contract.v1\";",
        "pub enum CodeBlockAgentIntent",
        "pub enum CodeBlockAgentAction",
        "pub enum CodeBlockAgentState",
        "pub enum CodeBlockAgentSource",
        "pub enum CodeBlockAgentCopyableSource",
        "pub enum CodeBlockAgentCopiedSource",
        "pub enum CodeBlockAgentMotionSource",
        "pub enum CodeBlockAgentOutputMode",
        "pub enum CodeBlockAgentOutputStatus",
        "pub struct CodeBlockRenderPolicy",
        "allow_inner_html: false,",
        "allow_script_injection: false,",
        "pub fn resolve_agent_data_attrs(input: CodeBlockAgentInput) -> CodeBlockAgentDataAttrs",
    ] {
        assert!(
            protocol_source.contains(needle),
            "CodeBlock protocol should keep typed agent-contract schema marker `{needle}`."
        );
    }

    for needle in [
        "name = \"agent-contract\"",
        "name = \"agent_contract_schema_markers\"",
        "name = \"llm_output_mode_streaming_snapshot_only\"",
        "name = \"llm_output_status_explicit_state_axis\"",
        "name = \"whitelist_render_policy_no_script_injection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "CodeBlock Component.toml should declare agent-contract capability `{needle}`."
        );
    }

    for needle in [
        "pub const CODE_BLOCK_AGENT_SCHEMA: &str;",
        "pub struct CodeBlockAgentInput",
        "pub struct CodeBlockAgentDataAttrs",
        "pub enum CodeBlockAgentOutputMode",
        "pub enum CodeBlockAgentOutputStatus",
        "pub fn resolve_agent_data_attrs(input: CodeBlockAgentInput) -> CodeBlockAgentDataAttrs;",
        "pub fn render_policy() -> CodeBlockRenderPolicy;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "CodeBlock RBI should project typed agent-contract API `{needle}`."
        );
    }

    for forbidden in [
        "inner_html",
        "dangerously_set_inner_html",
        "<script",
        "eval(",
        "Function(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CodeBlock view should avoid script injection marker `{forbidden}`."
        );
    }
}

#[test]
fn code_block_llm_output_mode_contract_is_streaming_or_snapshot_only() {
    let protocol_source = load_source("../../components/code-block/src/protocol.rs");
    let view_source = load_source("../../components/code-block/src/view.rs");
    let manifest_source = load_source("../../components/code-block/src/Component.toml");
    let rbi_source = load_source("../../components/code-block/src/code_block.rbi");

    for needle in [
        "pub enum CodeBlockAgentOutputMode {",
        "Streaming,",
        "Snapshot,",
        "Self::Streaming => \"streaming\"",
        "Self::Snapshot => \"snapshot\"",
        "pub enum CodeBlockAgentOutputStatus {",
        "Draft,",
        "Validated,",
        "ReadyToSubmit,",
        "Self::Draft => \"draft\"",
        "Self::Validated => \"validated\"",
        "Self::ReadyToSubmit => \"ready-to-submit\"",
    ] {
        assert!(
            protocol_source.contains(needle),
            "CodeBlock protocol should define two-mode output contract marker `{needle}`."
        );
    }

    assert!(
        view_source.contains(
            "let output_mode = output_mode.unwrap_or(protocol::CodeBlockAgentOutputMode::Snapshot);"
        ),
        "CodeBlock should default output mode to snapshot when caller does not opt into streaming."
    );
    assert!(
        view_source.contains(
            "#[prop(optional)] output_status: Option<protocol::CodeBlockAgentOutputStatus>"
        ),
        "CodeBlock should expose explicit typed output-status prop for draft/validated/ready-to-submit lifecycle."
    );
    assert!(
        view_source
            .contains("let output_status = output_status.unwrap_or(render_policy.output_status);"),
        "CodeBlock should default output status through render policy to keep snapshot path stable."
    );
    assert!(
        view_source.contains("data-ui-output-mode=move || agent_data.get().output_mode.as_attr()"),
        "CodeBlock view should expose machine-readable output mode marker."
    );
    assert!(
        view_source
            .contains("data-ui-output-status=move || agent_data.get().output_status.as_attr()"),
        "CodeBlock view should expose machine-readable output-status marker."
    );

    for needle in [
        "name = \"output_mode\"",
        "default = \"None (fallback: Snapshot)\"",
        "name = \"output_status\"",
        "default = \"None (fallback: Validated)\"",
        "name = \"llm_output_mode_streaming_snapshot_only\"",
        "name = \"llm_output_status_explicit_state_axis\"",
        "data-ui-output-mode",
        "data-ui-output-status",
    ] {
        assert!(
            manifest_source.contains(needle),
            "CodeBlock manifest should declare output mode contract marker `{needle}`."
        );
    }

    for needle in [
        "pub enum CodeBlockAgentOutputMode",
        "pub enum CodeBlockAgentOutputStatus",
        "output_status: CodeBlockAgentOutputStatus",
        "output_mode: Option<CodeBlockAgentOutputMode>",
        "output_status: Option<CodeBlockAgentOutputStatus>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "CodeBlock RBI should project output mode signature marker `{needle}`."
        );
    }

    for forbidden in ["Incremental", "Batch", "Partial", "Delta"] {
        assert!(
            !protocol_source.contains(forbidden),
            "CodeBlock protocol output mode taxonomy should stay limited; found `{forbidden}`."
        );
    }
}

#[test]
fn code_block_interaction_semantics_are_mounted_via_accessible_contracts() {
    let source = load_source("../../components/code-block/src/view.rs");

    for needle in [
        "on_press=on_copy_press",
        "aria_label=copy_to_clipboard_aria_label.get_value()",
        "aria-live=\"polite\"",
        "aria-atomic=\"true\"",
    ] {
        assert!(
            source.contains(needle),
            "CodeBlock should mount interaction semantics via `{needle}`."
        );
    }
}

#[test]
fn code_block_semantics_contract_covers_data_aria_role_and_source_markers() {
    let view_source = load_source("../../components/code-block/src/view.rs");
    let button_source = load_source("../../components/code-block/src/button.rs");

    for marker in [
        "data-state=state.state_attr",
        "data-header=state.header_attr",
        "data-copyable-source=copyable_contract.source.as_attr()",
        "data-copied-source=copy_logic.copied_source.as_attr()",
        "data-ui-source=move || agent_data.get().source.as_attr()",
        "data-ui-output-mode=move || agent_data.get().output_mode.as_attr()",
        "data-ui-output-status=move || agent_data.get().output_status.as_attr()",
    ] {
        assert!(
            view_source.contains(marker),
            "CodeBlock should keep semantic data-marker contract `{marker}`."
        );
    }

    for marker in [
        "aria-busy=move || copy_logic.aria_busy.get()",
        "aria-live=\"polite\"",
        "aria-atomic=\"true\"",
        "aria_label=copy_to_clipboard_aria_label.get_value()",
    ] {
        assert!(
            view_source.contains(marker),
            "CodeBlock should keep aria semantic marker `{marker}`."
        );
    }

    for marker in [
        "<button",
        "type=\"button\"",
        "on:click=move |_| on_press.run(())",
        "<pre class=\"ui-code-block__pre\"",
        "<code class=\"ui-code-block__code\"",
    ] {
        assert!(
            view_source.contains(marker) || button_source.contains(marker),
            "CodeBlock should keep native semantic-role carrier `{marker}`."
        );
    }

    assert!(
        !button_source.contains("role="),
        "CodeBlock copy button should rely on native button semantics instead of overriding role."
    );
}

#[test]
fn code_block_focus_flow_contract_is_keyboard_reachable() {
    let view_source = load_source("../../components/code-block/src/view.rs");
    let button_source = load_source("../../components/code-block/src/button.rs");
    let styles_source = load_source("../../components/code-block/src/styles.rs");

    for needle in [
        "<Button",
        "class_name=\"ui-code-block__copy-button\"",
        "aria_label=copy_to_clipboard_aria_label.get_value()",
        "on_press=on_copy_press",
    ] {
        assert!(
            view_source.contains(needle),
            "CodeBlock view should expose keyboard-reachable focus entry marker `{needle}`."
        );
    }

    for needle in [
        "<button",
        "type=\"button\"",
        "aria-label=aria_label",
        "on:click=move |_| on_press.run(())",
    ] {
        assert!(
            button_source.contains(needle),
            "CodeBlock button primitive should keep native focusable control marker `{needle}`."
        );
    }

    for needle in [
        ".ui-code-block__button:focus-visible",
        "outline: var(--ui-code-block-focus-ring-width) solid var(--ui-code-block-focus-ring);",
        "outline-offset: var(--ui-code-block-focus-ring-offset);",
    ] {
        assert!(
            styles_source.contains(needle),
            "CodeBlock styles should keep focus-visible ring contract marker `{needle}`."
        );
    }
}

#[test]
fn code_block_exposes_controlled_and_uncontrolled_copy_feedback_axis() {
    let view_source = load_source("../../components/code-block/src/view.rs");
    let logic_source = load_source("../../components/code-block/src/logic.rs");
    let snippet_source = load_source("../../components/code-block/src/snippet.rs");

    for needle in [
        "#[prop(optional)] is_copied: Option<Signal<bool>>",
        "#[prop(optional)] default_copied: Option<bool>",
        "#[prop(optional)] on_copied_change: Option<Callback<bool>>",
        "logic::resolve_copied_contract(is_copied, copied, default_copied, on_copied_change)",
        "use_snippet_logic_with_options(crate::snippet::SnippetLogicOptions {",
        "copied: copied_contract.copied,",
        "default_copied: copied_contract.default_copied,",
        "on_copied_change: copied_contract.on_copied_change,",
    ] {
        assert!(
            view_source.contains(needle),
            "CodeBlock should expose and forward controlled/uncontrolled copy feedback API `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_copied_contract(",
        "pub default_copied: bool",
        "default_copied: default_copied.unwrap_or(DEFAULT_COPIED)",
        "source: CodeBlockCopiedSource::Controlled",
    ] {
        assert!(
            logic_source.contains(needle),
            "CodeBlock logic should centralize copied default/source normalization contract `{needle}`."
        );
    }

    for needle in [
        "pub default_copied: bool",
        "pub copied_source: CodeBlockCopiedSource",
        "pub on_copied_change: Option<Callback<bool>>",
        "is_copied: options.copied",
        "default_copied: Some(options.default_copied)",
        "on_copied_change: options.on_copied_change",
    ] {
        assert!(
            snippet_source.contains(needle),
            "CodeBlock snippet adapter should consume normalized copied contract `{needle}`."
        );
    }
    assert!(
        !snippet_source.contains("unwrap_or("),
        "CodeBlock snippet adapter should not own copied defaults after logic normalization."
    );
}

#[test]
fn code_block_discrete_state_axes_are_modeled_by_typed_enums() {
    let logic_source = load_source("../../components/code-block/src/logic.rs");

    for needle in [
        "pub enum CodeBlockCopyableSource",
        "pub enum CodeBlockCopiedSource",
        "pub struct CodeBlockCopyableContract",
        "pub struct CodeBlockCopiedContract",
        "pub const fn as_attr(self) -> &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "CodeBlock should model discrete state/source axes via typed enums/contracts; missing `{needle}`."
        );
    }
}

#[test]
fn code_block_typed_axes_and_semantic_markers_form_machine_readable_contract() {
    let logic_source = load_source("../../components/code-block/src/logic.rs");
    let view_source = load_source("../../components/code-block/src/view.rs");
    let logic_test_source = load_source("../../components/code-block/test/logic.rs");

    for needle in [
        "pub enum CodeBlockCopyableSource",
        "pub enum CodeBlockCopiedSource",
        "pub struct CodeBlockCopyableContract",
        "pub struct CodeBlockCopiedContract",
        "pub fn resolve_copyable_contract(",
        "pub fn resolve_copied_contract(",
    ] {
        assert!(
            logic_source.contains(needle),
            "CodeBlock logic should keep typed state/source contract marker `{needle}`."
        );
    }

    for needle in [
        "data-state=state.state_attr",
        "data-header=state.header_attr",
        "data-copyable-source=copyable_contract.source.as_attr()",
        "data-copied-source=copy_logic.copied_source.as_attr()",
        "data-motion-source=state.motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "CodeBlock view should expose stable semantic marker `{needle}` for machine-readable state.",
        );
    }

    for needle in [
        "typed_sources_and_normalization_keep_machine_readable_state_contract",
        "assert_eq!(CodeBlockCopyableSource::Default.as_attr(), \"default\")",
        "assert_eq!(model.state.header_attr, \"hidden\")",
    ] {
        assert!(
            logic_test_source.contains(needle),
            "CodeBlock logic tests should keep normalization/typed-contract regression marker `{needle}`.",
        );
    }
}

#[test]
fn code_block_wires_locale_attrs_into_headless_copy_contract() {
    let view_source = load_source("../../components/code-block/src/view.rs");
    let snippet_source = load_source("../../components/code-block/src/snippet.rs");

    for needle in [
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "lang=copy_logic.lang.clone()",
        "dir=copy_logic.dir",
        "lang,",
        "dir,",
    ] {
        assert!(
            view_source.contains(needle),
            "CodeBlock view should expose and mount locale attrs `{needle}`."
        );
    }

    for needle in [
        "pub lang: Option<String>",
        "pub dir: Option<A11yDirection>",
        "lang: options.lang",
        "dir: options.dir",
        "lang: contract.attrs.lang",
        "dir: contract.attrs.dir",
    ] {
        assert!(
            snippet_source.contains(needle),
            "CodeBlock snippet adapter should forward locale attrs into headless contract `{needle}`."
        );
    }
}

#[test]
fn code_block_styles_include_state_marker_contracts() {
    let source = load_source("../../components/code-block/src/styles.rs");

    for selector in [
        ".ui-code-block--state-multiline",
        ".ui-code-block[data-state=\"single-line\"]",
        ".ui-code-block--header-visible",
        ".ui-code-block[data-header=\"hidden\"]",
        ".ui-code-block--copyable",
        ".ui-code-block[data-motion-source=\"custom\"]",
        ".ui-code-block--custom-class",
        ".ui-code-block[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "CodeBlock styles should include `{selector}` as stable state-marker contracts."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type"] {
        assert!(
            !source.contains(forbidden),
            "CodeBlock styles should avoid brittle structural selector `{forbidden}` for state inference.",
        );
    }

    let view_source = load_source("../../components/code-block/src/view.rs");
    assert!(
        !view_source.contains("style="),
        "CodeBlock view should not encode business styling via inline `style=` attributes.",
    );
}

#[test]
fn code_block_does_not_ignore_motion_contract() {
    let source = load_source("../../components/code-block/src/view.rs");

    assert!(
        !source.contains("let _ = motion"),
        "CodeBlock should honor `CodeBlockMotion` rather than ignoring it."
    );
}

#[test]
fn code_block_attaches_motion_driver() {
    let source = load_source("../../components/code-block/src/view.rs");

    assert!(
        source.contains("attach_motion"),
        "CodeBlock should attach its motion driver to deliver copy feedback motion."
    );
}

#[test]
fn code_block_styles_define_css_vars_for_motion() {
    let source = load_source("../../components/code-block/src/styles.rs");

    assert!(
        source.contains("--ui-code-block-copy-flash"),
        "CodeBlock styles should define `--ui-code-block-copy-flash` so motion updates only touch CSS variables."
    );
}

#[test]
fn code_block_uses_token_first_static_style_pipeline() {
    let styles_source = load_source("../../components/code-block/src/styles.rs");
    let css_aggregate_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "border: var(--ui-code-block-border-width) solid var(--ui-code-block-border-color);",
        "border-radius: var(--ui-code-block-radius-lg);",
        "background: var(--ui-code-block-bg);",
        "color: var(--ui-code-block-fg);",
        "box-shadow: var(--ui-code-block-shadow-sm);",
        "gap: var(--ui-code-block-space-sm);",
        "padding: var(--ui-code-block-space-sm) var(--ui-code-block-space-md);",
    ] {
        assert!(
            styles_source.contains(needle),
            "CodeBlock styles should stay token-first via `{needle}`."
        );
    }

    for forbidden in ["@apply", "tailwind", "styled(", "css!("] {
        assert!(
            !styles_source.contains(forbidden),
            "CodeBlock styles should not adopt utility/CSS-in-Rust default token `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-code_block\")]",
        "out.push_str(crate::code_block::styles::CSS);",
    ] {
        assert!(
            css_aggregate_source.contains(needle),
            "ui css aggregation should include CodeBlock styles via `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should inject aggregated component CSS via `{needle}`."
        );
    }
}

#[test]
fn code_block_styles_use_defensive_variable_fallback_chain() {
    let source = load_source("../../components/code-block/src/styles.rs");

    for needle in [
        "--ui-code-block-border-color: var(--ui-border, var(--ui-fallback-border));",
        "--ui-code-block-radius-lg: var(--ui-radius-lg, var(--ui-fallback-radius-lg));",
        "--ui-code-block-bg: var(--ui-bg, var(--ui-fallback-bg));",
        "--ui-code-block-fg: var(--ui-fg, var(--ui-fallback-fg));",
        "--ui-code-block-shadow-sm: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));",
        "--ui-code-block-space-sm: var(--ui-space-sm, var(--ui-fallback-space-sm));",
        "--ui-code-block-space-md: var(--ui-space-md, var(--ui-fallback-space-md));",
        "--ui-code-block-label-font-size: var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size));",
        "--ui-code-block-label-line-height: var(--ui-button-size-s-line-height, var(--ui-fallback-button-size-s-line-height));",
        "--ui-code-block-font-size-code: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
        "--ui-code-block-line-height-code: var(--ui-line-height-150, var(--ui-fallback-line-height-150));",
        "--ui-code-block-focus-ring: var(--ui-focus-ring, var(--ui-fallback-focus-ring));",
        "--ui-code-block-motion-duration: var(--ui-checkbox-group-motion-duration, var(--ui-fallback-checkbox-group-motion-duration));",
        "--ui-code-block-motion-easing: var(--ui-checkbox-group-motion-easing, var(--ui-fallback-checkbox-group-motion-easing));",
    ] {
        assert!(
            source.contains(needle),
            "CodeBlock styles should keep defensive fallback marker `{needle}`."
        );
    }

    for forbidden in [
        "border: 1px solid var(--ui-border);",
        "border-radius: var(--ui-radius-lg);",
        "background: var(--ui-bg);",
        "color: var(--ui-fg);",
        "box-shadow: var(--ui-shadow-sm);",
        "font-size: var(--ui-button-size-s-font-size, 13px);",
        "line-height: var(--ui-button-size-s-line-height, 18px);",
        "font-size: var(--ui-font-size-100, 12px);",
        "line-height: var(--ui-line-height-100, 16px);",
        "width: 16px;",
        "height: 16px;",
        "width: 28px;",
        "height: 28px;",
        "outline: 2px solid var(--ui-focus-ring);",
        "outline-offset: 2px;",
        "#",
    ] {
        assert!(
            !source.contains(forbidden),
            "CodeBlock styles should avoid hardcoded terminal style token `{forbidden}`."
        );
    }
}

#[test]
fn code_block_motion_uses_spring_animator() {
    let source = load_source("../../components/code-block/src/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "CodeBlock motion should animate via a spring to match the repo's motion spec."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn code_block_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("../../components/code-block/src/motion.rs");
    let view_source = load_source("../../components/code-block/src/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: CodeBlockMotion) -> CodeBlockMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "CodeBlock motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::motion::sanitize_motion(motion);"),
        "CodeBlock view should sanitize motion before attaching copy-flash driver.",
    );
}

#[test]
fn code_block_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn code_block() -> AnyView",
        "title=\"CodeBlock\"",
        "slug=\"code-block\"",
        "title=\"Hello World (Default API)\"",
        "title=\"Header + Copy Motion\"",
        "title=\"Compact + No Copy\"",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for CodeBlock.",
        );
    }
}

#[test]
fn code_block_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Hello World (Default API)\"",
        "code=\"cargo check -p ui\".to_string()",
        "title=\"Header + Copy Motion\"",
        "code=rust_code.to_string()",
        "language=\"rust\".to_string()",
        "label=\"deploy.rs\".to_string()",
        "title=\"Compact + No Copy\"",
        "code=\"cargo test -p ui --test code_block_semantics\".to_string()",
        "is_copyable=false",
        "class_name=\"docs-code-block-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "code-block docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn code_block_docs_parameter_matrix_stays_synced_with_logic_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let logic_source = load_source("../../components/code-block/src/logic.rs");
    let view_source = load_source("../../components/code-block/src/view.rs");

    for needle in [
        "data-slot=\"code-block-parameter-matrix\"",
        "data-slot=\"code-block-parameter-rows\"",
        "<code>\"is_copyable\"</code>\" default = true\"",
        "is_copyable > copyable > true",
        "<code>\"default_copied\"</code>\" default = false\"",
        "<code>\"is_copied + on_copied_change\"</code>",
        "<code>\"output_mode\"</code>\" default = snapshot\"",
        "<code>\"output_status\"</code>\" default = validated\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "CodeBlock docs parameter matrix should keep `{needle}`."
        );
    }

    for needle in [
        "pub const DEFAULT_IS_COPYABLE: bool = true;",
        "pub const DEFAULT_COPIED: bool = false;",
        "default_copied: default_copied.unwrap_or(DEFAULT_COPIED)",
    ] {
        assert!(
            logic_source.contains(needle),
            "CodeBlock logic defaults should keep `{needle}`."
        );
    }

    for needle in [
        "let output_mode = output_mode.unwrap_or(protocol::CodeBlockAgentOutputMode::Snapshot);",
        "let output_status = output_status.unwrap_or(render_policy.output_status);",
    ] {
        assert!(
            view_source.contains(needle),
            "CodeBlock view default mapping should keep `{needle}`."
        );
    }
}

#[test]
fn code_block_docs_workbench_supports_dx_state_and_css_hot_reload_contract() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "test_css_source=workbench_test_css",
        "test_source_path=\"components/code-block/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "ui::code_block::styles::CSS",
        "data-slot=\"code-block-workbench-controls\"",
        "id_base=\"docs-code-block-workbench-language\".to_string()",
        "id_base=\"docs-code-block-workbench-output-mode\".to_string()",
        "id_base=\"docs-code-block-workbench-output-status\".to_string()",
        "<Switch checked=workbench_preserve_state set_checked=set_workbench_preserve_state>",
        "on_press=on_workbench_load_template",
        "on_press=on_workbench_reset_copy_state",
        "id=\"docs-code-block-workbench-code\"",
        "data-slot=\"code-block-workbench-preview\"",
        "is_copied=workbench_copied_signal",
        "on_copied_change=workbench_on_copied_change",
        "Effect::new(move |_| {",
    ] {
        assert!(
            source.contains(needle),
            "CodeBlock docs workbench should keep DX marker `{needle}`."
        );
    }
}

#[test]
fn code_block_docs_interactive_playground_supports_props_state_and_spec_linkage() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_code_block_contract.spec.mjs");
    let check2_source = load_source("../../components/code-block/check2.md");

    for needle in [
        "<Playground",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "data-slot=\"code-block-workbench-controls\"",
        "id_base=\"docs-code-block-workbench-language\".to_string()",
        "id_base=\"docs-code-block-workbench-output-mode\".to_string()",
        "id_base=\"docs-code-block-workbench-output-status\".to_string()",
        "<Switch checked=workbench_is_copyable set_checked=set_workbench_is_copyable>",
        "<Switch checked=workbench_preserve_state set_checked=set_workbench_preserve_state>",
        "prop:value=move || workbench_code_text.get()",
        "on:input=move |ev| set_workbench_code_text.set(event_target_value(&ev))",
        "is_copied=workbench_copied_signal",
        "on_copied_change=workbench_on_copied_change",
        "let output_mode = workbench_output_mode.get();",
        "let output_status = workbench_output_status.get();",
        "CodeBlockWorkbenchSpecInput {",
        "CodeBlockPreviewExpectation {",
    ] {
        assert!(
            docs_source.contains(needle),
            "CodeBlock interactive playground contract should keep marker `{needle}`."
        );
    }

    for needle in [
        "gotoCodeBlockDocsAndWaitReady(",
        "runCopyFlowAndWaitSettled(",
        "docs-app code-block key flow is repeatable with semantic breakpoints",
    ] {
        assert!(
            e2e_source.contains(needle),
            "CodeBlock interactive playground should keep repeatable e2e marker `{needle}`."
        );
    }

    assert!(
        check2_source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "CodeBlock checklist should mark interactive playground item as passed with evidence."
    );
}

#[test]
fn code_block_source_first_docs_are_copy_paste_ready_and_traceable() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let check2_source = load_source("../../components/code-block/check2.md");

    for needle in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_signal=source_first_code",
        "code_imports=code_block_imports.clone()",
        "data-slot=\"code-block-source-first\"",
        "\"Source-first / Copy-Paste Ready\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "data-slot=\"code-block-source-prerequisites\"",
        "\"component-code_block\"",
        "\"inject-css\"",
        "label=\"Copy code starter\".to_string()",
        "copyable=true",
        "use leptos::prelude::*;\\nuse ui::CodeBlock;",
        "data-slot=\"code-block-source-paths\"",
        "components/code-block/src/mod.rs",
        "components/code-block/src/logic.rs",
        "components/code-block/src/view.rs",
        "components/code-block/src/styles.rs",
        "components/code-block/src/motion.rs",
    ] {
        assert!(
            docs_source.contains(needle),
            "CodeBlock source-first docs should include marker `{needle}`."
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        ".map(|snippet| compose_copy_ready_code(&snippet, &code_imports.get_value()))",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground copy-ready pipeline should include marker `{needle}`."
        );
    }

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "CodeBlock checklist should mark source-first copy-paste-ready item as passed with evidence."
    );
}

#[test]
fn code_block_heroui_strategy_and_component_docs_stay_synced() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let readme_source = load_source("../../components/code-block/src/README.md");
    let check2_source = load_source("../../components/code-block/check2.md");

    for needle in [
        "### CodeBlock 同步记录（2026-02-20）",
        "`CodeBlock` 参数主轴保持 `code` 必填",
        "`is_copied + on_copied_change + default_copied`",
        "`output_mode/output_status`",
        "component_doc!(\"CodeBlock\", \"code-block\", \"Display\", display::code_block)",
        "`apps/docs-app/src/pages/components/pages/display.rs::code_block()`",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "CodeBlock HeroUI strategy sync should include `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"CodeBlock\", \"code-block\", \"Display\", display::code_block)",
        "pub(super) fn code_block() -> AnyView",
        "title=\"CodeBlock\"",
        "slug=\"code-block\"",
    ] {
        assert!(
            docs_index_source.contains(needle) || docs_page_source.contains(needle),
            "CodeBlock docs entry/index should include `{needle}`."
        );
    }

    for needle in [
        "# CodeBlock",
        "## 先用起来（默认路径）",
        "## docs-app 文档入口",
    ] {
        assert!(
            readme_source.contains(needle),
            "CodeBlock README should remain an equivalent doc entry and include `{needle}`."
        );
    }

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "CodeBlock checklist should keep HeroUI/doc sync marker `{needle}`."
        );
    }
}

#[test]
fn code_block_documentation_is_beginner_friendly_with_readme_or_equivalent_entry() {
    let readme_source = load_source("../../components/code-block/src/README.md");
    let docs_page_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let docs_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let check2_source = load_source("../../components/code-block/check2.md");

    for needle in [
        "# CodeBlock",
        "## 先用起来（默认路径）",
        "### Hello World（最小可用）",
        "<CodeBlock code=",
        "## 常见用法",
        "## 再进阶（高级控制）",
        "默认 API 路径优先",
        "不需要用户手动接线 `ui-state-primitives` / `ui-headless`",
        "apps/docs-app/src/pages/components/pages/display.rs",
    ] {
        assert!(
            readme_source.contains(needle),
            "CodeBlock README should include beginner-friendly marker `{needle}`."
        );
    }

    let hello_index = readme_source
        .find("### Hello World（最小可用）")
        .expect("CodeBlock README should contain Hello World section.");
    let advanced_index = readme_source
        .find("## 再进阶（高级控制）")
        .expect("CodeBlock README should contain advanced section.");
    assert!(
        hello_index < advanced_index,
        "CodeBlock README should keep default path before advanced path."
    );

    for needle in [
        "pub(super) fn code_block() -> AnyView",
        "title=\"Hello World (Default API)\"",
        "title=\"Controlled vs Uncontrolled (Copied State)\"",
        "title=\"Streaming Optional / Snapshot\"",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "CodeBlock docs page should keep beginner and progression marker `{needle}`."
        );
    }

    for needle in [
        "const CODE_BLOCK_README_MD: &str =",
        "include_str!(\"../../../../../components/code-block/src/README.md\")",
        "\"code-block\" => Some(CODE_BLOCK_README_MD),",
        "let readme_html = component_readme_markdown(slug).map(crate::markdown::markdown_to_html);",
    ] {
        assert!(
            docs_shell_source.contains(needle),
            "docs-app component shell should keep CodeBlock README entry marker `{needle}`."
        );
    }

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "CodeBlock checklist should mark documentation-as-product item as passed with evidence."
    );
}

#[test]
fn code_block_css_is_layered_under_ui_and_runtime_style_updates_use_custom_properties_only() {
    let css_aggregate_source = load_source("src/css.rs");
    let view_source = load_source("../../components/code-block/src/view.rs");
    let motion_source = load_source("../../components/code-block/src/motion.rs");

    let layer_start = css_aggregate_source
        .find("out.push_str(\"\\n@layer ui {\\n\");")
        .expect("components CSS aggregation should start `@layer ui`.");
    let code_block_push = css_aggregate_source
        .find("out.push_str(crate::code_block::styles::CSS);")
        .expect("components CSS aggregation should include code-block styles.");
    let layer_end = css_aggregate_source
        .rfind("out.push_str(\"\\n}\\n\");")
        .expect("components CSS aggregation should close `@layer ui`.");

    assert!(
        layer_start < code_block_push && code_block_push < layer_end,
        "CodeBlock CSS should be aggregated inside `@layer ui` boundaries."
    );

    assert!(
        !view_source.contains("style="),
        "CodeBlock view should not emit inline runtime style assignments."
    );

    for needle in [
        "style.set_property(\"--ui-code-block-copy-flash\", \"0\")",
        "style.set_property(\"--ui-code-block-copy-flash\", &format!(\"{v}\"))",
    ] {
        assert!(
            motion_source.contains(needle),
            "CodeBlock runtime style mutation should target CSS custom property marker `{needle}`."
        );
    }

    for forbidden in [
        "set_property(\"top\",",
        "set_property(\"left\",",
        "set_property(\"right\",",
        "set_property(\"bottom\",",
        "set_property(\"width\",",
        "set_property(\"height\",",
        "set_property(\"transform\",",
        "set_property(\"opacity\",",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "CodeBlock runtime style mutation should avoid regular inline style property `{forbidden}`."
        );
    }
}

#[test]
fn code_block_engineering_contract_uses_serde_schema_tracing_and_runtime_agnostic_async() {
    let code_block_cargo = load_source("../../components/code-block/Cargo.toml");
    let ui_components_cargo = load_source("Cargo.toml");
    let protocol_source = load_source("../../components/code-block/src/protocol.rs");
    let protocol_test_source = load_source("../../components/code-block/test/protocol.rs");
    let view_source = load_source("../../components/code-block/src/view.rs");
    let mod_source = load_source("../../components/code-block/src/mod.rs");
    let logic_source = load_source("../../components/code-block/src/logic.rs");
    let snippet_source = load_source("../../components/code-block/src/snippet.rs");

    for needle in [
        "serde = { version = \"1.0\", features = [\"derive\"] }",
        "wasm-debug = [\"dep:tracing\"]",
        "tracing = { version = \"0.1\", optional = true }",
    ] {
        assert!(
            code_block_cargo.contains(needle),
            "CodeBlock crate contract should keep engineering marker `{needle}`."
        );
    }

    for needle in [
        "code-block-wasm-debug = [\"component-code_block\", \"ui-code-block/wasm-debug\"]",
        "tracing = { version = \"0.1\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui feature graph should keep CodeBlock tracing/debug marker `{needle}`."
        );
    }

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "pub enum CodeBlockComponentSchemaVersion",
        "V1",
        "pub struct CodeBlockComponentSpec",
        "#[serde(default)]",
        "pub schema_version: CodeBlockComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(needle),
            "CodeBlock protocol should keep structured serde schema marker `{needle}`."
        );
    }

    for forbidden in [
        "V2",
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecated_since",
        "deprecation_window",
    ] {
        assert!(
            !protocol_source.contains(forbidden),
            "CodeBlock protocol should not claim breaking-version migration marker `{forbidden}` without an actual major-version upgrade."
        );
    }

    for needle in [
        "fn protocol_types_implement_serde_contract()",
        "assert_serde::<CodeBlockComponentSchemaVersion>();",
        "assert_serde::<CodeBlockComponentSpec>();",
    ] {
        assert!(
            protocol_test_source.contains(needle),
            "CodeBlock protocol tests should keep serde contract marker `{needle}`."
        );
    }

    for needle in [
        "tracing::event!(",
        "target: \"ui::code_block::state_change\"",
        "tracing::Level::DEBUG",
    ] {
        assert!(
            view_source.contains(needle),
            "CodeBlock tracing contract should keep unified event marker `{needle}`."
        );
    }

    for forbidden in [
        "tokio",
        "async-std",
        "async_std",
        "Runtime",
        "JoinHandle",
        "Handle<",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "CodeBlock public module surface should not leak runtime-specific marker `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "CodeBlock logic should remain runtime-agnostic; found `{forbidden}`."
        );
        assert!(
            !snippet_source.contains(forbidden),
            "CodeBlock snippet boundary should remain runtime-agnostic; found `{forbidden}`."
        );
        assert!(
            !code_block_cargo.contains(forbidden),
            "CodeBlock crate dependencies should not bind to runtime marker `{forbidden}`."
        );
    }
}

#[test]
fn code_block_visual_desire_uses_theme_baseline_docs_and_e2e_guards() {
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        "use ui::{Button, ButtonVariant, Input, OnPress, Overlay};",
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            baseline_page_source.contains(needle),
            "Theme visual baseline docs page should keep `{needle}`."
        );
    }

    for needle in [
        "mod theme_visual_baseline;",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            baseline_registry_source.contains(needle),
            "docs-app component page registry should include visual baseline route via `{needle}`."
        );
    }

    for needle in [
        "theme visual baseline renders button/input/overlay",
        "theme visual baseline screenshots",
        "E2E_VISUAL_BASELINE",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "Theme visual baseline e2e suite should keep `{needle}`."
        );
    }
}

#[test]
fn code_block_e2e_suite_covers_repeatable_keyboard_async_critical_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_code_block_contract.spec.mjs");

    for needle in [
        "gotoCodeBlockDocsAndWaitReady(",
        "runCopyFlowAndWaitSettled(",
        "copyButton.focus()",
        "expect(copyButton).toBeFocused()",
        "page.keyboard.press(\"Space\")",
        "data-ui-state\", \"copied\"",
        "data-ui-state\", \"idle\"",
        "await page.reload();",
        "data-copied-source=\"uncontrolled\"",
        "semantic breakpoints",
    ] {
        assert!(
            e2e_source.contains(needle),
            "CodeBlock e2e regression should keep critical semantic checkpoint marker `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "CodeBlock e2e regression should avoid non-deterministic fixed wait marker `{forbidden}`."
        );
    }
}

#[test]
fn code_block_tree_shaking_contract_stays_feature_gated() {
    let ui_components_cargo = load_source("Cargo.toml");
    let ui_components_lib = load_source("src/lib.rs");
    let ui_components_css = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");

    for needle in [
        "component-code_block = [\"dep:ui-code-block\"]",
        "ui-code-block = { path = \"../../components/code-block\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui Cargo feature graph should keep code-block tree-shaking marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-code_block\")]",
        "pub use ui_code_block as code_block;",
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]",
        "#[cfg(feature = \"all-components\")]",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui lib export surface should keep feature-gated marker `{needle}`."
        );
    }

    let code_block_css_push = "out.push_str(crate::code_block::styles::CSS);";
    assert_eq!(
        ui_components_css.matches(code_block_css_push).count(),
        1,
        "code-block CSS aggregation should appear exactly once to keep reachability predictable."
    );
    assert!(
        ui_components_css.contains(
            "#[cfg(feature = \"component-code_block\")]\n    out.push_str(crate::code_block::styles::CSS);"
        ),
        "code-block CSS aggregation should stay feature-gated in css.rs."
    );

    for needle in [
        "ui = { path = \"../../crates/ui\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
        "ui-layout = { path = \"../../crates/ui-layout\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
    ] {
        assert!(
            web_demo_cargo.contains(needle),
            "web-demo should consume feature-trimmed component bundles via `{needle}`."
        );
    }
    assert!(
        !web_demo_cargo.contains("all-components"),
        "web-demo dependency contract should avoid implicit `all-components` pull-up.",
    );
}

#[test]
fn code_block_has_no_overlay_focus_stack_path() {
    let view_source = load_source("../../components/code-block/src/view.rs");
    let logic_source = load_source("../../components/code-block/src/logic.rs");

    for forbidden in [
        "<Overlay",
        "FallbackTo",
        "focus_manager",
        "restore_focus",
        "document.body",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CodeBlock should not introduce overlay focus-stack concern `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "CodeBlock logic should not carry overlay focus-stack concern `{forbidden}`.",
        );
    }
}

#[test]
fn code_block_has_no_foreign_zone_escape_hatch_path() {
    let mod_source = load_source("../../components/code-block/src/mod.rs");
    let logic_source = load_source("../../components/code-block/src/logic.rs");
    let view_source = load_source("../../components/code-block/src/view.rs");
    let motion_source = load_source("../../components/code-block/src/motion.rs");
    let snippet_source = load_source("../../components/code-block/src/snippet.rs");

    for forbidden in [
        "echarts",
        "ECharts",
        "mapbox",
        "Mapbox",
        "leaflet",
        "Leaflet",
        "YieldControl",
        "CleanupForeign",
        "ForeignZone",
        "Foreign Zone",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "CodeBlock public module boundary should not expose foreign imperative integration marker `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "CodeBlock logic should not be polluted by foreign imperative integration marker `{forbidden}`.",
        );
        assert!(
            !view_source.contains(forbidden),
            "CodeBlock view should not couple to foreign imperative integration marker `{forbidden}`.",
        );
        assert!(
            !motion_source.contains(forbidden),
            "CodeBlock motion should not embed foreign imperative integration marker `{forbidden}`.",
        );
        assert!(
            !snippet_source.contains(forbidden),
            "CodeBlock snippet adapter should not leak foreign imperative integration marker `{forbidden}`.",
        );
    }
}

#[test]
fn code_block_has_no_hydration_discontinuity_entropy_source() {
    let logic_source = load_source("../../components/code-block/src/logic.rs");
    let view_source = load_source("../../components/code-block/src/view.rs");
    let motion_source = load_source("../../components/code-block/src/motion.rs");
    let snippet_source = load_source("../../components/code-block/src/snippet.rs");

    for forbidden in [
        "SystemTime::now",
        "Instant::now",
        "js_sys::Date::now",
        "Date::now",
        "Uuid::new_v4",
        "uuid::Uuid",
        "thread_rng",
        "rand::random",
        "nanoid",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "CodeBlock logic should stay deterministic across SSR/hydration; found entropy source `{forbidden}`.",
        );
        assert!(
            !view_source.contains(forbidden),
            "CodeBlock view should stay deterministic across SSR/hydration; found entropy source `{forbidden}`.",
        );
        assert!(
            !motion_source.contains(forbidden),
            "CodeBlock motion should stay deterministic across SSR/hydration; found entropy source `{forbidden}`.",
        );
        assert!(
            !snippet_source.contains(forbidden),
            "CodeBlock snippet adapter should stay deterministic across SSR/hydration; found entropy source `{forbidden}`.",
        );
    }
}

#[test]
fn code_block_platform_branches_are_cfg_gated_and_non_wasm_safe() {
    let cargo_source = load_source("../../components/code-block/Cargo.toml");
    let motion_source = load_source("../../components/code-block/src/motion.rs");
    let view_source = load_source("../../components/code-block/src/view.rs");
    let logic_source = load_source("../../components/code-block/src/logic.rs");
    let snippet_source = load_source("../../components/code-block/src/snippet.rs");

    for needle in [
        "leptos = { version = \"0.8.15\", default-features = false, features = [\"csr\"] }",
        "ui-headless = { path = \"../../crates/ui-headless\" }",
        "ui-motion = { path = \"../../crates/ui-motion\" }",
    ] {
        assert!(
            cargo_source.contains(needle),
            "CodeBlock Cargo dependency contract should keep cross-platform dependency marker `{needle}`."
        );
    }

    for marker in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "let element: leptos::web_sys::HtmlElement = div.unchecked_into();",
    ] {
        assert!(
            motion_source.contains(marker),
            "CodeBlock motion should keep explicit platform gate marker `{marker}`."
        );
    }

    for forbidden in [
        "leptos::web_sys",
        "web_sys::",
        "js_sys::",
        "wasm_bindgen::",
        "window()",
        "document()",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CodeBlock view should not rely on browser-only API in non-wasm path `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "CodeBlock logic should not rely on browser-only API in non-wasm path `{forbidden}`.",
        );
        assert!(
            !snippet_source.contains(forbidden),
            "CodeBlock snippet adapter should not rely on browser-only API in non-wasm path `{forbidden}`.",
        );
    }
}

#[test]
fn code_block_preserves_ui_headless_web_ssr_mutex_contract() {
    let code_block_cargo = load_source("../../components/code-block/Cargo.toml");
    let ui_headless_lib = load_source("../../crates/ui-headless/src/lib.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_lib.contains(needle),
            "ui-headless should keep web/ssr mutual-exclusion guard marker `{needle}`."
        );
    }

    assert!(
        code_block_cargo.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "CodeBlock should consume the shared ui-headless crate boundary."
    );

    for forbidden in [
        "ui-headless = { path = \"../../crates/ui-headless\", features = [\"web\", \"ssr\"] }",
        "ui-headless = { path = \"../../crates/ui-headless\", default-features = false, features = [\"web\", \"ssr\"] }",
    ] {
        assert!(
            !code_block_cargo.contains(forbidden),
            "CodeBlock must not bypass ui-headless web/ssr mutex guard via dependency override `{forbidden}`."
        );
    }
}

#[test]
fn code_block_motion_consumes_ui_motion_non_wasm_noop_contract() {
    let ui_motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    let code_block_motion = load_source("../../components/code-block/src/motion.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion should keep non-wasm no-op/stub backend marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
        "let Some(driver) = driver.get_value() else {",
    ] {
        assert!(
            code_block_motion.contains(needle),
            "CodeBlock motion should keep safe non-wasm downgrade marker `{needle}`."
        );
    }

    assert!(
        !code_block_motion.contains("panic!"),
        "CodeBlock motion non-wasm path should not panic when animation runtime is unavailable."
    );
}

#[test]
fn code_block_motion_covers_reduced_motion_ssr_and_wasm_paths() {
    let ui_motion_spring = load_source("../../crates/ui-motion/src/spring.rs");
    let code_block_motion = load_source("../../components/code-block/src/motion.rs");
    let view_source = load_source("../../components/code-block/src/view.rs");

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            ui_motion_spring.contains(needle),
            "ui-motion spring runtime should keep reduced-motion immediate-settle marker `{needle}`."
        );
    }

    for marker in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "ui_motion::spring::SpringAnimator",
        "let Some(driver) = driver.get_value() else {",
    ] {
        assert!(
            code_block_motion.contains(marker),
            "CodeBlock motion should preserve wasm/ssr branch marker `{marker}`."
        );
    }

    for marker in [
        "data-state=state.state_attr",
        "data-copyable-source=copyable_contract.source.as_attr()",
        "data-copied-source=copy_logic.copied_source.as_attr()",
    ] {
        assert!(
            view_source.contains(marker),
            "CodeBlock view should preserve stable semantic markers across runtime branches via `{marker}`.",
        );
    }
}

#[test]
fn code_block_motion_contract_is_embedded_and_attached_with_safe_fallbacks() {
    let motion_source = load_source("../../components/code-block/src/motion.rs");
    let view_source = load_source("../../components/code-block/src/view.rs");
    let unit_test_source = load_source("../../components/code-block/test/motion.rs");
    let ui_motion_spring = load_source("../../crates/ui-motion/src/spring.rs");

    for marker in [
        "pub struct CodeBlockMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "pub flash_hold_ms: u64,",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0 {",
        "damping: if value.damping.is_finite() && value.damping > 0.0 {",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(marker),
            "CodeBlock motion contract should keep marker `{marker}`."
        );
    }

    assert!(
        view_source.contains("motion::attach_motion("),
        "CodeBlock view should mount motion through `attach_motion`."
    );

    for marker in [
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "assert_eq!(motion.spring.stiffness, default.spring.stiffness);",
        "assert_eq!(motion.spring.damping, default.spring.damping);",
    ] {
        assert!(
            unit_test_source.contains(marker),
            "CodeBlock motion unit tests should lock spring parameter contract marker `{marker}`."
        );
    }

    assert!(
        ui_motion_spring.contains("if crate::web::prefers_reduced_motion() {"),
        "ui-motion spring runtime should respect prefers-reduced-motion."
    );
}

#[test]
fn code_block_ui_components_entry_files_stay_on_expected_layer_boundaries() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let ui_components_src = manifest_dir.join("src");

    for required in ["lib.rs", "css.rs", "root.rs"] {
        assert!(
            ui_components_src.join(required).exists(),
            "ui entry file `{required}` should exist."
        );
    }

    for forbidden in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src.join(forbidden).exists(),
            "ui should not define `{forbidden}`; it belongs to headless/shared layers."
        );
    }

    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable_state =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let headless_presence = load_source("../../crates/ui-headless/src/presence.rs");
    let headless_a11y = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-code_block\")]",
        "pub use ui_code_block as code_block;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui lib.rs should keep entry/export boundary marker `{needle}`."
        );
    }

    for forbidden in ["pub mod overlay_open", "pub mod presence", "pub mod a11y"] {
        assert!(
            !lib_source.contains(forbidden),
            "ui lib.rs should not expose forbidden shared primitive module `{forbidden}`."
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String) {",
        "#[cfg(feature = \"component-active_highlight\")]\n    out.push_str(ui_visual_primitive::active_highlight::CSS);",
        "#[cfg(feature = \"component-code_block\")]\n    out.push_str(crate::code_block::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui css.rs should keep feature-gated CSS aggregation marker `{needle}`."
        );
    }

    for needle in [
        "#[component]",
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot entry pipeline should keep marker `{needle}`."
        );
    }

    for needle in [
        "pub struct ActiveHighlightMotion {",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "ui-visual-primitive active_highlight contract should keep marker `{needle}`."
        );
    }

    for needle in [
        "pub fn use_controllable_state",
        "pub fn use_presence",
        "pub fn aria_controls_when_open",
    ] {
        let found = headless_controllable_state.contains(needle)
            || headless_presence.contains(needle)
            || headless_a11y.contains(needle);
        assert!(
            found,
            "shared headless primitive marker `{needle}` should stay in ui-headless layer."
        );
    }
}

#[test]
fn code_block_performance_budget_uses_equivalent_regression_guards() {
    let view_source = load_source("../../components/code-block/src/view.rs");
    let motion_source = load_source("../../components/code-block/src/motion.rs");
    let logic_source = load_source("../../components/code-block/src/logic.rs");

    assert_eq!(
        view_source
            .matches("logic::resolve_render_model(logic::CodeBlockLogicInput {")
            .count(),
        1,
        "CodeBlock initial render path should keep a single normalization pass entry in view.rs."
    );

    for marker in [
        "if copied == last_copied.get_value() {",
        "if let Some(handle) = reset_timeout.get_value() {",
        "handle.clear();",
        "on_cleanup(move || {",
        "driver.borrow().stop();",
    ] {
        assert!(
            motion_source.contains(marker),
            "CodeBlock update path should keep bounded work/cleanup guard `{marker}`."
        );
    }

    for forbidden in [
        "set_interval",
        "setInterval",
        "requestIdleCallback",
        "loop {",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "CodeBlock motion should avoid unbounded scheduler pattern `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "CodeBlock view should avoid unbounded scheduler pattern `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "CodeBlock logic should avoid unbounded scheduler pattern `{forbidden}`."
        );
    }
}

#[test]
fn code_block_view_macro_is_split_into_semantic_subviews() {
    let view_source = load_source("../../components/code-block/src/view.rs");

    for needle in [
        "fn code_block_meta(",
        "fn code_block_header(",
        "fn code_block_code_content(",
        "fn code_block_status(",
        "{code_block_header(",
        "{code_block_code_content(code_value)}",
        "{code_block_status(copy_logic_for_status, copied_label)}",
    ] {
        assert!(
            view_source.contains(needle),
            "CodeBlock view should keep semantic subview split marker `{needle}`."
        );
    }

    assert!(
        !view_source.contains("<Show when=move || state.show_header>"),
        "CodeBlock root view block should delegate header rendering instead of inlining deep nested Show tree."
    );
}

#[test]
fn code_block_prefers_function_fragments_over_local_components() {
    let view_source = load_source("../../components/code-block/src/view.rs");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "CodeBlock view should keep only one component boundary and avoid promoting local fragments to extra #[component] items."
    );
    assert!(
        view_source.contains("#[component]\npub fn CodeBlock("),
        "CodeBlock should remain the sole component entry in view.rs."
    );

    for marker in [
        "fn copy_icon(copied: bool) -> impl IntoView",
        "fn code_block_meta(",
        "fn code_block_header(",
        "fn code_block_code_content(",
        "fn code_block_status(",
    ] {
        assert!(
            view_source.contains(marker),
            "CodeBlock view should preserve function-based fragment marker `{marker}`."
        );
    }
}

#[test]
fn code_block_static_svg_fragments_are_templated_as_constants() {
    let view_source = load_source("../../components/code-block/src/view.rs");

    for marker in [
        "const COPY_ICON_VIEWBOX: &str = \"0 0 20 20\";",
        "const COPIED_ICON_PATH_D: &str = \"M5 10.5l3 3 7-7\";",
        "const DEFAULT_COPY_ICON_BODY_PATH_D: &str = \"M5 13V5a2 2 0 0 1 2-2h8\";",
        "const COPY_ICON_STROKE: &str = \"currentColor\";",
        "viewBox=COPY_ICON_VIEWBOX",
        "d=COPIED_ICON_PATH_D",
        "d=DEFAULT_COPY_ICON_BODY_PATH_D",
    ] {
        assert!(
            view_source.contains(marker),
            "CodeBlock view should keep static SVG template marker `{marker}`."
        );
    }

    for forbidden in ["d=\"M5 10.5l3 3 7-7\"", "d=\"M5 13V5a2 2 0 0 1 2-2h8\""] {
        assert!(
            !view_source.contains(forbidden),
            "CodeBlock should avoid inlining repeated SVG path literal `{forbidden}` after templating."
        );
    }
}

#[test]
fn code_block_has_no_inner_html_injection_path() {
    let view_source = load_source("../../components/code-block/src/view.rs");
    let logic_source = load_source("../../components/code-block/src/logic.rs");
    let motion_source = load_source("../../components/code-block/src/motion.rs");
    let snippet_source = load_source("../../components/code-block/src/snippet.rs");

    for forbidden in ["inner_html", "innerHTML", "dangerously_set_inner_html"] {
        assert!(
            !view_source.contains(forbidden),
            "CodeBlock view should not inject HTML via `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "CodeBlock logic should not carry HTML injection path `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "CodeBlock motion should not carry HTML injection path `{forbidden}`."
        );
        assert!(
            !snippet_source.contains(forbidden),
            "CodeBlock snippet adapter should not carry HTML injection path `{forbidden}`."
        );
    }
}

#[test]
fn code_block_wasm_debug_contract_is_feature_gated_and_replayable() {
    let code_block_cargo = load_source("../../components/code-block/Cargo.toml");
    let ui_components_cargo = load_source("Cargo.toml");
    let mod_source = load_source("../../components/code-block/src/mod.rs");
    let view_source = load_source("../../components/code-block/src/view.rs");

    for needle in [
        "wasm-debug = [\"dep:tracing\"]",
        "tracing = { version = \"0.1\", optional = true }",
    ] {
        assert!(
            code_block_cargo.contains(needle),
            "ui-code-block should keep debug feature isolation marker `{needle}`."
        );
    }

    assert!(
        ui_components_cargo.contains(
            "code-block-wasm-debug = [\"component-code_block\", \"ui-code-block/wasm-debug\"]"
        ),
        "ui feature graph should expose code-block wasm debug through dedicated opt-in feature."
    );

    for needle in [
        "#[cfg(all(feature = \"wasm-debug\", debug_assertions, target_arch = \"wasm32\"))]",
        "mod wasm_debug {",
        "pub enum CodeBlockDebugSource",
        "pub struct CodeBlockDebugState",
        "pub struct CodeBlockDebugEvent",
        "pub struct CodeBlockDebugStore",
        "fn snapshot_debug_state(",
        "debug_store.record(wasm_debug::CodeBlockDebugSource::CopyButtonPress",
        "data-slot=\"code-block-debug\"",
        "data-slot=\"code-block-debug-event\"",
        "data-debug-sequence=event.sequence",
        "data-debug-logical-time=event.logical_time",
        "data-debug-source=source",
        "data-debug-before=before_attr",
        "data-debug-after=after_attr",
        "request_replay.run(event.source)",
        "CodeBlockDebugSource::Replay",
    ] {
        assert!(
            view_source.contains(needle),
            "CodeBlock wasm debug contract should include `{needle}`."
        );
    }

    for forbidden in ["pub use wasm_debug", "#[prop(optional)] wasm_debug"] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "CodeBlock debug capability should not leak through public API marker `{forbidden}`."
        );
    }
}
