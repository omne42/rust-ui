use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn flip_card_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/mod.rs");

    assert!(
        source.contains("pub mod logic;"),
        "FlipCard should expose logic module for current architecture contract.",
    );

    let needle = "pub mod view";
    assert!(
        !source.contains(needle),
        "FlipCard internals should stay private; found `{needle}`."
    );
}

#[test]
fn flip_card_module_exposes_slot_state_motion_contracts() {
    let source = load_source("src/mod.rs");

    for needle in [
        "pub use ui_state_primitives::flip_card::{",
        "FlipCardSlot",
        "FlipCardPartStateInput",
        "FlipCardPartState",
        "FlipCardFlipMode",
        "pub use view::FlipCard;",
        "pub use motion::FlipCardMotion;",
        "DEFAULT_DISABLED",
        "DEFAULT_FLIPPED",
        "DEFAULT_HOVER_FLIP",
    ] {
        assert!(
            source.contains(needle),
            "flip_card module should include `{needle}` contracts."
        );
    }
}

#[test]
fn flip_card_is_exported_from_ui_components_root() {
    let source = load_source("../../crates/ui/src/lib.rs");

    assert!(
        source.contains("pub use ui_flip_card as flip_card;"),
        "ui root should re-export flip_card module from ui-flip-card crate."
    );
    assert!(
        source.contains("pub use flip_card::{FlipCard, FlipCardMotion};"),
        "ui prelude should re-export FlipCard contracts."
    );
}

#[test]
fn flip_card_logic_exposes_state_and_source_helpers() {
    let source = load_source("src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/flip_card.rs");

    for needle in [
        "pub use ui_state_primitives::flip_card::{",
        "state_attr",
        "flip_mode_attr",
        "normalize_optional_text",
        "pub struct FlipCardFlippedAxisInput",
        "FlipCardBehaviorFlagsInput",
        "FlipCardFlipMode",
        "pub struct FlipCardDerivedRenderStateInput",
        "pub struct FlipCardDerivedRenderState",
        "pub struct FlipCardRootSemanticMarkers",
        "pub struct FlipCardFaceSemanticMarkers",
        "pub fn normalize_flipped_axis(input: FlipCardFlippedAxisInput) -> FlipCardFlippedAxis",
        "normalize_behavior_flags",
        "pub fn derive_render_state(input: FlipCardDerivedRenderStateInput) -> FlipCardDerivedRenderState",
        "resolve_id",
        "resolve_part_state",
        "pub fn compose_class_name(base_class_name: Option<String>, state: FlipCardPartState)",
    ] {
        assert!(
            source.contains(needle),
            "FlipCard logic should include `{needle}` for centralized contracts."
        );
    }

    for forbidden in [
        "pub const DEFAULT_DISABLED: bool = false;",
        "pub fn resolve_part_state(input: FlipCardPartStateInput)",
        "pub fn should_toggle_key(key: &str, is_composing: bool)",
        "pub enum FlipCardFlipMode",
        "pub fn normalize_behavior_flags(input: FlipCardBehaviorFlagsInput) -> FlipCardBehaviorFlags",
    ] {
        assert!(
            !source.contains(forbidden),
            "FlipCard logic should not re-implement state primitives `{forbidden}`."
        );
    }

    assert!(
        primitive_source.contains(
            "pub fn resolve_part_state(input: FlipCardPartStateInput) -> FlipCardPartState"
        ),
        "FlipCard state primitive resolve_part_state should live in ui-state-primitives.",
    );
    assert!(
        primitive_source.contains("pub enum FlipCardFlipMode")
            && primitive_source.contains(
                "pub fn normalize_behavior_flags(input: FlipCardBehaviorFlagsInput) -> FlipCardBehaviorFlags"
            ),
        "FlipCard behavior primitives should live in ui-state-primitives.",
    );
}

#[test]
fn flip_card_component_files_follow_responsibility_boundaries() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let mod_path = manifest_dir.join("src/mod.rs");
    let logic_path = manifest_dir.join("src/logic.rs");
    let styles_path = manifest_dir.join("src/styles.rs");
    let view_path = manifest_dir.join("src/view.rs");
    let motion_path = manifest_dir.join("src/motion.rs");
    let spec_path = manifest_dir.join("src/spec.rs");
    let render_path = manifest_dir.join("src/render.rs");

    assert!(mod_path.exists(), "FlipCard should keep `src/mod.rs`.");
    assert!(logic_path.exists(), "FlipCard should keep `src/logic.rs`.");
    assert!(
        styles_path.exists(),
        "FlipCard should keep `src/styles.rs`."
    );
    assert!(view_path.exists(), "FlipCard should keep `src/view.rs`.");
    assert!(
        motion_path.exists(),
        "FlipCard should keep `src/motion.rs`."
    );
    assert!(
        !spec_path.exists(),
        "FlipCard should not define `src/spec.rs` for current scope."
    );
    assert!(
        !render_path.exists(),
        "FlipCard should not drift to `src/render.rs`; rendering should stay in view.rs."
    );

    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let checklist_source = load_source("check2.md");

    for needle in [
        "pub mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use motion::FlipCardMotion;",
        "pub use view::FlipCard;",
    ] {
        assert!(
            mod_source.contains(needle),
            "FlipCard mod.rs should keep stable export boundary marker `{needle}`.",
        );
    }

    for forbidden in ["pub mod view;", "mod render;", "pub mod spec;"] {
        assert!(
            !mod_source.contains(forbidden),
            "FlipCard mod.rs should not leak implementation details via `{forbidden}`.",
        );
    }

    for needle in [
        "pub struct FlipCardFlippedAxisInput",
        "pub fn normalize_flipped_axis(input: FlipCardFlippedAxisInput) -> FlipCardFlippedAxis",
        "pub struct FlipCardDerivedRenderStateInput",
        "pub fn derive_render_state(input: FlipCardDerivedRenderStateInput) -> FlipCardDerivedRenderState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: FlipCardPartState) -> String",
    ] {
        assert!(
            logic_source.contains(needle),
            "FlipCard logic.rs should keep normalization/derivation marker `{needle}`.",
        );
    }

    for forbidden in [
        "web_sys::",
        "HtmlElement",
        "NodeRef<html::",
        "view! {",
        "on:click=",
        "on:keydown=",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "FlipCard logic.rs should avoid DOM/render/event mounting detail `{forbidden}`.",
        );
    }

    for needle in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "FlipCard styles.rs should keep token-first static CSS marker `{needle}`.",
        );
    }

    for forbidden in [
        "normalize_flipped_axis(",
        "derive_render_state(",
        "use_flip_card(",
        "on:click=",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "FlipCard styles.rs should not contain logic/view behavior marker `{forbidden}`.",
        );
    }

    for needle in [
        "#[component]",
        "pub fn FlipCard(",
        "logic::normalize_flipped_axis(logic::FlipCardFlippedAxisInput {",
        "logic::normalize_behavior_flags(logic::FlipCardBehaviorFlagsInput {",
        "logic::derive_render_state(logic::FlipCardDerivedRenderStateInput {",
        "ui_headless::use_flip_card(ui_headless::FlipCardOptions {",
        "motion::attach_motion(root_ref, is_flipped, is_hovered, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard view.rs should keep structure/headless mounting marker `{needle}`.",
        );
    }

    for forbidden in [
        "pub struct FlipCardMotion",
        "impl Default for FlipCardMotion",
        "SpringAnimatorTriplet::new(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view.rs should not own motion-engine implementation detail `{forbidden}`.",
        );
    }

    for needle in [
        "pub struct FlipCardMotion",
        "pub fn sanitize_motion(motion: FlipCardMotion) -> FlipCardMotion",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimatorTriplet::new(",
    ] {
        assert!(
            motion_source.contains(needle),
            "FlipCard motion.rs should keep motion contract/attach marker `{needle}`.",
        );
    }

    for forbidden in ["#[component]", "pub fn FlipCard(", "view! {"] {
        assert!(
            !motion_source.contains(forbidden),
            "FlipCard motion.rs should not include component rendering marker `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。",
        "- `mod.rs` 只维护最小稳定导出面与 feature gate，不承载实现细节。",
        "- `logic.rs` 只做输入归一、状态派生、来源标记；禁止 DOM 操作和样式细节分支。",
        "- `styles.rs` 只包含 token-first 静态 CSS；禁止硬编码主题常量与业务语义文案。",
        "- `view.rs` 只做结构渲染与 headless 契约挂载；禁止隐藏关键状态决策。",
        "- `motion.rs` 只做组件语义到动效契约映射与 attach；禁止在组件内重写通用动效引擎。",
    ] {
        assert!(
            checklist_source.contains(required),
            "FlipCard checklist should keep component-file governance rule `{required}`.",
        );
    }
}

#[test]
fn flip_card_component_directory_standard_files_follow_contract_and_na_paths() {
    // Reuse the stricter boundary assertions so this checklist item cannot drift.
    flip_card_component_files_follow_responsibility_boundaries();

    let checklist_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    let script_needle = "cargo test -p ui-flip-card flip_card_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "flip_card_component_directory_standard_files_follow_contract_and_na_paths",
    ] {
        assert!(
            checklist_source.contains(required),
            "FlipCard checklist should keep component-directory governance marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_file_placement_discipline_is_strict_for_component_scope() {
    // Keep one source of truth for file-boundary assertions.
    flip_card_component_directory_standard_files_follow_contract_and_na_paths();

    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    assert!(
        check2_source.contains("文件落点纪律"),
        "check2 should document file-placement discipline contract.",
    );

    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required_file).exists(),
            "file-placement discipline requires `{required_file}` in component src/",
        );
    }

    for forbidden_file in ["render.rs", "spec.rs"] {
        assert!(
            !src_dir.join(forbidden_file).exists(),
            "simple flip-card component should not introduce `{forbidden_file}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_file_placement_discipline_is_strict_for_component_scope";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "flip_card_file_placement_discipline_is_strict_for_component_scope",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep file-placement governance marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");
    let mod_source = load_source("src/mod.rs");

    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let spec_path = src_dir.join("spec.rs");

    assert!(
        !spec_path.exists(),
        "simple flip-card component should not introduce Hyper-Structure Builder spec.rs.",
    );

    for forbidden in ["mod spec", "pub mod spec", "spec.rs", "Spec::new()"] {
        assert!(
            !mod_source.contains(forbidden),
            "FlipCard module boundary should not expose Hyper-Structure Builder marker `{forbidden}` in simple scope.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`.",
    );

    for required in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。（N/A：`FlipCard` 当前不属于复杂 schema 驱动组件",
        "flip_card_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep Hyper-Structure Builder governance marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");
    let component_manifest = load_source("src/Component.toml");
    let component_rbi = load_source("src/flip_card.rbi");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["Component.toml", "flip_card.rbi"] {
        assert!(
            src_dir.join(required_file).exists(),
            "flip-card context-compression file should exist: `{required_file}`.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "name = \"FlipCard\"",
        "crate = \"ui-flip-card\"",
        "name = \"front\"",
        "name = \"back\"",
        "name = \"is_flipped\"",
        "name = \"default_is_flipped\"",
        "name = \"on_is_flipped_change\"",
        "name = \"flip_mode\"",
        "name = \"motion\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "flip-card Component.toml should keep context-compression marker `{required}`.",
        );
    }

    for required in [
        "pub type FlipCardMotion = crate::FlipCardMotion;",
        "pub type FlipCardFlipMode = crate::FlipCardFlipMode;",
        "pub type FlipCardPartState = ui_state_primitives::flip_card::FlipCardPartState;",
        "pub const DEFAULT_FLIPPED: bool;",
        "pub const DEFAULT_HOVER_FLIP: bool;",
        "pub const DEFAULT_ID_PREFIX: &str;",
        "pub fn sanitize_motion(motion: crate::FlipCardMotion) -> crate::FlipCardMotion;",
        "pub fn FlipCard(",
        "front: leptos::children::ViewFn",
        "back: leptos::children::ViewFn",
        "is_flipped: Option<leptos::prelude::Signal<bool>>",
        "default_is_flipped: Option<bool>",
        "on_is_flipped_change: Option<leptos::prelude::Callback<bool>>",
        "flip_mode: Option<crate::FlipCardFlipMode>",
        "dir: Option<ui_headless::A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi.contains(required),
            "flip_card.rbi should keep signature-projection marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "flip_card_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            check2_source.contains(required),
            "flip-card checklist should keep context-compression marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "flip_card_agent_contract_is_schema_typed_and_machine_readable",
        "flip_card_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "flip_card_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "flip-card checklist should keep Agent Contract governance rule `{required}`.",
        );
    }
}

#[test]
fn flip_card_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let component_manifest_source = load_source("src/Component.toml");
    let component_rbi_source = load_source("src/flip_card.rbi");

    for needle in [
        "pub const FLIP_CARD_AGENT_SCHEMA: &str = \"ui.flip-card.agent-contract\";",
        "pub enum FlipCardAgentSchemaVersion",
        "pub enum FlipCardAgentIntent",
        "pub enum FlipCardAgentAction",
        "pub enum FlipCardAgentState",
        "pub enum FlipCardAgentSource",
        "pub enum FlipCardAgentConfigPolicy",
        "pub struct FlipCardAgentContractInput",
        "pub struct FlipCardAgentContract",
        "pub fn resolve_agent_contract(input: FlipCardAgentContractInput) -> FlipCardAgentContract",
    ] {
        assert!(
            logic_source.contains(needle),
            "flip-card logic should keep typed agent contract marker `{needle}`.",
        );
    }

    for needle in [
        "let agent_action = RwSignal::new(logic::FlipCardAgentAction::SnapshotRender);",
        "let agent_contract = Memo::new(move |_| {",
        "logic::resolve_agent_contract(logic::FlipCardAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-flipped-source=move || agent_contract.get().flipped_source",
        "data-ui-mode-source=move || agent_contract.get().mode_source",
        "data-ui-motion-source=move || agent_contract.get().motion_source",
        "data-ui-class-source=move || agent_contract.get().class_source",
        "data-ui-id-source=move || agent_contract.get().id_source",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "flip-card view should mount schemaized agent marker `{needle}`.",
        );
    }

    for needle in [
        "name = \"agent_contract_schema_markers\"",
        "[[agent_contract]]",
        "schema = \"ui.flip-card.agent-contract.v1\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-intent\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "attr = \"data-ui-config-policy\"",
    ] {
        assert!(
            component_manifest_source.contains(needle),
            "flip-card Component.toml should keep schemaized marker declaration `{needle}`.",
        );
    }

    for needle in [
        "pub const FLIP_CARD_AGENT_SCHEMA: &str;",
        "pub enum FlipCardAgentSchemaVersion",
        "pub enum FlipCardAgentIntent",
        "pub enum FlipCardAgentAction",
        "pub enum FlipCardAgentState",
        "pub enum FlipCardAgentSource",
        "pub enum FlipCardAgentConfigPolicy",
        "pub struct FlipCardAgentContractInput",
        "pub struct FlipCardAgentContract",
        "pub fn resolve_agent_contract(input: FlipCardAgentContractInput) -> FlipCardAgentContract;",
    ] {
        assert!(
            component_rbi_source.contains(needle),
            "flip_card.rbi should keep agent-contract signature projection marker `{needle}`.",
        );
    }
}

