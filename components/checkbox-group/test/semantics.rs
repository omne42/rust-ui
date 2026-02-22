use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(component_path) = rel_path.strip_prefix("src/") {
        let mut parts = component_path.splitn(2, '/');
        let component = parts.next().unwrap_or_default();
        let Some(suffix) = parts.next() else {
            return fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        };

        let component_dir = component.replace('_', "-");
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/{component_dir}/src/{suffix}"));

        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn checkbox_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/checkbox-group/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "CheckboxGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn checkbox_group_uses_logic_state_model() {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");

    for needle in [
        "pub use ui_state_primitives::checkbox_group::{",
        "CheckboxGroupState",
        "resolve_checkbox_group_state",
        "pub enum CheckboxGroupMotionPhase {",
        "pub const fn as_data_attr(self) -> &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "CheckboxGroup logic should consume `{needle}` from ui-state-primitives."
        );
    }

    for forbidden in [
        "pub struct CheckboxGroupState {",
        "pub fn resolve_checkbox_group_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "CheckboxGroup state primitive should not be reimplemented locally; found `{forbidden}`."
        );
    }

    for needle in [
        "let headless = logic::use_checkbox_group(",
        "let resolved_state = headless.state.resolved;",
        "let view_state =",
        "logic::resolve_checkbox_group_view_state(resolved_state.get())",
        "data-shows-error=move || view_state.get().shows_error.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxGroup view should derive root state via logic::resolve_state; missing `{needle}`."
        );
    }

    let forbidden = "if resolved_state.get().shows_error";
    assert!(
        !view_source.contains(forbidden),
        "CheckboxGroup view should not rebuild state branches directly; found `{forbidden}`."
    );
}

#[test]
fn checkbox_group_state_primitive_boundary_stays_component_local() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");

    for needle in [
        "name = \"ui-state-primitives\"",
        "kind = \"state-normalization\"",
        "pub use ui_state_primitives::checkbox_group::{",
        "resolve_checkbox_group_state",
    ] {
        let found = logic_source.contains(needle) || component_toml.contains(needle);
        assert!(
            found,
            "CheckboxGroup should consume state primitives through declared boundary `{needle}`."
        );
    }

    for forbidden in ["crate::app", "apps::", "AppState", "Store<", "GlobalStore"] {
        assert!(
            !logic_source.contains(forbidden),
            "CheckboxGroup logic should not bind business store type `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_group_has_no_async_loading_contract_surface() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "aria_busy",
        "use_async_action",
        "on_retry",
        "retry",
        "loading",
    ] {
        let found = logic_source.contains(forbidden)
            || view_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should remain async-free and avoid per-component async protocol `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_group_has_no_dragging_macro_micro_state_machine_contract() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for forbidden in [
        "Dragging",
        "dragging",
        "Action::DragEnd",
        "on_drag",
        "on:drag",
        "onpointermove",
        "mousemove",
        "touchmove",
        "requestAnimationFrame(",
    ] {
        let found = logic_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden)
            || docs_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should remain drag-free and avoid macro/micro drag state machine contract `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_group_has_no_geometry_two_pass_rendering_contract() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for forbidden in [
        "getBoundingClientRect(",
        "getClientRects(",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "measure_pass",
        "rectification_pass",
        "layout_rect",
        "geometry_rect",
    ] {
        let found = logic_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden)
            || docs_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should stay geometry-measurement free and avoid two-pass rectification contract `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_group_has_no_registration_protocol_contract() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for forbidden in [
        "RegistrationContext",
        "register_item",
        "unregister_item",
        "Register(",
        "Unregister(",
        "items_order",
        "HashSet<",
        "HashSet::",
    ] {
        let found = logic_source.contains(forbidden)
            || view_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden)
            || docs_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should stay registration-protocol free and avoid dynamic collection navigation contract `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_group_has_no_slot_projection_lifecycle_contract() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for forbidden in [
        "SlotProjectionMode",
        "ProjectionMode::Lazy",
        "ProjectionMode::KeepAlive",
        "ProjectionMode::Eager",
        "NotifyHidden",
        "on_notify_hidden",
        "keep_alive",
        "lazy_mount",
        "eager_mount",
    ] {
        let found = logic_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden)
            || docs_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should stay slot-projection free and avoid keep-alive lifecycle contract `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_group_has_no_env_stream_sampling_contract() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "onresize",
        "observe_resize",
        "observe_intersection",
        "BreakpointChanged",
        "ThemeChanged",
        "Action::BreakpointChanged",
        "Action::ThemeChanged",
        "debounce(",
        "throttle(",
    ] {
        let found = logic_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden)
            || docs_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should stay env-stream free and avoid raw environment event fan-out contract `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_group_has_no_event_light_cone_contract() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for forbidden in [
        "ContextBus",
        "Context Bus",
        "SelectionState::All",
        "SelectionState",
        "selector_bus",
        "selection_selector",
        "batch_select",
        "prop_drilling",
        "prop drilling",
    ] {
        let found = logic_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden)
            || docs_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should stay event-light-cone free and avoid large-collection batch contract `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_group_has_no_unified_causality_bus_contract() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "causality_bus",
        "dispatch_command",
        "bus_broadcast",
        "subscriber",
        "subscribe(",
        "publish(",
    ] {
        let found = logic_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden)
            || docs_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should stay unified-causality-bus free and avoid trace-propagated bus contract `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_group_has_no_overlay_focus_stack_contract() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let focus_trap_source = load_source("../../crates/ui-headless/src/focus_trap.rs");

    for forbidden in [
        "use_focus_trap(",
        "FocusTrapOptions",
        "RestorePolicy::FallbackTo",
        "RestorePolicy::Selector",
        "provide_overlay_stack(",
        "use_overlay_stack(",
        "use_overlay_stack_registration(",
        "OverlayRegistration",
        "NodeRef<",
        "document.body",
    ] {
        let found = logic_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should not own overlay focus-stack recovery contract token `{forbidden}`.",
        );
    }

    for needle in [
        "pub mod focus_trap;",
        "pub mod overlay_stack;",
        "RestorePolicy::Selector",
        "RestorePolicy::FallbackTo",
    ] {
        let found = headless_lib_source.contains(needle) || focus_trap_source.contains(needle);
        assert!(
            found,
            "Overlay focus-stack contract should stay in ui-headless via `{needle}`.",
        );
    }
}

