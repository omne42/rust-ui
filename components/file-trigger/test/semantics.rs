#[test]
fn component_layer_keeps_structural_responsibilities() {
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let styles_source = include_str!("../src/styles.rs");
    let motion_source = include_str!("../src/motion.rs");

    assert!(
        !mod_source.contains("pub mod logic"),
        "logic module should remain private in component public boundary."
    );
    assert!(
        logic_source.contains("pub use ui_state_primitives::file_trigger::{"),
        "logic.rs should consume state-primitives instead of reimplementing state machines."
    );
    assert!(
        view_source.contains("use_file_trigger(FileTriggerOptions {"),
        "view.rs should mount ui-headless semantics contract."
    );
    assert!(
        view_source.contains("super::logic::resolve_render_state("),
        "view.rs should delegate state derivation to logic.rs."
    );
    assert!(
        view_source.contains("super::logic::FileTriggerRenderStateInput {"),
        "view.rs should pass typed input boundary into logic.rs."
    );
    assert!(
        !view_source.contains("super::logic::resolve_state("),
        "view.rs should not rebuild state machine rules directly."
    );
    assert!(
        !view_source.contains("super::logic::compose_class_name("),
        "view.rs should not derive class markers directly."
    );
    assert!(
        !view_source.contains("let has_custom_motion ="),
        "view.rs should not own intermediate state derivation variables."
    );
    assert!(
        !view_source.contains("unwrap_or(false)"),
        "view.rs should not own default fallback branches."
    );
    assert!(
        logic_source.contains("FileTriggerProps, FileTriggerPropsInput, FileTriggerRenderState,"),
        "logic.rs should re-export primitive normalization types from ui-state-primitives."
    );
    assert!(
        logic_source.contains("FileTriggerRenderStateInput, FileTriggerSelectionMode,"),
        "logic.rs should re-export primitive discrete-mode type from ui-state-primitives."
    );
    assert!(
        !logic_source.contains("pub enum FileTriggerSelectionMode"),
        "logic.rs should not re-implement primitive enums."
    );
    assert!(
        !logic_source.contains("pub fn resolve_props("),
        "logic.rs should not re-implement primitive normalization functions."
    );
    assert!(
        !logic_source.contains("pub fn resolve_render_state("),
        "logic.rs should consume primitive render-state derivation instead of rebuilding it."
    );
    assert!(
        view_source
            .contains("<Button is_disabled=is_disabled motion=motion.trigger on_press=on_press>"),
        "view.rs should compose trigger behavior through child components."
    );
    assert!(
        styles_source.contains("var(--ui-space-sm, var(--ui-fallback-space-sm))"),
        "styles.rs should consume token variables with fallback chain."
    );
    assert!(
        motion_source.contains("trigger: crate::button::motion::sanitize_motion(motion.trigger)"),
        "motion.rs should map component motion to shared motion contract, not implement a driver."
    );
}

#[test]
fn public_api_does_not_expose_dom_types() {
    let mod_source = include_str!("../src/mod.rs");

    for forbidden in ["web_sys", "HtmlInputElement", "NodeRef"] {
        assert!(
            !mod_source.contains(forbidden),
            "component public exports must not leak DOM details: `{forbidden}`"
        );
    }
}

#[test]
fn component_has_no_half_controlled_state_axis() {
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("Controlled/uncontrolled axis:"),
        "README should explicitly document controlled/uncontrolled applicability."
    );
    assert!(
        readme_source.contains("N/A for this component."),
        "README should state why controlled/uncontrolled triad is not applied."
    );
    assert!(
        view_source.contains("on_files: Option<Callback<Vec<FileTriggerFile>>>"),
        "component should expose file selection as event output."
    );
    assert!(
        !view_source.contains("default_files"),
        "component must not expose partial uncontrolled API without controlled counterpart."
    );
    assert!(
        !view_source.contains("on_files_change"),
        "component must not expose partial controlled API aliases."
    );
}

#[test]
fn component_has_no_async_loading_protocol() {
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("Async interaction:"),
        "README should explicitly describe async applicability."
    );
    assert!(
        readme_source.contains("N/A for this component."),
        "README should state async protocol is not applicable."
    );

    for forbidden in ["is_loading", "aria-busy", "on_retry", "use_async_action"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not introduce ad-hoc async loading protocol: `{forbidden}`"
        );
    }
}

#[test]
fn component_dx_default_path_is_minimal_and_docs_first() {
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");
    let docs_source = include_str!("../../../apps/docs-app/src/pages/components/pages/files.rs");

    assert!(
        readme_source.contains("<FileTrigger on_files=on_files>\"Pick files\"</FileTrigger>"),
        "README should provide a one-line default hello-world path."
    );
    assert!(
        readme_source.contains("No manual wiring of `ui-state-primitives` / `ui-headless` is required for baseline usage."),
        "README should explicitly state the default API requires no manual primitive wiring."
    );
    assert!(
        docs_source.contains("title=\"Quick Start (Default API)\""),
        "docs-app should expose a minimal quick-start section for default path."
    );
    assert!(
        docs_source.contains("<FileTrigger on_files=on_quick_start_files>"),
        "docs quick-start should demonstrate default usage without advanced props."
    );
    assert!(
        !view_source.contains("#[prop(optional)] state:"),
        "component should not expose internal state object as required public API."
    );
}