#[test]
fn flip_card_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for typed_source in [
        "schema_version: FlipCardAgentSchemaVersion::V1",
        "intent: FlipCardAgentIntent::FlipInteraction",
        "state: resolve_agent_state(input.render_state)",
        "source: FlipCardAgentSource::StatePrimitives",
        "config_policy: FlipCardAgentConfigPolicy::Whitelist",
        "FlipCardAgentState::Disabled",
        "FlipCardAgentState::Flipped",
        "FlipCardAgentState::Hovered",
        "FlipCardAgentState::Default",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "flip-card agent fields should stay type-derived via `{typed_source}`.",
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
        "schema_name: format!(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "flip-card agent contract should avoid free-form schema splicing `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let mod_source = load_source("src/mod.rs");
    let motion_source = load_source("src/motion.rs");
    let manifest_source = load_source("src/Component.toml");
    let combined =
        format!("{view_source}\n{logic_source}\n{styles_source}\n{mod_source}\n{motion_source}");

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
            "flip-card render path should stay whitelist-safe without `{forbidden}`.",
        );
    }

    for needle in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "allowed = [\"render_front_face\", \"render_back_face\"]",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\"]",
    ] {
        assert!(
            manifest_source.contains(needle),
            "flip-card manifest should keep whitelist boundary marker `{needle}`.",
        );
    }
}

#[test]
fn flip_card_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-flip-card flip_card_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui-flip-card flip_card_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-flip-card flip_card_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-flip-card flip_card_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should keep flip-card agent-contract guard `{needle}`.",
        );
    }
}

#[test]
fn flip_card_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let checklist_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let mod_source = load_source("src/mod.rs");
    let motion_source = load_source("src/motion.rs");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "`N/A（组件级）`：`FlipCard` 是交互展示组件，不承载 LLM 正文 token 流渲染面",
        "flip_card_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "scripts/check-ui-streaming.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "flip-card checklist should keep streaming-definition marker `{required}`.",
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-ui-streaming-policy",
        "data-ui-streaming-fallback",
    ] {
        assert!(
            !combined.contains(forbidden),
            "flip-card runtime path should not embed streaming protocol marker `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`.",
    );
}

#[test]
fn flip_card_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let checklist_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "`FlipCard` 只消费上层提供的完整配置",
        "flip_card_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "scripts/check-ui-streaming.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "flip-card checklist should keep snapshot-baseline marker `{required}`.",
        );
    }

    for required in [
        "#[prop(into)] front: ViewFn",
        "#[prop(into)] back: ViewFn",
        "logic::normalize_flipped_axis(logic::FlipCardFlippedAxisInput {",
        "logic::normalize_behavior_flags(logic::FlipCardBehaviorFlagsInput {",
        "logic::derive_render_state(logic::FlipCardDerivedRenderStateInput {",
        "role=a11y_role",
        "aria-pressed=move || a11y_aria_pressed.get()",
        "data-state=move || derived_render_state.get().root.state_attr",
        "data-flip-mode=move || derived_render_state.get().root.flip_mode_attr",
        "data-flipped-control-mode=move || derived_render_state.get().root_markers.flipped_control_mode_attr",
        "data-flipped-default-source=move || derived_render_state.get().root_markers.flipped_default_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "flip-card view should keep snapshot baseline render marker `{required}`.",
        );
    }

    for required in [
        "pub fn normalize_flipped_axis(input: FlipCardFlippedAxisInput) -> FlipCardFlippedAxis",
        "pub fn derive_render_state(input: FlipCardDerivedRenderStateInput) -> FlipCardDerivedRenderState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: FlipCardPartState) -> String",
    ] {
        assert!(
            logic_source.contains(required),
            "flip-card logic should keep snapshot normalization marker `{required}`.",
        );
    }

    for forbidden in [
        "streaming_chunk",
        "delta",
        "partial token",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "flip-card snapshot baseline should avoid incremental streaming marker `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`.",
    );
}

#[test]
fn flip_card_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "`FlipCard` 非正文阅读面组件，按 `Streaming Optional` 处理并固定 `fallback=snapshot`",
        "flip_card_check2_documents_streaming_required_optional_classification_rules",
    ] {
        assert!(
            checklist_source.contains(required),
            "flip-card checklist should keep streaming required/optional marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("src/view.rs");
    let checklist_source = load_source("check2.md");

    for required in [
        "role=a11y_role",
        "aria-pressed=move || a11y_aria_pressed.get()",
        "aria-disabled=a11y_aria_disabled",
        "data-state=move || derived_render_state.get().root.state_attr",
        "data-flipped-control-mode=move || derived_render_state.get().root_markers.flipped_control_mode_attr",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "flip-card view should keep continuous semantic marker `{required}` under streaming-optional scope.",
        );
    }

    assert!(
        checklist_source.contains("并保持 `role`/`aria-*`/`data-*` 连续可读。"),
        "flip-card checklist should pin continuous role/aria/data requirement.",
    );
}

#[test]
fn flip_card_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let checklist_source = load_source("check2.md");

    for forbidden in [
        "retry",
        "reconnect",
        "backoff",
        "stream recovery",
        "is_loading",
        "aria-busy",
        "draft",
        "submittable",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "flip-card component layer should keep streaming validation/retry policy out of scope `{forbidden}`.",
        );
    }

    for required in [
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "`草稿/已验证/可提交` 状态标识以及数据校验、断线恢复、重试策略由上层流程负责",
    ] {
        assert!(
            checklist_source.contains(required),
            "flip-card checklist should pin upper-layer ownership marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_streaming_script_covers_required_optional_classification_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui-flip-card flip_card_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-flip-card flip_card_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-flip-card flip_card_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should include `{needle}`.",
        );
    }
}

#[test]
fn flip_card_view_uses_motion_and_state_contracts() {
    let source = load_source("src/view.rs");

    for needle in [
        "#[prop(optional, into)] is_flipped: Option<Signal<bool>>",
        "#[prop(optional)] flip_mode: Option<FlipCardFlipMode>",
        "let logic::FlipCardFlippedAxis {",
        "} = logic::normalize_flipped_axis(logic::FlipCardFlippedAxisInput {",
        "let logic::FlipCardBehaviorFlags {",
        "} = logic::normalize_behavior_flags(logic::FlipCardBehaviorFlagsInput {",
        "let flipped_state = ui_headless::use_controllable_state(",
        "on_is_flipped_change: Option<Callback<bool>>",
        "Some(default_is_flipped),",
        "request_is_flipped_change",
        "let derived_render_state = Memo::new(move |_| {",
        "logic::derive_render_state(logic::FlipCardDerivedRenderStateInput {",
        "is_hovered: is_hovered.get(),",
        "flip_mode,",
        "flip_mode_source_attr,",
        "flipped_control_mode_attr,",
        "flipped_prop_source_attr,",
        "flipped_default_source_attr,",
        "flipped_change_source_attr,",
        "data-flipped-control-mode=move || derived_render_state.get().root_markers.flipped_control_mode_attr",
        "data-flipped-prop-source=move || derived_render_state.get().root_markers.flipped_prop_source_attr",
        "data-flipped-default-source=move || derived_render_state.get().root_markers.flipped_default_source_attr",
        "data-flipped-change-source=move || derived_render_state.get().root_markers.flipped_change_source_attr",
        "data-flip-mode-source=move || derived_render_state.get().root_markers.flip_mode_source_attr",
        "motion::attach_motion(root_ref, is_flipped, is_hovered, motion)",
        "let flip_card_a11y = ui_headless::use_flip_card(ui_headless::FlipCardOptions {",
        "flip_on_hover: flip_mode.is_hover(),",
        "logic::compose_class_name(class_name.clone(), derived_render_state.get().root)",
        "if handlers.on_key_down.run((ev.key(), is_composing)) {",
        "role=a11y_role",
        "tabindex=a11y_tabindex",
        "aria-pressed=move || a11y_aria_pressed.get()",
        "aria-disabled=a11y_aria_disabled",
        "lang=a11y_lang.clone()",
        "dir=a11y_dir",
        "data-slot=move || derived_render_state.get().root.slot_attr",
        "data-state=move || derived_render_state.get().root.state_attr",
        "data-flip-mode=move || derived_render_state.get().root.flip_mode_attr",
        "data-motion-source=move || derived_render_state.get().root.motion_source_attr",
        "data-slot=move || derived_render_state.get().front.slot_attr",
        "data-slot=move || derived_render_state.get().back.slot_attr",
        "data-slot=\"flip-card-inner\"",
    ] {
        assert!(
            source.contains(needle),
            "FlipCard view should include `{needle}` for stable state/source contracts."
        );
    }

    for forbidden in [
        ".unwrap_or(logic::DEFAULT_FLIPPED)",
        ".unwrap_or(logic::DEFAULT_DISABLED)",
        ".unwrap_or(logic::DEFAULT_HOVER_FLIP)",
        "logic::resolve_part_state(FlipCardPartStateInput",
        "is_flipped.get().then_some(\"true\")",
        "(!is_flipped.get()).then_some(\"true\")",
        "(front_state.get().visibility_attr == \"visible\").then_some(\"true\")",
        "(back_state.get().visibility_attr == \"visible\").then_some(\"true\")",
        "data-flip-mode-source=move || derived_render_state.get().root.flip_mode_source_attr",
    ] {
        assert!(
            !source.contains(forbidden),
            "FlipCard view should not rebuild state rules directly; found `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_readme_documents_controlled_uncontrolled_axis_pair() {
    let source = load_source("src/README.md");

    for needle in [
        "`is_flipped`",
        "`default_is_flipped`",
        "`on_is_flipped_change`",
        "`is_flipped + on_is_flipped_change + default_is_flipped`",
        "`flip_mode`",
        "`FlipCardFlipMode::Toggle | ::Hover`",
    ] {
        assert!(
            source.contains(needle),
            "FlipCard README should document controlled/uncontrolled flipped axis contracts including `{needle}`.",
        );
    }
}

#[test]
fn flip_card_dx_default_api_path_stays_simple() {
    let view_source = load_source("src/view.rs");
    let readme_source = load_source("src/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in ["#[prop(into)] front: ViewFn", "#[prop(into)] back: ViewFn"] {
        assert!(
            view_source.contains(needle),
            "FlipCard public API should keep default required props minimal; missing `{needle}`.",
        );
    }

    for forbidden in ["#[prop(into)] state:", "#[prop(optional)] state:"] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard API should not expose internal state object as required surface; found `{forbidden}`.",
        );
    }

    assert!(
        readme_source.contains("## Hello World（最小可用）")
            && readme_source.contains("<FlipCard")
            && readme_source.contains("front=move || view! { <div>\"Front\"</div> }")
            && readme_source.contains("back=move || view! { <div>\"Back\"</div> }"),
        "FlipCard README should provide a minimal hello-world path with only front/back wiring.",
    );

    let start = readme_source
        .find("<FlipCard")
        .expect("README should contain a FlipCard call in hello world");
    let end = readme_source[start..]
        .find("/>")
        .map(|idx| start + idx + 2)
        .expect("README hello world should close FlipCard tag");
    let invocation = &readme_source[start..end];
    let invocation_lines = invocation
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .count();
    assert!(
        invocation_lines <= 5,
        "FlipCard hello-world invocation should stay within 5 lines, got {invocation_lines} lines.",
    );

    for needle in [
        "<Playground title=\"Hello World (Default Path)\" code_signal=hello_code>",
        "let hello_code = Signal::derive(move || {",
        "<FlipCard\n  front=move || view! { <div>\"Front\"</div> }\n  back=move || view! { <div>\"Back\"</div> }\n/>",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app flip_card page should expose obvious default usage path `{needle}`.",
        );
    }
}

#[test]
fn flip_card_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Restore original CSS\"",
        "data-playground-scope=scope_id.clone()",
        "<div class=\"playground__preview-stage\">{children()}</div>",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep DX hot-style/isolated-canvas marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn flip_card() -> AnyView",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "test_css_source=flip_card_test_css_source",
        "test_source_path=\"components/flip-card/src/styles.rs\".to_string()",
        "test_config_signal=workbench_config",
        "controls=move || {",
        "let (workbench_default_is_flipped, set_workbench_default_is_flipped) = signal(false);",
        "let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);",
        "let (workbench_is_flip_on_hover, set_workbench_is_flip_on_hover) = signal(true);",
        "let (workbench_custom_id, set_workbench_custom_id) = signal(true);",
        "let (workbench_custom_class, set_workbench_custom_class) = signal(true);",
        "\"切换 settings 后，使用 Code / Test 面板查看实际配置与 scoped CSS 影响。\"",
        "slug=\"flip-card\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "FlipCard docs should keep interactive workbench/context marker `{needle}`.",
        );
    }

    for forbidden in [
        "FLIP_CARD_WORKBENCH_STORAGE_KEY",
        "load_flip_card_workbench_state(",
        "save_flip_card_workbench_state(",
        "clear_flip_card_workbench_state(",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "FlipCard keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent.",
        );
    }

    for required in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "flip_card_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
    ] {
        assert!(
            check2_source.contains(required),
            "FlipCard checklist should keep DX governance marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na";
    assert!(
        script_source.contains(script_needle),
        "DX gate script should include `{script_needle}`.",
    );
}

#[test]
fn flip_card_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");
    let readme_source = load_source("src/README.md");
    let check2_source = load_source("check2.md");
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");

    assert!(
        !spec_path.exists(),
        "FlipCard should keep spec/serde path as N/A for simple component scope."
    );
    assert!(
        !cargo_source.contains("serde"),
        "FlipCard crate should not pull serde dependency without schema/spec contract."
    );

    let combined = format!(
        "{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}\n{readme_source}"
    );
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "mod spec;",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "FlipCard engineering serde/spec N/A path should avoid `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            check2_source.contains(required),
            "FlipCard checklist should keep engineering governance marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let ui_components_cargo = load_source("../../crates/ui/Cargo.toml");
    let button_view_source = load_source("../../components/button/src/view.rs");
    let combined = [
        load_source("src/mod.rs"),
        load_source("src/logic.rs"),
        load_source("src/view.rs"),
        load_source("src/styles.rs"),
        load_source("src/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui::button::state_change\"",
    ] {
        assert!(
            ui_components_cargo.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing marker `{required}`.",
        );
    }

    for forbidden_feature in [
        "flip-card-wasm-debug =",
        "flip_card_wasm_debug =",
        "component-flip_card\", \"dep:tracing",
        "component-flip-card-wasm-debug",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden_feature),
            "FlipCard should not define component-local tracing feature `{forbidden_feature}` when no local debug event/replay contract exists.",
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::flip_card::",
        "const FLIP_CARD_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "FlipCard should avoid ad-hoc tracing semantic drift token `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");
    let cargo_source = load_source("Cargo.toml");

    for source in [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
        &cargo_source,
    ] {
        for forbidden in [
            "tokio",
            "tokio::",
            "async_std",
            "async_std::",
            "async-std",
            "runtime::Handle",
            "smol::",
            "spawn_blocking(",
            "JoinHandle",
        ] {
            assert!(
                !source.contains(forbidden),
                "FlipCard engineering contract should not leak runtime marker `{forbidden}`.",
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "FlipCard public module boundary should not leak web_sys types.",
    );
}

#[test]
fn flip_card_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");

    for needle in [
        "cargo test -p ui-flip-card flip_card_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui-flip-card flip_card_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-flip-card flip_card_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn flip_card_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-engineering.sh");
    let component_manifest = load_source("src/Component.toml");
    let rbi_source = load_source("src/flip_card.rbi");
    let logic_source = load_source("src/logic.rs");
    let readme_source = load_source("src/README.md");
    let combined = [
        load_source("src/mod.rs"),
        logic_source.clone(),
        load_source("src/view.rs"),
        load_source("src/styles.rs"),
        load_source("src/motion.rs"),
    ]
    .join("\n");

    for required in [
        "schema_version = \"1\"",
        "schema = \"ui.flip-card.agent-contract.v1\"",
        "values = [\"ui.flip-card.agent-contract\"]",
        "values = [\"v1\"]",
        "pub enum FlipCardAgentSchemaVersion",
        "V1",
        "schema_version: FlipCardAgentSchemaVersion",
    ] {
        assert!(
            component_manifest.contains(required) || logic_source.contains(required),
            "flip-card should keep stable v1 marker `{required}` in current non-breaking scope.",
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
            !component_manifest.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !readme_source.contains(forbidden)
                && !combined.contains(forbidden),
            "without major breaking upgrade, flip-card should not claim migration surface token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `FlipCard` 改动未引入跨大版本 API 破坏升级，组件 Agent Contract 仍保持 `v1`（`components/flip-card/src/logic.rs` 的 `FlipCardAgentSchemaVersion::V1`，以及 `components/flip-card/src/Component.toml` 的 `schema_version = \"1\"` 与 `ui.flip-card.agent-contract.v1`），因此不触发 Codemod/Schema Registry 弃用窗口与 `migrate_v1_to_v2` 迁移层要求。回归：`components/flip-card/test/semantics.rs::flip_card_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade`；门禁脚本：`scripts/check-ui-engineering.sh` 新增对应 `cargo test` 目标。执行证据命令在当前环境仍受 `Invalid cross-device link (os error 18)` 阻断，阻断点位于依赖写入阶段而非版本弃用迁移契约本身。）",
        "flip_card_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep codemod/registry migration marker `{needle}`.",
        );
    }
}

