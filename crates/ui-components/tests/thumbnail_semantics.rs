use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(rel_path)
        .exists()
}

#[test]
fn thumbnail_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/thumbnail/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Thumbnail internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn thumbnail_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/thumbnail/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Thumbnail;"),
        "thumbnail module should export `Thumbnail`."
    );
    assert!(
        crate_source.contains("pub use thumbnail::{Thumbnail, ThumbnailMotion, ThumbnailSize};"),
        "crate root should re-export Thumbnail contract."
    );
}

#[test]
fn thumbnail_module_boundary_stays_minimal_and_stable() {
    let source = load_source("src/thumbnail/mod.rs");

    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use logic::ThumbnailSize;",
        "pub use motion::ThumbnailMotion;",
        "pub use view::Thumbnail;",
    ] {
        assert!(
            source.contains(needle),
            "thumbnail module boundary should include `{needle}`."
        );
    }
}

#[test]
fn thumbnail_keeps_spec_out_and_docs_in_check2() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/thumbnail/spec.rs");
    let check2_path = manifest_dir.join("src/thumbnail/check2.md");
    let mod_source = load_source("src/thumbnail/mod.rs");

    assert!(
        !spec_path.exists(),
        "Thumbnail is a simple component; do not add `src/thumbnail/spec.rs` unless a stable external schema contract exists."
    );
    assert!(
        check2_path.exists(),
        "Thumbnail docs/contract notes should stay in `src/thumbnail/check2.md`."
    );
    for forbidden in ["mod spec;", "pub mod spec;"] {
        assert!(
            !mod_source.contains(forbidden),
            "Thumbnail module should not wire spec module via `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_component_directory_has_standard_file_layout() {
    for required in [
        "src/thumbnail/mod.rs",
        "src/thumbnail/logic.rs",
        "src/thumbnail/styles.rs",
        "src/thumbnail/view.rs",
        "src/thumbnail/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "thumbnail component directory should include required file `{required}`."
        );
    }

    assert!(
        !path_exists("src/thumbnail/spec.rs"),
        "thumbnail is a simple component; `spec.rs` should not exist unless schema-level contract is introduced."
    );
    assert!(
        !path_exists("src/thumbnail/render.rs"),
        "thumbnail should keep rendering in `view.rs`; `render.rs` drift is not allowed."
    );
}

