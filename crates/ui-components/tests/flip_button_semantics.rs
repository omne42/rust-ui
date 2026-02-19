use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn flip_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button/flip/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "FlipButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn flip_button_uses_logic_state_model() {
    let view_source = load_source("src/button/flip/view.rs");
    let logic_source = load_source("src/button/flip/logic.rs");

    for needle in [
        "pub struct FlipButtonInputNormalizationInput",
        "pub struct FlipButtonInputNormalization",
        "pub enum FlipDirection",
        "pub struct FlipButtonStateInput",
        "pub struct FlipButtonState",
        "pub fn normalize_input(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn resolve_agent_contract(",
        "super::super::logic::resolve_agent_contract_for_state_axis(",
        "super::super::logic::ButtonAgentStateAxis::Ready",
        "use ui_state_primitives::button_flip::{FlipButtonStateCoreInput, resolve_state_core};",
        "let core = resolve_state_core(FlipButtonStateCoreInput {",
        "pub direction_class: &'static str",
        "pub state_attr: &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "FlipButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let normalized = logic::normalize_input(FlipButtonInputNormalizationInput {",
        "logic::resolve_state(FlipButtonStateInput {",
        "let agent_contract = Signal::derive(move || logic::resolve_agent_contract(state.get()));",
        "class=move || logic::compose_class_name(class_name_source.clone(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipButton view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn flip_button_direction_axis_is_type_constrained() {
    let logic_source = load_source("src/button/flip/logic.rs");
    let view_source = load_source("src/button/flip/view.rs");

    for needle in [
        "pub enum FlipDirection",
        "Top,",
        "Bottom,",
        "Left,",
        "Right,",
        "pub from: Option<FlipDirection>",
        "direction: input.direction,",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "FlipButton should constrain direction axis via typed enum contract `{needle}`."
        );
    }

    for forbidden in [
        "from: Option<String>",
        "from: Option<bool>",
        "mode: Option<bool>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipButton should not model direction via weak input `{forbidden}`."
        );
    }
}

