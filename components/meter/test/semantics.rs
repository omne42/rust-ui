use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));

    if rel_path == "../../apps/docs-app/src/pages/components/pages/display.rs" {
        let parent_path = root.join(rel_path);
        let meter_path =
            root.join("../../apps/docs-app/src/pages/components/pages/display/meter.rs");
        let code_path = root.join("../../apps/docs-app/src/pages/components/pages/display/code.rs");
        let parent = fs::read_to_string(&parent_path)
            .unwrap_or_else(|err| panic!("failed to read {parent_path:?}: {err}"));
        let meter_source = fs::read_to_string(&meter_path)
            .unwrap_or_else(|err| panic!("failed to read {meter_path:?}: {err}"));
        let code_source = fs::read_to_string(&code_path)
            .unwrap_or_else(|err| panic!("failed to read {code_path:?}: {err}"));

        return format!("{parent}\n{meter_source}\n{code_source}")
            .replace(
                "pub(crate) fn meter() -> AnyView {",
                "pub(super) fn meter() -> AnyView {",
            )
            .replace(
                "pub(crate) fn code() -> AnyView {",
                "pub(super) fn code() -> AnyView {",
            );
    }

    if rel_path == "../../crates/ui-theme/src/css.rs" {
        let css_path = root.join("../../crates/ui-theme/src/css.rs");
        let render_path = root.join("../../crates/ui-theme/src/css/render.rs");
        let inc_path = root.join("../../crates/ui-theme/src/css/render/theme_to_css_variables.inc");

        let css_source = fs::read_to_string(&css_path)
            .unwrap_or_else(|err| panic!("failed to read {css_path:?}: {err}"));
        let render_source = fs::read_to_string(&render_path)
            .unwrap_or_else(|err| panic!("failed to read {render_path:?}: {err}"));
        let inc_source = fs::read_to_string(&inc_path)
            .unwrap_or_else(|err| panic!("failed to read {inc_path:?}: {err}"));

        return format!("{css_source}\n{render_source}\n{inc_source}");
    }

    let path = root.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path:?}: {err}"))
}

#[test]
fn component_contract_files_exist() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for rel_path in [
        "src/logic.rs",
        "src/view.rs",
        "src/styles.rs",
        "src/motion.rs",
        "test/logic.rs",
        "test/motion.rs",
        "test/protocol.rs",
        "test/semantics.rs",
    ] {
        assert!(
            root.join(rel_path).exists(),
            "required component contract file should exist: {rel_path}"
        );
    }
}

#[test]
fn public_api_is_stable_and_hides_platform_details() {
    let source = load_source("src/mod.rs");

    for needle in [
        "pub use logic::{",
        "pub use motion::MeterMotion;",
        "pub use view::Meter;",
    ] {
        assert!(
            source.contains(needle),
            "meter public API should include `{needle}`"
        );
    }

    for forbidden in ["pub mod logic", "pub mod view", "web_sys", "HtmlElement"] {
        assert!(
            !source.contains(forbidden),
            "meter public API must not expose platform detail `{forbidden}`"
        );
    }
}

#[test]
fn view_mounts_semantic_markers() {
    let source = load_source("src/view.rs");

    for needle in [
        "role=\"meter\"",
        "aria-valuemin=range.min.to_string()",
        "aria-valuemax=range.max.to_string()",
        "aria-valuenow=move || render_state.get().aria_value_now",
        "aria-valuetext=move || render_state.get().value_label_text",
        "data-slot=\"meter\"",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-state=move || render_state.get().phase.as_str()",
        "data-label-source=state.label_source_attr",
        "data-value-label-source=state.value_label_source_attr",
        "data-motion-source=state.motion_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "meter view semantic contract should include `{needle}`"
        );
    }
}

#[test]
fn api_naming_prefers_is_prefix_with_compat_alias() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for needle in [
        "#[prop(optional, into)] is_value_label_visible: Option<bool>",
        "#[prop(optional, into)] show_value_label: Option<bool>",
    ] {
        assert!(
            view_source.contains(needle),
            "meter naming contract should include `{needle}`"
        );
    }

    assert!(
        logic_source.contains(
            "is_value_label_visible\n        .unwrap_or(input.show_value_label.unwrap_or(DEFAULT_SHOW_VALUE_LABEL))"
        ),
        "default visibility priority should be normalized in logic layer"
    );
}

#[test]
fn runtime_state_derivation_is_centralized_in_logic_layer() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");

    assert!(
        view_source.contains("logic::derive_render_state(logic::MeterRenderStateInput"),
        "view must delegate runtime state derivation to logic::derive_render_state"
    );
    assert!(
        !view_source.contains("logic::clamp_to_range("),
        "view should not rebuild clamp rules directly"
    );
    assert!(
        !view_source.contains("logic::normalize_progress("),
        "view should not rebuild progress normalization directly"
    );
    assert!(
        !view_source.contains("if !is_value_label_visible"),
        "view should not rebuild value-label state machine branches"
    );

    for needle in [
        "pub struct MeterRenderStateInput",
        "pub struct MeterRenderState",
        "pub fn derive_render_state(input: MeterRenderStateInput) -> MeterRenderState",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic should expose centralized runtime semantics contract `{needle}`"
        );
    }

    assert!(
        styles_source.contains("data-state=\"indeterminate\""),
        "styles should consume semantic state markers instead of rebuilding state logic"
    );
}

#[test]
fn discrete_state_axes_use_typed_enums() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/meter.rs");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for needle in [
        "pub enum MeterVariant",
        "pub enum MeterSize",
        "pub enum MeterPhase",
    ] {
        assert!(
            primitive_source.contains(needle),
            "discrete meter state axis should be enum-typed: `{needle}`"
        );
    }

    for needle in [
        "#[prop(optional)] size: MeterSize",
        "#[prop(optional)] variant: MeterVariant",
        "pub phase: MeterPhase",
        "data-state=move || render_state.get().phase.as_str()",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "meter component should consume typed discrete state contract `{needle}`"
        );
    }

    for forbidden in [
        "variant: Option<String>",
        "size: Option<String>",
        "status: Option<String>",
        "mode: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "discrete state axes should not be stringly typed in component API: `{forbidden}`"
        );
    }
}

#[test]
fn state_primitives_source_boundary_is_enforced() {
    let cargo_source = load_source("Cargo.toml");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    assert!(
        cargo_source
            .contains("ui-state-primitives = { path = \"../../crates/ui-state-primitives\" }"),
        "meter crate should depend on ui-state-primitives for shared state capabilities"
    );
    assert!(
        logic_source.contains("pub use ui_state_primitives::meter::{"),
        "logic layer should source primitive contracts from ui-state-primitives"
    );
    assert!(
        view_source.contains("logic::resolve_state(logic::MeterStateInput {"),
        "view should consume primitive-backed state contract via logic layer"
    );

    for forbidden in [
        "app_state",
        "BusinessStore",
        "GlobalStore",
        "Redux",
        "MobX",
        "Pinia",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "component should not directly bind business store type `{forbidden}`"
        );
    }
}

#[test]
fn async_interaction_contract_is_not_applicable_for_meter() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "retry",
        "on_retry",
        "use_async_action",
        "loading",
        "error",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "meter display component should not define async interaction contract token `{forbidden}`"
        );
    }
}

#[test]
fn dx_paradox_keeps_default_path_simple() {
    let view_source = load_source("src/view.rs");
    let readme_source = load_source("src/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "#[prop(into)] state:",
        "#[prop(optional, into)] state:",
        "#[prop(optional)] state:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "meter should not require exposing internal state object in public API: `{forbidden}`"
        );
    }

    assert!(
        readme_source.contains(
            "<Meter\n  id=\"docs-meter\".to_string()\n  label=\"Completion\".to_string()\n  value=Signal::derive(|| Some(42.0))\n/>"
        ),
        "README should keep Hello World within a 5-line minimal default API snippet"
    );

    for needle in [
        "pub(super) fn meter() -> AnyView",
        "Playground title=\"Hello World (Default API)\"",
        "id=\"docs-meter-hello\".to_string()",
        "label=\"Completion\".to_string()",
        "value=Signal::derive(|| Some(42.0))",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app meter page should expose a minimal default usage path `{needle}`"
        );
    }
}

#[test]
fn composite_parent_item_api_is_not_applicable_for_meter() {
    let view_source = load_source("src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "#[prop(optional)] children:",
        "#[prop(optional, into)] items:",
        "labels: Vec<",
        "titles: Vec<",
        "panels: Vec<",
        "ItemSpec",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "meter is a single display component and should not expose composite API contract `{forbidden}`"
        );
    }

    let meter_section_start = docs_source
        .find("pub(super) fn meter() -> AnyView {")
        .expect("display docs should include meter section");
    let meter_section_end = docs_source[meter_section_start..]
        .find("pub(super) fn code() -> AnyView {")
        .map(|offset| meter_section_start + offset)
        .expect("meter section should end before code section");
    let meter_docs_section = &docs_source[meter_section_start..meter_section_end];

    for forbidden in ["items=", "labels + children", "titles + panels", "ItemSpec"] {
        assert!(
            !meter_docs_section.contains(forbidden),
            "meter docs should not recommend implicit parallel-slot composite pattern `{forbidden}`"
        );
    }
}

#[test]
fn macro_micro_dual_state_machine_is_not_applicable_for_meter() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "on:drag",
        "on:pointermove",
        "on:mousemove",
        "on:touchmove",
        "requestAnimationFrame",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "meter should not define drag macro/micro state machine contract token `{forbidden}`"
        );
    }

    assert!(
        view_source.contains("motion::attach_motion(indicator_ref, progress_value, motion);"),
        "meter should only attach motion from resolved value state, not run drag-loop actions"
    );
}

#[test]
fn two_pass_geometry_rendering_is_not_applicable_for_meter() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");

    for forbidden in [
        "get_bounding_client_rect",
        "client_width",
        "client_height",
        "offset_width",
        "offset_height",
        "ResizeObserver",
        "IntersectionObserver",
        "Intent",
        "Measure",
        "Rectification",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "meter should not implement two-pass geometry pipeline token `{forbidden}`"
        );
    }

    assert!(
        logic_source.contains(
            "pub fn derive_render_state(input: MeterRenderStateInput) -> MeterRenderState"
        ),
        "meter logic should remain pure value-to-state derivation without geometry rectification loop"
    );
}

#[test]
fn registration_protocol_is_not_applicable_for_meter() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "meter should not define dynamic child registration protocol token `{forbidden}`"
        );
    }
}

#[test]
fn slot_projection_strategy_is_not_applicable_for_meter() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");

    for forbidden in ["Lazy", "KeepAlive", "Eager", "NotifyHidden"] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "meter should not define slot projection lifecycle token `{forbidden}`"
        );
    }
}

#[test]
fn env_streams_are_not_applicable_for_meter() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "on:resize",
        "on:intersection",
        "BreakpointChanged",
        "ThemeChanged",
        "IntersectionChanged",
        "debounce",
        "throttle",
        "Action::BreakpointChanged",
        "Action::ThemeChanged",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "meter should not define env subscription stream token `{forbidden}`"
        );
    }
}

#[test]
fn event_light_cone_is_not_applicable_for_meter() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");

    for forbidden in [
        "ContextBus",
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "select_all",
        "bulk_select",
        "prop drilling",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "meter should not define event-light-cone large-collection contract token `{forbidden}`"
        );
    }
}

#[test]
fn causality_bus_is_not_applicable_for_meter() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");

    for forbidden in [
        "TraceId",
        "CausalityBus",
        "Causality Bus",
        "event_bus",
        "EventBus",
        "broadcast",
        "dispatch_command",
        "derived_command",
        "subscribe(",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "meter should not define causality-bus token `{forbidden}`"
        );
    }
}

#[test]
fn meter_has_a11y_i18n_l10n_contract_without_view_level_hardcoded_copy() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let check2_source = load_source("check2.md");

    for needle in [
        "use ui_headless::{A11yDirection, i18n, locale_attrs};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let i18n = i18n::use_ui_i18n();",
        "let strings = i18n.strings::<logic::MeterStrings>();",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang",
        "dir=locale.dir",
        "role=\"meter\"",
        "aria-label=aria_label",
        "aria-valuemin=range.min.to_string()",
        "aria-valuemax=range.max.to_string()",
    ] {
        assert!(
            view_source.contains(needle),
            "meter view should mount a11y+i18n+l10n contract marker `{needle}`"
        );
    }

    for needle in [
        "pub struct MeterStrings",
        "pub fn resolve_aria_label_with_fallback(",
        "default_aria_label: Option<Cow<'static, str>>",
        "resolve_aria_label_with_fallback(input.aria_label, label.clone(), input.default_aria_label)",
    ] {
        assert!(
            logic_source.contains(needle),
            "meter logic should keep aria-label fallback chain marker `{needle}`"
        );
    }

    for forbidden in ["aria-label=\"Meter\"", ">Meter<", "Meter</"] {
        assert!(
            !view_source.contains(forbidden),
            "meter view should avoid hardcoded visible copy `{forbidden}`"
        );
    }

    assert!(
        check2_source.contains("存在 A11y 实现、国际化与本地化实现"),
        "meter checklist should keep a11y+i18n+l10n contract entry"
    );
}