#[test]
fn component_is_not_composite_container_api() {
    let mod_source = include_str!("../src/mod.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");
    let docs_source = include_str!("../../../apps/docs-app/src/pages/components/pages/files.rs");

    assert!(
        readme_source.contains("## Composite API"),
        "README should document composite-API applicability."
    );
    assert!(
        readme_source.contains("N/A for this component."),
        "README should explicitly mark composite API rule as N/A."
    );
    assert!(
        !mod_source.contains("FileTriggerItem"),
        "public API should not expose a fake item subcomponent for non-composite components."
    );
    for forbidden in ["ItemSpec", "labels + children", "titles + panels"] {
        assert!(
            !docs_source.contains(forbidden),
            "docs should not recommend implicit parallel-array composite API: `{forbidden}`"
        );
    }
    assert!(
        !view_source.contains("items_order"),
        "view should not contain collection-registration logic for non-composite components."
    );
}

#[test]
fn component_has_no_dragging_macro_micro_loop() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Macro / Micro State Machine"),
        "README should document macro/micro applicability."
    );
    assert!(
        readme_source.contains("N/A for this component."),
        "README should explicitly mark macro/micro rule as N/A."
    );

    for forbidden in [
        "Dragging",
        "Action::DragEnd",
        "requestAnimationFrame",
        "on:pointermove",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not implement drag physics loop contract: `{forbidden}`"
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not implement drag physics loop contract: `{forbidden}`"
        );
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not implement drag physics loop contract: `{forbidden}`"
        );
    }
}

#[test]
fn component_has_no_two_pass_geometry_pipeline() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Two-Pass Rendering"),
        "README should document two-pass rendering applicability."
    );
    assert!(
        readme_source.contains("N/A for this component."),
        "README should explicitly mark two-pass geometry pipeline as N/A."
    );

    for forbidden in [
        "get_bounding_client_rect",
        "getBoundingClientRect",
        "Intent -> Measure(view) -> Rectification(logic)",
        "Rectification",
        "placement",
        "anchor_rect",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not implement geometry rectification loop: `{forbidden}`"
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not implement measurement-driven placement loop: `{forbidden}`"
        );
    }
}

#[test]
fn component_has_no_registration_protocol() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Registration Protocol"),
        "README should document registration-protocol applicability."
    );
    assert!(
        readme_source.contains("N/A for this component."),
        "README should explicitly mark registration protocol as N/A."
    );

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not implement collection registration protocol: `{forbidden}`"
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not implement collection registration protocol: `{forbidden}`"
        );
    }
}

#[test]
fn component_has_no_slot_projection_strategy() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Slot Projection"),
        "README should document slot-projection applicability."
    );
    assert!(
        readme_source.contains("N/A for this component."),
        "README should explicitly mark slot projection as N/A."
    );

    for forbidden in [
        "KeepAlive",
        "Lazy",
        "Eager",
        "NotifyHidden",
        "slot projection",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not implement slot projection lifecycle contract: `{forbidden}`"
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not implement slot projection lifecycle contract: `{forbidden}`"
        );
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not implement slot projection lifecycle contract: `{forbidden}`"
        );
    }
}

#[test]
fn component_has_no_env_stream_subscription_pipeline() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Env Streams"),
        "README should document env-stream applicability."
    );
    assert!(
        readme_source.contains("N/A for this component."),
        "README should explicitly mark env-stream pipeline as N/A."
    );

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "on:resize",
        "debounce",
        "throttle",
        "ThemeChanged",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not implement env-stream action aggregation: `{forbidden}`"
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not implement env-stream sampling pipeline: `{forbidden}`"
        );
    }
}

#[test]
fn component_has_no_event_light_cone_pipeline() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Event Light Cone"),
        "README should document event-light-cone applicability."
    );
    assert!(
        readme_source.contains("N/A for this component."),
        "README should explicitly mark event light cone as N/A."
    );

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "prop drilling",
        "Table",
        "Grid",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not implement large-collection batch bus protocol: `{forbidden}`"
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not implement large-collection batch bus protocol: `{forbidden}`"
        );
    }
}

#[test]
fn component_has_no_causality_bus_pipeline() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Causality Bus"),
        "README should document causality-bus applicability."
    );
    assert!(
        readme_source.contains("N/A for this component."),
        "README should explicitly mark causality bus as N/A."
    );

    for forbidden in [
        "TraceId",
        "broadcast",
        "subscriber",
        "Causality Bus",
        "event bus",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not implement causality bus broadcast chain: `{forbidden}`"
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not implement causality bus broadcast chain: `{forbidden}`"
        );
    }
}

#[test]
fn component_has_no_overlay_focus_stack_or_private_restore_target() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Focus Stack & GC"),
        "README should document focus-stack applicability."
    );
    assert!(
        readme_source.contains("N/A for this component."),
        "README should explicitly mark focus-stack rule as N/A."
    );

    for forbidden in [
        "FallbackTo",
        "document.body",
        "focus restore",
        "focus stack",
        "overlay focus",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not implement overlay focus-stack policy: `{forbidden}`"
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not implement overlay focus-stack policy: `{forbidden}`"
        );
    }

    assert!(
        view_source.contains("let input_ref: NodeRef<html::Input> = NodeRef::new();"),
        "view.rs should keep NodeRef usage scoped to input click bridge."
    );
    assert!(
        !view_source.contains("fallback_to"),
        "view.rs should not store private overlay restore target hooks."
    );
}