#[test]
fn flip_button_machine_readable_state_contract_is_typed_and_marker_driven() {
    let logic_source = load_source("src/button/flip/logic.rs");
    let view_source = load_source("src/button/flip/view.rs");

    for needle in [
        "pub enum FlipDirection",
        "pub fn normalize_input(",
        "pub fn resolve_state(",
        "resolve_state_core(FlipButtonStateCoreInput {",
        "super::super::logic::resolve_agent_contract_for_state_axis(",
    ] {
        assert!(
            logic_source.contains(needle),
            "FlipButton should keep typed machine-readable state contract in logic `{needle}`."
        );
    }

    for needle in [
        "data-from=move || state.get().direction_attr",
        "data-state=move || state.get().state_attr",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipButton should expose stable machine-readable semantic marker `{needle}`."
        );
    }

    for forbidden in ["pub from: Option<String>", "pub from: Option<bool>"] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "FlipButton should avoid weakly typed state input `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_state_primitive_stays_dom_and_style_free() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/button_flip.rs");

    for needle in [
        "pub struct FlipButtonStateCoreInput",
        "pub struct FlipButtonStateCore",
        "pub fn resolve_state_core(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "FlipButton state primitive should keep pure state contract `{needle}`."
        );
    }

    for forbidden in [
        "leptos",
        "web_sys",
        "NodeRef<",
        "set_property(",
        "style=",
        "class=",
        "ui-flip-button",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives button_flip should not contain DOM/style concern `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_delegates_reusable_state_core_to_state_primitives() {
    let logic_source = load_source("src/button/flip/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/button_flip.rs");

    for needle in [
        "use ui_state_primitives::button_flip::{FlipButtonStateCoreInput, resolve_state_core};",
        "let core = resolve_state_core(FlipButtonStateCoreInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "FlipButton logic should delegate reusable state core to ui-state-primitives via `{needle}`.",
        );
    }

    for forbidden in [
        "let is_active = input.is_hovered || input.is_focus_within;",
        "state_attr: if is_active { \"active\" } else { \"inactive\" }",
        "hover_attr: if input.is_hovered",
        "motion_source_attr: if input.has_custom_motion",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "FlipButton logic should not re-implement reusable state primitive branch `{forbidden}`.",
        );
    }

    assert!(
        primitive_source.contains("let is_active = input.is_hovered || input.is_focus_within;"),
        "Reusable flip state core derivation should live in ui-state-primitives.",
    );
}

#[test]
fn flip_button_headless_primitives_stay_visual_and_motion_free() {
    let hover_source = load_source("../../crates/ui-headless/src/hover.rs");
    let focus_within_source = load_source("../../crates/ui-headless/src/focus_within.rs");

    for needle in [
        "pub struct HoverOptions",
        "pub struct HoverHandlers",
        "pub struct HoverState",
        "pub fn use_hover(options: HoverOptions) -> HoverState",
        "pub struct FocusWithinOptions",
        "pub struct FocusWithinHandlers",
        "pub struct FocusWithinState",
        "pub fn use_focus_within(options: FocusWithinOptions) -> FocusWithinState",
    ] {
        assert!(
            hover_source.contains(needle) || focus_within_source.contains(needle),
            "ui-headless primitives should expose typed interaction contract `{needle}`."
        );
    }

    for forbidden in [
        "ui_motion",
        "SpringAnimator",
        "set_property(",
        "--ui-",
        ".ui-",
        "class=",
        "style=",
    ] {
        assert!(
            !hover_source.contains(forbidden) && !focus_within_source.contains(forbidden),
            "ui-headless hover/focus_within should not contain visual or animation orchestration `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_uses_headless_hover_and_focus_within_hooks() {
    let source = load_source("src/button/flip/view.rs");

    for needle in ["use_hover", "use_focus_within"] {
        assert!(
            source.contains(needle),
            "FlipButton should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn flip_button_mounts_ui_headless_contract_in_view_boundary() {
    let view_source = load_source("src/button/flip/view.rs");
    let logic_source = load_source("src/button/flip/logic.rs");
    let styles_source = load_source("src/button/flip/styles.rs");
    let motion_source = load_source("src/button/flip/motion.rs");

    for needle in [
        "use ui_headless::{",
        "A11yDirection, FocusWithinOptions, HoverOptions, locale_attrs, use_focus_within, use_hover,",
        "let hover = use_hover(HoverOptions { is_disabled: false });",
        "let focus_within = use_focus_within(FocusWithinOptions { is_disabled: false });",
        "hover.handlers.on_pointer_enter.run(())",
        "hover.handlers.on_pointer_leave.run(())",
        "focus_within.handlers.on_focus_in.run(())",
        "focus_within.handlers.on_focus_out.run(())",
        "is_hovered: hover.is_hovered.get(),",
        "is_focus_within: focus_within.is_focus_within.get(),",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipButton should mount typed headless attrs/handlers/state contract in view boundary `{needle}`.",
        );
    }

    for forbidden in ["ui_headless::", "use_hover(", "use_focus_within("] {
        assert!(
            !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Headless dependency should stay in view.rs boundary; found leaked usage `{forbidden}`.",
        );
    }
}

#[test]
fn ui_headless_feature_contract_keeps_web_and_ssr_mutually_exclusive() {
    let headless_source = load_source("../../crates/ui-headless/src/lib.rs");
    let view_source = load_source("src/button/flip/view.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_source.contains(needle),
            "ui-headless should keep web/ssr mutual-exclusion compile-time guard `{needle}`.",
        );
    }

    assert!(
        view_source.contains("use ui_headless::{")
            && view_source.contains(
                "A11yDirection, FocusWithinOptions, HoverOptions, locale_attrs, use_focus_within, use_hover,",
            ),
        "FlipButton should continue consuming headless API contract from ui-headless.",
    );
}

#[test]
fn flip_button_semantics_cover_pointer_focus_and_platform_paths() {
    let view_source = load_source("src/button/flip/view.rs");
    let motion_source = load_source("src/button/flip/motion.rs");

    for needle in [
        "on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())",
        "on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())",
        "on:focusin=move |_| focus_within.handlers.on_focus_in.run(())",
        "on:focusout=move |_| focus_within.handlers.on_focus_out.run(())",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipButton should cover pointer/focus semantics path `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "FlipButton should keep explicit wasm/non-wasm semantic-path contract `{needle}`."
        );
    }

    for forbidden in ["assert_snapshot", "insta::"] {
        assert!(
            !view_source.contains(forbidden) && !motion_source.contains(forbidden),
            "FlipButton contract tests should be semantic-marker based, not snapshot-first `{forbidden}`."
        );
    }
}

#[test]
fn ui_motion_and_flip_button_provide_non_wasm_safe_stub_path() {
    let ui_motion_source = load_source("../../crates/ui-motion/src/lib.rs");
    let flip_motion_source = load_source("src/button/flip/motion.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions)",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion should keep non-wasm no-op/stub contract `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "drop(sanitize_motion(motion));",
        "crate::button::motion::sanitize_motion(",
    ] {
        assert!(
            flip_motion_source.contains(needle),
            "FlipButton motion should safely degrade in non-wasm and reuse button sanitizer `{needle}`.",
        );
    }
}

#[test]
fn flip_button_component_files_keep_responsibility_boundaries() {
    let mod_source = load_source("src/button/flip/mod.rs");
    let logic_source = load_source("src/button/flip/logic.rs");
    let styles_source = load_source("src/button/flip/styles.rs");
    let view_source = load_source("src/button/flip/view.rs");
    let motion_source = load_source("src/button/flip/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::FlipDirection;",
        "pub use view::FlipButton;",
    ] {
        assert!(
            mod_source.contains(needle),
            "FlipButton mod.rs should keep a minimal export boundary with `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_input(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            logic_source.contains(needle),
            "FlipButton logic.rs should focus on normalization/derivation with `{needle}`."
        );
    }

    for forbidden in ["view! {", "NodeRef<", "set_property(", "HtmlElement"] {
        assert!(
            !logic_source.contains(forbidden),
            "FlipButton logic.rs should not own view/dom/motion responsibility `{forbidden}`."
        );
    }

    for needle in [
        ".ui-flip-button",
        ".ui-flip-button__front",
        ".ui-flip-button__back",
    ] {
        assert!(
            styles_source.contains(needle),
            "FlipButton styles.rs should own static style contract `{needle}`."
        );
    }

    for needle in [
        "view! {",
        "use_hover",
        "use_focus_within",
        "logic::resolve_state(FlipButtonStateInput {",
        "motion::attach_motion(node_ref, is_active, direction, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipButton view.rs should own structure/headless mount `{needle}`."
        );
    }

    for needle in [
        "pub fn attach_motion(",
        "crate::button::motion::sanitize_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "FlipButton motion.rs should keep motion mapping and reuse button capability `{needle}`."
        );
    }
}

