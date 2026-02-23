use std::fs;
use std::path::Path;

fn workspace_dir() -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"))
        .to_path_buf()
}

fn legend_component_src_dir() -> std::path::PathBuf {
    workspace_dir().join("components/legend/src")
}

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = if let Some(rest) = rel_path.strip_prefix("src/legend/") {
        legend_component_src_dir().join(rest)
    } else {
        manifest_dir.join(rel_path)
    };
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn legend_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/legend/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Legend internals should stay private; found `{needle}`."
        );
    }

    assert!(
        source.contains("pub use logic::{"),
        "Legend module should re-export state contracts through logic boundary."
    );

    let crate_source = load_source("src/lib.rs");
    let cargo_source = load_source("Cargo.toml");
    assert!(
        crate_source.contains("pub use ui_legend as legend;")
            && crate_source.contains("pub use legend::{Legend, LegendTone};"),
        "ui crate root should re-export ui-legend contracts."
    );
    assert!(
        cargo_source.contains("component-legend = [\"dep:ui-legend\"]"),
        "component-legend feature should depend on dep:ui-legend after extraction."
    );
    assert!(
        cargo_source
            .contains("ui-legend = { path = \"../../components/legend\", optional = true }"),
        "ui Cargo.toml should include optional ui-legend dependency."
    );
}

#[test]
fn legend_uses_state_primitive_and_headless_contracts() {
    let primitive_source = load_source("../ui-state-primitives/src/legend.rs");
    let headless_source = load_source("../ui-headless/src/legend.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let view_source = load_source("src/legend/view.rs");

    for needle in [
        "pub enum LegendTone",
        "pub struct LegendStateInput",
        "pub struct LegendState",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Legend state primitive should include `{needle}`."
        );
    }

    for needle in [
        "pub struct LegendOptions",
        "pub struct LegendAttrs",
        "pub struct LegendContract",
        "pub fn use_legend(",
    ] {
        assert!(
            headless_source.contains(needle),
            "Legend headless contract should include `{needle}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::legend::{",
        "pub fn normalize_required_state(",
        "pub fn normalize_accessibility_state(",
        "pub struct LegendNormalizeInput",
        "pub struct LegendResolvedModel",
        "pub fn normalize_component_state(",
        "pub fn resolve_agent_contract(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend logic should include `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, LegendOptions, use_legend};",
        "let normalized = logic::normalize_component_state(logic::LegendNormalizeInput {",
        "let required_state = normalized.required_state;",
        "let accessibility_state = normalized.accessibility_state;",
        "let state = normalized.state;",
        "let semantics = use_legend(LegendOptions {",
        "data-ui-schema=agent_contract.schema_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Legend view should mount state/headless/agent contracts; missing `{needle}`."
        );
    }
}

#[test]
fn legend_has_no_controllable_state_axis_and_avoids_half_controlled_api() {
    let view_source = load_source("src/legend/view.rs");

    for needle in [
        "#[prop(optional)] is_required: Option<bool>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "logic::normalize_component_state(logic::LegendNormalizeInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "Legend should keep one-way state projection; missing `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] required: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "default_required",
        "default_disabled",
        "default_is_required",
        "default_is_disabled",
        "on_required_change",
        "on_disabled_change",
        "normalize_required_state(is_required, false)",
        "normalize_accessibility_state(is_disabled, false)",
        "logic::resolve_state(LegendStateInput {",
        "use_controllable_state",
        "create_rw_signal(",
        "RwSignal<",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Legend should not introduce half-controlled API surface; found `{forbidden}`."
        );
    }
}

#[test]
fn legend_default_values_are_normalized_in_logic_not_view() {
    let logic_source = load_source("src/legend/logic.rs");
    let view_source = load_source("src/legend/view.rs");

    for needle in [
        "const DEFAULT_IS_REQUIRED: bool = false;",
        "const DEFAULT_IS_DISABLED: bool = false;",
        "ui_state_primitives::legend::normalize_required_state(is_required, DEFAULT_IS_REQUIRED)",
        "ui_state_primitives::legend::normalize_accessibility_state(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend logic should own default normalization rule `{needle}`."
        );
    }

    for forbidden in [", false)", "unwrap_or("] {
        assert!(
            !view_source.contains(forbidden),
            "Legend view should not introduce local default fallback; found `{forbidden}`."
        );
    }
}

#[test]
fn legend_state_normalization_is_centralized_in_logic() {
    let logic_source = load_source("src/legend/logic.rs");
    let view_source = load_source("src/legend/view.rs");
    let styles_source = load_source("src/legend/styles.rs");

    for needle in [
        "pub struct LegendNormalizeInput",
        "pub struct LegendResolvedModel",
        "pub fn normalize_component_state(",
        "let state = resolve_state(LegendStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend logic should centralize state derivation and include `{needle}`."
        );
    }

    for forbidden in [
        "has_custom_text",
        "has_custom_indicator",
        "has_custom_class_name",
        "resolve_state(LegendStateInput {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Legend view should not rebuild state machine details; found `{forbidden}`."
        );
    }

    for forbidden in ["if ", "match "] {
        assert!(
            !styles_source.contains(forbidden),
            "Legend styles should consume state markers only; found control-flow token `{forbidden}`."
        );
    }
}

#[test]
fn legend_discrete_state_axes_are_typed_and_not_stringly_modeled() {
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/legend.rs");

    for needle in [
        "#[prop(optional)] tone: LegendTone",
        "pub enum LegendTone",
        "LegendTone::Default",
        "LegendTone::Muted",
        "LegendTone::Strong",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || primitive_source.contains(needle),
            "Legend discrete state should stay enum-typed; missing `{needle}`."
        );
    }

    for forbidden in [
        "tone: Option<String>",
        "tone: String",
        "variant: Option<String>",
        "variant: String",
        "size: Option<String>",
        "size: String",
        "mode: Option<String>",
        "mode: String",
        "status: Option<String>",
        "status: String",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !primitive_source.contains(forbidden),
            "Legend should avoid stringly discrete API axis; found `{forbidden}`."
        );
    }
}

#[test]
fn legend_consumes_state_primitives_without_store_binding_or_primitive_reimplementation() {
    let primitive_source = load_source("../ui-state-primitives/src/legend.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let view_source = load_source("src/legend/view.rs");

    for needle in [
        "pub use ui_state_primitives::legend::{",
        "ui_state_primitives::legend::normalize_required_state(",
        "ui_state_primitives::legend::normalize_accessibility_state(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend logic should consume state primitives via `{needle}`."
        );
    }

    for primitive in [
        "pub enum LegendRequiredSource",
        "pub enum LegendDisabledSource",
        "pub struct RequiredState",
        "pub struct AccessibilityState",
    ] {
        assert!(
            primitive_source.contains(primitive),
            "State primitive crate should define `{primitive}`."
        );
        assert!(
            !logic_source.contains(&format!("{primitive} {{")),
            "Legend logic should not re-implement primitive `{primitive}`."
        );
    }

    for forbidden in [
        "store::",
        "Store<",
        "use_store",
        "create_rw_signal(",
        "RwSignal<",
        "Arc<Mutex<",
        "RwLock<",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Legend component should stay store-agnostic; found `{forbidden}`."
        );
    }
}

#[test]
fn legend_async_interaction_contract_is_not_applicable() {
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "on_retry",
        "use_async_action",
        "async fn",
        ".await",
        "tokio::",
        "spawn(",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend should keep async interaction protocol out of component scope; found `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("aria-disabled=legend_aria_disabled"),
        "Legend should only expose synchronous disabled semantics in view."
    );
}

#[test]
fn legend_rust_hygiene_contract_is_enforced_in_component_scope() {
    let logic_source = load_source("src/legend/logic.rs");
    let view_source = load_source("src/legend/view.rs");
    let motion_source = load_source("src/legend/motion.rs");
    let styles_source = load_source("src/legend/styles.rs");
    let protocol_source = load_source("src/legend/protocol.rs");
    let mod_source = load_source("src/legend/mod.rs");

    for source in [
        &logic_source,
        &view_source,
        &motion_source,
        &styles_source,
        &protocol_source,
        &mod_source,
    ] {
        for forbidden in [".unwrap(", ".expect(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "Legend non-test source should satisfy rust hygiene contract; found `{forbidden}`."
            );
        }
    }

    for needle in [
        "use std::borrow::Cow;",
        "Vec<Cow<'static, str>>",
        "Cow::Borrowed(\"ui-legend\")",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend class assembly should use Cow to reduce string clone churn; missing `{needle}`."
        );
    }

    for forbidden in [
        "\"ui-legend\".to_string()",
        "\"ui-legend--required\".to_string()",
        "\"ui-legend--disabled\".to_string()",
        "\"ui-legend--text-custom\".to_string()",
        "\"ui-legend--indicator-custom\".to_string()",
        "\"ui-legend--custom-class\".to_string()",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Legend should remove string clone hotspot `{forbidden}` in compose_class_name."
        );
    }
}

#[test]
fn legend_dx_paradox_keeps_simple_default_api_and_docs_path() {
    let view_source = load_source("src/legend/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional, into)] state:",
        "#[prop(optional)] headless_state:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Legend should not require internal state object wiring for base API; found `{forbidden}`."
        );
    }

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Legend text=\"Notification settings\".to_string() />",
        "Default path: only pass text; no state wiring required.",
    ] {
        assert!(
            docs_source.contains(needle),
            "Legend docs should expose obvious default-call DX path; missing `{needle}`.",
        );
    }
}

#[test]
fn legend_dx_hot_reload_and_workbench_contract_is_covered_for_low_interaction_component() {
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");
    let styles_source = load_source("src/legend/styles.rs");
    let css_aggregator_source = load_source("src/css.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    assert!(
        styles_source.contains("pub const CSS: &str"),
        "Legend should keep style edits concentrated in styles.rs static CSS payload."
    );
    assert!(
        css_aggregator_source.contains(
            "#[cfg(feature = \"component-legend\")]\n    out.push_str(crate::legend::styles::CSS);",
        ),
        "Legend style injection should stay feature-gated in css aggregator for isolated style iteration."
    );

    for needle in [
        "pub(super) fn legend() -> AnyView",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Required Legend\" code_signal=required_code>",
        "<Playground title=\"Tone + Custom Indicator + Disabled\" code_signal=states_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Legend should keep docs playground as isolated workbench/demo entry `{needle}`."
        );
    }

    for forbidden in [
        "create_rw_signal(",
        "create_signal(",
        "RwSignal<",
        "create_effect(",
        "create_resource(",
        "on:input=",
        "on:change=",
        "on:keydown=",
        "on:pointerdown=",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend should not introduce internal interactive context state that would need explicit hot-state preservation `{forbidden}`."
        );
    }
}

#[test]
fn legend_non_composite_api_avoids_parallel_array_conventions() {
    let view_source = load_source("src/legend/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for forbidden in [
        "#[prop(optional)] labels:",
        "#[prop(optional)] titles:",
        "#[prop(optional)] panels:",
        "#[prop(optional)] items:",
        "ItemSpec",
        "labels=",
        "titles=",
        "panels=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Legend should not expose composite parallel-array API surface; found `{forbidden}`."
        );
    }

    for needle in [
        "pub(super) fn legend() -> AnyView",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Legend text=\"Notification settings\".to_string() />",
    ] {
        assert!(
            docs_source.contains(needle),
            "Legend docs should keep non-composite explicit usage path; missing `{needle}`."
        );
    }
}

#[test]
fn legend_macro_micro_dragging_duality_is_not_applicable() {
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "on_drag",
        "pointermove",
        "mousemove",
        "requestAnimationFrame",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend should not introduce drag macro/micro state machine paths; found `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("data-ui-action=logic::LegendUiAction::Idle.as_attr()"),
        "Legend should stay in idle synchronous semantic contract without drag lifecycle actions."
    );
}