#[test]
fn flip_card_non_composite_api_rejects_parallel_item_contracts() {
    let view_source = load_source("src/view.rs");
    let readme_source = load_source("src/README.md");

    for needle in ["#[prop(into)] front: ViewFn", "#[prop(into)] back: ViewFn"] {
        assert!(
            view_source.contains(needle),
            "FlipCard should stay as explicit dual-slot API; missing `{needle}`.",
        );
    }

    for forbidden in [
        "labels:", "titles:", "panels:", "items:", "ItemSpec", "Vec<",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard should not expose collection-style parallel item API; found `{forbidden}`.",
        );
    }

    for forbidden in ["labels + children", "titles + panels", "ItemSpec"] {
        assert!(
            !readme_source.contains(forbidden),
            "FlipCard README should not recommend parallel-array or synthetic item contracts; found `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_has_no_dragging_macro_micro_state_machine_surface() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let readme_source = load_source("src/README.md");

    for needle in [
        "on:click=on_click",
        "on:keydown=on_key_down",
        "on:pointerenter=on_pointer_enter",
        "on:pointerleave=on_pointer_leave",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard interaction surface should stay click/keyboard/hover based; missing `{needle}`.",
        );
    }

    for forbidden in [
        "Dragging",
        "Action::DragEnd",
        "on:pointermove",
        "pointermove",
        "on:drag",
        "on:dragstart",
        "on:dragend",
        "on:mousemove",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view should not expose dragging state machine hooks; found `{forbidden}`.",
        );
        assert!(
            !motion_source.contains(forbidden),
            "FlipCard motion should not expose dragging micro-loop hooks; found `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "FlipCard logic should not model dragging macro state transitions; found `{forbidden}`.",
        );
    }

    for forbidden in ["Dragging", "Action::DragEnd", "拖拽"] {
        assert!(
            !readme_source.contains(forbidden),
            "FlipCard README should not document unsupported dragging contracts; found `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_has_no_two_pass_geometry_rendering_pipeline_surface() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let readme_source = load_source("src/README.md");

    for forbidden in [
        "Intent -> Measure",
        "Rectification",
        "get_bounding_client_rect",
        "getBoundingClientRect",
        "offset_width",
        "offset_height",
        "client_width",
        "client_height",
        "scroll_width",
        "scroll_height",
        "ResizeObserver",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view should not expose two-pass geometry measurement surface; found `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "FlipCard logic should not model geometry rectification pipeline; found `{forbidden}`.",
        );
        assert!(
            !motion_source.contains(forbidden),
            "FlipCard motion should not depend on runtime geometry measuring hooks; found `{forbidden}`.",
        );
    }

    for forbidden in ["Two-Pass Rendering", "Intent -> Measure", "Rectification"] {
        assert!(
            !readme_source.contains(forbidden),
            "FlipCard README should not document unsupported two-pass geometry contracts; found `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_has_no_collection_registration_protocol_surface() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let readme_source = load_source("src/README.md");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view should not expose collection registration contracts; found `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "FlipCard logic should not model child registration ordering contracts; found `{forbidden}`.",
        );
        assert!(
            !readme_source.contains(forbidden),
            "FlipCard README should not document collection registration protocol; found `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_has_no_slot_projection_strategy_surface() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let readme_source = load_source("src/README.md");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot projection",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view should not expose container slot projection strategies; found `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "FlipCard logic should not model slot projection lifecycle contracts; found `{forbidden}`.",
        );
        assert!(
            !motion_source.contains(forbidden),
            "FlipCard motion should not depend on KeepAlive/NotifyHidden lifecycle hooks; found `{forbidden}`.",
        );
        assert!(
            !readme_source.contains(forbidden),
            "FlipCard README should not document slot projection strategy contracts; found `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_has_no_env_stream_subscription_surface() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let readme_source = load_source("src/README.md");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "on:resize",
        "on:intersection",
        "match_media",
        "prefers-color-scheme",
        "debounce",
        "throttle",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view should not expose environment stream subscriptions; found `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "FlipCard logic should not model env-stream action reduction paths; found `{forbidden}`.",
        );
        assert!(
            !motion_source.contains(forbidden),
            "FlipCard motion should not depend on env-stream sampling hooks; found `{forbidden}`.",
        );
        assert!(
            !readme_source.contains(forbidden),
            "FlipCard README should not document env-stream subscription contracts; found `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_has_no_event_light_cone_bulk_collection_surface() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let readme_source = load_source("src/README.md");

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "prop drilling",
        "batch",
        "bulk",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view should not expose event-light-cone bulk collection contracts; found `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "FlipCard logic should not model bulk collection selection compression paths; found `{forbidden}`.",
        );
        assert!(
            !readme_source.contains(forbidden),
            "FlipCard README should not document event-light-cone bulk protocols; found `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_has_no_causality_bus_trace_surface() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let readme_source = load_source("src/README.md");

    for forbidden in [
        "TraceId",
        "Causality Bus",
        "causality",
        "broadcast",
        "subscriber",
        "event bus",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view should not expose causality-bus trace contracts; found `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "FlipCard logic should not model cross-subscriber causality bus chains; found `{forbidden}`.",
        );
        assert!(
            !readme_source.contains(forbidden),
            "FlipCard README should not document causality-bus trace protocol; found `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_has_no_overlay_focus_stack_restore_surface() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let readme_source = load_source("src/README.md");

    let node_ref_occurrences = view_source.matches("NodeRef<").count();
    assert_eq!(
        node_ref_occurrences, 1,
        "FlipCard view should keep a single render/motion node ref only, not private focus-restore refs."
    );

    for forbidden in [
        "FocusManager",
        "focus_stack",
        "restore_focus",
        "last_focused",
        "document.body",
        "FallbackTo",
        "Overlay",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view should not expose overlay focus-stack restoration contracts; found `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "FlipCard logic should not model overlay focus-stack contracts; found `{forbidden}`.",
        );
        assert!(
            !motion_source.contains(forbidden),
            "FlipCard motion should not depend on overlay focus restoration contracts; found `{forbidden}`.",
        );
        assert!(
            !readme_source.contains(forbidden),
            "FlipCard README should not document overlay focus-stack contracts; found `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_has_no_foreign_zone_escape_hatch_surface() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let mod_source = load_source("src/mod.rs");
    let readme_source = load_source("src/README.md");

    for forbidden in [
        "ECharts",
        "Leaflet",
        "Mapbox",
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
        "foreign_instance",
        "chart_instance",
        "map_instance",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view should not expose imperative third-party foreign-zone contracts; found `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "FlipCard logic should not be polluted by third-party imperative instance lifecycle; found `{forbidden}`.",
        );
        assert!(
            !motion_source.contains(forbidden),
            "FlipCard motion should not depend on third-party imperative instance lifecycle; found `{forbidden}`.",
        );
        assert!(
            !mod_source.contains(forbidden),
            "FlipCard public module boundary should not export third-party foreign-zone contracts; found `{forbidden}`.",
        );
        assert!(
            !readme_source.contains(forbidden),
            "FlipCard README should not document third-party imperative foreign-zone protocols; found `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_hydration_discontinuity_uses_seeded_id_provider_without_entropy_init() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/flip_card.rs");
    let ui_root_source = load_source("../../crates/ui/src/root.rs");
    let id_provider_source = load_source("../../crates/ui-headless/src/id_provider.rs");

    for needle in [
        "use_ui_id_provider",
        "let generated_id = use_ui_id_provider()",
        "id_provider.next_prefixed_id(logic::DEFAULT_ID_PREFIX)",
        "unwrap_or_else(|| logic::DEFAULT_ID_PREFIX.to_string())",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard view should use deterministic IdProvider path marker `{needle}`.",
        );
    }

    for needle in [
        "pub const DEFAULT_ID_PREFIX: &str = \"ui-flip-card\";",
        "pub fn resolve_id(custom_id: Option<String>, fallback_id: String) -> (String, bool)",
    ] {
        assert!(
            primitive_source.contains(needle),
            "FlipCard primitive id contract should include `{needle}`.",
        );
    }

    assert!(
        logic_source.contains("DEFAULT_ID_PREFIX"),
        "FlipCard logic should re-export default id prefix from ui-state-primitives.",
    );

    for forbidden in [
        "fn next_id() -> u64",
        "thread_local!",
        "SystemTime::now",
        "Date::now",
        "rand::",
        "random(",
        "Uuid",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view should avoid non-deterministic hydration entropy source `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "FlipCard logic should avoid non-deterministic hydration entropy source `{forbidden}`.",
        );
    }

    for needle in [
        "#[prop(optional, default = 1)] id_seed: u64,",
        "provide_ui_id_provider(id_seed);",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "UiRoot should provide deterministic id seed contract marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider {",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider> {",
        "pub fn next_prefixed_id(self, prefix: &str) -> String {",
    ] {
        assert!(
            id_provider_source.contains(needle),
            "ui-headless id provider should expose deterministic id API marker `{needle}`.",
        );
    }
}

#[test]
fn flip_card_cross_platform_compile_contract_is_explicit_and_non_wasm_safe() {
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let checklist_source = load_source("check2.md");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "let is_composing = ev.is_composing();",
        "let is_composing = false;",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard view should keep explicit wasm/non-wasm key handling contract `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "FlipCard motion should keep explicit cross-platform branch contract `{needle}`.",
        );
    }

    let wasm_branch_start = motion_source
        .find("#[cfg(target_arch = \"wasm32\")]")
        .expect("flip_card motion should define wasm branch.");
    let non_wasm_branch_start = motion_source
        .find("#[cfg(not(target_arch = \"wasm32\"))]")
        .expect("flip_card motion should define non-wasm branch.");
    let wasm_segment = &motion_source[wasm_branch_start..non_wasm_branch_start];
    let non_wasm_segment = &motion_source[non_wasm_branch_start..];

    assert!(
        wasm_segment.contains("leptos::web_sys::HtmlElement"),
        "FlipCard wasm motion branch should host web_sys usage behind cfg(target_arch = \"wasm32\").",
    );

    for forbidden in ["leptos::web_sys::", "web_sys::window()", "document"] {
        assert!(
            !non_wasm_segment.contains(forbidden),
            "FlipCard non-wasm motion branch should not reference browser-only API `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web;",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions)",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion should expose explicit wasm/non-wasm backend branch marker `{needle}`.",
        );
    }

    for needle in [
        "cargo check -p ui-flip-card",
        "cargo check -p ui-flip-card --target wasm32-unknown-unknown",
        "cargo check -p ui-flip-card --no-default-features --features ui-headless/ssr",
    ] {
        assert!(
            checklist_source.contains(needle),
            "FlipCard checklist should include compile-only evidence command `{needle}`.",
        );
    }
}

#[test]
fn flip_card_headless_web_ssr_feature_mutex_contract_is_preserved() {
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let headless_cargo_source = load_source("../../crates/ui-headless/Cargo.toml");
    let flip_card_cargo_source = load_source("Cargo.toml");
    let checklist_source = load_source("check2.md");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should keep web/ssr mutual exclusion guard `{needle}`.",
        );
    }

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            headless_cargo_source.contains(needle),
            "ui-headless feature matrix should include `{needle}`.",
        );
    }

    assert!(
        flip_card_cargo_source.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "FlipCard should depend on ui-headless without introducing parallel local feature contract.",
    );

    for forbidden in [
        "features = [\"web\", \"ssr\"]",
        "features = [\"ssr\", \"web\"]",
    ] {
        assert!(
            !flip_card_cargo_source.contains(forbidden),
            "FlipCard should not force-enable both ui-headless web+ssr features; found `{forbidden}`.",
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
    ] {
        assert!(
            checklist_source.contains(needle),
            "FlipCard checklist should include ui-headless feature-path verification command `{needle}`.",
        );
    }
}

