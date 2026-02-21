fn load_source(rel_path: &str) -> &'static str {
    match rel_path {
        "../../components/color-swatch-picker/src/lib.rs" => include_str!("../src/lib.rs"),
        "../../components/color-swatch-picker/src/mod.rs" => include_str!("../src/mod.rs"),
        "../../components/color-swatch-picker/src/logic.rs" => include_str!("../src/logic.rs"),
        "../../components/color-swatch-picker/src/motion.rs" => include_str!("../src/motion.rs"),
        "../../components/color-swatch-picker/src/styles.rs" => include_str!("../src/styles.rs"),
        "../../components/color-swatch-picker/src/view.rs" => include_str!("../src/view.rs"),
        "../../components/color-swatch-picker/src/README.md" => include_str!("../src/README.md"),
        "../../components/color-swatch-picker/src/Component.toml" => {
            include_str!("../src/Component.toml")
        }
        "../../components/color-swatch-picker/src/color_swatch_picker.rbi" => {
            include_str!("../src/color_swatch_picker.rbi")
        }
        "../../crates/ui-components/src/css.rs" => {
            include_str!("../../../crates/ui-components/src/css.rs")
        }
        "../../crates/ui-components/src/lib.rs" => {
            include_str!("../../../crates/ui-components/src/lib.rs")
        }
        "../../crates/ui-components/src/root.rs" => {
            include_str!("../../../crates/ui-components/src/root.rs")
        }
        "../../crates/ui-visual-primitive/src/active_highlight.rs" => {
            include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs")
        }
        "../../crates/ui-headless/src/controllable_state.rs" => {
            include_str!("../../../crates/ui-headless/src/controllable_state.rs")
        }
        "../../crates/ui-headless/src/presence.rs" => {
            include_str!("../../../crates/ui-headless/src/presence.rs")
        }
        "../../crates/ui-headless/src/a11y.rs" => {
            include_str!("../../../crates/ui-headless/src/a11y.rs")
        }
        "../../crates/ui-headless/src/lib.rs" => {
            include_str!("../../../crates/ui-headless/src/lib.rs")
        }
        "../../crates/ui-headless/src/trace.rs" => {
            include_str!("../../../crates/ui-headless/src/trace.rs")
        }
        "../../crates/ui-motion/src/lib.rs" => {
            include_str!("../../../crates/ui-motion/src/lib.rs")
        }
        "../../crates/ui-theme/src/css.rs" => include_str!("../../../crates/ui-theme/src/css.rs"),
        "../../crates/ui-components/Cargo.toml" => {
            include_str!("../../../crates/ui-components/Cargo.toml")
        }
        "../../apps/web-demo/Cargo.toml" => include_str!("../../../apps/web-demo/Cargo.toml"),
        "../../apps/docs-app/Cargo.toml" => include_str!("../../../apps/docs-app/Cargo.toml"),
        "../../apps/docs-app/src/lib.rs" => include_str!("../../../apps/docs-app/src/lib.rs"),
        "../../apps/docs-app/src/debug_overlay.rs" => {
            include_str!("../../../apps/docs-app/src/debug_overlay.rs")
        }
        "../../apps/docs-app/src/pages/components/shell.rs" => {
            include_str!("../../../apps/docs-app/src/pages/components/shell.rs")
        }
        "../../apps/docs-app/src/playground.rs" => {
            include_str!("../../../apps/docs-app/src/playground.rs")
        }
        "../../apps/docs-app/src/perf_probe.rs" => {
            include_str!("../../../apps/docs-app/src/perf_probe.rs")
        }
        "../../scripts/check-ui-components-tree-shaking.sh" => {
            include_str!("../../../scripts/check-ui-components-tree-shaking.sh")
        }
        "../../scripts/check-ui-components-performance.sh" => {
            include_str!("../../../scripts/check-ui-components-performance.sh")
        }
        "../../scripts/check-ui-components-entrypoints.sh" => {
            include_str!("../../../scripts/check-ui-components-entrypoints.sh")
        }
        "../../scripts/check-ui-components-platforms.sh" => {
            include_str!("../../../scripts/check-ui-components-platforms.sh")
        }
        "../../scripts/check-ui-components-view-macro.sh" => {
            include_str!("../../../scripts/check-ui-components-view-macro.sh")
        }
        "../../scripts/check-ui-components-wasm-debug.sh" => {
            include_str!("../../../scripts/check-ui-components-wasm-debug.sh")
        }
        "../../scripts/check-ui-components-dx.sh" => {
            include_str!("../../../scripts/check-ui-components-dx.sh")
        }
        "../../scripts/check-ui-components-engineering.sh" => {
            include_str!("../../../scripts/check-ui-components-engineering.sh")
        }
        "../../scripts/check-ui-components-inner-html.sh" => {
            include_str!("../../../scripts/check-ui-components-inner-html.sh")
        }
        "../../scripts/check-ui-components-contract-hygiene.sh" => {
            include_str!("../../../scripts/check-ui-components-contract-hygiene.sh")
        }
        "../../scripts/check-ui-components-component-files.sh" => {
            include_str!("../../../scripts/check-ui-components-component-files.sh")
        }
        "../../scripts/check-ui-components-streaming.sh" => {
            include_str!("../../../scripts/check-ui-components-streaming.sh")
        }
        "../../scripts/check-rust-hygiene.sh" => {
            include_str!("../../../scripts/check-rust-hygiene.sh")
        }
        "../../scripts/tree_shaking_budget.env" => {
            include_str!("../../../scripts/tree_shaking_budget.env")
        }
        "../../components/color-swatch-picker/check2.md" => include_str!("../check2.md"),
        "../../components/color-swatch-picker/src/check2.md" => include_str!("../src/check2.md"),
        "../../apps/docs-app/src/pages/components/pages.rs" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages.rs")
        }
        "../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs" => {
            include_str!(
                "../../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs"
            )
        }
        "../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs" => {
            include_str!("../../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs")
        }
        "../../e2e/tests/docs_app_components_coverage.spec.mjs" => {
            include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs")
        }
        "../../e2e/tests/docs_app_color_swatch_picker_contract.spec.mjs" => {
            include_str!("../../../e2e/tests/docs_app_color_swatch_picker_contract.spec.mjs")
        }
        "../../docs/plan/TODO.md" => include_str!("../../../docs/plan/TODO.md"),
        "../../docs/spec/heroui-parameter-design-strategy.md" => {
            include_str!("../../../docs/spec/heroui-parameter-design-strategy.md")
        }
        "../../apps/docs-app/src/pages/components/pages/display_extra.rs" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages/display_extra.rs")
        }
        "../../scripts/check-ui-components-e2e-color-swatch-picker.sh" => {
            include_str!("../../../scripts/check-ui-components-e2e-color-swatch-picker.sh")
        }
        "legacy_semantics" => {
            include_str!(
                "../../../components/color-swatch-picker/test/color_swatch_picker_semantics.rs"
            )
        }
        _ => panic!("unsupported source path: {rel_path}"),
    }
}

fn contains_hex_color_literal(source: &str) -> bool {
    let bytes = source.as_bytes();
    bytes
        .windows(2)
        .any(|pair| pair[0] == b'#' && pair[1].is_ascii_hexdigit())
}

fn path_exists(rel_path: &str) -> bool {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(rel_path)
        .exists()
}

#[test]
fn color_swatch_picker_semantics_tests_are_migrated_to_component_directory() {
    let lib_source = load_source("../../components/color-swatch-picker/src/lib.rs");
    let mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");
    let legacy_semantics = load_source("legacy_semantics");
    let local_semantics = include_str!("semantics.rs");

    for source in [lib_source, mod_source] {
        assert!(
            source.contains("#[path = \"../test/semantics.rs\"]")
                && source.contains("mod semantics_tests;"),
            "color-swatch-picker should wire `components/color-swatch-picker/test/semantics.rs` from entrypoints."
        );
    }

    assert!(
        legacy_semantics.contains("../../../components/color-swatch-picker/test/semantics.rs"),
        "legacy ui-components semantics entry should include migrated component semantics file.",
    );
    assert!(
        local_semantics
            .contains("color_swatch_picker_semantics_tests_are_migrated_to_component_directory"),
        "component-local semantics suite should provide migration coverage.",
    );
}

#[test]
fn color_swatch_picker_public_surface_does_not_expose_dom_platform_types() {
    let mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");
    let lib_source = load_source("../../components/color-swatch-picker/src/lib.rs");

    for forbidden in [
        "web_sys::",
        "web-sys",
        "wasm_bindgen",
        "JsValue",
        "HtmlElement",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "color-swatch-picker public module should not expose `{forbidden}`."
        );
        assert!(
            !lib_source.contains(forbidden),
            "color-swatch-picker crate entry should not expose `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_picker_component_layer_keeps_file_responsibilities() {
    let mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");
    let styles_source = load_source("../../components/color-swatch-picker/src/styles.rs");

    for needle in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::ColorSwatchPicker;",
        "pub use motion::ColorSwatchPickerMotion;",
    ] {
        assert!(
            mod_source.contains(needle),
            "color-swatch-picker module boundary should include `{needle}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::swatch_picker::{",
        "pub const DEFAULT_ID_BASE: &str = \"ui-color-swatch-picker\";",
        "pub fn normalize_id_base(value: Option<String>) -> String",
        "pub fn resolve_selection_mode_attr(is_controlled: bool) -> &'static str",
        "pub fn resolve_selection_init_source_attr(",
        "pub fn resolve_selection_source_attr(",
        "pub fn normalize_items(",
        "pub fn count_disabled_items(items: &[ColorSwatchPickerItem]) -> usize",
        "pub fn is_item_disabled_at(",
        "pub fn resolve_option_disabled(is_disabled: bool, item_disabled: bool) -> bool",
        "pub fn resolve_option_tabindex(",
        "pub fn resolve_component_state(",
        "pub fn resolve_state(input: ColorSwatchPickerStateInput) -> ColorSwatchPickerState",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "color-swatch-picker logic should include `{needle}`."
        );
    }
    for forbidden in ["use leptos", "web_sys::", "wasm_bindgen", "use ui_headless"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay platform-agnostic; found `{forbidden}`."
        );
    }

    for needle in [
        "let aria = use_radio(RadioOptions {",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional, default = true)] is_bordered: bool",
        "#[prop(optional)] size: ColorSwatchSize",
        "#[prop(optional)] rounding: ColorSwatchRounding",
        "#[prop(optional)] shape: ColorSwatchShape",
        "#[prop(optional, into)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_color: Option<String>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "let id_base = logic::normalize_id_base(id_base);",
        "let selected_state = overlay_open::use_controllable_state(",
        "selected_color,",
        "Some(logic::sanitize_selected_color(default_selected_color)),",
        "on_selected_change,",
        "let selection_mode_attr = logic::resolve_selection_mode_attr(is_controlled);",
        "logic::resolve_selection_init_source_attr(is_controlled, has_default_selected_color);",
        "logic::resolve_selection_source_attr(",
        "selected_state_request_change.run(next);",
        "logic::resolve_component_state(",
        "logic::is_item_disabled_at(is_disabled, &items.get_untracked(), index)",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "style=move || style_vars.get_value()",
        "logic::resolve_option_disabled(is_disabled, item.disabled);",
        "logic::resolve_option_tabindex(",
        "is_bordered=is_bordered",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should compose rendering/headless contracts; missing `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] disabled: bool",
        "#[prop(optional, default = true)] bordered: bool",
        "#[prop(optional, into)] size: Option<String>",
        "#[prop(optional, into)] rounding: Option<String>",
        "#[prop(optional, into)] shape: Option<String>",
        "selected_state.value.set(",
        "unwrap_or_else(|| \"ui-color-swatch-picker\".to_string())",
        "let disabled_item_count = items.iter().filter(|item| item.disabled).count();",
        "let option_disabled = is_disabled || item.disabled;",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should avoid legacy bool prop alias `{forbidden}`."
        );
    }

    for needle in [
        "pub fn sanitize_motion(motion: ColorSwatchPickerMotion) -> ColorSwatchPickerMotion",
        "pub fn attach_motion(base_vars: Option<String>, motion: ColorSwatchPickerMotion) -> String",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should map to shared ui-motion contract via `{needle}`."
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str"),
        "styles.rs should own static css output.",
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "styles.rs should consume ui-theme variables via `var(--ui-*)`.",
    );
}

#[test]
fn color_swatch_picker_state_primitive_source_boundary_is_enforced() {
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");

    for needle in [
        "ui_state_primitives::swatch_picker::normalize_items(items)",
        "ui_state_primitives::swatch_picker::normalize_aria_label(value)",
        "ui_state_primitives::swatch_picker::sanitize_selected_color(selected_color)",
        "ui_state_primitives::swatch_picker::resolve_selected_index(items, selected_color)",
        "ui_state_primitives::swatch_picker::resolve_selected_color(items, selected_index)",
        "ui_state_primitives::swatch_picker::resolve_option_label(item, index)",
        "ui_state_primitives::swatch_picker::resolve_state(input)",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should source state primitives from ui-state-primitives via `{needle}`."
        );
    }

    assert!(
        view_source.contains("overlay_open::use_controllable_state("),
        "view.rs should consume controlled/uncontrolled state via bridge adapter."
    );

    for forbidden in ["use leptos::", "Signal<", "RwSignal<", "WriteSignal<"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should remain primitive-focused and avoid framework-bound state `{forbidden}`."
        );
    }

    for forbidden in [
        "use crate::store",
        "::store::",
        "GlobalStore",
        "AppStore",
        "redux",
        "zustand",
        "pinia",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not couple directly to business store `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not couple directly to business store `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_picker_has_no_async_interaction_contract() {
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "aria_busy",
        "use_async_action",
        "on_retry",
        "on_error",
        "create_resource",
        "spawn_local",
        "tokio",
        "reqwest",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not define async interaction protocol `{forbidden}` for this component."
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not define async interaction protocol `{forbidden}` for this component."
        );
    }
}

#[test]
fn color_swatch_picker_dx_paradox_keeps_hello_world_simple() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "swatches: ReadSignal<Vec<ColorSwatchPickerItem>>",
        "#[prop(optional, into)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_color: Option<String>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep simple-by-default API and expose `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional, into)] state:",
        "state: ColorSwatchPickerState",
        "state=signal(",
        "state=move ||",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not require internal state object for basic usage; found `{forbidden}`."
        );
    }

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<ColorSwatchPicker",
        "swatches=signal(vec![ColorSwatchPickerItem::named(\"#f80\", \"Orange\")]).0",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs hello-world path should include `{needle}`.",
        );
    }
}

#[test]
fn color_swatch_picker_composite_api_uses_typed_item_spec_only() {
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "SwatchPickerItem as ColorSwatchPickerItem",
        "swatches: ReadSignal<Vec<ColorSwatchPickerItem>>",
        "ColorSwatchPickerItem::named(\"#A00\", \"Red\")",
        "ColorSwatchPickerItem::new(\"#08f\")",
    ] {
        let in_logic = logic_source.contains(needle);
        let in_view = view_source.contains(needle);
        let in_docs = docs_source.contains(needle);
        assert!(
            in_logic || in_view || in_docs,
            "composite API contract should be backed by typed item spec evidence `{needle}`.",
        );
    }

    for forbidden in [
        "labels: ReadSignal<Vec<String>>",
        "titles: ReadSignal<Vec<String>>",
        "panels: ReadSignal<Vec<String>>",
        "children: Children",
        "children: ChildrenFn",
        "#[prop(optional, into)] labels:",
        "#[prop(optional, into)] titles:",
        "#[prop(optional, into)] panels:",
        "#[prop(optional)] children:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not expose parallel-array or implicit-slot API `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_picker_macro_micro_duality_is_not_applicable_without_dragging() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");

    for forbidden in [
        "on:pointerdown",
        "on:pointermove",
        "on:pointerup",
        "on:dragstart",
        "on:drag",
        "on:dragend",
        "Dragging",
        "Action::DragEnd",
        "requestAnimationFrame",
        "request_animation_frame",
        "velocity",
        "inertia",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not introduce drag-loop mechanics `{forbidden}` for this component."
        );
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not model drag-loop convergence `{forbidden}` for this component."
        );
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not model drag physics `{forbidden}` for this component."
        );
    }

    assert!(
        view_source.contains("on:click=move |_| aria.handlers.on_radio_click.run(index)"),
        "component should keep discrete click-based selection interaction.",
    );
}

