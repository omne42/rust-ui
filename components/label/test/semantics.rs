const MOD_SOURCE: &str = include_str!("../src/mod.rs");
const LOGIC_SOURCE: &str = include_str!("../src/logic.rs");
const VIEW_SOURCE: &str = include_str!("../src/view.rs");
const STYLES_SOURCE: &str = include_str!("../src/styles.rs");
const MOTION_SOURCE: &str = include_str!("../src/motion.rs");
const PROTOCOL_SOURCE: &str = include_str!("../src/protocol.rs");
const README_SOURCE: &str = include_str!("../src/README.md");
const TEST_SOURCE: &str = include_str!("semantics.rs");
const CHECK2_SOURCE: &str = include_str!("../check2.md");
const COMPONENT_MANIFEST_SOURCE: &str = include_str!("../src/Component.toml");
const COMPONENT_RBI_SOURCE: &str = include_str!("../src/label.rbi");
const COMPONENT_CARGO_SOURCE: &str = include_str!("../Cargo.toml");
const UI_COMPONENTS_CARGO_SOURCE: &str = include_str!("../../../crates/ui-components/Cargo.toml");
const UI_COMPONENTS_CSS_SOURCE: &str = include_str!("../../../crates/ui-components/src/css.rs");
const UI_COMPONENTS_LIB_SOURCE: &str = include_str!("../../../crates/ui-components/src/lib.rs");
const UI_COMPONENTS_ROOT_SOURCE: &str = include_str!("../../../crates/ui-components/src/root.rs");
const UI_VISUAL_ACTIVE_HIGHLIGHT_SOURCE: &str =
    include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
const UI_THEME_CSS_SOURCE: &str = include_str!("../../../crates/ui-theme/src/css.rs");
const UI_HEADLESS_A11Y_SOURCE: &str = include_str!("../../../crates/ui-headless/src/a11y.rs");
const UI_HEADLESS_CONTROLLABLE_STATE_SOURCE: &str =
    include_str!("../../../crates/ui-headless/src/controllable_state.rs");
const UI_HEADLESS_PRESENCE_SOURCE: &str =
    include_str!("../../../crates/ui-headless/src/presence.rs");
const UI_TRACE_SOURCE: &str = include_str!("../../../crates/ui-headless/src/trace.rs");
const UI_DEBUG_OVERLAY_SOURCE: &str = include_str!("../../../apps/docs-app/src/debug_overlay.rs");
const DOCS_APP_LIB_SOURCE: &str = include_str!("../../../apps/docs-app/src/lib.rs");
const PLAYGROUND_SOURCE: &str = include_str!("../../../apps/docs-app/src/playground.rs");
const FORMS_EXTRA_SOURCE: &str =
    include_str!("../../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
const COMPONENT_PAGES_SOURCE: &str =
    include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
const HEROUI_PARAMETER_STRATEGY_SOURCE: &str =
    include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");
const E2E_LABEL_CONTRACT_SOURCE: &str =
    include_str!("../../../e2e/tests/docs_app_label_contract.spec.mjs");

#[test]
fn label_component_files_follow_responsibility_split() {
    for needle in ["mod logic;", "mod motion;", "pub mod styles;", "mod view;"] {
        assert!(
            MOD_SOURCE.contains(needle),
            "label/mod.rs should include `{needle}`."
        );
    }

    assert!(
        LOGIC_SOURCE.contains("pub use ui_state_primitives::label::{"),
        "label/logic.rs should consume state primitives instead of redefining them."
    );
    assert!(
        VIEW_SOURCE.contains("#[component]"),
        "label/view.rs should host Leptos rendering."
    );
    assert!(
        STYLES_SOURCE.contains("pub const CSS: &str"),
        "label/styles.rs should host static CSS contract."
    );
    assert!(
        MOTION_SOURCE.contains("pub fn attach_motion("),
        "label/motion.rs should expose motion attach contract."
    );
}

#[test]
fn label_logic_consumes_primitives_without_business_store_binding() {
    assert!(
        LOGIC_SOURCE.contains("pub use ui_state_primitives::label::{"),
        "label/logic.rs should consume state primitives as the only state source."
    );

    for forbidden in [
        "use leptos",
        "Signal<",
        "RwSignal",
        "ReadSignal",
        "WriteSignal",
        "Store",
        "AppState",
    ] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden),
            "label/logic.rs should not bind business/reactive store `{forbidden}`."
        );
    }
}

#[test]
fn label_public_api_avoids_web_sys_dom_type_leaks() {
    for source in [MOD_SOURCE, VIEW_SOURCE] {
        assert!(
            !source.contains("web_sys"),
            "label public surface should not expose `web_sys`."
        );
    }
}

#[test]
fn label_view_mounts_headless_and_motion_contracts() {
    for needle in [
        "use ui_headless::{A11yDirection, locale_attrs};",
        "let normalized = logic::normalize_view_input(logic::LabelViewInput {",
        "let render_state = logic::derive_render_state(",
        "let motion_style = Signal::derive(move || motion::attach_motion(None, motion));",
        "style=move || motion_style.get()",
        "data-motion-source=motion_source",
        "locale_attrs(normalized.lang, dir)",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "label/view.rs should mount semantic contract `{needle}`."
        );
    }
}

#[test]
fn label_defaults_are_normalized_in_logic_single_source() {
    for needle in [
        "pub(super) struct LabelViewInput",
        "pub(super) struct NormalizedLabelViewInput",
        "pub(super) fn normalize_view_input(input: LabelViewInput)",
        "normalize_label_text(input.text)",
        "normalize_required_indicator(input.required_indicator)",
        "normalize_optional_text(input.for_id)",
        "normalize_optional_text(input.class_name)",
        "normalize_optional_text(input.lang)",
    ] {
        assert!(
            LOGIC_SOURCE.contains(needle),
            "label/logic.rs should centralize default normalization with `{needle}`."
        );
    }

    for forbidden in [
        "logic::normalize_label_text(text)",
        "logic::normalize_required_indicator(required_indicator)",
        "logic::normalize_optional_text(for_id)",
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_optional_text(lang)",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label/view.rs should not re-normalize defaults via `{forbidden}`."
        );
    }
}

#[test]
fn label_state_normalization_is_derived_in_logic_single_source() {
    for needle in [
        "pub(super) struct LabelStateAxisInput",
        "pub(super) struct LabelRenderState",
        "pub(super) fn derive_render_state(",
        "let state = resolve_state(LabelStateInput {",
        "let class_name = compose_class_name(normalized.class_name.clone(), state);",
    ] {
        assert!(
            LOGIC_SOURCE.contains(needle),
            "label/logic.rs should centralize state derivation with `{needle}`."
        );
    }

    for forbidden in [
        "logic::resolve_state(LabelStateInput {",
        "logic::compose_class_name(",
        "LabelStateInput {",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label/view.rs should not rebuild state machine pieces via `{forbidden}`."
        );
    }
}

#[test]
fn label_discrete_state_axes_are_type_constrained() {
    for needle in [
        "#[prop(optional)] emphasis: LabelEmphasis",
        "pub enum LabelEmphasis",
        "pub(super) struct LabelStateAxisInput",
        "pub emphasis: LabelEmphasis,",
    ] {
        let present = VIEW_SOURCE.contains(needle) || LOGIC_SOURCE.contains(needle);
        assert!(
            present,
            "label discrete state contract should use typed axis `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] emphasis: Option<String>",
        "#[prop(optional, into)] variant: Option<String>",
        "#[prop(optional, into)] status: Option<String>",
        "match emphasis.as_str()",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden) && !LOGIC_SOURCE.contains(forbidden),
            "label discrete state contract should not rely on string axis `{forbidden}`."
        );
    }
}

#[test]
fn label_api_stays_projection_only_without_controlled_axes() {
    for required in [
        "#[prop(optional)] is_required: bool",
        "#[prop(optional)] is_disabled: bool",
    ] {
        assert!(
            VIEW_SOURCE.contains(required),
            "label/view.rs should keep projection prop `{required}`."
        );
    }

    for forbidden in [
        "on_required_change",
        "default_required",
        "on_disabled_change",
        "default_disabled",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label/view.rs should not define pseudo-controlled axis API `{forbidden}`."
        );
    }
}

#[test]
fn label_dx_hello_world_stays_simple_and_hides_internal_wiring() {
    assert!(
        README_SOURCE.contains("## Hello World（最小可用）"),
        "label README should document hello-world path."
    );

    let heading_start = README_SOURCE
        .find("## Hello World（最小可用）")
        .expect("README should include hello-world heading");
    let hello_world_section = &README_SOURCE[heading_start..];
    let code_fence_start = hello_world_section
        .find("```rust")
        .expect("README hello-world section should include rust code fence");
    let after_code_fence = &hello_world_section[code_fence_start + "```rust".len()..];
    let code_fence_end = after_code_fence
        .find("```")
        .expect("README hello-world section should close code fence");
    let snippet = after_code_fence[..code_fence_end].trim();

    let snippet_line_count = snippet
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    assert!(
        snippet_line_count <= 5,
        "label hello-world snippet should be <=5 lines, got {snippet_line_count} lines."
    );
    assert_eq!(
        snippet, "<Label text=\"Name\".to_string() />",
        "label hello-world snippet should prefer default path without advanced props."
    );

    assert!(
        !VIEW_SOURCE.contains("ui_state_primitives"),
        "label view should not require callers to wire ui-state-primitives directly."
    );
    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional)] state_input:",
        "#[prop(optional)] headless_state:",
        "#[prop(optional)] attrs:",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label should not expose internal state object prop `{forbidden}`."
        );
    }
}

#[test]
fn label_api_is_leaf_projection_not_parent_item_container() {
    for forbidden in [
        "children: Children",
        "children: ChildrenFn",
        "#[prop(optional)] items:",
        "#[prop(optional)] labels:",
        "#[prop(optional)] titles:",
        "#[prop(optional)] panels:",
        "ItemSpec",
        "LabelItem",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label/view.rs should remain a leaf projection component; found `{forbidden}`."
        );
    }

    for forbidden in ["labels + children", "titles + panels", "<Parent><Item"] {
        assert!(
            !README_SOURCE.contains(forbidden),
            "label README should not recommend implicit composite API pattern `{forbidden}`."
        );
    }
}

#[test]
fn label_has_no_macro_micro_dragging_state_machine_contract() {
    for source in [LOGIC_SOURCE, VIEW_SOURCE, MOTION_SOURCE] {
        for forbidden in [
            "Dragging",
            "Action::DragEnd",
            "DragEnd",
            "on:pointermove",
            "on:mousemove",
            "on:touchmove",
            "request_animation_frame",
        ] {
            assert!(
                !source.contains(forbidden),
                "label should not include dragging macro/micro state-machine token `{forbidden}`."
            );
        }
    }
}