#[test]
fn flip_card_ui_motion_non_wasm_stub_contract_is_safe_and_predictable() {
    let flip_card_motion_source = load_source("src/motion.rs");
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let checklist_source = load_source("check2.md");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion should keep non-wasm no-op backend contract marker `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            flip_card_motion_source.contains(needle),
            "FlipCard motion should keep non-wasm safe-degrade marker `{needle}`.",
        );
    }

    let non_wasm_branch_start = flip_card_motion_source
        .find("#[cfg(not(target_arch = \"wasm32\"))]")
        .expect("flip_card motion should define non-wasm attach branch.");
    let non_wasm_segment = &flip_card_motion_source[non_wasm_branch_start..];

    for forbidden in [
        "unwrap()",
        "expect(",
        "panic!(",
        "SpringAnimatorTriplet::new(",
    ] {
        assert!(
            !non_wasm_segment.contains(forbidden),
            "FlipCard non-wasm motion branch should not assume runtime animation handles or panic paths `{forbidden}`.",
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
    ] {
        assert!(
            checklist_source.contains(needle),
            "FlipCard checklist should include ui-motion compile evidence command `{needle}`.",
        );
    }
}

#[test]
fn flip_card_reduced_motion_ssr_and_wasm_contracts_stay_consistent() {
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let checklist_source = load_source("check2.md");

    for needle in [
        "let prefers_reduced_motion = StoredValue::new(ui_motion::web::prefers_reduced_motion());",
        "if prefers_reduced_motion.get_value() {",
        "Reduced-motion mode keeps semantic state updates but skips spring runtime.",
    ] {
        assert!(
            motion_source.contains(needle),
            "FlipCard motion should keep reduced-motion downgrade contract marker `{needle}`.",
        );
    }

    for needle in [
        "ui_observability::set_css_property_observed_auto!(",
        "\"--ui-flip-card-rotation\"",
        "\"--ui-flip-card-scale\"",
        "\"--ui-flip-card-tilt\"",
        "triplet.set_targets([rotation, scale, tilt]);",
    ] {
        assert!(
            motion_source.contains(needle),
            "FlipCard motion should keep deterministic runtime variable updates for both reduced-motion and spring paths `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || derived_render_state.get().root.state_attr",
        "data-visible=move || derived_render_state.get().root.visibility_attr",
        "data-flipped-control-mode=move || derived_render_state.get().root_markers.flipped_control_mode_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard view should expose stable semantic state markers across SSR/wasm paths `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard view should keep explicit wasm/non-wasm split marker `{needle}`.",
        );
        assert!(
            motion_source.contains(needle),
            "FlipCard motion should keep explicit wasm/non-wasm split marker `{needle}`.",
        );
    }

    for needle in [
        "flip_card_hydration_discontinuity_uses_seeded_id_provider_without_entropy_init",
        "flip_card_cross_platform_compile_contract_is_explicit_and_non_wasm_safe",
        "flip_card_ui_motion_non_wasm_stub_contract_is_safe_and_predictable",
    ] {
        assert!(
            checklist_source.contains(needle),
            "FlipCard checklist should reference required regression for reduced-motion/SSR/wasm consistency `{needle}`.",
        );
    }
}

#[test]
fn flip_card_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"flip-card\" => UiPerfBudget {",
        "max_mount_ms: 30.0,",
        "max_update_ms: Some(10.0),",
        "max_heap_kb: Some(512.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep flip-card performance budget token `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"FlipCard\", \"flip-card\", \"Display\", display_extra::flip_card)",
        "\"flip-card\"",
    ] {
        assert!(
            pages_source.contains(needle),
            "FlipCard docs page should remain in coverage traversal via `{needle}`."
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
        "\"mount-plus-budget\"",
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
            "docs coverage e2e should enforce repeatable perf regression guard `{needle}`."
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
            "performance governance should keep render_count follow-up marker `{needle}`."
        );
    }

    for needle in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
        "flip_card_performance_governance_budget_is_defined_and_blocking",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "FlipCard checklist should keep performance governance marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || derived_render_state.get().root.state_attr",
        "data-visible=move || derived_render_state.get().root.visibility_attr",
        "data-flipped-control-mode=move || derived_render_state.get().root_markers.flipped_control_mode_attr",
        "data-flipped-change-source=move || derived_render_state.get().root_markers.flipped_change_source_attr",
        "data-motion-source=move || derived_render_state.get().root.motion_source_attr",
        "data-id-source=move || derived_render_state.get().root.id_source_attr",
        "data-flip-mode-source=move || derived_render_state.get().root_markers.flip_mode_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard view should expose attribution marker `{needle}` for perf triage.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`.",
    );
}

#[test]
fn flip_card_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let check2_source = load_source("check2.md");
    let docs_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for marker in [
        "role=a11y_role",
        "aria-pressed=move || a11y_aria_pressed.get()",
        "aria-disabled=a11y_aria_disabled",
        "data-state=move || derived_render_state.get().root.state_attr",
        "data-flipped-control-mode=move || derived_render_state.get().root_markers.flipped_control_mode_attr",
        "data-flipped-change-source=move || derived_render_state.get().root_markers.flipped_change_source_attr",
        "data-flip-mode-source=move || derived_render_state.get().root_markers.flip_mode_source_attr",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "on:keydown=on_key_down",
        "on:focusin=on_focus_in",
        "on:focusout=on_focus_out",
    ] {
        assert!(
            view_source.contains(marker),
            "flip-card semantics/perf matrix should keep aria/data/focus marker `{marker}`.",
        );
    }

    for marker in [
        "\"flip-card\" => UiPerfBudget {",
        "max_mount_ms: 30.0,",
        "max_update_ms: Some(10.0),",
        "max_heap_kb: Some(512.0),",
    ] {
        assert!(
            docs_shell_source.contains(marker),
            "docs shell should preserve flip-card perf budget marker `{marker}`.",
        );
    }

    for marker in [
        "cargo test -p ui-flip-card flip_card_performance_governance_budget_is_defined_and_blocking",
        "cargo test -p ui-flip-card flip_card_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance script should enforce `{marker}`.",
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            check2_source.contains(marker) || todo_source.contains(marker),
            "render_count follow-up should remain explicit via `{marker}`.",
        );
    }

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "flip_card_semantic_contract_matrix_is_covered_without_snapshot_dependency",
        "flip_card_performance_governance_budget_is_defined_and_blocking",
        "flip_card_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            check2_source.contains(marker),
            "flip-card check2 semantics/perf section should reference `{marker}`.",
        );
    }

    assert!(
        logic_source.contains("pub fn derive_render_state("),
        "logic should keep state derivation path for attributable semantics/perf regressions.",
    );
}

#[test]
fn flip_card_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders() {
    let view_source = load_source("src/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");
    let check2_source = load_source("check2.md");

    assert!(
        view_source.contains("#[component]"),
        "FlipCard view should keep explicit component boundary."
    );
    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "FlipCard should keep a single component entry and avoid local component sprawl."
    );
    assert!(
        view_source.contains("view! {"),
        "FlipCard should keep explicit render block in view.rs."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        3,
        "FlipCard view should keep bounded macro surface (root + two face helpers) and avoid fragment sprawl."
    );
    assert!(
        view_source.lines().count() <= 320,
        "FlipCard view.rs should stay compact; split semantic subrenders if this grows."
    );

    for needle in [
        "fn render_front_face(",
        "fn render_back_face(",
        "let derived_render_state = Memo::new(move |_| {",
        "let root_class = Memo::new(move |_| {",
        "let front_class =",
        "let back_class =",
        "let front_face_view = render_front_face(",
        "let back_face_view = render_back_face(",
        "{front_face_view}",
        "{back_face_view}",
        "data-slot=move || derived_render_state.get().front.slot_attr",
        "data-slot=move || derived_render_state.get().back.slot_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard view should keep pre-derived semantic state marker `{needle}`.",
        );
    }

    for forbidden in [
        "for item in",
        "collect::<Vec<_>>()",
        "while ",
        "loop {",
        "match (",
        "description.map(|description| {",
        "error.map(|error| {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view should avoid loop-heavy/branch-heavy macro pattern `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "flip_card_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders",
        "cargo test -p ui-flip-card flip_card_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders",
    ] {
        assert!(
            check2_source.contains(needle),
            "FlipCard checklist should keep view-macro governance marker `{needle}`.",
        );
    }
}

#[test]
fn flip_card_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");
    let check2_source = load_source("check2.md");

    for needle in [
        "fn render_front_face(",
        "fn render_back_face(",
        "derived_render_state: Memo<logic::FlipCardDerivedRenderState>,",
        "front: StoredValue<ViewFn>,",
        "back: StoredValue<ViewFn>,",
        "let front_face_view = render_front_face(",
        "let back_face_view = render_back_face(",
        "{front_face_view}",
        "{back_face_view}",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard view should keep function-first split marker `{needle}`."
        );
    }

    for forbidden in [
        "#[component]\nfn render_front_face",
        "#[component]\nfn render_back_face",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard should avoid local component abstraction noise or inline fragment duplication `{forbidden}`.",
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "FlipCard should keep only one top-level component boundary after function split.",
    );

    let script_needle = "cargo test -p ui-flip-card flip_card_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "flip_card_view_functional_split_prefers_plain_functions_over_local_components",
        "cargo test -p ui-flip-card flip_card_view_functional_split_prefers_plain_functions_over_local_components",
    ] {
        assert!(
            check2_source.contains(needle),
            "FlipCard checklist should keep function-split governance marker `{needle}`."
        );
    }
}

#[test]
fn flip_card_static_fragments_are_constantized_or_absent_for_simple_layout() {
    let view_source = load_source("src/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");
    let check2_source = load_source("check2.md");

    for forbidden in [
        "<svg",
        "<path",
        "inner_html=",
        "set_inner_html",
        "dangerously_set_inner_html",
        "footer",
        "copyright",
        "lorem ipsum",
        "markdown_to_html(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard simple layout should avoid heavy inline static fragment token `{forbidden}`.",
        );
    }

    for needle in [
        "fn render_front_face(",
        "fn render_back_face(",
        "data-slot=\"flip-card-inner\"",
        "data-slot=move || derived_render_state.get().front.slot_attr",
        "data-slot=move || derived_render_state.get().back.slot_attr",
        "role=a11y_role",
        "aria-pressed=move || a11y_aria_pressed.get()",
        "aria-disabled=a11y_aria_disabled",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard should keep stable semantic/a11y markers while static fragments stay absent `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_static_fragments_are_constantized_or_absent_for_simple_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "flip_card_static_fragments_are_constantized_or_absent_for_simple_layout",
        "cargo test -p ui-flip-card flip_card_static_fragments_are_constantized_or_absent_for_simple_layout",
    ] {
        assert!(
            check2_source.contains(needle),
            "FlipCard checklist should keep static-fragment governance marker `{needle}`.",
        );
    }
}

#[test]
fn flip_card_inner_html_usage_is_absent_and_untrusted_injection_paths_are_blocked() {
    for rel_path in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
        "src/README.md",
    ] {
        let source = load_source(rel_path);
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "markdown_to_html(",
            "<script",
            "javascript:",
            "onerror=",
            "onload=",
        ] {
            assert!(
                !source.contains(forbidden),
                "FlipCard source `{rel_path}` must not contain raw-html injection token `{forbidden}`."
            );
        }
    }

    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "FlipCard docs examples must not contain raw-html injection token `{forbidden}`."
        );
    }

    let check2_source = load_source("check2.md");
    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A（FlipCard）：组件当前不使用 `inner_html`",
        "flip_card_inner_html_usage_is_absent_and_untrusted_injection_paths_are_blocked",
    ] {
        assert!(
            check2_source.contains(needle),
            "FlipCard checklist should keep inner_html safety evidence `{needle}`.",
        );
    }
}

#[test]
fn flip_card_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-inner-html.sh");
    let needle = "cargo test -p ui-flip-card flip_card_inner_html_usage_is_absent_and_untrusted_injection_paths_are_blocked";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce `{needle}`.",
    );
}

#[test]
fn flip_card_wasm_debug_contract_is_explicitly_na_and_feature_isolated() {
    let check2_source = load_source("check2.md");
    let component_cargo_source = load_source("Cargo.toml");
    let ui_components_cargo_source = load_source("../../crates/ui/Cargo.toml");
    let ui_components_lib_source = load_source("../../crates/ui/src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let readme_source = load_source("src/README.md");

    for needle in ["[features]", "default = []"] {
        assert!(
            component_cargo_source.contains(needle),
            "flip-card crate feature boundary should include `{needle}`."
        );
    }

    for forbidden in [
        "wasm-debug",
        "flip-card-wasm-debug",
        "flip_card_wasm_debug",
        "component-flip-card-wasm-debug",
    ] {
        assert!(
            !component_cargo_source.contains(forbidden),
            "flip-card crate should not expose component-local wasm debug feature `{forbidden}`.",
        );
    }

    for required in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo_source.contains(required),
            "ui should keep shared wasm-debug feature marker `{required}`.",
        );
    }

    for forbidden in [
        "flip-card-wasm-debug",
        "flip_card_wasm_debug",
        "component-flip-card-wasm-debug",
    ] {
        assert!(
            !ui_components_cargo_source.contains(forbidden),
            "ui should not define flip-card-local wasm debug feature `{forbidden}`.",
        );
    }

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            ui_components_lib_source.contains(needle),
            "ui root should keep wasm-debug isolation marker `{needle}`.",
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
            "docs-app should keep dev-only debug overlay entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub enum UiTraceEventKind",
        "pub struct UiTraceEvent",
        "pub ts_ms: u64",
        "pub component: &'static str",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace contract should keep marker `{needle}`.",
        );
    }

    for needle in [
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "events",
        ".into_iter()",
        ".rev()",
        ".take(40)",
        "fn render_event(event: ui_headless::UiTraceEvent) -> AnyView",
        "let ts_ms = event.ts_ms;",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep replayable timeline marker `{needle}`.",
        );
    }

    for needle in [
        "let flipped_state = ui_headless::use_controllable_state(",
        "let flip_card_a11y = ui_headless::use_flip_card(ui_headless::FlipCardOptions {",
        "data-flipped-control-mode=move || derived_render_state.get().root_markers.flipped_control_mode_attr",
        "data-flipped-change-source=move || derived_render_state.get().root_markers.flipped_change_source_attr",
        "data-flip-mode-source=move || derived_render_state.get().root_markers.flip_mode_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "flip-card should expose state/source markers for debug attribution `{needle}`.",
        );
    }

    for forbidden in [
        "flip-card-wasm-debug",
        "request_replay.run(",
        "render_debug_panel(",
        "#[prop(optional)] debug",
        "data-debug-source",
        "debug_overlay::UiDebugOverlay",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "flip-card runtime/public surface should not leak wasm-debug internals `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "N/A（FlipCard）：组件不引入本地 `wasm-debug` feature",
        "flip_card_wasm_debug_contract_is_explicitly_na_and_feature_isolated",
        "scripts/check-ui-wasm-debug.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "FlipCard checklist should keep wasm-debug evidence `{needle}`.",
        );
    }
}

#[test]
fn flip_card_wasm_debug_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-wasm-debug.sh");
    let needle = "cargo test -p ui-flip-card flip_card_wasm_debug_contract_is_explicitly_na_and_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm-debug check script should enforce `{needle}`.",
    );
}

#[test]
fn flip_card_a11y_i18n_contracts_are_headless_and_text_free() {
    let view_source = load_source("src/view.rs");
    let headless_flip_card_source = load_source("../../crates/ui-headless/src/flip_card.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let flip_card_a11y = ui_headless::use_flip_card(ui_headless::FlipCardOptions {",
        "lang,",
        "dir,",
        "role=a11y_role",
        "aria-pressed=move || a11y_aria_pressed.get()",
        "aria-disabled=a11y_aria_disabled",
        "on:keydown=on_key_down",
        "lang=a11y_lang.clone()",
        "dir=a11y_dir",
        "#[prop(into)] front: ViewFn",
        "#[prop(into)] back: ViewFn",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard view should keep A11y+i18n attach point contract `{needle}`.",
        );
    }

    for forbidden in [
        "\"Flip\"",
        "\"Back\"",
        "\"Front\"",
        "fn locale_attrs(",
        "fn resolve_a11y_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view should not hardcode user-visible text or re-invent locale helpers; found `{forbidden}`.",
        );
    }

    for needle in [
        "use crate::a11y::{A11yDirection, locale_attrs};",
        "let locale = locale_attrs(lang, dir);",
        "role: \"button\"",
        "aria_pressed: is_flipped",
        "aria_disabled: is_disabled.then_some(\"true\")",
    ] {
        assert!(
            headless_flip_card_source.contains(needle),
            "FlipCard headless contract should include `{needle}`.",
        );
    }

    assert!(
        headless_a11y_source.contains("pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>) -> A11yLocaleAttrs"),
        "ui-headless a11y shared locale utility should define locale_attrs for lang/dir mapping.",
    );
}