#[test]
fn thumbnail_mod_rs_keeps_minimal_stable_exports() {
    let source = load_source("src/thumbnail/mod.rs");

    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use logic::ThumbnailSize;",
        "pub use motion::ThumbnailMotion;",
        "pub use view::Thumbnail;",
    ] {
        assert!(
            source.contains(needle),
            "thumbnail/mod.rs should include stable export marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "web_sys",
        "leptos::web_sys",
    ] {
        assert!(
            !source.contains(forbidden),
            "thumbnail/mod.rs should not leak internal/platform marker `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_component_file_responsibilities_remain_scoped() {
    let logic_source = load_source("src/thumbnail/logic.rs");
    let styles_source = load_source("src/thumbnail/styles.rs");
    let view_source = load_source("src/thumbnail/view.rs");
    let motion_source = load_source("src/thumbnail/motion.rs");

    for forbidden in [
        "view!",
        "on:pointer",
        "on:keydown",
        "aria-",
        "data-slot",
        "NodeRef<",
        "web_sys",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "thumbnail/logic.rs should stay normalization-only; found `{forbidden}`."
        );
    }

    for required in ["var(--ui-", "pub const CSS: &str ="] {
        assert!(
            styles_source.contains(required),
            "thumbnail/styles.rs should keep token-first CSS marker `{required}`."
        );
    }

    for forbidden in ["#[component]", "use leptos::", "on:click=", "view!"] {
        assert!(
            !styles_source.contains(forbidden),
            "thumbnail/styles.rs should stay static style contract; found `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn Thumbnail(",
        "locale_attrs(",
        "render_thumbnail_content(",
        "thumbnail_motion::attach_motion(",
    ] {
        assert!(
            view_source.contains(required),
            "thumbnail/view.rs should keep rendering + headless mount marker `{required}`."
        );
    }

    for forbidden in ["resolve_state(", "ui_state_primitives::thumbnail::"] {
        assert!(
            !view_source.contains(forbidden),
            "thumbnail/view.rs should not bypass logic layer; found `{forbidden}`."
        );
    }

    for required in [
        "pub struct ThumbnailMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(required),
            "thumbnail/motion.rs should keep motion-contract marker `{required}`."
        );
    }

    for forbidden in ["aria-", "data-slot", "on:pointer", "on:click"] {
        assert!(
            !motion_source.contains(forbidden),
            "thumbnail/motion.rs should not carry view semantics; found `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_attaches_motion_driver() {
    let source = load_source("src/thumbnail/view.rs");

    assert!(
        source.contains("attach_motion"),
        "Thumbnail should attach its motion driver for focus/selection feedback."
    );
}

#[test]
fn thumbnail_mounts_locale_attrs_from_headless_a11y_helpers() {
    let source = load_source("src/thumbnail/view.rs");

    for needle in [
        "use ui_headless::{",
        "locale_attrs",
        "A11yDirection",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "let locale = locale_attrs(logic::normalize_lang(lang), dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
    ] {
        assert!(
            source.contains(needle),
            "Thumbnail should expose locale plumbing via shared headless a11y utility; missing `{needle}`."
        );
    }
}

#[test]
fn thumbnail_a11y_contract_is_non_interactive_by_design() {
    let source = load_source("src/thumbnail/view.rs");

    for forbidden in [
        "on:click=",
        "on:keydown=",
        "tabindex=",
        "use_button(",
        "use_focus_ring(",
    ] {
        assert!(
            !source.contains(forbidden),
            "Thumbnail is a non-interactive display primitive; `{forbidden}` should not appear unless headless interaction contract is introduced."
        );
    }
}

#[test]
fn thumbnail_emits_motion_marker_attributes() {
    let source = load_source("src/thumbnail/view.rs");

    for attr in [
        "data-motion-source=motion_source.as_attr()",
        "data-custom-motion=motion_source.custom_motion_attr()",
    ] {
        assert!(
            source.contains(attr),
            "Thumbnail should expose `{attr}` for stable motion-source inspection."
        );
    }
}

#[test]
fn thumbnail_emits_state_source_marker_attributes() {
    let source = load_source("src/thumbnail/view.rs");

    for attr in [
        "data-cover-source=cover_source.as_attr()",
        "data-layer-source=layer_source.as_attr()",
        "data-selected-source=selected_source.as_attr()",
        "data-focused-source=focused_source.as_attr()",
        "data-background-source=background_source.as_attr()",
        "data-class-source=class_name_source.as_attr()",
    ] {
        assert!(
            source.contains(attr),
            "Thumbnail should expose `{attr}` for explicit source observability."
        );
    }
}

#[test]
fn thumbnail_source_marker_values_are_closed_sets() {
    let source = load_source("src/thumbnail/logic.rs");

    for needle in [
        "pub enum ThumbnailBooleanSource",
        "Self::Default => \"default\"",
        "Self::Prop => \"prop\"",
        "pub enum ThumbnailValueSource",
        "Self::Custom => \"custom\"",
    ] {
        assert!(
            source.contains(needle),
            "Thumbnail source markers should use closed enum-backed values; missing `{needle}`."
        );
    }
}

#[test]
fn thumbnail_discrete_axes_are_type_constrained_by_enums() {
    let primitive_source = load_source("../ui-state-primitives/src/thumbnail.rs");
    let logic_source = load_source("src/thumbnail/logic.rs");
    let view_source = load_source("src/thumbnail/view.rs");

    for needle in [
        "pub enum ThumbnailSize",
        "pub enum ThumbnailDataState",
        "pub struct ThumbnailStateInput",
        "pub size: ThumbnailSize,",
        "pub data_state: ThumbnailDataState,",
        "pub enum ThumbnailMotionSource",
        "pub enum ThumbnailBooleanSource",
        "pub enum ThumbnailValueSource",
        "pub struct ThumbnailViewStateInput",
        "pub size: ThumbnailSize,",
        "pub motion_source: ThumbnailMotionSource,",
        "#[prop(optional)] size: ThumbnailSize,",
    ] {
        assert!(
            primitive_source.contains(needle)
                || logic_source.contains(needle)
                || view_source.contains(needle),
            "Thumbnail discrete state axes should be type-constrained via `{needle}`."
        );
    }

    for forbidden in [
        "pub size: String",
        "data-state=format!(",
        "data-size=format!(",
    ] {
        assert!(
            !primitive_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Thumbnail should avoid stringly-typed discrete state contracts `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_invalid_input_normalization_and_failure_location_are_tested() {
    let primitive_source = load_source("../ui-state-primitives/src/thumbnail.rs");
    let logic_source = load_source("src/thumbnail/logic.rs");
    let view_source = load_source("src/thumbnail/view.rs");

    for needle in [
        "pub fn sanitize_background(value: Option<String>) -> Option<String>",
        "background: sanitize_background(background),",
        "ThumbnailBooleanSource::resolve(input.cover)",
        "ThumbnailBooleanSource::resolve(input.layer)",
        "ThumbnailBooleanSource::resolve(input.selected)",
        "ThumbnailBooleanSource::resolve(input.focused)",
        "fn sanitize_background_rejects_invalid_content()",
        "fn normalize_input_filters_background_and_class_name()",
        "fn resolve_view_state_centralizes_defaults_and_markers()",
    ] {
        assert!(
            primitive_source.contains(needle) || logic_source.contains(needle),
            "Thumbnail should keep normalization + direct regression failure point `{needle}`."
        );
    }

    for needle in [
        "data-size=state.size_attr",
        "data-state=state.data_state.as_attr()",
        "data-cover-source=cover_source.as_attr()",
        "data-layer-source=layer_source.as_attr()",
        "data-selected-source=selected_source.as_attr()",
        "data-focused-source=focused_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "Thumbnail should expose machine-readable semantic marker `{needle}`."
        );
    }
}

#[test]
fn thumbnail_semantic_contract_exposes_state_and_source_markers() {
    let source = load_source("src/thumbnail/view.rs");

    for marker in [
        "data-state=state.data_state.as_attr()",
        "data-motion-source=motion_source.as_attr()",
        "data-cover-source=cover_source.as_attr()",
        "data-layer-source=layer_source.as_attr()",
        "data-selected-source=selected_source.as_attr()",
        "data-focused-source=focused_source.as_attr()",
        "data-background-source=background_source.as_attr()",
        "data-class-source=class_name_source.as_attr()",
    ] {
        assert!(
            source.contains(marker),
            "Thumbnail semantic contract should expose `{marker}` for machine-verifiable state coverage."
        );
    }
}

#[test]
fn thumbnail_agent_contract_is_schema_typed_and_traceable_without_dom_guessing() {
    let logic_source = load_source("src/thumbnail/logic.rs");
    let view_source = load_source("src/thumbnail/view.rs");

    for needle in [
        "pub enum ThumbnailAgentSchemaVersion",
        "pub enum ThumbnailAgentIntent",
        "pub enum ThumbnailAgentAction",
        "pub enum ThumbnailAgentStateAxis",
        "pub enum ThumbnailAgentSource",
        "pub struct ThumbnailAgentContract",
        "pub fn resolve_agent_state_axis(state: ThumbnailState) -> ThumbnailAgentStateAxis",
        "pub fn resolve_agent_source(state: &ThumbnailViewState) -> ThumbnailAgentSource",
        "pub fn resolve_agent_contract(state: &ThumbnailViewState) -> ThumbnailAgentContract",
        "schema_name: \"ui.thumbnail.agent-contract\"",
        "intent: ThumbnailAgentIntent::MediaPreview",
        "action: ThumbnailAgentAction::Inspect",
    ] {
        assert!(
            logic_source.contains(needle),
            "Thumbnail logic should keep typed agent contract marker `{needle}`."
        );
    }

    for needle in [
        "let agent_contract = logic::resolve_agent_contract(&view_state);",
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-schema-version=agent_contract.schema_version.as_str()",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-action=agent_contract.action.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-source=agent_contract.source.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "Thumbnail view should mount agent contract marker `{needle}`."
        );
    }

    for forbidden in [
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
        "data-ui-schema=format!(",
        "inner_html",
        "<script",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Thumbnail agent contract pipeline should stay whitelist-safe and avoid `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_stays_snapshot_only_and_does_not_mount_stream_contract_fields() {
    let view_source = load_source("src/thumbnail/view.rs");
    let logic_source = load_source("src/thumbnail/logic.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");

    for forbidden in [
        "use_ai_space_state",
        "AiSpace",
        "AiRenderMode",
        "AiOutputStatus",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-ui-output-status",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "Thumbnail is snapshot-only; forbidden streaming marker `{forbidden}` should not appear."
        );
    }
}

#[test]
fn thumbnail_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("src/thumbnail/check2.md");

    for needle in [
        "- [ ] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "- [ ] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "- [ ] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
    ] {
        assert!(
            check2_source.contains(needle),
            "thumbnail/check2.md should pin streaming contract marker `{needle}`."
        );
    }
}

#[test]
fn thumbnail_streaming_policy_is_optional_and_delegated_to_upper_layer() {
    let view_source = load_source("src/thumbnail/view.rs");
    let logic_source = load_source("src/thumbnail/logic.rs");
    let motion_source = load_source("src/thumbnail/motion.rs");

    for needle in [
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "data-state=state.data_state.as_attr()",
        "data-size=state.size_attr",
        "data-cover-source=cover_source.as_attr()",
        "data-layer-source=layer_source.as_attr()",
        "data-selected-source=selected_source.as_attr()",
        "data-focused-source=focused_source.as_attr()",
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-action=agent_contract.action.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-source=agent_contract.source.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "Thumbnail should keep continuous semantic readability marker `{needle}`."
        );
    }

    for forbidden in [
        "AiSpaceState",
        "use_ai_space_state",
        "data-ui-stream-mode",
        "data-ui-output-status",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "retry",
        "reconnect",
        "disconnect",
        "token_delta",
        "partial_chunk",
        "incremental",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Thumbnail should delegate streaming transport/recovery policy upward and avoid `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_check2_documents_semantic_test_priority_contract() {
    let check2_source = load_source("src/thumbnail/check2.md");

    for needle in [
        "- [ ] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            check2_source.contains(needle),
            "thumbnail/check2.md should pin semantic-test-priority marker `{needle}`."
        );
    }
}

#[test]
fn thumbnail_semantics_suite_is_contract_first_not_visual_snapshot_only() {
    let semantics_source = load_source("tests/thumbnail_semantics.rs");

    for required in [
        "thumbnail_semantic_contract_exposes_state_and_source_markers",
        "thumbnail_agent_contract_is_schema_typed_and_traceable_without_dom_guessing",
        "thumbnail_view_consumes_logic_outputs_without_rebuilding_state_machine",
        "thumbnail_streaming_policy_is_optional_and_delegated_to_upper_layer",
        "data-state=state.data_state.as_attr()",
        "data-cover-source=cover_source.as_attr()",
        "data-layer-source=layer_source.as_attr()",
        "data-selected-source=selected_source.as_attr()",
        "data-focused-source=focused_source.as_attr()",
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-action=agent_contract.action.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-source=agent_contract.source.as_str()",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
    ] {
        assert!(
            semantics_source.contains(required),
            "Thumbnail semantic suite should assert contract marker `{required}`."
        );
    }

    let forbidden_terms = [
        ["assert_", "snapshot!"].concat(),
        ["insta", "::"].concat(),
        ["snapbox", "::"].concat(),
        ["to_match_", "snapshot"].concat(),
    ];

    for forbidden in forbidden_terms {
        assert!(
            !semantics_source.contains(forbidden.as_str()),
            "Thumbnail semantic suite should not rely on visual snapshot assertion `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_snapshot_mode_consumes_complete_input_and_renders_in_one_pass() {
    let view_source = load_source("src/thumbnail/view.rs");
    let logic_source = load_source("src/thumbnail/logic.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");

    for needle in [
        "let view_state = logic::resolve_view_state(",
        "logic::ThumbnailViewStateInput {",
        "logic::normalize_input(background, class_name),",
        "let state = view_state.state;",
        "let class = view_state.class_name;",
        "let inline_style = StoredValue::new(Some(view_state.inline_css_vars));",
        "let content = render_thumbnail_content(children);",
    ] {
        assert!(
            view_source.contains(needle),
            "Thumbnail snapshot path should consume complete resolved input via `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_input(",
        "pub fn resolve_view_state(",
        "ThumbnailViewState {",
        "class_name: compose_class_name(normalized.class_name, state),",
        "inline_css_vars: compose_inline_style(normalized.background.as_deref()).unwrap_or_default(),",
    ] {
        assert!(
            logic_source.contains(needle),
            "Thumbnail logic should deterministically resolve full snapshot input via `{needle}`."
        );
    }

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Thumbnail>",
        "<Playground title=\"Cover + Background + Layer + Selected\" code_signal=state_code>",
        "size=ThumbnailSize::Size600",
        "background=\"#0f172a\".to_string()",
        "cover=true",
        "layer=true",
        "selected=true",
        "focused=true",
        "class_name=\"docs-thumbnail-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "Thumbnail docs should demonstrate full snapshot configuration token `{needle}`."
        );
    }

    for forbidden in [
        "data-ui-stream-mode",
        "data-ui-stream-support",
        "data-ui-output-status",
        "token_delta",
        "partial_chunk",
        "incremental_render",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Thumbnail snapshot rendering should not depend on streaming-only token `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_logic_normalizes_lang_without_hardcoded_copy() {
    let source = load_source("src/thumbnail/logic.rs");

    for needle in [
        "pub fn normalize_lang(value: Option<String>) -> Option<String>",
        "normalize_optional_text(value)",
    ] {
        assert!(
            source.contains(needle),
            "Thumbnail logic should normalize locale input in logic layer; missing `{needle}`."
        );
    }
}

#[test]
fn thumbnail_logic_remains_pure_normalization_mapping_layer() {
    let source = load_source("src/thumbnail/logic.rs");

    for forbidden in [
        "view! {",
        "NodeRef",
        "on:click",
        "on:keydown",
        "web_sys",
        "style.set_property",
    ] {
        assert!(
            !source.contains(forbidden),
            "Thumbnail logic.rs should not include DOM/event/render side effects; found `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_interaction_matrix_is_not_applicable_for_display_primitive() {
    let source = load_source("src/thumbnail/view.rs");

    for forbidden in [
        "selected_count",
        "default_selected",
        "on_selected_change",
        "is_disabled",
        "aria-disabled",
        "on:pointerdown=",
        "on:pointerup=",
        "on:keydown=",
        "on:keyup=",
    ] {
        assert!(
            !source.contains(forbidden),
            "Thumbnail has no controlled/uncontrolled or keyboard/pointer interaction contract; `{forbidden}` should not appear."
        );
    }
}

#[test]
fn thumbnail_view_consumes_logic_outputs_without_rebuilding_state_machine() {
    let source = load_source("src/thumbnail/view.rs");

    for needle in [
        "let view_state = logic::resolve_view_state(",
        "motion_source: logic::resolve_motion_source(motion),",
        "logic::normalize_input(background, class_name),",
    ] {
        assert!(
            source.contains(needle),
            "Thumbnail view should consume logic output via `{needle}`."
        );
    }

    for forbidden in [
        "logic::resolve_state(",
        "sanitize_background(",
        "normalize_optional_text(",
        "let data_state =",
    ] {
        assert!(
            !source.contains(forbidden),
            "Thumbnail view.rs should not rebuild core state decisions; found `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_view_macro_complexity_is_split_into_semantic_subrenders() {
    let source = load_source("src/thumbnail/view.rs");

    for needle in [
        "fn render_thumbnail_content(children: Children) -> impl IntoView",
        "let content = render_thumbnail_content(children);",
        "{content}",
    ] {
        assert!(
            source.contains(needle),
            "Thumbnail view should keep macro complexity split marker `{needle}`."
        );
    }

    assert_eq!(
        source.matches("#[component]").count(),
        1,
        "Thumbnail should keep a single public component boundary."
    );
}

#[test]
fn thumbnail_view_functional_split_prefers_plain_functions_over_local_components() {
    let source = load_source("src/thumbnail/view.rs");

    for needle in [
        "fn render_thumbnail_content(children: Children) -> impl IntoView",
        "pub fn Thumbnail(",
    ] {
        assert!(
            source.contains(needle),
            "Thumbnail view should keep function-first split marker `{needle}`."
        );
    }

    {
        let forbidden = "#[component]\nfn render_thumbnail_content(";
        assert!(
            !source.contains(forbidden),
            "Thumbnail local fragments should stay plain functions, not extra components `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_static_fragments_are_constantized_with_stable_semantics() {
    let source = load_source("src/thumbnail/view.rs");

    for needle in [
        "const SLOT_THUMBNAIL: &str = \"thumbnail\";",
        "const SLOT_THUMBNAIL_FRAME: &str = \"thumbnail-frame\";",
        "const SLOT_THUMBNAIL_CONTENT: &str = \"thumbnail-content\";",
        "const CLASS_THUMBNAIL_FRAME: &str = \"ui-thumbnail__frame\";",
        "const CLASS_THUMBNAIL_CONTENT: &str = \"ui-thumbnail__content\";",
        "data-slot=SLOT_THUMBNAIL",
        "data-slot=SLOT_THUMBNAIL_FRAME",
        "data-slot=SLOT_THUMBNAIL_CONTENT",
        "class=CLASS_THUMBNAIL_FRAME",
        "class=CLASS_THUMBNAIL_CONTENT",
    ] {
        assert!(
            source.contains(needle),
            "Thumbnail view should keep static fragment constantization marker `{needle}`."
        );
    }

    for needle in ["lang=locale.lang.clone()", "dir=locale.dir"] {
        assert!(
            source.contains(needle),
            "Thumbnail locale semantics should remain mounted after static-fragment constantization via `{needle}`."
        );
    }

    assert_eq!(
        source.matches("SLOT_THUMBNAIL_FRAME").count(),
        2,
        "Thumbnail frame slot should keep a single constant source + one mount usage."
    );
    assert_eq!(
        source.matches("SLOT_THUMBNAIL_CONTENT").count(),
        2,
        "Thumbnail content slot should keep a single constant source + one mount usage."
    );
}

#[test]
fn thumbnail_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    for rel_path in [
        "src/thumbnail/view.rs",
        "src/thumbnail/logic.rs",
        "src/thumbnail/motion.rs",
        "src/thumbnail/styles.rs",
        "src/thumbnail/mod.rs",
        "../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in ["inner_html", "set_inner_html", "dangerously_set_inner_html"] {
            assert!(
                !source.contains(forbidden),
                "Thumbnail path `{rel_path}` must not inject raw HTML; found `{forbidden}`."
            );
        }
    }
}

#[test]
fn thumbnail_docs_inner_html_is_restricted_to_trusted_whitelisted_markdown_sources() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let markdown_page_source = load_source("../../apps/docs-app/src/pages/docs/markdown_page.rs");

    for needle in [
        "const ACCORDION_README_MD: &str =",
        "include_str!(\"../../../../../components/accordion/src/README.md\")",
        "const DATE_PICKER_README_MD: &str =",
        "include_str!(\"../../../../../crates/ui-components/src/text_input/date_picker/README.md\")",
        "fn component_readme_markdown(slug: &str) -> Option<&'static str> {",
        "\"accordion\" => Some(ACCORDION_README_MD),",
        "\"date-picker\" => Some(DATE_PICKER_README_MD),",
        "_ => None,",
        "let readme_html = component_readme_markdown(slug).map(crate::markdown::markdown_to_html);",
        "<section class=\"docs-card docs-prose\" data-slot=\"component-readme\" inner_html=html></section>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep trusted inner_html whitelist marker `{needle}`."
        );
    }

    for forbidden in [
        "inner_html=format!(",
        "inner_html=slug",
        "inner_html=description",
    ] {
        assert!(
            !shell_source.contains(forbidden),
            "docs shell must not pipe dynamic text directly to inner_html via `{forbidden}`."
        );
    }

    for needle in [
        "pub fn MarkdownPage(markdown: &'static str) -> impl IntoView",
        "let crate::markdown::MarkdownDoc {",
        "html: rendered_html,",
        "} = crate::markdown::render_markdown(markdown);",
        "let html = StoredValue::new(rendered_html);",
        "<div node_ref=container_ref inner_html=move || html.get_value()></div>",
    ] {
        assert!(
            markdown_page_source.contains(needle),
            "docs markdown page should keep trusted static markdown-to-html flow marker `{needle}`."
        );
    }

    for forbidden in ["inner_html=markdown", "inner_html=move || markdown"] {
        assert!(
            !markdown_page_source.contains(forbidden),
            "docs markdown page must not directly inject markdown source via `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_wasm_debug_capability_stays_feature_isolated_and_non_polluting() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let view_source = load_source("src/thumbnail/view.rs");
    let mod_source = load_source("src/thumbnail/mod.rs");
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "#[cfg(target_arch = \"wasm32\")]\nmod observability;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui-components should keep wasm debug capability isolated via `{needle}`."
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components Cargo features should keep explicit wasm-debug opt-in marker `{needle}`."
        );
    }

    assert!(
        !cargo_source.contains("thumbnail-wasm-debug"),
        "Thumbnail should not expose a dedicated wasm-debug feature because it is a non-interactive display primitive."
    );

    for forbidden in [
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "request_replay",
        "tracing::",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !view_source.contains(forbidden) && !mod_source.contains(forbidden),
            "Thumbnail production contract should not leak wasm-debug internals `{forbidden}`."
        );
    }

    for needle in [
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features inject-css,button-wasm-debug",
        "cargo test -p ui-components --test button_semantics button_wasm_debug_contract_is_feature_gated_and_dev_only",
    ] {
        assert!(
            script_source.contains(needle),
            "wasm-debug gate script should keep feature-isolated verification marker `{needle}`."
        );
    }
}

#[test]
fn thumbnail_wasm_debug_observability_uses_global_trace_overlay_with_timestamped_events() {
    let view_source = load_source("src/thumbnail/view.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");

    for needle in [
        "data-state=state.data_state.as_attr()",
        "data-cover-source=cover_source.as_attr()",
        "data-layer-source=layer_source.as_attr()",
        "data-selected-source=selected_source.as_attr()",
        "data-focused-source=focused_source.as_attr()",
        "data-motion-source=motion_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "Thumbnail should expose stable source/state markers for wasm debug observability via `{needle}`."
        );
    }

    for forbidden in [
        "on:click=",
        "on:keydown=",
        "on:pointerdown=",
        "on:pointerup=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Thumbnail has no key interaction replay path; non-applicable interaction token `{forbidden}` should remain absent."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "ui_headless::UiTraceEventKind::Inspect",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "docs debug overlay should keep wasm dev visual-entry/timeline marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub enum UiTraceEventKind {",
        "Inspect {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "let event = UiTraceEvent {",
        "ts_ms: now_ms(),",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace contract should keep timestamped/source event marker `{needle}`."
        );
    }
}

#[test]
fn thumbnail_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "data-playground-scope=scope_id.clone()",
        "data-slot=\"playground-test\"",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
    ] {
        assert!(
            source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }
}

#[test]
fn thumbnail_dx_workbench_uses_interactive_playground_and_marks_persist_state_as_not_applicable() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");
    let view_source = load_source("src/thumbnail/view.rs");

    for needle in [
        "pub(super) fn thumbnail() -> AnyView",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Sizes\" code_signal=size_code>",
        "<Playground title=\"Cover + Background + Layer + Selected\" code_signal=state_code>",
        "<Playground title=\"Custom Motion Contract\" code_signal=motion_code>",
        "class_name=\"docs-thumbnail-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "Thumbnail docs should provide isolated interactive playground entry `{needle}`."
        );
    }

    for forbidden in [
        "WORKBENCH_STORAGE_KEY",
        "load_workbench_",
        "save_workbench_",
        "clear_workbench_",
        "Persist workbench state",
        "test_css_source=",
        "test_config_signal=",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Thumbnail has no interactive state-machine context to persist; workbench persistence is N/A and `{forbidden}` should remain absent."
        );
    }

    for forbidden in [
        "on:click=",
        "on:keydown=",
        "on:pointerdown=",
        "on:pointerup=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Thumbnail view should remain non-interactive display primitive; context-preserving interaction replay token `{forbidden}` is N/A."
        );
    }
}

