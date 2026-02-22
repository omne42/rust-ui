fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "motion" => include_str!("../src/motion.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "component_toml" => include_str!("../src/Component.toml"),
        "rbi" => include_str!("../src/chip.rbi"),
        "chip_cargo" => include_str!("../Cargo.toml"),
        "primitive" => include_str!("../../../crates/ui-state-primitives/src/chip.rs"),
        "ui_components_lib" => include_str!("../../../crates/ui/src/lib.rs"),
        "ui_components_css" => include_str!("../../../crates/ui/src/css.rs"),
        "ui_components_cargo" => include_str!("../../../crates/ui/Cargo.toml"),
        "docs_display" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages/display.rs")
        }
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn ui_components_reexports_chip_component_crate() {
    let lib_source = load_source("ui_components_lib");
    let cargo_source = load_source("ui_components_cargo");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-chip\")]")
            && lib_source.contains("pub use ui_chip as chip;"),
        "ui should re-export the external ui-chip crate as `chip`.",
    );
    assert!(
        cargo_source.contains("component-chip = [\"dep:ui-chip\"]"),
        "component-chip feature should depend on dep:ui-chip after extraction.",
    );
    assert!(
        cargo_source.contains("ui-chip = { path = \"../../components/chip\", optional = true }"),
        "ui Cargo.toml should include the optional ui-chip dependency.",
    );
}