#[test]
fn flip_button_view_avoids_hidden_state_machine_decisions() {
    let view_source = load_source("src/button/flip/view.rs");
    let logic_source = load_source("src/button/flip/logic.rs");

    for needle in [
        "let normalized = logic::normalize_input(FlipButtonInputNormalizationInput {",
        "let state = Signal::derive(move || {",
        "logic::resolve_state(FlipButtonStateInput {",
        "class=move || logic::compose_class_name(class_name_source.clone(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipButton view should consume logic outputs rather than rebuilding state decision `{needle}`.",
        );
    }

    for forbidden in [
        "if is_hovered",
        "if is_focus_within",
        "match direction",
        "match from",
        "ui-flip-button--state-active",
        "ui-flip-button--state-inactive",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "FlipButton view should not hide state-machine branching `{forbidden}`.",
        );
    }

    assert!(
        logic_source
            .contains("pub fn resolve_state(input: FlipButtonStateInput) -> FlipButtonState"),
        "FlipButton logic should remain the single state-derivation boundary.",
    );
}

#[test]
fn flip_button_props_are_wired_into_typed_and_tested_contract_path() {
    let view_source = load_source("src/button/flip/view.rs");
    let logic_source = load_source("src/button/flip/logic.rs");

    for needle in [
        "#[prop(optional, into)] from: Option<FlipDirection>,",
        "#[prop(optional, into)] motion: Option<FlipButtonMotion>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "let normalized = logic::normalize_input(FlipButtonInputNormalizationInput {",
        "from,",
        "motion,",
        "class_name,",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipButton public props should be type-constrained and routed through normalization `{needle}`.",
        );
    }

    for needle in [
        "pub struct FlipButtonInputNormalizationInput",
        "pub from: Option<FlipDirection>",
        "pub motion: Option<FlipButtonMotion>",
        "pub class_name: Option<String>",
        "let direction = input.from.unwrap_or_default();",
        ".motion",
        ".map(motion::sanitize_motion)",
        ".unwrap_or_default();",
        "super::super::logic::normalize_optional_text(input.class_name);",
    ] {
        assert!(
            logic_source.contains(needle),
            "FlipButton logic should include typed/defaulted param contract handling `{needle}`.",
        );
    }

    for needle in [
        "data-from=move || state.get().direction_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipButton should expose param-derived semantic contract marker `{needle}`.",
        );
    }
}