#[test]
fn component_has_no_foreign_zone_escape_hatch_integration() {
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Escape Hatches"),
        "README should document escape-hatch applicability."
    );
    assert!(
        readme_source.contains("N/A for this component."),
        "README should explicitly mark escape-hatch rule as N/A."
    );

    for forbidden in [
        "ECharts",
        "Mapbox",
        "Leaflet",
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not own imperative third-party instance lifecycle: `{forbidden}`"
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not mount imperative third-party instance lifecycle: `{forbidden}`"
        );
        assert!(
            !mod_source.contains(forbidden),
            "public API should not expose imperative third-party instance hooks: `{forbidden}`"
        );
    }
}

#[test]
fn component_keeps_hydration_id_path_deterministic() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Hydration Discontinuity"),
        "README should document hydration-discontinuity contract."
    );
    assert!(
        view_source.contains("#[prop(optional, into)] id: Option<String>"),
        "component should accept external deterministic id input."
    );
    assert!(
        view_source.contains("id=id"),
        "view.rs should pass caller-provided id directly without local regeneration."
    );

    for forbidden in [
        "now()",
        "Date::now",
        "Utc::now",
        "SystemTime::now",
        "Uuid::new_v4",
        "uuid::Uuid",
        "rand::",
        "random(",
        "Math::random",
        "IdProvider",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not introduce hydration-unstable id source: `{forbidden}`"
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not introduce hydration-unstable id source: `{forbidden}`"
        );
    }
}

#[test]
fn component_preserves_ssr_and_cross_platform_boundaries() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## SSR and Cross-Platform"),
        "README should document SSR/cross-platform boundary contract."
    );

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep explicit platform branch declaration: `{required}`"
        );
    }

    for required in [
        "pub(crate) fn collect_files_from_input(",
        "input: &leptos::html::Input,",
        "_input: &leptos::html::Input",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep cross-platform compile-safe input type boundary: `{required}`"
        );
    }

    for forbidden in [
        "_input: &leptos::web_sys::HtmlInputElement",
        "#[cfg(not(target_arch = \"wasm32\"))]\npub(crate) fn collect_files_from_input(\n    _input: &leptos::web_sys::HtmlInputElement,",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "non-wasm branch should not reference web-sys browser types: `{forbidden}`"
        );
    }
}

#[test]
fn component_respects_ui_headless_web_ssr_feature_mutex_contract() {
    let headless_lib_source = include_str!("../../../crates/ui-headless/src/lib.rs");
    let headless_cargo_source = include_str!("../../../crates/ui-headless/Cargo.toml");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");
    let component_cargo_source = include_str!("../Cargo.toml");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(required),
            "ui-headless should enforce feature mutex by compile_error guard: `{required}`"
        );
    }

    for required in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            headless_cargo_source.contains(required),
            "ui-headless feature table should keep explicit web/ssr split: `{required}`"
        );
    }

    assert!(
        view_source
            .contains("use ui_headless::{A11yDirection, FileTriggerOptions, use_file_trigger};"),
        "component should consume ui-headless contract and avoid ad-hoc platform feature mixing."
    );
    assert!(
        !component_cargo_source.contains("ui-headless/ssr")
            && !component_cargo_source.contains("ui-headless/web"),
        "component crate should not override ui-headless feature mutex policy locally."
    );
    assert!(
        readme_source.contains("## ui-headless Feature Exclusivity"),
        "README should document ui-headless feature exclusivity contract."
    );
}

#[test]
fn component_uses_ui_motion_non_wasm_noop_contract() {
    let ui_motion_source = include_str!("../../../crates/ui-motion/src/lib.rs");
    let button_motion_source = include_str!("../../../components/button/src/motion.rs");
    let file_motion_source = include_str!("../src/motion.rs");
    let readme_source = include_str!("../src/README.md");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(required),
            "ui-motion should provide predictable non-wasm no-op stub contract: `{required}`"
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            button_motion_source.contains(required),
            "button motion should keep non-wasm safe-degrade attach path: `{required}`"
        );
    }

    assert!(
        file_motion_source
            .contains("trigger: crate::button::motion::sanitize_motion(motion.trigger)"),
        "file-trigger motion should map to shared motion contract instead of assuming runtime animation handle."
    );
    assert!(
        readme_source.contains("## ui-motion Non-wasm No-op"),
        "README should document ui-motion non-wasm no-op contract."
    );
}

#[test]
fn component_covers_reduced_motion_ssr_and_wasm_without_semantic_split() {
    let button_motion_source = include_str!("../../../components/button/src/motion.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        button_motion_source.contains("if ui_motion::web::prefers_reduced_motion() {"),
        "button motion attach path should skip runtime animation when reduced-motion is requested."
    );

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "let semantics = use_file_trigger(FileTriggerOptions { state, lang, dir });",
        "data-state=data_state_attr",
        "data-disabled=data_disabled_attr",
        "data-enabled=data_enabled_attr",
        "input.click();",
        "std::hint::black_box(is_accept_directory);",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep explicit reduced-motion/SSR/wasm coverage evidence: `{required}`"
        );
    }

    assert!(
        readme_source.contains("## Reduced-motion / SSR / wasm Coverage"),
        "README should document reduced-motion + SSR/wasm coverage contract."
    );
}