#[test]
fn legend_two_pass_geometry_rendering_is_not_applicable() {
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");

    for forbidden in [
        "Intent",
        "Measure",
        "Rectification",
        "getBoundingClientRect",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "ResizeObserver",
        "IntersectionObserver",
        "placement",
        "compute_position",
        "reposition",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend should not introduce two-pass geometry measurement loops; found `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("data-ui-action=logic::LegendUiAction::Idle.as_attr()"),
        "Legend should stay in stable idle action contract without measure/rectification phases."
    );
}

#[test]
fn legend_registration_protocol_is_not_applicable() {
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "BTreeSet",
        "roving_index",
        "active_index",
        "focus_next",
        "focus_prev",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend should not introduce dynamic collection registration protocol; found `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("data-ui-action=logic::LegendUiAction::Idle.as_attr()"),
        "Legend should stay in a single-node semantic action contract without collection registration lifecycle."
    );
}

#[test]
fn legend_slot_projection_policy_is_not_applicable() {
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "notify_hidden",
        "suspend_polling",
        "resume_polling",
        "unmount_when_hidden",
        "mount_policy",
        "projection",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend should not introduce slot projection lifecycle protocol; found `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("data-slot=\"legend\"")
            && view_source.contains("data-ui-action=logic::LegendUiAction::Idle.as_attr()"),
        "Legend should remain a single semantic node without projected panel lifecycle phases."
    );
}

#[test]
fn legend_env_streams_are_not_applicable() {
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "match_media",
        "BreakpointChanged",
        "ThemeChanged",
        "on_resize",
        "on_theme_change",
        "on_intersection_change",
        "debounce",
        "throttle",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend should not introduce environment stream sampling/action fanout; found `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("data-ui-action=logic::LegendUiAction::Idle.as_attr()"),
        "Legend should stay in idle semantic action flow without env-stream derived actions."
    );
}

#[test]
fn legend_event_light_cone_is_not_applicable() {
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");

    for forbidden in [
        "Context Bus",
        "ContextBus",
        "Selector",
        "SelectionState::All",
        "bulk_select",
        "select_all",
        "deselect_all",
        "prop_drilling",
        "table_selection",
        "grid_selection",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend should not introduce event-light-cone batch-collection contracts; found `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("data-ui-action=logic::LegendUiAction::Idle.as_attr()"),
        "Legend should remain in a single-node idle semantic action and avoid collection event fanout."
    );
}

#[test]
fn legend_causality_bus_is_not_applicable() {
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");

    for forbidden in [
        "TraceId",
        "trace_id",
        "causality_bus",
        "causality",
        "dispatch_command",
        "publish(",
        "broadcast(",
        "subscribe(",
        "event_bus",
        "command_bus",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend should not introduce causality-bus chain contracts; found `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("data-ui-action=logic::LegendUiAction::Idle.as_attr()"),
        "Legend should remain in direct idle semantic action flow without bus-level trace propagation."
    );
}

#[test]
fn legend_focus_stack_gc_is_not_applicable_and_stays_outside_overlay_focus_manager() {
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");
    let headless_focus_trap_source = load_source("../ui-headless/src/focus_trap.rs");

    for forbidden in [
        "NodeRef",
        "focus_manager_push_trap",
        "focus_manager_pop_trap",
        "focus_manager_peek_trap",
        "RestorePolicy",
        "FallbackTo",
        "Selector",
        "document.body",
        "focus_trap",
        "overlay_stack",
        "use_overlay_stack",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend should not implement overlay focus-stack restore paths; found `{forbidden}`."
        );
    }

    for needle in [
        "pub enum RestorePolicy",
        "Selector(String)",
        "FallbackTo(String)",
        "fn focus_manager_push_trap(",
        "fn focus_manager_pop_trap(",
        "if let Some(body) = document.body()",
    ] {
        assert!(
            headless_focus_trap_source.contains(needle),
            "Overlay focus-stack and restore policy should stay in ui-headless focus manager `{needle}`."
        );
    }
}

#[test]
fn legend_escape_hatches_foreign_zone_is_not_applicable() {
    let mod_source = load_source("src/legend/mod.rs");
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");
    let protocol_source = load_source("src/legend/protocol.rs");

    for forbidden in [
        "Foreign Zone",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "foreign_instance",
        "chart_instance",
        "map_instance",
        "web_sys::HtmlCanvasElement",
        "js_sys::Object",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "Legend should not introduce imperative third-party escape-hatch contracts; found `{forbidden}`."
        );
    }

    for forbidden in ["pub use web_sys", "pub use js_sys", "pub use wasm_bindgen"] {
        assert!(
            !mod_source.contains(forbidden),
            "Legend public API should not expose third-party imperative instance handles `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("data-ui-action=logic::LegendUiAction::Idle.as_attr()"),
        "Legend should stay in local idle semantic rendering flow without foreign-zone command lifecycle."
    );
}

#[test]
fn legend_hydration_discontinuity_is_not_applicable_and_avoids_nondeterministic_init() {
    let mod_source = load_source("src/legend/mod.rs");
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");
    let protocol_source = load_source("src/legend/protocol.rs");
    let root_source = load_source("src/root.rs");
    let id_provider_source = load_source("../ui-headless/src/id_provider.rs");

    for forbidden in [
        "now(",
        "Instant::now",
        "SystemTime",
        "UNIX_EPOCH",
        "rand::",
        "random(",
        "Uuid",
        "uuid::",
        "new_v4",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "Legend should not initialize state from nondeterministic runtime sources `{forbidden}`."
        );
    }

    for needle in [
        "#[prop(optional, default = 1)] id_seed: u64",
        "provide_ui_id_provider(id_seed);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep deterministic hydration seed injection path `{needle}`."
        );
    }

    for needle in [
        "pub fn new(seed: u64) -> Self",
        "*value = value.saturating_add(1);",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider",
    ] {
        assert!(
            id_provider_source.contains(needle),
            "IdProvider should remain deterministic and monotonic with `{needle}`."
        );
    }
}