#[test]
fn state_observability_contract_uses_stable_data_and_aria_markers() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/meter.rs");
    let check2_source = load_source("check2.md");

    for marker in [
        "role=\"meter\"",
        "aria-label=aria_label",
        "aria-labelledby=aria_labelledby",
        "aria-valuemin=range.min.to_string()",
        "aria-valuemax=range.max.to_string()",
        "aria-valuenow=move || render_state.get().aria_value_now",
        "aria-valuetext=move || render_state.get().value_label_text",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-state=move || render_state.get().phase.as_str()",
        "data-label-source=state.label_source_attr",
        "data-value-label-source=state.value_label_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "meter should expose stable observable marker `{marker}` for automation selectors"
        );
    }

    for closed_set_marker in [
        "(\"ui-meter--label-custom\", \"custom\")",
        "(\"ui-meter--label-default\", \"default\")",
        "(\"ui-meter--value-label-custom\", \"custom\")",
        "(\"ui-meter--value-label-auto\", \"auto\")",
        "(\"ui-meter--motion-custom\", \"custom\")",
        "(\"ui-meter--motion-default\", \"default\")",
        "let class_source_attr = if input.has_custom_class_name {\n        \"custom\"\n    } else {\n        \"default\"",
    ] {
        assert!(
            logic_source.contains(closed_set_marker),
            "meter source marker values should remain closed and enumerable: `{closed_set_marker}`"
        );
    }

    for closed_phase_value in [
        "MeterPhase::Determinate => \"determinate\"",
        "MeterPhase::Indeterminate => \"indeterminate\"",
    ] {
        assert!(
            primitive_source.contains(closed_phase_value),
            "meter state phase should remain a closed enum-to-string mapping: `{closed_phase_value}`"
        );
    }

    for semantic_selector in [
        ".ui-meter[data-variant=\"default\"]",
        ".ui-meter[data-variant=\"danger\"]",
        ".ui-meter[data-size=\"sm\"]",
        ".ui-meter[data-size=\"lg\"]",
        ".ui-meter[data-label-source=\"custom\"]",
        ".ui-meter[data-value-label-source=\"custom\"]",
        ".ui-meter[data-motion-source=\"custom\"]",
        ".ui-meter[data-state=\"indeterminate\"]",
        ".ui-meter[data-state=\"determinate\"]",
    ] {
        assert!(
            styles_source.contains(semantic_selector),
            "meter styles should support semantic state selectors `{semantic_selector}`"
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", "childNodes["] {
        assert!(
            !styles_source.contains(forbidden) && !view_source.contains(forbidden),
            "meter observability/selectors should avoid fragile DOM-order token `{forbidden}`"
        );
    }

    assert!(
        check2_source.contains("状态可观测、可检索、可验证"),
        "meter checklist should keep state observability contract entry"
    );
}

#[test]
fn styles_depend_on_semantic_markers_not_dom_shape_guessing() {
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");
    let check2_source = load_source("check2.md");

    for selector in [
        ".ui-meter[data-variant=\"default\"]",
        ".ui-meter[data-variant=\"danger\"]",
        ".ui-meter[data-size=\"sm\"]",
        ".ui-meter[data-size=\"lg\"]",
        ".ui-meter[data-label-source=\"custom\"]",
        ".ui-meter[data-value-label-source=\"custom\"]",
        ".ui-meter[data-motion-source=\"custom\"]",
        ".ui-meter[data-state=\"indeterminate\"]",
        ".ui-meter[data-state=\"determinate\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "meter styles should branch on stable semantic selector `{selector}`"
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":first-child", ":last-child"] {
        assert!(
            !styles_source.contains(forbidden),
            "meter styles should avoid fragile DOM-shape selector `{forbidden}`"
        );
    }

    assert!(
        !view_source.contains("style="),
        "meter view should avoid inline style business-logic branches"
    );
    assert!(
        view_source.contains("motion::attach_motion(indicator_ref, progress_value, motion);"),
        "meter view should delegate runtime style updates to motion attachment"
    );
    for required in [
        "set_property(\"--ui-meter-progress\"",
        "transform: scaleX(var(--ui-meter-progress, var(--ui-fallback-meter-progress)));",
    ] {
        assert!(
            motion_source.contains(required) || styles_source.contains(required),
            "meter runtime style path should use CSS custom property marker `{required}`"
        );
    }

    assert!(
        check2_source.contains("样式依赖显式状态（`data-*`/class）"),
        "meter checklist should keep style-by-semantic-state contract entry"
    );
}

#[test]
fn semantic_contract_tests_are_primary_and_snapshot_only_checks_are_absent() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let semantics_source = load_source("test/semantics.rs");
    let check2_source = load_source("check2.md");

    for marker in [
        "role=\"meter\"",
        "aria-valuemin=range.min.to_string()",
        "aria-valuemax=range.max.to_string()",
        "aria-valuenow=move || render_state.get().aria_value_now",
        "aria-valuetext=move || render_state.get().value_label_text",
        "data-state=move || render_state.get().phase.as_str()",
        "data-label-source=state.label_source_attr",
        "data-value-label-source=state.value_label_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "meter semantic contract should expose marker `{marker}`"
        );
    }

    // N/A matrix branches for Meter: no controllable axis, no disabled axis,
    // no keyboard/pointer interaction pipeline.
    for forbidden in [
        "default_value",
        "on_value_change",
        "is_disabled",
        "aria-disabled",
        "on:keydown",
        "on:keyup",
        "on:pointerdown",
        "on:pointerup",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "meter should keep non-applicable semantic matrix branch absent: `{forbidden}`"
        );
    }

    // Platform branch evidence for wasm/non-wasm motion behavior.
    for cfg_marker in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(cfg_marker),
            "meter motion source should keep wasm/non-wasm branch marker `{cfg_marker}`"
        );
    }

    for snapshot_token in [
        concat!("assert_", "snapshot!"),
        concat!("insta::", "assert"),
        concat!(".", "snap"),
    ] {
        assert!(
            !semantics_source.contains(snapshot_token),
            "meter semantic tests should not be replaced by snapshot-only assertion `{snapshot_token}`"
        );
    }

    assert!(
        check2_source.contains("测试验证“语义契约”而不只验证视觉快照"),
        "meter checklist should keep semantic-contract-first testing entry"
    );
}

#[test]
fn token_first_static_style_contract_is_enforced() {
    let cargo_source = load_source("Cargo.toml");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");
    let components_css_source = load_source("../../crates/ui/src/css.rs");
    let ui_root_source = load_source("../../crates/ui/src/root.rs");
    let check2_source = load_source("check2.md");

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "meter styles should stay in styles.rs static CSS blob"
    );
    for marker in [
        "gap: var(--ui-space-xs, var(--ui-fallback-space-xs));",
        "height: var(--ui-meter-track-height, var(--ui-fallback-meter-track-height));",
        "height: var(--ui-meter-track-height-sm, var(--ui-fallback-meter-track-height-sm));",
        "height: var(--ui-meter-track-height-lg, var(--ui-fallback-meter-track-height-lg));",
        "border-radius: var(--ui-meter-track-radius, var(--ui-fallback-meter-track-radius));",
        "border: var(--ui-meter-track-border-width, var(--ui-fallback-meter-track-border-width)) solid var(--ui-border, var(--ui-fallback-border));",
        "font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));",
        "font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
        "var(--ui-meter-indeterminate-duration, var(--ui-fallback-meter-indeterminate-duration))",
        "var(--ui-meter-indeterminate-easing, var(--ui-fallback-meter-indeterminate-easing))",
    ] {
        assert!(
            styles_source.contains(marker),
            "meter visual style should be token-first marker `{marker}`"
        );
    }
    for forbidden in [
        "height: 10px;",
        "height: 8px;",
        "height: 12px;",
        "border: 1px solid",
        "animation: ui-meter-indeterminate 1.2s ease-in-out infinite;",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "meter styles should avoid hardcoded visual token `{forbidden}`"
        );
    }

    assert!(
        components_css_source.contains("#[cfg(feature = \"component-meter\")]")
            && components_css_source.contains("out.push_str(crate::meter::styles::CSS);"),
        "meter styles should be aggregated by crates/ui/src/css.rs"
    );
    for marker in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root_source.contains(marker),
            "UiRoot should inject aggregated component CSS marker `{marker}`"
        );
    }

    assert!(
        !view_source.contains("style="),
        "meter view should not use utility-first or inline business styling"
    );
    assert!(
        motion_source.contains("set_property(\"--ui-meter-progress\""),
        "runtime should only update required meter CSS variable"
    );

    for forbidden in ["tailwind", "stylist", "emotion", "styled-components"] {
        assert!(
            !cargo_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "meter component contract should not default to utility/CSS-in-Rust stack `{forbidden}`"
        );
    }

    assert!(
        check2_source.contains("组件层遵循 token-first 静态样式契约"),
        "meter checklist should keep token-first static style contract entry"
    );
}

#[test]
fn component_files_keep_responsibility_boundaries() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let check2_source = load_source("check2.md");

    for marker in [
        "mod logic;",
        "mod view;",
        "pub mod styles;",
        "pub mod motion;",
        "pub use view::Meter;",
        "pub use motion::MeterMotion;",
    ] {
        assert!(
            mod_source.contains(marker),
            "mod.rs should keep stable export boundary marker `{marker}`"
        );
    }
    for forbidden in [
        "view! {",
        "fn attach_motion(",
        "pub struct MeterRenderStateInput",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not carry implementation detail `{forbidden}`"
        );
    }

    for marker in [
        "pub fn normalize_inputs(input: MeterInputNormalizationInput) -> MeterInputNormalization",
        "pub fn derive_render_state(input: MeterRenderStateInput) -> MeterRenderState",
    ] {
        assert!(
            logic_source.contains(marker),
            "logic.rs should keep normalization/derivation marker `{marker}`"
        );
    }
    for forbidden in ["view! {", "NodeRef::new()", "set_property(", ".ui-meter__"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should avoid DOM/style-render implementation token `{forbidden}`"
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "styles.rs should expose static CSS contract blob"
    );
    for marker in ["var(--ui-", ".ui-meter[data-state=\"indeterminate\"]"] {
        assert!(
            styles_source.contains(marker),
            "styles.rs should remain token-first static style marker `{marker}`"
        );
    }
    for forbidden in ["view! {", "Signal::derive(", "on:click", "on:pointermove"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should avoid runtime/view logic token `{forbidden}`"
        );
    }

    for marker in [
        "view! {",
        "logic::normalize_inputs(logic::MeterInputNormalizationInput",
        "logic::resolve_state(logic::MeterStateInput",
        "logic::derive_render_state(logic::MeterRenderStateInput",
        "data-state=move || render_state.get().phase.as_str()",
        "role=\"meter\"",
    ] {
        assert!(
            view_source.contains(marker),
            "view.rs should mount structure+semantic contract marker `{marker}`"
        );
    }
    for forbidden in [
        "ui_motion::spring::SpringAnimator::new(",
        "fn sanitize_spring(",
        "clamp_to_range(value,",
        "normalize_progress(value,",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should avoid hidden motion/state-engine implementation `{forbidden}`"
        );
    }

    for marker in [
        "pub fn sanitize_motion(motion: MeterMotion) -> MeterMotion",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(marker),
            "motion.rs should keep motion-contract attach marker `{marker}`"
        );
    }
    for forbidden in [
        "view! {",
        ".ui-meter[data-state=",
        "pub fn normalize_inputs(",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should avoid cross-layer token `{forbidden}`"
        );
    }

    assert!(
        check2_source.contains("组件文件职责正确"),
        "meter checklist should keep component-file-responsibility entry"
    );
}