#[test]
fn component_has_repeatable_performance_budget_guards() {
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let motion_source = include_str!("../src/motion.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Performance Budget"),
        "README should document component-level performance budget and fallback evidence."
    );

    let effect_count = view_source.matches("Effect::new(").count();
    assert!(
        effect_count <= 1,
        "view.rs should keep at most one wasm-only synchronization effect for predictable update cost (found {effect_count})."
    );

    for forbidden in [
        "requestAnimationFrame",
        "setInterval",
        "setTimeout",
        "ResizeObserver",
        "IntersectionObserver",
        "spawn_local",
        "tokio::spawn",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not introduce high-frequency or background task loops in this component: `{forbidden}`"
        );
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not introduce high-frequency or background task loops in this component: `{forbidden}`"
        );
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not introduce high-frequency or background task loops in this component: `{forbidden}`"
        );
    }
}

#[test]
fn component_view_macro_complexity_is_bounded() {
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count == 1,
        "view.rs should keep exactly one compact `view!` block for this component (found {view_macro_count})."
    );

    for required in [
        "<span",
        "<input",
        "<Button is_disabled=is_disabled motion=motion.trigger on_press=on_press>",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep simple semantic structure marker: `{required}`"
        );
    }

    for forbidden in [
        "<For",
        "<Show",
        "<Suspense",
        "<Portal",
        "repeat(",
        "collect::<Vec",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not introduce heavy repeated/deep macro structures for this simple component: `{forbidden}`"
        );
    }

    assert!(
        readme_source.contains("## view! Macro Complexity"),
        "README should document view-macro complexity baseline."
    );
}

#[test]
fn component_prefers_function_decomposition_over_fragment_components() {
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    let component_attr_count = view_source.matches("#[component]").count();
    assert!(
        component_attr_count == 1,
        "view.rs should keep a single public component boundary and avoid fragment-level component proliferation (found {component_attr_count})."
    );
    assert!(
        view_source.contains("pub fn FileTrigger("),
        "view.rs should keep FileTrigger as the only component boundary for this simple surface."
    );
    assert!(
        readme_source.contains("## Functional Decomposition"),
        "README should document function-first decomposition strategy."
    );
}

#[test]
fn component_avoids_unconstantized_heavy_static_fragments() {
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    for forbidden in [
        "<svg",
        "</svg>",
        "<footer",
        "</footer>",
        "inner_html=",
        "Lorem ipsum",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not embed heavy static fragment directly without constant/template strategy: `{forbidden}`"
        );
    }

    assert!(
        readme_source.contains("## Static Fragment Constantization"),
        "README should document static-fragment constantization strategy and applicability."
    );
}

#[test]
fn component_forbids_untrusted_inner_html_paths() {
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let readme_source = include_str!("../src/README.md");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        ".set_inner_html(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not introduce raw html injection path: `{forbidden}`"
        );
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not introduce raw html injection path: `{forbidden}`"
        );
    }

    for forbidden in ["format!(\"<", "push_str(\"<", "replace(\"<", "concat!(\"<"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not build dynamic html template strings for injection: `{forbidden}`"
        );
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not build dynamic html template strings for injection: `{forbidden}`"
        );
    }

    assert!(
        readme_source.contains("## inner_html Safety"),
        "README should document inner_html safety policy."
    );
}

#[test]
fn component_keeps_wasm_debug_signals_traceable_without_debug_api_pollution() {
    let mod_source = include_str!("../src/mod.rs");
    let view_source = include_str!("../src/view.rs");
    let cargo_source = include_str!("../Cargo.toml");
    let readme_source = include_str!("../src/README.md");

    for required in [
        "data-state=data_state_attr",
        "data-disabled=data_disabled_attr",
        "data-enabled=data_enabled_attr",
        "data-motion-source=state.motion_source_attr",
        "let on_press = Callback::new",
        "let on_change = move |_ev: ev::Event| {",
        "on:change=on_change",
        "if let Some(cb) = on_files.get_value() {",
    ] {
        assert!(
            view_source.contains(required),
            "component should keep stable trace/replay anchors for wasm debugging workflows: `{required}`"
        );
    }

    for forbidden in [
        "feature = \"debug\"",
        "cfg(feature = \"debug\")",
        "debug_",
        "console_log",
        "console::log",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "public component boundary should not expose debug-only API surface: `{forbidden}`"
        );
        assert!(
            !cargo_source.contains(forbidden),
            "component crate features should not include debug-only production-polluting flag: `{forbidden}`"
        );
    }

    assert!(
        readme_source.contains("## WASM Debugability"),
        "README should document wasm debugability boundary and trace surface."
    );
}

#[test]
fn component_mounts_a11y_and_i18n_contract_from_headless() {
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        view_source
            .contains("use ui_headless::{A11yDirection, FileTriggerOptions, use_file_trigger};"),
        "view.rs should use shared headless contract instead of ad-hoc A11y helpers."
    );
    assert!(
        view_source.contains("#[prop(optional, into)] lang: Option<String>"),
        "component should expose lang passthrough for i18n/l10n integration."
    );
    assert!(
        view_source.contains("#[prop(optional)] dir: Option<A11yDirection>"),
        "component should expose dir passthrough for LTR/RTL support."
    );
    assert!(
        view_source
            .contains("let semantics = use_file_trigger(FileTriggerOptions { state, lang, dir });"),
        "view.rs should feed state/lang/dir into headless semantics contract."
    );
    assert!(
        view_source.contains("lang=lang_attr"),
        "view.rs should mount lang from headless attrs."
    );
    assert!(
        view_source.contains("dir=dir_attr"),
        "view.rs should mount dir from headless attrs."
    );
    assert!(
        view_source.contains("tabindex=input_tabindex"),
        "hidden input should mount keyboard accessibility attr from headless contract."
    );
    assert!(
        view_source.contains("aria-hidden=input_aria_hidden"),
        "hidden input should mount aria-hidden from headless contract."
    );
    assert!(
        view_source.contains("children: Children"),
        "visible copy should come from caller-provided children."
    );
    assert!(
        view_source.contains("{children()}"),
        "trigger label should be rendered from external children, not hardcoded text."
    );
    for forbidden in ["\"Pick files\"", "\"Upload\"", "\"Select files\""] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not hardcode user-visible business copy: `{forbidden}`"
        );
    }

    assert!(
        readme_source.contains("## Semantics and Accessibility"),
        "README should document A11y/i18n contract."
    );
}