#[test]
fn legend_ssr_and_cross_platform_paths_stay_cfg_guarded_and_non_wasm_safe() {
    let mod_source = load_source("src/legend/mod.rs");
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");
    let component_motion_test_source = load_source("../../components/legend/test/motion.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");

    for forbidden in [
        "web_sys",
        "wasm_bindgen",
        "js_sys",
        "window()",
        "document()",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend component layer should stay browser-binding free on non-wasm paths; found `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "pub mod web;",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion should keep explicit wasm/non-wasm cfg split with predictable no-op behavior `{needle}`."
        );
    }

    assert!(
        component_motion_test_source.contains("cfg!(target_arch = \"wasm32\")")
            && component_motion_test_source.contains("\"--ui-legend-motion-duration: 220ms;\"")
            && component_motion_test_source.contains("\"--ui-legend-motion-duration: 1ms;\"")
            && component_motion_test_source.contains("\"--ui-legend-motion-reduced: false;\"")
            && component_motion_test_source.contains("\"--ui-legend-motion-reduced: true;\""),
        "Legend component tests should lock wasm vs non-wasm semantic runtime branch outputs."
    );
}

#[test]
fn legend_ui_motion_non_wasm_noop_stub_contract_is_preserved() {
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");
    let legend_motion_source = load_source("../../components/legend/src/motion.rs");
    let legend_motion_test_source = load_source("../../components/legend/test/motion.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion non-wasm branch should keep predictable no-op/stub contract `{needle}`."
        );
    }

    for needle in [
        "resolve_effective_motion(motion, ui_motion::web::prefers_reduced_motion())",
        "1.0",
        "format!(\"--ui-legend-motion-duration: {duration_ms}ms;\")",
        "\"--ui-legend-motion-stiffness:\"",
        "\"--ui-legend-motion-damping:\"",
        "\"--ui-legend-motion-mass:\"",
        "\"--ui-legend-motion-precision:\"",
        "\"--ui-legend-motion-reduced:\"",
    ] {
        assert!(
            legend_motion_source.contains(needle),
            "Legend motion mapping should safely degrade on non-wasm without animation handle assumptions `{needle}`."
        );
    }

    assert!(
        legend_motion_test_source.contains("\"--ui-legend-motion-duration: 220ms;\"")
            && legend_motion_test_source.contains("\"--ui-legend-motion-duration: 1ms;\"")
            && legend_motion_test_source.contains("\"--ui-legend-motion-reduced: false;\"")
            && legend_motion_test_source.contains("\"--ui-legend-motion-reduced: true;\""),
        "Legend motion regression should lock wasm/non-wasm output branches to predictable values."
    );
}

#[test]
fn legend_reduced_motion_ssr_wasm_branches_preserve_semantic_contract() {
    let view_source = load_source("src/legend/view.rs");
    let motion_source = load_source("src/legend/motion.rs");
    let component_motion_test_source = load_source("../../components/legend/test/motion.rs");
    let headless_source = load_source("../ui-headless/src/legend.rs");

    for needle in [
        "resolve_effective_motion(motion, ui_motion::web::prefers_reduced_motion())",
        "1.0",
        "format!(\"--ui-legend-motion-duration: {duration_ms}ms;\")",
        "pub struct EffectiveLegendMotion",
        "pub fn resolve_effective_motion(",
        "spring: ui_motion::spring::sanitize_config(motion.spring, default.spring)",
        "\"--ui-legend-motion-stiffness:\"",
        "\"--ui-legend-motion-damping:\"",
        "\"--ui-legend-motion-mass:\"",
        "\"--ui-legend-motion-precision:\"",
        "\"--ui-legend-motion-reduced:\"",
    ] {
        assert!(
            motion_source.contains(needle),
            "Legend motion should explicitly implement reduced-motion fallback contract `{needle}`."
        );
    }

    assert!(
        component_motion_test_source.contains("cfg!(target_arch = \"wasm32\")")
            && component_motion_test_source.contains("\"--ui-legend-motion-duration: 220ms;\"")
            && component_motion_test_source.contains("\"--ui-legend-motion-duration: 1ms;\"")
            && component_motion_test_source.contains("\"--ui-legend-motion-reduced: false;\"")
            && component_motion_test_source.contains("\"--ui-legend-motion-reduced: true;\""),
        "Legend motion tests should lock wasm enhancement branch and non-wasm/SSR reduced branch outputs."
    );

    for marker in [
        "style=motion_style.clone()",
        "data-state=legend_data_state",
        "data-required=legend_data_required",
        "data-disabled=legend_data_disabled",
        "data-ui-state=legend_data_state",
        "aria-disabled=legend_aria_disabled",
    ] {
        assert!(
            view_source.contains(marker),
            "Legend view should keep semantic markers independent from motion runtime branch `{marker}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Legend semantic render path should not split by platform cfg in view layer `{forbidden}`."
        );
    }

    for needle in [
        "data_state: if state.is_required {",
        "\"required\"",
        "\"optional\"",
        "aria_disabled: state.is_disabled.then_some(\"true\")",
    ] {
        assert!(
            headless_source.contains(needle),
            "Legend semantic contract should remain in headless mapping and stay platform-independent `{needle}`."
        );
    }
}

#[test]
fn legend_performance_budget_has_reproducible_static_baseline() {
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");
    let styles_source = load_source("src/legend/styles.rs");
    let suite_source = load_source("tests/legend/semantics.rs");

    // Equivalent evidence for render-count budget: legend has no internal reactive state/effect loop,
    // so there is no component-owned re-render trigger after initial render without prop changes.
    for forbidden in [
        "create_rw_signal(",
        "create_signal(",
        "RwSignal<",
        "Signal<",
        "create_memo(",
        "Memo<",
        "create_effect(",
        "Effect<",
        "create_resource(",
        "Resource<",
        "spawn_local(",
        "tokio::spawn",
        "request_animation_frame",
        "set_timeout",
        "set_interval",
        "on:mousemove",
        "on:scroll",
        "ResizeObserver",
        "IntersectionObserver",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend should keep static render/update budget without internal reactive/event loop source `{forbidden}`."
        );
    }

    assert_eq!(
        view_source.matches("view! {").count(),
        1,
        "Legend should keep a single render entrypoint for stable profiling/trace baseline."
    );
    assert!(
        view_source.contains(
            "let normalized = logic::normalize_component_state(logic::LegendNormalizeInput {"
        ) && view_source.contains("let semantics = use_legend(LegendOptions {"),
        "Legend update path should stay attributable to logic normalization + headless semantic mapping."
    );
    assert!(
        motion_source.contains("duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 800.0)"),
        "Legend motion path should keep bounded runtime cost via clamped contract values."
    );
    assert!(
        styles_source.contains("--ui-legend-motion-duration")
            && !styles_source.contains("animation-timeline")
            && !styles_source.contains("will-change"),
        "Legend style path should avoid hidden long-running animation budgets."
    );

    assert!(
        suite_source.contains("legend_performance_budget_has_reproducible_static_baseline"),
        "Legend semantics suite should contain a stable, repeatable performance baseline regression hook."
    );
}

#[test]
fn legend_view_macro_complexity_is_controlled() {
    let view_source = load_source("src/legend/view.rs");

    assert_eq!(
        view_source.matches("view! {").count(),
        1,
        "Legend should keep a single view! expansion entrypoint instead of fragmented or duplicated macro blocks."
    );
    assert!(
        view_source.lines().count() <= 120,
        "Legend view.rs should remain small and readable to avoid macro expansion bloat."
    );
    assert_eq!(
        view_source.matches("<Show ").count(),
        1,
        "Legend should keep conditional sub-structure minimal and avoid deep nested control-flow trees."
    );

    for forbidden in [
        "<For ",
        "<Suspense",
        "<Transition",
        "<ErrorBoundary",
        "<Portal",
        "match ",
        "if let ",
        "while ",
        "loop ",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Legend view should avoid macro-heavy nested control or collection expansion `{forbidden}`."
        );
    }

    for marker in [
        "<legend",
        "data-slot=\"legend\"",
        "data-slot=\"legend-text\"",
        "data-slot=\"legend-required\"",
    ] {
        assert!(
            view_source.contains(marker),
            "Legend should keep a compact semantic structure and include `{marker}`."
        );
    }
}

#[test]
fn legend_prefers_function_extraction_for_lightweight_view_fragments() {
    let view_source = load_source("src/legend/view.rs");

    for needle in [
        "fn required_indicator_view(is_required: bool, required_indicator: String) -> impl IntoView",
        "{required_indicator_view(state.is_required, required_indicator)}",
        "data-slot=\"legend-required\"",
    ] {
        assert!(
            view_source.contains(needle),
            "Legend should extract lightweight view fragment into plain rust function `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Legend should keep a single component boundary and avoid fragment-to-component over-abstraction."
    );

    for forbidden in [
        "#[component]\nfn required_indicator_view(",
        "#[component]\r\nfn required_indicator_view(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Legend lightweight fragment extraction must not introduce extra component macro `{forbidden}`."
        );
    }
}

#[test]
fn legend_static_fragment_constantization_is_not_applicable_for_minimal_component() {
    let view_source = load_source("src/legend/view.rs");
    let styles_source = load_source("src/legend/styles.rs");

    for forbidden in [
        "<svg",
        "<path",
        "<footer",
        "inner_html",
        "dangerously_set_inner_html",
        "<canvas",
    ] {
        assert!(
            !view_source.contains(forbidden) && !styles_source.contains(forbidden),
            "Legend should not carry complex static asset template requiring constantized view payload `{forbidden}`."
        );
    }

    assert_eq!(
        view_source.matches("data-slot=\"legend-text\"").count(),
        1,
        "Legend text slot should have one stable declaration to keep static fragment change path clear."
    );
    assert_eq!(
        view_source.matches("data-slot=\"legend-required\"").count(),
        1,
        "Legend required slot should have one stable declaration to keep static fragment change path clear."
    );
    assert!(
        view_source.contains("fn required_indicator_view("),
        "Legend should keep lightweight static required-fragment markup centralized in helper function."
    );
}

#[test]
fn legend_inner_html_contract_is_not_applicable_and_security_regression_is_locked() {
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let styles_source = load_source("src/legend/styles.rs");
    let motion_source = load_source("src/legend/motion.rs");

    for forbidden in [
        "inner_html",
        "dangerously_set_inner_html",
        "set_inner_html(",
        ".set_inner_html(",
        "insert_adjacent_html",
        "outer_html",
        "innerHTML",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend should not introduce html-string injection path `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("{text}") && !view_source.contains("text.as_html"),
        "Legend visible text should stay in typed text-node path instead of html-string injection."
    );
}

#[test]
fn legend_wasm_debug_contract_is_not_applicable_and_feature_isolation_is_preserved() {
    let mod_source = load_source("src/legend/mod.rs");
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");
    let legend_cargo_source = load_source("../../components/legend/Cargo.toml");
    let ui_components_cargo_source = load_source("Cargo.toml");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for forbidden in [
        "trace_id",
        "timestamp",
        "record_event",
        "event_log",
        "replay",
        "debug_panel",
        "debug_overlay",
        "devtools",
        "wasm_debug",
        "tracing::",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend should not introduce wasm-debug state tracing/replay path `{forbidden}`."
        );
    }

    assert!(
        legend_cargo_source.contains("[features]\ndefault = []"),
        "Legend source crate should keep zero-default debug surface to avoid production pollution."
    );
    for forbidden in ["wasm-debug", "wasm_debug", "debug = [", "devtools"] {
        assert!(
            !legend_cargo_source.contains(forbidden),
            "Legend source crate should not expose debug feature flag `{forbidden}`."
        );
    }

    assert!(
        !ui_components_cargo_source.contains("legend-wasm-debug"),
        "ui feature graph should not add unused legend wasm debug toggle."
    );
    for needle in ["accordion-wasm-debug", "button-wasm-debug"] {
        assert!(
            ui_components_cargo_source.contains(needle),
            "Workspace should keep feature-isolated wasm-debug pattern for interactive components via `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn legend() -> AnyView",
        "slug=\"legend\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Legend should keep docs visual entry for manual inspection `{needle}`."
        );
    }
}