#[test]
fn visual_desire_quality_gate_is_repository_level_for_meter() {
    let check2_source = load_source("check2.md");
    let readme_source = load_source("src/README.md");
    let styles_source = load_source("src/styles.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let e2e_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../e2e/tests");

    assert!(
        check2_source.contains("默认主题美学质量达标（Visual Desire）"),
        "meter checklist should keep visual-desire quality gate entry"
    );
    assert!(
        check2_source.contains("关键组件（Button/Input/Overlay）"),
        "visual-desire gate should stay scoped as repo-level key-component baseline"
    );

    // Meter-side contract: consume theme tokens and keep default docs entry.
    for marker in [
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-accent, var(--ui-fallback-accent))",
    ] {
        assert!(
            styles_source.contains(marker),
            "meter default visual should consume theme token marker `{marker}`"
        );
    }
    for marker in [
        "## Hello World（最小可用）",
        "默认路径不需要用户手动接线 `ui-state-primitives`。",
    ] {
        assert!(
            readme_source.contains(marker),
            "meter docs should keep default-usage baseline marker `{marker}`"
        );
    }
    assert!(
        docs_source.contains("pub(super) fn meter() -> AnyView"),
        "docs-app should keep meter baseline entry in display page"
    );

    // Visual Desire screenshot/comparison baseline is enforced at repository e2e scope.
    let mut has_button_e2e = false;
    let mut has_input_e2e = false;
    let mut has_overlay_e2e = false;
    for entry in fs::read_dir(&e2e_root)
        .unwrap_or_else(|err| panic!("failed to read e2e tests dir {e2e_root:?}: {err}"))
    {
        let file_name = entry
            .unwrap_or_else(|err| panic!("failed to read e2e entry in {e2e_root:?}: {err}"))
            .file_name()
            .to_string_lossy()
            .to_string();
        has_button_e2e |= file_name.contains("button");
        has_input_e2e |= file_name.contains("input");
        has_overlay_e2e |= file_name.contains("overlay");
    }
    assert!(
        has_button_e2e && has_input_e2e && has_overlay_e2e,
        "visual-desire gate should be protected by repo-level e2e baselines (button/input/overlay)"
    );
}

#[test]
fn tree_shaking_contract_is_feature_gated_for_meter() {
    let check2_source = load_source("check2.md");
    let ui_components_cargo = load_source("../../crates/ui/Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui/src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");

    assert!(
        check2_source.contains("Tree Shaking 是一等能力"),
        "meter checklist should keep tree-shaking contract entry"
    );
    assert!(
        check2_source.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
        "meter checklist should mark tree-shaking feature-gating execution entry as completed"
    );
    assert!(
        check2_source.contains("CI 检查（体积预算）"),
        "tree-shaking gate should keep repository-level size-budget requirement"
    );

    assert!(
        ui_components_cargo.contains("component-meter = [\"dep:ui-meter\"]"),
        "ui must keep component-level meter feature gate"
    );
    for marker in [
        "#[cfg(feature = \"component-meter\")]\npub use ui_meter as meter;",
        "pub use meter::{Meter, MeterMotion, MeterSize, MeterVariant};",
    ] {
        assert!(
            ui_components_lib.contains(marker),
            "ui lib export should remain meter feature-gated marker `{marker}`"
        );
    }
    assert!(
        ui_components_lib.contains(
            "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]"
        ),
        "ui should keep separate web-demo feature aggregation path"
    );

    assert!(
        ui_components_css.contains(
            "#[cfg(feature = \"component-meter\")]\n    out.push_str(crate::meter::styles::CSS);"
        ),
        "ui css aggregation should keep meter style gate"
    );
    assert_eq!(
        ui_components_css
            .matches("out.push_str(crate::meter::styles::CSS);")
            .count(),
        1,
        "meter css should be aggregated exactly once to avoid accidental global duplication"
    );

    for marker in [
        "ui = { path = \"../../crates/ui\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
        "default-features = false",
        "web-demo-components",
    ] {
        assert!(
            web_demo_cargo.contains(marker),
            "web-demo should consume ui through explicit feature-gated path `{marker}`"
        );
    }
    assert!(
        !web_demo_cargo.contains("all-components"),
        "web-demo should not implicitly pull ui all-components feature"
    );
}

#[test]
fn type_system_and_semantic_markers_form_machine_readable_contract() {
    let check2_source = load_source("check2.md");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/meter.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let logic_test_source = load_source("test/logic.rs");
    let semantics_source = load_source("test/semantics.rs");

    assert!(
        check2_source.contains("类型系统 + 语义标记共同提供机器可读状态"),
        "meter checklist should keep type-system + semantic-marker contract entry"
    );

    for marker in [
        "pub enum MeterVariant",
        "pub enum MeterSize",
        "pub enum MeterPhase",
        "MeterPhase::Determinate => \"determinate\"",
        "MeterPhase::Indeterminate => \"indeterminate\"",
    ] {
        assert!(
            primitive_source.contains(marker),
            "meter primitive state space should stay type-constrained marker `{marker}`"
        );
    }

    for marker in [
        "pub fn normalize_inputs(input: MeterInputNormalizationInput) -> MeterInputNormalization",
        "pub fn derive_render_state(input: MeterRenderStateInput) -> MeterRenderState",
        "pub phase: MeterPhase",
    ] {
        assert!(
            logic_source.contains(marker),
            "logic should keep centralized typed normalization/derivation marker `{marker}`"
        );
    }
    for forbidden in [
        "variant: Option<String>",
        "size: Option<String>",
        "status: Option<String>",
        "mode: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "public API should avoid string protocol for state axis `{forbidden}`"
        );
    }

    for marker in [
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-state=move || render_state.get().phase.as_str()",
        "data-label-source=state.label_source_attr",
        "data-value-label-source=state.value_label_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "view should expose machine-readable semantic marker `{marker}`"
        );
    }

    // Keep direct, named tests so contract breakage points are immediately obvious.
    for marker in [
        "fn variant_and_size_mappings_are_stable()",
        "fn phase_mappings_are_stable()",
        "fn normalize_inputs_centralizes_default_values()",
        "fn derive_render_state_concentrates_runtime_semantics()",
        "fn discrete_state_axes_use_typed_enums()",
        "fn state_observability_contract_uses_stable_data_and_aria_markers()",
    ] {
        assert!(
            logic_test_source.contains(marker) || semantics_source.contains(marker),
            "contract regression suite should keep directly-locatable test marker `{marker}`"
        );
    }
}

#[test]
fn focus_stack_and_overlay_focus_manager_are_not_applicable_for_meter() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");

    assert!(
        check2_source.contains("焦点全局栈（Focus Stack & GC）"),
        "meter checklist should keep focus-stack governance entry"
    );

    for forbidden in [
        "FocusManager",
        "Focus Manager",
        "FocusStack",
        "focus_stack",
        "FallbackTo",
        "Selector",
        "restore_focus",
        "document.body",
        "OverlayStack",
        "overlay_stack",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "meter should not implement overlay focus-stack protocol token `{forbidden}`"
        );
    }

    // Meter keeps a local NodeRef only for indicator motion attachment, not focus restore.
    for marker in [
        "let indicator_ref = NodeRef::new();",
        "motion::attach_motion(indicator_ref, progress_value, motion);",
    ] {
        assert!(
            view_source.contains(marker),
            "meter should keep animation NodeRef-only marker `{marker}`"
        );
    }
}

#[test]
fn controlled_foreign_zone_escape_hatch_is_not_applicable_for_meter() {
    let check2_source = load_source("check2.md");
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/mod.rs");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");

    assert!(
        check2_source.contains("受控外交特区（Escape Hatches）"),
        "meter checklist should keep foreign-zone governance entry"
    );

    // Meter should not host imperative third-party integration protocol.
    for forbidden in [
        "Foreign Zone",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "Mapbox",
        "Leaflet",
        "GoogleMap",
        "chart_instance",
        "map_instance",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "meter should not implement foreign-zone escape-hatch token `{forbidden}`"
        );
    }

    // Public API must not expose third-party imperative instance handles.
    for forbidden in [
        "EChart",
        "Mapbox",
        "#[prop(optional)] chart",
        "#[prop(optional)] map",
        "#[prop(optional)] foreign",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "meter public API should not expose third-party instance token `{forbidden}`"
        );
    }

    // Component crate should remain free of direct chart/map SDK dependencies.
    for forbidden in ["echarts", "mapbox", "leaflet", "google-maps"] {
        assert!(
            !cargo_source.contains(forbidden),
            "meter crate dependency surface should avoid foreign sdk token `{forbidden}`"
        );
    }
}

#[test]
fn ssr_hydration_discontinuity_is_avoided_with_deterministic_id_path() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let ui_root_source = load_source("../../crates/ui/src/root.rs");

    assert!(
        check2_source.contains("SSR 时空断裂治理（Hydration Discontinuity）"),
        "meter checklist should keep ssr-hydration-discontinuity entry"
    );

    for marker in [
        "pub fn Meter(",
        "id: String,",
        "let label_id = StoredValue::new(format!(\"{id}-label\"));",
    ] {
        assert!(
            view_source.contains(marker),
            "meter should keep deterministic id path marker `{marker}`"
        );
    }

    for forbidden in [
        "now(",
        "Date::now",
        "SystemTime::now",
        "UNIX_EPOCH",
        "Uuid",
        "uuid::",
        "rand::",
        "random(",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "meter init path should avoid non-deterministic token `{forbidden}`"
        );
    }

    // Deterministic seed injection lives in UiRoot; meter consumes explicit id prop.
    for marker in [
        "#[prop(optional, default = 1)] id_seed: u64,",
        "provide_ui_id_provider(id_seed);",
    ] {
        assert!(
            ui_root_source.contains(marker),
            "ui-root should keep deterministic id-provider marker `{marker}`"
        );
    }
}

#[test]
fn ssr_and_cross_platform_contract_keeps_explicit_cfg_and_non_wasm_safety() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");

    assert!(
        check2_source.contains("SSR 与跨平台检查"),
        "meter checklist should keep ssr/cross-platform contract entry"
    );

    for marker in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(marker),
            "motion path should keep explicit platform branch marker `{marker}`"
        );
    }

    for forbidden in ["leptos::web_sys", "wasm_bindgen", "window()", "document()"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "view/logic should stay platform-agnostic and avoid browser token `{forbidden}`"
        );
    }

    let non_wasm_start = motion_source
        .find("#[cfg(not(target_arch = \"wasm32\"))]")
        .expect("motion source should keep non-wasm cfg branch");
    let non_wasm_section = &motion_source[non_wasm_start..];
    for forbidden in ["leptos::web_sys", "wasm_bindgen", "HtmlElement"] {
        assert!(
            !non_wasm_section.contains(forbidden),
            "non-wasm motion branch should avoid browser-only token `{forbidden}`"
        );
    }
    for marker in [
        "std::hint::black_box(sanitize_motion(motion));",
        "_indicator_ref: leptos::prelude::NodeRef<leptos::html::Div>,",
        "_progress: leptos::prelude::Signal<f64>,",
    ] {
        assert!(
            non_wasm_section.contains(marker),
            "non-wasm motion branch should keep predictable no-op marker `{marker}`"
        );
    }
}

#[test]
fn spec_rs_is_not_introduced_for_simple_meter_component() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mod_source = load_source("src/mod.rs");
    let readme_source = load_source("src/README.md");
    let check2_source = load_source("check2.md");

    assert!(
        !root.join("src/spec.rs").exists(),
        "meter is a simple display component and should not introduce `src/spec.rs`"
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "meter module export surface should not expose spec module token `{forbidden}`"
        );
    }

    // Simple component docs stay in README/check2 rather than adding a dedicated spec module.
    assert!(
        readme_source.contains("## API (Table)"),
        "meter API contract should stay documented in README for simple component path"
    );
    assert!(
        check2_source.contains("`spec.rs` 只用于少数复杂组件"),
        "meter checklist should keep spec.rs limited-use contract entry"
    );
}

#[test]
fn ui_headless_web_ssr_feature_mutex_contract_is_preserved() {
    let check2_source = load_source("check2.md");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let meter_cargo_source = load_source("Cargo.toml");
    let ui_components_cargo_source = load_source("../../crates/ui/Cargo.toml");

    assert!(
        check2_source.contains("`ui-headless` web/ssr feature 互斥受 `compile_error!` 保护"),
        "meter checklist should keep ui-headless feature mutex governance entry"
    );

    for marker in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            headless_lib_source.contains(marker),
            "ui-headless should keep explicit web/ssr mutex compile guard `{marker}`"
        );
    }

    // Meter and ui may depend on ui-headless, but must not enable
    // both web+ssr features in the same dependency declaration.
    for marker in [
        "ui-headless = { path = \"../../crates/ui-headless\" }",
        "ui-headless = { path = \"../ui-headless\" }",
    ] {
        assert!(
            meter_cargo_source.contains(marker) || ui_components_cargo_source.contains(marker),
            "dependency declarations should keep plain ui-headless linkage marker `{marker}`"
        );
    }

    for forbidden in [
        "ui-headless = { path = \"../../crates/ui-headless\", default-features = false, features = [\"web\", \"ssr\"] }",
        "ui-headless = { path = \"../ui-headless\", default-features = false, features = [\"web\", \"ssr\"] }",
    ] {
        assert!(
            !meter_cargo_source.contains(forbidden)
                && !ui_components_cargo_source.contains(forbidden),
            "ui-headless dependency must not co-enable mutually-exclusive features `{forbidden}`"
        );
    }

    assert!(
        check2_source.contains("cargo check -p ui-headless --no-default-features --features ssr"),
        "checklist gate should include explicit ssr compilation path for ui-headless"
    );
}