#[test]
fn color_swatch_picker_two_pass_geometry_is_not_applicable_without_dom_measurement() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");

    for forbidden in [
        "getBoundingClientRect",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "scrollWidth",
        "scrollHeight",
        "ResizeObserver",
        "IntersectionObserver",
        "Intent",
        "Measure(view)",
        "Rectification(logic)",
        "Rectification",
        "set_rect",
        "placement",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not include DOM measurement two-pass loop artifact `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not include geometry rectification loop artifact `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_picker_registration_protocol_is_not_applicable_without_dynamic_items() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");

    for forbidden in [
        "RegistrationContext",
        "Register(",
        "Unregister(",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not include dynamic registration protocol artifact `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not include dynamic registration protocol artifact `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("swatches: ReadSignal<Vec<ColorSwatchPickerItem>>"),
        "component should consume typed swatches list as ordered source.",
    );
    assert!(
        view_source.contains("items")
            && view_source.contains(".into_iter()")
            && view_source.contains(".enumerate()"),
        "component should render options by list order, not set iteration order.",
    );
}

#[test]
fn color_swatch_picker_slot_projection_is_not_applicable_without_container_slots() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "notify_hidden",
        "on_hidden",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not include slot projection lifecycle artifact `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not include slot projection lifecycle artifact `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not include slot projection lifecycle artifact `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("items")
            && view_source.contains(".into_iter()")
            && view_source.contains(".enumerate()"),
        "component should render swatches eagerly from ordered list input.",
    );
}

#[test]
fn color_swatch_picker_env_streams_are_not_applicable_without_environment_subscriptions() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "on:resize",
        "on:scroll",
        "debounce",
        "throttle",
        "BreakpointChanged",
        "ThemeChanged",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not include env stream sampling artifact `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not include env stream action artifact `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("on:click=move |_| aria.handlers.on_radio_click.run(index)"),
        "component should keep explicit user-triggered selection without env stream side-channel.",
    );
}

#[test]
fn color_swatch_picker_event_light_cone_is_not_applicable_without_bulk_collection_ops() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");

    for forbidden in [
        "Context Bus",
        "ContextBus",
        "Selector",
        "SelectionState::All",
        "prop drilling",
        "prop_drilling",
        "bulk_select",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not include large-collection event-light-cone artifact `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not include large-collection event-light-cone artifact `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("swatches: ReadSignal<Vec<ColorSwatchPickerItem>>"),
        "component should keep direct typed swatch list input instead of context-bus fanout.",
    );
    assert!(
        view_source.contains("items")
            && view_source.contains(".into_iter()")
            && view_source.contains(".enumerate()"),
        "component should render by direct ordered enumeration without selector compression protocol.",
    );
}

#[test]
fn color_swatch_picker_causality_bus_is_not_applicable_without_derived_bus_workflow() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "causality_bus",
        "broadcast(",
        "subscriber",
        "派生命令",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not include causality-bus artifact `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not include causality-bus artifact `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("on:click=move |_| aria.handlers.on_radio_click.run(index)"),
        "component should keep direct user-event to local-state convergence path.",
    );
    assert!(
        logic_source.contains("resolve_state(ColorSwatchPickerStateInput {"),
        "logic should converge selection locally through primitive state resolution, not derived bus broadcast.",
    );
}

#[test]
fn color_swatch_picker_a11y_i18n_contract_is_headless_driven_and_no_view_text_hardcode() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");

    for needle in [
        "use ui_headless::{A11yDirection, RadioGroupOptions, RadioOptions, RovingOrientation, use_radio};",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "let aria = use_radio(RadioOptions {",
        "lang,",
        "dir,",
        "role=aria.attrs.role",
        "aria-label=aria_label",
        "lang=aria.attrs.lang",
        "dir=aria.attrs.dir",
        "aria-label=option_label_for_button.clone()",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should mount headless/i18n contract `{needle}`."
        );
    }

    for needle in [
        "ui_state_primitives::swatch_picker::normalize_aria_label(value)",
        "ui_state_primitives::swatch_picker::resolve_option_label(item, index)",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should source text fallback primitive from ui-state-primitives `{needle}`."
        );
    }

    for forbidden in [
        "DEFAULT_ARIA_LABEL",
        "Color swatches",
        "format!(\"Color {}",
        "fn normalize_aria_label(",
        "fn resolve_option_label(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not hardcode user-facing text or duplicate a11y text helpers `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_picker_state_markers_are_observable_and_source_enums_are_closed() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");

    for attr in [
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
        "data-selection-mode=selection_mode_attr",
        "data-selection-init-source=selection_init_source_attr",
        "data-selection-source=move || selection_source.get()",
        "data-index=index",
    ] {
        assert!(
            view_source.contains(attr),
            "view.rs should expose stable observable marker `{attr}`."
        );
    }

    for enum_value in [
        "\"controlled\"",
        "\"uncontrolled\"",
        "\"external\"",
        "\"default\"",
        "\"internal\"",
        "\"interaction\"",
    ] {
        assert!(
            logic_source.contains(enum_value),
            "logic.rs source-marker values should stay within closed enum set and include {enum_value}."
        );
    }

    for needle in [
        "pub fn resolve_selection_mode_attr(is_controlled: bool) -> &'static str",
        "pub fn resolve_selection_init_source_attr(",
        "pub fn resolve_selection_source_attr(",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should centralize source marker derivation helper `{needle}`."
        );
    }
}

#[test]
fn color_swatch_picker_styles_depend_on_explicit_markers_and_runtime_vars_only() {
    let styles_source = load_source("../../components/color-swatch-picker/src/styles.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");

    for selector in [
        ".ui-color-swatch-picker__option[data-selected=\"true\"]",
        ".ui-color-swatch-picker__option[data-disabled=\"true\"]",
        ".ui-color-swatch-picker[data-empty=\"true\"] .ui-color-swatch-picker__list",
        ".ui-color-swatch-picker[data-custom-class=\"true\"]",
        ".ui-color-swatch-picker[data-motion-source=\"custom\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "styles.rs should branch visual states via explicit marker selector `{selector}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not depend on fragile structure selector `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("style=move || style_vars.get_value()"),
        "view.rs runtime style should attach CSS variable payload only.",
    );

    let style_attr_count = view_source.match_indices("style=").count();
    assert_eq!(
        style_attr_count, 1,
        "view.rs should keep a single style entrypoint for CSS variable injection."
    );

    for needle in [
        "--ui-color-swatch-picker-transition-ms",
        "--ui-color-swatch-picker-focus-ring-width",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should only expose runtime CSS variables like `{needle}`."
        );
    }

    for forbidden in ["background:", "color-mix(", "box-shadow:", "border:"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not inject business style rule `{forbidden}` at runtime."
        );
    }
}

#[test]
fn color_swatch_picker_token_first_style_pipeline_is_wired_through_css_and_ui_root() {
    let styles_source = load_source("../../components/color-swatch-picker/src/styles.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let css_source = load_source("../../crates/ui-components/src/css.rs");
    let root_source = load_source("../../crates/ui-components/src/root.rs");

    assert!(
        styles_source.contains("pub const CSS: &str"),
        "styles.rs should remain the static CSS aggregation point for this component."
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "styles.rs should consume ui-theme token variables via `var(--ui-*)`."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-color_swatch_picker\")]")
            && css_source.contains("out.push_str(crate::color::swatch_picker::styles::CSS);"),
        "ui-components css registry should aggregate color-swatch-picker styles via feature-gated styles.rs export."
    );
    assert!(
        root_source.contains("if inject_components_css.get_value() {")
            && root_source.contains("crate::css::push_components_css(&mut out);"),
        "UiRoot should inject component CSS through css.rs aggregation path."
    );
    assert!(
        view_source.contains("style=move || style_vars.get_value()"),
        "view.rs runtime style should only carry CSS variable payload."
    );

    for forbidden in [
        "tailwind",
        "tw-",
        "stylist::",
        "leptos_style",
        "stylex",
        "emotion",
        "styled_components",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not depend on utility-first/CSS-in-Rust marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not depend on utility-first/CSS-in-Rust marker `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_picker_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("../../components/color-swatch-picker/src/styles.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
        "var(--ui-color-swatch-border-width, var(--ui-fallback-color-swatch-border-width))",
        "var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width))",
        "var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity))",
        "var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))",
        "var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing))",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep defensive double-fallback token `{required}`."
        );
    }

    for required in [
        "--ui-fallback-space-xs:",
        "--ui-fallback-bg:",
        "--ui-fallback-accent:",
        "--ui-fallback-radius-sm:",
        "--ui-fallback-color-swatch-border-width:",
        "--ui-fallback-button-focus-outline-width:",
        "--ui-fallback-checkbox-disabled-opacity:",
        "--ui-fallback-text-field-motion-duration:",
        "--ui-fallback-text-field-motion-easing:",
        "--ui-fallback-component-height-100:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme css should provide fallback terminal `{required}`."
        );
    }

    for forbidden in [
        "1px color-mix(",
        "3px color-mix(",
        "var(--ui-color-swatch-picker-selected-border-width, 1px)",
        "var(--ui-color-swatch-picker-selected-ring-width, 3px)",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should avoid hard-coded terminal size `{forbidden}`."
        );
    }
    assert!(
        !styles_source.contains("px;"),
        "styles.rs should avoid raw px terminal values; route terminals through ui-theme fallbacks."
    );
    assert!(
        !contains_hex_color_literal(styles_source),
        "styles.rs should avoid hard-coded hex literals."
    );

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
            "color_swatch_picker_styles_use_defensive_variable_fallback_chain",
            "scripts/check-ui-components-contract-hygiene.sh",
        ] {
            assert!(
                source.contains(required),
                "check2 should keep defensive-variable governance marker `{required}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_cascade_layer_and_runtime_style_contract_is_enforced() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let css_source = load_source("../../crates/ui-components/src/css.rs");
    let root_source = load_source("../../crates/ui-components/src/root.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-color_swatch_picker\")]",
        "out.push_str(crate::color::swatch_picker::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(required),
            "css.rs should keep cascade-layer marker `{required}`."
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should keep component CSS injection marker `{required}`."
        );
    }

    for required in [
        "style=move || style_vars.get_value()",
        "let style_vars = StoredValue::new(motion::attach_motion(None, motion));",
        "--ui-color-swatch-picker-transition-ms",
        "--ui-color-swatch-picker-focus-ring-width",
    ] {
        assert!(
            view_source.contains(required)
                || logic_source.contains(required)
                || motion_source.contains(required),
            "runtime style path should keep css-variable-only marker `{required}`."
        );
    }

    let style_attr_count = view_source.match_indices("style=").count();
    assert_eq!(
        style_attr_count, 1,
        "view.rs should keep a single style entrypoint for CSS variable injection."
    );

    for source in [view_source, logic_source, motion_source] {
        for forbidden in [
            "style:top",
            "style:left",
            "style:right",
            "style:bottom",
            "style:width",
            "style:height",
            "style:margin",
            "style:padding",
            "style:background",
            "style:border",
            "style:color",
            "style=\"top:",
            "style=\"left:",
            "style=\"right:",
            "style=\"bottom:",
            "style=\"width:",
            "style=\"height:",
            "style=\"margin:",
            "style=\"padding:",
            "style=\"background:",
            "style=\"border:",
            "style=\"color:",
        ] {
            assert!(
                !source.contains(forbidden),
                "runtime style path should avoid non-variable inline style marker `{forbidden}`."
            );
        }
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`."
    );

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
            "color_swatch_picker_cascade_layer_and_runtime_style_contract_is_enforced",
            "scripts/check-ui-components-contract-hygiene.sh",
        ] {
            assert!(
                source.contains(required),
                "check2 should keep cascade-layer governance marker `{required}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_visual_desire_gate_reuses_theme_visual_baseline_and_heroui_contracts() {
    let picker_docs =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let e2e_spec = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_doc = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "title=\"ColorSwatchPicker\"",
        "slug=\"color-swatch-picker\"",
        "baseline-compatible selectable swatch group",
    ] {
        assert!(
            picker_docs.contains(needle),
            "color-swatch-picker docs should keep visual-baseline entry `{needle}`."
        );
    }

    for needle in ["\"ThemeVisualBaseline\"", "\"theme-visual-baseline\""] {
        assert!(
            pages_registry.contains(needle),
            "docs pages registry should expose theme visual baseline entry `{needle}`."
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "hover/active/focus",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            baseline_page.contains(needle),
            "theme visual baseline page should keep visual-quality contract token `{needle}`."
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "theme visual baseline renders button/input/overlay",
        "theme visual baseline screenshots",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
    ] {
        assert!(
            e2e_spec.contains(needle),
            "theme visual baseline e2e contract should include `{needle}`."
        );
    }

    for needle in ["# HeroUI 参数设计风格对齐策略", "HeroUI 对齐结论"] {
        assert!(
            heroui_doc.contains(needle),
            "HeroUI strategy doc should keep visual-alignment contract token `{needle}`."
        );
    }
}

#[test]
fn color_swatch_picker_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("../../crates/ui-components/Cargo.toml");
    let lib_source = load_source("../../components/color-swatch-picker/src/lib.rs");
    let css_source = load_source("../../crates/ui-components/src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "component-color_swatch_picker = []",
        "inject-css = []",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components feature map should include `{needle}` for tree-shaking boundaries."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-color_swatch_picker\")]")
            && lib_source.contains("pub mod color_swatch_picker;"),
        "lib.rs should feature-gate color_swatch_picker module export for tree-shaking."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-color_swatch_picker\")]")
            && css_source.contains("out.push_str(crate::color::swatch_picker::styles::CSS);"),
        "css.rs should gate color_swatch_picker CSS aggregation behind component-color_swatch_picker feature."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)"),
        "css.rs should keep top-level inject-css gate for component CSS injection."
    );

    for forbidden in ["component_registry", "ALL_COMPONENTS_MAP", "lazy_static!"] {
        assert!(
            !lib_source.contains(forbidden) && !css_source.contains(forbidden),
            "tree-shaking contract should avoid global keep-alive registries `{forbidden}`."
        );
    }

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
fn color_swatch_picker_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "COLOR_SWATCH_PICKER_MIN_FEATURES=\"component-color_swatch_picker,inject-css\"",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_tree_shaking_keeps_component_feature_and_css_boundaries",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_tree_shaking_check_script_covers_feature_tree_wasm_and_budget",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_marks_tree_shaking_contract_complete",
        "COLOR_SWATCH_PICKER_TREE_OUTPUT",
        "if grep -q 'all-components' <<<\"$COLOR_SWATCH_PICKER_TREE_OUTPUT\";",
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
fn color_swatch_picker_check2_marks_tree_shaking_contract_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        assert!(
            source.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
            "color-swatch-picker check2 should mark tree-shaking item complete."
        );
        assert!(
            source.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
            "color-swatch-picker check2 should mark tree-shaking feature-pruning checklist item complete."
        );

        for needle in [
            "color_swatch_picker_tree_shaking_keeps_component_feature_and_css_boundaries",
            "color_swatch_picker_tree_shaking_check_script_covers_feature_tree_wasm_and_budget",
            "color_swatch_picker_check2_marks_tree_shaking_contract_complete",
            "cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-color_swatch_picker,inject-css",
            "cargo tree -e features -i ui-components -p web-demo",
            "bash ./scripts/check-ui-components-tree-shaking.sh",
        ] {
            assert!(
                source.contains(needle),
                "color-swatch-picker check2 tree-shaking section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_focus_stack_gc_is_not_applicable_without_overlay_layering() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");

    for forbidden in [
        "NodeRef",
        "node_ref",
        "document.body",
        "document().body",
        "focus_manager",
        "FocusManager",
        "fallback_to",
        "FallbackTo",
        "role=\"dialog\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "focus stack gc N/A contract requires no overlay focus-restore path `{forbidden}` in view.rs."
        );
        assert!(
            !logic_source.contains(forbidden),
            "focus stack gc N/A contract requires no overlay focus-restore path `{forbidden}` in logic.rs."
        );
        assert!(
            !motion_source.contains(forbidden),
            "focus stack gc N/A contract requires no overlay focus-restore path `{forbidden}` in motion.rs."
        );
    }

    for needle in [
        "role=aria.attrs.role",
        "role=\"radio\"",
        "on:focus=move |_| aria.handlers.on_radio_focus.run(index)",
    ] {
        assert!(
            view_source.contains(needle),
            "color-swatch-picker should remain a radio-group focus model via `{needle}`."
        );
    }
}