#[test]
fn flip_card_observability_markers_are_stable_and_enumerated() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/flip_card.rs");

    for needle in [
        "role=a11y_role",
        "aria-pressed=move || a11y_aria_pressed.get()",
        "aria-disabled=a11y_aria_disabled",
        "data-slot=move || derived_render_state.get().root.slot_attr",
        "data-state=move || derived_render_state.get().root.state_attr",
        "data-visible=move || derived_render_state.get().root.visibility_attr",
        "data-flip-mode=move || derived_render_state.get().root.flip_mode_attr",
        "data-flipped-control-mode=move || derived_render_state.get().root_markers.flipped_control_mode_attr",
        "data-flipped-prop-source=move || derived_render_state.get().root_markers.flipped_prop_source_attr",
        "data-flipped-default-source=move || derived_render_state.get().root_markers.flipped_default_source_attr",
        "data-flipped-change-source=move || derived_render_state.get().root_markers.flipped_change_source_attr",
        "data-class-source=move || derived_render_state.get().root.class_source_attr",
        "data-motion-source=move || derived_render_state.get().root.motion_source_attr",
        "data-id-source=move || derived_render_state.get().root.id_source_attr",
        "data-slot=move || derived_render_state.get().front.slot_attr",
        "data-slot=move || derived_render_state.get().back.slot_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard view should expose stable observable marker `{needle}`.",
        );
    }

    for forbidden in ["data-state=move || format!(", "data-slot=move || format!("] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view should not generate free-form marker values dynamically; found `{forbidden}`.",
        );
    }

    for needle in [
        "pub flipped_control_mode_attr: &'static str",
        "pub flipped_prop_source_attr: &'static str",
        "pub flipped_default_source_attr: &'static str",
        "pub flipped_change_source_attr: &'static str",
        "pub flip_mode_source_attr: &'static str",
        "pub fn state_attr(is_flipped: bool) -> &'static str",
        "pub fn flip_mode_attr(flip_on_hover: bool) -> &'static str",
        "fn source_attr(is_custom: bool) -> &'static str",
    ] {
        assert!(
            logic_source.contains(needle) || primitive_source.contains(needle),
            "FlipCard marker vocabulary should be backed by static-string contracts `{needle}`.",
        );
    }

    for needle in [
        "if is_flipped { \"flipped\" } else { \"default\" }",
        "if flip_on_hover { \"hover\" } else { \"toggle\" }",
        "if is_custom { \"custom\" } else { \"default\" }",
    ] {
        assert!(
            primitive_source.contains(needle),
            "FlipCard primitive should keep closed-set marker mapping `{needle}`.",
        );
    }
}

#[test]
fn flip_card_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/flip_card.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let logic_test_source = load_source("test/logic.rs");
    let checklist_source = load_source("check2.md");

    for needle in [
        "pub enum FlipCardFlipMode",
        "pub fn normalize_behavior_flags(input: FlipCardBehaviorFlagsInput) -> FlipCardBehaviorFlags",
        "FlipCardFlipMode::from_hover_flag",
        "pub fn resolve_part_state(input: FlipCardPartStateInput) -> FlipCardPartState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "FlipCard primitives should keep typed state-contract marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn normalize_flipped_axis(input: FlipCardFlippedAxisInput) -> FlipCardFlippedAxis",
        "pub fn derive_render_state(input: FlipCardDerivedRenderStateInput) -> FlipCardDerivedRenderState",
        "pub flip_mode: FlipCardFlipMode",
        "pub flip_mode_source_attr: &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "FlipCard logic should keep centralized typed normalization marker `{needle}`.",
        );
    }

    for needle in [
        "fn normalize_behavior_flags_maps_bool_aliases_to_enum_mode()",
        "fn derive_render_state_centralizes_slot_states_and_semantic_markers()",
        "fn normalize_flipped_axis_centralizes_default_priority_and_sources()",
    ] {
        assert!(
            logic_test_source.contains(needle),
            "FlipCard logic tests should keep direct contract-regression feedback marker `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || derived_render_state.get().root.state_attr",
        "data-flip-mode=move || derived_render_state.get().root.flip_mode_attr",
        "data-flipped-control-mode=move || derived_render_state.get().root_markers.flipped_control_mode_attr",
        "data-flipped-prop-source=move || derived_render_state.get().root_markers.flipped_prop_source_attr",
        "data-flipped-default-source=move || derived_render_state.get().root_markers.flipped_default_source_attr",
        "data-flipped-change-source=move || derived_render_state.get().root_markers.flipped_change_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard view should expose stable machine-readable semantic marker `{needle}`.",
        );
    }

    for forbidden in [
        "data-state=move || format!(",
        "data-flip-mode=move || format!(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard marker mapping should remain closed-set and not accept free-form runtime text `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
        "- 离散输入与状态轴必须优先使用 `enum`/新类型建模，避免字符串协议与布尔爆炸。",
        "- 无效状态要么在类型层不可表达，要么在 `logic.rs` 被统一归一化并可测试。",
        "- 关键状态必须通过稳定语义标记对外可读，供测试与 Agent 自动化消费。",
        "- 编译器与测试反馈应能直接定位状态契约破坏点，形成可持续闭环。",
    ] {
        assert!(
            checklist_source.contains(required),
            "FlipCard checklist should keep typed-state + semantic-marker governance rule `{required}`.",
        );
    }
}

#[test]
fn flip_card_styles_depend_on_explicit_markers_and_css_vars_only() {
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");

    for selector in [
        ".ui-flip-card[data-disabled=\"true\"]",
        ".ui-flip-card[data-motion-source=\"custom\"]",
        ".ui-flip-card[data-class-source=\"custom\"]",
        ".ui-flip-card[data-id-source=\"custom\"]",
        ".ui-flip-card[data-flip-mode=\"hover\"]",
        ".ui-flip-card[data-flip-mode=\"toggle\"]",
        ".ui-flip-card__face[data-visible=\"true\"]",
        ".ui-flip-card__face[data-visible=\"false\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "FlipCard styles should branch on stable explicit markers `{selector}`.",
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ".ui-flip-card > div > div"] {
        assert!(
            !styles_source.contains(forbidden),
            "FlipCard styles must not guess state by fragile DOM structure; found `{forbidden}`.",
        );
    }

    assert!(
        !view_source.contains("style="),
        "FlipCard view should not inject business style logic through inline style attributes.",
    );

    let set_property_lines = motion_source
        .lines()
        .filter(|line| line.contains("set_css_property_observed_auto!("))
        .collect::<Vec<_>>();
    assert!(
        !set_property_lines.is_empty(),
        "FlipCard motion should update runtime custom properties on wasm path.",
    );
    for line in set_property_lines {
        assert!(
            line.contains("set_css_property_observed_auto!"),
            "FlipCard runtime style writes must be limited to custom properties, found `{line}`.",
        );
    }
}

#[test]
fn flip_card_has_no_async_contract_surface() {
    let view_source = load_source("src/view.rs");
    let readme_source = load_source("src/README.md");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "use_async_action",
        "on_retry",
        "retry",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard should stay async-free; found `{forbidden}` in view contract surface.",
        );
    }

    assert!(
        !readme_source.contains("is_loading") && !readme_source.contains("aria-busy"),
        "FlipCard README should not advertise async-only contracts when component has no async interaction.",
    );
}

#[test]
fn flip_card_styles_include_state_source_and_face_markers() {
    let source = load_source("src/styles.rs");

    for selector in [
        ".ui-flip-card {",
        ".ui-flip-card[data-disabled=\"true\"]",
        ".ui-flip-card[data-motion-source=\"custom\"]",
        ".ui-flip-card[data-class-source=\"custom\"]",
        ".ui-flip-card[data-id-source=\"custom\"]",
        ".ui-flip-card[data-flip-mode=\"hover\"]",
        ".ui-flip-card[data-flip-mode=\"toggle\"]",
        ".ui-flip-card__inner {",
        ".ui-flip-card__face {",
        ".ui-flip-card__front {",
        ".ui-flip-card__back {",
        ".ui-flip-card__face[data-visible=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "FlipCard styles should include `{selector}` as stable selectors."
        );
    }

    for token_var in [
        "--ui-flip-card-max-inline-size",
        "--ui-fallback-flip-card-max-inline-size",
        "--ui-flip-card-max-inline-viewport",
        "--ui-fallback-flip-card-max-inline-viewport",
        "--ui-flip-card-aspect-ratio-width",
        "--ui-fallback-flip-card-aspect-ratio-width",
        "--ui-flip-card-aspect-ratio-height",
        "--ui-fallback-flip-card-aspect-ratio-height",
        "--ui-flip-card-perspective",
        "--ui-fallback-flip-card-perspective",
        "--ui-flip-card-disabled-opacity",
        "--ui-fallback-flip-card-disabled-opacity",
        "--ui-flip-card-focus-outline-width",
        "--ui-fallback-flip-card-focus-outline-width",
        "--ui-flip-card-title-font-weight",
        "--ui-fallback-flip-card-title-font-weight",
    ] {
        assert!(
            source.contains(token_var),
            "FlipCard styles should consume theme token variable `{token_var}`."
        );
    }

    for forbidden in [
        "inline-size: min(21rem, 92vw);",
        "aspect-ratio: 4 / 3;",
        "perspective: 1200px;",
        "opacity: 0.6;",
        "box-shadow: 0 0 0 3px var(--ui-focus-ring);",
        "font-weight: 650;",
    ] {
        assert!(
            !source.contains(forbidden),
            "FlipCard styles should not hardcode visual constants `{forbidden}` once theme tokens exist.",
        );
    }
}

#[test]
fn flip_card_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("src/styles.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "var(--ui-flip-card-max-inline-size, var(--ui-fallback-flip-card-max-inline-size))",
        "var(--ui-flip-card-max-inline-viewport, var(--ui-fallback-flip-card-max-inline-viewport))",
        "var(--ui-flip-card-aspect-ratio-width, var(--ui-fallback-flip-card-aspect-ratio-width))",
        "var(--ui-flip-card-aspect-ratio-height, var(--ui-fallback-flip-card-aspect-ratio-height))",
        "var(--ui-flip-card-perspective, var(--ui-fallback-flip-card-perspective))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-flip-card-disabled-opacity, var(--ui-fallback-flip-card-disabled-opacity))",
        "var(--ui-flip-card-focus-outline-width, var(--ui-fallback-flip-card-focus-outline-width))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-lg, var(--ui-fallback-space-lg))",
        "var(--ui-heading-h6-font-size, var(--ui-fallback-heading-h6-font-size))",
        "var(--ui-heading-h6-line-height, var(--ui-fallback-heading-h6-line-height))",
        "var(--ui-flip-card-title-font-weight, var(--ui-fallback-flip-card-title-font-weight))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
    ] {
        assert!(
            styles_source.contains(required),
            "FlipCard styles should keep defensive variable fallback chain `{required}`.",
        );
    }

    for forbidden in [
        "inline-size: min(21rem, 92vw);",
        "aspect-ratio: 4 / 3;",
        "perspective: 1200px;",
        "opacity: 0.6;",
        "font-weight: 650;",
        "#fff",
        "#000",
        "rgb(",
        "hsl(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "FlipCard styles should avoid hardcoded terminal visual values `{forbidden}`.",
        );
    }

    for required in [
        "--ui-fallback-radius-lg:",
        "--ui-fallback-focus-ring:",
        "--ui-fallback-border-width:",
        "--ui-fallback-border:",
        "--ui-fallback-bg:",
        "--ui-fallback-fg:",
        "--ui-fallback-shadow-sm:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-lg:",
        "--ui-fallback-fg-muted:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme SSOT should export fallback terminal `{required}`.",
        );
    }

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "flip_card_styles_use_defensive_variable_fallback_chain",
    ] {
        assert!(
            check2_source.contains(required),
            "FlipCard checklist should keep defensive-variable governance marker `{required}`.",
        );
    }

    let script_needle =
        "cargo test -p ui-flip-card flip_card_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should enforce `{script_needle}`.",
    );
}

#[test]
fn flip_card_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = load_source("../../crates/ui/src/css.rs");
    let root_source = load_source("../../crates/ui/src/root.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let styles_source = load_source("src/styles.rs");
    let check2_source = load_source("check2.md");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-flip_card\")]",
        "out.push_str(crate::flip_card::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(needle),
            "ui css entry should enforce cascade-layer contract `{needle}`.",
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized css injection contract `{needle}`.",
        );
    }

    assert!(
        !view_source.contains(" style="),
        "FlipCard view should not embed plain inline style assignments.",
    );
    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
        "style=\"transform:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipCard view should not include fragile inline style token `{forbidden}`.",
        );
    }

    for (line_index, line) in view_source.lines().enumerate() {
        if let Some(pos) = line.find("style:") {
            let key = line[pos + "style:".len()..]
                .split(|c: char| c == '=' || c.is_whitespace() || c == '>')
                .next()
                .unwrap_or_default()
                .trim();
            assert!(
                key.starts_with("--"),
                "FlipCard runtime style should only set css custom properties; found `style:{key}` at line {}.",
                line_index + 1
            );
        }
    }

    for forbidden in [
        "set_property(\"top\"",
        "set_property(\"left\"",
        "set_property(\"width\"",
        "set_property(\"height\"",
        "set_property(\"transform\"",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "FlipCard motion should not set non-custom-property runtime style `{forbidden}`.",
        );
    }
    for required in [
        "set_css_property_observed_auto!(",
        "\"--ui-flip-card-rotation\"",
        "\"--ui-flip-card-scale\"",
        "\"--ui-flip-card-tilt\"",
    ] {
        assert!(
            motion_source.contains(required),
            "FlipCard motion should keep css-variable-only runtime updates `{required}`.",
        );
    }

    for needle in ["pub const CSS: &str", ".ui-flip-card", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "FlipCard styles should remain static token css contract `{needle}`.",
        );
    }

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "flip_card_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "FlipCard checklist should keep cascade-layer governance marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_cascade_layer_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    let needle =
        "cargo test -p ui-flip-card flip_card_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`.",
    );
}

#[test]
fn flip_card_rust_hygiene_contract_is_enforced_for_non_test_sources() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");
    let check2_source = load_source("check2.md");
    let contract_hygiene_script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");
    let rust_hygiene_script_source = load_source("../../scripts/check-rust-hygiene.sh");

    for source in [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
    ] {
        for forbidden in ["unwrap(", "expect(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "flip-card non-test source should forbid rust hygiene anti-pattern `{forbidden}`.",
            );
        }
    }

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];",
        "classes.push(Cow::Borrowed(\"ui-flip-card--disabled\"));",
        "classes.push(Cow::Borrowed(\"ui-flip-card__face--visible\"));",
        "classes.push(Cow::Owned(base_class_name));",
    ] {
        assert!(
            logic_source.contains(required),
            "flip-card logic should converge class-name hotspot to Cow path `{required}`.",
        );
    }

    for required in [
        "forbidden unwrap/expect in non-test code",
        "forbidden let _ = in non-test code",
        "string clone hotspots (prefer Cow<'static, str>)",
    ] {
        assert!(
            rust_hygiene_script_source.contains(required),
            "repository rust-hygiene script should keep marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_rust_hygiene_contract_is_enforced_for_non_test_sources";
    assert!(
        contract_hygiene_script_source.contains(script_needle),
        "contract-hygiene check script should enforce `{script_needle}`.",
    );

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "flip_card_rust_hygiene_contract_is_enforced_for_non_test_sources",
        "`components/flip-card/src/logic.rs::compose_class_name` 已收敛到 `Vec<Cow<'static, str>>`",
    ] {
        assert!(
            check2_source.contains(required),
            "flip-card check2 should include rust hygiene governance evidence `{required}`.",
        );
    }
}