#[test]
fn flip_button_api_uses_explicit_face_slots_not_parallel_arrays() {
    let view_source = load_source("src/button/flip/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "#[prop(into)] front: ViewFn,",
        "#[prop(into)] back: ViewFn,",
        "data-slot=\"flip-button-front\"",
        "data-slot=\"flip-button-back\"",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipButton should bind semantics explicitly via front/back slots `{needle}`.",
        );
    }

    for forbidden in ["Vec<ViewFn>", "children: Vec"] {
        assert!(
            !view_source.contains(forbidden),
            "FlipButton should avoid parallel-array/implicit-index API pattern `{forbidden}`.",
        );
    }

    for forbidden in ["labels + children", "titles + panels"] {
        assert!(
            !view_source.contains(forbidden) && !docs_source.contains(forbidden),
            "FlipButton should avoid parallel-array/implicit-index API pattern `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_public_api_does_not_leak_platform_private_types() {
    let mod_source = load_source("src/button/flip/mod.rs");
    let view_source = load_source("src/button/flip/view.rs");
    let logic_source = load_source("src/button/flip/logic.rs");
    let motion_source = load_source("src/button/flip/motion.rs");

    for needle in [
        "pub use logic::FlipDirection;",
        "pub use motion::FlipButtonMotion;",
        "pub use view::FlipButton;",
        "pub fn FlipButton(",
        "pub struct FlipButtonMotion",
        "pub fn sanitize_motion(motion: FlipButtonMotion) -> FlipButtonMotion",
        "pub fn attach_motion(",
    ] {
        assert!(
            mod_source.contains(needle)
                || view_source.contains(needle)
                || motion_source.contains(needle),
            "FlipButton public API surface should include stable contract item `{needle}`.",
        );
    }

    for forbidden in [
        "pub use leptos::web_sys",
        "pub use web_sys",
        "pub use wasm_bindgen",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "FlipButton module exports should not leak platform-specific type alias `{forbidden}`.",
        );
    }

    for forbidden in ["web_sys::", "wasm_bindgen::", "js_sys::"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "FlipButton public component/logic API should not expose platform private type token `{forbidden}`.",
        );
    }

    assert!(
        motion_source.contains(
            "pub fn attach_motion(\n    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,\n    is_active: leptos::prelude::Signal<bool>,\n    from: FlipDirection,\n    motion: FlipButtonMotion,\n)",
        ),
        "FlipButton motion public function signature should stay framework-level and avoid leaking web_sys/wasm private types.",
    );
}