#[test]
fn color_swatch_picker_check2_marks_focus_stack_gc_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        assert!(
            source.contains(
                "- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。"
            ),
            "color-swatch-picker check2 should mark focus stack gc item complete."
        );
        for needle in [
            "N/A 理由：`ColorSwatchPicker`",
            "overlay",
            "未私存 `NodeRef`",
            "焦点回落 `document.body`",
            "color_swatch_picker_focus_stack_gc_is_not_applicable_without_overlay_layering",
        ] {
            assert!(
                source.contains(needle),
                "focus stack gc check2 section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_escape_hatches_foreign_zone_not_applicable_without_third_party_imperative_instances()
 {
    let mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "GoogleMap",
        "ForeignZone",
        "foreign_zone",
        "YieldControl",
        "CleanupForeign",
        "Box<dyn",
        "JsValue",
        "web_sys::HtmlCanvasElement",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "escape-hatch N/A contract requires no third-party imperative instance exposure `{forbidden}` in mod.rs."
        );
        assert!(
            !view_source.contains(forbidden),
            "escape-hatch N/A contract requires no third-party imperative instance wiring `{forbidden}` in view.rs."
        );
        assert!(
            !logic_source.contains(forbidden),
            "escape-hatch N/A contract requires no third-party imperative instance state pollution `{forbidden}` in logic.rs."
        );
        assert!(
            !motion_source.contains(forbidden),
            "escape-hatch N/A contract requires no third-party imperative instance lifecycle `{forbidden}` in motion.rs."
        );
    }

    for needle in [
        "let selected_state = overlay_open::use_controllable_state(",
        "let aria = use_radio(RadioOptions {",
        "logic::resolve_component_state(",
    ] {
        assert!(
            view_source.contains(needle),
            "component should remain on internal primitives/headless pipeline via `{needle}`."
        );
    }
}

#[test]
fn color_swatch_picker_check2_marks_escape_hatches_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        assert!(
            source.contains(
                "- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。"
            ),
            "color-swatch-picker check2 should mark escape-hatches item complete."
        );
        for needle in [
            "N/A 理由：`ColorSwatchPicker`",
            "不集成 ECharts/Map",
            "Foreign Zone",
            "YieldControl/CleanupForeign",
            "color_swatch_picker_escape_hatches_foreign_zone_not_applicable_without_third_party_imperative_instances",
        ] {
            assert!(
                source.contains(needle),
                "escape-hatches check2 section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_hydration_discontinuity_avoids_time_and_random_id_initialization() {
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");

    for forbidden in [
        "now()",
        "SystemTime::now",
        "Instant::now",
        "rand::",
        "thread_rng",
        "uuid::",
        "Uuid::new",
        "js_sys::Date",
        "Math::random",
        "random_uuid",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "hydration path should avoid nondeterministic id initialization `{forbidden}` in logic.rs."
        );
        assert!(
            !view_source.contains(forbidden),
            "hydration path should avoid nondeterministic id initialization `{forbidden}` in view.rs."
        );
    }

    for needle in [
        "pub const DEFAULT_ID_BASE: &str = \"ui-color-swatch-picker\";",
        "pub fn normalize_id_base(value: Option<String>) -> String",
        "normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ID_BASE.to_string())",
        "let id_base = logic::normalize_id_base(id_base);",
        "id=id_base",
        "id_base: id_base.clone(),",
        "id=aria.state.radio_id.run(index)",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "hydration-stable id pipeline should include `{needle}`."
        );
    }
}

#[test]
fn color_swatch_picker_check2_marks_hydration_discontinuity_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        assert!(
            source.contains(
                "- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。"
            ),
            "color-swatch-picker check2 should mark hydration discontinuity item complete."
        );
        for needle in [
            "logic::normalize_id_base",
            "DEFAULT_ID_BASE",
            "id=id_base",
            "color_swatch_picker_hydration_discontinuity_avoids_time_and_random_id_initialization",
        ] {
            assert!(
                source.contains(needle),
                "hydration-discontinuity check2 section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_ssr_cross_platform_contract_keeps_compile_only_matrix_and_feature_boundaries()
 {
    let mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");
    let ui_components_cargo = load_source("../../crates/ui-components/Cargo.toml");

    for forbidden in [
        "web_sys::",
        "wasm_bindgen",
        "js_sys::",
        "window.",
        "document.",
        "HtmlElement",
        "HtmlInputElement",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "cross-platform contract forbids browser-only dependency leakage `{forbidden}` in mod.rs."
        );
        assert!(
            !view_source.contains(forbidden),
            "cross-platform contract forbids browser-only dependency leakage `{forbidden}` in view.rs."
        );
        assert!(
            !logic_source.contains(forbidden),
            "cross-platform contract forbids browser-only dependency leakage `{forbidden}` in logic.rs."
        );
        assert!(
            !motion_source.contains(forbidden),
            "cross-platform contract forbids browser-only dependency leakage `{forbidden}` in motion.rs."
        );
    }

    for needle in [
        "let aria = use_radio(RadioOptions {",
        "use ui_headless as overlay_open;",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            view_source.contains(needle) || motion_source.contains(needle),
            "component should use feature-managed abstractions via `{needle}`."
        );
    }

    for needle in [
        "component-color_swatch_picker = []",
        "[target.'cfg(target_arch = \"wasm32\")'.dependencies]",
        "ui-headless = { path = \"../ui-headless\" }",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components cargo feature/cfg boundary should include `{needle}`."
        );
    }
}