#[test]
fn component_exposes_observable_and_enumerable_state_markers() {
    let view_source = include_str!("../src/view.rs");
    let primitive_source = include_str!("../../../crates/ui-state-primitives/src/file_trigger.rs");
    let headless_source = include_str!("../../../crates/ui-headless/src/file_trigger.rs");
    let readme_source = include_str!("../src/README.md");

    for required in [
        "data-state=data_state_attr",
        "data-disabled=data_disabled_attr",
        "data-enabled=data_enabled_attr",
        "data-motion-source=state.motion_source_attr",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
        "tabindex=input_tabindex",
        "aria-hidden=input_aria_hidden",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should expose stable observable marker: `{required}`"
        );
    }

    for required in [
        "state_attr: if input.disabled { \"disabled\" } else { \"ready\" },",
        "motion_source_attr: if input.has_custom_motion {",
        "\"custom\"",
        "\"default\"",
    ] {
        assert!(
            primitive_source.contains(required),
            "state-primitives should keep state marker values in a closed set: `{required}`"
        );
    }

    for required in [
        "data_disabled: options.state.is_disabled.then_some(\"true\")",
        "data_enabled: options.state.is_enabled.then_some(\"true\")",
        "input_tabindex: -1",
        "input_aria_hidden: \"true\"",
    ] {
        assert!(
            headless_source.contains(required),
            "headless contract should expose enumerable marker value: `{required}`"
        );
    }

    assert!(
        readme_source.contains("## Observable Markers"),
        "README should document observable marker contract."
    );
}

#[test]
fn type_system_and_semantic_markers_form_machine_readable_contract() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let primitive_source = include_str!("../../../crates/ui-state-primitives/src/file_trigger.rs");
    let readme_source = include_str!("../src/README.md");

    for required in [
        "pub enum FileTriggerSelectionMode {",
        "SingleFile,",
        "MultipleFiles,",
        "Directory,",
        "pub fn resolve_render_state(input: FileTriggerRenderStateInput) -> FileTriggerRenderState",
        "let selection_mode = match (is_accept_directory, is_multiple) {",
        "(true, _) => FileTriggerSelectionMode::Directory,",
        "(false, true) => FileTriggerSelectionMode::MultipleFiles,",
        "(false, false) => FileTriggerSelectionMode::SingleFile,",
    ] {
        assert!(
            primitive_source.contains(required),
            "state-primitives should type and normalize discrete input space: `{required}`"
        );
    }

    for forbidden in [
        "pub enum FileTriggerSelectionMode",
        "pub fn resolve_render_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "component logic should consume primitive contract instead of re-implementing it: `{forbidden}`"
        );
    }

    for required in [
        "data-state=data_state_attr",
        "data-motion-source=state.motion_source_attr",
        "data-disabled=data_disabled_attr",
        "data-enabled=data_enabled_attr",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
        "aria-hidden=input_aria_hidden",
        "tabindex=input_tabindex",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should expose stable machine-readable state marker: `{required}`"
        );
    }

    assert!(
        readme_source.contains("## Type System + Machine-Readable State"),
        "README should document type-level constraints and marker contract."
    );
}

#[test]
fn component_styles_depend_on_explicit_state_markers() {
    let styles_source = include_str!("../src/styles.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    for required in [
        ".ui-file-trigger--disabled,",
        ".ui-file-trigger[data-disabled=\"true\"]",
        ".ui-file-trigger[data-motion-source=\"custom\"]",
        ".ui-file-trigger--custom-motion,",
        ".ui-file-trigger[data-custom-motion=\"true\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should branch visual states via stable class/data markers: `{required}`"
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":has(", ">"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not rely on fragile DOM-structure selectors: `{forbidden}`"
        );
    }

    for forbidden in ["style=\"", "style:", "style="] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not encode business style logic through inline styles: `{forbidden}`"
        );
    }

    assert!(
        readme_source.contains("## Style State Selectors"),
        "README should document style-state selector contract."
    );
}