#[test]
fn thumbnail_dx_check_script_covers_hot_reload_and_workbench_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "[dx] contract: playground css hot-reload path",
        "cargo test -p ui-components --test button_semantics button_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test button_semantics button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
        "cargo test -p ui-components --test button_copy_semantics button_copy_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
        "cargo test -p ui-components --test action_button_semantics action_button_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test action_button_semantics action_button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce marker `{needle}`."
        );
    }
}

#[test]
fn thumbnail_engineering_contract_stays_spec_free_and_runtime_agnostic() {
    let combined = [
        load_source("src/thumbnail/mod.rs"),
        load_source("src/thumbnail/logic.rs"),
        load_source("src/thumbnail/view.rs"),
        load_source("src/thumbnail/motion.rs"),
        load_source("src/thumbnail/styles.rs"),
    ]
    .join("\n");
    let checklist_source = load_source("src/thumbnail/check2.md");

    for forbidden in [
        "serde::",
        "Serialize",
        "Deserialize",
        "serde_json::",
        "tracing::",
        "tokio::",
        "async_std::",
        "async-std",
        "Runtime",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Thumbnail simple display primitive should keep engineering/runtime concerns out of component contract via `{forbidden}`.",
        );
    }

    for needle in [
        "- [ ] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            checklist_source.contains(needle),
            "Thumbnail checklist should retain engineering governance token `{needle}`."
        );
    }
}