#[test]
fn color_swatch_picker_check2_marks_ssr_cross_platform_contract_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        assert!(
            source.contains(
                "- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。"
            ),
            "color-swatch-picker check2 should mark SSR cross-platform item complete."
        );
        for needle in [
            "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-color_swatch_picker,inject-css",
            "cargo check -p ui-headless --no-default-features --features ssr",
            "cargo check -p ui-components",
            "color_swatch_picker_ssr_cross_platform_contract_keeps_compile_only_matrix_and_feature_boundaries",
        ] {
            assert!(
                source.contains(needle),
                "SSR cross-platform check2 section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_ui_headless_web_ssr_mutex_compile_error_guard_is_preserved() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let ui_headless_source = load_source("../../crates/ui-headless/src/lib.rs");
    let ui_components_cargo = load_source("../../crates/ui-components/Cargo.toml");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_source.contains(needle),
            "ui-headless mutex guard should include `{needle}`."
        );
    }

    for needle in [
        "use ui_headless as overlay_open;",
        "use ui_headless::{A11yDirection, RadioGroupOptions, RadioOptions, RovingOrientation, use_radio};",
    ] {
        assert!(
            view_source.contains(needle),
            "component should consume ui-headless contract via `{needle}`."
        );
    }

    for forbidden in [
        "ui-headless = { path = \"../ui-headless\", features = [\"web\", \"ssr\"]",
        "ui-headless = { path = \"../ui-headless\", default-features = false, features = [\"web\", \"ssr\"]",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "ui-components dependency should not force illegal headless feature pair `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_picker_check2_marks_ui_headless_web_ssr_mutex_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        assert!(
            source.contains(
                "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。"
            ),
            "color-swatch-picker check2 should mark ui-headless web/ssr mutex item complete."
        );
        for needle in [
            "cargo check -p ui-headless --no-default-features --features web",
            "cargo check -p ui-headless --no-default-features --features ssr",
            "cargo check -p ui-headless --no-default-features --features web,ssr",
            "color_swatch_picker_ui_headless_web_ssr_mutex_compile_error_guard_is_preserved",
        ] {
            assert!(
                source.contains(needle),
                "ui-headless web/ssr mutex check2 section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_ui_motion_non_wasm_noop_stub_contract_is_preserved() {
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");
    let ui_motion_source = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`."
        );
    }

    for needle in [
        "ui_motion::web::prefers_reduced_motion()",
        "resolve_effective_motion(motion, ui_motion::web::prefers_reduced_motion())",
    ] {
        assert!(
            motion_source.contains(needle),
            "component motion should consume ui-motion reduced-motion stub via `{needle}`."
        );
    }

    for forbidden in ["panic!(", ".expect(", "unwrap()"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs non-wasm path should avoid panic-prone assumption `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_picker_check2_marks_ui_motion_non_wasm_noop_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        assert!(
            source.contains(
                "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。"
            ),
            "color-swatch-picker check2 should mark ui-motion non-wasm noop item complete."
        );
        for needle in [
            "cargo check -p ui-motion",
            "cargo check -p ui-components --no-default-features --features component-color_swatch_picker,inject-css",
            "color_swatch_picker_ui_motion_non_wasm_noop_stub_contract_is_preserved",
        ] {
            assert!(
                source.contains(needle),
                "ui-motion non-wasm noop check2 section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_reduced_motion_ssr_wasm_branches_preserve_semantics_contract() {
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let ui_motion_source = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "pub fn resolve_effective_motion(",
        "if prefers_reduced_motion || !motion.enabled {",
        "transition_ms: 1,",
        "ui_motion::web::prefers_reduced_motion()",
        "resolve_effective_motion(motion, ui_motion::web::prefers_reduced_motion())",
    ] {
        assert!(
            motion_source.contains(needle),
            "reduced-motion branch contract should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web;",
        "pub mod web {",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion should expose wasm/non-wasm backend split via `{needle}`."
        );
    }

    for needle in [
        "role=aria.attrs.role",
        "data-state=move || state.get().data_state_attr",
        "data-selection-source=move || selection_source.get()",
        "let id_base = logic::normalize_id_base(id_base);",
        "id=id_base",
    ] {
        assert!(
            view_source.contains(needle),
            "SSR/wasm semantic parity should preserve `{needle}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "web_sys::",
        "wasm_bindgen",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view semantics should not split by platform via `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "logic semantics should not split by platform via `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_picker_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe()
{
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");

    for needle in [
        "pub struct ColorSwatchPickerMotion {",
        "pub enabled: bool,",
        "pub spring: ui_motion::spring::SpringConfig,",
        "use ui_theme::{default_swatch_motion_tokens, default_text_field_motion_tokens};",
        "let swatch_tokens = default_swatch_motion_tokens();",
        "stiffness: swatch_tokens.spring.stiffness,",
        "damping: swatch_tokens.spring.damping,",
        "mass: swatch_tokens.spring.mass,",
        "precision: swatch_tokens.spring.precision,",
        "pub fn sanitize_motion(motion: ColorSwatchPickerMotion) -> ColorSwatchPickerMotion {",
        "ui_motion::spring::sanitize_config(motion.spring, default.spring)",
        "pub fn attach_motion(base_vars: Option<String>, motion: ColorSwatchPickerMotion) -> String",
        "resolve_effective_motion(motion, ui_motion::web::prefers_reduced_motion())",
        "if prefers_reduced_motion || !motion.enabled {",
        "--ui-color-swatch-picker-spring-stiffness",
        "--ui-color-swatch-picker-spring-damping",
        "--ui-color-swatch-picker-spring-mass",
        "--ui-color-swatch-picker-spring-precision",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion contract should keep `{needle}`."
        );
    }

    assert!(
        view_source
            .contains("let style_vars = StoredValue::new(motion::attach_motion(None, motion));")
            && view_source.contains("style=move || style_vars.get_value()"),
        "view.rs should mount motion contract through attach_motion + css variable style entrypoint."
    );

    for forbidden in [
        "web_sys::",
        "wasm_bindgen",
        "panic!(",
        ".expect(",
        "unwrap()",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion contract should keep non-wasm/SSR-safe implementation without `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        script_source.contains(script_needle),
        "platform gate script should include `{script_needle}`."
    );

    for source in [check2_source, check2_source_mirror] {
        for needle in [
            "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
            "color_swatch_picker_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
            "scripts/check-ui-components-platforms.sh",
        ] {
            assert!(
                source.contains(needle),
                "checklist should keep motion-contract governance marker `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_check2_marks_reduced_motion_ssr_wasm_branch_coverage_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        assert!(
            source.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。"),
            "color-swatch-picker check2 should mark reduced-motion/SSR/wasm branch item complete."
        );
        for needle in [
            "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-color_swatch_picker,inject-css",
            "cargo check -p ui-components --no-default-features --features component-color_swatch_picker,inject-css",
            "color_swatch_picker_reduced_motion_ssr_wasm_branches_preserve_semantics_contract",
        ] {
            assert!(
                source.contains(needle),
                "reduced-motion/SSR/wasm check2 section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let styles_source = load_source("../../components/color-swatch-picker/src/styles.rs");

    for needle in [
        "component_doc!(",
        "\"ColorSwatchPicker\"",
        "\"color-swatch-picker\"",
        "display_extra::color_swatch_picker",
    ] {
        assert!(
            pages_source.contains(needle),
            "ColorSwatchPicker docs page should stay in component coverage traversal via `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn color_swatch_picker() -> AnyView",
        "title=\"ColorSwatchPicker\"",
        "slug=\"color-swatch-picker\"",
        "<ComponentPage",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "ColorSwatchPicker docs page should mount through ComponentPage contract `{needle}`."
        );
    }

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "max_update_ms: Some(10.0),",
        "\"color-swatch-picker\" => UiPerfBudget {",
        "max_mount_ms: 32.0,",
        "max_update_ms: Some(11.0),",
        "max_heap_kb: Some(576.0),",
        "let perf_budget = component_page_perf_budget(slug);",
        "let perf_name = format!(\"ComponentPage::{slug}\");",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "component shell should keep performance budget/probe marker `{needle}`."
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
            e2e_source.contains(needle),
            "docs coverage e2e should enforce perf regression guard `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance follow-up plan should keep `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_performance_governance_contract_is_budgeted_traceable_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`."
    );
    assert!(
        script_source.contains(
            "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
        ),
        "performance gate script should keep render_count follow-up blocker."
    );

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-selection-source=move || selection_source.get()",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
        "data-selection-mode=selection_mode_attr",
        "data-selection-init-source=selection_init_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "--ui-color-swatch-picker-transition-ms",
        "--ui-color-swatch-picker-focus-ring-width",
    ] {
        assert!(
            view_source.contains(needle) || styles_source.contains(needle),
            "ColorSwatchPicker should expose performance attribution marker `{needle}`."
        );
    }

    for source in [check2_source, check2_source_mirror] {
        for needle in [
            "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
            "渲染次数预算为 `1`",
            "render_count",
            "等价证据",
            "color_swatch_picker_performance_governance_contract_is_budgeted_traceable_and_blocking",
        ] {
            assert!(
                source.contains(needle),
                "ColorSwatchPicker check2 should include performance governance evidence token `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let local_semantics = include_str!("semantics.rs");
    let legacy_semantics = load_source("legacy_semantics");
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for required_test in [
        "fn color_swatch_picker_semantic_test_matrix_covers_key_paths_without_snapshot_reliance()",
        "fn color_swatch_picker_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn color_swatch_picker_focus_stack_gc_is_not_applicable_without_overlay_layering()",
        "fn color_swatch_picker_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            local_semantics.contains(required_test) && legacy_semantics.contains(required_test),
            "semantic/performance regression suite should include `{required_test}` in local and aggregated tests."
        );
    }

    for marker in [
        "role=aria.attrs.role",
        "aria-checked=move || if is_selected() { \"true\" } else { \"false\" }",
        "data-state=move || state.get().data_state_attr",
        "data-selection-source=move || selection_source.get()",
        "on:focus=move |_| aria.handlers.on_radio_focus.run(index)",
        "on:keydown=on_key_down",
        "on:click=move |_| aria.handlers.on_radio_click.run(index)",
    ] {
        assert!(
            view_source.contains(marker),
            "ColorSwatchPicker view should expose semantic/focus marker `{marker}`."
        );
    }

    for required in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(required),
            "render_count tracking contract should keep `{required}`."
        );
    }

    for script_needle in [
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(script_needle),
            "performance check script should include `{script_needle}`."
        );
    }

    for source in [check2_source, check2_source_mirror] {
        for check_needle in [
            "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
            "color_swatch_picker_semantic_test_matrix_covers_key_paths_without_snapshot_reliance",
            "color_swatch_picker_performance_governance_contract_is_budgeted_traceable_and_blocking",
            "color_swatch_picker_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
            "render_count",
            "等价证据",
            "scripts/check-ui-components-performance.sh",
        ] {
            assert!(
                source.contains(check_needle),
                "ColorSwatchPicker check2 semantic/performance entry should keep `{check_needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_check2_marks_performance_governance_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        assert!(
            source.contains("- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。"),
            "color-swatch-picker check2 should mark performance governance item complete."
        );
        for needle in [
            "\"color-swatch-picker\" => UiPerfBudget {",
            "max_mount_ms: 32.0",
            "max_update_ms: Some(11.0)",
            "max_heap_kb: Some(576.0)",
            "scripts/check-ui-components-performance.sh",
            "color_swatch_picker_check2_marks_performance_governance_complete",
        ] {
            assert!(
                source.contains(needle),
                "performance-governance check2 section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for needle in [
        "let render_options = move || {",
        "{render_options}",
        ".into_iter()",
        ".enumerate()",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep semantic subrender split token `{needle}`."
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert_eq!(
        view_macro_count, 2,
        "view.rs should keep two bounded view! blocks (root + item); found {view_macro_count}."
    );
    assert!(
        view_source.lines().count() <= 260,
        "view.rs should stay within bounded macro footprint after split."
    );

    let root_view_start = view_source
        .rfind("view! {")
        .expect("view.rs should contain root view! block");
    let root_view = &view_source[root_view_start..];

    for needle in [
        "{render_options}",
        "data-slot=SLOT_COLOR_SWATCH_PICKER_LIST",
    ] {
        assert!(
            root_view.contains(needle),
            "root view! should compose semantic section via `{needle}`."
        );
    }

    for forbidden in [
        ".map(|(index, item)|",
        "view! {\n                                <button",
    ] {
        assert!(
            !root_view.contains(forbidden),
            "root view! should avoid deep inline rendering token `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );

    for source in [check2_source, check2_source_mirror] {
        for needle in [
            "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
            "render_options",
            "color_swatch_picker_view_macro_complexity_is_split_into_semantic_subrenders",
        ] {
            assert!(
                source.contains(needle),
                "check2 should keep view-macro complexity evidence token `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for needle in [
        "fn render_option_swatch(",
        ") -> impl IntoView {",
        "{render_option_swatch(",
        "pub fn ColorSwatchPicker(",
    ] {
        assert!(
            view_source.contains(needle),
            "function-first split should keep `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "ColorSwatchPicker should keep exactly one public component boundary."
    );

    let forbidden = "#[component]\nfn render_option_swatch(";
    assert!(
        !view_source.contains(forbidden),
        "lightweight local fragment should stay plain function and avoid `{forbidden}`."
    );

    for needle in [
        "const SLOT_COLOR_SWATCH_PICKER: &str = \"color-swatch-picker\";",
        "const SLOT_COLOR_SWATCH_PICKER_LIST: &str = \"color-swatch-picker-list\";",
        "const SLOT_COLOR_SWATCH_PICKER_OPTION: &str = \"color-swatch-picker-option\";",
        "const CLASS_COLOR_SWATCH_PICKER_OPTION: &str = \"ui-color-swatch-picker__option\";",
        "const ATTR_BUTTON_TYPE: &str = \"button\";",
        "const ATTR_ROLE_RADIO: &str = \"radio\";",
        "data-slot=SLOT_COLOR_SWATCH_PICKER",
        "data-slot=SLOT_COLOR_SWATCH_PICKER_LIST",
        "data-slot=SLOT_COLOR_SWATCH_PICKER_OPTION",
        "class=CLASS_COLOR_SWATCH_PICKER_OPTION",
        "type=ATTR_BUTTON_TYPE",
        "role=ATTR_ROLE_RADIO",
        "data-slot=\"color-swatch-picker\"",
        "data-selection-source=move || selection_source.get()",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
    ] {
        assert!(
            view_source.contains(needle),
            "functional split should preserve stable semantic marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );

    for source in [check2_source, check2_source_mirror] {
        for needle in [
            "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
            "render_option_swatch",
            "color_swatch_picker_view_functional_split_prefers_plain_functions_over_local_components",
        ] {
            assert!(
                source.contains(needle),
                "check2 should keep function-split evidence token `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_static_fragments_are_constantized_or_absent_for_simple_layout() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for needle in [
        "const SLOT_COLOR_SWATCH_PICKER: &str = \"color-swatch-picker\";",
        "const SLOT_COLOR_SWATCH_PICKER_LIST: &str = \"color-swatch-picker-list\";",
        "const SLOT_COLOR_SWATCH_PICKER_OPTION: &str = \"color-swatch-picker-option\";",
        "const CLASS_COLOR_SWATCH_PICKER_OPTION: &str = \"ui-color-swatch-picker__option\";",
        "const ATTR_BUTTON_TYPE: &str = \"button\";",
        "const ATTR_ROLE_RADIO: &str = \"radio\";",
        "data-slot=SLOT_COLOR_SWATCH_PICKER",
        "data-slot=SLOT_COLOR_SWATCH_PICKER_LIST",
        "data-slot=SLOT_COLOR_SWATCH_PICKER_OPTION",
        "class=CLASS_COLOR_SWATCH_PICKER_OPTION",
        "type=ATTR_BUTTON_TYPE",
        "role=ATTR_ROLE_RADIO",
        "aria-label=option_label_for_button.clone()",
    ] {
        assert!(
            view_source.contains(needle),
            "static fragment constantization should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "inner_html=",
        "dangerously_set_inner_html",
        "<svg",
        "<path",
        "<footer",
        "markdown_to_html(",
        "let markdown",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "simple swatch-picker layout should avoid heavy static fragment token `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_static_fragments_are_constantized_or_absent_for_simple_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );

    for source in [check2_source, check2_source_mirror] {
        for needle in [
            "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
            "color_swatch_picker_static_fragments_are_constantized_or_absent_for_simple_layout",
        ] {
            assert!(
                source.contains(needle),
                "check2 should keep static-fragment evidence token `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    for rel_path in [
        "../../components/color-swatch-picker/src/mod.rs",
        "../../components/color-swatch-picker/src/logic.rs",
        "../../components/color-swatch-picker/src/styles.rs",
        "../../components/color-swatch-picker/src/motion.rs",
        "../../components/color-swatch-picker/src/view.rs",
        "../../apps/docs-app/src/pages/components/pages/display_extra.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in [
            "inner_html=",
            "set_inner_html(",
            "dangerously_set_inner_html",
            "dangerouslySetInnerHTML",
            "markdown_to_html(",
            "format!(\"<",
        ] {
            assert!(
                !source.contains(forbidden),
                "color-swatch-picker path `{rel_path}` must reject raw html injection token `{forbidden}`.",
            );
        }
    }

    let docs_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    for needle in [
        "const ACCORDION_README_MD: &str = include_str!(",
        "const CHECKBOX_README_MD: &str = include_str!(",
        "const MODAL_README_MD: &str = include_str!(",
        "fn component_readme_markdown(slug: &str) -> Option<&'static str>",
        "let readme_html = component_readme_markdown(slug).map(crate::markdown::markdown_to_html);",
        "<div data-slot=\"component-readme\" inner_html=html></div>",
    ] {
        assert!(
            docs_shell_source.contains(needle),
            "docs shell trusted inner_html whitelist should keep marker `{needle}`."
        );
    }

    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    for source in [check2_source, check2_source_mirror] {
        for needle in [
            "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
            "仅允许编译期常量或明确白名单内容进入 `inner_html`。",
            "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。",
            "使用 `inner_html` 的节点必须补语义测试与安全回归说明。",
            "color_swatch_picker_inner_html_usage_is_forbidden_in_component_and_docs_examples",
        ] {
            assert!(
                source.contains(needle),
                "check2 should keep inner_html safety contract marker `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-components-inner-html.sh");

    let needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce color-swatch-picker contract marker `{needle}`."
    );
}

#[test]
fn color_swatch_picker_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("../../crates/ui-components/Cargo.toml");
    let crate_root_source = load_source("../../crates/ui-components/src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components Cargo features should keep shared wasm-debug marker `{needle}`."
        );
    }
    assert!(
        !cargo_source.contains("color-swatch-picker-wasm-debug")
            && !cargo_source.contains("color_swatch_picker-wasm-debug"),
        "color-swatch-picker should not expose a component-local wasm-debug feature."
    );

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui-components root should keep wasm-debug isolation marker `{needle}`."
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
            "docs-app should keep debug-only wasm trace visual entry marker `{needle}`."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "global debug overlay should keep marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "Inspect {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "ts_ms: now_ms(),",
    ] {
        assert!(
            trace_source.contains(needle),
            "global trace model should keep typed source/timestamp marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-selection-mode=selection_mode_attr",
        "data-selection-init-source=selection_init_source_attr",
        "data-selection-source=move || selection_source.get()",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
        "on:keydown=on_key_down",
        "on:focus=move |_| aria.handlers.on_radio_focus.run(index)",
        "on:click=move |_| aria.handlers.on_radio_click.run(index)",
        "set_pending_user_selection_for_change.set(true);",
        "selected_state_request_change.run(next);",
    ] {
        assert!(
            view_source.contains(needle),
            "color-swatch-picker should keep trace/replay marker `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_selection_source_attr(",
        "if has_pending_user_selection {",
        "\"interaction\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic should keep interaction-source derivation marker `{needle}`."
        );
    }

    let combined = format!("{view_source}\n{logic_source}\n{motion_source}");
    for forbidden in [
        "#[prop(optional)] debug",
        "pub fn set_debug",
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
    ] {
        assert!(
            !combined.contains(forbidden),
            "production component contract should not leak wasm-debug internals `{forbidden}`."
        );
    }

    for source in [check2_source, check2_source_mirror] {
        for needle in [
            "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
            "color_swatch_picker_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
            "开发模式下至少能追踪关键状态变更来源与前后值。",
            "关键交互链路应支持最小可复现记录（事件顺序/状态转移）。",
            "调试开关默认不进入生产包体与公共 API。",
        ] {
            assert!(
                source.contains(needle),
                "check2 should keep wasm-debug contract marker `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    let needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm-debug check script should enforce color-swatch-picker contract marker `{needle}`."
    );
}

#[test]
fn color_swatch_picker_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na()
 {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let dx_script_source = load_source("../../scripts/check-ui-components-dx.sh");
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for required in [
        "let scope_selector = format!(\"[data-playground-scope=\\\"{scope_id}\\\"]\");",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "<div data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "let (test_css, set_test_css) = signal(default_test_css.get_untracked());",
        "<textarea",
        "class=\"playground__test-editor\"",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "Original CSS is loaded. Use :scope to target this playground only.",
        "on_press=on_reset_test_css",
        "\"Restore original CSS\"",
        "data-slot=\"playground-controls\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(required),
            "Playground should keep DX hot-style-feedback + isolated-canvas token `{required}`."
        );
    }

    let section_start = docs_source
        .find("pub(super) fn color_swatch_picker() -> AnyView {")
        .unwrap_or_else(|| panic!("display_extra docs should contain color_swatch_picker section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn skeleton_group() -> AnyView {")
        .unwrap_or_else(|| {
            panic!(
                "display_extra docs should contain skeleton_group section after color_swatch_picker"
            )
        });
    let section = &section_tail[..section_end_rel];

    for required in [
        "title=\"ColorSwatchPicker\"",
        "slug=\"color-swatch-picker\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Basic Selection\" code_signal=basic_code>",
        "<Playground title=\"Transparency + Disabled + Custom Class\" code_signal=state_code>",
        "swatches=signal(swatches).0",
        "swatches=signal(disabled_swatches).0",
        "default_selected_color=\"#f80\".to_string()",
        "shape=ColorSwatchShape::Wide",
        "rounding=ColorSwatchRounding::Default",
        "class_name=\"docs-color-swatch-picker-custom\".to_string()",
        "aria_label=\"Fill color\".to_string()",
    ] {
        assert!(
            section.contains(required),
            "ColorSwatchPicker docs should provide isolated demo/workbench token `{required}`."
        );
    }

    for forbidden in [
        "Persist workbench state",
        "workbench_persist_state",
        "load_color_swatch_picker_workbench_state",
        "save_color_swatch_picker_workbench_state",
        "clear_color_swatch_picker_workbench_state",
        "localStorage",
        "sessionStorage",
    ] {
        assert!(
            !section.contains(forbidden),
            "ColorSwatchPicker keeps optional persisted workbench state as N/A; token `{forbidden}` should stay absent."
        );
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na";
    assert!(
        dx_script_source.contains(
            "echo \"[dx] contract: color-swatch-picker playground css hot-reload + isolated canvas\""
        ) && dx_script_source.contains(script_needle),
        "DX gate script should include color-swatch-picker contract markers."
    );

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
            "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
            "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
            "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
            "color_swatch_picker_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
        ] {
            assert!(
                source.contains(required),
                "ColorSwatchPicker checklist should keep DX governance rule `{required}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_dx_check_script_covers_hot_reload_and_isolated_canvas_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    let needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na";
    assert!(
        script_source.contains(needle),
        "DX check script should enforce color-swatch-picker contract marker `{needle}`."
    );
}

#[test]
fn color_swatch_picker_engineering_contract_marks_spec_serde_path_as_na_and_keeps_tracing_runtime_boundaries()
 {
    let mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let styles_source = load_source("../../components/color-swatch-picker/src/styles.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");

    let cargo_source = load_source("../../crates/ui-components/Cargo.toml");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "ColorSwatchPicker keeps spec/schema boundary as N/A for current simple component scope.",
    );
    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "ColorSwatchPicker module boundary should not expose spec layer token `{forbidden}`.",
        );
    }

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");

    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "#[serde(",
        "SchemaVersion",
        "migrate_v1_to_v2",
        "spec/config",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ColorSwatchPicker engineering contract should keep spec/serde path as N/A and avoid `{forbidden}`.",
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "pub enum UiTraceEventKind {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            cargo_source.contains(needle) || trace_source.contains(needle),
            "engineering baseline should keep unified tracing marker `{needle}`."
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::color_swatch_picker::",
        "const COLOR_SWATCH_PICKER_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ColorSwatchPicker should avoid component-local tracing semantic drift token `{forbidden}`.",
        );
    }

    for forbidden in [
        "tokio",
        "tokio::",
        "async_std",
        "async_std::",
        "async-std",
        "smol::",
        "runtime::Handle",
        "spawn_blocking(",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ColorSwatchPicker engineering contract should not leak runtime marker `{forbidden}`.",
        );
    }

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
            "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
            "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
            "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
            "color_swatch_picker_engineering_contract_marks_spec_serde_path_as_na_and_keeps_tracing_runtime_boundaries",
        ] {
            assert!(
                source.contains(required),
                "ColorSwatchPicker checklist should keep engineering governance rule `{required}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_engineering_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");
    let needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_engineering_contract_marks_spec_serde_path_as_na_and_keeps_tracing_runtime_boundaries";

    assert!(
        script_source.contains(needle),
        "engineering check script should enforce color-swatch-picker contract marker `{needle}`."
    );
}

#[test]
fn color_swatch_picker_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let component_manifest = load_source("../../components/color-swatch-picker/src/Component.toml");
    let rbi_source =
        load_source("../../components/color-swatch-picker/src/color_swatch_picker.rbi");

    for required in [
        "pub enum ColorSwatchPickerAgentSchema",
        "pub enum ColorSwatchPickerAgentSchemaVersion",
        "Self::V1 => \"ui.color-swatch-picker.agent-contract.v1\"",
        "Self::V1 => \"1\"",
    ] {
        assert!(
            logic_source.contains(required),
            "ColorSwatchPicker logic should keep stable v1 schema marker `{required}` in non-breaking scope."
        );
    }

    for required in [
        "schema_version = \"1\"",
        "schema = \"ui.color-swatch-picker.agent-contract.v1\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "Component.toml should keep v1 schema marker `{required}` in current scope."
        );
    }

    for forbidden in [
        "V2",
        "migrate_v1_to_v2",
        "SchemaRegistry",
        "deprecated_window",
        "codemod",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !rbi_source.contains(forbidden),
            "without major breaking upgrade, ColorSwatchPicker should not claim migration path token `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`."
    );

    for source in [check2_source, check2_source_mirror] {
        for needle in [
            "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `ColorSwatchPicker` 改动未引入跨大版本 API 破坏升级，组件语义契约仍保持 `v1`（`components/color-swatch-picker/src/logic.rs` 的 `ColorSwatchPickerAgentSchema::V1`/`ColorSwatchPickerAgentSchemaVersion::V1`，`components/color-swatch-picker/src/Component.toml` 的 `schema_version = \"1\"` 与 `ui.color-swatch-picker.agent-contract.v1`），因此不触发 Codemod/Schema Registry 弃用窗口与 `migrate_v1_to_v2` 迁移层要求。回归：`components/color-swatch-picker/test/semantics.rs::color_swatch_picker_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade` 与 `components/color-swatch-picker/test/color_swatch_picker_semantics.rs::color_swatch_picker_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade`；门禁脚本：`scripts/check-ui-components-engineering.sh` 新增对应 `cargo test` 目标。）",
            "color_swatch_picker_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
        ] {
            assert!(
                source.contains(needle),
                "checklist should keep codemod/registry migration marker `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_type_system_and_semantic_markers_keep_machine_readable_contract() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");

    for needle in [
        "#[prop(optional)] size: ColorSwatchSize",
        "#[prop(optional)] rounding: ColorSwatchRounding",
        "#[prop(optional)] shape: ColorSwatchShape",
        "SwatchPickerState as ColorSwatchPickerState",
        "SwatchPickerStateInput as ColorSwatchPickerStateInput",
        "pub fn resolve_state(input: ColorSwatchPickerStateInput) -> ColorSwatchPickerState",
        "resolve_state(ColorSwatchPickerStateInput {",
        "pub fn resolve_component_state(",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-selection-mode=selection_mode_attr",
        "data-selection-init-source=selection_init_source_attr",
        "data-selection-source=move || selection_source.get()",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "type-system and marker contract should include `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] size: Option<String>",
        "#[prop(optional, into)] rounding: Option<String>",
        "#[prop(optional, into)] shape: Option<String>",
        "match size.as_str()",
        "match rounding.as_str()",
        "match shape.as_str()",
        "data-state=move || format!(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "type-system and marker contract should avoid string/bool protocol fallback `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_picker_check2_marks_type_system_machine_readable_contract_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        assert!(
            source.contains(
                "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。"
            ),
            "color-swatch-picker check2 should mark type-system machine-readable contract complete."
        );
        for needle in [
            "color_swatch_picker_type_system_and_semantic_markers_keep_machine_readable_contract",
            "#[prop(optional)] size: ColorSwatchSize",
            "#[prop(optional)] rounding: ColorSwatchRounding",
            "#[prop(optional)] shape: ColorSwatchShape",
            "data-selection-source",
        ] {
            assert!(
                source.contains(needle),
                "type-system machine-readable check2 section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_semantic_test_matrix_covers_key_paths_without_snapshot_reliance() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");
    let local_semantics_source = include_str!("semantics.rs");

    for needle in [
        "#[prop(optional, into)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_color: Option<String>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] is_disabled: bool",
        "on:keydown=on_key_down",
        "on:focus=move |_| aria.handlers.on_radio_focus.run(index)",
        "on:click=move |_| aria.handlers.on_radio_click.run(index)",
        "role=aria.attrs.role",
        "aria-checked=move || if is_selected() { \"true\" } else { \"false\" }",
        "data-state=move || state.get().data_state_attr",
        "data-selection-mode=selection_mode_attr",
        "data-selection-source=move || selection_source.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "semantic matrix should cover branch contract `{needle}`."
        );
    }

    for forbidden in ["web_sys::", "wasm_bindgen", "JsValue"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should keep SSR/non-wasm compatible surface without `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should keep SSR/non-wasm compatible surface without `{forbidden}`."
        );
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should keep SSR/non-wasm compatible export surface without `{forbidden}`."
        );
    }

    for forbidden in [
        "insta::assert_snapshot",
        "assert_snapshot!",
        "to_match_snapshot",
        "playwright::expect_screenshot",
    ] {
        assert!(
            !local_semantics_source.contains(forbidden),
            "semantics test suite should not rely on visual snapshot assertion `{forbidden}`.",
        );
    }
}

#[test]
fn color_swatch_picker_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only()
{
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let local_semantics_source = include_str!("semantics.rs");
    let legacy_semantics_source = load_source("legacy_semantics");
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "role=aria.attrs.role",
        "aria-checked=move || if is_selected() { \"true\" } else { \"false\" }",
        "data-state=move || state.get().data_state_attr",
        "data-selection-source=move || selection_source.get()",
        "on:keydown=on_key_down",
        "on:focus=move |_| aria.handlers.on_radio_focus.run(index)",
        "on:click=move |_| aria.handlers.on_radio_click.run(index)",
    ] {
        assert!(
            view_source.contains(required),
            "semantic contract must keep role/aria/data/action marker `{required}`.",
        );
    }

    for forbidden in [
        "insta::assert_snapshot(",
        "assert_snapshot!(",
        "to_match_snapshot(",
        "expect_screenshot(",
    ] {
        assert!(
            !local_semantics_source.contains(forbidden),
            "local semantics suite must not rely on visual snapshot assertion `{forbidden}`.",
        );
        assert!(
            !legacy_semantics_source.contains(forbidden),
            "aggregated semantics suite must not rely on visual snapshot assertion `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should enforce `{script_needle}`.",
    );

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
            "color_swatch_picker_semantic_test_matrix_covers_key_paths_without_snapshot_reliance",
            "color_swatch_picker_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only",
            "scripts/check-ui-components-contract-hygiene.sh",
        ] {
            assert!(
                source.contains(required),
                "check2 semantics-first section should include `{required}`.",
            );
        }
    }
}

#[test]
fn color_swatch_picker_contract_hygiene_script_covers_semantics_priority_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    let marker = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only";
    assert!(
        script_source.contains(marker),
        "contract-hygiene script should include semantics-priority marker `{marker}`.",
    );
}

#[test]
fn color_swatch_picker_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let lib_source = load_source("../../crates/ui-components/src/lib.rs");
    let css_source = load_source("../../crates/ui-components/src/css.rs");
    let root_source = load_source("../../crates/ui-components/src/root.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-color_swatch_picker\")]",
        "#[path = \"../../../components/color-swatch-picker/src/mod.rs\"]",
        "pub mod color_swatch_picker;",
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
        "pub mod overlay_open;",
        "pub mod presence;",
        "pub mod a11y;",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-components lib entry should not leak platform/internal marker `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-color_swatch_picker\")]",
        "out.push_str(crate::color::swatch_picker::styles::CSS);",
        "#[cfg(feature = \"component-active_highlight\")]",
        "out.push_str(ui_visual_primitive::active_highlight::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css registry should keep feature-gated marker `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n, provide_ui_id_provider};",
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
        "data-theme-scheme",
        "data-theme-color",
        "data-theme-system",
        "data-theme-scale",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot entry should keep centralized theme/i18n/css injection marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "ColorSwatchPicker",
        "color_swatch_picker",
        "data-slot",
        "aria-",
    ] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`."
        );
    }

    for forbidden in [
        "../../crates/ui-components/src/overlay_open.rs",
        "../../crates/ui-components/src/presence.rs",
        "../../crates/ui-components/src/a11y.rs",
    ] {
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
fn color_swatch_picker_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-components-entrypoints.sh");
    let needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_ui_components_fixed_entry_files_follow_layered_boundaries";

    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn color_swatch_picker_check2_marks_ui_components_fixed_entry_files_contract_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        for needle in [
            "- [x] `ui-components` 固定入口文件落点正确。",
            "color_swatch_picker_ui_components_fixed_entry_files_follow_layered_boundaries",
            "color_swatch_picker_entrypoints_check_script_covers_fixed_entrypoint_contract",
            "scripts/check-ui-components-entrypoints.sh",
            "crates/ui-components/src/lib.rs",
            "crates/ui-components/src/css.rs",
            "crates/ui-components/src/root.rs",
            "crates/ui-visual-primitive/src/active_highlight.rs",
            "crates/ui-headless/src/controllable_state.rs",
            "crates/ui-headless/src/presence.rs",
            "crates/ui-headless/src/a11y.rs",
            "crates/ui-components/src/overlay_open.rs",
            "crates/ui-components/src/presence.rs",
            "crates/ui-components/src/a11y.rs",
        ] {
            assert!(
                source.contains(needle),
                "color-swatch-picker check2 fixed-entry-files section should reference `{needle}`.",
            );
        }
    }
}

#[test]
fn color_swatch_picker_component_directory_standard_files_follow_contract_and_na_paths() {
    let mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let styles_source = load_source("../../components/color-swatch-picker/src/styles.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");

    for required in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "color-swatch-picker component should keep required standard file `{required}`."
        );
    }

    for forbidden in ["src/render.rs", "src/spec.rs"] {
        assert!(
            !path_exists(forbidden),
            "color-swatch-picker simple component should not introduce `{forbidden}`."
        );
    }

    for needle in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{",
        "pub use motion::ColorSwatchPickerMotion;",
        "pub use view::ColorSwatchPicker;",
    ] {
        assert!(
            mod_source.contains(needle),
            "mod.rs should keep minimal stable export marker `{needle}`."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;", "web_sys", "wasm_bindgen"] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not leak internal/platform marker `{forbidden}`."
        );
    }

    for needle in [
        "pub fn normalize_id_base(value: Option<String>) -> String",
        "pub fn resolve_selection_mode_attr(is_controlled: bool) -> &'static str",
        "pub fn resolve_state(input: ColorSwatchPickerStateInput) -> ColorSwatchPickerState",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep normalization/derivation marker `{needle}`."
        );
    }
    for forbidden in ["view! {", "on:click", "NodeRef<", "web_sys", "aria-"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay state-focused and avoid view/runtime marker `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str"),
        "styles.rs should expose static CSS contract."
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "styles.rs should consume token-first CSS variables."
    );
    assert!(
        !contains_hex_color_literal(styles_source),
        "styles.rs should avoid hardcoded hex literals and stick to theme tokens."
    );

    for needle in [
        "use ui_headless::{A11yDirection, RadioGroupOptions, RadioOptions, RovingOrientation, use_radio};",
        "let selected_state = overlay_open::use_controllable_state(",
        "style=move || style_vars.get_value()",
        "logic::resolve_component_state(",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep structural render + headless mount marker `{needle}`."
        );
    }
    for forbidden in [
        "unwrap_or_else(|| \"ui-color-swatch-picker\".to_string())",
        "let disabled_item_count = items.iter().filter(|item| item.disabled).count();",
        "let option_disabled = is_disabled || item.disabled;",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not hide core state decisions via `{forbidden}`."
        );
    }

    for needle in [
        "pub struct ColorSwatchPickerMotion {",
        "pub fn sanitize_motion(motion: ColorSwatchPickerMotion) -> ColorSwatchPickerMotion",
        "pub fn attach_motion(base_vars: Option<String>, motion: ColorSwatchPickerMotion) -> String",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should keep component contract mapping marker `{needle}`."
        );
    }
    for forbidden in ["request_animation_frame", "web_sys::", "wasm_bindgen::"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not reimplement runtime engine detail `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_picker_component_files_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_component_directory_standard_files_follow_contract_and_na_paths";

    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn color_swatch_picker_check2_marks_component_directory_standard_files_contract_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        for needle in [
            "- [x] 组件目录标准文件落点正确。",
            "color_swatch_picker_component_directory_standard_files_follow_contract_and_na_paths",
            "color_swatch_picker_component_files_check_script_covers_contract",
            "scripts/check-ui-components-component-files.sh",
            "components/color-swatch-picker/src/mod.rs",
            "components/color-swatch-picker/src/logic.rs",
            "components/color-swatch-picker/src/styles.rs",
            "components/color-swatch-picker/src/view.rs",
            "components/color-swatch-picker/src/motion.rs",
            "components/color-swatch-picker/src/render.rs",
            "components/color-swatch-picker/src/spec.rs",
        ] {
            assert!(
                source.contains(needle),
                "color-swatch-picker checklist should include component file contract marker `{needle}`.",
            );
        }
    }
}

#[test]
fn color_swatch_picker_file_placement_discipline_is_strict_for_component_scope() {
    let mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let styles_source = load_source("../../components/color-swatch-picker/src/styles.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");

    for required in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "file-placement discipline requires `{required}` to exist."
        );
    }

    for forbidden in ["src/render.rs", "src/protocol.rs"] {
        assert!(
            !path_exists(forbidden),
            "file-placement discipline forbids `{forbidden}` for this component."
        );
    }

    assert!(
        !path_exists("src/spec.rs"),
        "ColorSwatchPicker is not a complex schema-driven component; `spec.rs` should remain absent."
    );

    for forbidden in [
        "mod protocol;",
        "pub mod protocol;",
        "mod render;",
        "pub mod render;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not re-introduce forbidden file-placement module `{forbidden}`."
        );
    }

    for needle in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
    ] {
        assert!(
            mod_source.contains(needle),
            "mod.rs should keep strict module layout marker `{needle}`."
        );
    }

    for forbidden in ["view! {", "on:click", "NodeRef<", "web_sys"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay normalization-only; found `{forbidden}`."
        );
    }
    assert!(
        styles_source.contains("pub const CSS: &str"),
        "styles.rs should remain token-first static stylesheet surface."
    );
    assert!(
        view_source.contains("#[component]"),
        "view.rs should keep render entrypoint in component view layer."
    );
    assert!(
        motion_source.contains("pub struct ColorSwatchPickerMotion"),
        "motion.rs should keep motion contract mapping surface."
    );
}

#[test]
fn color_swatch_picker_component_files_script_covers_file_placement_discipline() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_file_placement_discipline_is_strict_for_component_scope";

    assert!(
        script_source.contains(needle),
        "component-files gate should enforce `{needle}`."
    );
}

#[test]
fn color_swatch_picker_check2_marks_file_placement_discipline_contract_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        for needle in [
            "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
            "color_swatch_picker_file_placement_discipline_is_strict_for_component_scope",
            "color_swatch_picker_component_files_script_covers_file_placement_discipline",
            "scripts/check-ui-components-component-files.sh",
            "components/color-swatch-picker/src/mod.rs",
            "components/color-swatch-picker/src/logic.rs",
            "components/color-swatch-picker/src/styles.rs",
            "components/color-swatch-picker/src/view.rs",
            "components/color-swatch-picker/src/motion.rs",
            "components/color-swatch-picker/src/render.rs",
            "components/color-swatch-picker/src/protocol.rs",
        ] {
            assert!(
                source.contains(needle),
                "file-placement discipline checklist should include `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");
    let styles_source = load_source("../../components/color-swatch-picker/src/styles.rs");

    assert!(
        !path_exists("src/spec.rs"),
        "ColorSwatchPicker should keep Hyper-Structure Builder as N/A-by-design for simple component scope."
    );
    assert!(
        path_exists("../../crates/ui-components/src/button/spec.rs"),
        "button/spec.rs should remain the canonical complex-component Hyper-Structure Builder surface."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not expose Hyper-Structure Builder token `{forbidden}`."
        );
    }

    for forbidden in [
        "ColorSwatchPickerSpec",
        "ColorSwatchPickerSpec::new(",
        "spec::ColorSwatchPickerSpec",
        "Spec::new(",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "ColorSwatchPicker should not expose Hyper-Structure Builder path token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_picker_component_files_script_covers_hyper_structure_builder_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_hyper_structure_builder_spec_is_not_applicable_for_simple_component";

    assert!(
        script_source.contains(needle),
        "component-files gate should enforce `{needle}`."
    );
}

#[test]
fn color_swatch_picker_check2_marks_hyper_structure_builder_contract_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        for needle in [
            "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
            "N/A-by-design：`ColorSwatchPicker` 不是复杂配置固化组件",
            "color_swatch_picker_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
            "color_swatch_picker_component_files_script_covers_hyper_structure_builder_contract",
            "scripts/check-ui-components-component-files.sh",
            "crates/ui-components/src/button/spec.rs",
        ] {
            assert!(
                source.contains(needle),
                "Hyper-Structure Builder checklist should include `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let component_manifest = load_source("../../components/color-swatch-picker/src/Component.toml");
    let component_rbi =
        load_source("../../components/color-swatch-picker/src/color_swatch_picker.rbi");

    assert!(
        path_exists("src/Component.toml"),
        "color-swatch-picker context-compression file should exist: `Component.toml`."
    );
    assert!(
        path_exists("src/color_swatch_picker.rbi"),
        "color-swatch-picker context-compression file should exist: `color_swatch_picker.rbi`."
    );

    for required in [
        "schema_version = \"1\"",
        "name = \"ColorSwatchPicker\"",
        "crate = \"ui-color-swatch-picker\"",
        "name = \"swatches\"",
        "name = \"selected_color\"",
        "name = \"default_selected_color\"",
        "name = \"on_selected_change\"",
        "name = \"is_disabled\"",
        "name = \"motion\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "Component.toml should keep context-compression marker `{required}`."
        );
    }

    for required in [
        "pub type ColorSwatchPickerItem = ui_state_primitives::swatch_picker::SwatchPickerItem;",
        "pub type ColorSwatchPickerState = ui_state_primitives::swatch_picker::SwatchPickerState;",
        "pub type ColorSwatchPickerStateInput = ui_state_primitives::swatch_picker::SwatchPickerStateInput;",
        "pub type ColorSwatchPickerMotion = crate::ColorSwatchPickerMotion;",
        "pub const DEFAULT_ARIA_LABEL: &str;",
        "pub fn ColorSwatchPicker(",
        "swatches: leptos::prelude::ReadSignal<Vec<ColorSwatchPickerItem>>",
        "selected_color: Option<leptos::prelude::Signal<Option<String>>>",
        "default_selected_color: Option<String>",
        "on_selected_change: Option<leptos::prelude::Callback<Option<String>>>",
        "dir: Option<ui_headless::A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi.contains(required),
            "color_swatch_picker.rbi should keep signature-projection marker `{required}`."
        );
    }
}

#[test]
fn color_swatch_picker_component_files_script_covers_context_compression_manifest_and_rbi() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_context_compression_manifest_and_rbi_projection_are_present_and_current";

    assert!(
        script_source.contains(needle),
        "component-files gate should enforce `{needle}`."
    );
}

#[test]
fn color_swatch_picker_check2_marks_context_compression_manifest_and_rbi_contract_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        for needle in [
            "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
            "`components/color-swatch-picker/src/Component.toml`",
            "`components/color-swatch-picker/src/color_swatch_picker.rbi`",
            "color_swatch_picker_context_compression_manifest_and_rbi_projection_are_present_and_current",
            "color_swatch_picker_component_files_script_covers_context_compression_manifest_and_rbi",
            "scripts/check-ui-components-component-files.sh",
        ] {
            assert!(
                source.contains(needle),
                "context-compression checklist should include `{needle}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_agent_contract_is_schema_typed_and_machine_readable() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let component_manifest = load_source("../../components/color-swatch-picker/src/Component.toml");
    let component_rbi =
        load_source("../../components/color-swatch-picker/src/color_swatch_picker.rbi");

    for typed_source in [
        "pub enum ColorSwatchPickerAgentSchema",
        "pub enum ColorSwatchPickerAgentSchemaVersion",
        "pub enum ColorSwatchPickerIntent",
        "pub enum ColorSwatchPickerUiAction",
        "pub enum ColorSwatchPickerUiState",
        "pub enum ColorSwatchPickerUiSource",
        "pub struct ColorSwatchPickerAgentContract",
        "pub fn resolve_agent_contract() -> ColorSwatchPickerAgentContract",
        "pub fn resolve_ui_action(selection_source_attr: &'static str) -> ColorSwatchPickerUiAction",
        "pub fn resolve_ui_state(is_disabled: bool, is_empty: bool) -> ColorSwatchPickerUiState",
        "pub fn resolve_ui_source(selection_source_attr: &'static str) -> ColorSwatchPickerUiSource",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "Agent Contract should stay type-derived via `{typed_source}`."
        );
    }

    for marker in [
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=move || logic::resolve_ui_action(selection_source.get()).as_attr()",
        "data-ui-state=move || logic::resolve_ui_state(state.get().is_disabled, state.get().is_empty).as_attr()",
        "data-ui-source=move || logic::resolve_ui_source(selection_source.get()).as_attr()",
    ] {
        assert!(
            view_source.contains(marker),
            "view.rs should mount Agent Contract marker `{marker}`."
        );
    }

    for required in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_contract_schema_markers\"",
        "[[agent_contract]]",
        "schema = \"ui.color-swatch-picker.agent-contract.v1\"",
        "intent = \"pick-color-swatch\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "ColorSwatchPickerAgentContract",
        "resolve_agent_contract",
    ] {
        assert!(
            component_manifest.contains(required) || component_rbi.contains(required),
            "context-compression assets should keep Agent Contract marker `{required}`."
        );
    }

    for forbidden in [
        "format!(\"data-ui-schema",
        "format!(\"data-ui-intent",
        "format!(\"data-ui-action",
        "format!(\"data-ui-state",
        "format!(\"data-ui-source",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Agent Contract should avoid free-form schema token `{forbidden}`."
        );
    }

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
            "color_swatch_picker_agent_contract_is_schema_typed_and_machine_readable",
        ] {
            assert!(
                source.contains(required),
                "checklist should keep Agent Contract evidence `{required}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let component_manifest = load_source("../../components/color-swatch-picker/src/Component.toml");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "allowed = [",
        "\"logic::resolve_agent_contract()\"",
        "\"logic::resolve_ui_action(...)\"",
        "\"logic::resolve_ui_state(...)\"",
        "\"logic::resolve_ui_source(...)\"",
        "\"logic::resolve_component_state(...)\"",
        "\"motion::sanitize_motion(...)\"",
        "\"motion::attach_motion(...)\"",
        "\"use_radio(...)\"",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\", \"eval(\"]",
    ] {
        assert!(
            component_manifest.contains(required),
            "manifest should keep whitelist-safe render path marker `{required}`."
        );
    }

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "eval(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Agent Contract render path should forbid `{forbidden}`."
        );
    }

    for script_needle in [
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(script_needle),
            "contract-hygiene gate script should include `{script_needle}`."
        );
    }

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "color_swatch_picker_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
            "白名单能力边界",
        ] {
            assert!(
                source.contains(required),
                "checklist should keep Agent Contract whitelist evidence `{required}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    let mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let styles_source = load_source("../../components/color-swatch-picker/src/styles.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
            "`Streaming`：LLM 还在生成，界面边生成边显示。",
            "`Snapshot`：LLM 全部生成完成后，一次性显示。",
            "N/A：`ColorSwatchPicker` 不是 LLM 正文渲染组件",
            "color_swatch_picker_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        ] {
            assert!(
                source.contains(required),
                "checklist should keep streaming-definition marker `{required}`."
            );
        }
    }

    for forbidden in ["use_ai_space_state", "project_streaming_"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !docs_page_source.contains(forbidden),
            "color-swatch-picker should stay out of LLM streaming protocol scope and avoid `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`."
    );
}

#[test]
fn color_swatch_picker_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
            "`ColorSwatchPicker` 已支持完整配置快照输入并稳定渲染",
            "color_swatch_picker_check2_documents_snapshot_as_default_baseline_capability",
            "color_swatch_picker_snapshot_baseline_consumes_complete_result_and_renders_stably",
            "所有组件都应能消费“完整生成结果”并稳定渲染。",
            "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        ] {
            assert!(
                source.contains(required),
                "checklist should keep snapshot-baseline marker `{required}`."
            );
        }
    }
}