#[test]
fn flip_button_reuses_button_capabilities_to_avoid_cross_component_drift() {
    let logic_source = load_source("src/button/flip/logic.rs");
    let motion_source = load_source("src/button/flip/motion.rs");

    for needle in [
        "super::super::logic::normalize_optional_text(input.class_name);",
        "super::super::logic::resolve_agent_contract_for_state_axis(",
        "crate::button::motion::sanitize_motion(",
    ] {
        assert!(
            logic_source.contains(needle) || motion_source.contains(needle),
            "FlipButton should reuse shared button capability `{needle}` to avoid one-off drift."
        );
    }

    for forbidden in [
        "fn normalize_optional_text(",
        "fn resolve_agent_contract_for_state_axis(",
        "fn sanitize_spring(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !motion_source.contains(forbidden),
            "FlipButton should not add local stopgap implementation `{forbidden}` that diverges from shared button contract."
        );
    }
}

#[test]
fn flip_button_component_directory_uses_standard_file_layout() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir.join("src/button/flip");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            component_dir.join(required).exists(),
            "FlipButton component directory should include standard file `{required}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !component_dir.join(forbidden).exists(),
            "FlipButton component directory should not drift to `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_keeps_schema_contract_in_button_without_local_spec_module() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let flip_spec = manifest_dir.join("src/button/flip/spec.rs");
    let mod_source = load_source("src/button/flip/mod.rs");
    let logic_source = load_source("src/button/flip/logic.rs");

    assert!(
        !flip_spec.exists(),
        "FlipButton should not introduce local `spec.rs`; simple component contracts stay in check/docs and shared button contract helpers."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "FlipButton should not expose a local spec module via `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains("super::super::logic::resolve_agent_contract_for_state_axis("),
        "FlipButton should reuse button-level schema contract resolver instead of duplicating spec logic.",
    );
}

#[test]
fn flip_button_emits_baseline_style_data_attributes() {
    let source = load_source("src/button/flip/view.rs");

    for needle in [
        "data-slot=\"flip-button\"",
        "data-from=move || state.get().direction_attr",
        "data-state=move || state.get().state_attr",
        "data-ui-agent-schema=move || agent_contract.get().schema_name",
        "data-ui-agent-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-capability-press=move || {",
        "data-ui-capability-focus=move || {",
        "data-ui-capability-hover=move || {",
        "data-ui-capability-popup-trigger=move || {",
        "data-hover=move || state.get().hover_attr",
        "data-focus-within-state=move || state.get().focus_within_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-active=move || state.get().is_active.then_some(\"true\")",
        "data-inactive=move || state.get().is_inactive.then_some(\"true\")",
        "data-hovered=move || state.get().is_hovered.then_some(\"true\")",
        "data-focus-within=move || state.get().is_focus_within.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "FlipButton should expose `{needle}` for baseline-style state inspection."
        );
    }
}