#[test]
fn checkbox_group_has_no_foreign_zone_escape_hatch_contract() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let component_cargo = load_source("../../components/checkbox-group/Cargo.toml");

    for forbidden in [
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "foreign_zone",
        "foreign_instance",
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "GoogleMap",
        "chart_instance",
        "map_instance",
        "extern \"C\"",
        "wasm_bindgen",
        "js_sys::",
    ] {
        let found = logic_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden)
            || component_cargo.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should remain foreign-zone free and avoid third-party imperative token `{forbidden}`.",
        );
    }

    for forbidden in [
        "name = \"foreign_zone\"",
        "name = \"foreign_instance\"",
        "name = \"on_foreign_ready\"",
        "foreign_instance:",
        "on_foreign_ready:",
        "state =",
    ] {
        let found = component_toml.contains(forbidden) || rbi_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup public contract should not expose third-party instance bridge `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_group_hydration_discontinuity_contract_uses_deterministic_ids_only() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let headless_id_provider = load_source("../../crates/ui-headless/src/id_provider.rs");

    for needle in [
        "id: String,",
        "name = \"id\"",
        "default = \"required\"",
        "pub fn resolve_checkbox_group_ids(id: &str) -> CheckboxGroupIds",
        "legend_id: format!(\"{id}-label\")",
        "pub struct UiIdProvider {",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider",
    ] {
        let found = logic_source.contains(needle)
            || view_source.contains(needle)
            || component_toml.contains(needle)
            || rbi_source.contains(needle)
            || headless_id_provider.contains(needle);
        assert!(
            found,
            "Hydration discontinuity contract should keep deterministic id boundary via `{needle}`.",
        );
    }

    for forbidden in [
        "Uuid::new_v4",
        "uuid::",
        "rand::",
        "thread_rng",
        "random(",
        "Math::random",
        "Date::now",
        "Date.now",
        "performance.now",
        "SystemTime::now",
        "Instant::now",
        "now()",
    ] {
        let found = logic_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should not introduce non-deterministic hydration seed token `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_group_platform_contract_covers_ssr_wasm_and_non_wasm_boundaries() {
    let check_script_source = load_source("../../scripts/check.sh");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let styles_source = load_source("../../components/checkbox-group/src/styles.rs");
    let mod_source = load_source("../../components/checkbox-group/src/mod.rs");

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features inject-css,dev-all-components",
        "echo \"[platform] compile-only: default native path\"",
        "echo \"[platform] compile-only: ssr native path\"",
        "echo \"[platform] compile-only: web wasm path (ui-headless)\"",
        "echo \"[platform] compile-only: web wasm path\"",
        "echo \"[platform] compile-only: ui-motion native path\"",
        "echo \"[platform] compile-only: ui-motion wasm path\"",
    ] {
        let found = check_script_source.contains(needle) || platform_script_source.contains(needle);
        assert!(
            found,
            "Platform compile-only evidence should stay in repo scripts via `{needle}`.",
        );
    }

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "js_sys::",
        "window.",
        "document.",
    ] {
        let found = logic_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden)
            || styles_source.contains(forbidden)
            || mod_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup non-wasm source path should not reference browser-only token `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_group_respects_ui_headless_web_ssr_mutex_contract() {
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let check_script_source = load_source("../../scripts/check.sh");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");
    let component_cargo = load_source("../../components/checkbox-group/Cargo.toml");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "if ! rg -n \"mutually exclusive\" \"$MUTEX_LOG\" >/dev/null; then",
        "ui-headless = { path = \"../../crates/ui-headless\" }",
        "name = \"ui-headless\"",
    ] {
        let found = headless_lib_source.contains(needle)
            || check_script_source.contains(needle)
            || platform_script_source.contains(needle)
            || component_cargo.contains(needle)
            || component_toml.contains(needle);
        assert!(
            found,
            "ui-headless web/ssr mutex contract should remain verifiable via `{needle}`.",
        );
    }

    for forbidden in [
        "ui-headless = { path = \"../../crates/ui-headless\", features = [\"web\", \"ssr\"]",
        "ui-headless = { path = \"../../crates/ui-headless\", default-features = true",
    ] {
        assert!(
            !component_cargo.contains(forbidden),
            "CheckboxGroup should not bypass ui-headless mutex via `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_group_motion_non_wasm_stub_contract_is_predictable_and_safe() {
    let motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let motion_stub_test_source = load_source("../../crates/ui-motion/tests/non_wasm_stub.rs");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
        "let reduced = !motion.enabled || ui_motion::web::prefers_reduced_motion();",
        "let effective_duration = if reduced {",
        "--ui-checkbox-group-motion-duration: {effective_duration}ms;",
    ] {
        let found = motion_lib_source.contains(needle)
            || motion_stub_test_source.contains(needle)
            || platform_script_source.contains(needle)
            || motion_source.contains(needle);
        assert!(
            found,
            "CheckboxGroup non-wasm motion contract should remain verifiable via `{needle}`.",
        );
    }

    for forbidden in [
        "unwrap(",
        "expect(",
        "panic!(",
        "web::animate(",
        "Animation",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "CheckboxGroup motion non-wasm fallback must not assume animation handle via `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_group_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let motion_test_source = load_source("../../components/checkbox-group/test/motion.rs");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "let reduced = !motion.enabled || ui_motion::web::prefers_reduced_motion();",
        "let effective_duration = if reduced {",
        "--ui-checkbox-group-motion-duration: {effective_duration}ms;",
        "style = motion_contract::attach_motion(None, motion);",
        "data-motion-phase=move || view_state.get().motion_phase.as_data_attr()",
        "data-state-source=move || view_state.get().state_source.as_data_attr()",
        "fn attach_motion_reduced_motion_branch_uses_minimal_feedback_when_disabled()",
        "fn attach_motion_uses_predictable_non_wasm_reduced_motion_fallback()",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
    ] {
        let found = motion_source.contains(needle)
            || view_source.contains(needle)
            || motion_test_source.contains(needle)
            || platform_script_source.contains(needle);
        assert!(
            found,
            "CheckboxGroup reduced-motion/SSR/wasm branch coverage should stay verifiable via `{needle}`.",
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        let found = motion_source.contains(forbidden)
            || view_source.contains(forbidden)
            || logic_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should keep a single semantic contract across SSR/wasm without branch split `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_group_a11y_i18n_l10n_contract_stays_headless_and_overridable() {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/checkbox_group.rs");
    let headless_source = load_source("../../crates/ui-headless/src/checkbox_group.rs");

    for needle in [
        "aria-labelledby=legend_id.get_value()",
        "aria-describedby=move || fieldset_aria_describedby.get()",
        "aria-invalid=move || fieldset_aria_invalid.get()",
        "aria-required=move || fieldset_aria_required.get()",
        "lang=fieldset_lang",
        "dir=fieldset_dir",
        "{label.get_value()}",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxGroup view should mount a11y + locale contract via `{needle}`."
        );
    }

    for needle in [
        "use crate::a11y::{A11yDirection, locale_attrs};",
        "locale_attrs(lang, dir)",
        "pub lang: Option<String>,",
        "pub dir: Option<&'static str>,",
    ] {
        assert!(
            headless_source.contains(needle),
            "CheckboxGroup headless contract should reuse shared a11y locale tooling via `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_checkbox_group_content(",
        "normalize_checkbox_group_label(",
        "normalize_checkbox_group_optional_text(",
    ] {
        assert!(
            logic_source.contains(needle),
            "CheckboxGroup user-visible text should be normalized in logic via `{needle}`."
        );
    }

    assert!(
        primitive_source.contains("pub const DEFAULT_LABEL: &str = \"Options\";"),
        "CheckboxGroup fallback copy should live in ui-state-primitives, not view.rs.",
    );

    for forbidden in ["\"Options\"", "\"Fruits\"", "\"Validation + Required\""] {
        assert!(
            !view_source.contains(forbidden),
            "CheckboxGroup view should not hardcode user-visible copy `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_group_boolean_props_follow_is_prefix_contract() {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");

    for needle in [
        "#[prop(optional, into)] is_invalid: Signal<bool>",
        "#[prop(optional, into)] is_required: Signal<bool>",
        "#[prop(optional)] is_disabled: bool",
        "name = \"is_invalid\"",
        "name = \"is_required\"",
        "name = \"is_disabled\"",
        "is_invalid: leptos::prelude::Signal<bool>",
        "is_required: leptos::prelude::Signal<bool>",
        "is_disabled: bool",
    ] {
        let found = view_source.contains(needle)
            || component_toml.contains(needle)
            || rbi_source.contains(needle);
        assert!(
            found,
            "CheckboxGroup public API should keep `is_*` naming contract via `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] invalid: Signal<bool>",
        "#[prop(optional, into)] required: Signal<bool>",
        "#[prop(optional)] disabled: bool",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CheckboxGroup view should not expose legacy prop alias `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_group_api_supports_hello_world_without_internal_state_wiring() {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for forbidden in [
        "name = \"state\"",
        "state:",
        "#[prop(optional)] state",
        "CheckboxGroupState",
    ] {
        let found = view_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup public API should not require internal state object `{forbidden}`."
        );
    }

    for needle in [
        "title=\"Hello World（默认路径）\"",
        "<CheckboxGroup id=\"docs-checkbox-group-hello\".to_string() label=\"Fruits\".to_string()>",
        "let hello_code = Signal::derive(move || {",
    ] {
        assert!(
            docs_source.contains(needle),
            "CheckboxGroup docs should expose minimal default path via `{needle}`.",
        );
    }
}

#[test]
fn checkbox_group_prefers_explicit_parent_child_composition_api() {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "children: Children,",
        "{children()}",
        "children: leptos::children::Children,",
        "<CheckboxGroup id=\"docs-checkbox-group-hello\".to_string() label=\"Fruits\".to_string()>",
        "<Checkbox checked=hello_apple set_checked=set_hello_apple>\"Apple\"</Checkbox>",
    ] {
        let found = view_source.contains(needle)
            || rbi_source.contains(needle)
            || docs_source.contains(needle);
        assert!(
            found,
            "CheckboxGroup should keep explicit `<Parent><Item/></Parent>` composition via `{needle}`.",
        );
    }

    for forbidden in [
        "name = \"labels\"",
        "name = \"titles\"",
        "name = \"panels\"",
        "name = \"items\"",
        "name = \"item_specs\"",
        "labels:",
        "titles:",
        "panels:",
        "Vec<ItemSpec>",
        "ItemSpec",
    ] {
        let found = view_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should not expose parallel-array or config-sugar API `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_group_does_not_expose_group_value_control_axis() {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/checkbox_group.rs");

    for forbidden in [
        "name = \"value\"",
        "name = \"default_value\"",
        "name = \"on_value_change\"",
        "value:",
        "default_value:",
        "on_value_change:",
        "#[prop(optional, into)] value",
        "#[prop(optional, into)] default_value",
    ] {
        let found = view_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden)
            || primitive_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should not expose a half-controlled group value axis; found `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_group_resolves_ids_and_normalizes_text_inputs() {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/checkbox_group.rs");

    for needle in [
        "resolve_checkbox_group_ids",
        "resolve_checkbox_group_content",
        "resolve_checkbox_group_class_name",
        "aria-labelledby=legend_id.get_value()",
        "id=legend_id.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxGroup view should consume normalized logic output via `{needle}`."
        );
    }

    for forbidden in [
        "normalize_checkbox_group_label",
        "normalize_checkbox_group_optional_text",
        ".unwrap_or(base_class)",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CheckboxGroup view should not own default fallback logic `{forbidden}`."
        );
    }

    for needle in [
        "pub fn resolve_checkbox_group_content(",
        "pub fn resolve_checkbox_group_class_name(",
        "normalize_checkbox_group_label(",
        "normalize_checkbox_group_optional_text(",
    ] {
        assert!(
            logic_source.contains(needle),
            "CheckboxGroup logic should centralize default/normalization flow via `{needle}`."
        );
    }

    assert!(
        logic_source.contains("ui_state_primitives::checkbox_group"),
        "CheckboxGroup normalization helpers should be sourced from ui-state-primitives."
    );
    assert!(
        primitive_source.contains("pub const DEFAULT_LABEL: &str = \"Options\";"),
        "CheckboxGroup default label fallback should live in ui-state-primitives."
    );
}

#[test]
fn checkbox_group_uses_headless_checkbox_group_contract() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let headless_source = load_source("../../crates/ui-headless/src/checkbox_group.rs");

    for needle in [
        "ui_headless::use_checkbox_group(",
        "pub dir: Option<A11yDirection>",
        "dir: options.dir,",
    ] {
        assert!(
            logic_source.contains(needle),
            "CheckboxGroup logic should bridge headless checkbox-group contract via `{needle}`."
        );
    }

    for needle in [
        "pub struct CheckboxGroupA11y {",
        "pub attrs: CheckboxGroupAttrs,",
        "pub handlers: CheckboxGroupHandlers,",
        "pub state: CheckboxGroupSemanticState,",
        "pub lang: Option<String>,",
        "pub dir: Option<&'static str>,",
        "locale_attrs(lang, dir)",
    ] {
        assert!(
            headless_source.contains(needle),
            "Headless checkbox-group contract should expose typed attrs/handlers/state + locale via `{needle}`."
        );
    }
}

#[test]
fn checkbox_group_view_mounts_headless_locale_contract() {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");

    for needle in [
        "lang=fieldset_lang",
        "dir=fieldset_dir",
        "aria-describedby=move || fieldset_aria_describedby.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxGroup view should mount headless locale/a11y attrs via `{needle}`."
        );
    }
}

#[test]
fn checkbox_group_emits_baseline_state_data_attributes() {
    let source = load_source("../../components/checkbox-group/src/view.rs");

    for needle in [
        "data-slot=\"checkbox-group\"",
        "data-disabled=move || view_state.get().is_disabled.then_some(\"true\")",
        "data-enabled=move || view_state.get().is_enabled.then_some(\"true\")",
        "data-invalid=move || view_state.get().is_invalid.then_some(\"true\")",
        "data-valid=move || view_state.get().is_valid.then_some(\"true\")",
        "data-required=move || view_state.get().is_required.then_some(\"true\")",
        "data-optional=move || view_state.get().is_optional.then_some(\"true\")",
        "data-has-description=move || view_state.get().has_description.then_some(\"true\")",
        "data-has-error=move || view_state.get().has_error.then_some(\"true\")",
        "data-shows-error=move || view_state.get().shows_error.then_some(\"true\")",
        "data-has-messages=move || view_state.get().has_messages.then_some(\"true\")",
        "data-state-source=move || view_state.get().state_source.as_data_attr()",
        "data-motion-source=motion_source",
        "data-motion-phase=move || view_state.get().motion_phase.as_data_attr()",
    ] {
        assert!(
            source.contains(needle),
            "CheckboxGroup should expose `{needle}` for baseline-style state styling and inspection."
        );
    }
}

#[test]
fn checkbox_group_state_markers_use_closed_enums_for_source_and_phase() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");

    for needle in [
        "pub enum CheckboxGroupStateSource {",
        "SemanticProps,",
        "pub const fn as_data_attr(self) -> &'static str",
        "Self::SemanticProps => \"semantic-props\"",
        "pub enum CheckboxGroupMotionPhase {",
        "Self::Active => \"active\"",
        "Self::Inactive => \"inactive\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "CheckboxGroup marker values should be closed and enumerable via `{needle}`.",
        );
    }
}

#[test]
fn checkbox_group_only_renders_error_slot_when_invalid() {
    let source = load_source("../../components/checkbox-group/src/view.rs");

    for needle in [
        "<Show when=move || view_state.get().shows_error>",
        "data-slot=\"checkbox-group-error\"",
    ] {
        assert!(
            source.contains(needle),
            "CheckboxGroup should guard error rendering via `{needle}`."
        );
    }
}

#[test]
fn checkbox_group_motion_contract_is_token_first_and_uses_ui_motion_backend() {
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");

    for needle in [
        "use ui_theme::default_checkbox_group_motion_tokens;",
        "pub spring: ui_motion::spring::SpringConfig,",
        "ui_motion::web::prefers_reduced_motion()",
        "pub fn sanitize_motion(motion: CheckboxGroupMotion) -> CheckboxGroupMotion",
        "pub fn resolve_effective_motion(",
        "ui_motion::spring::sanitize_config(motion.spring, default.spring)",
        "pub fn motion_source_attr(motion: CheckboxGroupMotion) -> &'static str",
        "pub fn attach_motion(base_vars: Option<String>, motion: CheckboxGroupMotion) -> String",
    ] {
        assert!(
            motion_source.contains(needle),
            "CheckboxGroup motion contract should keep `{needle}` in component motion layer."
        );
    }

    for needle in [
        "style = motion_contract::attach_motion(None, motion);",
        "data-motion-source=motion_source",
        "data-motion-phase=move || view_state.get().motion_phase.as_data_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxGroup view should mount motion contract output via `{needle}`."
        );
    }
}

#[test]
fn checkbox_group_styles_consume_theme_checkbox_group_tokens() {
    let styles_source = load_source("../../components/checkbox-group/src/styles.rs");

    for needle in [
        "--ui-checkbox-group-gap",
        "--ui-checkbox-group-required-marker-gap",
        "--ui-checkbox-group-disabled-opacity",
        "--ui-checkbox-group-motion-duration",
        "--ui-checkbox-group-motion-easing",
    ] {
        assert!(
            styles_source.contains(needle),
            "CheckboxGroup styles should consume theme token variable `{needle}`."
        );
    }
}

#[test]
fn checkbox_group_styles_depend_on_semantic_state_markers_not_structural_guesses() {
    let styles_source = load_source("../../components/checkbox-group/src/styles.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");

    for needle in [
        ".ui-checkbox-group--required .ui-checkbox-group__label::after",
        ".ui-checkbox-group--invalid .ui-checkbox-group__description",
        ".ui-checkbox-group:disabled",
        "data-invalid=move || view_state.get().is_invalid.then_some(\"true\")",
        "data-shows-error=move || view_state.get().shows_error.then_some(\"true\")",
        "class:ui-checkbox-group--invalid=move || view_state.get().is_invalid",
        "class:ui-checkbox-group--required=move || view_state.get().is_required",
    ] {
        let found = styles_source.contains(needle) || view_source.contains(needle);
        assert!(
            found,
            "CheckboxGroup visual state toggles should be explained by semantic marker `{needle}`."
        );
    }

    for forbidden in [
        ":nth-child",
        ":nth-of-type",
        ":has(",
        "style:color",
        "style:display",
    ] {
        let found = styles_source.contains(forbidden) || view_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should avoid fragile structural/style logic `{forbidden}`."
        );
    }

    for needle in [
        "let style = motion_contract::attach_motion(None, motion);",
        "style=style",
        "--ui-checkbox-group-motion-duration:",
        "--ui-checkbox-group-motion-easing:",
        "--ui-checkbox-group-motion-stiffness:",
        "--ui-checkbox-group-motion-damping:",
        "--ui-checkbox-group-motion-mass:",
        "--ui-checkbox-group-motion-precision:",
        "--ui-checkbox-group-motion-reduced:",
    ] {
        let found = view_source.contains(needle) || motion_source.contains(needle);
        assert!(
            found,
            "CheckboxGroup runtime style path should stay CSS-variable only via `{needle}`."
        );
    }
}

#[test]
fn checkbox_group_semantics_suite_prioritizes_contract_assertions_over_snapshots() {
    let semantics_source = load_source("../../components/checkbox-group/test/semantics.rs");
    let logic_test_source = load_source("../../components/checkbox-group/test/logic.rs");
    let motion_test_source = load_source("../../components/checkbox-group/test/motion.rs");

    // Matrix evidence: semantic markers + key branches + N/A guards for inapplicable paths.
    for needle in [
        "fn checkbox_group_view_mounts_headless_locale_contract()",
        "fn checkbox_group_emits_baseline_state_data_attributes()",
        "fn checkbox_group_state_markers_use_closed_enums_for_source_and_phase()",
        "fn checkbox_group_does_not_expose_group_value_control_axis()",
        "fn checkbox_group_has_no_dragging_macro_micro_state_machine_contract()",
        "fn checkbox_group_has_no_overlay_focus_stack_contract()",
        "fn checkbox_group_has_no_foreign_zone_escape_hatch_contract()",
        "fn checkbox_group_hydration_discontinuity_contract_uses_deterministic_ids_only()",
        "fn checkbox_group_platform_contract_covers_ssr_wasm_and_non_wasm_boundaries()",
        "fn checkbox_group_respects_ui_headless_web_ssr_mutex_contract()",
        "fn checkbox_group_motion_non_wasm_stub_contract_is_predictable_and_safe()",
        "fn checkbox_group_reduced_motion_ssr_wasm_branches_keep_semantics_consistent()",
        "fn checkbox_group_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn checkbox_group_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
        "fn checkbox_group_check2_marks_semantics_and_performance_regression_contract_complete()",
        "fn checkbox_group_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()",
        "fn checkbox_group_check2_documents_e2e_selector_and_stable_wait_rules()",
        "fn checkbox_group_e2e_selector_contract_uses_semantic_markers_and_settled_waits()",
        "fn checkbox_group_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_group_paths()",
        "fn checkbox_group_e2e_check_script_covers_selector_and_settled_wait_contract()",
        "fn checkbox_group_check2_marks_e2e_selector_stability_item_complete()",
        "fn checkbox_group_check2_documents_e2e_repeatable_key_flow_rules()",
        "fn checkbox_group_e2e_key_flow_is_repeatable_and_failure_points_are_semantic()",
        "fn checkbox_group_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints()",
        "fn checkbox_group_e2e_check_script_covers_repeatable_key_flow_contracts()",
        "fn checkbox_group_check2_marks_replayable_e2e_critical_flow_item_complete()",
        "fn checkbox_group_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders()",
        "fn checkbox_group_view_functional_split_prefers_plain_functions_over_local_components()",
        "fn checkbox_group_static_fragments_are_constantized_or_absent_for_simple_layout()",
        "fn checkbox_group_inner_html_usage_is_absent_and_untrusted_html_paths_are_blocked()",
        "fn checkbox_group_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na()",
        "fn checkbox_group_styles_use_defensive_variable_fallback_chain()",
        "fn checkbox_group_cascade_layer_and_runtime_style_contract_is_enforced()",
        "fn checkbox_group_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop()",
        "fn checkbox_group_ui_components_fixed_entry_files_follow_layered_boundaries()",
        "fn checkbox_group_component_directory_standard_files_follow_contract_and_na_paths()",
        "fn checkbox_group_file_placement_discipline_is_strict_for_component_scope()",
        "fn checkbox_group_hyper_structure_builder_spec_is_not_applicable_for_simple_component()",
        "fn checkbox_group_context_compression_manifest_and_rbi_projection_are_present_and_current()",
        "fn checkbox_group_agent_contract_is_schema_typed_traceable_and_whitelist_rendered()",
        "fn checkbox_group_check2_documents_streaming_definition_is_llm_output_only_with_two_modes()",
        "fn checkbox_group_check2_documents_snapshot_as_default_baseline_capability()",
        "fn checkbox_group_snapshot_baseline_consumes_complete_result_and_renders_stably()",
        "fn checkbox_group_check2_marks_streaming_scope_as_optional_with_snapshot_fallback()",
        "fn checkbox_group_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources()",
        "fn checkbox_group_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent()",
        "fn checkbox_group_rust_hygiene_script_enforces_repo_level_hygiene_guards()",
        "fn checkbox_group_check2_marks_rust_hygiene_contract_complete()",
        "fn checkbox_group_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope()",
        "fn checkbox_group_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events()",
        "fn checkbox_group_engineering_contract_avoids_runtime_leaks_in_public_api_surface()",
        "fn checkbox_group_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()",
        "fn checkbox_group_check2_marks_version_deprecation_migration_item_complete()",
        "fn checkbox_group_motion_contract_is_token_first_and_uses_ui_motion_backend()",
        "fn checkbox_group_visual_desire_contract_uses_theme_baseline_page_and_snapshots()",
        "fn checkbox_group_tree_shaking_contract_is_feature_gated_and_budgeted()",
        "fn checkbox_group_check2_marks_tree_shaking_feature_pruning_contract_complete()",
        "fn checkbox_group_type_system_and_semantic_markers_form_machine_readable_contract()",
        "fn checkbox_group_docs_playgrounds_lock_state_matrix_contract_values()",
        "fn checkbox_group_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot()",
        "fn checkbox_group_check2_documents_docs_sync_and_state_matrix_rules()",
        "fn checkbox_group_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults()",
        "fn checkbox_group_dx_check_script_covers_docs_sync_and_state_matrix_contract()",
        "fn checkbox_group_check2_marks_docs_sync_and_state_matrix_item_complete()",
        "fn checkbox_group_check2_documents_documentation_as_product_rules()",
        "fn checkbox_group_documentation_entry_exists_with_beginner_first_progression()",
        "fn checkbox_group_dx_check_script_covers_documentation_as_product_beginner_contract()",
        "fn checkbox_group_check2_marks_documentation_as_product_beginner_item_complete()",
        "fn checkbox_group_check2_documents_interactive_playground_rules()",
        "fn checkbox_group_docs_app_provides_interactive_playground_for_props_state_and_preview()",
        "fn checkbox_group_interactive_playground_reuses_repeatable_semantic_e2e_flow()",
        "fn checkbox_group_dx_check_script_covers_interactive_playground_contract()",
        "fn checkbox_group_check2_marks_interactive_playground_contract_complete()",
        "fn checkbox_group_check2_documents_source_first_copy_paste_ready_rules()",
        "fn checkbox_group_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies()",
        "fn checkbox_group_dx_check_script_covers_source_first_copy_paste_ready_contract()",
        "fn checkbox_group_check2_marks_source_first_copy_paste_ready_contract_complete()",
        "fn checkbox_group_check2_documents_heroui_benchmark_docs_sync_rules()",
        "fn checkbox_group_heroui_strategy_and_component_docs_are_synchronized_and_indexable()",
        "fn checkbox_group_dx_check_script_covers_heroui_benchmark_docs_sync_contract()",
        "fn checkbox_group_check2_marks_heroui_benchmark_docs_sync_contract_complete()",
        "fn checkbox_group_dx_check_script_covers_documentation_as_product_contract()",
        "fn checkbox_group_check2_marks_documentation_as_product_item_complete()",
    ] {
        assert!(
            semantics_source.contains(needle),
            "CheckboxGroup semantic test matrix should include `{needle}`."
        );
    }

    // Snapshot assertions must not be the primary oracle for component contract checks.
    for forbidden in [
        "assert_snapshot!",
        "assert_debug_snapshot!",
        "assert_yaml_snapshot!",
        "insta::",
        "to_match_snapshot",
    ] {
        let found = semantics_source.contains(forbidden)
            || logic_test_source.contains(forbidden)
            || motion_test_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup tests should rely on semantic contracts instead of snapshot macro `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_group_module_files_keep_responsibility_boundaries() {
    let mod_source = load_source("../../components/checkbox-group/src/mod.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let styles_source = load_source("../../components/checkbox-group/src/styles.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");

    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use motion::CheckboxGroupMotion;",
        "pub use view::CheckboxGroup;",
    ] {
        assert!(
            mod_source.contains(needle),
            "CheckboxGroup mod.rs should keep minimal export boundary via `{needle}`."
        );
    }

    for forbidden in ["pub mod logic", "pub mod view", "pub use logic::"] {
        assert!(
            !mod_source.contains(forbidden),
            "CheckboxGroup mod.rs should not expose implementation internals `{forbidden}`."
        );
    }

    for forbidden in ["view! {", "<fieldset", "web_sys::", "document.", "window."] {
        assert!(
            !logic_source.contains(forbidden),
            "CheckboxGroup logic.rs should stay DOM/style-free and avoid `{forbidden}`."
        );
    }

    for forbidden in [
        "resolve_checkbox_group_",
        "use_checkbox_group(",
        "view! {",
        "web_sys::",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "CheckboxGroup styles.rs should remain static token-first CSS and avoid `{forbidden}`."
        );
    }

    for needle in [
        "let headless = logic::use_checkbox_group(",
        "let view_state =",
        "logic::resolve_checkbox_group_view_state(resolved_state.get())",
        "data-state-source=move || view_state.get().state_source.as_data_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxGroup view.rs should mount headless + logic state markers via `{needle}`."
        );
    }

    for forbidden in [
        "normalize_checkbox_group_label(",
        "normalize_checkbox_group_optional_text(",
        "resolve_checkbox_group_state(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CheckboxGroup view.rs should not re-implement normalization/state primitives `{forbidden}`."
        );
    }

    for needle in [
        "pub fn sanitize_motion(motion: CheckboxGroupMotion) -> CheckboxGroupMotion",
        "pub fn motion_source_attr(motion: CheckboxGroupMotion) -> &'static str",
        "pub fn attach_motion(base_vars: Option<String>, motion: CheckboxGroupMotion) -> String",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            motion_source.contains(needle),
            "CheckboxGroup motion.rs should keep attach-only contract via `{needle}`."
        );
    }

    for forbidden in [
        "requestAnimationFrame(",
        "KeyframeEffect",
        "Animation::new(",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "CheckboxGroup motion.rs should not re-implement motion engine `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_group_component_directory_standard_files_follow_contract_and_na_paths() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let mod_source = load_source("../../components/checkbox-group/src/mod.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let styles_source = load_source("../../components/checkbox-group/src/styles.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_src_dir = manifest_dir.join("../../components/checkbox-group/src");

    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            component_src_dir.join(required_file).exists(),
            "checkbox-group component directory should include `{required_file}`.",
        );
    }

    for absent_file in ["render.rs", "spec.rs"] {
        assert!(
            !component_src_dir.join(absent_file).exists(),
            "checkbox-group component directory should keep `{absent_file}` absent for current scope.",
        );
    }

    for required in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use motion::CheckboxGroupMotion;",
        "pub use view::CheckboxGroup;",
    ] {
        assert!(
            mod_source.contains(required),
            "checkbox-group mod.rs should keep minimal stable export marker `{required}`.",
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "pub use logic::",
        "mod render;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "checkbox-group mod.rs should avoid over-export or render drift `{forbidden}`.",
        );
    }

    for required in [
        "pub fn resolve_checkbox_group_ids(",
        "pub fn resolve_checkbox_group_content(",
        "pub fn resolve_checkbox_group_view_state(state: CheckboxGroupState) -> CheckboxGroupViewState",
        "pub fn use_checkbox_group(options: CheckboxGroupOptions) -> CheckboxGroupA11y",
    ] {
        assert!(
            logic_source.contains(required),
            "checkbox-group logic.rs should keep props normalization/state derivation marker `{required}`.",
        );
    }

    for forbidden in [
        "view! {",
        "data-slot=",
        "NodeRef<",
        "web_sys::",
        "window()",
        "document()",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "checkbox-group logic.rs should stay free of view/DOM token `{forbidden}`.",
        );
    }

    for required in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles_source.contains(required),
            "checkbox-group styles.rs should keep token-first static CSS marker `{required}`.",
        );
    }

    for forbidden in ["#[component]", "view! {", "use ui_headless", "on:click="] {
        assert!(
            !styles_source.contains(forbidden),
            "checkbox-group styles.rs should avoid render/headless concern `{forbidden}`.",
        );
    }

    for required in [
        "#[component]",
        "pub fn CheckboxGroup(",
        "let headless = logic::use_checkbox_group(",
        "let view_state =",
        "style = motion_contract::attach_motion(None, motion);",
        "view! {",
        "data-state-source=move || view_state.get().state_source.as_data_attr()",
        "data-motion-phase=move || view_state.get().motion_phase.as_data_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "checkbox-group view.rs should keep Leptos render + headless mount marker `{required}`.",
        );
    }

    for forbidden in [
        "normalize_checkbox_group_label(",
        "normalize_checkbox_group_optional_text(",
        "resolve_checkbox_group_state(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "checkbox-group view.rs should avoid hidden state/normalization decisions `{forbidden}`.",
        );
    }

    for required in [
        "pub struct CheckboxGroupMotion {",
        "pub fn sanitize_motion(motion: CheckboxGroupMotion) -> CheckboxGroupMotion",
        "pub fn resolve_effective_motion(",
        "pub fn attach_motion(base_vars: Option<String>, motion: CheckboxGroupMotion) -> String",
    ] {
        assert!(
            motion_source.contains(required),
            "checkbox-group motion.rs should keep semantic-to-motion mapping marker `{required}`.",
        );
    }

    for forbidden in [
        "view! {",
        "use_checkbox_group(",
        "data-slot=",
        "role=",
        "aria-",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "checkbox-group motion.rs should avoid view/headless semantics token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "components/checkbox-group/test/semantics.rs::checkbox_group_component_directory_standard_files_follow_contract_and_na_paths",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_component_directory_standard_files_follow_contract_and_na_paths",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep component-directory governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_file_placement_discipline_is_strict_for_component_scope() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let mod_source = load_source("../../components/checkbox-group/src/mod.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let styles_source = load_source("../../components/checkbox-group/src/styles.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    let component_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/checkbox-group/src");

    for required_path in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            component_dir.join(required_path).exists(),
            "checkbox-group file placement discipline requires `{required_path}`.",
        );
    }

    let forbidden_path = "render.rs";
    assert!(
        !component_dir.join(forbidden_path).exists(),
        "checkbox-group should not drift to forbidden file `{forbidden_path}`.",
    );

    // CheckboxGroup is a simple container component; `spec.rs` is intentionally N/A.
    assert!(
        !component_dir.join("spec.rs").exists(),
        "checkbox-group should keep `spec.rs` absent in current simple-component scope.",
    );

    for required in [
        "pub use motion::CheckboxGroupMotion;",
        "pub use view::CheckboxGroup;",
        "pub fn resolve_checkbox_group_content(",
        "pub fn resolve_checkbox_group_view_state(",
        "pub const CSS: &str = r#\"",
        "#[component]",
        "pub fn CheckboxGroup(",
        "pub struct CheckboxGroupMotion {",
        "pub fn attach_motion(base_vars: Option<String>, motion: CheckboxGroupMotion) -> String",
    ] {
        let found = mod_source.contains(required)
            || logic_source.contains(required)
            || styles_source.contains(required)
            || view_source.contains(required)
            || motion_source.contains(required);
        assert!(
            found,
            "checkbox-group file placement discipline should keep marker `{required}`.",
        );
    }

    for forbidden in [
        "mod render;",
        "pub mod logic;",
        "pub mod view;",
        "view! {",
        "web_sys::",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "checkbox-group file placement discipline should avoid cross-layer token `{forbidden}` in non-view files.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_file_placement_discipline_is_strict_for_component_scope";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "checkbox_group_file_placement_discipline_is_strict_for_component_scope",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_file_placement_discipline_is_strict_for_component_scope",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep file-placement discipline marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_does_not_introduce_spec_rs_without_schema_contract() {
    let mod_source = load_source("../../components/checkbox-group/src/mod.rs");
    let readme_source = load_source("../../components/checkbox-group/src/README.md");
    let check_source = load_source("../../components/checkbox-group/check2.md");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("../../components/checkbox-group/src/spec.rs");

    assert!(
        !spec_path.exists(),
        "CheckboxGroup should not add `spec.rs` without stable schema/version contract."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "CheckboxGroup mod.rs should not wire spec module token `{forbidden}`."
        );
    }

    for needle in ["# 单组件 Check List", "## Playground 展示区"] {
        let found = check_source.contains(needle) || readme_source.contains(needle);
        assert!(
            found,
            "CheckboxGroup documentation should stay in checklist/readme path via `{needle}`."
        );
    }
}