#[test]
fn label_has_no_two_pass_geometry_measurement_contract() {
    for source in [LOGIC_SOURCE, VIEW_SOURCE, MOTION_SOURCE] {
        for forbidden in [
            "Intent",
            "Measure",
            "Rectification",
            "get_bounding_client_rect",
            "offset_width",
            "offset_height",
            "client_width",
            "client_height",
            "ResizeObserver",
        ] {
            assert!(
                !source.contains(forbidden),
                "label should not include two-pass geometry rendering token `{forbidden}`."
            );
        }
    }
}

#[test]
fn label_has_no_collection_registration_protocol_contract() {
    for source in [LOGIC_SOURCE, VIEW_SOURCE, MOTION_SOURCE] {
        for forbidden in [
            "RegistrationContext",
            "Register",
            "Unregister",
            "items_order",
            "HashSet",
        ] {
            assert!(
                !source.contains(forbidden),
                "label should not include collection-registration protocol token `{forbidden}`."
            );
        }
    }

    for forbidden in ["labels + children", "titles + panels", "ItemSpec"] {
        assert!(
            !README_SOURCE.contains(forbidden),
            "label docs should not introduce collection registration shorthand `{forbidden}`."
        );
    }
}

#[test]
fn label_has_no_slot_projection_strategy_contract() {
    for source in [LOGIC_SOURCE, VIEW_SOURCE, MOTION_SOURCE] {
        for forbidden in ["Lazy", "KeepAlive", "Eager", "NotifyHidden"] {
            assert!(
                !source.contains(forbidden),
                "label should not include slot-projection strategy token `{forbidden}`."
            );
        }
    }

    for forbidden in ["Lazy/KeepAlive/Eager", "NotifyHidden"] {
        assert!(
            !README_SOURCE.contains(forbidden),
            "label docs should not introduce slot-projection strategy `{forbidden}`."
        );
    }
}

#[test]
fn label_has_no_env_stream_sampling_or_action_projection_contract() {
    for source in [LOGIC_SOURCE, VIEW_SOURCE, MOTION_SOURCE] {
        for forbidden in [
            "BreakpointChanged",
            "Action::BreakpointChanged",
            "IntersectionObserver",
            "match_media",
            "on:resize",
            "debounce",
            "throttle",
        ] {
            assert!(
                !source.contains(forbidden),
                "label should not include env-stream projection token `{forbidden}`."
            );
        }
    }
}

#[test]
fn label_has_no_event_light_cone_batch_bus_or_selector_contract() {
    for source in [LOGIC_SOURCE, VIEW_SOURCE, MOTION_SOURCE] {
        for forbidden in [
            "Context Bus",
            "ContextBus",
            "Selector",
            "SelectionState::All",
            "SelectionState",
        ] {
            assert!(
                !source.contains(forbidden),
                "label should not include event-light-cone token `{forbidden}`."
            );
        }
    }

    assert!(
        !README_SOURCE.contains("prop drilling"),
        "label docs should not introduce event-light-cone anti-pattern `prop drilling`."
    );
}

#[test]
fn label_has_no_causality_bus_trace_id_contract() {
    for source in [LOGIC_SOURCE, VIEW_SOURCE, MOTION_SOURCE] {
        for forbidden in [
            "TraceId",
            "CausalityBus",
            "broadcast",
            "subscriber",
            "publish",
        ] {
            assert!(
                !source.contains(forbidden),
                "label should not include causality-bus token `{forbidden}`."
            );
        }
    }
}

#[test]
fn label_a11y_i18n_l10n_contract_is_mounted_via_headless_and_props() {
    for needle in [
        "use ui_headless::{A11yDirection, locale_attrs};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(normalized.lang, dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "aria-disabled=state.is_disabled.then_some(\"true\")",
        "aria-hidden=\"true\"",
        "normalize_label_text(input.text)",
        "normalize_required_indicator(input.required_indicator)",
    ] {
        let found = VIEW_SOURCE.contains(needle) || LOGIC_SOURCE.contains(needle);
        assert!(
            found,
            "label should expose a11y/i18n contract marker `{needle}`."
        );
    }

    for forbidden in ["\"Label\"", "\"(required)\"", "\"Required\""] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label/view.rs should not hardcode user-visible copy `{forbidden}`."
        );
    }
}

#[test]
fn label_state_markers_are_observable_searchable_and_closed_set() {
    for needle in [
        "data-emphasis=state.emphasis_attr",
        "data-state=if state.is_required { \"required\" } else { \"optional\" }",
        "data-required=state.is_required.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-has-for=state.has_for_id.then_some(\"true\")",
        "data-label-source=state.label_source_attr",
        "data-indicator-source=state.indicator_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=motion_source",
        "aria-disabled=state.is_disabled.then_some(\"true\")",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "label/view.rs should expose stable state marker `{needle}`."
        );
    }

    for forbidden in [
        "data-state=format!(",
        "data-label-source=format!(",
        "data-indicator-source=format!(",
        "data-class-source=format!(",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label/view.rs should keep marker values enumerable, not free-form via `{forbidden}`."
        );
    }
}

#[test]
fn label_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    for needle in [
        ".ui-label[data-emphasis=\"default\"]",
        ".ui-label[data-emphasis=\"subtle\"]",
        ".ui-label[data-emphasis=\"strong\"]",
        ".ui-label[data-required=\"true\"]",
        ".ui-label[data-disabled=\"true\"]",
        ".ui-label[data-has-for=\"true\"]",
        ".ui-label[data-label-source=\"custom\"]",
        ".ui-label[data-indicator-source=\"custom\"]",
        ".ui-label[data-custom-class=\"true\"]",
    ] {
        assert!(
            STYLES_SOURCE.contains(needle),
            "label/styles.rs should use explicit state selector `{needle}`."
        );
    }

    for forbidden in [
        ":nth-child",
        ":nth-of-type",
        ".ui-label .ui-label__required",
    ] {
        assert!(
            !STYLES_SOURCE.contains(forbidden),
            "label/styles.rs should not guess state from fragile selector `{forbidden}`."
        );
    }

    assert!(
        VIEW_SOURCE.contains("style=move || motion_style.get()"),
        "label/view.rs should only pass runtime custom properties via motion style contract."
    );
    for forbidden in ["style=\"top:", "style=\"left:", "style=\"display:"] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label/view.rs should not embed business visual logic inline via `{forbidden}`."
        );
    }
}

#[test]
fn label_tests_prioritize_semantic_contract_over_visual_snapshot() {
    for required in [
        "data-state=if state.is_required { \"required\" } else { \"optional\" }",
        "data-label-source=state.label_source_attr",
        "data-indicator-source=state.indicator_source_attr",
        "data-class-source=state.class_source_attr",
        "aria-disabled=state.is_disabled.then_some(\"true\")",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            VIEW_SOURCE.contains(required),
            "label semantic contract marker `{required}` must remain testable."
        );
    }

    let insta_snapshot_macro = ["insta::assert", "_snapshot!"].concat();
    let assert_snapshot_macro = ["assert", "_snapshot!("].concat();
    for forbidden in [insta_snapshot_macro, assert_snapshot_macro] {
        assert!(
            !TEST_SOURCE.contains(&forbidden),
            "label semantics tests should not rely on visual snapshot primitive `{forbidden}`."
        );
    }
}

#[test]
fn label_file_responsibilities_remain_separated() {
    assert!(
        !MOD_SOURCE.contains("fn "),
        "label/mod.rs should not contain implementation logic."
    );

    for forbidden in ["<label", "data-state=", "style=", "ui-label__"] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden),
            "label/logic.rs should avoid view-layer concern `{forbidden}`."
        );
    }

    for forbidden in ["#[component]", "use leptos", "fn "] {
        assert!(
            !STYLES_SOURCE.contains(forbidden),
            "label/styles.rs should remain static css contract, forbidden `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "locale_attrs(normalized.lang, dir)",
        "view! {",
    ] {
        assert!(
            VIEW_SOURCE.contains(required),
            "label/view.rs should keep rendering/headless mount marker `{required}`."
        );
    }
    for forbidden in [
        "logic::resolve_state(LabelStateInput {",
        "logic::compose_class_name(",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label/view.rs should not own logic derivation `{forbidden}`."
        );
    }

    for required in [
        "pub struct LabelMotion",
        "pub fn motion_source_attr(motion: LabelMotion)",
        "pub fn attach_motion(base_vars: Option<String>, motion: LabelMotion)",
    ] {
        assert!(
            MOTION_SOURCE.contains(required),
            "label/motion.rs should keep motion-contract marker `{required}`."
        );
    }
    for forbidden in ["#[component]", "<label", "locale_attrs", "LabelStateInput"] {
        assert!(
            !MOTION_SOURCE.contains(forbidden),
            "label/motion.rs should not absorb non-motion concern `{forbidden}`."
        );
    }
}

#[test]
fn label_does_not_introduce_spec_rs_for_simple_leaf_component() {
    assert!(
        CHECK2_SOURCE.contains("- [x] Hyper-Structure Builder（`spec.rs`）："),
        "label/check2.md should mark hyper-structure-builder checklist item handled for label scope."
    );

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "label is a simple leaf component and should not introduce `src/spec.rs`."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !MOD_SOURCE.contains(forbidden),
            "label/mod.rs should not expose spec module surface `{forbidden}`."
        );
    }
}

#[test]
fn label_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let component_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["Component.toml", "label.rbi"] {
        assert!(
            component_root.join(required_file).exists(),
            "components/label/src/{required_file} should exist for context-compression projection.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "name = \"Label\"",
        "crate = \"ui-label\"",
        "name = \"text\"",
        "name = \"for_id\"",
        "name = \"is_required\"",
        "name = \"is_disabled\"",
        "name = \"emphasis\"",
        "name = \"required_indicator\"",
        "name = \"class_name\"",
        "name = \"motion\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            COMPONENT_MANIFEST_SOURCE.contains(required),
            "label Component.toml should keep manifest marker `{required}`.",
        );
    }

    for required in [
        "pub type LabelEmphasis = ui_state_primitives::label::LabelEmphasis;",
        "pub type LabelState = ui_state_primitives::label::LabelState;",
        "pub type LabelStateInput = ui_state_primitives::label::LabelStateInput;",
        "pub type A11yDirection = ui_headless::A11yDirection;",
        "pub struct LabelMotion {",
        "pub fn Label(",
        "text: Option<String>,",
        "for_id: Option<String>,",
        "is_required: bool,",
        "is_disabled: bool,",
        "emphasis: LabelEmphasis,",
        "required_indicator: Option<String>,",
        "class_name: Option<String>,",
        "motion: LabelMotion,",
        "lang: Option<String>,",
        "dir: Option<A11yDirection>,",
    ] {
        assert!(
            COMPONENT_RBI_SOURCE.contains(required),
            "label.rbi should keep signature projection marker `{required}`.",
        );
    }

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "components/label/src/Component.toml",
        "components/label/src/label.rbi",
        "label_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "crates/ui-components/tests/label_semantics.rs::label_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            CHECK2_SOURCE.contains(required),
            "label/check2.md should keep context-compression evidence `{required}`.",
        );
    }
}