#[test]
fn semantic_tests_cover_contract_not_snapshots() {
    let view_source = include_str!("../src/view.rs");
    let semantics_source = include_str!("./semantics.rs");
    let readme_source = include_str!("../src/README.md");

    for required in [
        "data-state=data_state_attr",
        "data-disabled=data_disabled_attr",
        "data-motion-source=state.motion_source_attr",
        "aria-hidden=input_aria_hidden",
        "tabindex=input_tabindex",
    ] {
        assert!(
            view_source.contains(required),
            "semantic contract should expose stable marker for assertions: `{required}`"
        );
    }

    for required in [
        "let on_press = Callback::new",
        "if !is_disabled {",
        "let on_change = move |_ev: ev::Event| {",
        "if is_disabled {",
        "input.set_value(\"\");",
        "on:change=on_change",
    ] {
        assert!(
            view_source.contains(required),
            "semantic tests should be able to cover key interaction path hook: `{required}`"
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep explicit platform branches for SSR/wasm contract checks: `{required}`"
        );
    }

    for forbidden in [
        "snapshot",
        "insta::",
        "assert_snapshot",
        "to_match_snapshot",
    ] {
        assert!(
            !semantics_source.contains(forbidden),
            "semantics contract tests should not depend on visual snapshot assertions: `{forbidden}`"
        );
    }

    assert!(
        readme_source.contains("## Semantic Test Matrix"),
        "README should document semantic test matrix and applicability notes."
    );
}

#[test]
fn component_files_preserve_single_responsibility_boundaries() {
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let styles_source = include_str!("../src/styles.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let readme_source = include_str!("../src/README.md");

    for required in [
        "mod logic;",
        "mod view;",
        "pub use logic::FileTriggerFile;",
        "pub use motion::FileTriggerMotion;",
        "pub use view::FileTrigger;",
    ] {
        assert!(
            mod_source.contains(required),
            "mod.rs should expose minimal stable boundary: `{required}`"
        );
    }
    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not leak implementation modules: `{forbidden}`"
        );
    }

    for required in [
        "pub use ui_state_primitives::file_trigger::{",
        "pub fn compose_class_name(",
        "pub fn compose_class_name_from_render_state(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should focus on normalization/derived state assembly: `{required}`"
        );
    }
    for forbidden in ["view! {", ".ui-file-trigger", "use_file_trigger("] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not take view/styles/headless mounting concerns: `{forbidden}`"
        );
    }

    for required in [
        "pub const CSS: &str",
        "var(--ui-space-sm)",
        ".ui-file-trigger[data-disabled",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should remain static token-first CSS contract: `{required}`"
        );
    }
    for forbidden in ["view! {", "Callback::new", "use_file_trigger("] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not include runtime logic: `{forbidden}`"
        );
    }

    for required in [
        "#[component]",
        "let semantics = use_file_trigger(FileTriggerOptions { state, lang, dir });",
        "super::logic::resolve_render_state(",
        "<Button is_disabled=is_disabled motion=motion.trigger on_press=on_press>",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should render structure and mount headless semantics: `{required}`"
        );
    }
    for forbidden in ["pub fn resolve_props(", "pub fn resolve_state("] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not re-implement logic-layer primitives: `{forbidden}`"
        );
    }

    for required in [
        "pub struct FileTriggerMotion",
        "pub fn sanitize_motion(motion: FileTriggerMotion) -> FileTriggerMotion",
        "trigger: crate::button::motion::sanitize_motion(motion.trigger)",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should only map/sanitize motion contract: `{required}`"
        );
    }
    for forbidden in ["requestAnimationFrame", "Keyframe", "Animation"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not implement animation engine internals: `{forbidden}`"
        );
    }

    assert!(
        readme_source.contains("## File Responsibilities"),
        "README should document per-file responsibility boundaries."
    );
}

#[test]
fn component_does_not_introduce_spec_rs_for_simple_surface() {
    let mod_source = include_str!("../src/mod.rs");
    let readme_source = include_str!("../src/README.md");
    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");

    assert!(
        !spec_path.exists(),
        "simple FileTrigger surface should not introduce `src/spec.rs` without schema-level need."
    );

    for forbidden in ["mod spec;", "pub mod spec", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "component public boundary should not expose a `spec.rs` module: `{forbidden}`"
        );
    }

    assert!(
        readme_source.contains("## Spec Policy"),
        "README should document why `spec.rs` is intentionally absent for this component."
    );
}

#[test]
fn component_follows_token_first_static_style_contract() {
    let styles_source = include_str!("../src/styles.rs");
    let view_source = include_str!("../src/view.rs");
    let css_aggregate_source = include_str!("../../../crates/ui-components/src/css.rs");
    let root_source = include_str!("../../../crates/ui-components/src/root.rs");
    let readme_source = include_str!("../src/README.md");

    for required in [
        "pub const CSS: &str",
        "var(--ui-space-sm)",
        ".ui-file-trigger[data-disabled",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first static CSS contract: `{required}`"
        );
    }

    for required in [
        "#[cfg(feature = \"component-file_trigger\")]",
        "out.push_str(crate::file_trigger::styles::CSS);",
    ] {
        assert!(
            css_aggregate_source.contains(required),
            "ui-components css aggregation should include file-trigger behind feature gate: `{required}`"
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should own centralized CSS injection path: `{required}`"
        );
    }

    for forbidden in ["class=\"flex", "class=\"grid", "tw-", "styled(", "css!"] {
        assert!(
            !view_source.contains(forbidden),
            "component default style path should not drift to Utility-First/CSS-in-Rust: `{forbidden}`"
        );
    }

    assert!(
        readme_source.contains("## Token-first Style Contract"),
        "README should document token-first style contract and aggregation path."
    );
}

#[test]
fn component_marks_visual_desire_as_repo_level_concern() {
    let styles_source = include_str!("../src/styles.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Visual Desire"),
        "README should document visual-desire applicability for this component."
    );
    assert!(
        readme_source.contains("N/A for this single-component checklist"),
        "README should explicitly mark visual-desire baseline as repo-level concern."
    );

    for forbidden in ["body {", ".ui-root", "font-family:", "#"] {
        assert!(
            !styles_source.contains(forbidden),
            "component styles should not define global theme language directly: `{forbidden}`"
        );
    }
}