#[test]
fn color_swatch_picker_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for marker in [
        "pub fn ColorSwatchPicker(",
        "#[prop(optional, into)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_color: Option<String>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "#[prop(optional, into)] id_base: Option<String>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "#[prop(optional)] motion: ColorSwatchPickerMotion,",
        "let id_base = logic::normalize_id_base(id_base);",
        "let items = Memo::new(move |_| logic::normalize_items(swatches.get()));",
        "let selected_state = overlay_open::use_controllable_state(",
        "data-state=move || state.get().data_state_attr",
        "data-selection-mode=selection_mode_attr",
        "data-selection-init-source=selection_init_source_attr",
        "data-selection-source=move || selection_source.get()",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-source=move || logic::resolve_ui_source(selection_source.get()).as_attr()",
    ] {
        assert!(
            view_source.contains(marker),
            "snapshot baseline should keep complete-result render marker `{marker}`."
        );
    }

    for marker in [
        "pub fn resolve_agent_contract() -> ColorSwatchPickerAgentContract",
        "ColorSwatchPickerStreamFallback::Snapshot.as_attr()",
        "pub fn normalize_id_base(value: Option<String>) -> String",
        "pub fn normalize_items(items: Vec<ColorSwatchPickerItem>) -> Vec<ColorSwatchPickerItem>",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "pub fn resolve_component_state(",
    ] {
        assert!(
            logic_source.contains(marker),
            "logic should keep normalized snapshot baseline marker `{marker}`."
        );
    }

    for marker in [
        "title=\"ColorSwatchPicker\"",
        "slug=\"color-swatch-picker\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Basic Selection\" code_signal=basic_code>",
        "<Playground title=\"Transparency + Disabled + Custom Class\" code_signal=state_code>",
        "default_selected_color=\"#f80\".to_string()",
        "class_name=\"docs-color-swatch-picker-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(marker),
            "docs should keep snapshot-ready baseline usage marker `{marker}`."
        );
    }

    for script_needle in [
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming gate script should include `{script_needle}`."
        );
    }
}