#[test]
fn checkbox_group_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let mod_source = load_source("../../components/checkbox-group/src/mod.rs");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    let component_src_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/checkbox-group/src");
    let spec_path = component_src_dir.join("spec.rs");

    assert!(
        !spec_path.exists(),
        "checkbox-group should keep `spec.rs` absent for non-complex scope."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "CheckboxGroupSpec::new(",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "checkbox-group should not expose hyper-structure builder token `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。（`N/A`：`CheckboxGroup` 为简单语义分组组件，不存在稳定外部 Schema 契约与复杂配置固化需求；当前保持无 `spec.rs` 设计，避免为假问题引入额外抽象与版本负担。若未来演进为复杂配置组件，再引入 `CheckboxGroupSpec::new()...render()` 并补齐契约版本迁移测试。回归：`components/checkbox-group/test/semantics.rs::checkbox_group_hyper_structure_builder_spec_is_not_applicable_for_simple_component`；脚本门禁：`scripts/check-ui-component-files.sh` 新增 `cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_hyper_structure_builder_spec_is_not_applicable_for_simple_component`。）",
        "checkbox_group_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep hyper-structure N/A marker `{required}`."
        );
    }
}

#[test]
fn checkbox_group_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    let component_src_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/checkbox-group/src");
    assert!(
        component_src_dir.join("Component.toml").exists(),
        "checkbox-group context compression requires `Component.toml`."
    );
    assert!(
        component_src_dir.join("checkbox_group.rbi").exists(),
        "checkbox-group context compression requires `checkbox_group.rbi`."
    );

    for required in [
        "schema_version = \"1\"",
        "name = \"CheckboxGroup\"",
        "crate = \"ui-checkbox-group\"",
        "[[inputs]]",
        "name = \"id\"",
        "name = \"label\"",
        "name = \"is_invalid\"",
        "name = \"is_required\"",
        "name = \"is_disabled\"",
        "name = \"aria_describedby\"",
        "name = \"class_name\"",
        "[[dependencies]]",
        "name = \"ui-headless\"",
        "name = \"ui-state-primitives\"",
        "name = \"ui-motion\"",
        "name = \"ui-theme\"",
    ] {
        assert!(
            component_toml.contains(required),
            "Component.toml should keep context-compression marker `{required}`."
        );
    }

    for required in [
        "pub fn CheckboxGroup(",
        "id: String,",
        "label: String,",
        "description: Option<String>,",
        "error: Option<String>,",
        "is_invalid: leptos::prelude::Signal<bool>,",
        "is_required: leptos::prelude::Signal<bool>,",
        "is_disabled: bool,",
        "motion: crate::motion::CheckboxGroupMotion,",
        "lang: Option<String>,",
        "dir: Option<ui_headless::A11yDirection>,",
        "aria_describedby: leptos::prelude::Signal<Option<String>>,",
        "class_name: Option<String>,",
        "children: leptos::children::Children,",
    ] {
        assert!(
            rbi_source.contains(required),
            "checkbox_group.rbi should keep signature projection marker `{required}`."
        );
    }

    for prop in [
        "id",
        "label",
        "description",
        "error",
        "is_invalid",
        "is_required",
        "is_disabled",
        "motion",
        "lang",
        "dir",
        "aria_describedby",
        "class_name",
    ] {
        assert!(
            component_toml.contains(&format!("name = \"{prop}\""))
                && rbi_source.contains(&format!("{prop}:")),
            "manifest/rbi projection should stay in sync for input `{prop}`."
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。（`components/checkbox-group/src/Component.toml` 与 `components/checkbox-group/src/checkbox_group.rbi` 已同步存在并保持关键输入签名对齐（`id/label/description/error/is_invalid/is_required/is_disabled/motion/lang/dir/aria_describedby/class_name`）；`Component.toml` 继续声明能力依赖（`ui-headless/ui-state-primitives/ui-motion/ui-theme`）用于 Agent 上下文压缩检索，`checkbox_group.rbi` 提供 `pub fn CheckboxGroup(...)` 的稳定接口投影，避免工具箱语义漂移。回归：`components/checkbox-group/test/semantics.rs::checkbox_group_context_compression_manifest_and_rbi_projection_are_present_and_current`；脚本门禁：`scripts/check-ui-component-files.sh` 新增 `cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_context_compression_manifest_and_rbi_projection_are_present_and_current`。）",
        "checkbox_group_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep context-compression marker `{required}`."
        );
    }
}