#[test]
fn component_supports_tree_shaking_feature_gates() {
    let ui_components_cargo = include_str!("../../../crates/ui-components/Cargo.toml");
    let ui_components_lib = include_str!("../../../crates/ui-components/src/lib.rs");
    let ui_components_css = include_str!("../../../crates/ui-components/src/css.rs");
    let web_demo_cargo = include_str!("../../../apps/web-demo/Cargo.toml");
    let readme_source = include_str!("../src/README.md");

    assert!(
        ui_components_cargo.contains("component-file_trigger = []"),
        "ui-components should expose a dedicated `component-file_trigger` feature."
    );
    assert!(
        ui_components_lib.contains("#[cfg(feature = \"component-file_trigger\")]")
            && ui_components_lib.contains("pub mod file_trigger;"),
        "lib.rs should gate file-trigger module reachability by `component-file_trigger`."
    );
    assert!(
        ui_components_css.contains("#[cfg(feature = \"component-file_trigger\")]")
            && ui_components_css.contains("out.push_str(crate::file_trigger::styles::CSS);"),
        "css.rs should gate file-trigger CSS aggregation by component feature."
    );
    assert!(
        ui_components_css.contains("#[cfg(feature = \"inject-css\")]")
            && ui_components_css.contains("#[cfg(not(feature = \"inject-css\"))]"),
        "css aggregation should remain behind explicit `inject-css` gate."
    );
    assert!(
        web_demo_cargo.contains("ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }"),
        "web-demo should keep ui-components usage feature-scoped and avoid implicit default/all-components pull-up."
    );
    assert!(
        !web_demo_cargo.contains("all-components"),
        "web-demo should not directly enable `all-components` in dependency declaration."
    );

    assert!(
        readme_source.contains("## Tree Shaking"),
        "README should document tree-shaking strategy and scope."
    );
}

#[test]
fn component_dx_workbench_contract_is_documented_and_live() {
    let readme_source = include_str!("../src/README.md");
    let docs_file = include_str!("../../../apps/docs-app/src/pages/components/pages/files.rs");

    assert!(
        readme_source.contains("## DX Requirements"),
        "README should include DX requirement section."
    );
    for marker in [
        "Quick Start (Default API)",
        "Interactive Playground (展示 / Config / Code / CSS Test)",
        "State Comparison (Default / Disabled / Custom Motion)",
    ] {
        assert!(
            docs_file.contains(marker),
            "docs-app should provide DX/workbench evidence marker: `{marker}`"
        );
    }
}

#[test]
fn component_engineering_baseline_keeps_runtime_and_spec_leak_out() {
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Engineering Baseline"),
        "README should explain engineering baseline applicability."
    );
    for forbidden in [
        "tokio",
        "async_std",
        "serde_json",
        "Serialize",
        "Deserialize",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "component should not leak runtime/spec protocol details: `{forbidden}`"
        );
    }
}

#[test]
fn component_styles_follow_defensive_variable_contract() {
    let styles_source = include_str!("../src/styles.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Defensive Variables and Layering"),
        "README should document defensive variable and layering contract."
    );
    for expected in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-drop-zone-disabled-opacity, var(--ui-fallback-drop-zone-disabled-opacity))",
        "var(--ui-drop-zone-sr-only-size, var(--ui-fallback-drop-zone-sr-only-size))",
        "var(--ui-min-inline-size-none, var(--ui-fallback-min-inline-size-none))",
    ] {
        assert!(
            styles_source.contains(expected),
            "styles should keep token+fallback chain: `{expected}`"
        );
    }
    for forbidden in ["#", " 1px;", " 0.7;"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles should avoid hardcoded terminal visual constants: `{forbidden}`"
        );
    }
}

#[test]
fn component_css_layering_and_inline_style_policy_hold() {
    let view_source = include_str!("../src/view.rs");
    let css_source = include_str!("../../../crates/ui-components/src/css.rs");

    assert!(
        css_source.contains("@layer ui"),
        "components CSS aggregation should stay under @layer ui."
    );
    assert!(
        !view_source.contains("style="),
        "view should not emit raw inline style attributes."
    );
}

#[test]
fn component_motion_contract_is_typed_and_delegated() {
    let motion_source = include_str!("../src/motion.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Motion Contract"),
        "README should document motion contract behavior."
    );
    for expected in [
        "pub struct FileTriggerMotion",
        "pub fn sanitize_motion(",
        "crate::button::motion::sanitize_motion(motion.trigger)",
    ] {
        assert!(
            motion_source.contains(expected),
            "motion.rs should keep typed/delegated motion contract: `{expected}`"
        );
    }
    assert!(
        view_source.contains("motion=motion.trigger"),
        "view should pass typed motion contract into Button mount path."
    );
}

#[test]
fn component_and_ui_components_entry_layout_contract_holds() {
    let readme_source = include_str!("../src/README.md");
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../");

    let file_trigger_dir = root.join("components/file-trigger/src");
    for expected in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            file_trigger_dir.join(expected).exists(),
            "component directory should keep required file: `{expected}`"
        );
    }
    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !file_trigger_dir.join(forbidden).exists(),
            "simple component should not introduce `{forbidden}`"
        );
    }

    let ui_components_dir = root.join("crates/ui-components/src");
    for forbidden in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_dir.join(forbidden).exists(),
            "ui-components src should not contain forbidden shared file: `{forbidden}`"
        );
    }

    assert!(
        readme_source.contains("## Entry and File Layout"),
        "README should document entry/file layout contract."
    );
}