#[test]
fn color_swatch_picker_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("../../components/color-swatch-picker/check2.md");
    let checklist_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for source in [checklist_source, checklist_source_mirror] {
        for required in [
            "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。（`ColorSwatchPicker` 归类为 `Streaming Optional`；组件职责是色板选择语义装配而非 LLM 正文阅读面，默认走 `Snapshot` 渲染路径。实现显式输出 `data-ui-stream-support=\"unsupported\"`、`data-ui-stream-fallback=\"snapshot\"` 与 `data-ui-output-status=\"verified\"`，并保持 `role/aria/data-*` 连续可读。数据校验、断线恢复、重试策略继续留在上层编排，不下沉到组件。回归：`components/color-swatch-picker/test/semantics.rs::color_swatch_picker_check2_documents_streaming_required_optional_classification_rules`、`components/color-swatch-picker/test/semantics.rs::color_swatch_picker_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、`components/color-swatch-picker/test/semantics.rs::color_swatch_picker_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer`、`components/color-swatch-picker/test/color_swatch_picker_semantics.rs::color_swatch_picker_check2_documents_streaming_required_optional_classification_rules`、`components/color-swatch-picker/test/color_swatch_picker_semantics.rs::color_swatch_picker_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、`components/color-swatch-picker/test/color_swatch_picker_semantics.rs::color_swatch_picker_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer`；门禁脚本：`scripts/check-ui-components-streaming.sh` 新增对应 `cargo test` 目标。）",
            "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
            "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
            "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
            "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
            "`ColorSwatchPicker` 归类为 `Streaming Optional`",
        ] {
            assert!(
                source.contains(required),
                "checklist should keep streaming responsibility marker `{required}`."
            );
        }
    }

    for script_needle in [
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming check script should enforce `{script_needle}`."
        );
    }
}

#[test]
fn color_swatch_picker_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");

    for required in [
        "role=aria.attrs.role",
        "aria-disabled=aria.attrs.aria_disabled",
        "aria-label=aria_label",
        "lang=aria.attrs.lang",
        "dir=aria.attrs.dir",
        "data-state=move || state.get().data_state_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=move || logic::resolve_ui_action(selection_source.get()).as_attr()",
        "data-ui-source=move || logic::resolve_ui_source(selection_source.get()).as_attr()",
        "data-ui-state=move || logic::resolve_ui_state(state.get().is_disabled, state.get().is_empty).as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "color-swatch-picker should keep continuous aria/data semantics via `{required}` in optional-streaming scope.",
        );
    }
}

#[test]
fn color_swatch_picker_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer()
 {
    let mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");
    let styles_source = load_source("../../components/color-swatch-picker/src/styles.rs");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");
    let combined =
        format!("{mod_source}\n{view_source}\n{logic_source}\n{motion_source}\n{styles_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "network_error",
        "transport_error",
        "abort_controller",
        "exponential_backoff",
    ] {
        assert!(
            !combined.contains(forbidden),
            "color-swatch-picker should keep validation/retry/resilience orchestration out of component layer; found `{forbidden}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming gate script should include `{script_needle}`.",
        );
    }
}

#[test]
fn color_swatch_picker_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources()
 {
    let mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let styles_source = load_source("../../components/color-swatch-picker/src/styles.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let motion_source = load_source("../../components/color-swatch-picker/src/motion.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "color-swatch-picker non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn color_swatch_picker_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> =",
        "Cow::Borrowed(\"ui-color-swatch-picker\")",
        ".map(|class_name| class_name.as_ref())",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic_source.contains(required),
            "color-swatch-picker logic should keep Cow-based string hotspot mitigation marker `{required}`.",
        );
    }

    for forbidden in [
        "\"ui-color-swatch-picker\".to_string()",
        "\"ui-color-swatch-picker--disabled\".to_string()",
        "\"ui-color-swatch-picker--custom-class\".to_string()",
        "String::from(\"ui-color-swatch-picker\")",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "color-swatch-picker fallback normalization should avoid string clone hotspot `{forbidden}`.",
        );
    }
}

#[test]
fn color_swatch_picker_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = load_source("../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("../../scripts/check-ui-components-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            script_source.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn color_swatch_picker_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        for needle in [
            "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。（`components/color-swatch-picker/src/logic.rs::compose_class_name` 已引入 `Vec<Cow<'static, str>>` 收敛静态类名分配热点；组件非测试源码维持无 `unwrap/expect` 与无吞错 `let _ = ...`。回归：`components/color-swatch-picker/test/semantics.rs::color_swatch_picker_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources`、`components/color-swatch-picker/test/semantics.rs::color_swatch_picker_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent`、`components/color-swatch-picker/test/semantics.rs::color_swatch_picker_rust_hygiene_script_enforces_repo_level_hygiene_guards`、`components/color-swatch-picker/test/color_swatch_picker_semantics.rs::color_swatch_picker_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources`、`components/color-swatch-picker/test/color_swatch_picker_semantics.rs::color_swatch_picker_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent`、`components/color-swatch-picker/test/color_swatch_picker_semantics.rs::color_swatch_picker_rust_hygiene_script_enforces_repo_level_hygiene_guards`；门禁脚本：`scripts/check-ui-components-engineering.sh` 新增对应 `cargo test` 目标。另执行：`./scripts/check-rust-hygiene.sh`（当前环境已执行，若失败以脚本输出为准）。）",
            "color_swatch_picker_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
            "color_swatch_picker_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
            "color_swatch_picker_rust_hygiene_script_enforces_repo_level_hygiene_guards",
            "./scripts/check-rust-hygiene.sh",
            "Cow<'static, str>",
        ] {
            assert!(
                source.contains(needle),
                "color-swatch-picker check2 rust-hygiene section should reference `{needle}`.",
            );
        }
    }
}