#[test]
fn legend_engineering_capabilities_contract_is_unified_and_runtime_agnostic() {
    let protocol_source = load_source("src/legend/protocol.rs");
    let protocol_test_source = load_source("../../components/legend/test/protocol.rs");
    let mod_source = load_source("src/legend/mod.rs");
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");
    let legend_cargo_source = load_source("../../components/legend/Cargo.toml");
    let ui_components_cargo_source = load_source("Cargo.toml");

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "pub enum LegendComponentSchemaVersion",
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "pub struct LegendComponentSpec",
    ] {
        assert!(
            protocol_source.contains(needle),
            "Legend protocol should keep serde-based spec/version contract `{needle}`."
        );
    }
    assert!(
        protocol_test_source.contains("fn protocol_types_implement_serde_contract()")
            && protocol_test_source.contains("assert_serde::<LegendComponentSchemaVersion>();")
            && protocol_test_source.contains("assert_serde::<LegendComponentSpec>();"),
        "Legend should keep explicit serde regression coverage for protocol types."
    );
    assert!(
        legend_cargo_source.contains("serde = { version = \"1.0\", features = [\"derive\"] }"),
        "Legend crate should declare serde dependency for structured protocol serialization."
    );

    for forbidden in [
        "tracing::",
        "span!(",
        "event!(",
        "tokio::",
        "async_std::",
        "Runtime",
        "Executor",
        "async fn",
        ".await",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend component layer should not leak tracing/runtime-specific contracts `{forbidden}`."
        );
    }

    let needle = "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]";
    assert!(
        ui_components_cargo_source.contains(needle),
        "Workspace tracing debug path should remain feature-scoped at component boundary `{needle}`."
    );
    assert!(
        !ui_components_cargo_source.contains("legend-wasm-debug"),
        "Legend should not add ad-hoc tracing/debug feature surface without real interactive need."
    );
}

#[test]
fn legend_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let protocol_source = load_source("src/legend/protocol.rs");
    let manifest_source = load_source("src/legend/Component.toml");
    let rbi_source = load_source("src/legend/legend.rbi");
    let check2_source = load_source("../../components/legend/check2.md");

    for needle in [
        "pub enum LegendComponentSchemaVersion",
        "V1,",
        "pub struct LegendComponentSpec",
        "pub schema_version: LegendComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(needle),
            "Legend protocol should remain on v1 schema contract while no breaking upgrade is introduced `{needle}`."
        );
    }

    assert!(
        manifest_source.contains("schema_version = \"1\""),
        "Legend component manifest should keep schema version at 1 without deprecation migration trigger."
    );
    assert!(
        rbi_source.contains("pub fn Legend("),
        "Legend RBI projection should keep stable public signature when no major-breaking change is introduced."
    );

    for forbidden in [
        "V2",
        "migrate_v1_to_v2",
        "schema_registry",
        "deprecation_window",
        "codemod_rule",
        "contract.v2",
    ] {
        assert!(
            !protocol_source.contains(forbidden)
                && !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden),
            "Legend should not claim migration/registry artifacts without major-breaking upgrade `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。"),
        "Legend checklist should mark codemod/registry item as satisfied with explicit N/A rationale."
    );
}

#[test]
fn legend_headless_web_ssr_feature_mutex_contract_is_preserved() {
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let headless_cargo_source = load_source("../ui-headless/Cargo.toml");
    let legend_cargo_source = load_source("../../components/legend/Cargo.toml");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should keep compile-time web/ssr mutual exclusion guard `{needle}`."
        );
    }

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            headless_cargo_source.contains(needle),
            "ui-headless feature wiring should keep explicit platform split `{needle}`."
        );
    }

    assert!(
        legend_cargo_source.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "Legend should depend on ui-headless without re-declaring conflicting web/ssr feature set."
    );
    for forbidden in [
        "ui-headless = { path = \"../../crates/ui-headless\", features = [\"web\", \"ssr\"] }",
        "ui-headless = { path = \"../../crates/ui-headless\", default-features = false, features = [\"web\", \"ssr\"] }",
    ] {
        assert!(
            !legend_cargo_source.contains(forbidden),
            "Legend dependency must not enable mutually-exclusive ui-headless features `{forbidden}`."
        );
    }
}

#[test]
fn legend_a11y_i18n_l10n_contracts_are_wired_through_headless() {
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let headless_legend_source = load_source("../ui-headless/src/legend.rs");

    for needle in [
        "use ui_headless::{A11yDirection, LegendOptions, use_legend};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let semantics = use_legend(LegendOptions { state, lang, dir });",
        "lang=legend_lang",
        "dir=legend_dir",
        "aria-disabled=legend_aria_disabled",
        "aria-hidden=\"true\"",
        "data-text-source=legend_data_text_source",
        "data-indicator-source=legend_data_indicator_source",
    ] {
        assert!(
            view_source.contains(needle),
            "Legend view should expose a11y + i18n/l10n contract wiring; missing `{needle}`."
        );
    }

    for needle in [
        "normalize_text(input.text)",
        "normalize_required_indicator(input.required_indicator)",
        "DEFAULT_TEXT",
        "DEFAULT_REQUIRED_INDICATOR",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend logic should keep text fallback path in primitive/logic boundary; missing `{needle}`."
        );
    }

    for needle in [
        "use crate::a11y::{A11yDirection, locale_attrs};",
        "pub struct LegendOptions {",
        "pub lang: Option<String>,",
        "pub dir: Option<A11yDirection>,",
        "let locale = locale_attrs(options.lang, options.dir);",
    ] {
        assert!(
            headless_legend_source.contains(needle),
            "Legend headless contract should source locale/a11y from shared a11y utilities; missing `{needle}`."
        );
    }

    for forbidden in ["DEFAULT_TEXT", "DEFAULT_REQUIRED_INDICATOR", "unwrap_or("] {
        assert!(
            !view_source.contains(forbidden),
            "Legend view must not hardcode fallback copy or local text defaults; found `{forbidden}`."
        );
    }
}

#[test]
fn legend_state_observability_markers_are_stable_and_enumerable() {
    let view_source = load_source("src/legend/view.rs");
    let headless_source = load_source("../ui-headless/src/legend.rs");
    let primitive_source = load_source("../ui-state-primitives/src/legend.rs");

    for marker in [
        "data-state=legend_data_state",
        "data-required=legend_data_required",
        "data-disabled=legend_data_disabled",
        "data-required-source=required_state.required_source_attr",
        "data-disabled-source=accessibility_state.disabled_source_attr",
        "data-text-source=legend_data_text_source",
        "data-indicator-source=legend_data_indicator_source",
        "data-class-source=legend_data_class_source",
        "data-ui-state=legend_data_state",
        "aria-disabled=legend_aria_disabled",
    ] {
        assert!(
            view_source.contains(marker),
            "Legend view should expose stable observable marker `{marker}`."
        );
    }

    for needle in [
        "pub enum LegendTone",
        "LegendTone::Default => \"default\"",
        "LegendTone::Muted => \"muted\"",
        "LegendTone::Strong => \"strong\"",
        "pub enum LegendRequiredSource",
        "Self::IsRequired => \"is_required\"",
        "Self::Required => \"required\"",
        "Self::Default => \"default\"",
        "pub enum LegendDisabledSource",
        "Self::IsDisabled => \"is_disabled\"",
        "Self::Disabled => \"disabled\"",
        "Self::Default => \"default\"",
        "pub fn source_attr_from_presence(is_custom: bool) -> &'static str",
        "if is_custom { \"custom\" } else { \"default\" }",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Legend primitive should define closed enumerable state/source mapping `{needle}`."
        );
    }

    for needle in [
        "data_state: if state.is_required {",
        "\"required\"",
        "\"optional\"",
        "data_required: state.is_required.then_some(\"true\")",
        "data_disabled: state.is_disabled.then_some(\"true\")",
    ] {
        assert!(
            headless_source.contains(needle),
            "Legend headless should project closed semantic marker set via `{needle}`."
        );
    }
}

#[test]
fn legend_styles_depend_on_explicit_state_markers_and_css_vars_only() {
    let styles_source = load_source("src/legend/styles.rs");
    let view_source = load_source("src/legend/view.rs");
    let motion_source = load_source("src/legend/motion.rs");

    for selector in [
        ".ui-legend[data-tone=\"default\"]",
        ".ui-legend[data-tone=\"muted\"]",
        ".ui-legend[data-tone=\"strong\"]",
        ".ui-legend[data-required=\"true\"]",
        ".ui-legend[data-disabled=\"true\"]",
        ".ui-legend[data-text-source=\"custom\"]",
        ".ui-legend[data-indicator-source=\"custom\"]",
        ".ui-legend[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "Legend styles should key visual state from explicit semantic selector `{selector}`."
        );
    }

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        ":has(",
        ".ui-legend .",
        ".ui-legend >",
        ".ui-legend +",
        ".ui-legend ~",
        "[data-slot=\"legend-required\"]",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Legend styles should not depend on brittle DOM-structure selector `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("style=motion_style.clone()"),
        "Legend view should pass runtime style through explicit motion css variable payload only."
    );
    assert!(
        motion_source.contains("format!(\"--ui-legend-motion-duration: {duration_ms}ms;\")"),
        "Legend motion runtime style should only emit css custom property payload."
    );
}

#[test]
fn legend_semantic_contract_tests_cover_matrix_without_snapshot_lock_in() {
    let suite_source = load_source("tests/legend/semantics.rs");
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let motion_source = load_source("src/legend/motion.rs");
    let component_motion_test_source = load_source("../../components/legend/test/motion.rs");

    for forbidden in ["assert_snapshot", "insta::", "to_match_snapshot"] {
        assert!(
            !suite_source.contains(forbidden),
            "Legend semantic suite should not depend on visual snapshot assertion `{forbidden}`."
        );
    }

    for needle in [
        "fn legend_has_no_controllable_state_axis_and_avoids_half_controlled_api()",
        "fn legend_state_observability_markers_are_stable_and_enumerable()",
        "fn legend_async_interaction_contract_is_not_applicable()",
        "fn legend_a11y_i18n_l10n_contracts_are_wired_through_headless()",
    ] {
        assert!(
            suite_source.contains(needle),
            "Legend semantic matrix should include branch coverage test `{needle}`."
        );
    }

    // Legend is a non-interactive semantic node; keyboard/pointer paths are intentionally N/A.
    for forbidden in [
        "on_keydown",
        "on_keyup",
        "keydown",
        "keyup",
        "on_pointerdown",
        "on_pointerup",
        "pointerdown",
        "pointerup",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Legend should not expose interactive keyboard/pointer path `{forbidden}`."
        );
    }

    assert!(
        component_motion_test_source.contains("cfg!(target_arch = \"wasm32\")")
            && component_motion_test_source.contains("\"--ui-legend-motion-duration: 220ms;\"")
            && component_motion_test_source.contains("\"--ui-legend-motion-duration: 1ms;\""),
        "Legend motion test matrix should cover wasm vs non-wasm semantic runtime branch."
    );
}

#[test]
fn legend_component_file_responsibilities_are_strictly_layered() {
    let mod_source = load_source("src/legend/mod.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let styles_source = load_source("src/legend/styles.rs");
    let view_source = load_source("src/legend/view.rs");
    let motion_source = load_source("src/legend/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Legend module boundary should include `{needle}`."
        );
    }
    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !mod_source.contains(forbidden),
            "Legend module should keep implementation private; found `{forbidden}`."
        );
    }

    for needle in [
        "pub struct LegendNormalizeInput",
        "pub struct LegendResolvedModel",
        "pub fn normalize_component_state(",
        "pub fn resolve_agent_contract()",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend logic should keep state normalization/source derivation contract `{needle}`."
        );
    }
    for forbidden in [
        "view! {",
        "<legend",
        "pub const CSS: &str",
        "web_sys",
        "wasm_bindgen",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Legend logic should not host view/style/platform implementation detail `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str"),
        "Legend styles should be declared as static token-first css payload."
    );
    for forbidden in ["#[component]", "use_legend(", "LegendOptions {", "web_sys"] {
        assert!(
            !styles_source.contains(forbidden),
            "Legend styles should not carry component/headless/platform logic `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "let normalized = logic::normalize_component_state(logic::LegendNormalizeInput {",
        "let semantics = use_legend(LegendOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "Legend view should stay in render + headless mount path and include `{needle}`."
        );
    }
    for forbidden in [
        "default_text_field_motion_tokens",
        "sanitize_number(",
        "resolve_state(LegendStateInput {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Legend view should not absorb motion-engine or primitive implementation `{forbidden}`."
        );
    }

    for needle in [
        "pub struct LegendMotion",
        "pub struct EffectiveLegendMotion",
        "pub fn sanitize_motion(",
        "pub fn resolve_effective_motion(",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "Legend motion should keep semantic->animation contract mapping `{needle}`."
        );
    }
    for forbidden in [
        "SpringAnimator",
        "Keyframe",
        "requestAnimationFrame",
        "view! {",
        "<legend",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "Legend motion should not re-implement engine/render detail `{forbidden}`."
        );
    }
}