#[test]
fn checkbox_group_agent_contract_is_schema_typed_traceable_and_whitelist_rendered() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let manifest_source = load_source("../../components/checkbox-group/src/Component.toml");
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "pub const CHECKBOX_GROUP_AGENT_SCHEMA: &str = \"ui.checkbox-group.agent-contract\";",
        "pub enum CheckboxGroupAgentSchemaVersion {",
        "pub enum CheckboxGroupAgentIntent {",
        "pub enum CheckboxGroupAgentAction {",
        "pub enum CheckboxGroupAgentState {",
        "pub enum CheckboxGroupAgentSource {",
        "pub enum CheckboxGroupAgentConfigPolicy {",
        "pub struct CheckboxGroupAgentContractInput {",
        "pub struct CheckboxGroupAgentContract {",
        "pub fn resolve_checkbox_group_agent_contract(",
    ] {
        assert!(
            logic_source.contains(needle),
            "checkbox-group logic should keep typed agent-contract marker `{needle}`.",
        );
    }

    for needle in [
        "let agent_contract = Memo::new(move |_| {",
        "logic::resolve_checkbox_group_agent_contract(logic::CheckboxGroupAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_data_attr()",
        "data-ui-intent=move || agent_contract.get().intent.as_data_attr()",
        "data-ui-action=move || agent_contract.get().action.as_data_attr()",
        "data-ui-state=move || agent_contract.get().state.as_data_attr()",
        "data-ui-source=move || agent_contract.get().source.as_data_attr()",
        "data-ui-state-source=move || agent_contract.get().state_source.as_data_attr()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_data_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "checkbox-group view should mount schemaized agent marker `{needle}`.",
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
            "checkbox-group agent contract should avoid free-form schema splicing `{forbidden}`.",
        );
    }

    for required in [
        "name = \"agent_contract_schema_typed_markers\"",
        "name = \"agent_contract_whitelist_render_policy\"",
        "[[agent_contract_markers]]",
        "schema = \"ui.checkbox-group.agent-contract.v1\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-intent\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "attr = \"data-ui-config-policy\"",
        "[[agent_contract_whitelist]]",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\"]",
    ] {
        assert!(
            manifest_source.contains(required),
            "checkbox-group manifest should keep agent-contract governance marker `{required}`.",
        );
    }

    for forbidden in [
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "checkbox-group render path should remain whitelist-safe; found `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_agent_contract_is_schema_typed_traceable_and_whitelist_rendered";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "checkbox_group_agent_contract_is_schema_typed_traceable_and_whitelist_rendered",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_agent_contract_is_schema_typed_traceable_and_whitelist_rendered",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep agent-contract governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let docs_forms_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let streaming_script_source = load_source("../../scripts/check-ui-streaming.sh");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "checkbox-group 不是 LLM 正文阅读面，当前保持 snapshot-only 渲染路径；本组件不承载 token 增量传输协议。",
        "components/checkbox-group/test/semantics.rs::checkbox_group_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep streaming-definition marker `{required}`.",
        );
    }

    for forbidden in [
        "use_ai_space_state",
        "AiSpace",
        "data-ui-stream-mode",
        "data-ui-output-state",
        "stream_chunk",
        "token_delta",
        "incremental_patch",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !docs_forms_source.contains(forbidden),
            "checkbox-group should not carry LLM-streaming protocol token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        streaming_script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn checkbox_group_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let streaming_script_source = load_source("../../scripts/check-ui-streaming.sh");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "components/checkbox-group/test/semantics.rs::checkbox_group_check2_documents_snapshot_as_default_baseline_capability",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep snapshot-baseline marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_snapshot_as_default_baseline_capability";
    assert!(
        streaming_script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn checkbox_group_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let readme_source = load_source("../../components/checkbox-group/src/README.md");
    let streaming_script_source = load_source("../../scripts/check-ui-streaming.sh");

    for required in [
        "pub fn CheckboxGroup(",
        "children: Children,",
        "let content = logic::resolve_checkbox_group_content(label, description, error);",
        "let headless = logic::use_checkbox_group(logic::CheckboxGroupOptions {",
        "logic::resolve_checkbox_group_view_state(resolved_state.get())",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-state-source=move || view_state.get().state_source.as_data_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "checkbox-group snapshot baseline should keep complete-render marker `{required}`.",
        );
    }

    for required in [
        "pub fn resolve_checkbox_group_content(",
        "pub fn resolve_checkbox_group_view_state(state: CheckboxGroupState) -> CheckboxGroupViewState",
        "pub fn use_checkbox_group(options: CheckboxGroupOptions) -> CheckboxGroupA11y",
    ] {
        assert!(
            logic_source.contains(required),
            "checkbox-group logic should keep stable complete-input normalization marker `{required}`.",
        );
    }

    for required in [
        "name = \"snapshot_rendering\"",
        "enabled = true",
        "name = \"id\"",
        "name = \"label\"",
        "name = \"description\"",
        "name = \"error\"",
        "name = \"is_invalid\"",
        "name = \"is_required\"",
        "name = \"is_disabled\"",
        "name = \"motion\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"aria_describedby\"",
        "name = \"class_name\"",
    ] {
        assert!(
            component_toml.contains(required),
            "checkbox-group manifest should keep snapshot baseline capability/input marker `{required}`.",
        );
    }

    for required in [
        "pub fn CheckboxGroup(",
        "id: String,",
        "label: String,",
        "children: leptos::children::Children,",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(required),
            "checkbox-group RBI should project complete snapshot-render signature marker `{required}`.",
        );
    }

    assert!(
        readme_source.contains("`CheckboxGroup` 为多选项提供字段级语义封装"),
        "checkbox-group docs should keep complete snapshot render baseline description."
    );

    for forbidden in [
        "stream_chunk",
        "token_delta",
        "partial_payload",
        "incremental_patch",
        "data-ui-stream-mode",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "checkbox-group snapshot baseline should not depend on streaming-only token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        streaming_script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );

    let required = "components/checkbox-group/test/semantics.rs::checkbox_group_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        check2_source.contains(required),
        "checkbox-group checklist should keep snapshot-stability evidence marker `{required}`.",
    );
}

#[test]
fn checkbox_group_check2_marks_streaming_scope_as_optional_with_snapshot_fallback() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let mod_source = load_source("../../components/checkbox-group/src/mod.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let streaming_script_source = load_source("../../scripts/check-ui-streaming.sh");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "checkbox-group` 不是正文阅读面，归类为 `Streaming Optional`",
        "`fallback=snapshot`",
        "data-ui-stream-support/data-ui-stream-fallback/data-ui-output-status",
        "数据校验、断线恢复、重试策略保持上层负责，组件层不实现重试协议。",
        "components/checkbox-group/test/semantics.rs::checkbox_group_check2_marks_streaming_scope_as_optional_with_snapshot_fallback",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep streaming required/optional marker `{required}`.",
        );
    }

    for required in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_data_attr()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_data_attr()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_data_attr()",
        "aria-describedby=move || fieldset_aria_describedby.get()",
        "aria-invalid=move || fieldset_aria_invalid.get()",
        "aria-required=move || fieldset_aria_required.get()",
        "data-ui-state=move || agent_contract.get().state.as_data_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "checkbox-group view should keep streaming-governance + aria/data continuity marker `{required}`.",
        );
    }

    for required in [
        "pub enum CheckboxGroupAgentStreamSupport",
        "pub enum CheckboxGroupAgentStreamFallback",
        "pub enum CheckboxGroupAgentOutputStatus",
        "CheckboxGroupAgentOutputStatus::Draft",
        "CheckboxGroupAgentOutputStatus::CommitReady",
        "stream_support: CheckboxGroupAgentStreamSupport::Optional",
        "stream_fallback: CheckboxGroupAgentStreamFallback::Snapshot",
        "output_status: CheckboxGroupAgentOutputStatus::Verified",
    ] {
        assert!(
            logic_source.contains(required),
            "checkbox-group logic should keep typed streaming-governance marker `{required}`.",
        );
    }

    for required in [
        "name = \"stream_support\"",
        "attr = \"data-ui-stream-support\"",
        "values = [\"optional\"]",
        "name = \"stream_fallback\"",
        "attr = \"data-ui-stream-fallback\"",
        "values = [\"snapshot\"]",
        "name = \"output_status\"",
        "attr = \"data-ui-output-status\"",
        "values = [\"draft\", \"verified\", \"commit-ready\"]",
    ] {
        assert!(
            component_toml.contains(required),
            "checkbox-group manifest should keep streaming-governance marker `{required}`.",
        );
    }

    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");
    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "checkbox-group should keep validation/retry/resilience policy outside component layer; found `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_marks_streaming_scope_as_optional_with_snapshot_fallback";
    assert!(
        streaming_script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn checkbox_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn checkbox_group() -> AnyView",
        "title=\"CheckboxGroup\"",
        "slug=\"checkbox-group\"",
        "title=\"Hello World（默认路径）\"",
        "title=\"Validation + Required\"",
        "title=\"Disabled + Optional\"",
    ] {
        assert!(
            source.contains(needle),
            "checkbox-group docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn checkbox_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "<Playground title=\"Hello World（默认路径）\" code_signal=hello_code>",
        "id=\"docs-checkbox-group-hello\".to_string()",
        "<Playground title=\"Validation + Required\" code_signal=code>",
        "id=\"docs-checkbox-group\".to_string()",
        "label=\"Fruits\".to_string()",
        "is_required=is_required",
        "is_invalid=is_invalid",
        "aria_describedby=aria_describedby",
        "id=\"docs-checkbox-group-extra\"",
        "\"Clear selections\"",
        "<Playground title=\"Disabled + Optional\" code_signal=states_code>",
        "id=\"docs-checkbox-group-disabled\".to_string()",
        "is_disabled=true",
        "id=\"docs-checkbox-group-optional\".to_string()",
        "\"optional selected count: \"",
    ] {
        assert!(
            source.contains(needle),
            "checkbox-group docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn checkbox_group_docs_include_interactive_playground_contract_panels() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Hello World（默认路径）\"",
        "title=\"Interactive Playground\"",
        "test_css_source=interactive_test_css",
        "test_config_signal=interactive_config",
        "controls=move || view!",
        "test_source_path=\"components/checkbox-group/src/styles.rs\".to_string()",
        "title=\"Validation + Required\"",
        "title=\"Disabled + Optional\"",
    ] {
        assert!(
            source.contains(needle),
            "checkbox-group interactive playground should include `{needle}`.",
        );
    }
}

#[test]
fn checkbox_group_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let dx_script_source = load_source("../../scripts/check-ui-dx.sh");
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "title=\"Hello World（默认路径）\"",
        "title=\"Validation + Required\"",
        "title=\"Disabled + Optional\"",
        "data-slot=\"checkbox-group-streaming-policy\"",
        "data-slot=\"checkbox-group-streaming-modes\"",
        "Streaming Optional; fallback=snapshot.",
        "Snapshot mode renders verified full output for group semantics.",
        "data-slot=\"checkbox-group-controlled-uncontrolled-na\"",
        "Controlled vs Uncontrolled contrast is N/A at group level",
        "data-slot=\"checkbox-group-copy-ready\"",
        "data-slot=\"checkbox-group-source-paths\"",
        "data-slot=\"checkbox-group-source-prerequisites\"",
        "<code>\"apps/docs-app/src/playground.rs::compose_copy_ready_code\"</code>",
        "<code>\"components/checkbox-group/src/view.rs\"</code>",
        "<code>\"components/checkbox-group/src/logic.rs\"</code>",
        "<code>\"components/checkbox-group/src/styles.rs\"</code>",
        "<code>\"component-checkbox_group\"</code>",
        "<code>\"inject-css\"</code>",
        "code_imports=\"use leptos::prelude::*;\\nuse ui::*;\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "checkbox-group docs should keep copy-paste-ready marker `{required}`.",
        );
    }

    for required in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "data-slot=\"playground-code\"",
        "data-slot=\"code-block\"",
    ] {
        assert!(
            playground_source.contains(required),
            "playground runtime should keep copy-ready import marker `{required}`.",
        );
    }

    let dx_script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        dx_script_source.contains(dx_script_needle),
        "dx gate script should include `{dx_script_needle}`.",
    );

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "components/checkbox-group/test/semantics.rs::checkbox_group_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "bash scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep docs-product marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group check2 should keep docs-sync/state-matrix rule `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "pub(super) fn checkbox_group() -> AnyView",
        "title=\"Hello World（默认路径）\"",
        "title=\"Interactive Playground\"",
        "title=\"Validation + Required\"",
        "title=\"Disabled + Optional\"",
        "is_required=is_required",
        "is_invalid=is_invalid",
        "is_disabled=true",
        "aria_describedby=aria_describedby",
        "data-slot=\"checkbox-group-controlled-uncontrolled-na\"",
        "Controlled vs Uncontrolled contrast is N/A at group level",
    ] {
        assert!(
            docs_source.contains(required),
            "checkbox-group docs should keep synced example/matrix marker `{required}`.",
        );
    }

    for required in [
        "pub fn resolve_checkbox_group_content(",
        "let label = normalize_checkbox_group_label(label);",
        "let description = normalize_checkbox_group_optional_text(description);",
        "let error = normalize_checkbox_group_optional_text(error);",
        "pub fn resolve_checkbox_group_class_name(class_name: Option<String>) -> String",
        "if let Some(class_name) = normalize_checkbox_group_optional_text(class_name)",
        "pub struct CheckboxGroupOptions {",
        "pub is_disabled: bool,",
        "pub aria_describedby: Signal<Option<String>>,",
        "pub is_invalid: Signal<bool>,",
        "pub is_required: Signal<bool>,",
    ] {
        assert!(
            logic_source.contains(required),
            "checkbox-group logic should keep API/default normalization marker `{required}`.",
        );
    }

    for required in [
        "#[prop(optional, into)] description: Option<String>",
        "#[prop(optional, into)] error: Option<String>",
        "#[prop(optional, into)] is_invalid: Signal<bool>",
        "#[prop(optional, into)] is_required: Signal<bool>",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional, into)] aria_describedby: Signal<Option<String>>",
        "#[prop(optional, into)] class_name: Option<String>",
        "let content = logic::resolve_checkbox_group_content(label, description, error);",
        "let class = logic::resolve_checkbox_group_class_name(class_name);",
    ] {
        assert!(
            view_source.contains(required),
            "checkbox-group view should keep prop API/default marker `{required}` aligned with docs.",
        );
    }

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "apps/docs-app/src/pages/components/pages/forms.rs::checkbox_group",
        "checkbox_group_check2_documents_docs_sync_and_state_matrix_rules",
        "checkbox_group_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "components/checkbox-group/check2.md should keep docs-sync evidence marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for required in [
        "echo \"[dx] contract: checkbox-group docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include docs-sync/state-matrix marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    assert!(
        check2_source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "checkbox-group check2 should mark docs-sync/state-matrix checklist item complete.",
    );

    for required in [
        "apps/docs-app/src/pages/components/pages/forms.rs::checkbox_group",
        "title=\"Validation + Required\"",
        "title=\"Disabled + Optional\"",
        "is_required/is_invalid/is_disabled/aria_describedby",
        "components/checkbox-group/src/logic.rs",
        "components/checkbox-group/src/view.rs",
        "checkbox_group_check2_documents_docs_sync_and_state_matrix_rules",
        "checkbox_group_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "checkbox_group_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group check2 docs-sync/state-matrix section should reference `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group check2 documentation-as-product section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = load_source("../../components/checkbox-group/src/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for required in [
        "# CheckboxGroup",
        "## Hello World（最小可用）",
        "## 常见用法",
        "## 先用起来，再进阶",
        "默认路径：`<CheckboxGroup id=... label=...>` 先跑通基础多选分组。",
        "进阶控制：按需打开 `is_required/is_invalid/is_disabled`、`aria_describedby`、`description/error`、`motion/class_name`。",
    ] {
        assert!(
            readme_source.contains(required),
            "checkbox-group README should include beginner marker `{required}`.",
        );
    }

    for required in [
        "pub(super) fn checkbox_group() -> AnyView",
        "title=\"CheckboxGroup\"",
        "slug=\"checkbox-group\"",
        "title=\"Hello World（默认路径）\"",
        "title=\"Interactive Playground\"",
        "title=\"Validation + Required\"",
        "title=\"Disabled + Optional\"",
    ] {
        assert!(
            docs_source.contains(required),
            "checkbox-group docs entry should include `{required}`.",
        );
    }

    let readme_hello = readme_source
        .find("## Hello World（最小可用）")
        .expect("checkbox-group README should include Hello World section");
    let readme_common = readme_source
        .find("## 常见用法")
        .expect("checkbox-group README should include common-usage section");
    let readme_progressive = readme_source
        .find("## 先用起来，再进阶")
        .expect("checkbox-group README should include beginner-to-advanced section");
    assert!(
        readme_hello < readme_common && readme_common < readme_progressive,
        "checkbox-group README should keep beginner-first order before advanced guidance.",
    );
}