#[test]
fn label_agent_contract_schema_markers_are_typed_traceable_and_whitelisted() {
    for required in [
        "pub const LABEL_AGENT_SCHEMA: &str = \"ui.label.agent-contract.v1\";",
        "pub const LABEL_AGENT_SCHEMA_VERSION: &str = \"v1\";",
        "pub enum LabelAgentIntent",
        "pub enum LabelAgentAction",
        "pub enum LabelAgentState",
        "pub enum LabelAgentSource",
        "pub struct LabelAgentContractAttrs",
        "pub fn resolve_agent_contract_attrs(",
        "typed_agent_contract_from_logic::resolve_agent_contract_attrs",
    ] {
        assert!(
            LOGIC_SOURCE.contains(required) || COMPONENT_MANIFEST_SOURCE.contains(required),
            "label agent contract should keep typed marker `{required}`.",
        );
    }

    assert!(
        !LOGIC_SOURCE.contains("format!(\"data-ui-")
            && !LOGIC_SOURCE.contains("format!(\"ui.label"),
        "label agent contract attrs should not be assembled via ad-hoc string formatting."
    );

    for required in [
        "let agent_contract = logic::resolve_agent_contract_attrs(state, motion_source);",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=agent_contract.action_attr",
        "data-ui-state=agent_contract.state_attr",
        "data-ui-source=agent_contract.source_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-label-source=agent_contract.label_source_attr",
        "data-ui-indicator-source=agent_contract.indicator_source_attr",
        "data-ui-class-source=agent_contract.class_source_attr",
        "data-ui-motion-source=agent_contract.motion_source_attr",
    ] {
        assert!(
            VIEW_SOURCE.contains(required),
            "label view should mount agent-contract semantic marker `{required}`.",
        );
    }

    for required in [
        "name = \"agent_contract_schema_markers\"",
        "[agent_contract]",
        "schema = \"ui.label.agent-contract.v1\"",
        "intent = \"form-label\"",
        "[[agent_contract_markers]]",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-output-status\"",
        "[[agent_contract_whitelist]]",
        "typed_agent_contract_from_logic::resolve_agent_contract_attrs",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            COMPONENT_MANIFEST_SOURCE.contains(required),
            "label Component.toml should keep agent-contract/whitelist marker `{required}`.",
        );
    }

    for required in [
        "pub const LABEL_AGENT_SCHEMA: &str;",
        "pub const LABEL_AGENT_SCHEMA_VERSION: &str;",
        "pub enum LabelAgentIntent",
        "pub enum LabelAgentAction",
        "pub enum LabelAgentState",
        "pub enum LabelAgentSource",
        "pub struct LabelAgentContractAttrs",
        "pub fn resolve_agent_contract_attrs(",
    ] {
        assert!(
            COMPONENT_RBI_SOURCE.contains(required),
            "label.rbi should expose typed agent-contract projection `{required}`.",
        );
    }

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "components/label/src/logic.rs",
        "components/label/src/view.rs",
        "components/label/src/Component.toml",
        "label_agent_contract_schema_markers_are_typed_traceable_and_whitelisted",
        "crates/ui-components/tests/label_semantics.rs::label_agent_contract_schema_markers_are_typed_traceable_and_whitelisted",
    ] {
        assert!(
            CHECK2_SOURCE.contains(required),
            "label/check2.md should keep agent-contract governance marker `{required}`.",
        );
    }
}

#[test]
fn label_streaming_definition_is_llm_output_only_with_two_modes() {
    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "label_streaming_definition_is_llm_output_only_with_two_modes",
        "crates/ui-components/tests/label_semantics.rs::label_streaming_definition_is_llm_output_only_with_two_modes",
    ] {
        assert!(
            CHECK2_SOURCE.contains(required),
            "label/check2.md should keep streaming-definition marker `{required}`.",
        );
    }

    for required in [
        "pub enum LabelAgentAction",
        "RenderSnapshot,",
        "pub enum LabelAgentStreamSupport",
        "Optional,",
        "pub enum LabelAgentStreamFallback",
        "Snapshot,",
    ] {
        assert!(
            LOGIC_SOURCE.contains(required),
            "label logic should keep typed streaming/snapshot marker `{required}`.",
        );
    }

    for required in [
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
    ] {
        assert!(
            VIEW_SOURCE.contains(required),
            "label view should mount streaming-definition marker `{required}`.",
        );
    }

    for forbidden in [
        "data-ui-stream-token",
        "data-ui-retry-count",
        "data-ui-reconnect",
        "on_stream_chunk",
        "on_transport_error",
    ] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden) && !VIEW_SOURCE.contains(forbidden),
            "label should not absorb transport-layer streaming token `{forbidden}`.",
        );
    }
}

#[test]
fn label_snapshot_is_baseline_capability_and_consumes_complete_result() {
    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "components/label/src/logic.rs",
        "components/label/src/Component.toml",
        "components/label/src/view.rs",
        "label_snapshot_is_baseline_capability_and_consumes_complete_result",
        "crates/ui-components/tests/label_semantics.rs::label_snapshot_is_baseline_capability_and_consumes_complete_result",
    ] {
        assert!(
            CHECK2_SOURCE.contains(required),
            "label/check2.md should keep snapshot-baseline marker `{required}`.",
        );
    }

    for required in [
        "pub enum LabelAgentAction",
        "RenderSnapshot,",
        "pub enum LabelAgentStreamFallback",
        "Snapshot,",
    ] {
        assert!(
            LOGIC_SOURCE.contains(required),
            "label logic should keep snapshot-baseline marker `{required}`.",
        );
    }

    for required in [
        "output_mode_axis = [\"snapshot\"]",
        "action_axis = [\"render-snapshot\"]",
        "name = \"snapshot_rendering\"",
        "attr = \"data-ui-output-status\"",
    ] {
        assert!(
            COMPONENT_MANIFEST_SOURCE.contains(required),
            "label Component.toml should keep snapshot contract marker `{required}`.",
        );
    }

    for required in [
        "let normalized = logic::normalize_view_input(logic::LabelViewInput {",
        "let render_state = logic::derive_render_state(",
        "data-ui-output-status=agent_contract.output_status_attr",
    ] {
        assert!(
            VIEW_SOURCE.contains(required),
            "label view should consume complete snapshot result marker `{required}`.",
        );
    }

    for forbidden in [
        "stream_chunk",
        "on_stream_chunk",
        "incremental_token",
        "partial_delta",
        "delta_patch",
    ] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden) && !VIEW_SOURCE.contains(forbidden),
            "label snapshot baseline should not depend on streaming chunk token `{forbidden}`.",
        );
    }
}

#[test]
fn label_streaming_requirement_is_optional_for_non_reader_component_with_snapshot_fallback() {
    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "label_streaming_requirement_is_optional_for_non_reader_component_with_snapshot_fallback",
        "crates/ui-components/tests/label_semantics.rs::label_streaming_requirement_is_optional_for_non_reader_component_with_snapshot_fallback",
    ] {
        assert!(
            CHECK2_SOURCE.contains(required),
            "label/check2.md should keep streaming-requirement marker `{required}`.",
        );
    }

    for required in [
        "pub enum LabelAgentStreamSupport",
        "Optional,",
        "pub enum LabelAgentStreamFallback",
        "Snapshot,",
        "pub enum LabelAgentOutputStatus",
        "Verified,",
    ] {
        assert!(
            LOGIC_SOURCE.contains(required),
            "label logic should keep optional-streaming marker `{required}`.",
        );
    }
    assert!(
        !LOGIC_SOURCE.contains("LabelAgentStreamSupport {\n    Required"),
        "label should not expose streaming-required variant for non-reader component scope.",
    );

    for required in [
        "name = \"stream_support\"",
        "attr = \"data-ui-stream-support\"",
        "values = [\"optional\"]",
        "name = \"stream_fallback\"",
        "attr = \"data-ui-stream-fallback\"",
        "values = [\"snapshot\"]",
        "attr = \"data-ui-output-status\"",
    ] {
        assert!(
            COMPONENT_MANIFEST_SOURCE.contains(required),
            "label Component.toml should keep optional-streaming contract marker `{required}`.",
        );
    }

    for required in [
        "pub enum LabelAgentStreamSupport {",
        "Optional,",
        "pub enum LabelAgentStreamFallback {",
        "Snapshot,",
        "pub enum LabelAgentOutputStatus {",
        "Verified,",
    ] {
        assert!(
            COMPONENT_RBI_SOURCE.contains(required),
            "label.rbi should project optional-streaming marker `{required}`.",
        );
    }

    for required in [
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-slot=\"label\"",
        "data-state=if state.is_required { \"required\" } else { \"optional\" }",
        "aria-disabled=state.is_disabled.then_some(\"true\")",
    ] {
        assert!(
            VIEW_SOURCE.contains(required),
            "label view should keep stream/output/aria continuity marker `{required}`.",
        );
    }

    for forbidden in [
        "retry",
        "reconnect",
        "transport",
        "on_stream_chunk",
        "data-ui-retry",
    ] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden)
                && !VIEW_SOURCE.contains(forbidden)
                && !COMPONENT_MANIFEST_SOURCE.contains(forbidden),
            "label component scope should not absorb transport/retry token `{forbidden}`.",
        );
    }
}

#[test]
fn label_styles_follow_token_first_static_css_and_no_utility_first_backflow() {
    for needle in [
        "pub const CSS: &str",
        "gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
        "font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));",
        "line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        "outline: var(--ui-border-width, var(--ui-fallback-border-width)) dashed var(--ui-accent-soft, var(--ui-fallback-accent-soft));",
        "outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));",
        "color: var(--ui-danger, var(--ui-fallback-danger));",
    ] {
        assert!(
            STYLES_SOURCE.contains(needle),
            "label/styles.rs should keep token-first css marker `{needle}`."
        );
    }

    assert!(
        VIEW_SOURCE.contains("style=move || motion_style.get()"),
        "label/view.rs should only expose runtime css custom properties through motion attach."
    );
    for forbidden in ["style=\"top:", "style=\"left:", "style=\"display:"] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label/view.rs should not carry utility-like inline style `{forbidden}`."
        );
    }

    for source in [STYLES_SOURCE, VIEW_SOURCE, LOGIC_SOURCE] {
        for forbidden in [
            " class=\"flex",
            " class=\"grid",
            " class=\"px-",
            " class=\"text-",
            "tailwind",
            "tw-",
            "css!{",
            "css!(\"",
            "styled(",
        ] {
            assert!(
                !source.contains(forbidden),
                "label component layer should not import utility-first/css-in-rust pattern `{forbidden}`."
            );
        }
    }
}