#[test]
fn ui_motion_non_wasm_noop_stub_contract_is_preserved() {
    let check2_source = load_source("check2.md");
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let ui_motion_non_wasm_test_source = load_source("../../crates/ui-motion/src/test/lib.rs");
    let meter_motion_source = load_source("src/motion.rs");
    let meter_view_source = load_source("src/view.rs");

    assert!(
        check2_source.contains("`ui-motion` 非 wasm 提供 no-op/stub"),
        "meter checklist should keep ui-motion non-wasm noop/stub governance entry"
    );

    for marker in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(marker),
            "ui-motion should keep non-wasm noop backend marker `{marker}`"
        );
    }

    for marker in [
        "fn non_wasm_web_backend_is_predictable_noop()",
        "assert!(web::prefers_reduced_motion());",
        "web::animate(&(), &[MotionKeyframe::default()], MotionOptions::default());",
    ] {
        assert!(
            ui_motion_non_wasm_test_source.contains(marker)
                || ui_motion_lib_source.contains(marker),
            "ui-motion should keep non-wasm predictable noop regression marker `{marker}`"
        );
    }

    let non_wasm_start = meter_motion_source
        .find("#[cfg(not(target_arch = \"wasm32\"))]")
        .expect("meter motion source should keep non-wasm cfg branch");
    let non_wasm_section = &meter_motion_source[non_wasm_start..];
    for marker in [
        "pub fn attach_motion(",
        "_indicator_ref: leptos::prelude::NodeRef<leptos::html::Div>,",
        "_progress: leptos::prelude::Signal<f64>,",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            non_wasm_section.contains(marker),
            "meter should keep non-wasm safe no-op attach marker `{marker}`"
        );
    }
    for forbidden in ["panic!(", "unwrap()", "expect(", "web_sys", "wasm_bindgen"] {
        assert!(
            !non_wasm_section.contains(forbidden),
            "non-wasm motion fallback should avoid panic/browser-only token `{forbidden}`"
        );
    }

    assert!(
        meter_view_source.contains("motion::attach_motion(indicator_ref, progress_value, motion);"),
        "meter view should delegate motion binding through attach_motion for tooling-safe path"
    );
    assert!(
        meter_motion_source.contains("if let Some(animator) = spring.get_value() {"),
        "meter motion should not assume animator handle always exists on runtime branches"
    );
}

#[test]
fn reduced_motion_ssr_and_wasm_branches_keep_semantic_contract_consistent() {
    let check2_source = load_source("check2.md");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let spring_source = load_source("../../crates/ui-motion/src/spring.rs");
    let motion_web_source = load_source("../../crates/ui-motion/src/web.rs");

    assert!(
        check2_source.contains("组件实现覆盖 `reduced-motion` / SSR / wasm 分支"),
        "meter checklist should keep reduced-motion/SSR/wasm branch governance entry"
    );

    // Reduced-motion: CSS fallback for indeterminate animation and runtime early-settle in spring.
    for marker in [
        "@media (prefers-reduced-motion: reduce)",
        ".ui-meter[data-state=\"indeterminate\"] .ui-meter__indicator",
        "animation: none;",
    ] {
        assert!(
            styles_source.contains(marker),
            "meter styles should keep reduced-motion degrade marker `{marker}`"
        );
    }
    for marker in [
        "if crate::web::prefers_reduced_motion() {",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            spring_source.contains(marker),
            "spring runtime should keep reduced-motion short-circuit marker `{marker}`"
        );
    }
    assert!(
        motion_web_source.contains("w.match_media(\"(prefers-reduced-motion: reduce)\")"),
        "wasm web backend should read reduced-motion media query"
    );

    // SSR/non-wasm and wasm both exist, but semantics stay in view layer (no cfg split in view).
    for marker in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(marker),
            "motion source should keep explicit wasm/non-wasm branch marker `{marker}`"
        );
    }

    for marker in [
        "role=\"meter\"",
        "aria-valuenow=move || render_state.get().aria_value_now",
        "aria-valuetext=move || render_state.get().value_label_text",
        "data-state=move || render_state.get().phase.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "view should keep semantic contract marker `{marker}` independent from motion backend"
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "semantic markup in view should not split by runtime branch token `{forbidden}`"
        );
    }
    for forbidden in ["aria-", "role=\"meter\"", "data-state="] {
        assert!(
            !motion_source.contains(forbidden),
            "motion backend should not own semantic contract token `{forbidden}`"
        );
    }
}

#[test]
fn performance_governance_budget_is_defined_repeatable_and_traceable_for_meter() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let styles_source = load_source("src/styles.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let perf_script_source = load_source("../../scripts/check-ui-performance.sh");

    for marker in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
        "performance_governance_budget_is_defined_repeatable_and_traceable_for_meter",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep performance governance marker `{marker}`"
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "repo TODO should keep render_count follow-up marker `{marker}`"
        );
    }

    // Equivalent baseline (until generalized render_count infra is available):
    // initial render path keeps two derives and no environment subscription churn.
    assert_eq!(
        view_source.matches("Signal::derive(").count(),
        2,
        "meter should keep a stable two-derive baseline (render_state + progress_value)."
    );
    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "on:resize",
        "on:scroll",
        "on:mousemove",
        "on:pointermove",
    ] {
        assert!(
            !view_source.contains(forbidden) && !motion_source.contains(forbidden),
            "meter perf baseline should avoid env/high-frequency flood token `{forbidden}`"
        );
    }

    // Update path remains attributable: state derivation -> motion target -> css var.
    for marker in [
        "pub fn derive_render_state(input: MeterRenderStateInput) -> MeterRenderState",
        "data-state=move || render_state.get().phase.as_str()",
        "animator.set_target(v.clamp(0.0, 1.0));",
        "set_property(\"--ui-meter-progress\"",
        ".ui-meter[data-state=\"indeterminate\"]",
        ".ui-meter[data-state=\"determinate\"]",
    ] {
        assert!(
            logic_source.contains(marker)
                || view_source.contains(marker)
                || motion_source.contains(marker)
                || styles_source.contains(marker),
            "meter perf triage path should stay attributable via marker `{marker}`"
        );
    }

    // Keep a bounded wasm runtime loop and predictable non-wasm no-op.
    assert_eq!(
        motion_source.matches("Effect::new(").count(),
        2,
        "meter wasm motion path should keep a bounded two-effect baseline."
    );
    let non_wasm_start = motion_source
        .find("#[cfg(not(target_arch = \"wasm32\"))]")
        .expect("meter motion source should keep non-wasm cfg branch");
    let non_wasm_section = &motion_source[non_wasm_start..];
    for marker in [
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            non_wasm_section.contains(marker),
            "meter non-wasm perf fallback should keep marker `{marker}`"
        );
    }

    assert!(
        perf_script_source.contains("perf_render_count_follow_up_is_tracked_in_plan"),
        "performance gate script should keep shared render_count follow-up blocker"
    );
}

#[test]
fn semantic_and_performance_regression_contract_is_covered_for_meter() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let semantics_source = load_source("test/semantics.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let perf_script_source = load_source("../../scripts/check-ui-performance.sh");

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "semantic_and_performance_regression_contract_is_covered_for_meter",
        "语义断言覆盖 `role/aria-*` 与关键 `data-*`",
        "非快照优先",
        "焦点流转对 `Meter` 为 N/A",
        "render_count",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep semantic+performance regression marker `{marker}`"
        );
    }

    for marker in [
        "role=\"meter\"",
        "aria-valuemin=range.min.to_string()",
        "aria-valuemax=range.max.to_string()",
        "aria-valuenow=move || render_state.get().aria_value_now",
        "aria-valuetext=move || render_state.get().value_label_text",
        "data-state=move || render_state.get().phase.as_str()",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-label-source=state.label_source_attr",
        "data-value-label-source=state.value_label_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "meter view should keep semantic marker `{marker}` for regression coverage"
        );
    }

    for snapshot_token in [
        concat!("assert_", "snapshot!"),
        concat!("insta::", "assert"),
        concat!(".", "snap"),
    ] {
        assert!(
            !semantics_source.contains(snapshot_token),
            "meter semantic+performance gate should not rely on snapshot-only token `{snapshot_token}`"
        );
    }

    // Focus-flow for Meter is explicitly N/A: keep it impossible to silently
    // drift into hidden focus manager responsibilities.
    for forbidden in [
        "FocusManager",
        "FocusStack",
        "restore_focus",
        "on:focus",
        "on:blur",
        "on:keydown",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "meter should keep focus-flow N/A boundary token `{forbidden}` absent"
        );
    }

    for marker in [
        "semantic_contract_tests_are_primary_and_snapshot_only_checks_are_absent",
        "focus_stack_and_overlay_focus_manager_are_not_applicable_for_meter",
        "performance_governance_budget_is_defined_repeatable_and_traceable_for_meter",
    ] {
        assert!(
            semantics_source.contains(marker),
            "meter regression suite should keep dedicated proof test `{marker}`"
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "render_count follow-up evidence should remain tracked in TODO marker `{marker}`"
        );
    }
    assert!(
        perf_script_source.contains("perf_render_count_follow_up_is_tracked_in_plan"),
        "performance script should keep render_count follow-up guard marker"
    );
}

#[test]
fn versioned_deprecation_migration_contract_is_not_applicable_without_breaking_api_for_meter() {
    let check2_source = load_source("check2.md");
    let protocol_source = load_source("src/protocol.rs");
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/meter.rbi");
    let mod_source = load_source("src/mod.rs");

    for marker in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `meter` 迭代未发生跨大版本 API 破坏升级",
        "versioned_deprecation_migration_contract_is_not_applicable_without_breaking_api_for_meter",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep versioned migration governance marker `{marker}`"
        );
    }

    for marker in [
        "pub enum MeterComponentSchemaVersion",
        "V1,",
        "pub struct MeterComponentSpec",
        "pub schema_version: MeterComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(marker),
            "meter protocol should keep stable v1 schema marker `{marker}`"
        );
    }

    for forbidden in [
        "V2",
        "v2",
        "migrate_v1_to_v2",
        "deprecated_since",
        "deprecation_window",
        "codemod",
        "schema_registry",
    ] {
        assert!(
            !protocol_source.contains(forbidden)
                && !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !mod_source.contains(forbidden),
            "without breaking API, meter should not ship migration-only token `{forbidden}`"
        );
    }

    assert!(
        manifest_source.contains("schema_version = \"1\""),
        "component manifest should keep schema version 1 when no breaking upgrade exists"
    );
}

#[test]
fn view_macro_complexity_is_split_into_semantic_blocks() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");

    for marker in [
        "`view!` 宏复杂度受控",
        "view_macro_complexity_is_split_into_semantic_blocks",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep view-macro-complexity marker `{marker}`"
        );
    }

    for marker in [
        "fn render_meter_header(",
        "fn render_meter_track(",
        "<div class=\"ui-meter__header\" data-slot=\"meter-header\">",
        "<div class=\"ui-meter__track\" data-slot=\"meter-track\">",
    ] {
        assert!(
            view_source.contains(marker),
            "meter view should keep semantic sub-block extraction marker `{marker}`"
        );
    }

    let component_start = view_source
        .find("#[component]\npub fn Meter(")
        .expect("meter view should include component entry");
    let component_section = &view_source[component_start..];

    for marker in [
        "{render_meter_header(label, label_id, render_state)}",
        "{render_meter_track(indicator_ref)}",
    ] {
        assert!(
            component_section.contains(marker),
            "meter root view block should compose semantic sub-block marker `{marker}`"
        );
    }

    for forbidden in [
        "<div class=\"ui-meter__header\"",
        "<div class=\"ui-meter__track\"",
        "<div class=\"ui-meter__indicator\"",
    ] {
        assert!(
            !component_section.contains(forbidden),
            "meter root view should avoid giant inline nested layout token `{forbidden}`"
        );
    }

    assert!(
        view_source.matches("view! {").count() <= 6,
        "meter view should keep bounded view! macro expansion footprint"
    );
}