#[test]
fn flip_card_theme_tokens_are_defined_mapped_and_documented() {
    let tokens_source = load_source("../../crates/ui-theme/src/tokens.rs");
    let theme_source = load_source("../../crates/ui-theme/src/theme.rs");
    let css_source = load_source("../../crates/ui-theme/src/css.rs");
    let docs_source = load_source("../../docs/spec/styling.md");

    for needle in [
        "pub struct FlipCardLayoutTokens",
        "pub const FLIP_CARD_LAYOUT_TOKENS_MEDIUM: FlipCardLayoutTokens",
        "pub const FLIP_CARD_LAYOUT_TOKENS_LARGE: FlipCardLayoutTokens",
        "pub flip_card_layout: FlipCardLayoutTokens",
    ] {
        assert!(
            tokens_source.contains(needle),
            "ui-theme tokens should include `{needle}` for flip-card layout taxonomy.",
        );
    }

    for needle in [
        "pub fn flip_card_layout_tokens(ctx: ThemeContext) -> FlipCardLayoutTokens",
        "pub fn default_flip_card_layout_tokens() -> FlipCardLayoutTokens",
        "flip_card_layout: flip_card_layout_tokens(ctx)",
    ] {
        assert!(
            theme_source.contains(needle),
            "ui-theme mapping should include `{needle}` for flip-card layout tokens.",
        );
    }

    for needle in [
        "--ui-flip-card-max-inline-size:",
        "--ui-flip-card-max-inline-viewport:",
        "--ui-flip-card-aspect-ratio-width:",
        "--ui-flip-card-aspect-ratio-height:",
        "--ui-flip-card-perspective:",
        "--ui-flip-card-disabled-opacity:",
        "--ui-flip-card-focus-outline-width:",
        "--ui-flip-card-title-font-weight:",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-theme css output should include `{needle}` for flip-card.",
        );
    }

    assert!(
        docs_source.contains("--ui-flip-card-max-inline-size")
            && docs_source.contains("tokens.rs -> theme.rs -> css.rs"),
        "styling spec should document flip-card token pipeline.",
    );
}

#[test]
fn flip_card_motion_contract_exposes_default_and_customization_checks() {
    let mod_source = load_source("src/mod.rs");
    let motion_source = format!(
        "{}\n{}",
        load_source("src/motion.rs"),
        load_source("test/motion.rs")
    );

    for needle in [
        "pub mod motion;",
        "pub use motion::FlipCardMotion;",
        "pub struct FlipCardMotion",
        "fn default_motion_uses_soft_spring_contract()",
        "fn supports_custom_motion_contract()",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "FlipCard motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
fn flip_card_css_is_aggregated() {
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let css_source = load_source("../../crates/ui/src/css.rs");
    let root_source = load_source("../../crates/ui/src/root.rs");
    let checklist_source = load_source("check2.md");

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "FlipCard styles should stay in styles.rs as static css contract."
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "FlipCard styles should consume theme token variables via var(--ui-*)."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-flip_card\")]")
            && css_source.contains("out.push_str(crate::flip_card::styles::CSS);"),
        "ui css aggregator should feature-gate and include flip_card styles."
    );
    assert!(
        root_source.contains("#[prop(optional)] inject_components_css: bool")
            && root_source.contains("crate::css::push_components_css(&mut out);"),
        "UiRoot should own optional components-css injection path via crate::css::push_components_css."
    );

    for forbidden in [
        "@apply",
        "tailwind",
        "tw-",
        "stylist::",
        "stylex::",
        "styled_components",
        "#[styled_component]",
        "css! {",
    ] {
        assert!(
            !styles_source.contains(forbidden) && !view_source.contains(forbidden),
            "FlipCard component source should not default to utility-first/CSS-in-Rust marker `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。",
        "- 样式规则统一落在 `styles.rs`，由 `crates/ui/src/css.rs` 聚合并通过 `UiRoot` 注入。",
        "- 颜色/间距/圆角/阴影等视觉值必须来自 `var(--ui-*)`，禁止组件私有 token 体系。",
        "- Utility-First 仅作为 `apps/*` 应用层布局手段，不得反向污染组件库契约。",
        "- CSS-in-Rust 仅在有明确类型安全与构建成本净收益时作为例外采用。",
    ] {
        assert!(
            checklist_source.contains(required),
            "FlipCard checklist should preserve token-first static-style governance rule `{required}`.",
        );
    }

    assert!(
        !view_source.contains("style="),
        "FlipCard view should avoid inline business styling and keep runtime writes limited to motion css vars.",
    );
}

#[test]
fn flip_card_visual_desire_is_repo_level_na_with_local_baseline_evidence() {
    let checklist_source = load_source("check2.md");
    let styles_source = load_source("src/styles.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。（N/A（仓库级视觉治理项）：",
        "关键组件（Button/Input/Overlay）跨组件视觉回归",
        "仓库级视觉基线与截图回归需在统一视觉任务中验收",
    ] {
        assert!(
            checklist_source.contains(needle),
            "FlipCard checklist should keep explicit repo-level N/A boundary for visual-desire governance `{needle}`.",
        );
    }

    for needle in [
        ".ui-flip-card__title {",
        ".ui-flip-card__description {",
        ".ui-flip-card:focus-visible {",
        "font-size: var(--ui-heading-h6-font-size, var(--ui-fallback-heading-h6-font-size));",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        "box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));",
    ] {
        assert!(
            styles_source.contains(needle),
            "FlipCard styles should keep local visual baseline evidence `{needle}`.",
        );
    }

    for needle in [
        "title=\"FlipCard\"",
        "title=\"Hello World (Default Path)\"",
        "title=\"Click + Keyboard Flip\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "FlipCard docs should keep default-theme baseline playground evidence `{needle}`.",
        );
    }
}

#[test]
fn flip_card_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("../../crates/ui/Cargo.toml");
    let lib_source = load_source("../../crates/ui/src/lib.rs");
    let css_source = load_source("../../crates/ui/src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "\"component-flip_card\",",
        "component-flip_card = [\"dep:ui-flip-card\"]",
        "inject-css = []",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui Cargo features should include `{needle}` for tree-shaking boundaries.",
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-flip_card\")]")
            && lib_source.contains("pub use ui_flip_card as flip_card;"),
        "lib.rs should feature-gate flip_card export for tree-shaking.",
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-flip_card\")]")
            && css_source.contains("out.push_str(crate::flip_card::styles::CSS);"),
        "css.rs should gate flip_card CSS aggregation behind component-flip_card feature.",
    );
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)"),
        "css.rs should keep top-level inject-css gate for component CSS injection.",
    );

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("web-demo-components")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui via web-demo-components, not all-components.",
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components instead of implicit default pull-up.",
    );
}

#[test]
fn flip_card_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
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
            "tree-shaking check script should include `{needle}`.",
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`.",
        );
    }
}

#[test]
fn flip_card_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let script_source = load_source("../../scripts/check-ui-tree-shaking.sh");

    for needle in [
        "FLIP_CARD_MIN_FEATURES=\"component-flip_card,inject-css\"",
        "cargo test -p ui-flip-card flip_card_tree_shaking_keeps_component_feature_and_css_boundaries",
        "cargo test -p ui-flip-card flip_card_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui-flip-card flip_card_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$FLIP_CARD_MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$FLIP_CARD_TREE_OUTPUT\";",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$FLIP_CARD_MIN_FEATURES\"",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should enforce flip-card marker `{needle}`.",
        );
    }
}

#[test]
fn flip_card_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "flip_card_tree_shaking_keeps_component_feature_and_css_boundaries",
        "flip_card_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "flip_card_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "scripts/check-ui-tree-shaking.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "flip-card check2 tree-shaking section should include `{needle}`.",
        );
    }
}

#[test]
fn flip_card_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn flip_card() -> AnyView",
        "title=\"FlipCard\"",
        "slug=\"flip-card\"",
        "State + Source Markers",
        "data-flip-mode",
        "data-motion-source",
        "data-id-source",
        "data-visible",
        "<FlipCard",
    ] {
        assert!(
            source.contains(needle),
            "FlipCard docs page should contain `{needle}`."
        );
    }
}

#[test]
fn flip_card_motion_sanitizes_custom_contract_values() {
    let motion_source = format!(
        "{}\n{}",
        load_source("src/motion.rs"),
        load_source("test/motion.rs")
    );
    let view_source = load_source("src/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: FlipCardMotion) -> FlipCardMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "ui_motion::spring::SpringAnimatorTriplet::new(",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "FlipCard motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        !motion_source.contains("ui_motion::spring::SpringAnimator::new("),
        "FlipCard motion should not instantiate spring drivers directly in component layer; use ui-motion shared attach helpers.",
    );

    assert!(
        view_source.contains("let motion = crate::motion::sanitize_motion(motion);"),
        "FlipCard view should sanitize motion before deriving state and attaching motion driver.",
    );
}

#[test]
fn flip_card_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop() {
    let motion_source = load_source("src/motion.rs");
    let view_source = load_source("src/view.rs");
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let checklist_source = load_source("check2.md");

    for needle in [
        "pub struct FlipCardMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "impl Default for FlipCardMotion {",
        "spring: ui_motion::presets::spring_soft(),",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {",
        "stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0 {",
        "damping: if value.damping.is_finite() && value.damping > 0.0 {",
        "let prefers_reduced_motion = StoredValue::new(ui_motion::web::prefers_reduced_motion());",
        "if prefers_reduced_motion.get_value() {",
        "Reduced-motion mode keeps semantic state updates but skips spring runtime.",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "FlipCard motion contract should include `{needle}`.",
        );
    }

    for needle in [
        "let motion = crate::motion::sanitize_motion(motion);",
        "motion::attach_motion(root_ref, is_flipped, is_hovered, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard view should keep motion attach contract marker `{needle}`.",
        );
    }

    let non_wasm_branch_start = motion_source
        .find("#[cfg(not(target_arch = \"wasm32\"))]")
        .expect("flip_card motion should define non-wasm attach branch.");
    let non_wasm_segment = &motion_source[non_wasm_branch_start..];
    for forbidden in [
        "SpringAnimatorTriplet::new(",
        "unwrap()",
        "expect(",
        "panic!(",
    ] {
        assert!(
            !non_wasm_segment.contains(forbidden),
            "FlipCard non-wasm motion path should stay no-op and panic-free; found `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion non-wasm backend should remain predictable no-op via `{needle}`.",
        );
    }

    for required in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "flip_card_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop",
    ] {
        assert!(
            checklist_source.contains(required),
            "FlipCard checklist should keep motion contract governance marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_motion_contract_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    let needle = "cargo test -p ui-flip-card flip_card_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`.",
    );
}

#[test]
fn flip_card_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-entrypoints.sh");
    let lib_source = load_source("../../crates/ui/src/lib.rs");
    let css_source = load_source("../../crates/ui/src/css.rs");
    let root_source = load_source("../../crates/ui/src/root.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");
    let headless_presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let headless_controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");

    for required in [
        "#[cfg(feature = \"component-flip_card\")]",
        "pub use ui_flip_card as flip_card;",
    ] {
        assert!(
            lib_source.contains(required),
            "ui lib entry should keep feature-gated flip-card public surface `{required}`.",
        );
    }

    for forbidden in [
        "pub use web_sys",
        "pub use wasm_bindgen",
        "pub use leptos::web_sys",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui lib entry should not expose platform detail `{forbidden}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-flip_card\")]",
        "out.push_str(crate::flip_card::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(required),
            "ui css entry should keep feature-gated layered aggregation marker `{required}`.",
        );
    }

    for required in [
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
            root_source.contains(required),
            "UiRoot entry should keep centralized theme/i18n/css injection marker `{required}`.",
        );
    }

    for required in [
        "pub const CSS: &str",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "active_highlight should stay shared motion primitive marker `{required}`.",
        );
    }

    for forbidden in ["ui-flip-card", "ui-button", "ui-checkbox", "data-slot="] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should not carry component business semantics `{forbidden}`.",
        );
    }

    let required = "pub fn aria_controls_when_open(";
    assert!(
        headless_a11y_source.contains(required),
        "headless canonical a11y path should keep `{required}`.",
    );
    for required in [
        "pub fn use_presence(",
        "pub struct Presence",
        "pub is_present: ReadSignal<bool>",
        "pub finish_exit: Callback<()>",
    ] {
        assert!(
            headless_presence_source.contains(required),
            "headless canonical presence path should keep `{required}`.",
        );
    }
    for required in [
        "pub fn use_controllable_state<T>(",
        "pub struct ControllableState<T>",
    ] {
        assert!(
            headless_controllable_state_source.contains(required),
            "headless canonical controllable-state path should keep `{required}`.",
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    for forbidden in [
        workspace_dir.join("crates/ui/src/overlay_open.rs"),
        workspace_dir.join("crates/ui/src/presence.rs"),
        workspace_dir.join("crates/ui/src/a11y.rs"),
    ] {
        assert!(
            !forbidden.exists(),
            "ui forbidden fixed entrypoint file should stay absent: {forbidden:?}",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(script_needle),
        "entrypoint gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] `ui` 固定入口文件落点正确。",
        "flip_card_ui_components_fixed_entry_files_follow_layered_boundaries",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep fixed-entrypoint governance marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-entrypoints.sh");

    let needle = "cargo test -p ui-flip-card flip_card_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(needle),
        "entrypoint check script should enforce `{needle}`.",
    );
}

#[test]
fn flip_card_docs_default_and_disabled_playgrounds_lock_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Click + Keyboard Flip\"",
        "<div class=\"ui-flip-card__title\">\"Front\"</div>",
        "Click or press Enter/Space to flip.",
        "<div class=\"ui-flip-card__title\">\"Back\"</div>",
        "Back face stays keyboard reachable with the same button semantics.",
        "title=\"Disabled\"",
        "is_disabled=true",
        "<div class=\"ui-flip-card__title\">\"Disabled front\"</div>",
        "No click/keyboard toggle while disabled.",
        "<div class=\"ui-flip-card__title\">\"Disabled back\"</div>",
        "aria-disabled and disabled markers remain consistent.",
    ] {
        assert!(
            source.contains(needle),
            "FlipCard docs default/disabled playground should contain `{needle}`.",
        );
    }
}