#[test]
fn label_defensive_variables_contract_uses_double_fallback_chain_and_ssot_tokens() {
    assert!(
        CHECK2_SOURCE.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "label/check2.md should mark defensive-variables checklist item completed."
    );

    for needle in [
        "text-underline-offset: var(--ui-space-3xs, var(--ui-fallback-space-3xs));",
        "outline: var(--ui-border-width, var(--ui-fallback-border-width)) dashed var(--ui-accent-soft, var(--ui-fallback-accent-soft));",
        "font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
    ] {
        assert!(
            STYLES_SOURCE.contains(needle),
            "label/styles.rs should keep defensive-variable marker `{needle}`."
        );
    }

    for line in STYLES_SOURCE.lines() {
        let trimmed = line.trim();
        if trimmed.contains("var(--ui-") {
            assert!(
                trimmed.contains("var(--ui-fallback-"),
                "label style var chain should include ui-fallback branch: `{trimmed}`."
            );
        }
    }

    for forbidden in [
        "#",
        "text-underline-offset: 0.12em;",
        "outline: 1px dashed",
        "font-size: 0.85em;",
    ] {
        assert!(
            !STYLES_SOURCE.contains(forbidden),
            "label/styles.rs should avoid hardcoded hex/raw-size token `{forbidden}`."
        );
    }

    for needle in [
        "--ui-fallback-space-3xs",
        "--ui-fallback-border-width",
        "--ui-fallback-font-size-100",
    ] {
        assert!(
            UI_THEME_CSS_SOURCE.contains(needle),
            "ui-theme SSOT should expose defensive fallback variable `{needle}`."
        );
    }
}

#[test]
fn label_cascade_layer_contract_wraps_css_in_ui_layer_and_restricts_inline_style() {
    assert!(
        CHECK2_SOURCE.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "label/check2.md should mark @layer-ui coverage checklist item completed."
    );

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-label\")]",
        "out.push_str(crate::label::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            UI_COMPONENTS_CSS_SOURCE.contains(needle),
            "ui-components css aggregator should keep cascade-layer marker `{needle}`."
        );
    }

    let layer_start = UI_COMPONENTS_CSS_SOURCE
        .find("out.push_str(\"\\n@layer ui {\\n\");")
        .expect("ui-components css should open @layer ui.");
    let label_css = UI_COMPONENTS_CSS_SOURCE
        .find("out.push_str(crate::label::styles::CSS);")
        .expect("ui-components css should include label css push.");
    let layer_end = UI_COMPONENTS_CSS_SOURCE
        .rfind("out.push_str(\"\\n}\\n\");")
        .expect("ui-components css should close @layer ui.");
    assert!(
        layer_start < label_css && label_css < layer_end,
        "label css injection must stay inside @layer ui wrapper."
    );

    assert!(
        VIEW_SOURCE.contains("style=move || motion_style.get()"),
        "label/view.rs should keep runtime style path limited to motion custom properties."
    );
    for forbidden in ["style=\"top:", "style=\"left:", "style=\"display:"] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label/view.rs should not expose ordinary inline style branch `{forbidden}`."
        );
    }

    for required in [
        "--ui-label-motion-color-duration",
        "--ui-label-motion-weight-duration",
    ] {
        assert!(
            MOTION_SOURCE.contains(required),
            "label motion attach should keep css custom property marker `{required}`."
        );
    }
    for forbidden in [
        "top:",
        "left:",
        "display:",
        "position:",
        "width:",
        "height:",
    ] {
        assert!(
            !MOTION_SOURCE.contains(forbidden),
            "label motion contract should avoid regular inline style token `{forbidden}`."
        );
    }
}

#[test]
fn label_visual_desire_baseline_is_documented_with_component_scope_boundary() {
    for needle in [
        "## Visual Desire Baseline（Label 范围）",
        "LabelEmphasis::{Default, Subtle, Strong}",
        "hover/active",
        "HeroUI 对标原则",
        "不复制 API 表层",
        "Button/Input/Overlay",
        "仓库级任务",
    ] {
        assert!(
            README_SOURCE.contains(needle),
            "label README should keep visual-desire baseline marker `{needle}`."
        );
    }

    for needle in [
        ".ui-label--for:not(.ui-label--disabled):hover",
        ".ui-label[data-has-for=\"true\"]:not([data-disabled=\"true\"]):hover",
        ".ui-label--for:not(.ui-label--disabled):active",
        ".ui-label[data-has-for=\"true\"]:not([data-disabled=\"true\"]):active",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
    ] {
        assert!(
            STYLES_SOURCE.contains(needle),
            "label/styles.rs should keep visual feedback contract `{needle}`."
        );
    }
}

#[test]
fn label_tree_shaking_contract_is_feature_gated_in_ui_components() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root should be two levels above components/label");
    let cargo_toml =
        std::fs::read_to_string(workspace_root.join("crates/ui-components/Cargo.toml"))
            .expect("should read crates/ui-components/Cargo.toml");
    let lib_source =
        std::fs::read_to_string(workspace_root.join("crates/ui-components/src/lib.rs"))
            .expect("should read crates/ui-components/src/lib.rs");
    let css_source =
        std::fs::read_to_string(workspace_root.join("crates/ui-components/src/css.rs"))
            .expect("should read crates/ui-components/src/css.rs");

    assert!(
        cargo_toml.contains("component-label = []"),
        "ui-components feature tree should register `component-label`."
    );

    for needle in [
        "#[cfg(feature = \"component-label\")]",
        "#[path = \"../../../components/label/src/mod.rs\"]",
        "pub mod label;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib gate should keep `{needle}` for label source-mode tree shaking."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-label\")]",
        "out.push_str(crate::label::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css gate should keep `{needle}` for label style tree shaking."
        );
    }

    for forbidden in [
        "crate::label::styles::CSS); out.push_str",
        "let all_components_registry =",
        "static ALL_COMPONENTS",
    ] {
        assert!(
            !css_source.contains(forbidden) && !lib_source.contains(forbidden),
            "ui-components should not introduce unconditional central registry token `{forbidden}`."
        );
    }

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "`component-label = []`",
        "`#[cfg(feature = \"component-label\")]`",
        "`#[path = \"../../../components/label/src/mod.rs\"]`",
        "`out.push_str(crate::label::styles::CSS);`",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-label,inject-css",
        "cargo tree -e features -i ui-components -p web-demo | rg all-components",
        "Invalid cross-device link (os error 18)",
        "components/label/test/semantics.rs::label_tree_shaking_contract_is_feature_gated_in_ui_components",
        "crates/ui-components/tests/label_semantics.rs::label_tree_shaking_is_feature_gated_across_cargo_lib_and_css",
    ] {
        assert!(
            CHECK2_SOURCE.contains(needle),
            "label/check2.md should keep tree-shaking feature-pruning evidence `{needle}`.",
        );
    }
}

#[test]
fn label_machine_readable_state_contract_is_type_constrained_and_locatable() {
    let primitive_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root should be two levels above components/label")
        .join("crates/ui-state-primitives/src/label.rs");
    let primitive_source =
        std::fs::read_to_string(primitive_path).expect("should read ui-state-primitives label");

    for needle in [
        "pub enum LabelEmphasis",
        "LabelEmphasis::Default => \"default\"",
        "LabelEmphasis::Subtle => \"subtle\"",
        "LabelEmphasis::Strong => \"strong\"",
        "pub(super) fn normalize_view_input(input: LabelViewInput)",
        "pub(super) fn derive_render_state(",
        "resolve_state(LabelStateInput {",
        "data-emphasis=state.emphasis_attr",
        "data-state=if state.is_required { \"required\" } else { \"optional\" }",
        "data-label-source=state.label_source_attr",
        "data-indicator-source=state.indicator_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        let found = primitive_source.contains(needle)
            || LOGIC_SOURCE.contains(needle)
            || VIEW_SOURCE.contains(needle);
        assert!(
            found,
            "machine-readable type/state contract marker `{needle}` should remain in primitive/logic/view."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] emphasis: Option<String>",
        "match emphasis.as_str()",
        "data-state=format!(",
        "data-label-source=format!(",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden) && !LOGIC_SOURCE.contains(forbidden),
            "contract should not regress to weak string protocol `{forbidden}`."
        );
    }
}

#[test]
fn label_does_not_own_overlay_focus_stack_or_noderef_restore_logic() {
    for source in [LOGIC_SOURCE, VIEW_SOURCE, MOTION_SOURCE] {
        for forbidden in [
            "NodeRef",
            "node_ref",
            "FocusManager",
            "FallbackTo",
            "document.body",
            "document().body",
            "focus_stack",
            "restore_focus",
            "Overlay",
            "overlay_stack",
        ] {
            assert!(
                !source.contains(forbidden),
                "label should stay non-overlay leaf and avoid focus-stack token `{forbidden}`."
            );
        }
    }
}

#[test]
fn label_does_not_expose_foreign_zone_or_imperative_third_party_handles() {
    for source in [LOGIC_SOURCE, VIEW_SOURCE, MOTION_SOURCE, MOD_SOURCE] {
        for forbidden in [
            "ECharts",
            "echarts",
            "Mapbox",
            "Leaflet",
            "GoogleMap",
            "ForeignZone",
            "YieldControl",
            "CleanupForeign",
            "imperative_handle",
            "third_party_instance",
            "JsValue",
            "web_sys::HtmlCanvasElement",
        ] {
            assert!(
                !source.contains(forbidden),
                "label should not carry foreign-zone or imperative third-party token `{forbidden}`."
            );
        }
    }
}

#[test]
fn label_ssr_hydration_path_stays_deterministic_without_time_or_random_id_init() {
    for source in [LOGIC_SOURCE, VIEW_SOURCE, MOTION_SOURCE, MOD_SOURCE] {
        for forbidden in [
            "now()",
            "Instant::now",
            "SystemTime",
            "UNIX_EPOCH",
            "Uuid",
            "uuid::",
            "rand::",
            "thread_rng",
            "random()",
            "getrandom",
            "id_seed",
            "IdProvider",
            "provide_ui_id_provider",
            "hydrate",
            "hydration_id",
        ] {
            assert!(
                !source.contains(forbidden),
                "label should avoid nondeterministic ssr init token `{forbidden}`."
            );
        }
    }
}

#[test]
fn label_headless_web_ssr_feature_mutex_guard_remains_enforced() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root should be two levels above components/label");
    let ui_headless_lib =
        std::fs::read_to_string(workspace_root.join("crates/ui-headless/src/lib.rs"))
            .expect("should read crates/ui-headless/src/lib.rs");
    let ui_headless_cargo =
        std::fs::read_to_string(workspace_root.join("crates/ui-headless/Cargo.toml"))
            .expect("should read crates/ui-headless/Cargo.toml");
    let ui_components_cargo =
        std::fs::read_to_string(workspace_root.join("crates/ui-components/Cargo.toml"))
            .expect("should read crates/ui-components/Cargo.toml");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_lib.contains(needle),
            "ui-headless should enforce web/ssr mutex guard `{needle}`.",
        );
    }

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            ui_headless_cargo.contains(needle),
            "ui-headless Cargo feature contract should keep `{needle}`.",
        );
    }

    assert!(
        VIEW_SOURCE.contains("use ui_headless::{A11yDirection, locale_attrs};"),
        "label should consume ui-headless contract through typed API, not feature rewiring.",
    );

    for forbidden in [
        "ui-headless/web",
        "ui-headless/ssr",
        "features = [\"web\", \"ssr\"]",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "ui-components should not override ui-headless mutex boundary via `{forbidden}`.",
        );
    }
}