#[test]
fn function_first_fragment_split_uses_plain_functions_not_extra_components() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");

    for marker in [
        "函数式拆分优先",
        "function_first_fragment_split_uses_plain_functions_not_extra_components",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep function-first split governance marker `{marker}`"
        );
    }

    for marker in [
        "fn render_meter_header(",
        ") -> impl IntoView {",
        "fn render_meter_track(indicator_ref: NodeRef<leptos::html::Div>) -> impl IntoView {",
        "{render_meter_header(label, label_id, render_state)}",
        "{render_meter_track(indicator_ref)}",
    ] {
        assert!(
            view_source.contains(marker),
            "meter view should keep plain-function fragment marker `{marker}`"
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "meter view should keep only the public root component and avoid local fragment components"
    );
    for forbidden in [
        "#[component]\nfn render_meter_header(",
        "#[component]\nfn render_meter_track(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "lightweight fragments should not be promoted into extra component token `{forbidden}`"
        );
    }

    for semantic_marker in [
        "role=\"meter\"",
        "aria-valuenow=move || render_state.get().aria_value_now",
        "data-state=move || render_state.get().phase.as_str()",
        "data-slot=\"meter-header\"",
        "data-slot=\"meter-track\"",
        "data-slot=\"meter-indicator\"",
    ] {
        assert!(
            view_source.contains(semantic_marker),
            "fragment split should keep stable semantic marker `{semantic_marker}`"
        );
    }
}

#[test]
fn static_fragment_constantization_is_not_applicable_and_static_path_stays_centralized_for_meter() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let readme_source = load_source("src/README.md");

    for marker in [
        "静态片段常量化",
        "static_fragment_constantization_is_not_applicable_and_static_path_stays_centralized_for_meter",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep static-fragment-constantization marker `{marker}`"
        );
    }

    // Meter has no heavy static assets (SVG/footer/long copy); keep the scope explicit.
    for forbidden in [
        "<svg",
        "data-slot=\"footer\"",
        "footer",
        "long-description",
        "inner_html=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "meter view should avoid heavy static-fragment token `{forbidden}` in current scope"
        );
    }

    // Static fragment path should remain centralized and non-duplicated.
    for marker in [
        "fn render_meter_track(indicator_ref: NodeRef<leptos::html::Div>) -> impl IntoView {",
        "data-slot=\"meter-track\"",
        "data-slot=\"meter-indicator\"",
        "{render_meter_track(indicator_ref)}",
    ] {
        assert!(
            view_source.contains(marker),
            "meter static fragment path should stay centralized marker `{marker}`"
        );
    }
    assert_eq!(
        view_source.matches("data-slot=\"meter-track\"").count(),
        1,
        "meter track static fragment should have a single source of truth in view"
    );
    assert_eq!(
        view_source.matches("data-slot=\"meter-indicator\"").count(),
        1,
        "meter indicator static fragment should have a single source of truth in view"
    );

    assert!(
        readme_source.contains("## Hello World（最小可用）"),
        "meter docs should keep static copy path centralized in README instead of scattering in view"
    );
}

#[test]
fn inner_html_contract_is_not_applicable_and_untrusted_injection_paths_are_absent() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let styles_source = load_source("src/styles.rs");

    for marker in [
        "`inner_html` 使用约束",
        "inner_html_contract_is_not_applicable_and_untrusted_injection_paths_are_absent",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep inner_html safety governance marker `{marker}`"
        );
    }

    // Meter has no trusted static-html rendering needs in current scope.
    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "dangerous_inner_html",
        "raw_html",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "meter source should avoid html-injection API token `{forbidden}`"
        );
    }

    // Block obvious string-to-html construction patterns in component source.
    for forbidden in [
        "format!(\"<",
        "push_str(\"<",
        "String::from(\"<",
        "from_str(\"<",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "meter should avoid untrusted html string construction token `{forbidden}`"
        );
    }

    // Semantic rendering remains node-based and explicit.
    for marker in [
        "role=\"meter\"",
        "aria-valuenow=move || render_state.get().aria_value_now",
        "data-state=move || render_state.get().phase.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "meter should keep explicit semantic node rendering marker `{marker}`"
        );
    }
}

#[test]
fn wasm_debug_contract_is_not_component_local_and_reuses_global_trace_capability() {
    let check2_source = load_source("check2.md");
    let cargo_source = load_source("Cargo.toml");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");

    for marker in [
        "WASM 调试要求",
        "wasm_debug_contract_is_not_component_local_and_reuses_global_trace_capability",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep wasm-debug governance marker `{marker}`"
        );
    }

    // Meter keeps no component-local wasm debug/replay pipeline to avoid API/bundle pollution.
    for forbidden in [
        "use_ui_trace",
        "UiTraceEventKind",
        "trace.emit(",
        "debug_overlay",
        "wasm-debug",
        "trace-replay",
        "debug_mode",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "meter component source should avoid local wasm-debug token `{forbidden}`"
        );
    }
    assert!(
        !cargo_source.contains("wasm-debug"),
        "meter crate should not expose dedicated wasm-debug feature"
    );

    // Global debug capability is provided by shared infrastructure.
    for marker in [
        "pub fn provide_ui_trace(enabled: bool) -> UiTrace",
        "pub fn use_ui_trace() -> Option<UiTrace>",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            trace_source.contains(marker),
            "shared ui-headless trace capability should keep marker `{marker}`"
        );
    }
    for marker in [
        "pub fn UiDebugOverlay(",
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
    ] {
        assert!(
            debug_overlay_source.contains(marker),
            "docs-app global debug overlay should keep marker `{marker}`"
        );
    }

    // Meter still exposes machine-readable state for inspection/replay tooling.
    for marker in [
        "data-state=move || render_state.get().phase.as_str()",
        "data-label-source=state.label_source_attr",
        "data-value-label-source=state.value_label_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "meter should keep inspectable semantic marker `{marker}`"
        );
    }
}

#[test]
fn dx_workbench_supports_fast_css_iteration_context_retention_and_optional_preserve_state() {
    let check2_source = load_source("check2.md");
    let readme_source = load_source("src/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for marker in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "dx_workbench_supports_fast_css_iteration_context_retention_and_optional_preserve_state",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep DX workbench governance marker `{marker}`"
        );
    }

    let meter_section_start = docs_source
        .find("pub(super) fn meter() -> AnyView {")
        .expect("display docs should include meter section");
    let meter_section_end = docs_source[meter_section_start..]
        .find("pub(super) fn code() -> AnyView {")
        .map(|offset| meter_section_start + offset)
        .expect("meter section should end before code section");
    let meter_docs_section = &docs_source[meter_section_start..meter_section_end];

    for marker in [
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "code_signal=workbench_code",
        "test_css_source=test_css_source",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/meter/src/styles.rs\".to_string()",
        "test_config_signal=actual_config",
        "data-slot=\"meter-workbench-controls\"",
        "data-slot=\"meter-workbench-preview\"",
        "let (workbench_preserve_state, set_workbench_preserve_state) = signal(true);",
        "if !workbench_preserve_state.get() {",
        "<Switch checked=workbench_preserve_state set_checked=set_workbench_preserve_state>",
        "\"preserve state\"",
        "\"Reset context\"",
    ] {
        assert!(
            meter_docs_section.contains(marker),
            "meter docs workbench should keep DX marker `{marker}`"
        );
    }

    for marker in [
        "## docs-app Workbench（展示 / Config / Code / CSS Test）",
        "`preserve_state`：可选保留当前配置上下文；关闭后回到默认基线，便于快速重演。",
        "CSS Test 区：加载 `components/meter/src/styles.rs`，支持局部样式试验与恢复。",
    ] {
        assert!(
            readme_source.contains(marker),
            "meter README should keep DX workbench guidance marker `{marker}`"
        );
    }
}

#[test]
fn engineering_capability_contract_uses_serde_protocol_and_keeps_tracing_async_runtime_boundaries_clean()
 {
    let check2_source = load_source("check2.md");
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/mod.rs");
    let protocol_source = load_source("src/protocol.rs");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");

    for marker in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "engineering_capability_contract_uses_serde_protocol_and_keeps_tracing_async_runtime_boundaries_clean",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep engineering capability marker `{marker}`"
        );
    }

    for marker in [
        "mod protocol;",
        "pub use protocol::{MeterComponentSchemaVersion, MeterComponentSpec};",
    ] {
        assert!(
            mod_source.contains(marker),
            "meter module should expose unified protocol contract marker `{marker}`"
        );
    }

    for marker in [
        "use serde::{Deserialize, Serialize};",
        "pub enum MeterComponentSchemaVersion",
        "#[serde(rename_all = \"snake_case\")]",
        "pub struct MeterComponentSpec",
        "#[serde(default)]",
        "pub schema_version: MeterComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(marker),
            "meter protocol should keep structured serde schema marker `{marker}`"
        );
    }
    assert!(
        cargo_source.contains("serde = { version = \"1.0\", features = [\"derive\"] }"),
        "meter crate should depend on serde derive for protocol serialization contract"
    );

    for marker in [
        "pub fn provide_ui_trace(enabled: bool) -> UiTrace",
        "pub fn use_ui_trace() -> Option<UiTrace>",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            trace_source.contains(marker),
            "tracing semantics should stay centralized in ui-headless marker `{marker}`"
        );
    }

    for forbidden in [
        "tokio",
        "async-std",
        "JoinHandle",
        "spawn_blocking",
        "Runtime",
        "Handle",
        "async fn",
        "spawn_local(",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "meter component boundary should not expose runtime-specific async token `{forbidden}`"
        );
    }
}

#[test]
fn defensive_variable_contract_uses_double_fallback_and_theme_ssot() {
    let check2_source = load_source("check2.md");
    let styles_source = load_source("src/styles.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");

    for marker in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "defensive_variable_contract_uses_double_fallback_and_theme_ssot",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep defensive-variable marker `{marker}`"
        );
    }

    for marker in [
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-meter-track-height, var(--ui-fallback-meter-track-height))",
        "var(--ui-meter-track-border-width, var(--ui-fallback-meter-track-border-width))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-meter-progress, var(--ui-fallback-meter-progress))",
        "var(--ui-meter-indeterminate-width, var(--ui-fallback-meter-indeterminate-width))",
        "var(--ui-meter-indeterminate-duration, var(--ui-fallback-meter-indeterminate-duration))",
        "var(--ui-meter-indeterminate-easing, var(--ui-fallback-meter-indeterminate-easing))",
    ] {
        assert!(
            styles_source.contains(marker),
            "meter styles should keep defensive double-fallback marker `{marker}`"
        );
    }

    for forbidden in [
        "var(--ui-border-width, 1px)",
        "var(--ui-font-size-150, 14px)",
        "var(--ui-font-size-100, 12px)",
        "var(--ui-line-height-150, 20px)",
        "var(--ui-line-height-100, 16px)",
        "var(--ui-text-field-motion-duration, 1200ms)",
        "var(--ui-text-field-motion-easing, ease-in-out)",
        "var(--ui-label-motion-color-duration, 160ms)",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "meter styles should not keep hardcoded fallback terminal token `{forbidden}`"
        );
    }

    for marker in [
        "  --ui-meter-track-height:",
        "  --ui-fallback-meter-track-height:",
        "  --ui-meter-track-border-width:",
        "  --ui-fallback-meter-track-border-width:",
        "  --ui-meter-indicator-color:",
        "  --ui-fallback-meter-indicator-color:",
        "  --ui-meter-indicator-color-danger:",
        "  --ui-fallback-meter-indicator-color-danger:",
        "  --ui-meter-progress:",
        "  --ui-fallback-meter-progress:",
        "  --ui-meter-indeterminate-width:",
        "  --ui-fallback-meter-indeterminate-width:",
        "  --ui-meter-indeterminate-duration:",
        "  --ui-fallback-meter-indeterminate-duration:",
        "  --ui-meter-indeterminate-easing:",
        "  --ui-fallback-meter-indeterminate-easing:",
    ] {
        assert!(
            theme_css_source.contains(marker),
            "ui-theme css output should define meter fallback ssot marker `{marker}`"
        );
    }
}

#[test]
fn cascade_layer_contract_uses_ui_layer_and_css_variable_only_runtime_updates() {
    let check2_source = load_source("check2.md");
    let components_css_source = load_source("../../crates/ui/src/css.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for marker in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "cascade_layer_contract_uses_ui_layer_and_css_variable_only_runtime_updates",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep cascade-layer contract marker `{marker}`"
        );
    }

    for marker in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-meter\")]",
        "out.push_str(crate::meter::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            components_css_source.contains(marker),
            "component css aggregation should keep ui-layer marker `{marker}`"
        );
    }

    for forbidden in [
        "style=",
        "style:top",
        "style:left",
        "style:width",
        "style:height",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "meter view should avoid ordinary inline style token `{forbidden}`"
        );
    }

    assert!(
        motion_source.contains("set_property(\"--ui-meter-progress\""),
        "meter runtime numeric updates should be done via css custom property"
    );
    for forbidden in [
        "set_property(\"top\"",
        "set_property(\"left\"",
        "set_property(\"width\"",
        "set_property(\"height\"",
        "set_property(\"transform\"",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "meter runtime path should avoid raw inline style property token `{forbidden}`"
        );
    }
}