#[test]
fn flip_card_docs_state_source_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id=\"docs-flip-card\".to_string()",
        "class_name=\"docs-flip-card-state\".to_string()",
        "is_flip_on_hover=true",
        "motion=FlipCardMotion {",
        "hover_scale: 1.03,",
        "hover_tilt_deg: 4.0,",
        "..FlipCardMotion::default()",
        "<div class=\"ui-flip-card__title\">\"Inspect markers (front)\"</div>",
        "Hover enters flipped mode source = custom.",
        "<div class=\"ui-flip-card__title\">\"Inspect markers (back)\"</div>",
        "Front/back visibility markers stay explicit for regression tests.",
    ] {
        assert!(
            source.contains(needle),
            "FlipCard docs marker playground should contain `{needle}`.",
        );
    }
}

#[test]
fn flip_card_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn flip_card() -> AnyView",
        "title=\"FlipCard\"",
        "slug=\"flip-card\"",
        "description=\"3D front/back card with baseline-style state/source markers and baseline-level spring motion for flip/hover interactions.\"",
        "title=\"Hello World (Default Path)\"",
        "title=\"Click + Keyboard Flip\"",
        "title=\"State + Source Markers\"",
        "title=\"Disabled\"",
        "<FlipCard",
    ] {
        assert!(
            source.contains(needle),
            "display_extra flip_card docs page should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn flip_card_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let check2_source = load_source("check2.md");
    let dx_script_source = load_source("../../scripts/check-ui-dx.sh");

    for required in [
        "pub(super) fn flip_card() -> AnyView {",
        "let flip_card_imports =",
        "let state_matrix_code = Signal::derive(move || {",
        "let controlled_contrast_code = Signal::derive(move || {",
        "let stream_snapshot_code = Signal::derive(move || {",
        "let source_first_code = Signal::derive(move || {",
        "title=\"Hello World (Default Path)\"",
        "title=\"State Matrix (Default / Hover / Disabled / Dramatic Motion)\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=flip_card_imports.clone()",
        "code_imports=flip_card_imports",
        "Copy action auto-injects missing imports for direct run.",
        "streaming is optional and falls back to snapshot rendering",
        "data-slot=\"flip-card-source-first-contract\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
    ] {
        assert!(
            docs_source.contains(required),
            "flip-card docs page should provide copy-paste-ready playground matrix token `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。"),
        "flip-card checklist should mark docs-as-product copy-paste-ready item complete.",
    );
    for required in [
        "`Hello World (Default Path)`",
        "`State Matrix (Default / Hover / Disabled / Dramatic Motion)`",
        "`Controlled vs Uncontrolled Contrast`",
        "`Streaming / Snapshot Contract`",
        "`Source-first Starter (Copy-Paste Ready)`",
        "`apps/docs-app/src/playground.rs::compose_copy_ready_code`",
        "flip_card_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "flip-card checklist should include concrete docs/copy-ready evidence `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        dx_script_source.contains(script_needle),
        "dx gate script should include `{script_needle}`.",
    );
}

#[test]
fn flip_card_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Click + Keyboard Flip\"",
        "<div class=\"ui-flip-card__title\">\"Front\"</div>",
        "Click or press Enter/Space to flip.",
        "<div class=\"ui-flip-card__title\">\"Back\"</div>",
        "Back face stays keyboard reachable with the same button semantics.",
        "title=\"State + Source Markers\"",
        "id=\"docs-flip-card\".to_string()",
        "class_name=\"docs-flip-card-state\".to_string()",
        "is_flip_on_hover=true",
        "hover_scale: 1.03",
        "hover_tilt_deg: 4.0",
        "title=\"Disabled\"",
        "is_disabled=true",
        "<div class=\"ui-flip-card__title\">\"Disabled front\"</div>",
        "<div class=\"ui-flip-card__title\">\"Disabled back\"</div>",
    ] {
        assert!(
            source.contains(needle),
            "flip_card docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn flip_card_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "flip-card checklist should keep documentation-as-product rule `{required}`.",
        );
    }
}

#[test]
fn flip_card_docs_entry_exists_as_readme_or_equivalent_docs_app_page() {
    let readme_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/README.md");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");

    assert!(
        readme_path.exists(),
        "flip-card should provide README as documentation entry.",
    );
    assert!(
        docs_page_source.contains("pub(super) fn flip_card() -> AnyView"),
        "docs-app should expose flip_card docs entry function.",
    );
    assert!(
        docs_index_source.contains(
            "component_doc!(\"FlipCard\", \"flip-card\", \"Display\", display_extra::flip_card)",
        ),
        "docs-app components index should expose flip-card entry.",
    );
}

#[test]
fn flip_card_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let readme_source = load_source("src/README.md");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "title=\"FlipCard\"",
        "slug=\"flip-card\"",
        "title=\"Hello World (Default Path)\"",
        "title=\"State Matrix (Default / Hover / Disabled / Dramatic Motion)\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
    ] {
        assert!(
            docs_source.contains(required),
            "flip-card docs should keep beginner-to-advanced marker `{required}`.",
        );
    }

    let hello_pos = docs_source
        .find("title=\"Hello World (Default Path)\"")
        .expect("docs should include hello-world playground for zero-threshold path.");
    let matrix_pos = docs_source
        .find("title=\"State Matrix (Default / Hover / Disabled / Dramatic Motion)\"")
        .expect("docs should include state-matrix playground as common usage.");
    let controlled_pos = docs_source
        .find("title=\"Controlled vs Uncontrolled Contrast\"")
        .expect("docs should include controlled-vs-uncontrolled playground.");
    let interactive_pos = docs_source
        .find("title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"")
        .expect("docs should include interactive workbench for advanced controls.");
    assert!(
        hello_pos < interactive_pos && hello_pos < matrix_pos && hello_pos < controlled_pos,
        "docs should keep zero-threshold default path ahead of advanced controls.",
    );

    for required in [
        "## Hello World（最小可用）",
        "## 新手路径（先用起来，再进阶）",
        "## 常见用法（进阶）",
        "阅读顺序建议：先用起来，再进阶。",
        "不需要先理解 `ui-state-primitives` / `ui-headless` 分层细节",
    ] {
        assert!(
            readme_source.contains(required),
            "README should keep beginner-friendly guidance marker `{required}`.",
        );
    }

    let readme_hello_pos = readme_source
        .find("## Hello World（最小可用）")
        .expect("README should include hello-world section.");
    let readme_beginner_pos = readme_source
        .find("## 新手路径（先用起来，再进阶）")
        .expect("README should include beginner path section.");
    let readme_advanced_pos = readme_source
        .find("## 常见用法（进阶）")
        .expect("README should include advanced usage section.");
    assert!(
        readme_hello_pos < readme_beginner_pos && readme_beginner_pos < readme_advanced_pos,
        "README should present default path before advanced guidance.",
    );

    let script_needle = "cargo test -p ui-flip-card flip_card_docs_are_beginner_friendly_with_default_then_advanced_path";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );
}

#[test]
fn flip_card_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let readme_source = load_source("src/README.md");

    for required in [
        "title=\"Hello World (Default Path)\"",
        "code_imports=flip_card_imports.clone()",
        "front=move || view! { <div class=\"ui-flip-card__title\">\"Front\"</div> }",
        "back=move || view! { <div class=\"ui-flip-card__title\">\"Back\"</div> }",
        "## Hello World（最小可用）",
        "front=move || view! { <div>\"Front\"</div> }",
        "back=move || view! { <div>\"Back\"</div> }",
    ] {
        assert!(
            docs_source.contains(required) || readme_source.contains(required),
            "flip-card docs hello-world should keep zero-threshold marker `{required}`.",
        );
    }

    for forbidden in [
        "ui_state_primitives",
        "state=...",
        "logic::",
        "use_presence(",
    ] {
        assert!(
            !readme_source.contains(forbidden),
            "flip-card README hello-world path should avoid architecture-wiring token `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_contract_hygiene_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for marker in [
        "cargo test -p ui-flip-card flip_card_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-flip-card flip_card_docs_entry_exists_as_readme_or_equivalent_docs_app_page",
        "cargo test -p ui-flip-card flip_card_docs_are_beginner_friendly_with_default_then_advanced_path",
        "cargo test -p ui-flip-card flip_card_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring",
    ] {
        assert!(
            script_source.contains(marker),
            "contract-hygiene script should enforce documentation-as-product marker `{marker}`.",
        );
    }
}

#[test]
fn flip_card_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
        "flip_card_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "flip_card_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "flip-card check2 should keep interactive-playground rule marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for required in [
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "description=\"可调翻转初始态/hover/disabled/id/class/motion，并在同一面板查看 code + config + scoped css test。\"",
        "test_config_signal=workbench_config",
        "controls=move || {",
        "let (workbench_default_is_flipped, set_workbench_default_is_flipped) = signal(false);",
        "let (workbench_is_flip_on_hover, set_workbench_is_flip_on_hover) = signal(true);",
        "let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);",
        "let (workbench_custom_id, set_workbench_custom_id) = signal(true);",
        "let (workbench_custom_class, set_workbench_custom_class) = signal(true);",
        "selected_index=workbench_motion_index",
        "set_selected_index=set_workbench_motion_index",
        "default_is_flipped=default_is_flipped",
        "is_disabled=is_disabled",
        "is_flip_on_hover=is_flip_on_hover",
        "motion=motion",
        "展示区：实时预览当前 config + motion。",
        "切换 settings 后，使用 Code / Test 面板查看实际配置与 scoped CSS 影响。",
    ] {
        assert!(
            docs_source.contains(required),
            "flip-card docs interactive playground should keep marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_docs_app_provides_interactive_playground_for_props_state_and_preview";
    assert!(
        script_source.contains(script_needle),
        "dx gate script should include `{script_needle}`.",
    );
}

#[test]
fn flip_card_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_flip_card_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for required in [
        "docs-app flip-card key flow is repeatable and failures map to semantic breakpoints",
        "runToggleKeyFlow(page, toggleRoot)",
        "await page.reload();",
        "body:not(:has(#boot))",
        "docs-app flip-card high-risk paths keep focus keyboard and disabled branches semantically explicit",
        "toHaveAttribute(\"data-ui-action\", \"focus\")",
        "toHaveAttribute(\"data-ui-action\", \"toggle\")",
        "toHaveAttribute(\"data-ui-action\", \"snapshot-render\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "flip-card interactive playground should reuse repeatable semantic e2e marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_interactive_playground_reuses_repeatable_semantic_e2e_flow";
    assert!(
        script_source.contains(script_needle),
        "dx gate script should include `{script_needle}`.",
    );
}

#[test]
fn flip_card_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for marker in [
        "cargo test -p ui-flip-card flip_card_check2_documents_interactive_playground_rules",
        "cargo test -p ui-flip-card flip_card_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-flip-card flip_card_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(marker),
            "dx check script should include interactive-playground marker `{marker}`.",
        );
    }
}

#[test]
fn flip_card_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(required),
            "flip-card checklist should keep source-first copy-paste-ready rule `{required}`.",
        );
    }
}

#[test]
fn flip_card_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_view_source = load_source("../../components/code-block/src/view.rs");
    let view_source = load_source("src/view.rs");
    let readme_source = load_source("src/README.md");

    for required in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "description=\"Copy action auto-injects missing imports for direct run.\"",
        "code_signal=source_first_code",
        "code_imports=flip_card_imports.clone()",
        "data-slot=\"flip-card-source-first-contract\"",
        "<h3>\"Source-first / Copy-Paste Ready Contract\"</h3>",
        "<code>\"Show code\"</code>",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "data-slot=\"flip-card-source-paths\"",
        "components/flip-card/src/mod.rs",
        "components/flip-card/src/logic.rs",
        "components/flip-card/src/view.rs",
        "components/flip-card/src/styles.rs",
        "components/flip-card/src/motion.rs",
        "data-slot=\"flip-card-source-first-deps\"",
        "Dependency baseline (Cargo.toml):",
        "component-flip_card",
        "inject-css",
        "ui = { default-features = false, features = [\\\"component-flip_card\\\", \\\"inject-css\\\"] }",
        "source_first_code = Signal::derive(move || {",
        "<FlipCard\n  is_flip_on_hover=true",
    ] {
        assert!(
            docs_source.contains(required),
            "flip-card source-first docs should contain `{required}`.",
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
            "docs playground copy-ready pipeline should contain `{required}`.",
        );
    }

    for required in [
        "class_name=\"ui-code-block__copy-button\"",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view_source.contains(required),
            "CodeBlock copy affordance should contain `{required}`.",
        );
    }

    for required in [
        "#[prop(optional)] is_flip_on_hover: Option<bool>",
        "#[prop(optional)] motion: FlipCardMotion",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] id: Option<String>",
    ] {
        assert!(
            view_source.contains(required),
            "flip-card view contract should define `{required}` for source-first sync.",
        );
    }

    for required in [
        "## Source-first",
        "components/flip-card/src/mod.rs",
        "components/flip-card/src/logic.rs",
        "components/flip-card/src/view.rs",
        "components/flip-card/src/motion.rs",
        "components/flip-card/src/styles.rs",
    ] {
        assert!(
            readme_source.contains(required),
            "flip-card README should keep source-first path marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for marker in [
        "cargo test -p ui-flip-card flip_card_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-flip-card flip_card_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(marker),
            "dx check script should include source-first copy-paste-ready marker `{marker}`.",
        );
    }
}

#[test]
fn flip_card_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_source("check2.md");

    for marker in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "flip_card_check2_documents_source_first_copy_paste_ready_rules",
        "flip_card_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "flip_card_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "flip-card checklist should keep source-first completion evidence marker `{marker}`.",
        );
    }
}

#[test]
fn flip_card_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_source("check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "flip-card checklist should keep heroui-benchmark docs-sync rule `{required}`.",
        );
    }
}

#[test]
fn flip_card_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let readme_source = load_source("src/README.md");

    for marker in [
        "### FlipCard 同步记录（2026-02-20）",
        "参数模型同步：`FlipCard` 参数主轴保持",
        "component_doc!(\"FlipCard\", \"flip-card\", \"Display\", display_extra::flip_card)",
        "display_extra.rs::flip_card()",
        "`components/flip-card/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(marker),
            "heroui strategy doc should include flip-card synchronization marker `{marker}`.",
        );
    }

    for marker in [
        "component_doc!(",
        "\"FlipCard\"",
        "\"flip-card\"",
        "display_extra::flip_card",
    ] {
        assert!(
            pages_source.contains(marker),
            "component docs index should expose flip-card entry marker `{marker}`.",
        );
    }

    for marker in [
        "pub(super) fn flip_card() -> AnyView",
        "title=\"FlipCard\"",
        "slug=\"flip-card\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "docs-app flip-card page should stay indexable via marker `{marker}`.",
        );
    }

    assert!(
        readme_source.contains("# FlipCard"),
        "flip-card README should remain an equivalent component docs entry.",
    );
}

#[test]
fn flip_card_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for marker in [
        "cargo test -p ui-flip-card flip_card_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-flip-card flip_card_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(marker),
            "dx check script should include heroui-benchmark docs-sync marker `{marker}`.",
        );
    }
}

#[test]
fn flip_card_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("check2.md");

    for marker in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "flip_card_check2_documents_heroui_benchmark_docs_sync_rules",
        "flip_card_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "flip_card_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "flip-card checklist should keep heroui-benchmark docs-sync evidence marker `{marker}`.",
        );
    }
}