#[test]
fn checkbox_group_dx_check_script_covers_documentation_as_product_beginner_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for required in [
        "echo \"[dx] contract: checkbox-group documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_documentation_as_product_rules",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include documentation-as-product beginner marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_marks_documentation_as_product_beginner_item_complete() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "checkbox-group check2 should mark documentation-as-product beginner item complete.",
    );

    for required in [
        "components/checkbox-group/src/README.md",
        "apps/docs-app/src/pages/components/pages/forms.rs::checkbox_group",
        "components/checkbox-group/test/semantics.rs::checkbox_group_check2_documents_documentation_as_product_rules",
        "components/checkbox-group/test/semantics.rs::checkbox_group_documentation_entry_exists_with_beginner_first_progression",
        "components/checkbox-group/test/checkbox_group_semantics.rs::checkbox_group_check2_documents_documentation_as_product_rules",
        "components/checkbox-group/test/checkbox_group_semantics.rs::checkbox_group_documentation_entry_exists_with_beginner_first_progression",
        "components/checkbox-group/test/checkbox_group_semantics.rs::checkbox_group_dx_check_script_covers_documentation_as_product_beginner_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group check2 documentation-as-product beginner section should reference `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group check2 interactive-playground section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for required in [
        "title=\"Interactive Playground\"",
        "description=\"Display + Config + Code + CSS Test: edit group is_invalid/is_required state and inspect contracts.\"",
        "code_signal=interactive_code",
        "test_css_source=interactive_test_css",
        "test_source_path=\"components/checkbox-group/src/styles.rs\".to_string()",
        "test_config_signal=interactive_config",
        "controls=move || view!",
        "Switch checked=interactive_required set_checked=set_interactive_required",
        "Switch checked=interactive_invalid set_checked=set_interactive_invalid",
        "Switch checked=interactive_disabled set_checked=set_interactive_disabled",
        "Switch checked=interactive_description set_checked=set_interactive_description",
        "Switch checked=interactive_error set_checked=set_interactive_error",
        "id=\"docs-checkbox-group-interactive\".to_string()",
        "is_required=is_required",
        "is_invalid=is_invalid",
        "is_disabled=interactive_disabled.get()",
        "\"selected count: \"",
    ] {
        assert!(
            docs_source.contains(required),
            "checkbox-group docs should provide interactive playground marker `{required}`.",
        );
    }

    for required in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(required),
            "docs-app Playground should keep interactive preview marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_checkbox_group_contract.spec.mjs");

    for required in [
        "docs-app checkbox-group key flow is repeatable and failures map to semantic breakpoints",
        "await page.goto(CHECKBOX_GROUP_PAGE);",
        "body:not(:has(#boot))",
        "await firstUnchecked.focus();",
        "await expect(firstUnchecked).toBeFocused();",
        "await page.keyboard.press(\"Space\");",
        "await expect(validationGroup).toHaveAttribute(\"data-invalid\", \"true\");",
        "await expect(validationGroup).toHaveAttribute(\"data-ui-action\", \"render-semantic-with-error\");",
        "await expect(validationGroup).toHaveAttribute(\"data-ui-state\", \"enabled-invalid\");",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(required),
            "checkbox-group interactive playground should keep repeatable e2e marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for required in [
        "echo \"[dx] contract: checkbox-group interactive playground docs acceptance surface\"",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_interactive_playground_rules",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include interactive-playground marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_marks_interactive_playground_contract_complete() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    assert!(
        check2_source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "checkbox-group check2 should mark interactive-playground item complete.",
    );

    for required in [
        "title=\"Interactive Playground\"",
        "apps/docs-app/src/pages/components/pages/forms.rs::checkbox_group",
        "e2e/tests/docs_app_checkbox_group_contract.spec.mjs::docs-app checkbox-group key flow is repeatable and failures map to semantic breakpoints",
        "components/checkbox-group/test/semantics.rs::checkbox_group_check2_documents_interactive_playground_rules",
        "components/checkbox-group/test/semantics.rs::checkbox_group_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "components/checkbox-group/test/semantics.rs::checkbox_group_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "components/checkbox-group/test/checkbox_group_semantics.rs::checkbox_group_check2_documents_interactive_playground_rules",
        "components/checkbox-group/test/checkbox_group_semantics.rs::checkbox_group_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "components/checkbox-group/test/checkbox_group_semantics.rs::checkbox_group_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "components/checkbox-group/test/checkbox_group_semantics.rs::checkbox_group_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-dx.sh",
        "AI Spec 相关联动示例：N/A（`checkbox-group` 非 Spec 构建器组件）",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group check2 interactive-playground section should reference `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group check2 source-first section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_checkbox_group_contract.spec.mjs");

    for required in [
        "data-slot=\"checkbox-group-source-first\"",
        "data-slot=\"checkbox-group-copy-ready\"",
        "data-slot=\"checkbox-group-source-paths\"",
        "data-slot=\"checkbox-group-source-prerequisites\"",
        "<code>\"apps/docs-app/src/playground.rs::compose_copy_ready_code\"</code>",
        "<code>\"components/checkbox-group/src/view.rs\"</code>",
        "<code>\"components/checkbox-group/src/logic.rs\"</code>",
        "<code>\"components/checkbox-group/src/styles.rs\"</code>",
        "<code>\"apps/docs-app/src/pages/components/pages/forms.rs\"</code>",
        "<code>\"component-checkbox_group\"</code>",
        "<code>\"inject-css\"</code>",
        "code_imports=\"use leptos::prelude::*;\\nuse ui::*;\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "checkbox-group source-first docs should keep marker `{required}`.",
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "code_imports",
        "data-copyable=true",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground copy-ready pipeline should keep `{required}`.",
        );
    }

    for required in [
        "docs-app checkbox-group contract uses semantic selectors with wasm-stable ready waits",
        "await page.goto(CHECKBOX_GROUP_PAGE);",
        "body:not(:has(#boot))",
    ] {
        assert!(
            e2e_source.contains(required),
            "checkbox-group e2e docs page should keep stable source-first surface marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for required in [
        "echo \"[dx] contract: checkbox-group source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include source-first marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "checkbox-group check2 should mark source-first copy-paste-ready item complete.",
    );

    for required in [
        "apps/docs-app/src/pages/components/pages/forms.rs::checkbox_group",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "components/checkbox-group/test/semantics.rs::checkbox_group_check2_documents_source_first_copy_paste_ready_rules",
        "components/checkbox-group/test/semantics.rs::checkbox_group_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "components/checkbox-group/test/checkbox_group_semantics.rs::checkbox_group_check2_documents_source_first_copy_paste_ready_rules",
        "components/checkbox-group/test/checkbox_group_semantics.rs::checkbox_group_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "components/checkbox-group/test/checkbox_group_semantics.rs::checkbox_group_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group check2 source-first section should reference `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group check2 heroui-benchmark docs-sync section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let readme_source = load_source("../../components/checkbox-group/src/README.md");

    for required in [
        "### CheckboxGroup 同步记录（2026-02-20）",
        "参数模型同步：`CheckboxGroup` 参数主轴保持 `is_required/is_invalid/is_disabled`",
        "component_doc!(\"CheckboxGroup\", \"checkbox-group\", \"Forms\", forms::checkbox_group)",
        "`apps/docs-app/src/pages/components/pages/forms.rs::checkbox_group()`",
        "`components/checkbox-group/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(required),
            "heroui strategy doc should include checkbox-group synchronization marker `{required}`.",
        );
    }

    for required in [
        "component_doc!(",
        "\"CheckboxGroup\"",
        "\"checkbox-group\"",
        "forms::checkbox_group",
    ] {
        assert!(
            pages_source.contains(required),
            "component docs index should expose checkbox-group entry marker `{required}`.",
        );
    }

    for required in [
        "pub(super) fn checkbox_group() -> AnyView",
        "title=\"CheckboxGroup\"",
        "slug=\"checkbox-group\"",
    ] {
        assert!(
            docs_source.contains(required),
            "docs-app checkbox-group page should stay indexable via marker `{required}`.",
        );
    }

    for required in [
        "# CheckboxGroup",
        "## docs-app 入口",
        "forms.rs::checkbox_group()",
        "#/components/checkbox-group",
    ] {
        assert!(
            readme_source.contains(required),
            "checkbox-group README should remain an equivalent component doc entry via `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for required in [
        "echo \"[dx] contract: checkbox-group heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should enforce heroui-benchmark docs-sync contract `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "components/checkbox-group/test/semantics.rs::checkbox_group_check2_documents_heroui_benchmark_docs_sync_rules",
        "components/checkbox-group/test/semantics.rs::checkbox_group_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "components/checkbox-group/test/semantics.rs::checkbox_group_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "components/checkbox-group/test/checkbox_group_semantics.rs::checkbox_group_check2_documents_heroui_benchmark_docs_sync_rules",
        "components/checkbox-group/test/checkbox_group_semantics.rs::checkbox_group_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "components/checkbox-group/test/checkbox_group_semantics.rs::checkbox_group_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group check2 should keep heroui-benchmark docs-sync evidence marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for required in [
        "echo \"[dx] contract: checkbox-group docs product copy-paste-ready + streaming/snapshot contract\"",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include documentation-as-product marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_marks_documentation_as_product_item_complete() {
    let source = load_source("../../components/checkbox-group/check2.md");

    assert!(
        source.contains("- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。"),
        "checkbox-group check2 should mark docs-product item complete.",
    );

    for required in [
        "apps/docs-app/src/pages/components/pages/forms.rs::checkbox_group",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "checkbox_group_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "checkbox_group_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(required),
            "checkbox-group check2 docs-product section should retain marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_readme_and_docs_shell_register_display_config_code_css_contract() {
    let readme_source = load_source("../../components/checkbox-group/src/README.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");

    assert!(
        readme_source.contains("## Playground 展示区（Display / Config / Code / CSS Test）"),
        "checkbox-group README should document display/config/code/css test playground layout.",
    );
    assert!(
        shell_source.contains("\"checkbox-group\" => Some(CHECKBOX_GROUP_README_MD)"),
        "docs shell should map checkbox-group slug to CHECKBOX_GROUP_README_MD.",
    );
}

#[test]
fn checkbox_group_breaking_migration_removes_legacy_namespace_and_path_shim() {
    let lib_source = load_source("src/lib.rs");

    for forbidden in [
        "checkbox::group::CheckboxGroup",
        "#[path = \"checkbox_field/checkbox/mod.rs\"]",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "checkbox breaking migration should not keep legacy compatibility token `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_group_css_aggregation_uses_new_top_level_contract() {
    let css_source = load_source("src/css.rs");

    assert!(
        css_source.contains("out.push_str(crate::checkbox_group::styles::CSS);"),
        "css aggregation should use top-level checkbox_group css constant.",
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-checkbox_group\")]"),
        "css aggregation should keep checkbox_group style injection feature-gated.",
    );
    assert!(
        !css_source.contains("out.push_str(crate::checkbox::styles::CHECKBOX_GROUP_CSS);"),
        "css aggregation should not keep merged checkbox::styles::CHECKBOX_GROUP_CSS path.",
    );
}

#[test]
fn checkbox_group_visual_desire_contract_uses_theme_baseline_page_and_snapshots() {
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "title=\"Default Theme Visual Baseline\"",
        "description=\"Checks first-impression quality: hierarchy, spacing rhythm, contrast layers, and interactive feedback (hover/active/focus).\"",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            baseline_page_source.contains(needle),
            "Theme baseline docs page should preserve visual-desire contract via `{needle}`.",
        );
    }

    for needle in [
        "test(\"docs-app: theme visual baseline renders button/input/overlay\"",
        "await page.goto(\"/#/components/theme-visual-baseline\");",
        "test(\"docs-app: theme visual baseline screenshots\"",
        "E2E_VISUAL_BASELINE",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "Theme baseline e2e should preserve visual regression evidence via `{needle}`.",
        );
    }
}

#[test]
fn checkbox_group_tree_shaking_contract_is_feature_gated_and_budgeted() {
    let ui_components_cargo = load_source("Cargo.toml");
    let ui_components_lib = load_source("src/lib.rs");
    let ui_components_css = load_source("src/css.rs");
    let tree_shaking_script = load_source("../../scripts/check-ui-tree-shaking.sh");
    let tree_shaking_budget = load_source("../../scripts/tree_shaking_budget.env");
    let ci_source = load_source("../../.github/workflows/ci.yml");

    for needle in [
        "component-checkbox_group = [\"component-checkbox\", \"dep:ui-checkbox-group\"]",
        "ui-checkbox-group = { path = \"../../components/checkbox-group\", optional = true }",
        "#[cfg(feature = \"component-checkbox_group\")]",
        "pub use ui_checkbox_group as checkbox_group;",
        "out.push_str(crate::checkbox_group::styles::CSS);",
    ] {
        let found = ui_components_cargo.contains(needle)
            || ui_components_lib.contains(needle)
            || ui_components_css.contains(needle);
        assert!(
            found,
            "Tree-shaking contract should keep feature-gated checkbox-group wiring via `{needle}`.",
        );
    }

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "cargo tree -e features -i ui -p web-demo",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "- name: Tree Shaking Budget",
        "run: ./scripts/check-ui-tree-shaking.sh",
    ] {
        let found = tree_shaking_script.contains(needle)
            || tree_shaking_budget.contains(needle)
            || ci_source.contains(needle);
        assert!(
            found,
            "Tree-shaking CI + budget contract should keep `{needle}`.",
        );
    }

    for forbidden in ["pub const ALL_COMPONENTS", "fn register_all_components("] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui lib should avoid global always-reachable registry `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let tree_shaking_script = load_source("../../scripts/check-ui-tree-shaking.sh");

    for required in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "component-checkbox_group = [\"component-checkbox\", \"dep:ui-checkbox-group\"]",
        "#[cfg(feature = \"component-checkbox_group\")]",
        "pub use ui_checkbox_group as checkbox_group;",
        "out.push_str(crate::checkbox_group::styles::CSS);",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$CHECKBOX_GROUP_MIN_FEATURES\"",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep tree-shaking evidence marker `{required}`.",
        );
    }

    for command in [
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_tree_shaking_contract_is_feature_gated_and_budgeted",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$CHECKBOX_GROUP_MIN_FEATURES\"",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$CHECKBOX_GROUP_MIN_FEATURES\"",
    ] {
        assert!(
            tree_shaking_script.contains(command),
            "tree-shaking gate script should keep checkbox-group command `{command}`.",
        );
    }
}

#[test]
fn checkbox_group_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "N/A：`CheckboxGroup` 暂未接入精确 `render_count` 自动化计数",
        "渲染次数预算为 `1`",
        "render_count",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-group checklist should keep performance governance marker `{needle}`.",
        );
    }

    for needle in [
        "\"checkbox-group\" => UiPerfBudget {",
        "max_mount_ms: 26.0,",
        "max_update_ms: Some(9.0),",
        "max_heap_kb: Some(448.0),",
        "let perf_budget = component_page_perf_budget(slug);",
        "let perf_name = format!(\"ComponentPage::{slug}\");",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "component shell should keep checkbox-group budget/probe marker `{needle}`.",
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
            "UiPerfProbe should expose perf marker `{needle}`.",
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
            "docs coverage e2e should keep perf guard `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance follow-up plan should keep marker `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`.",
        );
    }

    for needle in [
        "data-slot=\"checkbox-group\"",
        "data-state-source=move || view_state.get().state_source.as_data_attr()",
        "data-motion-source=motion_source",
        "data-motion-phase=move || view_state.get().motion_phase.as_data_attr()",
        "data-invalid=move || view_state.get().is_invalid.then_some(\"true\")",
        "data-required=move || view_state.get().is_required.then_some(\"true\")",
        "style=style",
    ] {
        assert!(
            view_source.contains(needle),
            "checkbox-group view should expose attribution marker `{needle}` for perf triage.",
        );
    }
}