#[test]
fn label_ssr_cross_platform_path_avoids_direct_browser_binding_and_uses_motion_cfg_backend() {
    for source in [MOD_SOURCE, LOGIC_SOURCE, VIEW_SOURCE, MOTION_SOURCE] {
        for forbidden in [
            "web_sys",
            "js_sys",
            "wasm_bindgen",
            "window()",
            "document()",
            "HtmlElement",
        ] {
            assert!(
                !source.contains(forbidden),
                "label source should avoid direct browser binding `{forbidden}`."
            );
        }
    }

    assert!(
        MOTION_SOURCE.contains("ui_motion::web::prefers_reduced_motion();"),
        "label motion should route platform behavior via ui_motion web adapter."
    );

    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root should be two levels above components/label");
    let ui_motion_lib = std::fs::read_to_string(workspace_root.join("crates/ui-motion/src/lib.rs"))
        .expect("should read crates/ui-motion/src/lib.rs");
    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion cross-platform backend contract marker `{needle}` should remain."
        );
    }
}

#[test]
fn label_motion_contract_covers_reduced_motion_ssr_and_wasm_without_semantic_split() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root should be two levels above components/label");
    let ui_motion_lib = std::fs::read_to_string(workspace_root.join("crates/ui-motion/src/lib.rs"))
        .expect("should read crates/ui-motion/src/lib.rs");

    for needle in [
        "pub fn sanitize_motion(motion: LabelMotion) -> LabelMotion",
        "pub fn motion_source_attr(motion: LabelMotion) -> &'static str",
        "let reduced_motion = ui_motion::web::prefers_reduced_motion();",
        "let color_transition_ms = if reduced_motion {",
        "let weight_transition_ms = if reduced_motion {",
        "MIN_DURATION_MS",
        "MAX_DURATION_MS",
        "--ui-label-motion-color-duration",
        "--ui-label-motion-weight-duration",
    ] {
        assert!(
            MOTION_SOURCE.contains(needle),
            "label motion should keep reduced-motion fallback marker `{needle}`."
        );
    }

    for needle in [
        "data-motion-source=motion_source",
        "style=move || motion_style.get()",
        "data-state=if state.is_required { \"required\" } else { \"optional\" }",
        "data-label-source=state.label_source_attr",
        "data-indicator-source=state.indicator_source_attr",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "label view should keep platform-stable semantic marker `{needle}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "cfg!(target_arch = \"wasm32\")",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label view should not split semantics by platform via `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web;",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion should keep wasm/non-wasm backend split marker `{needle}`."
        );
    }

    assert!(
        CHECK2_SOURCE.contains("- [x] Motion 合同化："),
        "label/check2.md should mark motion-contract checklist item completed."
    );
}

#[test]
fn label_performance_budget_contract_stays_traceable_and_predictable() {
    assert!(
        CHECK2_SOURCE.contains(
            "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。"
        ),
        "label/check2.md should mark performance-governance checklist item completed."
    );

    let derive_count = VIEW_SOURCE.matches("Signal::derive(").count();
    assert_eq!(
        derive_count, 1,
        "label view should keep a single derived render path; got {derive_count}."
    );
    assert!(
        VIEW_SOURCE.contains(
            "let motion_style = Signal::derive(move || motion::attach_motion(None, motion));"
        ),
        "label view should keep motion derive as the only runtime style derivation."
    );

    for forbidden in [
        "on:",
        "create_effect(",
        "create_resource(",
        "spawn_local(",
        "request_animation_frame",
        "set_interval",
        "set_timeout",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label view should avoid runtime update loop token `{forbidden}`."
        );
    }

    for forbidden in [
        "web::animate(",
        "request_animation_frame",
        "AnimationHandle",
    ] {
        assert!(
            !MOTION_SOURCE.contains(forbidden),
            "label motion should not own imperative frame/handle loop `{forbidden}`."
        );
    }

    for forbidden in ["create_effect(", "spawn_local(", "tokio::", "async fn"] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden),
            "label logic should stay pure/traceable without async side path `{forbidden}`."
        );
    }
}

#[test]
fn label_semantic_and_performance_regression_contract_is_scoped_and_traceable() {
    for needle in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "components/label/test/semantics.rs::label_a11y_i18n_l10n_contract_is_mounted_via_headless_and_props",
        "components/label/test/semantics.rs::label_state_markers_are_observable_searchable_and_closed_set",
        "crates/ui-components/tests/label_semantics.rs::label_exposes_a11y_i18n_l10n_hooks_without_hardcoded_view_copy",
        "crates/ui-components/tests/label_semantics.rs::label_state_markers_remain_observable_and_enumerated",
        "components/label/test/semantics.rs::label_does_not_own_overlay_focus_stack_or_noderef_restore_logic",
        "crates/ui-components/tests/label_semantics.rs::label_remains_outside_overlay_focus_stack_responsibility",
        "components/label/test/semantics.rs::label_performance_budget_contract_stays_traceable_and_predictable",
        "crates/ui-components/tests/label_semantics.rs::label_performance_budget_contract_uses_source_level_budget_baseline",
        "Signal::derive` 单路径（预算=1）",
        "render_count` 精确计数在当前测试栈仍属仓库级能力",
        "`Label` 非高频/重型组件按清单边界以等价证据通过",
    ] {
        assert!(
            CHECK2_SOURCE.contains(needle),
            "label/check2 semantic-and-performance evidence should keep `{needle}`.",
        );
    }
}

#[test]
fn label_semantic_test_priority_regression_contract_is_scoped_and_traceable() {
    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "components/label/test/semantics.rs::label_a11y_i18n_l10n_contract_is_mounted_via_headless_and_props",
        "components/label/test/semantics.rs::label_state_markers_are_observable_searchable_and_closed_set",
        "components/label/test/semantics.rs::label_tests_prioritize_semantic_contract_over_visual_snapshot",
        "crates/ui-components/tests/label_semantics.rs::label_exposes_a11y_i18n_l10n_hooks_without_hardcoded_view_copy",
        "crates/ui-components/tests/label_semantics.rs::label_state_markers_remain_observable_and_enumerated",
        "crates/ui-components/tests/label_semantic_contract_tests_are_primary_not_visual_snapshot_only",
        "label_api_is_leaf_projection_not_parent_item_container",
        "label_does_not_own_overlay_focus_stack_or_noderef_restore_logic",
        "label_machine_readable_state_contract_is_type_constrained_and_locatable",
        "label_agent_contract_schema_markers_are_typed_traceable_and_whitelisted",
        "components/label/test/semantics.rs::label_semantic_test_priority_regression_contract_is_scoped_and_traceable",
        "crates/ui-components/tests/label_semantics.rs::label_semantic_test_priority_regression_contract_is_scoped_and_traceable",
    ] {
        assert!(
            CHECK2_SOURCE.contains(needle),
            "label/check2 semantic-test-priority evidence should keep `{needle}`.",
        );
    }
}

#[test]
fn label_e2e_selector_stability_contract_uses_semantic_markers_and_wasm_ready_waits() {
    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "e2e/tests/docs_app_label_contract.spec.mjs",
        "docs-app label uses semantic selectors with wasm-stable readiness waits",
        "docs-app label key flow is repeatable with semantic focus breakpoints",
        "docs-app label streaming/snapshot markers stay settled without fixed sleeps",
        "body:not(:has(#boot))",
        "data-ui-output-status=\"verified\"",
        "components/label/test/semantics.rs::label_e2e_selector_stability_contract_uses_semantic_markers_and_wasm_ready_waits",
        "crates/ui-components/tests/label_semantics.rs::label_e2e_selector_stability_contract_uses_semantic_markers_and_wasm_ready_waits",
    ] {
        assert!(
            CHECK2_SOURCE.contains(needle),
            "label/check2 e2e-selector-stability evidence should keep `{needle}`.",
        );
    }

    for needle in [
        "test(\"docs-app label uses semantic selectors with wasm-stable readiness waits\"",
        "test(\"docs-app label key flow is repeatable with semantic focus breakpoints\"",
        "test(\"docs-app label streaming/snapshot markers stay settled without fixed sleeps\"",
        "body:not(:has(#boot))",
        "data-slot=\"label\"",
        "data-ui-output-status=\"verified\"",
        "data-ui-stream-fallback=\"snapshot\"",
        "toBeFocused()",
    ] {
        assert!(
            E2E_LABEL_CONTRACT_SOURCE.contains(needle),
            "label e2e contract should keep semantic selector marker `{needle}`.",
        );
    }

    for forbidden in ["waitForTimeout(", "wait_for_timeout(", "setTimeout("] {
        assert!(
            !E2E_LABEL_CONTRACT_SOURCE.contains(forbidden),
            "label e2e contract should avoid fixed-sleep wait token `{forbidden}`.",
        );
    }
}

#[test]
fn label_key_flow_regression_collection_is_repeatable_and_semantically_locatable() {
    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "e2e/tests/docs_app_label_contract.spec.mjs",
        "docs-app label key flow is repeatable with semantic focus breakpoints",
        "docs-app label uses semantic selectors with wasm-stable readiness waits",
        "docs-app label streaming/snapshot markers stay settled without fixed sleeps",
        "focus/keyboard",
        "overlay/async",
        "components/label/test/semantics.rs::label_key_flow_regression_collection_is_repeatable_and_semantically_locatable",
        "crates/ui-components/tests/label_semantics.rs::label_key_flow_regression_collection_is_repeatable_and_semantically_locatable",
    ] {
        assert!(
            CHECK2_SOURCE.contains(needle),
            "label/check2 key-flow-regression evidence should keep `{needle}`.",
        );
    }

    for needle in [
        "test(\"docs-app label key flow is repeatable with semantic focus breakpoints\"",
        "gotoLabelDocsAndWaitSettled",
        "data-ui-output-status=\"verified\"",
        "focusLabel.click()",
        "toBeFocused()",
        "page.keyboard.type(\"owner-one\")",
        "page.reload()",
        "page.keyboard.type(\"owner-two\")",
    ] {
        assert!(
            E2E_LABEL_CONTRACT_SOURCE.contains(needle),
            "label e2e key-flow contract should keep marker `{needle}`.",
        );
    }

    for forbidden in ["waitForTimeout(", "wait_for_timeout(", "setTimeout("] {
        assert!(
            !E2E_LABEL_CONTRACT_SOURCE.contains(forbidden),
            "label key-flow e2e should avoid fixed-sleep token `{forbidden}`.",
        );
    }
}