#[test]
fn thumbnail_engineering_contract_reuses_global_trace_semantics_without_local_drift() {
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let thumbnail_sources = [
        load_source("src/thumbnail/view.rs"),
        load_source("src/thumbnail/logic.rs"),
        load_source("src/thumbnail/motion.rs"),
    ]
    .join("\n");

    for needle in [
        "pub enum UiTraceEventKind {",
        "OpenChange {",
        "Inspect {",
        "Note {",
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            trace_source.contains(needle),
            "Global tracing contract should keep unified event semantics marker `{needle}`."
        );
    }

    for needle in [
        "fn render_event(event: ui_headless::UiTraceEvent) -> AnyView",
        "ui_headless::UiTraceEventKind::OpenChange { open }",
        "ui_headless::UiTraceEventKind::Inspect { tag, data_slot }",
        "ui_headless::UiTraceEventKind::Note { message }",
        "data-kind=kind_attr",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "Debug overlay should render unified trace taxonomy via `{needle}`."
        );
    }

    for forbidden in [
        "target: \"ui_components::thumbnail",
        "trace.emit(",
        "UiTraceEventKind::",
        "span!(",
    ] {
        assert!(
            !thumbnail_sources.contains(forbidden),
            "Thumbnail should not invent component-local tracing taxonomy `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_engineering_check_script_covers_unified_serde_tracing_runtime_baseline() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "[engineering] contract: serde schema + structured migration errors",
        "cargo test -p ui-components --test button_semantics button_engineering_contract_uses_serde_schema_and_structured_migration_errors",
        "[engineering] contract: tracing target semantics",
        "cargo test -p ui-components --test button_semantics button_engineering_contract_uses_consistent_tracing_targets",
        "[engineering] contract: runtime boundary leakage",
        "cargo test -p ui-components --test button_semantics button_engineering_contract_avoids_runtime_leaks_in_public_api",
        "cargo test -p ui-components --test button_copy_semantics button_copy_engineering_contract_reuses_button_tracing_and_avoids_runtime_leaks",
        "cargo test -p ui-components --test action_button_semantics action_button_engineering_contract_reuses_button_tracing_and_avoids_runtime_leaks",
    ] {
        assert!(
            script_source.contains(needle),
            "Engineering check script should enforce baseline marker `{needle}`."
        );
    }
}

#[test]
fn thumbnail_ui_components_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks() {
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "pub use ui_headless::{MenuItemKind, OnPress};",
        "#[cfg(feature = \"component-thumbnail\")]",
        "pub mod thumbnail;",
        "#[cfg(feature = \"all-components\")]",
        "pub use all_components::*;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod css;",
        "leptos::web_sys",
        "web_sys::",
        "wasm_bindgen",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-components lib entry should not leak platform/internal marker `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_ui_components_css_registry_remains_feature_gated_and_non_global() {
    let css_source = load_source("src/css.rs");

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-thumbnail\")]",
        "out.push_str(crate::thumbnail::styles::CSS);",
        "#[cfg(feature = \"component-active_highlight\")]",
        "out.push_str(crate::active_highlight::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css registry should keep feature-gated marker `{needle}`."
        );
    }
}

#[test]
fn thumbnail_ui_root_centralizes_theme_injection_and_i18n_context() {
    let root_source = load_source("src/root.rs");

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n};",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if let Some(overrides) = semantic_overrides.get_value() {",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "data-theme-scheme",
        "data-theme-color",
        "data-theme-system",
        "data-theme-scale",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized theme/i18n marker `{needle}`."
        );
    }
}

#[test]
fn thumbnail_active_highlight_stays_shared_motion_primitive_without_component_semantics() {
    let source = load_source("src/active_highlight.rs");

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "Thumbnail",
        "Accordion",
        "Button",
        "Sidebar",
        "aria-",
        "data-slot",
        "on:click",
    ] {
        assert!(
            !source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present() {
    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden),
            "ui-components forbidden entrypoint file should not exist: `{forbidden}`."
        );
    }

    for required in [
        "../../crates/ui-headless/src/controllable_state.rs",
        "../../crates/ui-headless/src/presence.rs",
        "../../crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            path_exists(required),
            "ui-headless canonical primitive file should exist: `{required}`."
        );
    }

    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            controllable_state_source.contains(needle)
                || presence_source.contains(needle)
                || a11y_source.contains(needle),
            "headless canonical primitive files should keep marker `{needle}`."
        );
    }
}

#[test]
fn thumbnail_ui_components_entrypoints_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-entrypoints.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics ui_components_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks",
        "cargo test -p ui-components --test button_semantics ui_components_css_registry_remains_feature_gated_and_non_global",
        "cargo test -p ui-components --test button_semantics ui_root_centralizes_theme_injection_and_i18n_context",
        "cargo test -p ui-components --test button_semantics active_highlight_stays_shared_motion_primitive_without_component_semantics",
        "cargo test -p ui-components --test button_semantics ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present",
    ] {
        assert!(
            script_source.contains(needle),
            "entrypoints check script should enforce `{needle}`."
        );
    }
}

#[test]
fn thumbnail_styles_file_is_static_css_contract_only() {
    let source = load_source("src/thumbnail/styles.rs");

    assert!(
        source.contains("pub const CSS: &str = r#\""),
        "Thumbnail styles.rs should expose a static CSS contract constant."
    );
    for forbidden in ["fn ", "view! {", "on:click", "NodeRef", "web_sys"] {
        assert!(
            !source.contains(forbidden),
            "Thumbnail styles.rs should not contain runtime/render logic; found `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_is_not_collection_composition_api() {
    let source = load_source("src/thumbnail/view.rs");

    for forbidden in ["labels", "titles", "panels", "ItemSpec", "default_items"] {
        assert!(
            !source.contains(forbidden),
            "Thumbnail is a single-slot primitive, not a Parent/Item collection API; `{forbidden}` should not appear."
        );
    }
}

#[test]
fn thumbnail_motion_covers_wasm_and_non_wasm_contract_paths() {
    let source = load_source("src/thumbnail/motion.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "_node_ref: leptos::prelude::NodeRef<leptos::html::Div>",
    ] {
        assert!(
            source.contains(needle),
            "Thumbnail motion should keep `{needle}` so SSR/non-wasm and wasm paths are both contract-tested."
        );
    }
}

#[test]
fn thumbnail_platform_compile_script_covers_default_ssr_wasm_paths() {
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "[platform] compile-only: default native path",
        "cargo check -p ui-components",
        "[platform] compile-only: ssr native path",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "[platform] compile-only: web wasm path",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should include `{needle}` for native/ssr/wasm compile-only evidence."
        );
    }
}

#[test]
fn thumbnail_ui_headless_web_ssr_mutex_is_compile_error_guarded() {
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should keep web/ssr mutex compile guard `{needle}`."
        );
    }
}

#[test]
fn thumbnail_platform_script_enforces_ui_headless_web_ssr_mutex() {
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "[platform] compile guard: ui-headless web+ssr must fail",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "mutually exclusive",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            script_source.contains(needle),
            "platform guard script should enforce ui-headless feature mutex and dual compile paths via `{needle}`."
        );
    }
}