#[test]
fn color_swatch_picker_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn color_swatch_picker() -> AnyView",
        "title=\"ColorSwatchPicker\"",
        "slug=\"color-swatch-picker\"",
        "title=\"Hello World\"",
        "title=\"Basic Selection\"",
        "title=\"Transparency + Disabled + Custom Class\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Interactive Playground\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
    ] {
        assert!(
            source.contains(needle),
            "color-swatch-picker docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn color_swatch_picker_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "swatches=signal(vec![ColorSwatchPickerItem::named(\"#f80\", \"Orange\")]).0",
        "ColorSwatchPickerItem::named(\"#A00\", \"Red\")",
        "ColorSwatchPickerItem::named(\"#f80\", \"Orange\")",
        "ColorSwatchPickerItem::named(\"#080\", \"Green\")",
        "ColorSwatchPickerItem::named(\"#08f\", \"Blue\")",
        "default_selected_color=\"#f80\".to_string()",
        "ColorSwatchPickerItem::named(\"rgba(14, 116, 144, 0.4)\", \"Cyan 40%\").disabled(true)",
        "ColorSwatchPickerItem::named(\"rgba(255, 0, 0, 0)\", \"Transparent\")",
        "ColorSwatchPickerItem::new(\"#08f\")",
        "shape=ColorSwatchShape::Wide",
        "rounding=ColorSwatchRounding::Default",
        "class_name=\"docs-color-swatch-picker-custom\".to_string()",
        "aria_label=\"Fill color\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "color-swatch-picker docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn color_swatch_picker_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
            "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
            "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
            "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
        ] {
            assert!(
                source.contains(required),
                "color-swatch-picker checklist should keep documentation-as-product rule `{required}`.",
            );
        }
    }
}

#[test]
fn color_swatch_picker_docs_entry_exists_as_readme_or_equivalent_docs_app_page() {
    let readme_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../components/color-swatch-picker/src/README.md");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    assert!(
        readme_path.exists(),
        "color-swatch-picker should provide README as documentation entry.",
    );
    assert!(
        docs_page_source.contains("pub(super) fn color_swatch_picker() -> AnyView"),
        "docs-app should expose color_swatch_picker docs entry function.",
    );
}

#[test]
fn color_swatch_picker_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let readme_source = load_source("../../components/color-swatch-picker/src/README.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "title=\"ColorSwatchPicker\"",
        "slug=\"color-swatch-picker\"",
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
    ] {
        assert!(
            docs_source.contains(required),
            "color-swatch-picker docs should keep beginner-to-advanced marker `{required}`.",
        );
    }

    let hello_pos = docs_source
        .find("title=\"Hello World\"")
        .expect("docs should include hello-world playground for zero-threshold path.");
    let matrix_pos = docs_source
        .find("title=\"State Matrix\"")
        .expect("docs should include state-matrix playground as common usage.");
    let controlled_pos = docs_source
        .find("title=\"Controlled vs Uncontrolled Contrast\"")
        .expect("docs should include controlled-vs-uncontrolled playground.");
    assert!(
        hello_pos < matrix_pos && matrix_pos < controlled_pos,
        "docs should present default usage before advanced controls.",
    );

    for required in [
        "## Hello World",
        "## 受控用法",
        "## 常见用法（进阶）",
        "阅读顺序建议：先看 `Hello World` 直接跑起来，再按需启用受控与高级配置。",
        "默认路径不需要手动接线 `ui-state-primitives` 或 `ui-headless` 状态机。",
    ] {
        assert!(
            readme_source.contains(required),
            "README should keep beginner-friendly guidance marker `{required}`.",
        );
    }

    let readme_hello_pos = readme_source
        .find("## Hello World")
        .expect("README should include hello-world section.");
    let readme_controlled_pos = readme_source
        .find("## 受控用法")
        .expect("README should include controlled section.");
    let readme_advanced_pos = readme_source
        .find("## 常见用法（进阶）")
        .expect("README should include advanced usage section.");
    assert!(
        readme_hello_pos < readme_controlled_pos && readme_controlled_pos < readme_advanced_pos,
        "README should present default path before advanced guidance.",
    );

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_are_beginner_friendly_with_default_then_advanced_path";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_swatch_picker_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let readme_source = load_source("../../components/color-swatch-picker/src/README.md");

    for required in [
        "title=\"Hello World\"",
        "swatches=signal(vec![ColorSwatchPickerItem::named(\"#f80\", \"Orange\")]).0",
        "## Hello World",
        "ColorSwatchPickerItem::named(\"#f80\", \"Orange\")",
    ] {
        assert!(
            docs_source.contains(required) || readme_source.contains(required),
            "color-swatch-picker docs hello-world should keep zero-threshold marker `{required}`.",
        );
    }

    for forbidden in ["ui_state_primitives", "use_radio(", "state=...", "logic::"] {
        assert!(
            !readme_source.contains(forbidden),
            "color-swatch-picker README hello-world path should avoid architecture-wiring token `{forbidden}`.",
        );
    }
}

#[test]
fn color_swatch_picker_contract_hygiene_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_entry_exists_as_readme_or_equivalent_docs_app_page",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_are_beginner_friendly_with_default_then_advanced_path",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`.",
        );
    }
}

#[test]
fn color_swatch_picker_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
            "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
            "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
            "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
            "color_swatch_picker_check2_documents_docs_sync_and_state_matrix_rules",
            "color_swatch_picker_docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults",
            "scripts/check-ui-components-contract-hygiene.sh",
        ] {
            assert!(
                source.contains(required),
                "color-swatch-picker checklist should keep docs-sync/state-matrix rule `{required}`.",
            );
        }
    }
}

#[test]
fn color_swatch_picker_docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");

    for required in [
        "pub(super) fn color_swatch_picker() -> AnyView",
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "title=\"Transparency + Disabled + Custom Class\"",
        "swatches=signal(swatches.clone()).0",
        "swatches=signal(disabled_swatches.clone()).0",
        "ColorSwatchPickerItem::named(\"rgba(14, 116, 144, 0.4)\", \"Cyan 40%\").disabled(true)",
        "shape=ColorSwatchShape::Wide",
        "rounding=ColorSwatchRounding::Default",
        "default_selected_color=\"#f80\".to_string()",
        "selected_color=controlled_selected_color",
        "on_selected_change=Callback::new(move |next| {",
        "aria_label=\"Controlled swatch picker\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "color-swatch-picker docs should keep API/default/state-matrix marker `{required}`.",
        );
    }

    for required in [
        "#[prop(optional, into)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_color: Option<String>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional, default = true)] is_bordered: bool",
        "let selected_state = overlay_open::use_controllable_state(",
        "selected_color,",
        "Some(logic::sanitize_selected_color(default_selected_color)),",
        "on_selected_change,",
    ] {
        assert!(
            view_source.contains(required),
            "color-swatch-picker view contract should keep `{required}`.",
        );
    }

    for required in [
        "pub fn resolve_selection_mode_attr(is_controlled: bool) -> &'static str",
        "pub fn resolve_selection_init_source_attr(",
        "pub fn resolve_selection_source_attr(",
        "pub fn normalize_id_base(value: Option<String>) -> String",
        "normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ID_BASE.to_string())",
    ] {
        assert!(
            logic_source.contains(required),
            "color-swatch-picker logic normalization/default contract should keep `{required}`.",
        );
    }

    for forbidden in [
        "default_color=",
        "on_change=",
        "selected=controlled_selected_color",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "color-swatch-picker docs should avoid stale API alias `{forbidden}`.",
        );
    }
}

#[test]
fn color_swatch_picker_contract_hygiene_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`.",
        );
    }
}

#[test]
fn color_swatch_picker_docs_are_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_logic_source = include_str!("../../../components/code-block/src/logic.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");

    for marker in [
        "pub(super) fn color_swatch_picker() -> AnyView",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=color_swatch_picker_imports.clone()",
        "code_imports=color_swatch_picker_imports",
        "data-slot=\"color-swatch-picker-copy-ready\"",
        "data-slot=\"color-swatch-picker-source-prerequisites\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "components/color-swatch-picker/src/mod.rs",
        "components/color-swatch-picker/src/logic.rs",
        "components/color-swatch-picker/src/view.rs",
        "components/color-swatch-picker/src/styles.rs",
        "components/color-swatch-picker/src/motion.rs",
        "\"component-color_swatch_picker\"",
        "\"inject-css\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "color-swatch-picker docs should keep copy-ready marker `{marker}`.",
        );
    }

    for marker in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let missing_imports = missing_import_lines(&raw, &imports);",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "<CodeBlock code=resolved_code.get() />",
        "\"Show code\"",
    ] {
        assert!(
            playground_source.contains(marker),
            "docs-app playground should keep copy-ready pipeline marker `{marker}`.",
        );
    }

    for marker in [
        "pub const DEFAULT_IS_COPYABLE: bool = true;",
        "pub fn resolve_copyable_contract(",
        "is_copyable: DEFAULT_IS_COPYABLE,",
    ] {
        assert!(
            code_block_logic_source.contains(marker),
            "code-block copy contract should keep marker `{marker}` for docs copy action.",
        );
    }

    for marker in [
        "#[prop(optional, into)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_color: Option<String>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] is_disabled: bool",
        "pub fn normalize_is_disabled(",
        "pub fn resolve_selected_color(",
    ] {
        assert!(
            view_source.contains(marker) || logic_source.contains(marker),
            "color-swatch-picker source-first snippets should stay synced with implementation marker `{marker}`.",
        );
    }
}

#[test]
fn color_swatch_picker_check2_documents_docs_product_copy_paste_ready_contract() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for source in [check2_source, check2_source_mirror] {
        for marker in [
            "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
            "color_swatch_picker_docs_page_covers_primary_playgrounds",
            "color_swatch_picker_docs_playgrounds_lock_state_matrix_contract_values",
            "color_swatch_picker_docs_are_copy_paste_ready_with_imports_copy_button_and_sync",
            "color_swatch_picker_check2_documents_docs_product_copy_paste_ready_contract",
            "scripts/check-ui-components-contract-hygiene.sh",
        ] {
            assert!(
                source.contains(marker),
                "color-swatch-picker checklist should lock docs-product contract marker `{marker}`.",
            );
        }
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_docs_product_copy_paste_ready_contract";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should enforce docs-product contract marker `{script_needle}`.",
    );
}

#[test]
fn color_swatch_picker_contract_hygiene_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_page_covers_primary_playgrounds",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_playgrounds_lock_state_matrix_contract_values",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_are_copy_paste_ready_with_imports_copy_button_and_sync",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_docs_product_copy_paste_ready_contract",
    ] {
        assert!(
            script_source.contains(marker),
            "contract-hygiene script should enforce docs-product marker `{marker}`.",
        );
    }
}

#[test]
fn color_swatch_picker_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
            "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
            "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
            "文档代码与当前实现必须同步，防止示例漂移。",
            "color_swatch_picker_check2_documents_source_first_copy_paste_ready_rules",
            "color_swatch_picker_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
            "color_swatch_picker_contract_hygiene_script_covers_source_first_copy_paste_ready_contract",
            "scripts/check-ui-components-contract-hygiene.sh",
        ] {
            assert!(
                source.contains(required),
                "color-swatch-picker checklist should keep source-first copy-paste-ready marker `{required}`.",
            );
        }
    }
}

#[test]
fn color_swatch_picker_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_logic_source = include_str!("../../../components/code-block/src/logic.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");

    for required in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "let source_first_code = Signal::derive(move || {",
        "code_signal=source_first_code",
        "code_imports=color_swatch_picker_imports",
        "data-slot=\"color-swatch-picker-copy-ready\"",
        "data-slot=\"color-swatch-picker-source-prerequisites\"",
        "data-slot=\"color-swatch-picker-source-paths\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "\"component-color_swatch_picker\"",
        "\"inject-css\"",
        "components/color-swatch-picker/src/mod.rs",
        "components/color-swatch-picker/src/logic.rs",
        "components/color-swatch-picker/src/view.rs",
        "components/color-swatch-picker/src/styles.rs",
        "components/color-swatch-picker/src/motion.rs",
        "default_selected_color=\"#f80\".to_string()",
        "class_name=\"docs-color-swatch-picker-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "source-first docs should keep marker `{required}`.",
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let missing_imports = missing_import_lines(&raw, &imports);",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "<CodeBlock code=resolved_code.get() />",
        "\"Show code\"",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground copy-ready pipeline should keep marker `{required}`.",
        );
    }

    for required in [
        "pub const DEFAULT_IS_COPYABLE: bool = true;",
        "pub fn resolve_copyable_contract(",
        "is_copyable: DEFAULT_IS_COPYABLE,",
    ] {
        assert!(
            code_block_logic_source.contains(required),
            "code-block copy contract should keep marker `{required}`.",
        );
    }

    for required in [
        "#[prop(optional, into)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_color: Option<String>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "pub fn resolve_selected_color(",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "source-first snippets should stay synced with current API marker `{required}`.",
        );
    }

    for forbidden in [
        "default_color=",
        "on_change=",
        "selected=controlled_selected_color",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "source-first docs should avoid stale API alias `{forbidden}`.",
        );
    }
}

#[test]
fn color_swatch_picker_contract_hygiene_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(required),
            "contract-hygiene script should enforce source-first marker `{required}`.",
        );
    }
}

#[test]
fn color_swatch_picker_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
            "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
            "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
            "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
            "color_swatch_picker_check2_documents_heroui_benchmark_docs_sync_rules",
            "color_swatch_picker_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
            "color_swatch_picker_contract_hygiene_script_covers_heroui_benchmark_docs_sync_contract",
            "color_swatch_picker_check2_marks_heroui_benchmark_docs_sync_contract_complete",
            "docs/spec/heroui-parameter-design-strategy.md",
            "scripts/check-ui-components-contract-hygiene.sh",
        ] {
            assert!(
                source.contains(required),
                "color-swatch-picker checklist should keep heroui-benchmark docs-sync marker `{required}`.",
            );
        }
    }
}

#[test]
fn color_swatch_picker_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let readme_source = load_source("../../components/color-swatch-picker/src/README.md");

    for required in [
        "### ColorSwatchPicker 同步记录（2026-02-20）",
        "参数模型同步：`ColorSwatchPicker` 维持单选色板 primitive 定位",
        "component_doc!(\"ColorSwatchPicker\", \"color-swatch-picker\", \"Display\", display_extra::color_swatch_picker)",
        "#/components/color-swatch-picker",
        "`components/color-swatch-picker/src/README.md` 提供等价文档入口",
        "display_extra.rs::color_swatch_picker()",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入。",
    ] {
        assert!(
            strategy_source.contains(required),
            "heroui strategy doc should include color-swatch-picker synchronization marker `{required}`.",
        );
    }

    for required in [
        "component_doc!(",
        "\"ColorSwatchPicker\"",
        "\"color-swatch-picker\"",
        "display_extra::color_swatch_picker",
    ] {
        assert!(
            pages_source.contains(required),
            "component docs index should expose color-swatch-picker entry marker `{required}`.",
        );
    }

    for required in [
        "pub(super) fn color_swatch_picker() -> AnyView {",
        "title=\"ColorSwatchPicker\"",
        "slug=\"color-swatch-picker\"",
    ] {
        assert!(
            docs_source.contains(required),
            "docs-app color-swatch-picker page should stay indexable via marker `{required}`.",
        );
    }

    assert!(
        readme_source.contains("# ColorSwatchPicker"),
        "color-swatch-picker README should remain an equivalent component doc entry.",
    );
}

#[test]
fn color_swatch_picker_contract_hygiene_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_marks_heroui_benchmark_docs_sync_contract_complete",
    ] {
        assert!(
            script_source.contains(required),
            "contract-hygiene script should enforce heroui-benchmark docs-sync marker `{required}`.",
        );
    }
}

#[test]
fn color_swatch_picker_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
            "color_swatch_picker_check2_documents_heroui_benchmark_docs_sync_rules",
            "color_swatch_picker_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
            "color_swatch_picker_contract_hygiene_script_covers_heroui_benchmark_docs_sync_contract",
            "color_swatch_picker_check2_marks_heroui_benchmark_docs_sync_contract_complete",
            "docs/spec/heroui-parameter-design-strategy.md",
            "scripts/check-ui-components-contract-hygiene.sh",
        ] {
            assert!(
                source.contains(required),
                "color-swatch-picker check2 should keep heroui-benchmark docs-sync evidence marker `{required}`.",
            );
        }
    }
}

#[test]
fn color_swatch_picker_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
            "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
            "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
            "Playground 作为验收面，需可重复复现关键交互路径。",
            "color_swatch_picker_check2_documents_interactive_playground_rules",
            "color_swatch_picker_docs_interactive_playground_supports_live_props_state_and_feedback_preview",
            "color_swatch_picker_docs_interactive_playground_replay_path_is_explicit_and_repeatable",
            "color_swatch_picker_docs_interactive_playground_spec_linkage_is_not_applicable_for_non_spec_component",
            "scripts/check-ui-components-contract-hygiene.sh",
        ] {
            assert!(
                source.contains(required),
                "color-swatch-picker checklist should keep interactive playground marker `{required}`.",
            );
        }
    }
}