#[test]
fn label_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade() {
    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Label` 改动未引入跨大版本 API 破坏升级",
        "LabelComponentSchemaVersion::V1",
        "schema_version = \"1\"",
        "ui.label.agent-contract.v1",
        "components/label/test/semantics.rs::label_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
        "crates/ui-components/tests/label_semantics.rs::label_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            CHECK2_SOURCE.contains(needle),
            "label/check2 version-deprecation migration evidence should keep `{needle}`.",
        );
    }

    for required in [
        "pub enum LabelComponentSchemaVersion",
        "V1",
        "schema_version = \"1\"",
        "schema = \"ui.label.agent-contract.v1\"",
    ] {
        assert!(
            PROTOCOL_SOURCE.contains(required) || COMPONENT_MANIFEST_SOURCE.contains(required),
            "label protocol/manifest should keep v1 contract marker `{required}`.",
        );
    }

    for source in [
        LOGIC_SOURCE,
        VIEW_SOURCE,
        PROTOCOL_SOURCE,
        COMPONENT_MANIFEST_SOURCE,
    ] {
        for forbidden in [
            "V2",
            "migrate_v1_to_v2",
            "deprecation_window",
            "schema_registry",
            "#[deprecated",
            "deprecated(",
        ] {
            assert!(
                !source.contains(forbidden),
                "label source should not carry migration token `{forbidden}` without a major break.",
            );
        }
    }
}

#[test]
fn label_view_macro_complexity_stays_small_for_leaf_structure() {
    assert!(
        CHECK2_SOURCE.contains("- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。"),
        "label/check2.md should mark view-macro-complexity checklist item completed."
    );

    let view_macro_count = VIEW_SOURCE.matches("view! {").count();
    assert_eq!(
        view_macro_count, 1,
        "label/view.rs should keep a single `view!` block; got {view_macro_count}.",
    );

    for needle in [
        "<label",
        "<span class=\"ui-label__text\" data-slot=\"label-text\">",
        "render_required_indicator(required_indicator, state.is_required)",
        "<span class=\"ui-label__required\" data-slot=\"label-required\" aria-hidden=\"true\">",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "label/view.rs should keep explicit small semantic segment `{needle}`."
        );
    }

    assert_eq!(
        VIEW_SOURCE.matches("data-slot=\"label-text\"").count(),
        1,
        "label text segment should appear exactly once.",
    );
    assert_eq!(
        VIEW_SOURCE.matches("data-slot=\"label-required\"").count(),
        1,
        "label required segment should appear exactly once.",
    );

    for forbidden in [
        "<For ",
        "<Suspense",
        "<Transition",
        "<Portal",
        "view! { view! {",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label/view.rs should avoid macro-complexity expansion token `{forbidden}`."
        );
    }
}

#[test]
fn label_prefers_function_fragment_split_without_extra_component_noise() {
    assert!(
        CHECK2_SOURCE.contains("- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。"),
        "label/check2.md should mark function-split checklist item completed."
    );

    for needle in [
        "fn render_required_indicator(required_indicator: String, is_required: bool) -> impl IntoView",
        "<Show when=move || is_required>",
        "render_required_indicator(required_indicator, state.is_required)",
        "data-slot=\"label-required\"",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "label/view.rs should keep function-split marker `{needle}`."
        );
    }

    let component_attr_count = VIEW_SOURCE.matches("#[component]").count();
    assert_eq!(
        component_attr_count, 1,
        "label/view.rs should keep a single component entry; got {component_attr_count}.",
    );
}

#[test]
fn label_static_fragment_scope_stays_lightweight_without_heavy_template_payload() {
    assert!(
        CHECK2_SOURCE.contains("- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。"),
        "label/check2.md should mark static-fragment-constantization item completed."
    );

    for forbidden in ["<svg", "inner_html=", "<footer", "markdown", "<article"] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label/view.rs should not carry heavy static fragment token `{forbidden}`."
        );
    }

    assert_eq!(
        VIEW_SOURCE.matches("data-slot=\"label-text\"").count(),
        1,
        "label static text slot should remain single-source."
    );
    assert_eq!(
        VIEW_SOURCE.matches("data-slot=\"label-required\"").count(),
        1,
        "label static required slot should remain single-source."
    );
}

#[test]
fn label_inner_html_security_contract_disallows_dynamic_html_injection_path() {
    assert!(
        CHECK2_SOURCE.contains("- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。"),
        "label/check2.md should mark inner_html security checklist item completed."
    );

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "Html::from(",
        "from_html(",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label/view.rs should not expose dynamic html injection token `{forbidden}`."
        );
    }
}

#[test]
fn label_wasm_debug_contract_is_na_but_traceable_and_feature_isolated() {
    assert!(
        CHECK2_SOURCE.contains("- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。"),
        "label/check2.md should mark wasm-debug checklist item completed."
    );
    assert!(
        CHECK2_SOURCE.contains("调试开关默认不进入生产包体与公共 API"),
        "label/check2.md should keep production-isolation wasm-debug guard text."
    );

    for needle in [
        "data-state=if state.is_required { \"required\" } else { \"optional\" }",
        "data-label-source=state.label_source_attr",
        "data-indicator-source=state.indicator_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "label/view.rs should keep wasm-debug traceability marker `{needle}`."
        );
    }

    for source in [LOGIC_SOURCE, VIEW_SOURCE, MOTION_SOURCE, MOD_SOURCE] {
        for forbidden in [
            "request_replay",
            "trace.emit(",
            "UiTrace",
            "provide_ui_trace(",
            "debug_overlay_enabled",
            "data-slot=\"label-debug-replay\"",
        ] {
            assert!(
                !source.contains(forbidden),
                "label implementation should not leak component-local wasm-debug runtime `{forbidden}`."
            );
        }
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            UI_COMPONENTS_CARGO_SOURCE.contains(needle),
            "workspace should keep explicit wasm-debug feature isolation marker `{needle}`."
        );
    }
    assert!(
        !UI_COMPONENTS_CARGO_SOURCE.contains("label-wasm-debug"),
        "label should not expose dedicated wasm-debug feature in ui-components feature surface."
    );
    for forbidden in ["wasm-debug", "debug", "replay"] {
        assert!(
            !COMPONENT_CARGO_SOURCE.contains(forbidden),
            "label crate should not expose local debug feature token `{forbidden}`."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(",
        "data-slot=\"ui-debug-overlay\"",
        "trace.emit(",
        "provide_ui_trace(debug_overlay_enabled);",
    ] {
        assert!(
            UI_DEBUG_OVERLAY_SOURCE.contains(needle) || DOCS_APP_LIB_SOURCE.contains(needle),
            "docs-app should keep shared wasm debug visual entry marker `{needle}`."
        );
    }

    for needle in [
        "pub fn use_ui_trace() -> UiTrace",
        "pub fn provide_ui_trace(enabled: bool)",
        "pub fn events(&self) -> Vec<UiTraceEvent>",
        "ts_ms",
    ] {
        assert!(
            UI_TRACE_SOURCE.contains(needle),
            "ui-headless trace contract should keep timestamped event marker `{needle}`."
        );
    }
}

#[test]
fn label_dx_contract_uses_hot_css_playground_and_isolated_workbench_with_optional_persist_na() {
    assert!(
        CHECK2_SOURCE.contains("- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。"),
        "label/check2.md should mark DX checklist item completed."
    );

    for required in [
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
    ] {
        assert!(
            CHECK2_SOURCE.contains(required),
            "label/check2.md should keep DX governance detail `{required}`."
        );
    }

    for needle in [
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "data-playground-scope=scope_id.clone()",
        "data-slot=\"playground-test\"",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Restore original CSS\"",
    ] {
        assert!(
            PLAYGROUND_SOURCE.contains(needle),
            "playground should keep DX hot-style iteration marker `{needle}`."
        );
    }

    let label_docs_start = FORMS_EXTRA_SOURCE
        .find("pub(super) fn label() -> AnyView")
        .expect("forms_extra.rs should contain label docs section");
    let label_docs_end = FORMS_EXTRA_SOURCE
        .find("pub(super) fn field() -> AnyView")
        .expect("forms_extra.rs should contain field docs section after label");
    let label_docs = &FORMS_EXTRA_SOURCE[label_docs_start..label_docs_end];

    for needle in [
        "title=\"Interactive Playground\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "id_base=\"docs-label-workbench-emphasis\".to_string()",
        "<Switch checked=is_required set_checked=set_is_required>",
        "<Switch checked=is_disabled set_checked=set_is_disabled>",
        "id=\"docs-label-workbench-input\"",
        "Comparison (Strong + Required + Custom Indicator)",
    ] {
        assert!(
            label_docs.contains(needle),
            "label docs should keep isolated workbench/context marker `{needle}`."
        );
    }

    for forbidden in [
        "LABEL_WORKBENCH_STORAGE_KEY",
        "load_label_workbench_state(",
        "save_label_workbench_state(",
        "clear_label_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !label_docs.contains(forbidden),
            "label keeps optional persisted state as N/A; `{forbidden}` should remain absent."
        );
    }
}

#[test]
fn label_docs_product_contract_is_copy_paste_ready_with_playground_matrix_and_imports() {
    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs::label()",
        "code_imports=label_imports.clone()",
        "Controlled vs Uncontrolled (N/A for Label)",
        "Streaming Optional (fallback=snapshot)",
        "data-slot=\"label-source-first\"",
        "compose_copy_ready_code",
        "components/label/test/semantics.rs::label_docs_product_contract_is_copy_paste_ready_with_playground_matrix_and_imports",
        "crates/ui-components/tests/label_semantics.rs::label_docs_product_contract_is_copy_paste_ready_with_playground_matrix_and_imports",
    ] {
        assert!(
            CHECK2_SOURCE.contains(needle),
            "label/check2 docs-product evidence should keep `{needle}`."
        );
    }

    let label_docs_start = FORMS_EXTRA_SOURCE
        .find("pub(super) fn label() -> AnyView")
        .expect("forms_extra.rs should contain label docs section");
    let label_docs_end = FORMS_EXTRA_SOURCE
        .find("pub(super) fn field() -> AnyView")
        .expect("forms_extra.rs should contain field docs section after label");
    let label_docs = &FORMS_EXTRA_SOURCE[label_docs_start..label_docs_end];

    for needle in [
        "let label_imports =",
        "use leptos::prelude::*;\\nuse ui_components::{Label, LabelEmphasis};",
        "title=\"Hello World\"",
        "title=\"Interactive Playground\"",
        "title=\"Controlled vs Uncontrolled (N/A for Label)\"",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "code_imports=label_imports.clone()",
        "fallback=snapshot",
        "data-slot=\"label-source-first\"",
        "label=\"Copy starter\".to_string()",
        "copyable=true",
        "use ui_components::{Label, LabelEmphasis};",
        "<code>\"apps/docs-app/src/playground.rs::compose_copy_ready_code\"</code>",
    ] {
        assert!(
            label_docs.contains(needle),
            "label docs product contract should keep `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "code_imports: Option<String>",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
    ] {
        assert!(
            PLAYGROUND_SOURCE.contains(needle),
            "playground copy-ready import glue should keep `{needle}`.",
        );
    }
}