#[test]
fn thumbnail_non_wasm_files_stay_web_sys_free() {
    for rel in [
        "src/thumbnail/mod.rs",
        "src/thumbnail/logic.rs",
        "src/thumbnail/styles.rs",
        "src/thumbnail/view.rs",
    ] {
        let source = load_source(rel);
        for forbidden in ["web_sys", "wasm_bindgen", "JsCast"] {
            assert!(
                !source.contains(forbidden),
                "non-wasm thumbnail file `{rel}` should not reference browser-specific token `{forbidden}`."
            );
        }
    }
}

#[test]
fn thumbnail_motion_web_sys_usage_is_explicitly_cfg_gated() {
    let source = load_source("src/thumbnail/motion.rs");
    let wasm_cfg_pos = source
        .find("#[cfg(target_arch = \"wasm32\")]")
        .expect("thumbnail motion should have wasm cfg branch");
    let non_wasm_cfg_pos = source
        .find("#[cfg(not(target_arch = \"wasm32\"))]")
        .expect("thumbnail motion should have non-wasm cfg branch");
    let web_sys_pos = source
        .find("leptos::web_sys::HtmlElement")
        .expect("thumbnail motion wasm path should use web_sys element access");

    assert!(
        web_sys_pos > wasm_cfg_pos,
        "web_sys usage should appear only inside the wasm cfg branch."
    );
    assert!(
        non_wasm_cfg_pos > wasm_cfg_pos,
        "non-wasm cfg branch should be explicitly declared after wasm branch."
    );
    assert!(
        source.contains("pub fn attach_motion(")
            && source.contains("_node_ref: leptos::prelude::NodeRef<leptos::html::Div>")
            && source.contains("_active: leptos::prelude::Signal<bool>")
            && source.contains("_motion: ThumbnailMotion"),
        "thumbnail motion should keep explicit non-wasm stub signature for predictable SSR/tooling behavior."
    );
}

#[test]
fn thumbnail_ui_motion_non_wasm_stub_contract_is_explicit_and_predictable() {
    let motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "Compile on non-wasm targets (no-op stubs) to support SSR/tooling builds.",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`."
        );
    }
}

#[test]
fn thumbnail_platform_script_covers_ui_motion_native_wasm_and_stub_paths() {
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "echo \"[platform] compile-only: ui-motion native path\"",
        "cargo check -p ui-motion",
        "echo \"[platform] compile-only: ui-motion wasm path\"",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "echo \"[platform] ui-motion non-wasm stub tests\"",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            script_source.contains(needle),
            "platform script should cover ui-motion stub/compile path token `{needle}`."
        );
    }
}

#[test]
fn thumbnail_motion_file_stays_in_motion_contract_scope() {
    let source = load_source("src/thumbnail/motion.rs");

    for needle in [
        "ui_motion::spring::SpringAnimator",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
    ] {
        assert!(
            source.contains(needle),
            "Thumbnail motion.rs should map component semantics to motion contract with `{needle}`."
        );
    }
    for forbidden in ["view! {", "data-slot=", "on:click=", "aria-"] {
        assert!(
            !source.contains(forbidden),
            "Thumbnail motion.rs should not include view/a11y/event rendering concerns; found `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_async_semantics_are_not_applicable() {
    let source = load_source("src/thumbnail/view.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "data-loading",
        "on_retry",
        "data-error",
    ] {
        assert!(
            !source.contains(forbidden),
            "Thumbnail has no remote request or async state machine; `{forbidden}` should not appear."
        );
    }
}

#[test]
fn thumbnail_styles_include_motion_marker_contracts() {
    let source = load_source("src/thumbnail/styles.rs");

    for selector in [
        ".ui-thumbnail[data-motion-source=\"custom\"]",
        ".ui-thumbnail[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Thumbnail styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn thumbnail_styles_state_selectors_use_explicit_markers() {
    let source = load_source("src/thumbnail/styles.rs");

    for selector in [
        ".ui-thumbnail[data-cover=\"true\"]",
        ".ui-thumbnail[data-layer=\"true\"]",
        ".ui-thumbnail[data-selected=\"true\"]",
        ".ui-thumbnail[data-focused=\"true\"]",
        ".ui-thumbnail[data-size=\"500\"]",
    ] {
        assert!(
            source.contains(selector),
            "Thumbnail styles should drive state via explicit semantic selector `{selector}`."
        );
    }
}

#[test]
fn thumbnail_styles_avoid_fragile_structure_guessing_selectors() {
    let source = load_source("src/thumbnail/styles.rs");

    for forbidden in [":nth-child", ":nth-of-type"] {
        assert!(
            !source.contains(forbidden),
            "Thumbnail styles should not rely on fragile structure selector `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_token_first_styles_are_static_and_aggregated_via_ui_root() {
    let styles_source = load_source("src/thumbnail/styles.rs");
    let view_source = load_source("src/thumbnail/view.rs");
    let logic_source = load_source("src/thumbnail/logic.rs");
    let motion_source = load_source("src/thumbnail/motion.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-radius-sm)",
        "var(--ui-accent)",
        "var(--ui-fg)",
        "var(--ui-thumbnail-size)",
        "var(--ui-thumbnail-background)",
    ] {
        assert!(
            styles_source.contains(needle),
            "Thumbnail styles should stay token-first/static and include `{needle}`."
        );
    }

    for forbidden in [
        "@apply",
        "styled(",
        "css!(",
        "tailwind",
        "tw-",
        "var(--thumbnail-",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Thumbnail styles should avoid utility/CSS-in-Rust/private-token pattern `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-thumbnail\")]",
        "out.push_str(crate::thumbnail::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "Thumbnail styles must be aggregated through css.rs via `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] inject_components_css: bool",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should remain the centralized injection path via `{needle}`."
        );
    }

    assert!(
        view_source.contains("style=inline_style.get_value().unwrap_or_default()"),
        "Thumbnail view should only mount precomputed css-variable style output."
    );
    assert!(
        logic_source.contains("format!(\"--ui-thumbnail-background: {background};\")"),
        "Thumbnail runtime style mapping should stay css-variable-only."
    );

    for forbidden in [
        "set_property(\"transform\"",
        "set_property(\"opacity\"",
        "set_property(\"width\"",
        "set_property(\"height\"",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "Thumbnail motion runtime writes should avoid non-variable style mutations `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_runtime_style_only_sets_css_custom_property() {
    let view_source = load_source("src/thumbnail/view.rs");
    let logic_source = load_source("src/thumbnail/logic.rs");

    assert!(
        view_source.contains("style=inline_style.get_value().unwrap_or_default()"),
        "Thumbnail view should mount precomputed inline style only."
    );
    assert!(
        logic_source.contains("format!(\"--ui-thumbnail-background: {background};\")"),
        "Thumbnail runtime style should only set CSS custom properties."
    );
}

#[test]
fn thumbnail_motion_uses_spring_animator() {
    let source = load_source("src/thumbnail/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Thumbnail motion should animate via springs to match the repo motion spec."
    );
}

#[test]
fn thumbnail_motion_contract_defaults_match_upstream_level_expectations() {
    let source = load_source("src/thumbnail/motion.rs");

    for needle in [
        "stiffness: 260.0",
        "damping: 19.0",
        "mass: 1.0",
        "active_scale: 1.03",
        "active_ring_opacity: 1.0",
        "pub fn disabled() -> Self",
        "enabled: false",
    ] {
        assert!(
            source.contains(needle),
            "Thumbnail motion contract should include `{needle}` for baseline-level defaults and disabled-path stability."
        );
    }
}

#[test]
fn thumbnail_motion_sanitization_and_reduced_motion_paths_are_locked() {
    let source = load_source("src/thumbnail/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ThumbnailMotion) -> ThumbnailMotion",
        ".clamp(1.0, 1.2)",
        ".clamp(0.0, 1.0)",
        "!motion.enabled || ui_motion::web::prefers_reduced_motion()",
        "fn sanitize_motion_falls_back_for_invalid_values()",
    ] {
        assert!(
            source.contains(needle),
            "Thumbnail motion implementation should include `{needle}` to avoid baseline-level motion regressions."
        );
    }
}

#[test]
fn thumbnail_reduced_motion_degrades_via_ui_motion_fast_path() {
    let thumbnail_motion_source = load_source("src/thumbnail/motion.rs");
    let spring_source = load_source("../../crates/ui-motion/src/spring.rs");

    for needle in [
        "!motion.enabled || ui_motion::web::prefers_reduced_motion()",
        "scale.set_target(target_scale);",
        "ring.set_target(target_ring);",
    ] {
        assert!(
            thumbnail_motion_source.contains(needle),
            "Thumbnail reduced-motion branch should keep `{needle}`."
        );
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            spring_source.contains(needle),
            "ui-motion spring reduced-motion fast path should include `{needle}`."
        );
    }
}

#[test]
fn thumbnail_ssr_and_wasm_keep_single_semantic_contract_surface() {
    let view_source = load_source("src/thumbnail/view.rs");
    let motion_source = load_source("src/thumbnail/motion.rs");

    for marker in [
        "data-size=state.size_attr",
        "data-state=state.data_state.as_attr()",
        "data-cover-source=cover_source.as_attr()",
        "data-layer-source=layer_source.as_attr()",
        "data-selected-source=selected_source.as_attr()",
        "data-focused-source=focused_source.as_attr()",
        "data-motion-source=motion_source.as_attr()",
    ] {
        assert!(
            view_source.contains(marker),
            "Thumbnail semantic markers should stay mounted from view.rs via `{marker}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not split semantic markup by platform via `{forbidden}`."
        );
    }

    for forbidden in [
        "data-state=",
        "data-cover-source=",
        "data-layer-source=",
        "aria-",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not own semantic contract output `{forbidden}`; semantics must remain platform-invariant in view.rs."
        );
    }
}

#[test]
fn thumbnail_styles_use_css_variables_for_motion() {
    let source = load_source("src/thumbnail/styles.rs");

    for name in ["--ui-thumbnail-scale", "--ui-thumbnail-ring-opacity"] {
        assert!(
            source.contains(name),
            "Thumbnail styles should define `{name}` so motion updates only touch CSS variables."
        );
    }
}

#[test]
fn thumbnail_docs_page_exists_in_display_extra_thumbnail() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");

    for needle in [
        "pub(super) fn thumbnail() -> AnyView",
        "title=\"Thumbnail\"",
        "slug=\"thumbnail\"",
        "<Thumbnail",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_thumbnail docs page should contain `{needle}`."
        );
    }
}