#[test]
fn chip_does_not_expose_logic_or_view_modules() {
    let source = load_source("mod");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Chip internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn chip_exposes_motion_contract_with_attach_mount() {
    let mod_source = load_source("mod");
    let motion_source = load_source("motion");
    let view_source = load_source("view");
    let cargo_source = load_source("chip_cargo");

    for needle in [
        "pub mod motion;",
        "pub use motion::ChipMotion;",
        "#[prop(optional)] motion: ChipMotion,",
        "let motion = chip_motion::sanitize_motion(motion);",
        "chip_motion::attach_motion(node_ref, motion);",
        "node_ref=node_ref",
    ] {
        assert!(
            mod_source.contains(needle)
                || view_source.contains(needle)
                || motion_source.contains(needle),
            "Chip motion contract wiring should include `{needle}`.",
        );
    }

    assert!(
        cargo_source.contains("ui-motion = { path = \"../../crates/ui-motion\" }"),
        "Chip crate should depend on ui-motion for shared motion runtime contract.",
    );
}

#[test]
fn chip_context_compression_manifest_and_rbi_are_kept_in_sync() {
    let manifest_source = load_source("component_toml");
    let rbi_source = load_source("rbi");
    let view_source = load_source("view");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Chip\"",
        "crate = \"ui-chip\"",
        "name = \"variant\"",
        "name = \"size\"",
        "name = \"is_disabled\"",
        "name = \"on_dismiss\"",
        "name = \"motion\"",
        "name = \"dismiss_aria_label\"",
        "name = \"class_name\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"ui-state-primitives\"",
        "name = \"ui-motion\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "Chip Component.toml should include `{needle}`.",
        );
    }

    for needle in [
        "pub type ChipVariant = ui_state_primitives::chip::ChipVariant;",
        "pub type ChipSize = ui_state_primitives::chip::ChipSize;",
        "pub struct ChipMotion",
        "pub fn Chip(",
        "on_dismiss: Option<ui_headless::OnPress>",
        "children: leptos::children::Children,",
        "-> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "Chip RBI projection should include `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional)] variant: ChipVariant",
        "#[prop(optional)] size: ChipSize",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] on_dismiss: Option<OnPress>",
        "#[prop(optional)] motion: ChipMotion",
        "#[prop(optional, into)] dismiss_aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            view_source.contains(needle),
            "Chip view contract should include `{needle}` to stay in sync with manifest/RBI.",
        );
    }
}

#[test]
fn chip_uses_logic_state_model() {
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let primitive_source = load_source("primitive");

    for needle in [
        "pub use ui_state_primitives::chip::{",
        "ChipStateInput",
        "ChipState",
        "normalize_optional_text",
        "resolve_dismiss_aria_label",
        "resolve_state",
        "pub fn compose_class_name(",
        "ui-chip--custom-class",
    ] {
        assert!(
            logic_source.contains(needle),
            "Chip logic should consume state primitives and keep assembly helpers; missing `{needle}`."
        );
    }

    for forbidden in [
        "pub struct ChipStateInput {",
        "pub struct ChipState {",
        "pub enum ChipVariant {",
        "pub enum ChipSize {",
        "pub fn resolve_dismiss_aria_label(",
        "pub fn resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Chip logic must not reimplement state primitives; found `{forbidden}`."
        );
    }

    for needle in [
        "pub enum ChipVariant",
        "pub enum ChipSize",
        "pub struct ChipStateInput",
        "pub struct ChipState",
        "pub fn resolve_dismiss_aria_label(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Chip state primitive layer should include `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_dismiss_aria_label(dismiss_aria_label)",
        "logic::resolve_state(ChipStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Chip view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn chip_emits_baseline_style_state_data_attributes() {
    let source = load_source("view");

    for attr in [
        "data-slot=\"chip\"",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-state=state.state_attr",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-removable=state.has_dismiss_action.then_some(\"true\")",
        "data-static=state.is_static.then_some(\"true\")",
        "data-dismiss-label-source=state.dismiss_label_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "data-motion-source=if motion == ChipMotion::default() {",
        "data-custom-motion=(motion != ChipMotion::default()).then_some(\"true\")",
        "data-slot=\"chip-content\"",
        "data-slot=\"chip-dismiss\"",
        "data-label-source=state.dismiss_label_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Chip should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn chip_styles_include_variant_size_and_state_source_markers() {
    let source = load_source("styles");

    for selector in [
        ".ui-chip--size-sm",
        ".ui-chip[data-size=\"md\"]",
        ".ui-chip--variant-danger",
        ".ui-chip[data-variant=\"outline\"]",
        ".ui-chip--enabled",
        ".ui-chip[data-state=\"disabled\"]",
        ".ui-chip[data-state=\"static\"]",
        ".ui-chip[data-state=\"removable\"]",
        ".ui-chip--dismiss-label-custom",
        ".ui-chip[data-dismiss-label-source=\"custom\"]",
        ".ui-chip--custom-class",
        ".ui-chip[data-custom-class=\"true\"]",
        ".ui-chip[data-class-source=\"custom\"]",
        ".ui-chip__dismiss[data-disabled=\"true\"]",
        ".ui-chip__dismiss[data-label-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "Chip styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn chip_motion_contract_respects_reduced_motion_and_non_wasm_noop() {
    let source = load_source("motion");

    for needle in [
        "pub struct ChipMotion {",
        "pub spring: SpringConfig,",
        "pub enter_offset_y_px: f64,",
        "pub enter_scale: f64,",
        "ui_motion::presets::spring_soft()",
        "pub fn sanitize_motion(motion: ChipMotion) -> ChipMotion",
        "ui_motion::spring::sanitize_config(motion.spring, default.spring)",
        "#[cfg(target_arch = \"wasm32\")]",
        "if ui_motion::web::prefers_reduced_motion() {",
        "pub fn attach_motion(node_ref: NodeRef<html::Span>, motion: ChipMotion)",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            source.contains(needle),
            "Chip motion contract should contain `{needle}`.",
        );
    }
}

#[test]
fn chip_marks_hyper_structure_builder_spec_as_not_applicable() {
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let spec_path = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/spec.rs"));

    assert!(
        !spec_path.exists(),
        "Chip is not a complex builder-driven component and should not carry `src/spec.rs`."
    );

    for forbidden in [
        "mod spec;",
        "pub use spec::",
        "ChipSpec",
        "Spec::new(",
        ".render()",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Chip should not expose hyper-structure builder spec API marker `{forbidden}`.",
        );
    }
}

#[test]
fn chip_styles_use_defensive_variable_fallback_chains() {
    let source = load_source("styles");

    for required in [
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size))",
        "var(--ui-button-size-s-line-height, var(--ui-fallback-button-size-s-line-height))",
    ] {
        assert!(
            source.contains(required),
            "Chip styles should use defensive fallback chain `{required}`.",
        );
    }

    for forbidden in [
        "var(--ui-fg);",
        "var(--ui-bg-muted);",
        "var(--ui-border);",
        "var(--ui-focus-ring);",
        "24px",
        "28px",
        "32px",
        "18px",
        "9999px",
    ] {
        assert!(
            !source.contains(forbidden),
            "Chip styles should not keep raw terminal style literal `{forbidden}`."
        );
    }
}

#[test]
fn chip_css_is_aggregated_under_ui_layer_without_plain_inline_styles() {
    let css_registry = load_source("ui_components_css");
    let view = load_source("view");

    assert!(
        css_registry.contains("out.push_str(\"\\n@layer ui {\\n\");"),
        "ui css registry should aggregate component styles under `@layer ui`.",
    );
    assert!(
        css_registry.contains("#[cfg(feature = \"component-chip\")]")
            && css_registry.contains("out.push_str(crate::chip::styles::CSS);"),
        "chip styles should be feature-gated and injected through the centralized ui layer registry.",
    );

    for line in view.lines() {
        let trimmed = line.trim_start();

        assert!(
            !trimmed.starts_with("style="),
            "chip view should not use plain inline `style=...`; found `{trimmed}`.",
        );

        if trimmed.contains("style:") {
            assert!(
                trimmed.contains("style:--"),
                "chip runtime style mutation must use CSS custom properties only; found `{trimmed}`.",
            );
        }
    }
}

#[test]
fn chip_docs_page_covers_primary_playgrounds() {
    let source = load_source("docs_display");

    for needle in [
        "pub(super) fn chip() -> AnyView",
        "title=\"Chip\"",
        "slug=\"chip\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Comparison Matrix (Variant / Size / Disabled / Custom)\"",
        "test_css_source=chip_test_css_source",
        "test_source_path=\"components/chip/src/styles.rs\".to_string()",
        "test_config_signal=workbench_config",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Chip.",
        );
    }
}