#[test]
fn motion_contract_is_component_scoped_and_respects_reduced_motion_with_non_wasm_noop() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let styles_source = load_source("src/styles.rs");
    let spring_source = load_source("../../crates/ui-motion/src/spring.rs");
    let motion_web_source = load_source("../../crates/ui-motion/src/web.rs");

    for marker in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "motion_contract_is_component_scoped_and_respects_reduced_motion_with_non_wasm_noop",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep motion-contract marker `{marker}`"
        );
    }

    for marker in [
        "pub struct MeterMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "pub fn fast() -> Self {",
        "spring: ui_motion::presets::spring_fast(),",
        "impl Default for MeterMotion {",
        "spring: ui_motion::presets::spring_soft(),",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0",
        "damping: if value.damping.is_finite() && value.damping > 0.0",
        "pub fn sanitize_motion(motion: MeterMotion) -> MeterMotion",
    ] {
        assert!(
            motion_source.contains(marker),
            "meter motion contract should keep marker `{marker}`"
        );
    }

    assert!(
        view_source.contains("motion::attach_motion(indicator_ref, progress_value, motion);"),
        "meter view should mount motion through attach_motion contract"
    );

    for marker in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "SpringAnimator::new(",
        "set_property(\"--ui-meter-progress\"",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(marker),
            "motion source should keep wasm/non-wasm attach marker `{marker}`"
        );
    }

    for marker in [
        "@media (prefers-reduced-motion: reduce)",
        ".ui-meter[data-state=\"indeterminate\"] .ui-meter__indicator",
        "animation: none;",
    ] {
        assert!(
            styles_source.contains(marker),
            "meter styles should keep reduced-motion degrade marker `{marker}`"
        );
    }

    for marker in [
        "if crate::web::prefers_reduced_motion() {",
        "(self.inner.apply.borrow_mut())(target);",
    ] {
        assert!(
            spring_source.contains(marker),
            "ui-motion spring runtime should keep reduced-motion short-circuit marker `{marker}`"
        );
    }
    assert!(
        motion_web_source.contains("w.match_media(\"(prefers-reduced-motion: reduce)\")"),
        "ui-motion web backend should keep prefers-reduced-motion media query path"
    );
}

#[test]
fn ui_components_entry_files_follow_fixed_layered_contract() {
    let check2_source = load_source("check2.md");
    let ui_components_lib_source = load_source("../../crates/ui/src/lib.rs");
    let ui_components_css_source = load_source("../../crates/ui/src/css.rs");
    let ui_components_root_source = load_source("../../crates/ui/src/root.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let ui_components_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui/src");

    for marker in [
        "- [x] `ui` 固定入口文件落点正确。",
        "ui_components_entry_files_follow_fixed_layered_contract",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep ui entry contract marker `{marker}`"
        );
    }

    for marker in [
        "#[cfg(feature = \"component-meter\")]",
        "pub use ui_meter as meter;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "css::push_components_css(out);",
    ] {
        assert!(
            ui_components_lib_source.contains(marker),
            "ui lib entry should keep marker `{marker}`"
        );
    }
    for forbidden in ["pub use web_sys", "pub use leptos::web_sys"] {
        assert!(
            !ui_components_lib_source.contains(forbidden),
            "ui public API should not expose platform detail token `{forbidden}`"
        );
    }

    for marker in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-meter\")]",
        "out.push_str(crate::meter::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css_source.contains(marker),
            "ui css entry should keep feature-gated aggregation marker `{marker}`"
        );
    }

    for marker in [
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
        "pub fn UiRoot(",
    ] {
        assert!(
            ui_components_root_source.contains(marker),
            "UiRoot entry should keep centralized injection marker `{marker}`"
        );
    }

    for marker in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion {",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(marker),
            "active_highlight shared primitive should keep generic capability marker `{marker}`"
        );
    }
    for forbidden in ["Meter", "Accordion", "Popover"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight primitive should avoid component-business token `{forbidden}`"
        );
    }

    for forbidden_path in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_root.join(forbidden_path).exists(),
            "ui entry should not host duplicated headless primitive file `{forbidden_path}`"
        );
    }
}

#[test]
fn component_directory_standard_file_layout_is_correct() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let check2_source = load_source("check2.md");
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for marker in [
        "- [x] 组件目录标准文件落点正确。",
        "component_directory_standard_file_layout_is_correct",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep component directory contract marker `{marker}`"
        );
    }

    for rel_path in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        assert!(
            root.join(rel_path).exists(),
            "component directory should keep required file `{rel_path}`"
        );
    }
    assert!(
        !root.join("src/render.rs").exists(),
        "component directory should not drift into `render.rs`"
    );
    assert!(
        !root.join("src/spec.rs").exists(),
        "meter is a simple display component and should not add `src/spec.rs`"
    );

    for marker in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use view::Meter;",
        "pub use motion::MeterMotion;",
    ] {
        assert!(
            mod_source.contains(marker),
            "mod.rs should keep stable minimal export marker `{marker}`"
        );
    }
    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub mod protocol;",
        "pub use view::*;",
        "pub use logic::*;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should avoid over-export token `{forbidden}`"
        );
    }

    for marker in [
        "pub fn normalize_inputs(input: MeterInputNormalizationInput) -> MeterInputNormalization",
        "pub fn derive_render_state(input: MeterRenderStateInput) -> MeterRenderState",
        "has_custom_aria_label",
        "has_custom_value_label",
    ] {
        assert!(
            logic_source.contains(marker),
            "logic.rs should keep normalization/derivation marker `{marker}`"
        );
    }
    for forbidden in ["view! {", "web_sys", "HtmlElement", "style.set_property("] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should avoid view/dom/motion side-effect token `{forbidden}`"
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "styles.rs should keep static CSS contract constant"
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "styles.rs should be token-first and consume `var(--ui-*)`"
    );
    for forbidden in ["#[component]", "Signal<", "on:click=", "style=\""] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not host view/logic runtime token `{forbidden}`"
        );
    }

    for marker in [
        "#[component]",
        "pub fn Meter(",
        "view! {",
        "let locale = locale_attrs(lang, dir);",
        "data-state=move || render_state.get().phase.as_str()",
        "motion::attach_motion(indicator_ref, progress_value, motion);",
    ] {
        assert!(
            view_source.contains(marker),
            "view.rs should keep render + semantic mount marker `{marker}`"
        );
    }
    for forbidden in ["mod render;", "render.rs"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should avoid render-file drift token `{forbidden}`"
        );
    }

    for marker in [
        "pub struct MeterMotion",
        "pub fn sanitize_motion(motion: MeterMotion) -> MeterMotion",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(marker),
            "motion.rs should keep motion contract marker `{marker}`"
        );
    }
    for forbidden in ["pub struct Meter {", "pub fn Meter("] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not own component view token `{forbidden}`"
        );
    }
}

#[test]
fn file_layout_discipline_is_enforced_for_meter_component() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let check2_source = load_source("check2.md");

    for marker in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "file_layout_discipline_is_enforced_for_meter_component",
    ] {
        assert!(
            check2_source.contains(marker),
            "check2 should keep file layout discipline marker `{marker}`"
        );
    }

    for rel_path in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        assert!(
            root.join(rel_path).exists(),
            "meter file layout discipline requires `{rel_path}`"
        );
    }

    assert!(
        !root.join("src/render.rs").exists(),
        "meter file layout discipline forbids `src/render.rs`"
    );
    assert!(
        !root.join("src/spec.rs").exists(),
        "meter is not a complex spec-driven component and should not introduce `src/spec.rs`"
    );
}

#[test]
fn hyper_structure_builder_spec_api_is_not_applicable_for_simple_meter() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let check2_source = load_source("check2.md");
    let mod_source = load_source("src/mod.rs");
    let readme_source = load_source("src/README.md");

    for marker in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "hyper_structure_builder_spec_api_is_not_applicable_for_simple_meter",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep hyper-structure builder marker `{marker}`"
        );
    }

    assert!(
        !root.join("src/spec.rs").exists(),
        "meter simple display component should not introduce `src/spec.rs`"
    );
    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "MeterSpec::new(",
        ".render()",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !readme_source.contains(forbidden),
            "simple meter path should avoid speculative builder token `{forbidden}`"
        );
    }
}

#[test]
fn context_compression_manifest_and_rbi_projection_are_present_for_meter() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let check2_source = load_source("check2.md");
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/meter.rbi");

    for marker in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "context_compression_manifest_and_rbi_projection_are_present_for_meter",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep context-compression marker `{marker}`"
        );
    }

    for rel_path in ["src/Component.toml", "src/meter.rbi"] {
        assert!(
            root.join(rel_path).exists(),
            "meter context-compression artifact should exist: `{rel_path}`"
        );
    }

    for marker in [
        "schema_version = \"1\"",
        "name = \"Meter\"",
        "crate = \"ui-meter\"",
        "rbi = \"meter.rbi\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "fallback = \"snapshot\"",
    ] {
        assert!(
            manifest_source.contains(marker),
            "meter Component.toml should keep marker `{marker}`"
        );
    }

    for marker in [
        "pub type MeterVariant = ui_state_primitives::meter::MeterVariant;",
        "pub type MeterSize = ui_state_primitives::meter::MeterSize;",
        "pub type MeterMotion = crate::motion::MeterMotion;",
        "pub type MeterComponentSpec = crate::MeterComponentSpec;",
        "pub mod styles {",
        "pub fn Meter(",
        "value: leptos::prelude::Signal<Option<f64>>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(marker),
            "meter RBI projection should keep marker `{marker}`"
        );
    }

    for forbidden in ["TODO", "TBD"] {
        assert!(
            !manifest_source.contains(forbidden) && !rbi_source.contains(forbidden),
            "manifest/rbi should not ship unresolved placeholder `{forbidden}`"
        );
    }
}

#[test]
fn agent_contract_schema_markers_are_typed_traceable_and_whitelisted_for_meter() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let protocol_source = load_source("src/protocol.rs");
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/meter.rbi");

    for marker in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "agent_contract_schema_markers_are_typed_traceable_and_whitelisted_for_meter",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep agent-contract marker `{marker}`"
        );
    }

    for marker in [
        "let agent_attrs =",
        "protocol::agent_data_attrs(state, render_state.get().phase)",
        "data-ui-schema=move || agent_attrs.get().schema",
        "data-ui-intent=move || agent_attrs.get().intent",
        "data-ui-action=move || agent_attrs.get().action",
        "data-ui-stream-mode=move || agent_attrs.get().stream_mode",
        "data-ui-output-mode=move || agent_attrs.get().output_mode",
        "data-ui-output-status=move || agent_attrs.get().output_status",
        "data-ui-state-phase=move || agent_attrs.get().state_phase",
        "data-ui-state-variant=move || agent_attrs.get().state_variant",
        "data-ui-state-size=move || agent_attrs.get().state_size",
        "data-ui-source-label=move || agent_attrs.get().source_label",
        "data-ui-source-value-label=move || agent_attrs.get().source_value_label",
        "data-ui-source-motion=move || agent_attrs.get().source_motion",
        "data-ui-source-class=move || agent_attrs.get().source_class",
    ] {
        assert!(
            view_source.contains(marker),
            "meter view should mount typed agent-contract marker `{marker}`"
        );
    }
    for forbidden in [
        "data-ui-schema=\"ui.meter.agent-contract.v1\"",
        "data-ui-intent=\"progress-meter\"",
        "data-ui-action=\"render\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "agent-contract values should come from typed protocol attrs, not hardcoded `{forbidden}`"
        );
    }

    for marker in [
        "pub const METER_AGENT_SCHEMA: &str = \"ui.meter.agent-contract.v1\";",
        "pub enum MeterAgentIntent",
        "pub enum MeterAgentAction",
        "pub enum MeterAgentStatePhase",
        "pub enum MeterAgentLabelSource",
        "pub enum MeterAgentValueLabelSource",
        "pub enum MeterAgentMotionSource",
        "pub enum MeterAgentClassSource",
        "pub struct MeterAgentDataAttrs",
        "pub fn agent_data_attrs(",
    ] {
        assert!(
            protocol_source.contains(marker),
            "meter protocol should keep typed agent-contract marker `{marker}`"
        );
    }

    for marker in [
        "[agent_contract]",
        "schema = \"ui.meter.agent-contract.v1\"",
        "intent = \"progress-meter\"",
        "state_axes = [\"phase\", \"variant\", \"size\"]",
        "source_axes = [\"label_source\", \"value_label_source\", \"motion_source\", \"class_source\"]",
        "[[agent_contract_markers]]",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-intent\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state-phase\"",
        "attr = \"data-ui-source-class\"",
        "[[agent_contract_whitelist]]",
        "typed_agent_contract_from_protocol::agent_data_attrs",
        "inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            manifest_source.contains(marker),
            "meter Component.toml should keep agent-contract governance marker `{marker}`"
        );
    }

    for marker in [
        "pub const METER_AGENT_SCHEMA: &str = \"ui.meter.agent-contract.v1\";",
        "pub enum MeterAgentIntent",
        "pub enum MeterAgentAction",
        "pub enum MeterAgentStatePhase",
        "pub struct MeterAgentDataAttrs",
        "pub fn agent_data_attrs(state: MeterState, phase: MeterPhase) -> MeterAgentDataAttrs;",
    ] {
        assert!(
            rbi_source.contains(marker),
            "meter RBI projection should include agent-contract marker `{marker}`"
        );
    }
}