#[test]
fn thumbnail_docs_page_includes_custom_motion_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");

    for needle in [
        "title=\"Custom Motion Contract\"",
        "ThumbnailMotion {",
        "motion=custom_motion",
        "motion=ThumbnailMotion::disabled()",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_thumbnail docs page should include `{needle}` for custom motion contract demos."
        );
    }
}

#[test]
fn thumbnail_docs_default_and_state_playgrounds_lock_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Thumbnail>",
        "alt=\"Thumbnail sample\"",
        "<Playground title=\"Sizes\" code_signal=size_code>",
        "size=ThumbnailSize::Size100",
        "size=ThumbnailSize::Size500",
        "size=ThumbnailSize::Size900",
        "alt=\"Landscape\"",
        "alt=\"Portrait\"",
        "alt=\"Panorama\"",
        "<Playground title=\"Cover + Background + Layer + Selected\" code_signal=state_code>",
        "size=ThumbnailSize::Size600",
        "background=\"#0f172a\".to_string()",
        "cover=true",
        "layer=true",
        "selected=true",
        "focused=true",
        "class_name=\"docs-thumbnail-custom\".to_string()",
        "alt=\"Cover sample\"",
    ] {
        assert!(
            source.contains(needle),
            "thumbnail docs default/state playground should contain `{needle}`.",
        );
    }
}

#[test]
fn thumbnail_docs_custom_motion_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");

    for needle in [
        "<Playground title=\"Custom Motion Contract\" code_signal=motion_code>",
        "let custom_motion = ThumbnailMotion {",
        "active_scale: 1.08,",
        "active_ring_opacity: 0.9,",
        "..ThumbnailMotion::default()",
        "motion=custom_motion",
        "motion=ThumbnailMotion::disabled()",
        "alt=\"Featured motion contract\"",
        "alt=\"Reduced motion contract\"",
    ] {
        assert!(
            source.contains(needle),
            "thumbnail docs custom-motion playground should contain `{needle}`.",
        );
    }
}

#[test]
fn thumbnail_docs_page_covers_primary_playgrounds() {
    thumbnail_docs_page_exists_in_display_extra_thumbnail();
}

#[test]
fn thumbnail_docs_playgrounds_lock_state_matrix_contract_values() {
    thumbnail_docs_default_and_state_playgrounds_lock_contract_values();
    thumbnail_docs_custom_motion_playground_locks_contract_values();
}

#[test]
fn thumbnail_docs_sync_covers_examples_parameter_and_state_matrices() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");
    let check2_source = load_source("src/thumbnail/check2.md");

    for needle in [
        "docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(needle),
            "thumbnail checklist should keep docs-sync governance marker `{needle}`."
        );
    }

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Sizes\" code_signal=size_code>",
        "<Playground title=\"Cover + Background + Layer + Selected\" code_signal=state_code>",
        "<Playground title=\"Custom Motion Contract\" code_signal=motion_code>",
        "size=ThumbnailSize::Size100",
        "size=ThumbnailSize::Size500",
        "size=ThumbnailSize::Size900",
        "background=\"#0f172a\".to_string()",
        "cover=true",
        "layer=true",
        "selected=true",
        "focused=true",
    ] {
        assert!(
            docs_source.contains(needle),
            "thumbnail docs should keep matrix/example coverage marker `{needle}`."
        );
    }
}

#[test]
fn thumbnail_docs_api_names_and_defaults_match_logic_contract() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");
    let view_source = load_source("src/thumbnail/view.rs");
    let logic_source = load_source("src/thumbnail/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/thumbnail.rs");

    for needle in [
        "#[prop(optional)] size: ThumbnailSize,",
        "#[prop(optional, into)] background: Option<String>,",
        "#[prop(optional, into)] cover: Option<bool>,",
        "#[prop(optional, into)] layer: Option<bool>,",
        "#[prop(optional, into)] selected: Option<bool>,",
        "#[prop(optional, into)] focused: Option<bool>,",
        "#[prop(optional)] motion: ThumbnailMotion,",
        "#[prop(optional, into)] class_name: Option<String>,",
    ] {
        assert!(
            view_source.contains(needle),
            "thumbnail view public API should include `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve(value: Option<bool>) -> (bool, Self)",
        "(false, Self::Default)",
        "#[default]",
        "Size500",
        "<Thumbnail>",
        "size=ThumbnailSize::Size500",
        "cover=true",
        "layer=true",
        "selected=true",
        "focused=true",
        "class_name=\"docs-thumbnail-custom\".to_string()",
    ] {
        assert!(
            logic_source.contains(needle)
                || primitive_source.contains(needle)
                || docs_source.contains(needle),
            "thumbnail docs/default alignment should keep marker `{needle}`."
        );
    }

    for forbidden in ["is_cover=", "is_layer=", "is_selected=", "is_focused="] {
        assert!(
            !docs_source.contains(forbidden),
            "thumbnail docs should not drift to unsupported API alias `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_docs_entry_exists_as_readme_or_equivalent_docs_app_page() {
    let has_readme = path_exists("src/thumbnail/README.md");
    let has_docs_page =
        path_exists("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");

    assert!(
        has_readme || has_docs_page,
        "Thumbnail must provide README or equivalent docs-app entry."
    );
    assert!(
        docs_source.contains("pub(super) fn thumbnail() -> AnyView"),
        "Equivalent docs entry should expose thumbnail page function."
    );
}

#[test]
fn thumbnail_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");
    let check2_source = load_source("src/thumbnail/check2.md");

    for needle in [
        "组件文档必须对新手友好（Documentation as Product）",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法",
        "文档需明确“先用起来，再进阶”",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "thumbnail checklist should keep documentation-as-product marker `{needle}`."
        );
    }

    for needle in [
        "title=\"Thumbnail\"",
        "slug=\"thumbnail\"",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Sizes\" code_signal=size_code>",
        "<Playground title=\"Cover + Background + Layer + Selected\" code_signal=state_code>",
        "<Playground title=\"Custom Motion Contract\" code_signal=motion_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Thumbnail docs should include beginner-to-advanced progression marker `{needle}`."
        );
    }

    let hello_pos = docs_source
        .find("<Playground title=\"Hello World\" code_signal=hello_world_code>")
        .expect("thumbnail docs should include hello-world playground");
    let sizes_pos = docs_source
        .find("<Playground title=\"Sizes\" code_signal=size_code>")
        .expect("thumbnail docs should include common usage playground");
    let advanced_pos = docs_source
        .find("<Playground title=\"Custom Motion Contract\" code_signal=motion_code>")
        .expect("thumbnail docs should include advanced playground");
    assert!(
        hello_pos < sizes_pos && sizes_pos < advanced_pos,
        "Thumbnail docs should present default usage before advanced controls."
    );
}