#[test]
fn label_source_first_docs_are_copy_paste_ready_with_real_paths_prerequisites_and_synced_code() {
    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "data-slot=\"label-source-first\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "components/label/src/{mod,logic,view,styles,motion}.rs",
        "`component-label`/`inject-css`",
        "components/label/test/semantics.rs::label_source_first_docs_are_copy_paste_ready_with_real_paths_prerequisites_and_synced_code",
        "crates/ui-components/tests/label_semantics.rs::label_source_first_docs_are_copy_paste_ready_with_real_paths_prerequisites_and_synced_code",
    ] {
        assert!(
            CHECK2_SOURCE.contains(needle),
            "label/check2 source-first copy-paste evidence should keep `{needle}`.",
        );
    }

    let label_docs_start = FORMS_EXTRA_SOURCE
        .find("pub(super) fn label() -> AnyView")
        .expect("forms_extra.rs should contain label docs section");
    let label_docs_end = FORMS_EXTRA_SOURCE
        .find("pub(super) fn field() -> AnyView")
        .expect("forms_extra.rs should contain field docs section after label");
    let label_docs = &FORMS_EXTRA_SOURCE[label_docs_start..label_docs_end];

    for needle in [
        "data-slot=\"label-source-first\"",
        "<h3>\"Source-first / Copy-Paste Ready\"</h3>",
        "<code>\"apps/docs-app/src/playground.rs::compose_copy_ready_code\"</code>",
        "label=\"Copy starter\".to_string()",
        "copyable=true",
        "<li><code>\"components/label/src/mod.rs\"</code></li>",
        "<li><code>\"components/label/src/logic.rs\"</code></li>",
        "<li><code>\"components/label/src/view.rs\"</code></li>",
        "<li><code>\"components/label/src/styles.rs\"</code></li>",
        "<li><code>\"components/label/src/motion.rs\"</code></li>",
        "<ul data-slot=\"label-source-prerequisites\">",
        "<li><code>\"component-label\"</code></li>",
        "<li><code>\"inject-css\"</code></li>",
        "code_signal=workbench_code",
        "code_imports=label_imports.clone()",
    ] {
        assert!(
            label_docs.contains(needle),
            "label docs source-first copy-paste contract should keep `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "code_imports: Option<String>",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
    ] {
        assert!(
            PLAYGROUND_SOURCE.contains(needle),
            "playground source-first copy-ready glue should keep `{needle}`.",
        );
    }
}

#[test]
fn label_heroui_strategy_and_component_docs_sync_contract_is_documented_and_indexable() {
    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "### Label 同步记录（2026-02-21）",
        "component_doc!(\"Label\", \"label\", \"Forms\", forms_extra::label)",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs::label()",
        "components/label/src/README.md",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "components/label/test/semantics.rs::label_heroui_strategy_and_component_docs_sync_contract_is_documented_and_indexable",
        "crates/ui-components/tests/label_semantics.rs::label_heroui_strategy_and_component_docs_sync_contract_is_documented_and_indexable",
    ] {
        assert!(
            CHECK2_SOURCE.contains(needle),
            "label/check2 heroui-doc-sync evidence should keep `{needle}`.",
        );
    }

    for needle in [
        "### Label 同步记录（2026-02-21）",
        "参数模型同步：`Label` 维持 form primitive 定位",
        "docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!(\"Label\", \"label\", \"Forms\", forms_extra::label)` 暴露入口；`#/components/label` 可索引访问",
        "示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_extra.rs::label()` 已覆盖",
        "Source-first / Copy-Paste Ready：Label playground 代码继续通过 `code_imports=label_imports.clone()` 与 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 输出可运行片段",
        "研究文档补充判定：本轮仅为 Label 参数模型与组件文档同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。",
        "HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。",
    ] {
        assert!(
            HEROUI_PARAMETER_STRATEGY_SOURCE.contains(needle),
            "heroui strategy doc should keep label-sync marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(\"Label\", \"label\", \"Forms\", forms_extra::label)",
        "pub(super) fn label() -> AnyView",
        "slug=\"label\"",
        "## Docs Playground（展示 / Config / Code / CSS Test）",
    ] {
        assert!(
            COMPONENT_PAGES_SOURCE.contains(needle)
                || FORMS_EXTRA_SOURCE.contains(needle)
                || README_SOURCE.contains(needle),
            "label docs entry sync contract should keep marker `{needle}`.",
        );
    }
}

#[test]
fn label_docs_sync_contract_covers_examples_matrices_and_logic_api_names() {
    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs::label()",
        "Interactive Playground",
        "Emphasis + Required",
        "Custom Indicator + Class",
        "Controlled vs Uncontrolled (N/A for Label)",
        "Streaming Optional (fallback=snapshot)",
        "is_required/is_disabled/emphasis/for_id/required_indicator/class_name",
        "components/label/test/semantics.rs::label_docs_sync_contract_covers_examples_matrices_and_logic_api_names",
        "crates/ui-components/tests/label_semantics.rs::label_docs_sync_contract_covers_examples_matrices_and_logic_api_names",
    ] {
        assert!(
            CHECK2_SOURCE.contains(needle),
            "label/check2 docs-sync evidence should keep `{needle}`.",
        );
    }

    let label_docs_start = FORMS_EXTRA_SOURCE
        .find("pub(super) fn label() -> AnyView")
        .expect("forms_extra.rs should contain label docs section");
    let label_docs_end = FORMS_EXTRA_SOURCE
        .find("pub(super) fn field() -> AnyView")
        .expect("forms_extra.rs should contain field docs section after label");
    let label_docs = &FORMS_EXTRA_SOURCE[label_docs_start..label_docs_end];

    for needle in [
        "title=\"Interactive Playground\"",
        "title=\"Emphasis + Required\"",
        "title=\"Custom Indicator + Class\"",
        "title=\"Controlled vs Uncontrolled (N/A for Label)\"",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "is_required=is_required.get()",
        "is_disabled=is_disabled.get()",
        "emphasis=selected_emphasis.get()",
        "for_id=if has_for_id.get()",
        "required_indicator=if custom_indicator.get()",
        "class_name=if custom_class.get()",
    ] {
        assert!(
            label_docs.contains(needle),
            "label docs should keep synchronized matrix/api marker `{needle}`.",
        );
    }

    for required in [
        "pub(super) struct LabelStateAxisInput",
        "pub emphasis: LabelEmphasis",
        "pub is_required: bool",
        "pub is_disabled: bool",
        "pub(super) fn normalize_view_input(",
        "pub(super) fn derive_render_state(",
    ] {
        assert!(
            LOGIC_SOURCE.contains(required),
            "label logic should keep api normalization marker `{required}`.",
        );
    }

    for forbidden in [
        "on_open_change",
        "default_open",
        "is_open=",
        "variant=",
        "size=",
    ] {
        assert!(
            !label_docs.contains(forbidden),
            "label docs should not drift to non-label api token `{forbidden}`.",
        );
    }
}

#[test]
fn label_interactive_playground_contract_supports_live_controls_and_repeatable_acceptance_flow() {
    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs::label()",
        "title=\"Interactive Playground\"",
        "test_config_signal=workbench_actual_config",
        "test_css_source=workbench_test_css_source",
        "AI Spec 子条对 `Label` 为 N/A",
        "docs-app label uses semantic selectors with wasm-stable readiness waits",
        "docs-app label key flow is repeatable with semantic focus breakpoints",
        "components/label/test/semantics.rs::label_interactive_playground_contract_supports_live_controls_and_repeatable_acceptance_flow",
        "crates/ui-components/tests/label_semantics.rs::label_interactive_playground_contract_supports_live_controls_and_repeatable_acceptance_flow",
    ] {
        assert!(
            CHECK2_SOURCE.contains(needle),
            "label/check2 interactive-playground evidence should keep `{needle}`.",
        );
    }

    let label_docs_start = FORMS_EXTRA_SOURCE
        .find("pub(super) fn label() -> AnyView")
        .expect("forms_extra.rs should contain label docs section");
    let label_docs_end = FORMS_EXTRA_SOURCE
        .find("pub(super) fn field() -> AnyView")
        .expect("forms_extra.rs should contain field docs section after label");
    let label_docs = &FORMS_EXTRA_SOURCE[label_docs_start..label_docs_end];

    for needle in [
        "title=\"Interactive Playground\"",
        "description=\"展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "<Switch checked=is_required set_checked=set_is_required>",
        "<Switch checked=is_disabled set_checked=set_is_disabled>",
        "<Switch checked=has_for_id set_checked=set_has_for_id>",
        "<Switch checked=custom_text set_checked=set_custom_text>",
        "<Switch checked=custom_indicator set_checked=set_custom_indicator>",
        "<Switch checked=custom_class set_checked=set_custom_class>",
        "id_base=\"docs-label-workbench-emphasis\".to_string()",
    ] {
        assert!(
            label_docs.contains(needle),
            "label docs interactive-playground contract should keep `{needle}`.",
        );
    }

    for needle in [
        "docs-app label uses semantic selectors with wasm-stable readiness waits",
        "docs-app label key flow is repeatable with semantic focus breakpoints",
    ] {
        assert!(
            E2E_LABEL_CONTRACT_SOURCE.contains(needle),
            "label e2e contract should keep repeatable interactive-playground flow marker `{needle}`.",
        );
    }
}

#[test]
fn label_documentation_as_product_contract_is_beginner_friendly_and_progressive() {
    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "components/label/src/README.md",
        "## Hello World（最小可用）",
        "<Label text=\"Name\".to_string() />",
        "默认路径无需手动接线 `ui-state-primitives` / `ui-headless`",
        "进阶需求再按需开启 `emphasis`、`required_indicator`、`lang/dir`、`class_name`",
        "## Docs Playground（展示 / Config / Code / CSS Test）",
        "components/label/test/semantics.rs::label_documentation_as_product_contract_is_beginner_friendly_and_progressive",
        "crates/ui-components/tests/label_semantics.rs::label_documentation_as_product_contract_is_beginner_friendly_and_progressive",
    ] {
        assert!(
            CHECK2_SOURCE.contains(needle),
            "label/check2 beginner-docs evidence should keep `{needle}`.",
        );
    }

    for needle in [
        "# Label",
        "## Hello World（最小可用）",
        "<Label text=\"Name\".to_string() />",
        "默认路径无需手动接线 `ui-state-primitives` / `ui-headless`。",
        "进阶需求再按需开启 `emphasis`、`required_indicator`、`lang/dir`、`class_name`。",
        "## Docs Playground（展示 / Config / Code / CSS Test）",
    ] {
        assert!(
            README_SOURCE.contains(needle),
            "label README should keep beginner-friendly docs marker `{needle}`.",
        );
    }
}