#[test]
fn component_manifest_and_rbi_projection_are_present() {
    let manifest_source = include_str!("../src/Component.toml");
    let rbi_source = include_str!("../src/file_trigger.rbi");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Manifest and RBI"),
        "README should document context-compression assets."
    );
    for expected in [
        "name = \"FileTrigger\"",
        "crate = \"ui-file-trigger\"",
        "context_compression_manifest",
        "rbi_signature_projection",
        "agent_contract_schema",
        "snapshot_rendering",
    ] {
        assert!(
            manifest_source.contains(expected),
            "Component.toml should include capability metadata: `{expected}`"
        );
    }
    for expected in [
        "pub struct FileTriggerMotion",
        "pub struct FileTriggerAgentContract",
        "pub fn resolve_agent_contract(",
        "pub fn FileTrigger(",
    ] {
        assert!(
            rbi_source.contains(expected),
            "RBI projection should include signature shape marker: `{expected}`"
        );
    }
}

#[test]
fn component_agent_contract_schema_markers_are_typed_and_mounted() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");

    assert!(
        readme_source.contains("## Agent Contract Schema"),
        "README should document Agent Contract schema."
    );
    for expected in [
        "pub const FILE_TRIGGER_COMPONENT_SCHEMA_NAME",
        "pub const FILE_TRIGGER_COMPONENT_SCHEMA_VERSION",
        "pub enum FileTriggerAgentIntent",
        "pub struct FileTriggerAgentContract",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            logic_source.contains(expected),
            "logic.rs should define typed Agent Contract surface: `{expected}`"
        );
    }
    for marker in [
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-schema-version=agent_contract.schema_version",
        "data-ui-intent=agent_contract.intent.as_attr()",
        "data-ui-action=agent_contract.action.as_attr()",
        "data-ui-state=agent_contract.state",
        "data-ui-source=agent_contract.source.as_attr()",
        "data-ui-stream-support=agent_contract.stream_support.as_attr()",
        "data-ui-stream-fallback=agent_contract.stream_fallback.as_attr()",
        "data-ui-output-status=agent_contract.output_status.as_attr()",
    ] {
        assert!(
            view_source.contains(marker),
            "view should mount agent marker: `{marker}`"
        );
    }
}

#[test]
fn component_snapshot_and_streaming_scope_are_explicit() {
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");
    let manifest_source = include_str!("../src/Component.toml");

    assert!(
        readme_source.contains("## Snapshot and Streaming Scope"),
        "README should explain snapshot/streaming applicability."
    );
    assert!(
        readme_source.contains("Streaming Optional"),
        "README should describe this component as streaming-optional."
    );
    assert!(
        view_source.contains("data-ui-stream-support=agent_contract.stream_support.as_attr()")
            && view_source
                .contains("data-ui-stream-fallback=agent_contract.stream_fallback.as_attr()"),
        "view should expose explicit stream support/fallback markers."
    );
    assert!(
        manifest_source.contains("streaming_optional_snapshot_fallback"),
        "Component.toml should project streaming optional + snapshot fallback capability."
    );
}

#[test]
fn component_rust_hygiene_local_guards_hold() {
    let logic_source = include_str!("../src/logic.rs");
    let motion_source = include_str!("../src/motion.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");
    let source = [logic_source, motion_source, view_source].join("\n");

    assert!(
        readme_source.contains("## Rust Hygiene"),
        "README should document rust-hygiene policy for this component."
    );
    for forbidden in [".unwrap(", ".expect(", "let _ = "] {
        assert!(
            !source.contains(forbidden),
            "component source should not contain forbidden hygiene pattern: `{forbidden}`"
        );
    }
}

#[test]
fn component_docs_and_copy_ready_contract_are_present() {
    let readme_source = include_str!("../src/README.md");
    let docs_file = include_str!("../../../apps/docs-app/src/pages/components/pages/files.rs");
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");

    assert!(
        readme_source.contains("## E2E / Docs Product"),
        "README should document docs-product and e2e boundary."
    );
    for marker in [
        "Quick Start (Default API)",
        "Interactive Playground (展示 / Config / Code / CSS Test)",
        "State Comparison (Default / Disabled / Custom Motion)",
    ] {
        assert!(
            docs_file.contains(marker),
            "docs file should include productized section marker: `{marker}`"
        );
    }
    for marker in ["compose_copy_ready_code", "DEFAULT_PLAYGROUND_IMPORTS"] {
        assert!(
            playground_source.contains(marker),
            "Playground should support copy-ready source contract: `{marker}`"
        );
    }
}

#[test]
fn component_heroui_sync_rule_is_noop_without_parameter_changes() {
    let readme_source = include_str!("../src/README.md");
    let heroui_strategy = include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");

    assert!(
        readme_source.contains("HeroUI parameter-strategy docs do not require sync updates"),
        "README should explicitly state why HeroUI doc sync is N/A for this patch."
    );
    assert!(
        heroui_strategy.contains("HeroUI") || heroui_strategy.contains("Hero"),
        "reference strategy doc should exist and be readable."
    );
}