#[test]
fn legend_avoids_spec_rs_sprawl_and_keeps_contract_in_protocol_module() {
    let mod_source = load_source("src/legend/mod.rs");
    let protocol_source = load_source("src/legend/protocol.rs");
    let protocol_test_source = load_source("../../components/legend/test/protocol.rs");
    let spec_path = legend_component_src_dir().join("spec.rs");

    assert!(
        !spec_path.exists(),
        "Legend should not add `src/spec.rs` for simple component shape; keep contract in docs/protocol."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "Legend module boundary should not expose spec module drift `{forbidden}`."
        );
    }

    for needle in [
        "pub enum LegendComponentSchemaVersion",
        "pub struct LegendComponentSpec",
        "pub schema_version: LegendComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(needle),
            "Legend versioned contract should stay in protocol.rs and include `{needle}`."
        );
    }

    assert!(
        protocol_test_source.contains("fn protocol_types_implement_serde_contract()")
            && protocol_test_source.contains("assert_serde::<LegendComponentSchemaVersion>();")
            && protocol_test_source.contains("assert_serde::<LegendComponentSpec>();"),
        "Legend protocol contract should keep explicit serde regression coverage."
    );
}

#[test]
fn legend_token_first_static_style_contract_is_enforced() {
    let styles_source = load_source("src/legend/styles.rs");
    let view_source = load_source("src/legend/view.rs");
    let motion_source = load_source("src/legend/motion.rs");
    let css_aggregate_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "--ui-legend-motion-duration: var(",
        "var(--ui-fg",
        "var(--ui-space-2xs",
        "var(--ui-danger",
        ".ui-legend[data-tone=\"default\"]",
        ".ui-legend[data-disabled=\"true\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "Legend styles should be token-first and include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-legend\")]",
        "out.push_str(crate::legend::styles::CSS);",
    ] {
        assert!(
            css_aggregate_source.contains(needle),
            "ui css aggregator should include legend style injection hook `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should inject aggregated component css through `{needle}`."
        );
    }

    assert!(
        view_source.contains("style=motion_style.clone()")
            && motion_source.contains("format!(\"--ui-legend-motion-duration: {duration_ms}ms;\")")
            && motion_source.contains("\"--ui-legend-motion-stiffness:\""),
        "Legend runtime style payload should be limited to css custom properties from motion contract."
    );

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"gap-",
        "class=\"text-",
        "class=\"bg-",
        "tailwind",
        "stylex",
        "styled_components",
        "css! {",
    ] {
        assert!(
            !view_source.contains(forbidden) && !styles_source.contains(forbidden),
            "Legend component should not be polluted by utility-first/CSS-in-Rust default patterns `{forbidden}`."
        );
    }
}

#[test]
fn legend_css_cascade_layer_and_runtime_style_payload_are_constrained() {
    let css_aggregate_source = load_source("src/css.rs");
    let view_source = load_source("src/legend/view.rs");
    let motion_source = load_source("src/legend/motion.rs");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_aggregate_source.contains(needle),
            "Component css aggregation should keep legend styles inside @layer ui boundary `{needle}`."
        );
    }

    assert!(
        view_source.contains("style=motion_style.clone()"),
        "Legend runtime style hookup should stay explicit and sourced from motion css-variable payload."
    );
    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
        "style=\"position:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Legend view should not inline raw layout style literal `{forbidden}`."
        );
    }

    assert!(
        motion_source.contains("format!(\"--ui-legend-motion-duration: {duration_ms}ms;\")")
            && motion_source.contains("\"--ui-legend-motion-stiffness:\"")
            && motion_source.contains("\"--ui-legend-motion-damping:\"")
            && motion_source.contains("\"--ui-legend-motion-reduced:\""),
        "Legend runtime style should emit only css custom property payload."
    );
    for forbidden in [
        "top:",
        "left:",
        "right:",
        "bottom:",
        "width:",
        "height:",
        "position:",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "Legend motion payload should not include raw inline layout style segment `{forbidden}`."
        );
    }
}

#[test]
fn legend_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let motion_source = load_source("src/legend/motion.rs");
    let view_source = load_source("src/legend/view.rs");
    let component_motion_test_source = load_source("../../components/legend/test/motion.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");

    for needle in [
        "pub struct LegendMotion",
        "pub spring: ui_motion::spring::SpringConfig",
        "spring: ui_motion::presets::spring_soft()",
        "spring: ui_motion::spring::sanitize_config(motion.spring, default.spring)",
        "pub fn resolve_effective_motion(",
        "resolve_effective_motion(motion, ui_motion::web::prefers_reduced_motion())",
        "duration_ms: if reduced { 1.0 } else { motion.duration_ms }",
        "\"--ui-legend-motion-stiffness:\"",
        "\"--ui-legend-motion-damping:\"",
        "\"--ui-legend-motion-mass:\"",
        "\"--ui-legend-motion-precision:\"",
        "\"--ui-legend-motion-reduced:\"",
    ] {
        assert!(
            motion_source.contains(needle),
            "Legend motion contract should stay component-scoped and include `{needle}`."
        );
    }

    assert!(
        view_source.contains("let motion_style = motion::attach_motion(motion);"),
        "Legend view should mount motion contract via attach_motion."
    );

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion should keep non-wasm/SSR no-op stub behavior `{needle}`."
        );
    }

    for needle in [
        "fn resolve_effective_motion_respects_reduced_motion_branch()",
        "\"--ui-legend-motion-duration: 220ms;\"",
        "\"--ui-legend-motion-duration: 1ms;\"",
        "\"--ui-legend-motion-reduced: false;\"",
        "\"--ui-legend-motion-reduced: true;\"",
    ] {
        assert!(
            component_motion_test_source.contains(needle),
            "Legend component motion tests should lock reduced-motion + platform branches `{needle}`."
        );
    }
}