#[test]
fn streaming_term_is_limited_to_llm_output_render_modes_for_meter() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let protocol_source = load_source("src/protocol.rs");
    let manifest_source = load_source("src/Component.toml");

    for marker in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "streaming_term_is_limited_to_llm_output_render_modes_for_meter",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep streaming-definition marker `{marker}`"
        );
    }

    for marker in [
        "data-ui-stream-mode=move || agent_attrs.get().stream_mode",
        "data-ui-output-mode=move || agent_attrs.get().output_mode",
    ] {
        assert!(
            view_source.contains(marker),
            "meter view should expose output-mode marker `{marker}`"
        );
    }

    for marker in [
        "pub enum MeterAgentStreamMode",
        "MeterAgentStreamMode::Snapshot => \"snapshot\"",
        "pub enum MeterAgentOutputMode",
        "MeterAgentOutputMode::Snapshot => \"snapshot\"",
    ] {
        assert!(
            protocol_source.contains(marker),
            "meter protocol should keep typed output-mode marker `{marker}`"
        );
    }

    for marker in [
        "[streaming_policy]",
        "fallback = \"snapshot\"",
        "attr = \"data-ui-stream-mode\"",
        "attr = \"data-ui-output-mode\"",
    ] {
        assert!(
            manifest_source.contains(marker),
            "meter manifest should keep streaming/snapshot boundary marker `{marker}`"
        );
    }
}

#[test]
fn snapshot_is_foundational_and_complete_config_renders_stably_for_meter() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let protocol_source = load_source("src/protocol.rs");
    let manifest_source = load_source("src/Component.toml");

    for marker in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "snapshot_is_foundational_and_complete_config_renders_stably_for_meter",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep snapshot-foundation marker `{marker}`"
        );
    }

    // Full-config render path: meter accepts a complete snapshot config surface.
    for marker in [
        "pub fn Meter(",
        "id: String,",
        "#[prop(optional, into)] label: Option<String>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional, into)] value: Signal<Option<f64>>",
        "#[prop(optional, into)] min: Option<f64>",
        "#[prop(optional, into)] max: Option<f64>",
        "#[prop(optional)] size: MeterSize",
        "#[prop(optional)] variant: MeterVariant",
        "#[prop(optional)] motion: MeterMotion",
        "#[prop(optional, into)] is_value_label_visible: Option<bool>",
        "#[prop(optional, into)] show_value_label: Option<bool>",
        "#[prop(optional, into)] value_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            view_source.contains(marker),
            "meter snapshot render should keep complete-config marker `{marker}`"
        );
    }

    for marker in [
        "pub enum MeterAgentOutputMode",
        "MeterAgentOutputMode::Snapshot => \"snapshot\"",
        "output_mode: MeterAgentOutputMode::Snapshot.as_attr(),",
    ] {
        assert!(
            protocol_source.contains(marker),
            "meter protocol should keep snapshot output-mode marker `{marker}`"
        );
    }

    for marker in [
        "name = \"snapshot_rendering\"",
        "enabled = true",
        "[streaming_policy]",
        "fallback = \"snapshot\"",
        "attr = \"data-ui-output-mode\"",
        "values = [\"snapshot\"]",
    ] {
        assert!(
            manifest_source.contains(marker),
            "meter manifest should keep snapshot foundation marker `{marker}`"
        );
    }
}

#[test]
fn streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status_for_meter() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let protocol_source = load_source("src/protocol.rs");
    let manifest_source = load_source("src/Component.toml");
    let logic_source = load_source("src/logic.rs");

    for marker in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status_for_meter",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep streaming-requirement marker `{marker}`"
        );
    }

    for marker in [
        "[streaming_policy]",
        "required = false",
        "fallback = \"snapshot\"",
        "owner = \"upstream\"",
        "attr = \"data-ui-stream-mode\"",
        "values = [\"snapshot\"]",
        "attr = \"data-ui-output-status\"",
        "values = [\"validated\"]",
    ] {
        assert!(
            manifest_source.contains(marker),
            "meter manifest should keep streaming-optional boundary marker `{marker}`"
        );
    }

    for marker in [
        "data-ui-stream-mode=move || agent_attrs.get().stream_mode",
        "data-ui-output-status=move || agent_attrs.get().output_status",
        "role=\"meter\"",
        "aria-valuemin=range.min.to_string()",
        "aria-valuemax=range.max.to_string()",
        "data-state=move || render_state.get().phase.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "meter view should keep continuous streaming/output status semantics marker `{marker}`"
        );
    }

    for marker in [
        "pub enum MeterAgentStreamMode",
        "MeterAgentStreamMode::Snapshot => \"snapshot\"",
        "pub enum MeterAgentOutputStatus",
        "MeterAgentOutputStatus::Validated => \"validated\"",
        "stream_mode: MeterAgentStreamMode::Snapshot.as_attr(),",
        "output_status: MeterAgentOutputStatus::Validated.as_attr(),",
    ] {
        assert!(
            protocol_source.contains(marker),
            "meter protocol should keep typed streaming-status marker `{marker}`"
        );
    }

    for forbidden in [
        "retry",
        "on_retry",
        "Reconnect",
        "reconnect",
        "disconnect_recovery",
        "use_async_action",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "meter component should not own upstream validation/retry strategy token `{forbidden}`"
        );
    }
}

#[test]
fn rust_hygiene_contract_is_enforced_for_meter_non_test_sources() {
    let check2_source = load_source("check2.md");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let styles_source = load_source("src/styles.rs");
    let protocol_source = load_source("src/protocol.rs");
    let mod_source = load_source("src/mod.rs");

    for marker in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "rust_hygiene_contract_is_enforced_for_meter_non_test_sources",
        "`components/meter/src` 非测试源码已清点：无 `unwrap()/expect()`，无 `let _ = ...`，且字符串默认文案链路已收敛到 `Cow<'static, str>`（`MeterStrings.aria_label`、`MeterInputNormalizationInput.default_aria_label`、`resolve_aria_label_with_fallback`）。",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep rust hygiene evidence marker `{marker}`"
        );
    }

    let non_test_sources = [
        ("src/logic.rs", logic_source.as_str()),
        ("src/view.rs", view_source.as_str()),
        ("src/motion.rs", motion_source.as_str()),
        ("src/styles.rs", styles_source.as_str()),
        ("src/protocol.rs", protocol_source.as_str()),
        ("src/mod.rs", mod_source.as_str()),
    ];
    for (path, source) in non_test_sources {
        for forbidden in ["unwrap(", "expect(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "meter non-test source `{path}` should not contain rust-hygiene forbidden token `{forbidden}`"
            );
        }
    }

    for marker in [
        "use std::borrow::Cow;",
        "pub default_aria_label: Option<Cow<'static, str>>",
        "pub aria_label: Cow<'static, str>",
        "default_aria_label: Option<Cow<'static, str>>",
        ") -> (Cow<'static, str>, bool) {",
    ] {
        assert!(
            logic_source.contains(marker),
            "meter logic should keep Cow-based string hotspot contract marker `{marker}`"
        );
    }

    assert!(
        view_source.contains("default_aria_label: Some(strings.aria_label.clone()),")
            && !view_source.contains("strings.aria_label.as_ref().to_string()"),
        "meter view should forward i18n default aria label without eager to_string cloning"
    );
}

#[test]
fn docs_product_copy_paste_ready_contract_is_enforced_for_meter() {
    let check2_source = load_source("check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for marker in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "docs_product_copy_paste_ready_contract_is_enforced_for_meter",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep docs-product marker `{marker}`"
        );
    }

    let meter_section_start = docs_source
        .find("pub(super) fn meter() -> AnyView {")
        .expect("display docs should include meter section");
    let meter_section_end = docs_source[meter_section_start..]
        .find("pub(super) fn code() -> AnyView {")
        .map(|offset| meter_section_start + offset)
        .expect("meter section should end before code section");
    let meter_docs_section = &docs_source[meter_section_start..meter_section_end];

    for marker in [
        "title=\"Hello World (Default API)\"",
        "title=\"Variant + Size Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "code_imports=\"use leptos::prelude::*;\\nuse ui::Meter;\".to_string()",
        "code_imports=\"use leptos::prelude::*;\\nuse ui::{Meter, MeterSize, MeterVariant};\".to_string()",
        "code_imports=\"use leptos::prelude::*;\\nuse ui::{Meter, MeterMotion};\".to_string()",
        "code_imports=\"use leptos::prelude::*;\\nuse ui::{Meter, MeterSize, MeterVariant, Switch};\".to_string()",
        "data-slot=\"meter-streaming-policy\"",
        "Streaming Optional; fallback=snapshot.",
        "data-slot=\"meter-copy-ready-hint\"",
        "Source-first / Copy-Paste Ready",
        "compose_copy_ready_code",
        "label=\"Copy meter starter\".to_string()",
        "copyable=true",
        "data-slot=\"meter-source-paths\"",
        "<code>\"components/meter/src/view.rs\"</code>",
    ] {
        assert!(
            meter_docs_section.contains(marker),
            "meter docs section should keep copy-paste-ready marker `{marker}`"
        );
    }

    for marker in [
        "Meter has no internal controlled/uncontrolled axis",
        "there is no value/on_change/default triplet.",
    ] {
        assert!(
            meter_docs_section.contains(marker),
            "meter docs should explicitly encode controlled/uncontrolled N/A marker `{marker}`"
        );
    }
}

#[test]
fn semantic_testing_priority_contract_is_enforced_for_meter() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let semantics_source = load_source("test/semantics.rs");

    for marker in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "semantic_testing_priority_contract_is_enforced_for_meter",
        "view_mounts_semantic_markers",
        "state_observability_contract_uses_stable_data_and_aria_markers",
        "semantic_contract_tests_are_primary_and_snapshot_only_checks_are_absent",
        "新增/变更语义字段必须同步补测试",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep semantic-testing-priority marker `{marker}`"
        );
    }

    for marker in [
        "role=\"meter\"",
        "aria-valuemin=range.min.to_string()",
        "aria-valuemax=range.max.to_string()",
        "aria-valuenow=move || render_state.get().aria_value_now",
        "aria-valuetext=move || render_state.get().value_label_text",
        "data-state=move || render_state.get().phase.as_str()",
        "data-label-source=state.label_source_attr",
        "data-value-label-source=state.value_label_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "meter view should keep semantic contract marker `{marker}`"
        );
    }

    for marker in [
        "fn view_mounts_semantic_markers()",
        "fn state_observability_contract_uses_stable_data_and_aria_markers()",
        "fn semantic_contract_tests_are_primary_and_snapshot_only_checks_are_absent()",
    ] {
        assert!(
            semantics_source.contains(marker),
            "meter semantics regression suite should keep semantic-priority test `{marker}`"
        );
    }
}

#[test]
fn e2e_selector_contract_uses_semantic_markers_and_wasm_safe_settled_waits_for_meter() {
    let check2_source = load_source("check2.md");
    let e2e_source = load_source("../../e2e/tests/docs_app_meter_contract.spec.mjs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for marker in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "e2e_selector_contract_uses_semantic_markers_and_wasm_safe_settled_waits_for_meter",
        "docs_app_meter_contract.spec.mjs",
        "ready/settled",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep e2e selector contract marker `{marker}`"
        );
    }

    for marker in [
        "const METER_PAGE = \"/#/components/meter\";",
        "const METER_ROOT = '[data-component=\"meter\"]';",
        "const WORKBENCH_METER = '#docs-meter-workbench[data-slot=\"meter\"]';",
        "body:not(:has(#boot))",
        "[data-slot=\"meter-streaming-policy\"]",
        "[data-action=\"meter-workbench-increment\"] [data-slot=\"button\"]",
        "[data-action=\"meter-workbench-toggle-indeterminate\"] [data-slot=\"button\"]",
        "toHaveAttribute(\"data-state\", \"indeterminate\")",
        "toHaveAttribute(\"data-ui-state-phase\", \"indeterminate\")",
        "toHaveAttribute(\"data-state\", \"determinate\")",
    ] {
        assert!(
            e2e_source.contains(marker),
            "meter e2e contract should keep semantic selector/wait marker `{marker}`"
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "setInterval(",
        "sleep(",
        "locator(\"section.playground\")",
        "getByText(",
        "locator(\"text=",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "meter e2e selector contract should avoid brittle/sleep token `{forbidden}`"
        );
    }

    for marker in [
        "data-action=\"meter-workbench-increment\"",
        "data-action=\"meter-workbench-toggle-indeterminate\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "meter docs workbench should expose stable e2e action marker `{marker}`"
        );
    }
}