#[test]
fn thumbnail_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");
    let start = docs_source
        .find("let hello_world_code = Signal::derive(move || {")
        .expect("thumbnail docs should define hello_world_code");
    let end = docs_source[start..]
        .find("let size_code = Signal::derive(move || {")
        .map(|offset| start + offset)
        .expect("thumbnail docs should define size_code after hello world");
    let hello_block = &docs_source[start..end];

    let snippet_start = hello_block
        .find("r#\"")
        .map(|idx| idx + 3)
        .expect("hello snippet should be embedded as raw string");
    let snippet_end = hello_block[snippet_start..]
        .find("\"#")
        .map(|offset| snippet_start + offset)
        .expect("hello snippet should terminate raw string");
    let hello_snippet = &hello_block[snippet_start..snippet_end];
    let meaningful_lines = hello_snippet
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();

    assert!(
        meaningful_lines <= 5,
        "Thumbnail Hello World snippet should stay <= 5 lines for beginner DX, got {meaningful_lines} lines:\n{hello_snippet}"
    );

    for forbidden in [
        "ui_state_primitives",
        "ui-headless",
        "ui_headless",
        "state=",
        "controller=",
        "Signal<",
    ] {
        assert!(
            !hello_snippet.contains(forbidden),
            "Thumbnail Hello World path should not require architecture-level wiring `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_docs_app_provides_interactive_playground_with_live_props_and_state_preview() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let check2_source = load_source("src/thumbnail/check2.md");

    for needle in [
        "`apps/docs-app` 必须提供 Interactive Playground",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(needle),
            "thumbnail checklist should keep interactive-playground marker `{needle}`."
        );
    }

    for needle in [
        "let hello_world_code = Signal::derive(move || {",
        "let size_code = Signal::derive(move || {",
        "let state_code = Signal::derive(move || {",
        "let motion_code = Signal::derive(move || {",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Sizes\" code_signal=size_code>",
        "<Playground title=\"Cover + Background + Layer + Selected\" code_signal=state_code>",
        "<Playground title=\"Custom Motion Contract\" code_signal=motion_code>",
        "size=ThumbnailSize::Size600",
        "background=\"#0f172a\".to_string()",
        "cover=true",
        "layer=true",
        "selected=true",
        "focused=true",
    ] {
        assert!(
            docs_source.contains(needle),
            "thumbnail docs interactive playground should include `{needle}`."
        );
    }

    for needle in [
        "pub fn Playground(",
        "#[prop(optional, into)] code_signal: Option<Signal<String>>",
        "children: Children,",
        "let resolved_code = Signal::derive(move || {",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground runtime should keep live-preview marker `{needle}`."
        );
    }
}

#[test]
fn thumbnail_docs_source_is_copy_paste_ready_with_imports_and_copy_control() {
    let check2_source = load_source("src/thumbnail/check2.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_field_button_contract.spec.mjs");

    for needle in [
        "Source-first 文档必须 Copy-Paste Ready",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(needle),
            "thumbnail checklist should keep copy-paste-ready governance marker `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        ".map(|snippet| compose_copy_ready_code(&snippet, &code_imports.get_value()))",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground source-first copy pipeline should keep token `{needle}`."
        );
    }

    for needle in [
        "docs-app field-button playground source is copy-paste ready",
        "toHaveAttribute(\"data-copyable\", \"true\")",
        "toHaveAttribute(\"aria-label\", /Copy to clipboard/i)",
    ] {
        assert!(
            e2e_source.contains(needle),
            "docs copy-flow e2e evidence should keep acceptance token `{needle}`."
        );
    }

    for needle in [
        "use ui_components::{Thumbnail, ThumbnailMotion, ThumbnailSize};",
        "let hello_world_code = Signal::derive(move || {",
        "let size_code = Signal::derive(move || {",
        "let state_code = Signal::derive(move || {",
        "let motion_code = Signal::derive(move || {",
    ] {
        assert!(
            docs_source.contains(needle),
            "thumbnail docs source-first snippets should keep copy-ready token `{needle}`."
        );
    }
}

#[test]
fn thumbnail_docs_snippets_stay_synced_with_runtime_thumbnail_api() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");
    let view_source = load_source("src/thumbnail/view.rs");

    for (doc_token, api_token) in [
        (
            "size=ThumbnailSize::Size100",
            "#[prop(optional)] size: ThumbnailSize,",
        ),
        (
            "background=\"#0f172a\".to_string()",
            "#[prop(optional, into)] background: Option<String>,",
        ),
        ("cover=true", "#[prop(optional, into)] cover: Option<bool>,"),
        ("layer=true", "#[prop(optional, into)] layer: Option<bool>,"),
        (
            "selected=true",
            "#[prop(optional, into)] selected: Option<bool>,",
        ),
        (
            "focused=true",
            "#[prop(optional, into)] focused: Option<bool>,",
        ),
        (
            "class_name=\"docs-thumbnail-custom\".to_string()",
            "#[prop(optional, into)] class_name: Option<String>,",
        ),
        (
            "motion=custom_motion",
            "#[prop(optional)] motion: ThumbnailMotion,",
        ),
    ] {
        assert!(
            docs_source.contains(doc_token),
            "thumbnail docs snippet should keep token `{doc_token}`."
        );
        assert!(
            view_source.contains(api_token),
            "thumbnail public API should keep token `{api_token}` for docs/runtime sync."
        );
    }

    for forbidden in ["is_cover=", "is_layer=", "is_selected=", "is_focused="] {
        assert!(
            !docs_source.contains(forbidden),
            "thumbnail docs snippets should not drift to unsupported alias `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_heroui_alignment_doc_and_docs_entry_stay_in_sync() {
    let check2_source = load_source("src/thumbnail/check2.md");
    let heroui_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");
    let view_source = load_source("src/thumbnail/view.rs");

    for needle in [
        "HeroUI 对标文档与组件文档同步",
        "参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "thumbnail checklist should keep HeroUI-alignment governance marker `{needle}`."
        );
    }

    for needle in [
        "### Thumbnail 同步记录（2026-02-17）",
        "`Thumbnail` 维持 display primitive 定位",
        "`size/background/cover/layer/selected/focused/motion/class_name/lang/dir`",
        "component_doc!(\"Thumbnail\", \"thumbnail\", \"Display\", display_extra_thumbnail::thumbnail)",
        "`#/components/thumbnail` 可索引访问",
        "`Hello World`、`Sizes`、`Cover + Background + Layer + Selected`、`Custom Motion Contract`",
    ] {
        assert!(
            heroui_source.contains(needle),
            "HeroUI strategy doc should keep thumbnail sync token `{needle}`."
        );
    }

    for needle in [
        "\"Thumbnail\"",
        "\"thumbnail\"",
        "display_extra_thumbnail::thumbnail",
    ] {
        assert!(
            pages_source.contains(needle),
            "docs catalog entry should expose thumbnail token `{needle}`."
        );
    }

    for needle in [
        "slug=\"thumbnail\"",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Sizes\" code_signal=size_code>",
        "<Playground title=\"Cover + Background + Layer + Selected\" code_signal=state_code>",
        "<Playground title=\"Custom Motion Contract\" code_signal=motion_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "thumbnail docs page should keep indexed/example marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] size: ThumbnailSize,",
        "#[prop(optional, into)] background: Option<String>,",
        "#[prop(optional, into)] cover: Option<bool>,",
        "#[prop(optional, into)] layer: Option<bool>,",
        "#[prop(optional, into)] selected: Option<bool>,",
        "#[prop(optional, into)] focused: Option<bool>,",
        "#[prop(optional)] motion: ThumbnailMotion,",
    ] {
        assert!(
            view_source.contains(needle),
            "thumbnail runtime API should keep token `{needle}` to prevent docs/implementation drift."
        );
    }
}

#[test]
fn thumbnail_antipattern_guardrails_are_explicit_and_enforced() {
    let check2_source = load_source("src/thumbnail/check2.md");
    let primitive_source = load_source("../ui-state-primitives/src/thumbnail.rs");
    let headless_controllable_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let headless_presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");
    let logic_source = load_source("src/thumbnail/logic.rs");
    let view_source = load_source("src/thumbnail/view.rs");
    let mod_source = load_source("src/thumbnail/mod.rs");
    let crate_source = load_source("src/lib.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");
    let semantics_source = load_source("tests/thumbnail_semantics.rs");
    let headless_combined = format!(
        "{headless_controllable_source}\n{headless_presence_source}\n{headless_a11y_source}"
    );

    for needle in [
        "### 8. 明确禁止的反模式",
        "在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "在 `ui-headless` 写视觉和动画编排。",
        "在 `view` 层隐藏关键状态决策。",
        "新增参数但不纳入统一命名与契约。",
        "用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "公共 API 泄露底层实现细节类型。",
        "用临时补丁破坏跨组件一致性。",
        "明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
    ] {
        assert!(
            check2_source.contains(needle),
            "thumbnail checklist must keep anti-pattern governance marker `{needle}`."
        );
    }

    for forbidden in [
        "use leptos",
        "web_sys",
        "view! {",
        "NodeRef",
        "on:click",
        "on:keydown",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives thumbnail primitive must stay DOM/event-free; found `{forbidden}`."
        );
    }

    for forbidden in [
        "class=",
        ".css",
        "@keyframes",
        "animation:",
        "transition:",
        "style.set_property",
        "request_animation_frame",
    ] {
        assert!(
            !headless_combined.contains(forbidden),
            "ui-headless primitives must not carry visual/animation orchestration marker `{forbidden}`."
        );
    }

    for needle in [
        "let view_state = logic::resolve_view_state(",
        "logic::normalize_input(background, class_name),",
        "data-state=state.data_state.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "thumbnail view should consume normalized logic output via `{needle}`."
        );
    }
    for forbidden in [
        "sanitize_background(",
        "normalize_optional_text(",
        "let data_state =",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "thumbnail view must not hide key state decisions via `{forbidden}`."
        );
    }

    for needle in [
        "#[prop(optional)] size: ThumbnailSize,",
        "#[prop(optional, into)] background: Option<String>,",
        "#[prop(optional, into)] cover: Option<bool>,",
        "#[prop(optional, into)] layer: Option<bool>,",
        "#[prop(optional, into)] selected: Option<bool>,",
        "#[prop(optional, into)] focused: Option<bool>,",
        "#[prop(optional)] motion: ThumbnailMotion,",
        "#[prop(optional, into)] class_name: Option<String>,",
    ] {
        assert!(
            view_source.contains(needle),
            "thumbnail public props should remain in typed naming contract via `{needle}`."
        );
    }
    for forbidden in ["is_cover=", "is_layer=", "is_selected=", "is_focused="] {
        assert!(
            !docs_source.contains(forbidden),
            "thumbnail docs should reject alias drift from naming contract `{forbidden}`."
        );
    }

    for forbidden in ["labels", "titles", "panels", "ItemSpec", "default_items"] {
        assert!(
            !view_source.contains(forbidden) && !docs_source.contains(forbidden),
            "thumbnail API should not regress to parallel-array/implicit-structure token `{forbidden}`."
        );
    }

    for forbidden in ["web_sys", "leptos::web_sys", "pub use web_sys"] {
        assert!(
            !mod_source.contains(forbidden) && !crate_source.contains(forbidden),
            "thumbnail public API surface must not leak platform-private token `{forbidden}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::thumbnail::",
        "resolve_state(ThumbnailStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "thumbnail logic should consume sunk primitive capability via `{needle}`."
        );
    }
    assert!(
        !logic_source.contains("pub fn resolve_state("),
        "thumbnail logic must not re-implement state primitive `resolve_state`."
    );

    for needle in [
        "thumbnail_docs_api_names_and_defaults_match_logic_contract",
        "thumbnail_view_consumes_logic_outputs_without_rebuilding_state_machine",
        "thumbnail_is_not_collection_composition_api",
        "thumbnail_non_wasm_files_stay_web_sys_free",
    ] {
        assert!(
            semantics_source.contains(needle),
            "thumbnail semantics suite should keep anti-pattern regression guard `{needle}`."
        );
    }
}