#[test]
fn legend_ui_components_entrypoints_follow_layered_contract_boundaries() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for needle in [
        "#[cfg(feature = \"component-legend\")]",
        "pub use ui_legend as legend;",
        "pub mod root;",
        "pub use root::UiRoot;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui lib entrypoint should keep feature-gated public boundary `{needle}`."
        );
    }
    for forbidden in ["web_sys", "wasm_bindgen"] {
        assert!(
            !lib_source.contains(forbidden),
            "ui lib entrypoint should not expose platform detail `{forbidden}`."
        );
    }

    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)")
            && css_source.contains("out.push_str(\"\\n@layer ui {\\n\");")
            && css_source.contains("#[cfg(feature = \"component-legend\")]")
            && css_source.contains("out.push_str(crate::legend::styles::CSS);"),
        "ui css entrypoint should aggregate by feature in @layer ui."
    );

    for needle in [
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot entrypoint should centralize theme/css/i18n injection via `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringAnimator",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight primitive should remain shared visual motion capability `{needle}`."
        );
    }
    for forbidden in ["Legend", "Accordion", "MenuItemKind", "Tooltip"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight primitive should not carry component business semantic `{forbidden}`."
        );
    }

    for path in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !manifest_dir.join(path).exists(),
            "ui should not host `{path}`; semantic primitives belong to ui-headless."
        );
    }
}

#[test]
fn legend_component_directory_standard_files_are_present_and_layer_scoped() {
    let src_dir = legend_component_src_dir();
    let mod_source = load_source("src/legend/mod.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let styles_source = load_source("src/legend/styles.rs");
    let view_source = load_source("src/legend/view.rs");
    let motion_source = load_source("src/legend/motion.rs");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required).exists(),
            "Legend component directory should include standard file `{required}`."
        );
    }
    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !src_dir.join(forbidden).exists(),
            "Legend component directory should not include `{forbidden}` for this component."
        );
    }

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{",
        "pub use motion::LegendMotion;",
        "pub use view::Legend;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Legend mod.rs should keep minimal stable export surface `{needle}`."
        );
    }
    for forbidden in ["pub mod logic", "pub mod view", "pub use protocol::"] {
        assert!(
            !mod_source.contains(forbidden),
            "Legend mod.rs should avoid over-exporting internals `{forbidden}`."
        );
    }

    for needle in [
        "pub struct LegendNormalizeInput",
        "pub struct LegendResolvedModel",
        "pub fn normalize_component_state(",
        "pub fn resolve_agent_contract(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend logic.rs should keep normalization/derivation duties `{needle}`."
        );
    }
    for forbidden in [
        "view! {",
        "<legend",
        "web_sys",
        "wasm_bindgen",
        "pub const CSS: &str",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Legend logic.rs should not host render/style/platform details `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str", "var(--ui-", "var(--ui-fallback-"] {
        assert!(
            styles_source.contains(needle),
            "Legend styles.rs should keep static token CSS contract `{needle}`."
        );
    }
    for forbidden in ["#[component]", "use_legend(", "LegendOptions {", "web_sys"] {
        assert!(
            !styles_source.contains(forbidden),
            "Legend styles.rs should not carry render/headless/platform behavior `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "let normalized = logic::normalize_component_state(logic::LegendNormalizeInput {",
        "let semantics = use_legend(LegendOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "Legend view.rs should keep Leptos structure + headless mount responsibility `{needle}`."
        );
    }
    for forbidden in [
        "resolve_state(LegendStateInput {",
        "default_text_field_motion_tokens",
        "SpringAnimator",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Legend view.rs should not absorb primitive or motion-engine implementation `{forbidden}`."
        );
    }

    for needle in [
        "pub struct LegendMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "resolve_effective_motion(motion, ui_motion::web::prefers_reduced_motion())",
    ] {
        assert!(
            motion_source.contains(needle),
            "Legend motion.rs should stay as semantic->motion contract mapping `{needle}`."
        );
    }
    for forbidden in [
        "view! {",
        "<legend",
        "requestAnimationFrame",
        "ResizeObserver",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "Legend motion.rs should not include rendering or DOM-driver detail `{forbidden}`."
        );
    }
}

#[test]
fn legend_context_compression_manifest_and_rbi_are_present_and_aligned() {
    let src_dir = legend_component_src_dir();
    let manifest_path = src_dir.join("Component.toml");
    let rbi_path = src_dir.join("legend.rbi");
    let manifest_source = load_source("src/legend/Component.toml");
    let rbi_source = load_source("src/legend/legend.rbi");
    let view_source = load_source("src/legend/view.rs");
    let motion_source = load_source("src/legend/motion.rs");

    assert!(
        manifest_path.exists(),
        "Legend context-compression manifest should exist at src/Component.toml."
    );
    assert!(
        rbi_path.exists(),
        "Legend RBI signature projection should exist at src/legend.rbi."
    );

    for needle in [
        "schema_version = \"1\"",
        "name = \"Legend\"",
        "crate = \"ui-legend\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"agent_contract_schema_markers\"",
        "name = \"snapshot_rendering\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "Legend manifest should declare context-compression contract `{needle}`."
        );
    }

    for needle in [
        "pub type LegendTone = ui_state_primitives::legend::LegendTone;",
        "pub struct LegendMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "pub fn Legend(",
        "required_indicator: Option<String>",
        "dir: Option<ui_headless::A11yDirection>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "Legend RBI should project stable machine-readable signature `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] text: Option<String>",
        "#[prop(optional)] tone: LegendTone",
        "#[prop(optional)] motion: LegendMotion",
        "#[prop(optional)] dir: Option<A11yDirection>",
    ] {
        assert!(
            view_source.contains(needle),
            "Legend component signature should stay aligned with RBI projection `{needle}`."
        );
    }

    for needle in [
        "pub struct LegendMotion",
        "pub spring: ui_motion::spring::SpringConfig",
    ] {
        assert!(
            motion_source.contains(needle),
            "Legend motion contract should match RBI-declared fields `{needle}`."
        );
    }
}

#[test]
fn legend_defensive_variable_chain_is_enforced_without_literal_size_fallbacks() {
    let styles_source = load_source("src/legend/styles.rs");
    let theme_css_source = load_source("../ui-theme/src/css.rs");

    for needle in [
        "--ui-legend-strong-letter-spacing: var(",
        "--ui-command-group-heading-letter-spacing,",
        "var(--ui-fallback-command-group-heading-letter-spacing)",
        "--ui-legend-underline-offset: var(",
        "--ui-action-bar-clear-underline-offset,",
        "var(--ui-fallback-action-bar-clear-underline-offset)",
        "--ui-legend-outline-width: var(",
        "var(--ui-fallback-button-focus-outline-width)",
        "--ui-legend-outline-offset: var(",
        "var(--ui-fallback-button-focus-outline-offset)",
        "gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
    ] {
        assert!(
            styles_source.contains(needle),
            "Legend styles should keep defensive variable chain with theme fallbacks `{needle}`."
        );
    }

    for forbidden in ["#", "1px", "2px", "0.01em", "0.12em", "1ms"] {
        assert!(
            !styles_source.contains(forbidden),
            "Legend styles should not keep literal hardcoded hex/size fallback `{forbidden}`."
        );
    }

    for theme_fallback in [
        "--ui-fallback-command-group-heading-letter-spacing:",
        "--ui-fallback-action-bar-clear-underline-offset:",
        "--ui-fallback-button-focus-outline-width:",
        "--ui-fallback-button-focus-outline-offset:",
    ] {
        assert!(
            theme_css_source.contains(theme_fallback),
            "Theme SSOT should emit defensive fallback variable `{theme_fallback}`."
        );
    }
}

#[test]
fn legend_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/legend/view.rs");

    for attr in [
        "data-slot=\"legend\"",
        "data-tone=legend_data_tone",
        "data-state=legend_data_state",
        "data-required=legend_data_required",
        "data-disabled=legend_data_disabled",
        "data-required-source=required_state.required_source_attr",
        "data-disabled-source=accessibility_state.disabled_source_attr",
        "data-text-source=legend_data_text_source",
        "data-indicator-source=legend_data_indicator_source",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=logic::LegendUiAction::Idle.as_attr()",
        "data-ui-state=legend_data_state",
        "data-slot=\"legend-text\"",
        "data-slot=\"legend-required\"",
    ] {
        assert!(
            source.contains(attr),
            "Legend should expose `{attr}` for baseline-style state inspection and styling."
        );
    }
}

#[test]
fn legend_agent_contract_schema_is_typed_and_whitelisted() {
    let logic_source = load_source("src/legend/logic.rs");
    let view_source = load_source("src/legend/view.rs");
    let manifest_source = load_source("src/legend/Component.toml");
    let rbi_source = load_source("src/legend/legend.rbi");

    for needle in [
        "pub enum LegendAgentSchema",
        "pub enum LegendAgentSchemaVersion",
        "pub enum LegendStreamSupport",
        "pub enum LegendStreamFallback",
        "pub enum LegendStreamMode",
        "pub enum LegendOutputStatus",
        "pub enum LegendIntent",
        "pub enum LegendUiAction",
        "pub enum LegendUiSource",
        "pub struct LegendAgentContract",
        "pub schema_attr: &'static str",
        "pub schema_version_attr: &'static str",
        "pub fn resolve_agent_contract() -> LegendAgentContract",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend agent contract fields should be generated from typed closed enums `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=logic::LegendUiAction::Idle.as_attr()",
        "data-ui-source=logic::LegendUiSource::Component.as_attr()",
        "data-ui-state=legend_data_state",
        "data-required-source=required_state.required_source_attr",
        "data-disabled-source=accessibility_state.disabled_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Legend view should expose traceable agent semantic marker `{needle}`."
        );
    }

    for forbidden in [
        "data-ui-source=\"component\"",
        "dangerously_set_inner_html",
        "inner_html",
        "<script",
        "javascript:",
        "eval(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Legend render path should enforce whitelist-safe config boundary; found `{forbidden}`."
        );
    }

    assert!(
        manifest_source.contains("name = \"agent_contract_schema_markers\""),
        "Legend manifest should declare explicit agent-contract marker capability."
    );
    assert!(
        rbi_source.contains("pub fn Legend(") && rbi_source.contains("pub struct LegendMotion {"),
        "Legend RBI should project typed inputs used by the schema-marked render chain."
    );
}

#[test]
fn legend_streaming_term_is_limited_to_llm_output_render_modes() {
    let logic_source = load_source("src/legend/logic.rs");
    let view_source = load_source("src/legend/view.rs");
    let manifest_source = load_source("src/legend/Component.toml");

    for needle in [
        "pub enum LegendStreamSupport",
        "Self::Unsupported => \"unsupported\"",
        "pub enum LegendStreamFallback",
        "Self::Snapshot => \"snapshot\"",
        "pub enum LegendStreamMode",
        "Self::Snapshot => \"snapshot\"",
        "stream_support_attr: LegendStreamSupport::Unsupported.as_attr()",
        "stream_fallback_attr: LegendStreamFallback::Snapshot.as_attr()",
        "stream_mode_attr: LegendStreamMode::Snapshot.as_attr()",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend stream contract should keep LLM-output mode semantics via `{needle}`."
        );
    }

    for forbidden in [
        "LegendStreamMode::Streaming",
        "Self::Streaming =>",
        "stream_mode_attr: \"streaming\"",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Legend stream contract should not invent extra stream display modes `{forbidden}`."
        );
    }

    for needle in [
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Legend view should surface stream-mode markers for agent-readable display state `{needle}`."
        );
    }

    assert!(
        manifest_source.contains("name = \"snapshot_rendering\""),
        "Legend manifest should explicitly declare snapshot rendering capability."
    );
}