#[test]
fn flip_button_styles_include_state_marker_contracts() {
    let source = load_source("src/button/flip/styles.rs");

    for selector in [
        ".ui-flip-button[data-class-source=\"custom\"]",
        ".ui-flip-button--custom-class",
        ".ui-flip-button[data-custom-class=\"true\"]",
        ".ui-flip-button[data-motion-source=\"custom\"]",
        ".ui-flip-button--custom-motion",
        ".ui-flip-button[data-custom-motion=\"true\"]",
        ".ui-flip-button--state-active .ui-flip-button__front",
        ".ui-flip-button[data-state=\"active\"] .ui-flip-button__back",
        ".ui-flip-button--from-top .ui-flip-button__front",
        ".ui-flip-button[data-from=\"left\"] .ui-flip-button__back",
        ".ui-flip-button--from-right .ui-flip-button__back",
    ] {
        assert!(
            source.contains(selector),
            "FlipButton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn flip_button_styles_avoid_brittle_dom_structure_selectors() {
    let source = load_source("src/button/flip/styles.rs");

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        ":first-child",
        ":last-child",
        "> .ui-flip-button__",
    ] {
        assert!(
            !source.contains(forbidden),
            "FlipButton styles should rely on explicit state markers, not brittle selector `{forbidden}`."
        );
    }
}

#[test]
fn flip_button_attaches_motion_driver() {
    let view_source = load_source("src/button/flip/view.rs");
    let motion_source = load_source("src/button/flip/motion.rs");

    assert!(
        view_source.contains("motion::attach_motion(node_ref, is_active, direction, motion);"),
        "FlipButton view should attach the motion driver to synchronize spring progress."
    );

    for needle in [
        "set_property(\"--ui-flip-progress\"",
        "SpringAnimator::new",
        "spring.set_target(if active { 1.0 } else { 0.0 });",
    ] {
        assert!(
            motion_source.contains(needle),
            "FlipButton motion should include `{needle}` for spring-based flip transitions."
        );
    }
}

#[test]
fn flip_button_exposes_front_and_back_face_slots() {
    let source = load_source("src/button/flip/view.rs");

    for needle in [
        "data-slot=\"flip-button-front\"",
        "data-slot=\"flip-button-back\"",
    ] {
        assert!(
            source.contains(needle),
            "FlipButton should include `{needle}` to make face composition contract explicit."
        );
    }
}

#[test]
fn flip_button_motion_contract_exposes_default_and_custom_tests() {
    let source = load_source("src/button/flip/motion.rs");

    for needle in [
        "pub struct FlipButtonMotion",
        "fn default_motion_matches_flip_button_spring_contract()",
        "fn supports_custom_flip_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "FlipButton motion module should include `{needle}` for baseline-level motion contract coverage."
        );
    }
}

#[test]
fn flip_button_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/button/flip/motion.rs");
    let view_source = load_source("src/button/flip/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: FlipButtonMotion) -> FlipButtonMotion",
        "let sanitized = crate::button::motion::sanitize_motion(crate::button::motion::ButtonMotion {",
        "sanitize_spring_with_fallback(motion.spring, base.spring)",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "FlipButton motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains(
            "let normalized = logic::normalize_input(FlipButtonInputNormalizationInput {"
        ),
        "FlipButton view should route defaults and sanitize paths through logic normalization.",
    );
}

#[test]
fn flip_button_declares_no_async_loading_protocol() {
    let source = load_source("src/button/flip/view.rs");

    for forbidden in ["is_loading", "aria-busy", "use_async_action", "retry"] {
        assert!(
            !source.contains(forbidden),
            "FlipButton should not define async/loading protocol marker `{forbidden}` because this component is synchronous.",
        );
    }
}

#[test]
fn flip_button_does_not_use_inner_html_in_component_layers() {
    for rel_path in [
        "src/button/flip/view.rs",
        "src/button/flip/logic.rs",
        "src/button/flip/styles.rs",
        "src/button/flip/motion.rs",
    ] {
        let source = load_source(rel_path);
        assert!(
            !source.contains("inner_html"),
            "FlipButton should not use `inner_html` in component layer file `{rel_path}`."
        );
    }
}

#[test]
fn flip_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn flip_button() -> AnyView",
        "title=\"FlipButton\"",
        "slug=\"flip-button\"",
        "description=\"baseline-level spring flip surface with centralized direction/interaction/class-source state attrs.\"",
        "<Playground title=\"Top flip\" code_signal=code>",
        "<Playground title=\"Direction matrix\" code_signal=states_code>",
        "<Playground title=\"Custom Class\" code_signal=custom_code>",
        "<FlipButton",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for flip_button primary playground coverage.",
        );
    }
}

#[test]
fn flip_button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "title=\"Top flip\"",
        "from=FlipDirection::Top",
        "variant=ButtonVariant::Secondary",
        "variant=ButtonVariant::Accent",
        "title=\"Direction matrix\"",
        "from=FlipDirection::Bottom",
        "from=FlipDirection::Left",
        "from=FlipDirection::Right",
        "title=\"Custom Class\"",
        "class_name=\"docs-flip-button-custom\".to_string()",
        "variant=ButtonVariant::Outline",
        "\"Inspect\"",
        "\"Inspecting\"",
    ] {
        assert!(
            source.contains(needle),
            "flip_button docs playgrounds should contain `{needle}`.",
        );
    }
}