#[test]
fn thumbnail_merge_gate_final_verdict_is_traceable_except_full_repo_gate_deferred() {
    let check2_source = load_source("src/thumbnail/check2.md");
    let semantics_source = load_source("tests/thumbnail_semantics.rs");

    for needle in [
        "### 9. 合并门禁（最终裁决）",
        "架构正确（边界不破）",
        "行为正确（状态与交互语义成立）",
        "可访问性达标（默认可用）",
        "默认主题美学质量达标（与可访问性同级门禁）",
        "可测试（契约可断言）",
        "可维护（命名和模式一致）",
        "可解释（人和自动化都能读懂）",
        "改动在正确层。",
        "命名与全库一致。",
        "无效状态被限制或归一化。",
        "暴露必要语义标记。",
        "覆盖 reduced-motion / SSR / wasm 分支。",
        "文档与示例同步更新。",
        "门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "thumbnail checklist should keep final-merge-gate marker `{needle}`."
        );
    }

    for evidence in [
        "thumbnail_component_file_responsibilities_remain_scoped",
        "thumbnail_view_consumes_logic_outputs_without_rebuilding_state_machine",
        "thumbnail_a11y_contract_is_non_interactive_by_design",
        "thumbnail_mounts_locale_attrs_from_headless_a11y_helpers",
        "default_theme_visual_baseline_docs_contract_exists",
        "default_theme_visual_baseline_e2e_screenshot_contract_exists",
        "thumbnail_semantics_suite_is_contract_first_not_visual_snapshot_only",
        "thumbnail_docs_api_names_and_defaults_match_logic_contract",
        "thumbnail_agent_contract_is_schema_typed_and_traceable_without_dom_guessing",
        "thumbnail_discrete_axes_are_type_constrained_by_enums",
        "thumbnail_invalid_input_normalization_and_failure_location_are_tested",
        "thumbnail_semantic_contract_exposes_state_and_source_markers",
        "thumbnail_reduced_motion_degrades_via_ui_motion_fast_path",
        "thumbnail_ssr_and_wasm_keep_single_semantic_contract_surface",
        "thumbnail_docs_sync_covers_examples_parameter_and_state_matrices",
    ] {
        assert!(
            semantics_source.contains(evidence),
            "thumbnail merge verdict must remain evidence-traceable via `{evidence}`."
        );
    }

    let full_repo_gate_deferred_for_now = true;
    assert!(
        full_repo_gate_deferred_for_now,
        "Per current thumbnail checkpoint, full repository gate (fmt/clippy/test/smoke) is intentionally deferred."
    );
}

#[test]
fn thumbnail_ai_spec_playground_linkage_is_not_applicable_because_component_has_no_spec_surface() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");
    let check2_source = load_source("src/thumbnail/check2.md");

    assert!(
        !path_exists("src/thumbnail/spec.rs"),
        "Thumbnail is not an AI-Spec component; `spec.rs` should remain absent unless schema contract is introduced."
    );

    for needle in [
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "`<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。",
    ] {
        assert!(
            check2_source.contains(needle),
            "thumbnail checklist should keep AI-Spec linkage governance marker `{needle}`."
        );
    }

    for forbidden in [
        "Spec 输入",
        "spec_input",
        "schema_json",
        "schema_version",
        "preview output",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "thumbnail docs should not claim AI-Spec playground linkage token `{forbidden}` when component is non-spec."
        );
    }
}

#[test]
fn thumbnail_playground_acceptance_path_is_repeatable_in_e2e_suite() {
    let coverage_e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let playground_e2e_source =
        load_source("../../e2e/tests/docs_app_playground_code_toggle.spec.mjs");

    for needle in [
        "docs-app components pages render playgrounds (sample)",
        "docs-app components pages render playgrounds (all)",
        "body:not(:has(#boot))",
        "await expect(page.locator(\"section.playground\").first()).toBeVisible();",
        "await expect(page.locator(`[data-slot=\"${slug}\"]`).first()).toBeVisible();",
    ] {
        assert!(
            coverage_e2e_source.contains(needle),
            "docs components coverage e2e should keep repeatable playground acceptance marker `{needle}`."
        );
    }

    for needle in [
        "docs-app component playground can toggle code visibility",
        "const playground = page.locator(\"section.playground\").first();",
        "const toggle = playground.getByRole(\"button\", { name: /Hide code|Show code/ });",
        "const codeBlock = playground.locator('[data-slot=\"code-block\"]');",
    ] {
        assert!(
            playground_e2e_source.contains(needle),
            "playground e2e should keep repeatable interactive-flow marker `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !coverage_e2e_source.contains(forbidden) && !playground_e2e_source.contains(forbidden),
            "playground e2e acceptance path should avoid fragile fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn thumbnail_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "component-thumbnail",
        "inject-css = []",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components Cargo features should include `{needle}` for tree-shaking boundaries."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-thumbnail\")]\npub mod thumbnail;"),
        "lib.rs should feature-gate thumbnail module export for tree-shaking.",
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-thumbnail\")]")
            && css_source.contains("out.push_str(crate::thumbnail::styles::CSS);"),
        "css.rs should gate thumbnail CSS aggregation behind component-thumbnail feature."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)"),
        "css.rs should keep top-level inject-css gate for component CSS injection."
    );

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("web-demo-components")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui-components via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components instead of implicit default pull-up."
    );
}

#[test]
fn thumbnail_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features",
        "cargo tree -e features -i ui-components -p web-demo",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "size regression",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should include `{needle}`."
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
fn default_theme_visual_baseline_docs_contract_exists() {
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");

    for needle in [
        "mod theme_visual_baseline;",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            pages_source.contains(needle),
            "docs pages registry should keep visual baseline route token `{needle}`."
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline",
        "Includes Button/Input/Overlay for visual regression snapshots.",
        "use ui_components::{Button, ButtonVariant, Input, OnPress, Overlay};",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            page_source.contains(needle),
            "theme visual baseline docs page should include `{needle}`."
        );
    }
}

#[test]
fn default_theme_visual_baseline_e2e_screenshot_contract_exists() {
    let e2e_source = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        "process.env.E2E_VISUAL_BASELINE",
        "page.goto(\"/#/components/theme-visual-baseline\")",
        "theme visual baseline renders button/input/overlay",
        "theme visual baseline screenshots",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            e2e_source.contains(needle),
            "visual baseline e2e contract should include `{needle}`."
        );
    }
}

#[test]
fn thumbnail_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../crates/ui-headless/src/perf.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/thumbnail/check2.md");
    let view_source = load_source("src/thumbnail/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "_ => UiPerfBudget::mount_only(120.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep performance budget contract token `{needle}`."
        );
    }

    for needle in [
        "component_doc!(",
        "\"Thumbnail\"",
        "\"thumbnail\"",
        "display_extra_thumbnail::thumbnail",
    ] {
        assert!(
            pages_source.contains(needle),
            "Thumbnail docs page should remain in coverage traversal via `{needle}`.",
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
        "\"mount-only\"",
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
            e2e_source.contains(needle),
            "docs e2e should enforce repeatable perf regression guard `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace-based perf attribution token `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`.",
        );
    }

    for needle in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "Thumbnail checklist should keep performance governance marker `{needle}`.",
        );
    }

    for needle in [
        "data-state=state.data_state.as_attr()",
        "data-size=state.size_attr",
        "data-cover-source=cover_source.as_attr()",
        "data-layer-source=layer_source.as_attr()",
        "data-selected-source=selected_source.as_attr()",
        "data-focused-source=focused_source.as_attr()",
        "data-motion-source=motion_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "Thumbnail view should expose perf triage attribution marker `{needle}`.",
        );
    }
}

#[test]
fn thumbnail_performance_check_script_keeps_budget_and_follow_up_gates() {
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }
}