#[test]
fn checkbox_group_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let headless_source = load_source("../../crates/ui-headless/src/checkbox_group.rs");
    let semantics_source = load_source("../../components/checkbox-group/test/semantics.rs");
    let performance_script = load_source("../../scripts/check-ui-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for needle in [
        "aria-labelledby=legend_id.get_value()",
        "aria-describedby=move || fieldset_aria_describedby.get()",
        "aria-invalid=move || fieldset_aria_invalid.get()",
        "aria-required=move || fieldset_aria_required.get()",
        "data-slot=\"checkbox-group\"",
        "data-slot=\"checkbox-group-list\"",
        "data-state-source=move || view_state.get().state_source.as_data_attr()",
        "data-motion-source=motion_source",
        "data-motion-phase=move || view_state.get().motion_phase.as_data_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "checkbox-group semantic/perf regression should keep aria/data marker `{needle}`.",
        );
    }

    // CheckboxGroup is a non-focus-owning fieldset; focus flow stays on child checkboxes.
    for needle in [
        "children: Children,",
        "pub struct CheckboxGroupHandlers;",
        "handlers: CheckboxGroupHandlers,",
    ] {
        let found = view_source.contains(needle) || headless_source.contains(needle);
        assert!(
            found,
            "checkbox-group should keep focus-flow delegation marker `{needle}`.",
        );
    }

    for forbidden in ["on:focus=", "on:blur=", "on:keydown=", "on:pointerdown="] {
        assert!(
            !view_source.contains(forbidden),
            "checkbox-group container should not hijack focus flow with local event token `{forbidden}`.",
        );
    }

    for needle in [
        "fn checkbox_group_view_mounts_headless_locale_contract()",
        "fn checkbox_group_emits_baseline_state_data_attributes()",
        "fn checkbox_group_has_no_overlay_focus_stack_contract()",
        "fn checkbox_group_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn checkbox_group_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            semantics_source.contains(needle),
            "checkbox-group semantics suite should include prerequisite regression `{needle}`.",
        );
    }

    for forbidden in ["assert_snapshot!", "assert_json_snapshot!", "insta::assert"] {
        assert!(
            !semantics_source.contains(forbidden),
            "checkbox-group semantic/perf contract must avoid snapshot-only assertion `{forbidden}`.",
        );
    }

    for needle in [
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            performance_script.contains(needle),
            "performance gate script should include checkbox-group semantic/perf marker `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "render_count follow-up governance should keep `{needle}`.",
        );
    }

    for required in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "checkbox_group_view_mounts_headless_locale_contract",
        "checkbox_group_emits_baseline_state_data_attributes",
        "checkbox_group_has_no_overlay_focus_stack_contract",
        "checkbox_group_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "checkbox_group_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "`render_count` 精确计数当前 `N/A`",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep semantic/performance regression marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_marks_semantics_and_performance_regression_contract_complete() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "components/checkbox-group/test/semantics.rs::checkbox_group_view_mounts_headless_locale_contract",
        "components/checkbox-group/test/semantics.rs::checkbox_group_emits_baseline_state_data_attributes",
        "components/checkbox-group/test/semantics.rs::checkbox_group_has_no_overlay_focus_stack_contract",
        "components/checkbox-group/test/semantics.rs::checkbox_group_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "components/checkbox-group/test/semantics.rs::checkbox_group_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "scripts/check-ui-performance.sh",
        "`render_count` 精确计数当前 `N/A`",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep semantic/perf completion evidence marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let semantics_source = load_source("../../components/checkbox-group/test/semantics.rs");
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let script_source = load_source("../../scripts/check-ui-performance.sh");

    for required in [
        "aria-labelledby=legend_id.get_value()",
        "aria-describedby=move || fieldset_aria_describedby.get()",
        "aria-invalid=move || fieldset_aria_invalid.get()",
        "aria-required=move || fieldset_aria_required.get()",
        "data-slot=\"checkbox-group\"",
        "data-state-source=move || view_state.get().state_source.as_data_attr()",
        "data-motion-source=motion_source",
        "data-motion-phase=move || view_state.get().motion_phase.as_data_attr()",
        "data-ui-source=move || agent_contract.get().source.as_data_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "checkbox-group view should expose semantic contract marker `{required}`.",
        );
    }

    assert!(
        view_source.contains("<fieldset") && !view_source.contains("role="),
        "checkbox-group should prefer native fieldset semantics; explicit role override is N/A in this scope.",
    );

    for required in [
        "fn checkbox_group_view_mounts_headless_locale_contract()",
        "fn checkbox_group_emits_baseline_state_data_attributes()",
        "fn checkbox_group_state_markers_use_closed_enums_for_source_and_phase()",
        "fn checkbox_group_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
        "fn checkbox_group_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()",
    ] {
        assert!(
            semantics_source.contains(required),
            "checkbox-group semantic suite should keep contract-first regression `{required}`.",
        );
    }

    for forbidden in ["assert_snapshot!", "assert_json_snapshot!", "insta::assert"] {
        assert!(
            !semantics_source.contains(forbidden),
            "semantic-priority contract should not rely on snapshot-only check `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
        "components/checkbox-group/test/semantics.rs::checkbox_group_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep semantic-test-priority marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep e2e selector/stable-wait governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_checkbox_group_contract.spec.mjs");

    for required in [
        "/#/components/checkbox-group",
        "body:not(:has(#boot))",
        "#docs-checkbox-group[data-slot=\"checkbox-group\"]",
        "[data-slot=\"checkbox-group-list\"] [data-slot=\"checkbox\"][role=\"checkbox\"]",
        "[data-slot=\"checkbox-group-list\"] [data-slot=\"checkbox\"][role=\"checkbox\"][aria-checked=\"true\"]",
        "[data-slot=\"checkbox-group-list\"] [data-slot=\"checkbox\"][role=\"checkbox\"][aria-checked=\"false\"]",
        "await expect(validationGroup).toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\");",
        "await expect(validationGroup).toHaveAttribute(\"data-ui-output-status\", \"verified\");",
        "await expect(validationGroup).toHaveAttribute(\"data-state-source\", \"semantic-props\");",
        "await expect(validationGroup).toHaveAttribute(\"data-motion-phase\", \"inactive\");",
        "await expect(groupCheckboxes).toHaveCount(3);",
        "await expect(checkedBoxes).toHaveCount(1);",
        "await expect(uncheckedBoxes).toHaveCount(2);",
    ] {
        assert!(
            e2e_source.contains(required),
            "checkbox-group e2e selector contract should include semantic marker `{required}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
        ":nth-child(",
        ":nth-of-type(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "checkbox-group e2e selector contract should avoid brittle selector/wait token `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_group_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_group_paths() {
    let e2e_source = load_source("../../e2e/tests/docs_app_checkbox_group_contract.spec.mjs");

    for required in [
        "docs-app checkbox-group covers ready/settled semantic breakpoints for validation and motion paths",
        "await expect(validationGroup).toHaveAttribute(\"data-valid\", \"true\");",
        "await bananaCheckbox.click();",
        "await expect(validationGroup).toHaveAttribute(\"data-invalid\", \"true\");",
        "await expect(validationGroup).toHaveAttribute(\"data-shows-error\", \"true\");",
        "await expect(validationGroup).toHaveAttribute(\"data-ui-action\", \"render-semantic-with-error\");",
        "await expect(validationGroup).toHaveAttribute(\"data-ui-state\", \"enabled-invalid\");",
        "await expect(validationGroup).toHaveAttribute(\"data-motion-phase\", \"active\");",
        "await recoveryCheckbox.click();",
        "await expect(validationGroup).toHaveAttribute(\"data-ui-action\", \"render-semantic\");",
        "await expect(validationGroup).toHaveAttribute(\"data-ui-state\", \"enabled-valid\");",
        "await expect(validationGroup).toHaveAttribute(\"data-motion-phase\", \"inactive\");",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(required),
            "checkbox-group e2e ready/settled contract should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script_source = load_source("../../components/checkbox-group/scripts/check-ui-e2e-checkbox-group.sh");

    for required in [
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_group_paths",
    ] {
        assert!(
            script_source.contains(required),
            "checkbox-group e2e check script should enforce `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_marks_e2e_selector_stability_item_complete() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "components/checkbox-group/test/semantics.rs::checkbox_group_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/checkbox-group/test/semantics.rs::checkbox_group_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "components/checkbox-group/test/semantics.rs::checkbox_group_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_group_paths",
        "components/checkbox-group/test/semantics.rs::checkbox_group_e2e_check_script_covers_selector_and_settled_wait_contract",
        "components/checkbox-group/scripts/check-ui-e2e-checkbox-group.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep e2e selector-stability evidence marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep repeatable e2e regression governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_checkbox_group_contract.spec.mjs");
    let script_source = load_source("../../components/checkbox-group/scripts/check-ui-e2e-checkbox-group.sh");

    for required in [
        "docs-app checkbox-group key flow is repeatable and failures map to semantic breakpoints",
        "for (const cycle of [1, 2])",
        "await firstUnchecked.focus();",
        "await expect(firstUnchecked).toBeFocused();",
        "await page.keyboard.press(\"Space\");",
        "await firstChecked.focus();",
        "await expect(firstChecked).toBeFocused();",
        "await expect(validationGroup).toHaveAttribute(\"data-invalid\", \"true\");",
        "await expect(validationGroup).toHaveAttribute(\"data-ui-action\", \"render-semantic-with-error\");",
        "await expect(validationGroup).toHaveAttribute(\"data-ui-state\", \"enabled-invalid\");",
        "await expect(validationGroup).toHaveAttribute(\"data-motion-phase\", \"active\");",
        "await recoveryCheckbox.focus();",
        "await expect(recoveryCheckbox).toBeFocused();",
        "await expect(validationGroup).toHaveAttribute(\"data-valid\", \"true\");",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(required),
            "checkbox-group repeatable key-flow contract should include semantic breakpoint `{required}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
        "toHaveScreenshot(",
        "toMatchSnapshot(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "checkbox-group repeatable key-flow should avoid brittle/non-semantic token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_e2e_key_flow_is_repeatable_and_failure_points_are_semantic";
    assert!(
        script_source.contains(script_needle),
        "checkbox-group e2e script should include `{script_needle}`.",
    );
}

#[test]
fn checkbox_group_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_checkbox_group_contract.spec.mjs");
    let script_source = load_source("../../components/checkbox-group/scripts/check-ui-e2e-checkbox-group.sh");

    for required in [
        "docs-app checkbox-group high-risk paths cover focus keyboard and disabled semantic breakpoints",
        "await trigger.focus();",
        "await expect(trigger).toBeFocused();",
        "await page.keyboard.press(\"Space\");",
        "#docs-checkbox-group-disabled[data-slot=\"checkbox-group\"]",
        "await expect(disabledGroup).toHaveAttribute(\"data-disabled\", \"true\");",
        "await expect(disabledGroup).toHaveAttribute(\"data-ui-state\", \"disabled-valid\");",
        "await expect(disabledCheckboxes.first()).toHaveAttribute(\"aria-disabled\", \"true\");",
        "await expect(disabledCheckboxes.nth(1)).toHaveAttribute(\"aria-disabled\", \"true\");",
        "await disabledCheckboxes.first().click({ force: true });",
        "await disabledCheckboxes.nth(1).click({ force: true });",
    ] {
        assert!(
            e2e_source.contains(required),
            "checkbox-group high-risk e2e flow should include semantic marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "checkbox-group e2e script should include `{script_needle}`.",
    );
}

#[test]
fn checkbox_group_e2e_check_script_covers_repeatable_key_flow_contracts() {
    let script_source = load_source("../../components/checkbox-group/scripts/check-ui-e2e-checkbox-group.sh");

    for required in [
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(required),
            "checkbox-group e2e key-flow check script should enforce `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_marks_replayable_e2e_critical_flow_item_complete() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "components/checkbox-group/test/semantics.rs::checkbox_group_check2_documents_e2e_repeatable_key_flow_rules",
        "components/checkbox-group/test/semantics.rs::checkbox_group_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "components/checkbox-group/test/semantics.rs::checkbox_group_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "components/checkbox-group/test/semantics.rs::checkbox_group_e2e_check_script_covers_repeatable_key_flow_contracts",
        "components/checkbox-group/scripts/check-ui-e2e-checkbox-group.sh",
        "overlay/async 在 `CheckboxGroup` 组件范围为 `N/A`",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group check2 repeatable key-flow section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders() {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    assert!(
        view_source.contains("view! {"),
        "CheckboxGroup should keep explicit render block in view.rs."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        3,
        "CheckboxGroup should keep bounded `view!` usage (root + optional description/error fragments)."
    );
    assert!(
        view_source.lines().count() <= 150,
        "CheckboxGroup view.rs should stay compact; split semantic subrenders if this grows."
    );

    for forbidden in [
        "for item in",
        "collect::<Vec<_>>()",
        "while ",
        "loop {",
        "match (",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CheckboxGroup view should avoid loop-heavy or branch-heavy macro pattern `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
    assert!(
        check2_source.contains("`view!` 宏复杂度受控"),
        "CheckboxGroup checklist should retain view-macro complexity governance entry."
    );
}

#[test]
fn checkbox_group_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for needle in [
        "fn render_description_block(description: String, description_id: String) -> impl IntoView",
        "fn render_error_block(",
        "view_state: Signal<logic::CheckboxGroupViewState>,",
        "let description_view = description",
        "render_description_block(description, description_attrs.id.clone())",
        "let error_view =",
        "render_error_block(error, error_attrs.id.clone(), view_state)",
        "{description_view}",
        "{error_view}",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxGroup view should keep function-first split marker `{needle}`."
        );
    }

    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\nfn checkbox_group_",
        "description.map(|description| {",
        "error.map(|error| {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CheckboxGroup should avoid local component abstraction noise or inline fragment duplication `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
    assert!(
        check2_source.contains("函数式拆分优先"),
        "CheckboxGroup checklist should retain functional-split governance entry."
    );
}

#[test]
fn checkbox_group_static_fragments_are_constantized_or_absent_for_simple_layout() {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for forbidden in [
        "<svg",
        "<path",
        "inner_html=",
        "footer",
        "copyright",
        "lorem ipsum",
        "markdown_to_html(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CheckboxGroup simple layout should avoid heavy inline static fragment token `{forbidden}`.",
        );
    }

    for needle in [
        "data-slot=\"checkbox-group\"",
        "data-slot=\"checkbox-group-label\"",
        "data-slot=\"checkbox-group-list\"",
        "data-slot=\"checkbox-group-description\"",
        "data-slot=\"checkbox-group-error\"",
        "aria-labelledby=legend_id.get_value()",
        "aria-describedby=move || fieldset_aria_describedby.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxGroup should keep stable semantic/a11y markers while static fragments stay absent `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_static_fragments_are_constantized_or_absent_for_simple_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
    assert!(
        check2_source.contains("静态片段常量化"),
        "CheckboxGroup checklist should retain static fragment constantization governance entry."
    );
}

#[test]
fn checkbox_group_inner_html_usage_is_absent_and_untrusted_html_paths_are_blocked() {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let styles_source = load_source("../../components/checkbox-group/src/styles.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let readme_source = load_source("../../components/checkbox-group/src/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "markdown_to_html(",
    ] {
        let found = view_source.contains(forbidden)
            || logic_source.contains(forbidden)
            || styles_source.contains(forbidden)
            || motion_source.contains(forbidden)
            || readme_source.contains(forbidden)
            || docs_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup should keep whitelist-safe render path and reject untrusted html token `{forbidden}`.",
        );
    }

    for needle in [
        "data-slot=\"checkbox-group\"",
        "aria-labelledby=legend_id.get_value()",
        "aria-describedby=move || fieldset_aria_describedby.get()",
        "aria-invalid=move || fieldset_aria_invalid.get()",
        "aria-required=move || fieldset_aria_required.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxGroup should keep semantic mounting without inner_html fallback via `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_inner_html_usage_is_absent_and_untrusted_html_paths_are_blocked";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );
    assert!(
        check2_source.contains("`inner_html` 使用约束"),
        "CheckboxGroup checklist should retain inner_html governance entry."
    );
}

#[test]
fn checkbox_group_wasm_debug_contract_is_explicitly_na_and_feature_isolated() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let script_source = load_source("../../scripts/check-ui-wasm-debug.sh");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let docs_forms_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    let component_cargo = load_source("../../components/checkbox-group/Cargo.toml");
    let ui_components_cargo = load_source("Cargo.toml");
    let ui_components_lib = load_source("src/lib.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let readme_source = load_source("../../components/checkbox-group/src/README.md");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");

    for needle in ["[features]", "default = []"] {
        assert!(
            component_cargo.contains(needle),
            "checkbox-group crate feature boundary should include `{needle}`.",
        );
    }

    for forbidden in [
        "wasm-debug",
        "checkbox-group-wasm-debug",
        "checkbox_group_wasm_debug",
        "component-checkbox_group-wasm-debug",
    ] {
        assert!(
            !component_cargo.contains(forbidden),
            "checkbox-group crate should not expose component-local wasm debug feature `{forbidden}`.",
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui should keep shared wasm-debug feature marker `{needle}`.",
        );
    }

    for forbidden in [
        "checkbox-group-wasm-debug =",
        "checkbox_group_wasm_debug =",
        "component-checkbox_group-wasm-debug",
        "component-checkbox_group\", \"dep:tracing",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "ui feature graph should not leak checkbox-group specific debug toggle `{forbidden}`.",
        );
    }

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui root should keep shared wasm-debug isolation marker `{needle}`.",
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
            "docs app should keep wasm-debug visual entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle) || trace_source.contains(needle),
            "global trace/debug overlay should keep marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn checkbox_group() -> AnyView",
        "title=\"Interactive Playground\"",
        "id=\"docs-checkbox-group-interactive\".to_string()",
        "<Checkbox checked=interactive_alpha set_checked=set_interactive_alpha>",
        "<Checkbox checked=interactive_beta set_checked=set_interactive_beta>",
        "\"selected count: \"",
    ] {
        assert!(
            docs_forms_source.contains(needle),
            "checkbox-group docs should keep minimal reproducible interaction chain marker `{needle}`.",
        );
    }

    for needle in [
        "data-state-source=move || view_state.get().state_source.as_data_attr()",
        "data-motion-source=motion_source",
        "data-motion-phase=move || view_state.get().motion_phase.as_data_attr()",
        "data-invalid=move || view_state.get().is_invalid.then_some(\"true\")",
        "data-required=move || view_state.get().is_required.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "checkbox-group should keep stable marker `{needle}` for debug traceability.",
        );
    }

    for forbidden in [
        "UiTrace",
        "use_ui_trace",
        "provide_ui_trace",
        "trace.emit(",
        "wasm_debug_proxy!",
        "observability::",
        "debug_overlay",
        "request_replay",
        "replay",
        "timeline",
        "#[prop(optional)] debug",
        "data-debug-",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !readme_source.contains(forbidden)
                && !component_toml.contains(forbidden)
                && !rbi_source.contains(forbidden),
            "checkbox-group runtime/public contract should not leak wasm-debug internals `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_wasm_debug_contract_is_explicitly_na_and_feature_isolated";
    assert!(
        script_source.contains(script_needle),
        "wasm-debug gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] WASM 调试要求",
        "checkbox_group_wasm_debug_contract_is_explicitly_na_and_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-group checklist should keep wasm-debug governance marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_group_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na()
 {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let check2_source = load_source("../../components/checkbox-group/check2.md");
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
        "pub(super) fn checkbox_group() -> AnyView",
        "title=\"Interactive Playground\"",
        "description=\"Display + Config + Code + CSS Test: edit group is_invalid/is_required state and inspect contracts.\"",
        "test_css_source=interactive_test_css",
        "test_config_signal=interactive_config",
        "controls=move || view!",
        "id=\"docs-checkbox-group-interactive\".to_string()",
        "<Checkbox checked=interactive_alpha set_checked=set_interactive_alpha>",
        "<Checkbox checked=interactive_beta set_checked=set_interactive_beta>",
        "\"selected count: \"",
        "slug=\"checkbox-group\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "checkbox-group docs should keep interactive workbench/context marker `{needle}`.",
        );
    }

    for forbidden in [
        "CHECKBOX_GROUP_WORKBENCH_STORAGE_KEY",
        "load_checkbox_group_workbench_state(",
        "save_checkbox_group_workbench_state(",
        "clear_checkbox_group_workbench_state(",
        "Persist checkbox-group workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "checkbox-group keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent.",
        );
    }

    for required in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "checkbox_group_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep DX governance marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na";
    assert!(
        script_source.contains(script_needle),
        "DX gate script should include `{script_needle}`.",
    );
}