#[test]
fn legend_snapshot_is_baseline_render_mode_and_can_consume_full_config() {
    let logic_source = load_source("src/legend/logic.rs");
    let view_source = load_source("src/legend/view.rs");
    let protocol_source = load_source("src/legend/protocol.rs");
    let manifest_source = load_source("src/legend/Component.toml");

    for needle in [
        "pub enum LegendStreamFallback",
        "Self::Snapshot => \"snapshot\"",
        "pub enum LegendStreamMode",
        "Self::Snapshot => \"snapshot\"",
        "pub enum LegendOutputStatus",
        "Self::Verified => \"verified\"",
        "stream_fallback_attr: LegendStreamFallback::Snapshot.as_attr()",
        "stream_mode_attr: LegendStreamMode::Snapshot.as_attr()",
        "output_status_attr: LegendOutputStatus::Verified.as_attr()",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend should keep snapshot as baseline completed-output render mode `{needle}`."
        );
    }

    for needle in [
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=logic::LegendUiAction::Idle.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "Legend view should expose baseline snapshot output contract marker `{needle}`."
        );
    }

    for forbidden in ["streaming", "draft", "partial"] {
        assert!(
            !view_source.contains(forbidden),
            "Legend should not expose non-baseline partial output mode marker `{forbidden}`."
        );
    }

    for needle in [
        "pub struct LegendComponentSpec",
        "pub schema_version: LegendComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(needle),
            "Legend should accept a complete typed config payload via protocol `{needle}`."
        );
    }

    assert!(
        manifest_source.contains("name = \"snapshot_rendering\""),
        "Legend manifest should declare snapshot rendering capability as baseline."
    );
}

#[test]
fn legend_streaming_requirement_is_optional_and_component_scope_stays_render_only() {
    let logic_source = load_source("src/legend/logic.rs");
    let view_source = load_source("src/legend/view.rs");
    let manifest_source = load_source("src/legend/Component.toml");

    for needle in [
        "stream_support_attr: LegendStreamSupport::Unsupported.as_attr()",
        "stream_fallback_attr: LegendStreamFallback::Snapshot.as_attr()",
        "output_status_attr: LegendOutputStatus::Verified.as_attr()",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend should declare stream optional-by-scope contract via `{needle}`."
        );
    }

    for needle in [
        "<legend",
        "aria-disabled=legend_aria_disabled",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-state=legend_data_state",
    ] {
        assert!(
            view_source.contains(needle),
            "Legend should keep role/aria/data markers continuously readable via `{needle}`."
        );
    }

    for forbidden in [
        "on_retry",
        "retry_count",
        "reconnect",
        "aria-busy",
        "is_loading",
        "use_async_action",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Legend should leave validation/retry/recovery policy to upper layers; found `{forbidden}`."
        );
    }

    assert!(
        manifest_source.contains("name = \"snapshot_rendering\""),
        "Legend manifest should keep snapshot capability as the explicit fallback mode."
    );
    assert!(
        !manifest_source.contains("name = \"streaming_rendering\""),
        "Legend should not claim reader-surface streaming capability."
    );
}

#[test]
fn legend_styles_include_state_marker_contracts() {
    let source = load_source("src/legend/styles.rs");

    for selector in [
        ".ui-legend--tone-default",
        ".ui-legend[data-tone=\"strong\"]",
        ".ui-legend--required",
        ".ui-legend[data-disabled=\"true\"]",
        ".ui-legend--text-custom",
        ".ui-legend[data-indicator-source=\"custom\"]",
        ".ui-legend--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Legend styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn legend_visual_desire_baseline_is_repo_level_and_legend_keeps_tokenized_defaults() {
    let styles_source = load_source("src/legend/styles.rs");
    let button_docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let input_docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let overlay_docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
        "font-size: var(--ui-button-size-l-font-size, var(--ui-fallback-font-size-150));",
        "line-height: var(--ui-button-size-l-line-height, var(--ui-fallback-line-height-150));",
        "font-weight: 600;",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        "color: var(--ui-danger, var(--ui-fallback-danger));",
    ] {
        assert!(
            styles_source.contains(needle),
            "Legend default visual hierarchy/contrast should remain tokenized and include `{needle}`."
        );
    }

    for forbidden in [":hover", ":active", ":focus"] {
        assert!(
            !styles_source.contains(forbidden),
            "Legend is non-interactive semantic text and should not add pseudo interaction branch `{forbidden}`."
        );
    }

    for needle in [
        "pub(super) fn button() -> AnyView",
        "title=\"Button\"",
        "slug=\"button\"",
    ] {
        assert!(
            button_docs_source.contains(needle),
            "Visual Desire baseline ownership stays at docs/global layer; button baseline route should include `{needle}`."
        );
    }
    for needle in [
        "pub(super) fn input() -> AnyView",
        "title=\"Input\"",
        "slug=\"input\"",
    ] {
        assert!(
            input_docs_source.contains(needle),
            "Visual Desire baseline ownership stays at docs/global layer; input baseline route should include `{needle}`."
        );
    }
    for needle in [
        "pub(super) fn overlay() -> AnyView",
        "title=\"Overlay\"",
        "slug=\"overlay\"",
    ] {
        assert!(
            overlay_docs_source.contains(needle),
            "Visual Desire baseline ownership stays at docs/global layer; overlay baseline route should include `{needle}`."
        );
    }
}

#[test]
fn legend_tree_shaking_contract_is_feature_gated_end_to_end() {
    let ui_components_cargo = load_source("Cargo.toml");
    let ui_components_lib = load_source("src/lib.rs");
    let ui_components_css = load_source("src/css.rs");
    let legend_cargo = load_source("../../components/legend/Cargo.toml");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");

    for needle in [
        "component-legend = [\"dep:ui-legend\"]",
        "ui-legend = { path = \"../../components/legend\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui Cargo feature graph should keep legend tree-shake anchor `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-legend\")]",
        "pub use ui_legend as legend;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui lib export should be feature-gated for legend with `{needle}`."
        );
    }

    assert!(
        ui_components_css.contains(
            "#[cfg(feature = \"component-legend\")]\n    out.push_str(crate::legend::styles::CSS);",
        ),
        "Legend css aggregation should stay behind component-legend feature gate."
    );
    assert!(
        !ui_components_css.contains("out.push_str(crate::legend::styles::CSS);\n    #[cfg"),
        "Legend css push must not appear as unconditional aggregation."
    );

    assert!(
        legend_cargo.contains("[features]\ndefault = []"),
        "Legend source crate should keep empty default feature set for source-mode natural trimming."
    );

    assert!(
        web_demo_cargo.contains(
            "ui = { path = \"../../crates/ui\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
        ),
        "web-demo should consume ui with default-features disabled and explicit feature slice."
    );
    assert!(
        !web_demo_cargo.contains("all-components"),
        "web-demo dependency should not implicitly pull `all-components`."
    );
}

#[test]
fn legend_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitive_source = load_source("../ui-state-primitives/src/legend.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let view_source = load_source("src/legend/view.rs");
    let component_logic_test_source = load_source("../../components/legend/test/logic.rs");
    let semantics_test_source = load_source("tests/legend/semantics.rs");

    for needle in [
        "pub enum LegendTone",
        "LegendTone::Default => \"default\"",
        "LegendTone::Muted => \"muted\"",
        "LegendTone::Strong => \"strong\"",
        "pub struct LegendStateInput",
        "pub struct LegendState",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Legend primitive type-space should stay closed and machine-readable with `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] tone: Option<String>",
        "tone: String",
        "variant: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Legend discrete state axis should not regress to stringly input `{forbidden}`."
        );
    }

    for needle in [
        "pub struct LegendNormalizeInput",
        "pub struct LegendResolvedModel",
        "pub fn normalize_component_state(",
        "let state = resolve_state(LegendStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend invalid-state handling should be centralized and testable in logic.rs `{needle}`."
        );
    }

    for marker in [
        "data-state=legend_data_state",
        "data-required=legend_data_required",
        "data-disabled=legend_data_disabled",
        "data-required-source=required_state.required_source_attr",
        "data-disabled-source=accessibility_state.disabled_source_attr",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=logic::LegendUiAction::Idle.as_attr()",
    ] {
        assert!(
            view_source.contains(marker),
            "Legend should expose stable semantic markers for machine consumers `{marker}`."
        );
    }

    for feedback_anchor in [
        "fn normalize_component_state_centralizes_state_derivation()",
        "fn legend_discrete_state_axes_are_typed_and_not_stringly_modeled()",
        "fn legend_state_observability_markers_are_stable_and_enumerable()",
        "fn legend_type_system_and_semantic_markers_form_machine_readable_contract()",
    ] {
        assert!(
            component_logic_test_source.contains(feedback_anchor)
                || semantics_test_source.contains(feedback_anchor),
            "State-contract regressions should be directly locatable by test anchor `{feedback_anchor}`."
        );
    }
}

#[test]
fn legend_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "pub(super) fn legend() -> AnyView",
        "title=\"Legend\"",
        "slug=\"legend\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Required Legend\" code_signal=required_code>",
        "<Playground title=\"Tone + Custom Indicator + Disabled\" code_signal=states_code>",
        "<Playground title=\"Controlled vs Default (Comparison)\" code_signal=controlled_code>",
    ] {
        assert!(
            source.contains(needle),
            "legend docs should include `{needle}`.",
        );
    }
}

#[test]
fn legend_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "<Legend text=\"Notification settings\".to_string() />",
        "<Legend text=\"Notification settings\".to_string() is_required=true />",
        "text=\"Billing preferences\".to_string()",
        "tone=LegendTone::Muted",
        "is_required=true",
        "required_indicator=\"(required)\".to_string()",
        "class_name=\"docs-legend-custom\".to_string()",
        "text=\"Read-only group\".to_string()",
        "tone=LegendTone::Strong",
        "is_disabled=true",
        "is_required=Some(controlled_required.get())",
        "<Switch checked=controlled_required set_checked=set_controlled_required>",
    ] {
        assert!(
            source.contains(needle),
            "legend docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn legend_docs_copy_paste_ready_stream_snapshot_and_controlled_comparison_are_explicit() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "Streaming Optional; fallback=snapshot.",
        "Snapshot mode renders verified full output for legend semantics.",
        "Copy-ready snippets prepend imports automatically: use ui::{Legend, LegendTone, Switch};",
        "Uncontrolled path keeps default props; controlled path keeps parent signal as source of truth.",
        "use ui::Legend;",
        "use ui::{Legend, LegendTone};",
        "use ui::{Legend, LegendTone, Switch};",
    ] {
        assert!(
            source.contains(needle),
            "legend docs should keep copy-paste-ready + stream/snapshot + controlled contrast contract `{needle}`."
        );
    }
}

#[test]
fn legend_source_first_docs_are_copy_paste_ready_and_traceable() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_legend_contract.spec.mjs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let check2_source = load_source("../../components/legend/check2.md");

    for needle in [
        "data-slot=\"legend-copy-ready\"",
        "Copy-ready snippets prepend imports automatically",
        "apps/docs-app/src/pages/components/pages/forms_groups_extra.rs",
        "data-slot=\"legend-source-paths\"",
        "components/legend/src/mod.rs",
        "components/legend/src/logic.rs",
        "components/legend/src/view.rs",
        "components/legend/src/styles.rs",
        "components/legend/src/motion.rs",
        "data-slot=\"legend-source-prerequisites\"",
        "component-legend",
        "inject-css",
    ] {
        assert!(
            docs_source.contains(needle),
            "Legend source-first docs should include `{needle}`."
        );
    }

    for needle in [
        "docs-app legend source-first snippets are copy-paste ready and traceable",
        "data-slot=\"code-block\"",
        "data-copyable",
        "data-slot=\"code-block-code\"",
        "use leptos::prelude::*;",
        "use ui::*;",
        "data-slot=\"legend-source-paths\"",
        "data-slot=\"legend-source-prerequisites\"",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Legend e2e source-first contract should include `{needle}`."
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground copy-ready mechanism should include `{needle}`."
        );
    }

    assert!(
        check2_source.contains("Source-first 文档必须 Copy-Paste Ready"),
        "Legend checklist should keep source-first copy-paste governance item."
    );
}

#[test]
fn legend_heroui_strategy_and_component_docs_stay_synced() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let check2_source = load_source("../../components/legend/check2.md");

    for needle in [
        "### Legend 同步记录（2026-02-21）",
        "`Legend` 继续保持语义标题组件定位",
        "component_doc!(\"Legend\", \"legend\", \"Forms\", forms_groups_extra::legend)",
        "forms_groups_extra.rs::legend()",
        "研究文档补充判定：本轮仅为 Legend 参数模型与文档入口同步",
        "HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "Legend HeroUI strategy sync should include `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"Legend\", \"legend\", \"Forms\", forms_groups_extra::legend)",
        "pub(super) fn legend() -> AnyView",
        "slug=\"legend\"",
    ] {
        assert!(
            docs_index_source.contains(needle) || docs_page_source.contains(needle),
            "Legend docs entry/index should include `{needle}`."
        );
    }

    assert!(
        check2_source.contains("HeroUI 对标文档与组件文档同步"),
        "Legend checklist should keep HeroUI strategy sync governance item."
    );
}