#[test]
fn color_swatch_picker_docs_interactive_playground_supports_live_props_state_and_feedback_preview()
{
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for required in [
        "title=\"Interactive Playground\"",
        "controls=move || {",
        "data-slot=\"color-swatch-picker-workbench-controls\"",
        "data-slot=\"color-swatch-picker-workbench-shape-control\"",
        "data-slot=\"color-swatch-picker-workbench-rounding-control\"",
        "data-slot=\"color-swatch-picker-workbench-selection-control\"",
        "data-slot=\"color-swatch-picker-workbench-mode-switch\"",
        "data-slot=\"color-swatch-picker-workbench-disabled-switch\"",
        "data-slot=\"color-swatch-picker-workbench-feedback\"",
        "Switch checked=workbench_use_controlled set_checked=set_workbench_use_controlled",
        "Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled",
        "Switch checked=workbench_is_bordered set_checked=set_workbench_is_bordered",
        "selected_color=workbench_selected_color",
        "default_selected_color=default_selected_color",
        "on_selected_change=Callback::new(move |next| {",
        "set_workbench_last_selected.set(next.clone());",
        "set_workbench_selected_index.set(Some(next_index));",
        "test_config_signal=workbench_actual_config",
        "test_source_path=\"components/color-swatch-picker/src/styles.rs\".to_string()",
        "mode={}, palette={}, last_selected={}, disabled={}, bordered={}",
    ] {
        assert!(
            docs_source.contains(required),
            "interactive playground should keep live props/state marker `{required}`.",
        );
    }
}

#[test]
fn color_swatch_picker_docs_interactive_playground_replay_path_is_explicit_and_repeatable() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_color_swatch_picker_contract.spec.mjs");

    for required in [
        "data-slot=\"color-swatch-picker-workbench-replay\"",
        "Replay path: focus Orange swatch, press ArrowRight, observe selected marker change.",
        "Toggle Controlled mode and repeat ArrowRight to verify controlled callback sync.",
        "Enable disabled palette and Disabled switch to verify blocked interaction branch.",
        "set_workbench_last_selected.set(next);",
        "set_workbench_selected_index.set(Some(next_index));",
    ] {
        assert!(
            docs_source.contains(required),
            "interactive playground replay contract should include `{required}`.",
        );
    }

    for required in [
        "key flow is repeatable and failures map to semantic breakpoints",
        "keyboard.press(\"ArrowRight\")",
        "data-selection-source\", \"interaction\"",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(required),
            "interactive acceptance replay should stay aligned with e2e marker `{required}`.",
        );
    }
}

#[test]
fn color_swatch_picker_docs_interactive_playground_spec_linkage_is_not_applicable_for_non_spec_component()
 {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    let component_mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");

    assert!(
        !docs_source.contains("Spec::new(")
            && !docs_source.contains("ColorSwatchPickerSpec")
            && !component_mod_source.contains("mod spec;")
            && !component_mod_source.contains("pub mod spec;"),
        "color-swatch-picker interactive playground should keep spec-linkage out of non-spec component scope.",
    );

    for source in [check2_source, check2_source_mirror] {
        assert!(
            source.contains("AI Spec 联动条款对该组件按 N/A 处理"),
            "interactive playground checklist evidence should document AI Spec clause as N/A for this component.",
        );
    }
}

#[test]
fn color_swatch_picker_contract_hygiene_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_interactive_playground_supports_live_props_state_and_feedback_preview",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_interactive_playground_replay_path_is_explicit_and_repeatable",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_interactive_playground_spec_linkage_is_not_applicable_for_non_spec_component",
    ] {
        assert!(
            script_source.contains(marker),
            "contract-hygiene script should enforce interactive-playground marker `{marker}`.",
        );
    }
}

#[test]
fn color_swatch_picker_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。（新增 `e2e/tests/docs_app_color_swatch_picker_contract.spec.mjs`，路由固定 `/#/components/color-swatch-picker`，并以 `body:not(:has(#boot))` 作为 wasm 稳定就绪等待；定位仅使用 `data-*`/`aria-*` 语义标记（如 `[data-component=\"color-swatch-picker\"] [data-slot=\"color-swatch-picker\"]` 与 `data-slot=\"color-swatch-picker-option\"`），禁用 `waitForTimeout`/`:nth-child`/文本定位；交互路径显式覆盖 ready/settled 语义断点（`data-selection-source`、`data-ui-action`、`data-ui-source`）与 disabled 分支。回归：`components/color-swatch-picker/test/semantics.rs::color_swatch_picker_check2_documents_e2e_selector_and_stable_wait_rules`、`components/color-swatch-picker/test/semantics.rs::color_swatch_picker_e2e_selector_contract_uses_semantic_markers_and_stable_waits`、`components/color-swatch-picker/test/semantics.rs::color_swatch_picker_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints`、`components/color-swatch-picker/test/color_swatch_picker_semantics.rs::color_swatch_picker_check2_documents_e2e_selector_and_stable_wait_rules`、`components/color-swatch-picker/test/color_swatch_picker_semantics.rs::color_swatch_picker_e2e_selector_contract_uses_semantic_markers_and_stable_waits`、`components/color-swatch-picker/test/color_swatch_picker_semantics.rs::color_swatch_picker_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints`；门禁脚本：`scripts/check-ui-components-e2e-color-swatch-picker.sh`。）",
            "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
            "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
            "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
            "color_swatch_picker_check2_documents_e2e_selector_and_stable_wait_rules",
            "color_swatch_picker_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
            "color_swatch_picker_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints",
            "scripts/check-ui-components-e2e-color-swatch-picker.sh",
        ] {
            assert!(
                source.contains(required),
                "check2 should keep e2e selector/stable-wait governance marker `{required}`.",
            );
        }
    }
}

#[test]
fn color_swatch_picker_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_swatch_picker_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-e2e-color-swatch-picker.sh");

    for required in [
        "const COLOR_SWATCH_PICKER_PAGE = \"/#/components/color-swatch-picker\";",
        "body:not(:has(#boot))",
        "[data-component=\"color-swatch-picker\"] [data-slot=\"color-swatch-picker\"][data-selection-mode=\"controlled\"][aria-label=\"Controlled swatch picker\"]",
        "data-slot=\"color-swatch-picker-list\"",
        "data-slot=\"color-swatch-picker-option\"",
        "data-ui-schema",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-output-status",
        "data-selection-mode",
        "data-selection-source",
        "toHaveAttribute(\"role\", \"radiogroup\")",
        "toHaveAttribute(\"role\", \"radio\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-swatch-picker e2e selector contract should include `{required}`.",
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
            "color-swatch-picker e2e contract should avoid brittle selector/wait token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_e2e_selector_contract_uses_semantic_markers_and_stable_waits";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-swatch-picker gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_swatch_picker_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_swatch_picker_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-e2e-color-swatch-picker.sh");

    for required in [
        "data-selection-init-source\", \"default\"",
        "data-selection-source\", \"default\"",
        "data-ui-action\", \"sync\"",
        "data-ui-state\", \"active\"",
        "option_green.click()",
        "data-selected-index\", \"2\"",
        "data-selection-source\", \"interaction\"",
        "data-ui-action\", \"select\"",
        "data-ui-source\", \"interaction\"",
        "const disabled_root = page",
        "data-has-disabled-items\", \"true\"",
        "aria-disabled\", \"true\"",
        "toBeDisabled()",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-swatch-picker e2e ready/settled contract should include `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-swatch-picker gate script should include `{script_needle}`.",
    );

    let check2_script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_e2e_selector_and_stable_wait_rules";
    assert!(
        script_source.contains(check2_script_needle),
        "e2e-color-swatch-picker gate script should include checklist governance marker `{check2_script_needle}`.",
    );
}

#[test]
fn color_swatch_picker_check2_documents_repeatable_e2e_regression_collection() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-e2e-color-swatch-picker.sh");

    for source in [check2_source, check2_source_mirror] {
        for required in [
            "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。（`e2e/tests/docs_app_color_swatch_picker_contract.spec.mjs` 新增可重复关键流程 `docs-app color-swatch-picker key flow is repeatable and failures map to semantic breakpoints`：固定路由进入后通过语义标记定位默认受控/非受控状态，执行键盘链路 `focus -> ArrowRight`，断言 `data-selected-index/data-selection-source/data-ui-action/data-ui-source`，随后 `page.reload()` 重放同路径并复验同一语义断点，确保失败可定位到具体契约字段。高风险路径新增 `docs-app color-swatch-picker high-risk paths keep focus keyboard and disabled branches semantically explicit`，覆盖 keyboard/focus 与 disabled 分支（`aria-disabled` + `toBeDisabled`）；overlay/async 在该组件职责下为 N/A。回归：`components/color-swatch-picker/test/semantics.rs::color_swatch_picker_check2_documents_repeatable_e2e_regression_collection`、`components/color-swatch-picker/test/semantics.rs::color_swatch_picker_e2e_key_flow_is_repeatable_and_failure_points_are_semantic`、`components/color-swatch-picker/test/semantics.rs::color_swatch_picker_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints`、`components/color-swatch-picker/test/color_swatch_picker_semantics.rs::color_swatch_picker_check2_documents_repeatable_e2e_regression_collection`、`components/color-swatch-picker/test/color_swatch_picker_semantics.rs::color_swatch_picker_e2e_key_flow_is_repeatable_and_failure_points_are_semantic`、`components/color-swatch-picker/test/color_swatch_picker_semantics.rs::color_swatch_picker_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints`；门禁脚本：`scripts/check-ui-components-e2e-color-swatch-picker.sh`。）",
            "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
            "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
            "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
            "color_swatch_picker_check2_documents_repeatable_e2e_regression_collection",
            "color_swatch_picker_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
            "color_swatch_picker_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
            "scripts/check-ui-components-e2e-color-swatch-picker.sh",
        ] {
            assert!(
                source.contains(required),
                "check2 should keep repeatable e2e regression governance marker `{required}`.",
            );
        }
    }

    for script_needle in [
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_repeatable_e2e_regression_collection",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(script_needle),
            "e2e-color-swatch-picker script should include repeatable/high-risk marker `{script_needle}`.",
        );
    }
}

#[test]
fn color_swatch_picker_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_swatch_picker_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-e2e-color-swatch-picker.sh");

    for required in [
        "key flow is repeatable and failures map to semantic breakpoints",
        "let uncontrolled_root = await resolveUncontrolledRoot(page);",
        "option_orange.focus()",
        "toBeFocused()",
        "keyboard.press(\"ArrowRight\")",
        "data-selected-index\", \"2\"",
        "data-selection-source\", \"interaction\"",
        "data-ui-action\", \"select\"",
        "data-ui-source\", \"interaction\"",
        "await page.reload();",
        "body:not(:has(#boot))",
    ] {
        assert!(
            e2e_source.contains(required),
            "repeatable color-swatch-picker e2e flow should include semantic breakpoint `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_e2e_key_flow_is_repeatable_and_failure_points_are_semantic";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-swatch-picker script should include `{script_needle}`.",
    );
}

#[test]
fn color_swatch_picker_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_swatch_picker_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-e2e-color-swatch-picker.sh");

    for required in [
        "high-risk paths keep focus keyboard and disabled branches semantically explicit",
        "option_orange.focus()",
        "toBeFocused()",
        "keyboard.press(\"ArrowLeft\")",
        "data-selected-index\", \"0\"",
        "data-selection-source\", \"interaction\"",
        "data-ui-action\", \"select\"",
        "data-ui-state\", \"active\"",
        "const disabled_root = page",
        "aria-disabled\", \"true\"",
        "toBeDisabled()",
    ] {
        assert!(
            e2e_source.contains(required),
            "high-risk color-swatch-picker e2e flow should include `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-swatch-picker script should include `{script_needle}`.",
    );
}

#[test]
fn color_swatch_picker_check2_documents_explicit_forbidden_antipattern_rules() {
    let check2_source = load_source("../../components/color-swatch-picker/src/check2.md");

    for needle in [
        "### 8. 明确禁止的反模式",
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
    ] {
        assert!(
            check2_source.contains(needle),
            "color-swatch-picker src/check2 should keep explicit forbidden-antipattern rule `{needle}`."
        );
    }
}

#[test]
fn color_swatch_picker_forbidden_antipatterns_keep_architecture_boundaries_intact() {
    let logic_source = load_source("../../components/color-swatch-picker/src/logic.rs");
    let view_source = load_source("../../components/color-swatch-picker/src/view.rs");
    let mod_source = load_source("../../components/color-swatch-picker/src/mod.rs");

    for forbidden in [
        "web_sys::",
        "wasm_bindgen",
        "leptos::",
        "use leptos",
        "class=",
        "style=",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "color-swatch-picker logic should stay primitive/normalization-only and avoid `{forbidden}`."
        );
    }

    for required in [
        "use_radio(RadioOptions {",
        "logic::resolve_state(",
        "logic::resolve_component_state(",
        "logic::resolve_selection_source_attr(",
    ] {
        assert!(
            view_source.contains(required),
            "color-swatch-picker view should mount headless + logic contract via `{required}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "web_sys::",
        "wasm_bindgen",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "color-swatch-picker public module boundary should avoid anti-pattern `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_picker_check2_documents_final_merge_gate_rules() {
    let check2_source = load_source("../../components/color-swatch-picker/src/check2.md");

    for needle in [
        "### 9. 合并门禁（最终裁决）",
        "- [x] 架构正确（边界不破）。",
        "- [x] 行为正确（状态与交互语义成立）。",
        "- [x] 可访问性达标（默认可用）。",
        "- [x] 默认主题美学质量达标（与可访问性同级门禁）。",
        "- [x] 可测试（契约可断言）。",
        "- [x] 可维护（命名和模式一致）。",
        "- [x] 可解释（人和自动化都能读懂）。",
        "- [x] 改动在正确层。",
        "- [x] 命名与全库一致。",
        "- [x] 无效状态被限制或归一化。",
        "- [x] 暴露必要语义标记。",
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
        "- [x] 文档与示例同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "color-swatch-picker src/check2 should keep final merge-gate rule `{needle}`."
        );
    }
}

#[test]
fn color_swatch_picker_final_merge_gate_capabilities_are_backed_by_contract_checks() {
    color_swatch_picker_forbidden_antipatterns_keep_architecture_boundaries_intact();
    color_swatch_picker_component_layer_keeps_file_responsibilities();
    color_swatch_picker_semantic_test_matrix_covers_key_paths_without_snapshot_reliance();
    color_swatch_picker_a11y_i18n_contract_is_headless_driven_and_no_view_text_hardcode();
    color_swatch_picker_visual_desire_gate_reuses_theme_visual_baseline_and_heroui_contracts();
    color_swatch_picker_type_system_and_semantic_markers_keep_machine_readable_contract();
    color_swatch_picker_state_markers_are_observable_and_source_enums_are_closed();
    color_swatch_picker_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe(
    );
    color_swatch_picker_ssr_cross_platform_contract_keeps_compile_only_matrix_and_feature_boundaries();
    color_swatch_picker_docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults();
}

#[test]
fn color_swatch_picker_check2_has_no_unchecked_checklist_items() {
    let check2_source = load_source("../../components/color-swatch-picker/check2.md");
    let check2_source_mirror = load_source("../../components/color-swatch-picker/src/check2.md");

    for source in [check2_source, check2_source_mirror] {
        assert!(
            !source.contains("- [ ]"),
            "color-swatch-picker check2 files should not keep unchecked checklist items after completion."
        );
    }
}

#[test]
fn color_swatch_picker_contract_hygiene_script_covers_forbidden_antipattern_and_final_gate_contract()
 {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_explicit_forbidden_antipattern_rules",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_forbidden_antipatterns_keep_architecture_boundaries_intact",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_final_merge_gate_rules",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_final_merge_gate_capabilities_are_backed_by_contract_checks",
        "cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_has_no_unchecked_checklist_items",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce color-swatch-picker gate marker `{needle}`."
        );
    }
}