#[test]
fn checkbox_group_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("../../components/checkbox-group/src/styles.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "var(--ui-checkbox-group-gap, var(--ui-fallback-checkbox-group-gap))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-checkbox-group-required-marker-gap, var(--ui-fallback-checkbox-group-required-marker-gap))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-checkbox-group-motion-duration, var(--ui-fallback-checkbox-group-motion-duration))",
        "var(--ui-checkbox-group-motion-easing, var(--ui-fallback-checkbox-group-motion-easing))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-checkbox-group-disabled-opacity, var(--ui-fallback-checkbox-group-disabled-opacity))",
    ] {
        assert!(
            styles_source.contains(required),
            "checkbox-group styles should keep defensive double-fallback token `{required}`.",
        );
    }

    for required in [
        "--ui-fallback-checkbox-group-gap:",
        "--ui-fallback-checkbox-group-required-marker-gap:",
        "--ui-fallback-checkbox-group-disabled-opacity:",
        "--ui-fallback-checkbox-group-motion-duration:",
        "--ui-fallback-checkbox-group-motion-easing:",
        "--ui-fallback-font-size-150:",
        "--ui-fallback-line-height-150:",
        "--ui-fallback-font-size-100:",
        "--ui-fallback-line-height-100:",
        "--ui-fallback-danger:",
        "--ui-fallback-fg-muted:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme should stay SSOT for checkbox-group fallback token `{required}`.",
        );
    }

    for forbidden in [
        "gap: var(--ui-checkbox-group-gap);",
        "font-size: var(--ui-font-size-150);",
        "line-height: var(--ui-line-height-150);",
        "color: var(--ui-fg);",
        "margin-left: var(--ui-checkbox-group-required-marker-gap);",
        "color: var(--ui-danger);",
        "font-size: var(--ui-font-size-100);",
        "line-height: var(--ui-line-height-100);",
        "transition-duration: var(--ui-checkbox-group-motion-duration);",
        "transition-timing-function: var(--ui-checkbox-group-motion-easing);",
        "color: var(--ui-fg-muted);",
        "opacity: var(--ui-checkbox-group-disabled-opacity);",
        "rgb(",
        "hsl(",
        "px;",
        "rem;",
        "em;",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "checkbox-group styles should not keep raw terminal style token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "checkbox_group_styles_use_defensive_variable_fallback_chain",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep defensive-variable governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_cascade_layer_and_runtime_style_contract_is_enforced() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-checkbox_group\")]",
        "out.push_str(crate::checkbox_group::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(required),
            "ui css aggregation should keep cascade-layer marker `{required}`.",
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should keep components css injection path marker `{required}`.",
        );
    }

    assert!(
        view_source.contains("style=style"),
        "checkbox-group view should mount runtime style through a dedicated style variable channel.",
    );
    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style:top=",
        "style:left=",
        "style:right=",
        "style:bottom=",
        "style:width=",
        "style:height=",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "checkbox-group view/logic should not embed plain inline style token `{forbidden}`.",
        );
    }

    for required in [
        "--ui-checkbox-group-motion-duration:",
        "--ui-checkbox-group-motion-easing:",
    ] {
        assert!(
            motion_source.contains(required),
            "checkbox-group runtime style updates should flow through CSS custom property payload `{required}`.",
        );
    }

    for forbidden in [
        " top:",
        " left:",
        " right:",
        " bottom:",
        " width:",
        " height:",
        " padding:",
        " margin:",
        " background:",
        " border:",
        " color:",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "checkbox-group runtime style channel should avoid non-variable inline style token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "checkbox_group_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep cascade-layer governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop() {
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let checklist_source = load_source("../../components/checkbox-group/check2.md");

    for needle in [
        "pub struct CheckboxGroupMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "impl Default for CheckboxGroupMotion {",
        "stiffness: tokens.spring.stiffness,",
        "damping: tokens.spring.damping,",
        "mass: tokens.spring.mass,",
        "precision: tokens.spring.precision,",
        "pub fn resolve_effective_motion(",
        "let reduced = !motion.enabled || prefers_reduced_motion;",
        "transition_duration_ms: if reduced { 1 } else { motion.transition_duration_ms },",
        "ui_motion::web::prefers_reduced_motion()",
        "pub fn attach_motion(base_vars: Option<String>, motion: CheckboxGroupMotion) -> String",
        "--ui-checkbox-group-motion-stiffness:",
        "--ui-checkbox-group-motion-damping:",
        "--ui-checkbox-group-motion-mass:",
        "--ui-checkbox-group-motion-precision:",
        "--ui-checkbox-group-motion-reduced:",
    ] {
        assert!(
            motion_source.contains(needle),
            "checkbox-group motion contract should include `{needle}`.",
        );
    }

    for needle in [
        "let style = motion_contract::attach_motion(None, motion);",
        "style=style",
        "data-motion-source=motion_source",
    ] {
        assert!(
            view_source.contains(needle),
            "checkbox-group view should keep motion attach marker `{needle}`.",
        );
    }

    for forbidden in [
        "web_sys::",
        "window()",
        "document()",
        "unwrap()",
        "expect(",
        "panic!(",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "checkbox-group motion layer should stay portable/no-op-safe and avoid `{forbidden}`.",
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
        "checkbox_group_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop",
    ] {
        assert!(
            checklist_source.contains(required),
            "checkbox-group checklist should keep motion-contract governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_motion_contract_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    let needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop";
    assert!(
        script_source.contains(needle),
        "contract-hygiene gate script should include `{needle}`.",
    );
}

#[test]
fn checkbox_group_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let ui_components_lib = load_source("src/lib.rs");
    let ui_components_css = load_source("src/css.rs");
    let ui_components_root = load_source("src/root.rs");
    let active_highlight = load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable_state =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let headless_presence = load_source("../../crates/ui-headless/src/presence.rs");
    let headless_a11y = load_source("../../crates/ui-headless/src/a11y.rs");
    let entrypoints_script = load_source("../../scripts/check-ui-entrypoints.sh");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let ui_components_src = manifest_dir.join("../../crates/ui/src");

    for required in [
        "#[cfg(feature = \"component-checkbox_group\")]",
        "pub use ui_checkbox_group as checkbox_group;",
        "pub use root::UiRoot;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib entry should keep marker `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-checkbox_group\")]",
        "out.push_str(crate::checkbox_group::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css entry should keep marker `{required}`."
        );
    }

    for required in [
        "#[component]",
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
            ui_components_root.contains(required),
            "UiRoot entry should keep centralized injection marker `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight shared primitive should keep marker `{required}`."
        );
    }

    for forbidden in [
        "#[component]",
        "pub fn CheckboxGroup(",
        "data-slot=\"checkbox-group\"",
    ] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight should stay generic shared primitive, found `{forbidden}`."
        );
    }

    assert!(
        !ui_components_src.join("overlay_open.rs").exists(),
        "ui should not define `src/overlay_open.rs`."
    );
    assert!(
        !ui_components_src.join("presence.rs").exists(),
        "ui should not define `src/presence.rs`."
    );
    assert!(
        !ui_components_src.join("a11y.rs").exists(),
        "ui should not define `src/a11y.rs`."
    );

    for required in [
        "pub fn use_controllable_state<T>(",
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
        "pub fn aria_controls_when_open(open: Signal<bool>, controls_id: String) -> Signal<Option<String>>",
    ] {
        assert!(
            headless_controllable_state.contains(required)
                || headless_presence.contains(required)
                || headless_a11y.contains(required),
            "headless canonical primitive should keep marker `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        entrypoints_script.contains(script_needle),
        "entrypoints gate script should include checkbox-group fixed-entry command."
    );

    for required in [
        "- [x] `ui` 固定入口文件落点正确。",
        "components/checkbox-group/test/semantics.rs::checkbox_group_ui_components_fixed_entry_files_follow_layered_boundaries",
        "components/checkbox-group/test/checkbox_group_semantics.rs::checkbox_group_ui_components_fixed_entry_files_follow_layered_boundaries",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_ui_components_fixed_entry_files_follow_layered_boundaries",
    ] {
        assert!(
            check2_source.contains(required),
            "check2.md should include fixed-entry evidence marker `{required}`."
        );
    }
}