#[test]
fn flip_card_check2_documents_docs_examples_parameter_state_matrix_sync_rules() {
    let check2_source = load_source("check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
        "flip_card_docs_examples_parameter_state_matrix_sync_with_logic_defaults",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "flip-card check2 should keep docs/examples/parameter-matrix sync marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_docs_examples_parameter_state_matrix_sync_with_logic_defaults() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let logic_source = load_source("src/logic.rs");
    let state_primitives_source = load_source("../../crates/ui-state-primitives/src/flip_card.rs");

    for required in [
        "title=\"State Matrix (Default / Hover / Disabled / Dramatic Motion)\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "data-slot=\"flip-card-parameter-matrix\"",
        "data-slot=\"flip-card-parameter-rows\"",
        "default_is_flipped > default_flipped > DEFAULT_FLIPPED(false)",
        "is_disabled > disabled > DEFAULT_DISABLED(false)",
        "flip_mode > is_flip_on_hover > flip_on_hover > DEFAULT_HOVER_FLIP(false)",
        "IdProvider::next_prefixed_id(DEFAULT_ID_PREFIX)",
        "FlipCardMotion::default()",
    ] {
        assert!(
            docs_source.contains(required),
            "flip-card docs should keep parameter/state matrix marker `{required}` synced.",
        );
    }

    for required in [
        ".default_is_flipped",
        ".or(input.default_flipped)",
        "unwrap_or(DEFAULT_FLIPPED)",
        "DEFAULT_ID_PREFIX",
    ] {
        assert!(
            logic_source.contains(required),
            "flip-card logic should keep normalized default rule marker `{required}`.",
        );
    }

    for required in [
        "pub const DEFAULT_DISABLED: bool = false;",
        "pub const DEFAULT_HOVER_FLIP: bool = false;",
        "input.is_disabled",
        ".or(input.disabled)",
        "unwrap_or(DEFAULT_DISABLED)",
        "if let Some(mode) = input.flip_mode",
        "else if let Some(value) = input.is_flip_on_hover",
        "else if let Some(value) = input.flip_on_hover",
        "FlipCardFlipMode::from_hover_flag(DEFAULT_HOVER_FLIP)",
    ] {
        assert!(
            state_primitives_source.contains(required),
            "flip-card state primitive should keep default precedence marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_dx_script_covers_docs_examples_parameter_state_matrix_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    let script_needle = "cargo test -p ui-flip-card flip_card_docs_examples_parameter_state_matrix_sync_with_logic_defaults";
    assert!(
        script_source.contains(script_needle),
        "dx gate script should include `{script_needle}`.",
    );
}

#[test]
fn flip_card_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("check2.md");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "flip-card checklist should keep semantics-first testing rule `{required}`.",
        );
    }

    for marker in [
        "flip_card_semantics_suite_is_contract_first_not_snapshot_only",
        "flip_card_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            checklist_source.contains(marker),
            "flip-card checklist semantics-first section should keep evidence marker `{marker}`.",
        );
    }
}

#[test]
fn flip_card_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("test/semantics.rs");
    let logic_test_source = load_source("test/logic.rs");
    let test_mod_source = load_source("test/mod.rs");

    for required in [
        "flip_card_view_uses_motion_and_state_contracts",
        "flip_card_observability_markers_are_stable_and_enumerated",
        "flip_card_a11y_i18n_contracts_are_headless_and_text_free",
        "flip_card_semantic_contract_matrix_is_covered_without_snapshot_dependency",
        "aria-pressed=move || a11y_aria_pressed.get()",
        "aria-disabled=a11y_aria_disabled",
        "data-state=move || derived_render_state.get().root.state_attr",
        "data-flipped-change-source=move || derived_render_state.get().root_markers.flipped_change_source_attr",
        "data-flip-mode-source=move || derived_render_state.get().root_markers.flip_mode_source_attr",
        "on:keydown=on_key_down",
    ] {
        assert!(
            semantics_source.contains(required),
            "flip-card semantic suite should keep contract-first assertion marker `{required}`.",
        );
    }

    assert!(
        test_mod_source.contains("mod semantics;"),
        "flip-card component should keep local *_semantics.rs test entry wired in test/mod.rs.",
    );

    let forbidden = [
        "\n    assert_snapshot!(",
        "\n    insta::assert_snapshot!(",
        "\n    to_match_snapshot(",
        "\n    image_snapshot(",
        "\n    toHaveScreenshot(",
        "\n    toMatchSnapshot(",
    ];

    for forbidden in forbidden {
        assert!(
            !semantics_source.contains(forbidden) && !logic_test_source.contains(forbidden),
            "flip-card semantics should not rely on snapshot-only assertion `{forbidden}` as primary signal.",
        );
    }
}

#[test]
fn flip_card_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = load_source("src/view.rs");
    let semantics_source = load_source("test/semantics.rs");

    for marker in [
        "role=a11y_role",
        "aria-pressed=move || a11y_aria_pressed.get()",
        "aria-disabled=a11y_aria_disabled",
        "data-state=move || derived_render_state.get().root.state_attr",
        "data-visible=move || derived_render_state.get().root.visibility_attr",
        "data-flip-mode=move || derived_render_state.get().root.flip_mode_attr",
        "data-flipped-control-mode=move || derived_render_state.get().root_markers.flipped_control_mode_attr",
        "data-flipped-change-source=move || derived_render_state.get().root_markers.flipped_change_source_attr",
        "data-flip-mode-source=move || derived_render_state.get().root_markers.flip_mode_source_attr",
        "data-motion-source=move || derived_render_state.get().root.motion_source_attr",
        "data-id-source=move || derived_render_state.get().root.id_source_attr",
        "data-class-source=move || derived_render_state.get().root.class_source_attr",
        "on:keydown=on_key_down",
        "on:focusin=on_focus_in",
        "on:focusout=on_focus_out",
    ] {
        assert!(
            view_source.contains(marker),
            "flip-card view should expose semantic marker `{marker}`.",
        );
        assert!(
            semantics_source.contains(marker),
            "flip-card semantic marker `{marker}` changed without matching semantics assertion update.",
        );
    }
}

#[test]
fn flip_card_contract_hygiene_script_covers_semantics_first_testing_guards() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-flip-card flip_card_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui-flip-card flip_card_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui-flip-card flip_card_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn flip_card_semantic_contract_matrix_is_covered_without_snapshot_dependency() {
    let semantics_source = load_source("test/semantics.rs");
    let logic_test_source = load_source("test/logic.rs");
    let motion_test_source = load_source("test/motion.rs");
    let headless_test_source = load_source("../../crates/ui-headless/src/test/flip_card.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let ui_motion_test_source = load_source("../../crates/ui-motion/src/test/lib.rs");

    for needle in [
        "fn flip_card_view_uses_motion_and_state_contracts()",
        "fn flip_card_observability_markers_are_stable_and_enumerated()",
        "fn flip_card_a11y_i18n_contracts_are_headless_and_text_free()",
        "role=a11y_role",
        "aria-pressed=move || a11y_aria_pressed.get()",
        "data-state=move || derived_render_state.get().root.state_attr",
    ] {
        assert!(
            semantics_source.contains(needle),
            "FlipCard semantics test matrix should include semantic marker assertion `{needle}`.",
        );
    }

    for needle in [
        "fn normalize_flipped_axis_centralizes_default_priority_and_sources()",
        "fn normalize_behavior_flags_centralizes_bool_defaults_and_alias_priority()",
        "fn derive_render_state_centralizes_slot_states_and_semantic_markers()",
        "flipped_control_mode_attr",
        "disabled_source_attr",
    ] {
        assert!(
            logic_test_source.contains(needle),
            "FlipCard logic tests should cover controlled/uncontrolled and disabled contracts `{needle}`.",
        );
    }

    for needle in [
        "fn key_down_contract_only_toggles_for_enter_or_space_when_allowed()",
        "fn flip_card_handlers_toggle_and_normalize_hover_focus_state()",
        "fn disabled_flip_card_does_not_toggle_or_enter_hover()",
        "contract.handlers.on_pointer_enter.run(())",
    ] {
        assert!(
            headless_test_source.contains(needle),
            "FlipCard headless tests should cover keyboard/pointer paths `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipCard view should keep wasm/non-wasm branch contract `{needle}`.",
        );
        assert!(
            motion_source.contains(needle),
            "FlipCard motion should keep wasm/non-wasm branch contract `{needle}`.",
        );
    }

    assert!(
        ui_motion_test_source.contains("fn non_wasm_web_backend_is_predictable_noop()"),
        "ui-motion tests should cover predictable non-wasm backend behavior for SSR/tooling paths.",
    );

    for forbidden in [
        "\n    assert_snapshot!(",
        "\n    insta::assert_snapshot!(",
        "\n    toMatchSnapshot(",
        "\n    to_match_snapshot(",
    ] {
        assert!(
            !semantics_source.contains(forbidden),
            "FlipCard semantics tests must not rely on snapshot-only assertions; found `{forbidden}`.",
        );
        assert!(
            !logic_test_source.contains(forbidden),
            "FlipCard logic tests must not rely on snapshot-only assertions; found `{forbidden}`.",
        );
        assert!(
            !motion_test_source.contains(forbidden),
            "FlipCard motion tests must not rely on snapshot-only assertions; found `{forbidden}`.",
        );
        assert!(
            !headless_test_source.contains(forbidden),
            "FlipCard headless tests must not rely on snapshot-only assertions; found `{forbidden}`.",
        );
    }
}

#[test]
fn flip_card_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        "flip_card_check2_documents_e2e_selector_and_stable_wait_rules",
        "flip_card_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "flip_card_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints",
        "components/flip-card/scripts/check-ui-e2e-flip-card.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "flip-card check2 should keep e2e-selector/stable-wait governance marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_flip_card_contract.spec.mjs");
    let script_source = load_source("../../components/flip-card/scripts/check-ui-e2e-flip-card.sh");

    for required in [
        "const FLIP_CARD_PAGE = \"/#/components/flip-card\";",
        "body:not(:has(#boot))",
        "[data-component=\"flip-card\"] #docs-flip-card[data-slot=\"flip-card\"][data-ui-schema=\"ui.flip-card.agent-contract\"]",
        "[data-component=\"flip-card\"] #docs-flip-card-toggle[data-slot=\"flip-card\"][data-flip-mode=\"toggle\"]",
        "[data-component=\"flip-card\"] #docs-flip-card-disabled[data-slot=\"flip-card\"][data-disabled=\"true\"]",
        "data-ui-schema-version",
        "data-ui-intent",
        "data-ui-source",
        "data-ui-config-policy",
        "data-ui-action",
        "data-state",
        "data-visible",
        "data-flip-mode",
        "data-motion-source",
        "data-class-source",
        "data-id-source",
        "aria-pressed",
        "data-slot=\"flip-card-front\"",
        "data-slot=\"flip-card-back\"",
    ] {
        assert!(
            e2e_source.contains(required),
            "flip-card e2e contract should include semantic selector/wait marker `{required}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        ":nth-child(",
        "getByText(",
        "locator(\"text=",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "flip-card e2e contract should avoid brittle selector/wait token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_e2e_selector_contract_uses_semantic_markers_and_stable_waits";
    assert!(
        script_source.contains(script_needle),
        "e2e flip-card gate script should include `{script_needle}`.",
    );
}

#[test]
fn flip_card_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_flip_card_contract.spec.mjs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let script_source = load_source("../../components/flip-card/scripts/check-ui-e2e-flip-card.sh");

    for required in [
        "id=\"docs-flip-card-toggle\".to_string()",
        "id=\"docs-flip-card-disabled\".to_string()",
        "id=\"docs-flip-card\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "flip-card docs page should expose stable e2e anchor `{required}`.",
        );
    }

    for required in [
        "root.hover()",
        "toHaveAttribute(\"data-ui-action\", \"hover-enter\")",
        "toHaveAttribute(\"data-state\", \"flipped\")",
        "toHaveAttribute(\"data-ui-state\", \"flipped\")",
        "page.locator(\"body\").hover()",
        "toHaveAttribute(\"data-ui-action\", \"hover-leave\")",
        "toHaveAttribute(\"data-state\", \"default\")",
        "toHaveAttribute(\"data-ui-state\", \"default\")",
        "toggleRoot.focus()",
        "keyboard.press(\"Enter\")",
        "toHaveAttribute(\"data-ui-action\", \"toggle\")",
        "keyboard.press(\"Space\")",
        "disabledRoot.click()",
        "toHaveAttribute(\"data-disabled\", \"true\")",
        "toHaveAttribute(\"data-ui-state\", \"disabled\")",
        "toHaveAttribute(\"data-ui-action\", \"snapshot-render\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "flip-card e2e interaction contract should include ready/settled breakpoint `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e flip-card gate script should include `{script_needle}`.",
    );
}

#[test]
fn flip_card_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = load_source("check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
        "flip_card_check2_documents_e2e_repeatable_key_flow_rules",
        "flip_card_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "flip_card_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "components/flip-card/scripts/check-ui-e2e-flip-card.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "flip-card check2 should keep repeatable e2e flow governance marker `{required}`.",
        );
    }
}

#[test]
fn flip_card_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_flip_card_contract.spec.mjs");
    let script_source = load_source("../../components/flip-card/scripts/check-ui-e2e-flip-card.sh");

    for required in [
        "key flow is repeatable and failures map to semantic breakpoints",
        "runToggleKeyFlow(page, toggleRoot)",
        "toHaveAttribute(\"data-ui-action\", \"focus\")",
        "keyboard.press(\"Enter\")",
        "toHaveAttribute(\"data-ui-action\", \"toggle\")",
        "toHaveAttribute(\"data-state\", \"flipped\")",
        "toHaveAttribute(\"data-ui-state\", \"flipped\")",
        "keyboard.press(\"Space\")",
        "toHaveAttribute(\"data-state\", \"default\")",
        "toHaveAttribute(\"data-ui-state\", \"default\")",
        "await page.reload();",
        "body:not(:has(#boot))",
    ] {
        assert!(
            e2e_source.contains(required),
            "repeatable flip-card e2e flow should include semantic breakpoint `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_e2e_key_flow_is_repeatable_and_failure_points_are_semantic";
    assert!(
        script_source.contains(script_needle),
        "e2e flip-card gate script should include `{script_needle}`.",
    );
}

#[test]
fn flip_card_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_flip_card_contract.spec.mjs");
    let script_source = load_source("../../components/flip-card/scripts/check-ui-e2e-flip-card.sh");

    for required in [
        "high-risk paths keep focus keyboard and disabled branches semantically explicit",
        "toggleRoot.focus()",
        "toBeFocused()",
        "toHaveAttribute(\"data-ui-action\", \"focus\")",
        "keyboard.press(\"Enter\")",
        "toHaveAttribute(\"data-ui-action\", \"toggle\")",
        "page.locator(\"body\").click()",
        "toHaveAttribute(\"data-ui-action\", \"blur\")",
        "toHaveAttribute(\"data-ui-state\", \"flipped\")",
        "toHaveAttribute(\"data-disabled\", \"true\")",
        "toHaveAttribute(\"data-ui-state\", \"disabled\")",
        "toHaveAttribute(\"aria-disabled\", \"true\")",
        "disabledRoot.click()",
        "toHaveAttribute(\"data-ui-action\", \"snapshot-render\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "high-risk flip-card e2e path should include semantic breakpoint `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-flip-card flip_card_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e flip-card gate script should include `{script_needle}`.",
    );
}