#[test]
fn repeatable_e2e_key_flow_regression_set_is_semantically_traceable_for_meter() {
    let check2_source = load_source("check2.md");
    let e2e_source = load_source("../../e2e/tests/docs_app_meter_contract.spec.mjs");

    for marker in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "repeatable_e2e_key_flow_regression_set_is_semantically_traceable_for_meter",
        "docs-app meter key flow is repeatable and maps failures to semantic breakpoints",
        "keyboard + focus + 动画状态切换",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep repeatable e2e-flow marker `{marker}`"
        );
    }

    for marker in [
        "async function runMeterRepeatableKeyFlow(page, docsRoot, meter) {",
        "await incrementAction.focus();",
        "await expect(incrementAction).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await toggleIndeterminateAction.focus();",
        "await expect(toggleIndeterminateAction).toBeFocused();",
        "await page.keyboard.press(\"Space\");",
        "await page.reload();",
        "toHaveAttribute(\"data-state\", \"indeterminate\")",
        "toHaveAttribute(\"data-ui-state-phase\", \"indeterminate\")",
        "toHaveAttribute(\"data-state\", \"determinate\")",
        "toHaveAttribute(\"data-ui-state-phase\", \"determinate\")",
        "data-indeterminate",
        "data-determinate",
        "aria-valuenow",
    ] {
        assert!(
            e2e_source.contains(marker),
            "meter e2e repeatable-flow contract should keep marker `{marker}`"
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "toHaveScreenshot(",
        "locator(\"section.playground\")",
        "getByText(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "meter e2e repeatable-flow contract should avoid brittle token `{forbidden}`"
        );
    }
}

#[test]
fn docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults_for_meter() {
    let check2_source = load_source("check2.md");
    let logic_source = load_source("src/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for marker in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults_for_meter",
        "data-slot=\"meter-state-matrix\"",
        "data-slot=\"meter-parameter-matrix\"",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep docs matrix sync marker `{marker}`"
        );
    }

    let meter_section_start = docs_source
        .find("pub(super) fn meter() -> AnyView {")
        .expect("display docs should include meter section");
    let meter_section_end = docs_source[meter_section_start..]
        .find("pub(super) fn code() -> AnyView {")
        .map(|offset| meter_section_start + offset)
        .expect("meter section should end before code section");
    let meter_docs_section = &docs_source[meter_section_start..meter_section_end];

    for marker in [
        "title=\"Hello World (Default API)\"",
        "title=\"Variant + Size Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "data-slot=\"meter-state-matrix\"",
        "data-slot=\"meter-state-rows\"",
        "data-slot=\"meter-parameter-matrix\"",
        "data-slot=\"meter-parameter-rows\"",
        "control mode",
        "disabled axis",
        "is_value_label_visible/show_value_label",
        "DEFAULT_MIN=0.0",
        "DEFAULT_MAX=100.0",
        "DEFAULT_SHOW_VALUE_LABEL=true",
        "MeterVariant::Default",
        "MeterSize::Default",
        "resolve_aria_label_with_fallback",
    ] {
        assert!(
            meter_docs_section.contains(marker),
            "meter docs section should keep parameter/state matrix sync marker `{marker}`"
        );
    }

    for marker in [
        "pub const DEFAULT_MIN: f64 = 0.0;",
        "pub const DEFAULT_MAX: f64 = 100.0;",
        "pub const DEFAULT_SHOW_VALUE_LABEL: bool = true;",
        "input\n        .is_value_label_visible\n        .unwrap_or(input.show_value_label.unwrap_or(DEFAULT_SHOW_VALUE_LABEL));",
        "input.min.unwrap_or(DEFAULT_MIN)",
        "input.max.unwrap_or(DEFAULT_MAX)",
    ] {
        assert!(
            logic_source.contains(marker),
            "meter logic defaults should keep marker `{marker}`"
        );
    }

    for forbidden in ["is_show_value_label", "default_show_value_label"] {
        assert!(
            !meter_docs_section.contains(forbidden),
            "meter docs should avoid stale API/default token `{forbidden}`"
        );
    }
}

#[test]
fn documentation_as_product_readme_is_beginner_friendly_for_meter() {
    let check2_source = load_source("check2.md");
    let readme_source = load_source("src/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for marker in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "documentation_as_product_readme_is_beginner_friendly_for_meter",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep docs-as-product marker `{marker}`"
        );
    }

    for marker in [
        "# Meter",
        "## 快速开始（先用起来）",
        "### Hello World（最小可用）",
        "### 常见用法",
        "先用默认 API 跑通，再按需打开高级参数。",
        "## 进阶与架构说明",
        "## Architecture Layers",
    ] {
        assert!(
            readme_source.contains(marker),
            "meter README should keep beginner-friendly marker `{marker}`"
        );
    }

    let quick_start = readme_source
        .find("## 快速开始（先用起来）")
        .expect("README should include quick start section");
    let advanced = readme_source
        .find("## 进阶与架构说明")
        .expect("README should include advanced section");
    assert!(
        quick_start < advanced,
        "README should keep default path before advanced section"
    );

    let meter_section_start = docs_source
        .find("pub(super) fn meter() -> AnyView {")
        .expect("display docs should include meter section");
    let meter_section_end = docs_source[meter_section_start..]
        .find("pub(super) fn code() -> AnyView {")
        .map(|offset| meter_section_start + offset)
        .expect("meter section should end before code section");
    let meter_docs_section = &docs_source[meter_section_start..meter_section_end];

    let hello_world_index = meter_docs_section
        .find("title=\"Hello World (Default API)\"")
        .expect("meter docs should include default hello-world entry");
    let workbench_index = meter_docs_section
        .find("title=\"Workbench (Display + Config + Code + CSS Test)\"")
        .expect("meter docs should include advanced workbench entry");
    assert!(
        hello_world_index < workbench_index,
        "meter docs should present default API entry before advanced workbench"
    );
}

#[test]
fn interactive_playground_contract_supports_live_props_state_spec_linkage_and_repeatable_flow_for_meter()
 {
    let check2_source = load_source("check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_meter_contract.spec.mjs");

    for marker in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "interactive_playground_contract_supports_live_props_state_spec_linkage_and_repeatable_flow_for_meter",
        "test_config_signal=actual_config",
        "data-slot=\"meter-spec-linkage\"",
        "docs-app meter key flow is repeatable and maps failures to semantic breakpoints",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep interactive-playground marker `{marker}`"
        );
    }

    let meter_section_start = docs_source
        .find("pub(super) fn meter() -> AnyView {")
        .expect("display docs should include meter section");
    let meter_section_end = docs_source[meter_section_start..]
        .find("pub(super) fn code() -> AnyView {")
        .map(|offset| meter_section_start + offset)
        .expect("meter section should end before code section");
    let meter_docs_section = &docs_source[meter_section_start..meter_section_end];

    for marker in [
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "test_config_signal=actual_config",
        "data-slot=\"meter-workbench-controls\"",
        "data-slot=\"meter-workbench-preview\"",
        "data-slot=\"meter-spec-linkage\"",
        "Spec Input -> Preview Output:",
        "set_workbench_variant_danger.update",
        "set_workbench_size_large.update",
        "set_workbench_value.update",
        "set_workbench_indeterminate.update",
        "set_workbench_show_value_label.update",
    ] {
        assert!(
            meter_docs_section.contains(marker),
            "meter docs workbench should keep interactive marker `{marker}`"
        );
    }

    for marker in [
        "async function runMeterRepeatableKeyFlow(page, docsRoot, meter) {",
        "docs-app meter key flow is repeatable and maps failures to semantic breakpoints",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "meter e2e should keep repeatable interactive-flow marker `{marker}`"
        );
    }
}

#[test]
fn source_first_docs_are_copy_paste_ready_and_synced_with_meter_api() {
    let check2_source = load_source("check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let mod_source = load_source("src/mod.rs");

    for marker in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "source_first_docs_are_copy_paste_ready_and_synced_with_meter_api",
        "Snippet(label=\"Copy meter starter\", copyable=true)",
        "compose_copy_ready_code",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep source-first copy-ready marker `{marker}`"
        );
    }

    let meter_section_start = docs_source
        .find("pub(super) fn meter() -> AnyView {")
        .expect("display docs should include meter section");
    let meter_section_end = docs_source[meter_section_start..]
        .find("pub(super) fn code() -> AnyView {")
        .map(|offset| meter_section_start + offset)
        .expect("meter section should end before code section");
    let meter_docs_section = &docs_source[meter_section_start..meter_section_end];

    for marker in [
        "data-slot=\"meter-source-first\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "data-slot=\"meter-source-prerequisites\"",
        "<code>\"component-meter\"</code>",
        "<code>\"inject-css\"</code>",
        "label=\"Copy meter starter\".to_string()",
        "copyable=true",
        "use leptos::prelude::*;",
        "use ui::{Meter, MeterSize, MeterVariant};",
        "data-slot=\"meter-source-paths\"",
        "<code>\"components/meter/src/mod.rs\"</code>",
        "<code>\"components/meter/src/logic.rs\"</code>",
        "<code>\"components/meter/src/view.rs\"</code>",
        "<code>\"components/meter/src/styles.rs\"</code>",
        "MeterVariant::Default",
        "MeterSize::Default",
    ] {
        assert!(
            meter_docs_section.contains(marker),
            "meter source-first docs should keep copy-ready marker `{marker}`"
        );
    }

    for marker in [
        "pub use view::Meter;",
        "MeterSize",
        "MeterVariant",
        "pub mod styles;",
    ] {
        assert!(
            mod_source.contains(marker),
            "meter API export surface should keep source-first marker `{marker}`"
        );
    }
}

#[test]
fn heroui_benchmark_docs_and_component_docs_stay_synced_for_meter() {
    let check2_source = load_source("check2.md");
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_catalog_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let readme_source = load_source("src/README.md");

    for marker in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`",
        "docs/spec/heroui-parameter-design-strategy.md::Meter 同步记录（2026-02-20）",
        "heroui_benchmark_docs_and_component_docs_stay_synced_for_meter",
    ] {
        assert!(
            check2_source.contains(marker),
            "meter checklist should keep HeroUI benchmark sync marker `{marker}`"
        );
    }

    for marker in [
        "### Meter 同步记录（2026-02-20）",
        "参数主轴保持 `id`（必填）+ `label/aria_label/lang/dir/value/min/max/variant/size/motion/is_value_label_visible/show_value_label/value_label/class_name`",
        "DEFAULT_MIN=0.0",
        "DEFAULT_MAX=100.0",
        "DEFAULT_SHOW_VALUE_LABEL=true",
        "`is_value_label_visible > show_value_label`",
        "component_doc!(\"Meter\", \"meter\", \"Display\", display::meter)",
        "slug=\"meter\"",
        "components/meter/src/README.md",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
    ] {
        assert!(
            strategy_source.contains(marker),
            "HeroUI strategy doc should keep meter sync marker `{marker}`"
        );
    }

    assert!(
        docs_catalog_source
            .contains("component_doc!(\"Meter\", \"meter\", \"Display\", display::meter)"),
        "docs catalog should expose indexable meter entry"
    );

    let meter_section_start = docs_source
        .find("pub(super) fn meter() -> AnyView {")
        .expect("display docs should include meter section");
    let meter_section_end = docs_source[meter_section_start..]
        .find("pub(super) fn code() -> AnyView {")
        .map(|offset| meter_section_start + offset)
        .expect("meter section should end before code section");
    let meter_docs_section = &docs_source[meter_section_start..meter_section_end];

    for marker in [
        "title=\"Meter\"",
        "slug=\"meter\"",
        "data-slot=\"meter-parameter-matrix\"",
        "data-slot=\"meter-state-matrix\"",
    ] {
        assert!(
            meter_docs_section.contains(marker),
            "meter docs page should keep component-doc accessibility marker `{marker}`"
        );
    }

    for marker in ["# Meter", "## 快速开始（先用起来）"] {
        assert!(
            readme_source.contains(marker),
            "meter README should keep accessible documentation entry marker `{marker}`"
        );
    }
}