#[test]
fn label_engineering_contract_marks_spec_path_na_and_keeps_tracing_runtime_boundaries_clean() {
    assert!(
        CHECK2_SOURCE.contains("- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。"),
        "label/check2.md should mark engineering checklist item completed."
    );
    for required in [
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            CHECK2_SOURCE.contains(required),
            "label/check2.md should keep engineering governance detail `{required}`."
        );
    }

    assert!(
        MOD_SOURCE.contains("pub use view::Label;"),
        "label/mod.rs should keep narrow public export boundary."
    );
    for forbidden in ["mod protocol;", "pub mod protocol;", "pub use protocol::"] {
        assert!(
            !MOD_SOURCE.contains(forbidden),
            "label public boundary should keep spec/protocol input out of public API via `{forbidden}`."
        );
    }

    for source in [
        MOD_SOURCE,
        LOGIC_SOURCE,
        VIEW_SOURCE,
        STYLES_SOURCE,
        MOTION_SOURCE,
    ] {
        for forbidden in [
            "serde::",
            "serde_json::",
            "tracing::span!(",
            "tracing::event!(",
            "#[tracing::instrument]",
            "target: \"ui_components::label::",
            "tokio::",
            "async_std::",
            "async-std",
            "runtime::Handle",
            "smol::",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "label implementation should avoid engineering-boundary leak token `{forbidden}`."
            );
        }
    }

    assert!(
        UI_COMPONENTS_CARGO_SOURCE.contains("component-label = []"),
        "ui-components feature tree should keep label as zero-extra-dependency feature."
    );
    for forbidden in [
        "component-label = [\"dep:serde\"",
        "component-label = [\"dep:serde_json\"",
        "component-label = [\"dep:tracing\"",
        "label-wasm-debug",
    ] {
        assert!(
            !UI_COMPONENTS_CARGO_SOURCE.contains(forbidden),
            "label feature should not pin spec/tracing debug dependency fan-out `{forbidden}`."
        );
    }
    for required in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            UI_COMPONENTS_CARGO_SOURCE.contains(required),
            "workspace should keep shared tracing feature baseline marker `{required}`."
        );
    }

    for forbidden in ["serde", "tracing", "tokio", "async-std", "async_std"] {
        assert!(
            !COMPONENT_CARGO_SOURCE.contains(forbidden),
            "components/label/Cargo.toml should avoid runtime/spec dependency leak `{forbidden}`."
        );
    }
}

#[test]
fn label_component_directory_contract_keeps_standard_file_layout_and_responsibility_split() {
    assert!(
        CHECK2_SOURCE.contains("- [x] 组件目录标准文件落点正确。"),
        "label/check2.md should mark component-directory checklist item completed."
    );
    assert!(
        CHECK2_SOURCE.contains("- [x] 文件落点纪律："),
        "label/check2.md should mark file-placement-discipline checklist item completed."
    );

    let component_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            component_root.join(required).exists(),
            "components/label/src/{required} should exist."
        );
    }
    for forbidden in ["spec.rs", "render.rs"] {
        assert!(
            !component_root.join(forbidden).exists(),
            "components/label/src/{forbidden} should not exist for label scope."
        );
    }

    for needle in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Label;",
    ] {
        assert!(
            MOD_SOURCE.contains(needle),
            "label/mod.rs should keep boundary marker `{needle}`."
        );
    }
    for forbidden in ["pub mod logic;", "pub mod view;", "fn "] {
        assert!(
            !MOD_SOURCE.contains(forbidden),
            "label/mod.rs should avoid implementation/export leak `{forbidden}`."
        );
    }

    for needle in [
        "pub(super) fn normalize_view_input(",
        "pub(super) fn derive_render_state(",
        "resolve_state(LabelStateInput {",
        "compose_class_name(",
    ] {
        assert!(
            LOGIC_SOURCE.contains(needle),
            "label/logic.rs should keep normalization/derivation marker `{needle}`."
        );
    }
    for forbidden in ["#[component]", "view! {", "web_sys::", "wasm_bindgen::"] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden),
            "label/logic.rs should not absorb render/platform token `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str", "var(--ui-", "var(--ui-fallback-"] {
        assert!(
            STYLES_SOURCE.contains(needle),
            "label/styles.rs should keep token-first static css marker `{needle}`."
        );
    }
    for forbidden in ["#[component]", "fn ", "leptos::"] {
        assert!(
            !STYLES_SOURCE.contains(forbidden),
            "label/styles.rs should stay static-css only; found `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "locale_attrs(normalized.lang, dir)",
        "view! {",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "label/view.rs should keep render/headless mount marker `{needle}`."
        );
    }
    for forbidden in [
        "logic::resolve_state(LabelStateInput {",
        "logic::compose_class_name(",
        "crate::label::styles::CSS",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "label/view.rs should not own logic/static-css token `{forbidden}`."
        );
    }

    for needle in [
        "pub struct LabelMotion",
        "pub fn motion_source_attr(motion: LabelMotion)",
        "pub fn attach_motion(base_vars: Option<String>, motion: LabelMotion) -> String",
    ] {
        assert!(
            MOTION_SOURCE.contains(needle),
            "label/motion.rs should keep motion contract marker `{needle}`."
        );
    }
    for forbidden in ["#[component]", "view! {", "<label", "locale_attrs"] {
        assert!(
            !MOTION_SOURCE.contains(forbidden),
            "label/motion.rs should not absorb view concern `{forbidden}`."
        );
    }
}

#[test]
fn label_ui_components_entrypoint_contract_paths_are_stable_and_boundary_scoped() {
    assert!(
        CHECK2_SOURCE.contains("- [x] `ui-components` 固定入口文件落点正确。"),
        "label/check2.md should mark ui-components entrypoint checklist item completed."
    );

    for needle in [
        "mod css;",
        "pub use root::UiRoot;",
        "pub fn push_components_css(out: &mut String) {",
        "css::push_components_css(out);",
        "#[cfg(feature = \"component-label\")]",
        "#[path = \"../../../components/label/src/mod.rs\"]",
    ] {
        assert!(
            UI_COMPONENTS_LIB_SOURCE.contains(needle),
            "ui-components/lib.rs should keep entrypoint marker `{needle}`."
        );
    }
    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "HtmlElement",
        "NodeRef<html::",
    ] {
        assert!(
            !UI_COMPONENTS_LIB_SOURCE.contains(forbidden),
            "ui-components/lib.rs public entry should not expose platform detail token `{forbidden}`."
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String) {",
        "#[cfg(feature = \"component-label\")]",
        "out.push_str(crate::label::styles::CSS);",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            UI_COMPONENTS_CSS_SOURCE.contains(needle),
            "ui-components/css.rs should keep css aggregation marker `{needle}`."
        );
    }

    for needle in [
        "#[component]",
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            UI_COMPONENTS_ROOT_SOURCE.contains(needle),
            "ui-components/root.rs should keep root-injection marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            UI_VISUAL_ACTIVE_HIGHLIGHT_SOURCE.contains(needle),
            "ui-visual-primitive/active_highlight.rs should keep shared primitive marker `{needle}`."
        );
    }
    for forbidden in ["ui-label", "data-slot=\"label\"", "LabelMotion"] {
        assert!(
            !UI_VISUAL_ACTIVE_HIGHLIGHT_SOURCE.contains(forbidden),
            "active_highlight shared primitive should not absorb label business token `{forbidden}`."
        );
    }

    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root should be two levels above components/label");
    for absent in [
        "crates/ui-components/src/overlay_open.rs",
        "crates/ui-components/src/presence.rs",
        "crates/ui-components/src/a11y.rs",
    ] {
        assert!(
            !workspace_root.join(absent).exists(),
            "`{absent}` should not exist under ui-components entrypoints."
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
        "pub fn aria_controls_when_open(",
    ] {
        let found = UI_HEADLESS_CONTROLLABLE_STATE_SOURCE.contains(needle)
            || UI_HEADLESS_PRESENCE_SOURCE.contains(needle)
            || UI_HEADLESS_A11Y_SOURCE.contains(needle);
        assert!(
            found,
            "headless source-of-truth marker `{needle}` should remain in ui-headless."
        );
    }
}

#[test]
fn label_rust_hygiene_contract_disallows_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let non_test_sources = [
        ("components/label/src/mod.rs", MOD_SOURCE),
        ("components/label/src/logic.rs", LOGIC_SOURCE),
        ("components/label/src/view.rs", VIEW_SOURCE),
        ("components/label/src/styles.rs", STYLES_SOURCE),
        ("components/label/src/motion.rs", MOTION_SOURCE),
        ("components/label/src/protocol.rs", PROTOCOL_SOURCE),
    ];

    for (path, source) in non_test_sources {
        for forbidden in ["unwrap(", "expect(", "unwrap_err(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "label non-test source `{path}` should not contain rust-hygiene anti-pattern `{forbidden}`.",
            );
        }
    }
}

#[test]
fn label_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let non_test_sources = [
        ("components/label/src/mod.rs", MOD_SOURCE),
        ("components/label/src/logic.rs", LOGIC_SOURCE),
        ("components/label/src/view.rs", VIEW_SOURCE),
        ("components/label/src/styles.rs", STYLES_SOURCE),
        ("components/label/src/motion.rs", MOTION_SOURCE),
        ("components/label/src/protocol.rs", PROTOCOL_SOURCE),
    ];

    for (path, source) in non_test_sources {
        for forbidden in [".to_owned(", "String::from(", ".to_string()"] {
            assert!(
                !source.contains(forbidden),
                "label non-test source `{path}` should avoid string clone hotspot `{forbidden}`.",
            );
        }
    }

    for required in [
        "pub use ui_state_primitives::label::{",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_REQUIRED_INDICATOR",
    ] {
        assert!(
            LOGIC_SOURCE.contains(required),
            "label logic should keep primitive-backed default string path marker `{required}`.",
        );
    }
}

#[test]
fn label_check2_marks_rust_hygiene_contract_complete_with_scope_evidence() {
    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "非测试源码约束满足：`components/label/src/mod.rs`、`components/label/src/logic.rs`、`components/label/src/view.rs`、`components/label/src/styles.rs`、`components/label/src/motion.rs`、`components/label/src/protocol.rs` 中不存在 `unwrap/expect/unwrap_err` 与 `let _ = ...`。",
        "字符串复制热点约束满足：上述非测试源码未出现 `.to_owned()` / `String::from(...)` / 热点 `.to_string()`；默认文案与状态来源由 `ui-state-primitives::label` 统一提供。",
        "components/label/test/semantics.rs::label_rust_hygiene_contract_disallows_unwrap_expect_and_let_underscore_in_non_test_sources",
        "components/label/test/semantics.rs::label_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "crates/ui-components/tests/label_semantics.rs::label_rust_hygiene_contract_disallows_unwrap_expect_and_let_underscore_in_non_test_sources",
        "crates/ui-components/tests/label_semantics.rs::label_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "执行记录：`./scripts/check-rust-hygiene.sh` 已执行；当前环境 `rg` 缺少 PCRE2 且 `check-api-contracts` baseline drift 属仓库级噪声，组件级定向扫描结论不受影响。",
    ] {
        assert!(
            CHECK2_SOURCE.contains(needle),
            "label/check2 rust-hygiene section should keep evidence `{needle}`.",
        );
    }
}