#[test]
fn chip_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("docs_display");

    for needle in [
        "id_base=\"docs-chip-variant\".to_string()",
        "id_base=\"docs-chip-size\".to_string()",
        "\"is_disabled\"",
        "\"Dismiss action\"",
        "\"Custom dismiss aria label\"",
        "\"Custom class_name\"",
        "title=\"Comparison Matrix (Variant / Size / Disabled / Custom)\"",
        "dismiss_aria_label=\"Remove reviewer\".to_string()",
        "class_name=\"docs-chip-custom\".to_string()",
        "variant=ChipVariant::Danger",
        "size=ChipSize::Lg",
        "is_disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "chip docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn chip_does_not_depend_on_two_pass_geometry_measurement_loop() {
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in [
        "getBoundingClientRect",
        "get_bounding_client_rect",
        "ResizeObserver",
        "DomRect",
        "offset_width",
        "offset_height",
        "client_width",
        "client_height",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "Chip should not introduce two-pass geometry measurement paths; found `{forbidden}`."
        );
    }
}

#[test]
fn chip_does_not_introduce_collection_registration_protocol_paths() {
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet<",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "Chip should not introduce collection registration protocol paths; found `{forbidden}`."
        );
    }
}

#[test]
fn chip_does_not_introduce_slot_projection_or_keep_alive_lifecycle_paths() {
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in [
        "KeepAlive",
        "Lazy",
        "Eager",
        "NotifyHidden",
        "slot projection",
        "slot_projection",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "Chip should not introduce slot projection lifecycle paths; found `{forbidden}`."
        );
    }
}

#[test]
fn chip_does_not_introduce_environment_stream_sampling_or_breakpoint_actions() {
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in [
        "BreakpointChanged",
        "IntersectionObserver",
        "ResizeObserver",
        "matchMedia",
        "on:resize",
        "theme change",
        "debounce",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "Chip should not introduce environment-stream sampling paths; found `{forbidden}`."
        );
    }
}

#[test]
fn chip_does_not_introduce_event_light_cone_bulk_collection_paths() {
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in [
        "Context Bus",
        "context_bus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "prop drilling",
        "Table",
        "Grid",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "Chip should not introduce event-light-cone bulk-collection paths; found `{forbidden}`."
        );
    }
}

#[test]
fn chip_does_not_introduce_causality_bus_trace_propagation_paths() {
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "causality_bus",
        "bus_broadcast",
        "derive_command",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "Chip should not introduce causality-bus trace propagation paths; found `{forbidden}`."
        );
    }
}

#[test]
fn chip_does_not_introduce_focus_stack_or_overlay_focus_restore_paths() {
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in [
        "FocusManager",
        "focus_manager",
        "FallbackTo",
        "fallback_to",
        "document.body",
        "restore_focus",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "Chip should not introduce overlay focus-stack restore paths; found `{forbidden}`."
        );
    }
}

#[test]
fn chip_does_not_introduce_foreign_zone_escape_hatch_paths() {
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in [
        "ForeignZone",
        "foreign_zone",
        "YieldControl",
        "yield_control",
        "CleanupForeign",
        "cleanup_foreign",
        "ECharts",
        "Mapbox",
        "Leaflet",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "Chip should not introduce foreign-zone escape hatch paths; found `{forbidden}`."
        );
    }
}

#[test]
fn chip_does_not_introduce_nondeterministic_ssr_hydration_id_paths() {
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in [
        "now()",
        "SystemTime::now",
        "Date::now",
        "uuid::Uuid",
        "Uuid::new_v4",
        "rand::random",
        "thread_rng",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "Chip should not introduce non-deterministic SSR/hydration ID paths; found `{forbidden}`."
        );
    }
}

#[test]
fn chip_non_test_sources_follow_rust_hygiene_contract() {
    let mod_source = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let styles = load_source("styles");

    for forbidden in [".unwrap(", ".expect(", ".unwrap_err(", "let _ ="] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !motion.contains(forbidden)
                && !styles.contains(forbidden),
            "Chip non-test source should not contain `{forbidden}`."
        );
    }

    for required in [
        "use std::borrow::Cow;",
        "Vec<Cow<'_, str>>",
        "Cow::Borrowed(\"ui-chip\")",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic.contains(required),
            "Chip class assembly should use Cow-based string ownership for `{required}`."
        );
    }
}

#[test]
fn chip_does_not_require_versioned_schema_registry_migration_for_current_change_set() {
    let mod_source = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let manifest = load_source("component_toml");
    let rbi = load_source("rbi");

    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "SchemaRegistry",
        "schema_registry",
        "deprecation_window",
        "deprecated_since",
        "breaking_change",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !manifest.contains(forbidden)
                && !rbi.contains(forbidden),
            "Chip should not claim a versioned schema migration path for this change set; found `{forbidden}`."
        );
    }
}