#[test]
fn legend_docs_api_names_and_default_contracts_stay_in_sync_with_logic() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let view_source = load_source("src/legend/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/legend.rs");

    for needle in [
        "const DEFAULT_IS_REQUIRED: bool = false;",
        "const DEFAULT_IS_DISABLED: bool = false;",
        "ui_state_primitives::legend::normalize_required_state(is_required, DEFAULT_IS_REQUIRED)",
        "ui_state_primitives::legend::normalize_accessibility_state(is_disabled, DEFAULT_IS_DISABLED)",
        "DEFAULT_TEXT",
        "DEFAULT_REQUIRED_INDICATOR",
        "#[prop(optional)] is_required: Option<bool>",
        "#[prop(optional)] is_disabled: Option<bool>",
    ] {
        assert!(
            logic_source.contains(needle)
                || view_source.contains(needle)
                || primitive_source.contains(needle),
            "Legend source contracts should keep canonical default/API marker `{needle}`."
        );
    }

    for needle in [
        "<Legend text=\"Notification settings\".to_string() />",
        "<Legend text=\"Notification settings\".to_string() is_required=true />",
        "tone=LegendTone::Muted",
        "tone=LegendTone::Strong",
        "is_required=true",
        "is_disabled=true",
        "required_indicator=\"(required)\".to_string()",
        "is_required=Some(controlled_required.get())",
    ] {
        assert!(
            docs_source.contains(needle),
            "Legend docs examples should keep API/matrix contract marker `{needle}`."
        );
    }

    for forbidden in [
        "default_required",
        "default_disabled",
        " required=true",
        " disabled=true",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Legend docs should not drift to non-canonical prop name `{forbidden}`."
        );
    }
}

#[test]
fn legend_semantics_testing_prioritizes_role_aria_data_and_source_contracts() {
    let suite_source = load_source("tests/legend/semantics.rs");
    let view_source = load_source("src/legend/view.rs");
    let headless_source = load_source("../ui-headless/src/legend.rs");

    for needle in [
        "fn legend_state_observability_markers_are_stable_and_enumerable()",
        "fn legend_a11y_i18n_l10n_contracts_are_wired_through_headless()",
        "fn legend_semantic_contract_tests_cover_matrix_without_snapshot_lock_in()",
    ] {
        assert!(
            suite_source.contains(needle),
            "Legend should keep dedicated semantics regression anchors in *_semantics suite `{needle}`."
        );
    }

    for forbidden in ["assert_snapshot", "insta::", "to_match_snapshot"] {
        assert!(
            !suite_source.contains(forbidden),
            "Legend semantic assertions should not collapse to visual snapshot-only checks `{forbidden}`."
        );
    }

    for needle in [
        "<legend",
        "aria-disabled=legend_aria_disabled",
        "data-state=legend_data_state",
        "data-required=legend_data_required",
        "data-disabled=legend_data_disabled",
        "data-required-source=required_state.required_source_attr",
        "data-disabled-source=accessibility_state.disabled_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Legend view should expose role/aria/data/source semantics for contract assertions `{needle}`."
        );
    }

    for needle in [
        "pub struct LegendAttrs",
        "aria_disabled: state.is_disabled.then_some(\"true\")",
        "data_state: if state.is_required {",
    ] {
        assert!(
            headless_source.contains(needle),
            "Legend headless semantics source-of-truth should keep typed attrs mapping `{needle}`."
        );
    }
}

#[test]
fn legend_e2e_contract_uses_semantic_selectors_and_wasm_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_legend_contract.spec.mjs");

    for needle in [
        "body:not(:has(#boot))",
        "[data-component=\"legend\"]",
        "data-slot=\"legend\"",
        "data-ui-output-status=\"verified\"",
        "data-ui-stream-fallback=\"snapshot\"",
        "data-ui-stream-mode=\"snapshot\"",
        "data-required-source=\"is_required\"",
        "data-disabled-source=\"is_disabled\"",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Legend E2E contract should contain semantic selector/ready marker `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "nth-child(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "Legend E2E contract should avoid fragile or timer-based selector strategy `{forbidden}`."
        );
    }
}

#[test]
fn legend_e2e_key_flow_is_repeatable_with_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_legend_contract.spec.mjs");

    for needle in [
        "docs-app legend key flow is repeatable with semantic breakpoints",
        "runLegendControlledRequiredFlow(",
        "await controlledSwitch.focus();",
        "await page.keyboard.press(\"Space\");",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Legend E2E contract should include repeatable key-flow anchor `{needle}`."
        );
    }
}

#[test]
fn legend_docs_app_interactive_playground_supports_live_state_and_repeatable_flow() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_legend_contract.spec.mjs");
    let component_source = load_source("src/legend/README.md");

    for needle in [
        "let (controlled_required, set_controlled_required) = signal(true);",
        "<Playground title=\"Controlled vs Default (Comparison)\" code_signal=controlled_code>",
        "is_required=Some(controlled_required.get())",
        "<Switch checked=controlled_required set_checked=set_controlled_required>",
        "Controlled required (parent signal)",
    ] {
        assert!(
            docs_source.contains(needle),
            "Legend docs-app should expose interactive playground state wiring `{needle}`."
        );
    }

    for needle in [
        "runLegendControlledRequiredFlow(",
        "await controlledSwitch.focus();",
        "await page.keyboard.press(\"Space\");",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Legend e2e contract should keep repeatable interactive flow anchor `{needle}`."
        );
    }

    assert!(
        !component_source.contains("spec.rs"),
        "Legend is not an AI-spec component; interactive requirement should not force spec input/output demo."
    );
}

#[test]
fn legend_readme_is_copy_paste_ready() {
    let source = load_source("src/legend/README.md");

    for needle in [
        "# Legend",
        "## 先用起来（默认路径）",
        "### Hello World（最小可用）",
        "<Legend",
        "text=\"Notification settings\".to_string()",
        "## API 约定",
        "is_required",
        "is_disabled",
    ] {
        assert!(
            source.contains(needle),
            "Legend README should include `{needle}`.",
        );
    }
}

#[test]
fn legend_documentation_is_beginner_friendly_with_readme_or_equivalent_entry() {
    let readme = load_source("src/legend/README.md");
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let check2 = load_source("../../components/legend/check2.md");

    for needle in [
        "# Legend",
        "## 先用起来（默认路径）",
        "### Hello World（最小可用）",
        "## 常见用法",
        "## 再进阶（高级控制）",
        "默认 API 路径优先",
        "不需要用户手动接线 `ui-state-primitives` / `ui-headless`",
        "apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::legend()",
    ] {
        assert!(
            readme.contains(needle),
            "Legend README should include beginner-friendly marker `{needle}`."
        );
    }

    let hello_idx = readme
        .find("### Hello World（最小可用）")
        .expect("Legend README should contain Hello World section");
    let advanced_idx = readme
        .find("## 再进阶（高级控制）")
        .expect("Legend README should contain advanced section");
    assert!(
        hello_idx < advanced_idx,
        "Legend README should keep default path before advanced path."
    );

    for needle in [
        "pub(super) fn legend() -> AnyView",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Required Legend\" code_signal=required_code>",
        "<Playground title=\"Tone + Custom Indicator + Disabled\" code_signal=states_code>",
        "<Playground title=\"Controlled vs Default (Comparison)\" code_signal=controlled_code>",
    ] {
        assert!(
            docs.contains(needle),
            "docs-app equivalent entry should include `{needle}`."
        );
    }

    assert!(
        check2.contains("组件文档必须对新手友好（Documentation as Product）"),
        "Legend checklist should keep documentation-as-product governance item."
    );
}