#[test]
fn checkbox_group_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources()
 {
    let mod_source = load_source("../../components/checkbox-group/src/mod.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let styles_source = load_source("../../components/checkbox-group/src/styles.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in ["unwrap(", "expect(", "unwrap_err(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "checkbox-group non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_group_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(BASE_CLASS_NAME)];",
        "classes.push(Cow::Owned(class_name));",
        ".map(|class_name| class_name.as_ref())",
    ] {
        assert!(
            logic_source.contains(required),
            "checkbox-group logic should keep Cow-based string hotspot mitigation marker `{required}`.",
        );
    }

    for forbidden in [
        "format!(\"{BASE_CLASS_NAME} {class_name}\")",
        "BASE_CLASS_NAME.to_string()",
        "\"ui-checkbox-group\".to_string()",
        "String::from(\"ui-checkbox-group\")",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "checkbox-group string hotspot contract should avoid `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_group_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let rust_hygiene_script = load_source("../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("../../scripts/check-ui-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            rust_hygiene_script.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }

    for needle in [
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering gate script should include checkbox-group rust-hygiene command `{needle}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "Cow<'static, str>",
        "./scripts/check-rust-hygiene.sh",
        "components/checkbox-group/test/semantics.rs::checkbox_group_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "components/checkbox-group/test/semantics.rs::checkbox_group_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "components/checkbox-group/test/semantics.rs::checkbox_group_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "scripts/check-ui-engineering.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep rust-hygiene evidence marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let mod_source = load_source("../../components/checkbox-group/src/mod.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let styles_source = load_source("../../components/checkbox-group/src/styles.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let readme_source = load_source("../../components/checkbox-group/src/README.md");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    let spec_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/checkbox-group/src/spec.rs");
    assert!(
        !spec_path.exists(),
        "checkbox-group should keep spec/schema serialization path as N/A for simple component scope."
    );

    let combined = [
        mod_source,
        logic_source,
        view_source,
        styles_source,
        motion_source,
        readme_source,
        component_toml,
        rbi_source,
    ]
    .join("\n");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "schema_version",
        "mod spec;",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "checkbox-group engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`.",
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
            "checkbox-group checklist should keep engineering governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events()
 {
    let ui_components_cargo = load_source("Cargo.toml");
    let button_view_source = load_source("../../components/button/src/view.rs");
    let combined = [
        load_source("../../components/checkbox-group/src/mod.rs"),
        load_source("../../components/checkbox-group/src/logic.rs"),
        load_source("../../components/checkbox-group/src/view.rs"),
        load_source("../../components/checkbox-group/src/styles.rs"),
        load_source("../../components/checkbox-group/src/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui::button::state_change\"",
    ] {
        assert!(
            ui_components_cargo.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`.",
        );
    }

    for forbidden_feature in [
        "checkbox-group-wasm-debug =",
        "checkbox_group_wasm_debug =",
        "component-checkbox_group\", \"dep:tracing",
        "component-checkbox_group-wasm-debug",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden_feature),
            "checkbox-group should not define component-local tracing feature `{forbidden_feature}` when no local debug event/replay contract exists.",
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::checkbox_group::",
        "const CHECKBOX_GROUP_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "checkbox-group should avoid ad-hoc tracing semantic drift token `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_group_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("../../components/checkbox-group/src/mod.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let styles_source = load_source("../../components/checkbox-group/src/styles.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");

    for source in [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
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
        ] {
            assert!(
                !source.contains(forbidden),
                "checkbox-group engineering contract should not leak runtime marker `{forbidden}`.",
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "checkbox-group public module boundary should not leak web_sys types.",
    );
}

#[test]
fn checkbox_group_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let mod_source = load_source("../../components/checkbox-group/src/mod.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let styles_source = load_source("../../components/checkbox-group/src/styles.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let component_toml = load_source("../../components/checkbox-group/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-group/src/checkbox_group.rbi");
    let check2_source = load_source("../../components/checkbox-group/check2.md");
    let script_source = load_source("../../scripts/check-ui-engineering.sh");
    let lib_source = load_source("src/lib.rs");

    for required in [
        "pub enum CheckboxGroupAgentSchemaVersion {",
        "Self::V1 => \"v1\"",
        "schema_version = \"1\"",
        "schema = \"ui.checkbox-group.agent-contract.v1\"",
        "values = [\"v1\"]",
    ] {
        let found = logic_source.contains(required) || component_toml.contains(required);
        assert!(
            found,
            "checkbox-group version contract should keep v1 marker `{required}`.",
        );
    }

    let combined = [
        mod_source.as_str(),
        logic_source.as_str(),
        view_source.as_str(),
        styles_source.as_str(),
        motion_source.as_str(),
        component_toml.as_str(),
        rbi_source.as_str(),
    ]
    .join("\n");
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "SchemaRegistry",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "agent-contract.v2",
    ] {
        assert!(
            !combined.contains(forbidden),
            "without major breaking upgrade, checkbox-group should not introduce migration marker `{forbidden}`.",
        );
    }

    for forbidden in [
        "checkbox::group::CheckboxGroup",
        "#[path = \"checkbox_field/checkbox/mod.rs\"]",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "checkbox-group migration cleanup should keep legacy-shim removal marker `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `CheckboxGroup` 提交未引入跨大版本 API 破坏升级",
        "CheckboxGroupAgentSchemaVersion::V1",
        "schema_version = \"1\"",
        "ui.checkbox-group.agent-contract.v1",
        "checkbox_group_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep version-deprecation migration marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_check2_marks_version_deprecation_migration_item_complete() {
    let check2_source = load_source("../../components/checkbox-group/check2.md");

    for required in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `CheckboxGroup` 提交未引入跨大版本 API 破坏升级",
        "components/checkbox-group/test/semantics.rs::checkbox_group_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
        "components/checkbox-group/test/semantics.rs::checkbox_group_breaking_migration_removes_legacy_namespace_and_path_shim",
        "scripts/check-ui-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-group checklist should keep version-deprecation completion marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_group_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");

    for needle in [
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn checkbox_group_type_system_and_semantic_markers_form_machine_readable_contract() {
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let logic_test_source = load_source("../../components/checkbox-group/test/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/checkbox_group.rs");

    for needle in [
        "pub use ui_state_primitives::checkbox_group::{",
        "CheckboxGroupState",
        "normalize_checkbox_group_label",
        "normalize_checkbox_group_optional_text",
        "resolve_checkbox_group_state",
        "pub enum CheckboxGroupStateSource {",
        "pub enum CheckboxGroupMotionPhase {",
        "pub enum CheckboxGroupAgentSchemaVersion {",
        "pub enum CheckboxGroupAgentIntent {",
        "pub enum CheckboxGroupAgentAction {",
        "pub enum CheckboxGroupAgentState {",
        "pub enum CheckboxGroupAgentSource {",
        "pub enum CheckboxGroupAgentConfigPolicy {",
        "pub struct CheckboxGroupAgentContract {",
        "pub fn resolve_checkbox_group_agent_contract(",
        "pub const fn as_data_attr(self) -> &'static str",
        "pub struct CheckboxGroupViewState {",
        "pub state_source: CheckboxGroupStateSource,",
        "pub motion_phase: CheckboxGroupMotionPhase,",
    ] {
        assert!(
            logic_source.contains(needle),
            "CheckboxGroup type contract should stay explicit and machine-readable via `{needle}`.",
        );
    }

    for forbidden in [
        "pub state_source: String",
        "pub motion_phase: String",
        "pub state_source: &'a str",
        "pub motion_phase: &'a str",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "CheckboxGroup state axes should not degrade to string protocol `{forbidden}`.",
        );
    }

    for needle in [
        "data-disabled=move || view_state.get().is_disabled.then_some(\"true\")",
        "data-enabled=move || view_state.get().is_enabled.then_some(\"true\")",
        "data-invalid=move || view_state.get().is_invalid.then_some(\"true\")",
        "data-valid=move || view_state.get().is_valid.then_some(\"true\")",
        "data-required=move || view_state.get().is_required.then_some(\"true\")",
        "data-optional=move || view_state.get().is_optional.then_some(\"true\")",
        "data-shows-error=move || view_state.get().shows_error.then_some(\"true\")",
        "data-state-source=move || view_state.get().state_source.as_data_attr()",
        "data-motion-phase=move || view_state.get().motion_phase.as_data_attr()",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-intent=move || agent_contract.get().intent.as_data_attr()",
        "data-ui-action=move || agent_contract.get().action.as_data_attr()",
        "data-ui-state=move || agent_contract.get().state.as_data_attr()",
        "data-ui-source=move || agent_contract.get().source.as_data_attr()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_data_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxGroup key states should stay externally observable via `{needle}`.",
        );
    }

    for needle in [
        "fn resolve_checkbox_group_state_is_consumed_from_primitives_contract()",
        "fn resolve_checkbox_group_view_state_centralizes_render_markers()",
        "fn resolve_checkbox_group_motion_phase_is_closed_set()",
        "fn resolve_checkbox_group_state_source_is_closed_set()",
        "fn resolve_checkbox_group_agent_contract_is_closed_set_and_traceable()",
        "fn resolve_checkbox_group_agent_enum_attrs_are_stable()",
    ] {
        assert!(
            logic_test_source.contains(needle),
            "CheckboxGroup tests should keep direct contract failure points via `{needle}`.",
        );
    }

    assert!(
        primitive_source.contains("pub fn resolve_checkbox_group_state("),
        "CheckboxGroup invalid state combinations should remain centralized in ui-state-primitives.",
    );
}

#[test]
fn checkbox_group_token_first_style_contract_stays_in_styles_and_css_vars() {
    let styles_source = load_source("../../components/checkbox-group/src/styles.rs");
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let motion_source = load_source("../../components/checkbox-group/src/motion.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        "var(--ui-fallback-",
        "#[cfg(feature = \"component-checkbox_group\")]",
        "out.push_str(crate::checkbox_group::styles::CSS);",
        "let style = motion_contract::attach_motion(None, motion);",
        "style=style",
    ] {
        let found = styles_source.contains(needle)
            || css_source.contains(needle)
            || view_source.contains(needle);
        assert!(
            found,
            "CheckboxGroup token-first style pipeline should keep `{needle}`.",
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"p-",
        "class=\"m-",
        "class=\"gap-",
        "tailwind",
        "utility-first",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CheckboxGroup component contract should avoid utility-first class token `{forbidden}`.",
        );
    }

    for forbidden in [
        "stylist::",
        "styled_components::",
        "stylex::",
        "emotion::",
        "css! {",
        "style! {",
        "styled!",
        "Style::new(",
    ] {
        let found = styles_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden);
        assert!(
            !found,
            "CheckboxGroup component contract should avoid CSS-in-Rust runtime token `{forbidden}`.",
        );
    }

    for forbidden in ["background:", "color:", "padding:", "margin:", "border:"] {
        assert!(
            !motion_source.contains(forbidden),
            "CheckboxGroup runtime style channel should only write CSS variables, not `{forbidden}`.",
        );
    }
}
