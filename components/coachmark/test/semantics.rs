use ui_test_support::source_contract;

fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "motion" => include_str!("../src/motion.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "primitive" => include_str!("../../../crates/ui-state-primitives/src/coachmark.rs"),
        "contextual_help_view" => include_str!("../../contextual-help/src/view.rs"),
        "contextual_help_logic" => include_str!("../../contextual-help/src/logic.rs"),
        "contextual_help_styles" => include_str!("../../contextual-help/src/styles.rs"),
        "popover_view" => include_str!("../../popover/src/view.rs"),
        "popover_logic" => include_str!("../../popover/src/logic.rs"),
        "popover_styles" => include_str!("../../popover/src/styles.rs"),
        "overlay_view" => include_str!("../../overlay/src/view.rs"),
        "focus_trap_source" => include_str!("../../../crates/ui-headless/src/focus_trap.rs"),
        "ui_components_css" => include_str!("../../../crates/ui/src/css.rs"),
        "ui_components_root" => include_str!("../../../crates/ui/src/root.rs"),
        "legacy_semantics" => {
            include_str!("../../../components/coachmark/test/coachmark_semantics.rs")
        }
        "theme_visual_baseline_page" => source_contract::source_from_file_relative(
            file!(),
            "../../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs",
        ),
        "theme_visual_baseline_e2e" => {
            include_str!("../../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs")
        }
        "heroui_strategy_doc" => {
            include_str!("../../../docs/spec/heroui-parameter-design-strategy.md")
        }
        "ui_components_manifest" => include_str!("../../../crates/ui/Cargo.toml"),
        "ui_components_lib" => include_str!("../../../crates/ui/src/lib.rs"),
        "tree_shaking_spec" => include_str!("../../../docs/spec/tree_shaking.md"),
        "tree_shaking_script" => {
            include_str!("../../../scripts/check-ui-tree-shaking.sh")
        }
        "tree_shaking_budget" => include_str!("../../../scripts/tree_shaking_budget.env"),
        "platform_script" => include_str!("../../../scripts/check-ui-platforms.sh"),
        "ci_workflow" => include_str!("../../../.github/workflows/ci.yml"),
        "web_demo_manifest" => include_str!("../../../apps/web-demo/Cargo.toml"),
        "docs_components_shell" => {
            include_str!("../../../apps/docs-app/src/pages/components/shell.rs")
        }
        "docs_perf_probe" => include_str!("../../../apps/docs-app/src/perf_probe.rs"),
        "docs_components_coverage_e2e" => {
            include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs")
        }
        "docs_debug_overlay" => include_str!("../../../apps/docs-app/src/debug_overlay.rs"),
        "todo_plan" => include_str!("../../../docs/plan/TODO.md"),
        "coachmark_e2e_contract" => {
            include_str!("../../../e2e/tests/docs_app_coachmark_contract.spec.mjs")
        }
        "coachmark_e2e_script" => {
            include_str!("../../../components/coachmark/scripts/check-ui-e2e-coachmark.sh")
        }
        "coachmark_readme" => include_str!("../src/README.md"),
        "docs_pages_catalog" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages.rs")
        }
        "docs_coachmark_page" => source_contract::source_from_file_relative(
            file!(),
            "../../../apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs",
        ),
        "coachmark_checklist" => include_str!("../check2.md"),
        "perf_script" => include_str!("../../../scripts/check-ui-performance.sh"),
        "headless_a11y" => include_str!("../../../crates/ui-headless/src/a11y.rs"),
        "headless_lib" => include_str!("../../../crates/ui-headless/src/lib.rs"),
        "headless_id_provider" => include_str!("../../../crates/ui-headless/src/id_provider.rs"),
        "ui_motion_lib" => include_str!("../../../crates/ui-motion/src/lib.rs"),
        "ui_motion_spring" => include_str!("../../../crates/ui-motion/src/spring.rs"),
        "ui_motion_non_wasm_stub_test" => {
            include_str!("../../../crates/ui-motion/tests/non_wasm_stub.rs")
        }
        "popover_position" => include_str!("../../../crates/ui-headless/src/popover_position.rs"),
        "popover_motion" => include_str!("../../popover/src/motion.rs"),
        "controllable_open" => {
            include_str!("../../../crates/ui-headless/src/controllable_state.rs")
        }
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn coachmark_keeps_internal_modules_private() {
    let module = load_source("mod");

    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !module.contains(forbidden),
            "Coachmark internals should stay private: `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_logic_consumes_state_primitives_without_reimplementation() {
    let logic = load_source("logic");
    let primitive = load_source("primitive");

    for required in [
        "pub use ui_state_primitives::coachmark::{",
        "CoachmarkStateInput",
        "CoachmarkState",
        "resolve_cta_mode",
        "resolve_asset_source",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic.contains(required),
            "Coachmark logic should consume state primitives via `{required}`."
        );
    }

    for forbidden in [
        "pub struct CoachmarkStateInput {",
        "pub struct CoachmarkState {",
        "pub fn resolve_cta_mode(",
        "pub fn resolve_asset_source(",
    ] {
        assert!(
            !logic.contains(forbidden),
            "Coachmark logic must not redefine primitives: `{forbidden}`."
        );
    }

    for forbidden in ["RwSignal<", "ReadSignal<", "WriteSignal<"] {
        assert!(
            !primitive.contains(forbidden),
            "ui-state-primitives::coachmark must stay framework-agnostic: `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_view_assembles_headless_and_component_layers() {
    let view = load_source("view");

    for required in [
        "use ui_headless::{A11yDirection, PopoverPlacement};",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "let view_model = logic::resolve_view_model(logic::CoachmarkViewModelInput {",
        "let state = view_model.state;",
        "<ContextualHelp",
        "motion=motion",
        "data-slot=\"coachmark-content\"",
    ] {
        assert!(
            view.contains(required),
            "Coachmark view should assemble layered contracts via `{required}`."
        );
    }

    for forbidden in ["web_sys::", "wasm_bindgen", "#[prop(optional)] state:"] {
        assert!(
            !view.contains(forbidden),
            "Coachmark public view should not expose platform-specific details: `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_styles_are_token_first_css() {
    let styles = load_source("styles");
    assert!(
        styles.contains("var(--ui-"),
        "Coachmark styles should consume ui-theme CSS variables."
    );
}

#[test]
fn coachmark_open_axis_supports_controlled_and_uncontrolled_contract() {
    let view = load_source("view");
    let controllable_open = load_source("controllable_open");

    for required in [
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "match (open, dir) {",
        "open=open",
        "default_open=default_open",
        "on_open_change=on_open_change",
    ] {
        assert!(
            view.contains(required),
            "Coachmark open axis must expose controllable triple contract marker `{required}`."
        );
    }

    for required in [
        "let (uncontrolled_value, set_uncontrolled_value) = signal(default_value.unwrap_or_default());",
        "let is_controlled = value.is_some();",
        "if !is_controlled {",
        "set_uncontrolled_value.set(next);",
    ] {
        assert!(
            controllable_open.contains(required),
            "ui-headless controllable state should keep single source of truth marker `{required}`."
        );
    }
}

#[test]
fn coachmark_defaults_are_normalized_in_logic_layer() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "pub struct CoachmarkViewModelInput {",
        "pub struct CoachmarkViewModel {",
        "CoachmarkAssetSource",
        "pub fn resolve_view_model(input: CoachmarkViewModelInput) -> CoachmarkViewModel {",
        "resolve_cta_mode",
        "resolve_asset_source",
        "pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool {",
        "is_disabled.or(disabled).unwrap_or(false)",
        "pub fn resolve_asset_label(asset_label: Option<String>) -> String {",
        "normalize_optional_text(asset_label).unwrap_or_else(|| DEFAULT_ASSET_LABEL.into())",
        "pub fn resolve_asset_alt(asset_alt: Option<String>, asset_label: &str) -> String {",
        "pub fn resolve_default_open(default_open: Option<bool>) -> bool {",
        "default_open.unwrap_or(false)",
        "pub fn resolve_on_open_change(on_open_change: Option<Callback<bool>>) -> Callback<bool> {",
        "pub fn resolve_on_press(on_press: Option<OnPress>) -> OnPress {",
    ] {
        assert!(
            logic.contains(required),
            "Coachmark logic should own default normalization marker `{required}`."
        );
    }

    for required in [
        "let view_model = logic::resolve_view_model(logic::CoachmarkViewModelInput {",
        "let class_name = StoredValue::new(view_model.class_name);",
        "let trigger_label = StoredValue::new(view_model.trigger_label);",
        "let heading = StoredValue::new(view_model.heading);",
        "let step_label = StoredValue::new(view_model.step_label);",
        "let on_primary = logic::resolve_on_press(on_primary);",
        "let default_open = logic::resolve_default_open(default_open);",
    ] {
        assert!(
            view.contains(required),
            "Coachmark view should consume logic-level defaults via `{required}`."
        );
    }
}

#[test]
fn coachmark_state_uses_enum_typed_discrete_axes() {
    let logic = load_source("logic");
    let primitive = load_source("primitive");

    for required in [
        "pub enum CoachmarkCtaMode {",
        "pub enum CoachmarkAssetSource {",
        "pub fn resolve_cta_mode(",
        "pub fn resolve_asset_source(",
        "pub cta_mode: CoachmarkCtaMode,",
        "pub asset_source: CoachmarkAssetSource,",
        "let cta_mode = resolve_cta_mode(",
        "asset_source,",
    ] {
        assert!(
            primitive.contains(required) || logic.contains(required),
            "Coachmark should type discrete axes with enum marker `{required}`."
        );
    }
}

#[test]
fn coachmark_async_protocol_is_not_applicable_and_not_implemented() {
    let view = load_source("view");
    let contextual_help_view = load_source("contextual_help_view");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "on_retry",
        "on_error",
        "use_async_action",
        "retry",
    ] {
        assert!(
            !view.contains(forbidden),
            "Coachmark should not expose async protocol marker `{forbidden}` when async is N/A."
        );
        assert!(
            !contextual_help_view.contains(forbidden),
            "ContextualHelp should not expose async protocol marker `{forbidden}` in coachmark composition path."
        );
    }
}

#[test]
fn coachmark_keeps_explicit_children_composition_without_parallel_item_arrays() {
    let view = load_source("view");

    for required in [
        "children: ChildrenFn,",
        "<div class=\"ui-coachmark__body\" data-slot=\"coachmark-body\">",
        "{children()}",
    ] {
        assert!(
            view.contains(required),
            "Coachmark should keep explicit composition marker `{required}`."
        );
    }

    for forbidden in [
        "labels:",
        "titles:",
        "panels:",
        "item_specs:",
        "ItemSpec",
        "items: Vec<",
    ] {
        assert!(
            !view.contains(forbidden),
            "Coachmark should not introduce parallel-item API marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_has_no_dragging_macro_micro_state_machine_path() {
    let view = load_source("view");
    let logic = load_source("logic");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "on:drag",
        "on:dragstart",
        "on:dragend",
        "on:pointermove",
        "on:mousemove",
        "requestAnimationFrame",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "Coachmark should not expose drag macro/micro state machine marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_overlay_uses_two_pass_measure_and_idempotent_rectification() {
    let contextual_help_view = load_source("contextual_help_view");
    let popover_view = load_source("popover_view");
    let popover_position = load_source("popover_position");

    for required in [
        "<Popover",
        "use_popover_position(PopoverPositionOptions {",
        "anchor_ref=anchor_ref",
        "placement=placement",
        "let anchor_rect = anchor_el.get_bounding_client_rect();",
        "let panel_rect = panel_el.get_bounding_client_rect();",
        "let computed = compute_popover_position(",
        "if should_update_scalar(top_px.get_untracked(), computed.top) {",
        "if should_update_scalar(left_px.get_untracked(), computed.left) {",
        "if placement.get_untracked() != computed.placement {",
    ] {
        assert!(
            contextual_help_view.contains(required)
                || popover_view.contains(required)
                || popover_position.contains(required),
            "Coachmark overlay geometry should keep two-pass + convergence marker `{required}`."
        );
    }
}

#[test]
fn coachmark_has_no_collection_registration_protocol_surface() {
    let view = load_source("view");
    let logic = load_source("logic");
    let contextual_help_view = load_source("contextual_help_view");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "roving",
        "menuitem",
        "tablist",
        "accordion",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !contextual_help_view.contains(forbidden),
            "Coachmark should not expose collection registration protocol marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_slot_projection_does_not_expose_keepalive_protocol_surface() {
    let view = load_source("view");
    let logic = load_source("logic");
    let contextual_help_view = load_source("contextual_help_view");

    for required in [
        "use ui_headless::use_presence;",
        "<Show when=move || presence.is_present.get()>",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            contextual_help_view.contains(required),
            "Coachmark composition should keep presence-driven unmount marker `{required}`."
        );
    }

    for forbidden in [
        "projection_mode",
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "on_hidden",
        "pause_polling",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !contextual_help_view.contains(forbidden),
            "Coachmark should not expose slot projection mode marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_does_not_handle_raw_env_streams_in_component_layer() {
    let view = load_source("view");
    let logic = load_source("logic");
    let contextual_help_view = load_source("contextual_help_view");
    let popover_view = load_source("popover_view");

    for required in [
        "let position = use_popover_position(PopoverPositionOptions {",
        "<Popover",
    ] {
        assert!(
            contextual_help_view.contains(required) || popover_view.contains(required),
            "Coachmark composition should delegate environment measurement via `{required}`."
        );
    }

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "window.add_event_listener",
        "match_media",
        "BreakpointChanged",
        "on:resize",
        "on:scroll",
        "on:intersection",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !contextual_help_view.contains(forbidden),
            "Coachmark component layer should not expose raw env stream marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_has_no_event_light_cone_batch_collection_surface() {
    let view = load_source("view");
    let logic = load_source("logic");
    let contextual_help_view = load_source("contextual_help_view");

    for forbidden in [
        "ContextBus",
        "context_bus",
        "SelectorBus",
        "SelectionState::All",
        "SelectionState",
        "bulk_select",
        "batch_select",
        "prop_drilling",
        "table_row",
        "grid_row",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !contextual_help_view.contains(forbidden),
            "Coachmark should not expose event-light-cone marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_has_no_causality_bus_traceid_surface() {
    let view = load_source("view");
    let logic = load_source("logic");
    let contextual_help_view = load_source("contextual_help_view");

    for required in ["on_open_change", "on_primary", "on_secondary"] {
        assert!(
            view.contains(required) || logic.contains(required),
            "Coachmark should keep direct callback causality marker `{required}`."
        );
    }

    for forbidden in [
        "TraceId",
        "trace_id",
        "ContextBus",
        "context_bus",
        "broadcast",
        "subscriber",
        "publish(",
        "dispatch(",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !contextual_help_view.contains(forbidden),
            "Coachmark should not expose causality-bus marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_a11y_i18n_contract_is_wired_through_headless_without_view_copy_hardcode() {
    let view = load_source("view");
    let contextual_help_view = load_source("contextual_help_view");
    let headless_a11y = load_source("headless_a11y");

    for required in [
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "lang=lang.clone()",
        "dir=dir",
        "role=\"status\"",
        "aria-live=\"polite\"",
    ] {
        assert!(
            view.contains(required),
            "Coachmark view should keep a11y/i18n marker `{required}`."
        );
    }

    for required in [
        "let panel_a11y = ui_headless::overlay_dialog_attrs(",
        "aria_haspopup=\"dialog\"",
        "aria_expanded=open",
        "role=\"dialog\"",
        "aria-labelledby=panel_aria_labelledby.get_value()",
        "aria-describedby=panel_aria_describedby.get_value()",
        "lang=panel_lang.get_value()",
        "dir=panel_dir",
    ] {
        assert!(
            contextual_help_view.contains(required),
            "ContextualHelp should attach headless a11y contract marker `{required}`."
        );
    }

    for required in [
        "pub enum A11yDirection {",
        "pub fn overlay_dialog_attrs(",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            headless_a11y.contains(required),
            "Shared ui-headless a11y utility should provide `{required}`."
        );
    }

    for forbidden in ["\"Back\"", "\"Next\"", "\"Dismiss\"", "\"Understood\""] {
        assert!(
            !view.contains(forbidden),
            "Coachmark view should not hardcode user-visible copy marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_state_markers_are_observable_queryable_and_enumerated() {
    let coachmark_view = load_source("view");
    let contextual_help_view = load_source("contextual_help_view");
    let contextual_help_logic = load_source("contextual_help_logic");

    for required in [
        "data-state=state.state_attr",
        "data-open-mode=state.open_mode_attr",
        "data-open-source=state.open_source_attr",
        "data-default-open-source=state.default_open_source_attr",
        "data-open-change-source=state.open_change_source_attr",
        "data-open-interaction-source=move || open_interaction_source.get().as_attr()",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "aria_expanded=open",
        "aria_haspopup=\"dialog\"",
        "role=\"dialog\"",
    ] {
        assert!(
            coachmark_view.contains(required) || contextual_help_view.contains(required),
            "Coachmark composition should expose stable observable marker `{required}`."
        );
    }

    for required in [
        "open_mode_attr: if input.is_controlled {",
        "\"controlled\"",
        "\"uncontrolled\"",
        "open_source_attr: if input.has_custom_open {",
        "\"custom\"",
        "\"default\"",
        "default_open_source_attr: if input.has_custom_default_open {",
        "\"provided\"",
        "\"implicit\"",
        "open_change_source_attr: if input.has_custom_on_open_change {",
        "\"none\"",
        "RwSignal::new(ContextualHelpOpenInteractionSource::Initial)",
        "ContextualHelpOpenInteractionIntent::TriggerPress",
        "ContextualHelpOpenInteractionIntent::DismissPress",
        "open_interaction_source.set(sync.next_source);",
        "open_interaction_source_for_trigger.set(intent.next_source);",
        "open_interaction_source_for_close.set(intent.next_source);",
    ] {
        assert!(
            contextual_help_logic.contains(required) || contextual_help_view.contains(required),
            "State/source markers should stay in enumerable closed-set contract `{required}`."
        );
    }
}

#[test]
fn coachmark_styles_depend_on_explicit_state_markers_not_fragile_dom_guesses() {
    let coachmark_styles = load_source("styles");
    let contextual_help_styles = load_source("contextual_help_styles");
    let popover_styles = load_source("popover_styles");
    let coachmark_view = load_source("view");
    let contextual_help_view = load_source("contextual_help_view");
    let popover_view = load_source("popover_view");
    let popover_logic = load_source("popover_logic");

    for required in [
        ".ui-coachmark[data-state=\"disabled\"]",
        ".ui-coachmark[data-state=\"enabled\"]",
        ".ui-coachmark[data-asset=\"present\"] .ui-coachmark__content",
        ".ui-coachmark[data-cta=\"none\"] .ui-coachmark__actions",
        ".ui-coachmark[data-variant=\"help\"]",
        ".ui-contextual-help[data-state=\"enabled\"]",
        ".ui-contextual-help[data-state=\"disabled\"]",
        ".ui-contextual-help[data-placement=\"bottom-start\"]",
        ".ui-contextual-help[data-heading=\"absent\"] .ui-contextual-help__panel",
        ".ui-contextual-help__panel[data-footer=\"present\"] .ui-contextual-help__footer",
        ".ui-popover[data-state=\"open\"]",
        ".ui-popover[data-state=\"closed\"]",
        ".ui-popover__panel[data-placement=\"bottom-start\"]",
        ".ui-popover__panel[data-state=\"panel\"]",
    ] {
        assert!(
            coachmark_styles.contains(required)
                || contextual_help_styles.contains(required)
                || popover_styles.contains(required),
            "Styles should branch via explicit semantic selector `{required}`."
        );
    }

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        ":first-child",
        ":last-child",
    ] {
        assert!(
            !coachmark_styles.contains(forbidden)
                && !contextual_help_styles.contains(forbidden)
                && !popover_styles.contains(forbidden),
            "State styling must not rely on fragile structural selector `{forbidden}`."
        );
    }

    assert!(
        !coachmark_view.contains("style=") && !contextual_help_view.contains("style="),
        "Coachmark/ContextualHelp should not embed runtime business styling via inline style."
    );
    assert!(
        popover_view.contains("style=panel_vars"),
        "Overlay runtime style should be constrained to CSS variable transport."
    );
    assert!(
        popover_logic.contains("--ui-popover-top:")
            && popover_logic.contains("--ui-popover-left:")
            && popover_logic.contains("--ui-popover-anchor-width:"),
        "Popover runtime style payload should carry only explicit CSS custom properties."
    );
}

#[test]
fn coachmark_semantics_tests_cover_key_contract_paths_not_visual_snapshot_only() {
    let coachmark_view = load_source("view");
    let primitive = load_source("primitive");
    let contextual_help_view = load_source("contextual_help_view");
    let contextual_help_logic = load_source("contextual_help_logic");
    let popover_view = load_source("popover_view");
    let local_semantics = include_str!("../test/semantics.rs");
    let legacy_semantics = load_source("legacy_semantics");

    for required in [
        "role=\"dialog\"",
        "aria_expanded=open",
        "aria_haspopup=\"dialog\"",
        "aria-labelledby=panel_aria_labelledby.get_value()",
        "aria-describedby=panel_aria_describedby.get_value()",
        "data-state=state.state_attr",
        "data-open-mode=state.open_mode_attr",
        "data-open-source=state.open_source_attr",
        "data-default-open-source=state.default_open_source_attr",
        "data-open-change-source=state.open_change_source_attr",
    ] {
        assert!(
            coachmark_view.contains(required) || contextual_help_view.contains(required),
            "Semantic contract tests should cover role/aria/data/source marker `{required}`."
        );
    }

    for required in [
        "open_mode_attr: if input.is_controlled {",
        "\"controlled\"",
        "\"uncontrolled\"",
        "state_attr: if input.disabled {",
        "\"disabled\"",
        "\"enabled\"",
    ] {
        assert!(
            primitive.contains(required) || contextual_help_logic.contains(required),
            "Semantic contract tests should cover controlled/uncontrolled + disabled marker `{required}`."
        );
    }

    for required in [
        "on:keydown=on_key_down",
        "if logic::should_close_on_escape(",
        "on:click=move |_| on_close.run(())",
        "on:pointerdown=move |ev| ev.stop_propagation()",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            popover_view.contains(required),
            "Semantic contract tests should cover keyboard/pointer + wasm/non-wasm marker `{required}`."
        );
    }

    let snapshot_macro = ["assert", "_snapshot"].concat();
    let insta_prefix = ["insta", "::"].concat();
    let to_match_macro = ["to_match", "_snapshot"].concat();
    let snapshot_bang = ["snapshot", "!"].concat();

    for forbidden in [
        &snapshot_macro,
        &insta_prefix,
        &to_match_macro,
        &snapshot_bang,
    ] {
        assert!(
            !local_semantics.contains(forbidden.as_str())
                && !legacy_semantics.contains(forbidden.as_str()),
            "Coachmark contract tests should not rely on visual snapshot assertion `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_check2_documents_semantics_first_testing_rules() {
    let check2_source = load_source("coachmark_checklist");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark check2 should keep semantics-first testing rule `{required}`."
        );
    }
}

#[test]
fn coachmark_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = include_str!("../test/semantics.rs");
    let logic_source = load_source("logic");
    let module_source = load_source("mod");

    for required in [
        "coachmark_semantics_tests_cover_key_contract_paths_not_visual_snapshot_only",
        "coachmark_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "coachmark_a11y_i18n_contract_is_wired_through_headless_without_view_copy_hardcode",
        "coachmark_state_observability_uses_stable_data_and_aria_markers",
    ] {
        assert!(
            semantics_source.contains(required),
            "coachmark semantics suite should keep contract-first assertion `{required}`."
        );
    }

    for required in [
        "pub fn resolve_view_model(input: CoachmarkViewModelInput) -> CoachmarkViewModel {",
        "pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool {",
        "pub fn resolve_asset_label(asset_label: Option<String>) -> String {",
    ] {
        assert!(
            logic_source.contains(required),
            "coachmark logic tests should keep semantic state-axis regression `{required}`."
        );
    }

    assert!(
        module_source.contains("#[path = \"../test/semantics.rs\"]"),
        "coachmark module should keep `*_semantics.rs` test entry point."
    );

    let insta_prefix = ["insta", "::"].concat();
    let snapshot_bang = ["assert", "_snapshot", "!"].concat();
    let debug_snapshot_bang = ["assert_debug", "_snapshot", "!"].concat();
    let to_match_call = [".to_match", "_snapshot("].concat();

    for forbidden in [
        insta_prefix.as_str(),
        snapshot_bang.as_str(),
        debug_snapshot_bang.as_str(),
        to_match_call.as_str(),
    ] {
        assert!(
            !semantics_source.contains(forbidden) && !logic_source.contains(forbidden),
            "coachmark semantics suite should not rely on snapshot-only assertion `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let coachmark_view = load_source("view");
    let contextual_help_view = load_source("contextual_help_view");
    let local_semantics = include_str!("../test/semantics.rs");

    for marker in [
        "data-state=state.state_attr",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-output-status=agent_contract.output_status.as_str()",
        "role=\"dialog\"",
        "aria-labelledby=panel_aria_labelledby.get_value()",
        "aria-describedby=panel_aria_describedby.get_value()",
    ] {
        assert!(
            coachmark_view.contains(marker) || contextual_help_view.contains(marker),
            "coachmark runtime should expose semantic marker `{marker}`."
        );
        assert!(
            local_semantics.contains(marker) || contextual_help_view.contains(marker),
            "coachmark semantics tests should cover semantic marker `{marker}` changes."
        );
    }
}

#[test]
fn coachmark_semantics_first_testing_script_covers_contract() {
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for marker in [
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_semantics_first_testing_script_covers_contract",
    ] {
        assert!(
            script_source.contains(marker),
            "contract-hygiene script should include `{marker}`."
        );
    }
}

#[test]
fn coachmark_check2_marks_semantics_first_testing_contract_complete() {
    let check2_source = load_source("coachmark_checklist");

    for marker in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "coachmark_check2_documents_semantics_first_testing_rules",
        "coachmark_semantics_suite_is_contract_first_not_snapshot_only",
        "coachmark_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
        "scripts/check-ui-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "coachmark check2 should include semantics-first testing evidence `{marker}`."
        );
    }
}

#[test]
fn coachmark_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("coachmark_checklist");

    for marker in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2_source.contains(marker),
            "coachmark check2 should keep e2e selector stability rule `{marker}`."
        );
    }
}

#[test]
fn coachmark_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("coachmark_e2e_contract");
    let docs_page_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs",
    );

    for marker in [
        "const WASM_READY_SELECTOR = \"body:not(:has(#boot))\";",
        "const COACHMARK_CONTENT_SELECTOR =",
        "const COACHMARK_CONTROLLED_TOGGLE_SELECTOR = '[data-slot=\"coachmark-controlled-toggle\"]';",
        "waitForCoachmarkReady(page)",
        "waitForCoachmarkSettled(root)",
        "[data-slot=\"coachmark-content\"]",
        "data-ui-output-status\", \"verified\"",
        "data-ui-stream-mode\", \"snapshot\"",
        "data-stream-fallback\", \"snapshot\"",
    ] {
        assert!(
            e2e_source.contains(marker),
            "coachmark e2e selector/stable-wait contract should include `{marker}`."
        );
    }

    for marker in [
        "data-slot=\"coachmark-controlled-toggle\"",
        "data-slot=\"coachmark-controlled-actions\"",
        "data-slot=\"coachmark-controlled-vs-uncontrolled\"",
        "data-slot=\"coachmark-streaming-modes\"",
    ] {
        assert!(
            docs_page_source.contains(marker),
            "coachmark docs should keep e2e semantic anchor `{marker}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "getByText(",
        "locator(\"text=",
        ":nth-child(",
        ":nth-of-type(",
        "toHaveScreenshot(",
        "toMatchSnapshot(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "coachmark e2e contract should avoid flaky/text/snapshot selector token `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths() {
    let e2e_source = load_source("coachmark_e2e_contract");

    for marker in [
        "await waitForCoachmarkReady(page);",
        "await waitForCoachmarkSettled(root);",
        "await toggle.click();",
        "await page.reload();",
        "await page.locator(WASM_READY_SELECTOR).waitFor();",
        "await expect(content).toHaveAttribute(\"data-open-mode\", \"uncontrolled\");",
        "await expect(content).toHaveAttribute(\"data-ui-source\", \"external\");",
    ] {
        assert!(
            e2e_source.contains(marker),
            "coachmark e2e ready/settled contract should include `{marker}`."
        );
    }
}

#[test]
fn coachmark_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script_source = load_source("coachmark_e2e_script");

    for marker in [
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths",
    ] {
        assert!(
            script_source.contains(marker),
            "coachmark e2e check script should include `{marker}`."
        );
    }
}

#[test]
fn coachmark_check2_marks_e2e_selector_stability_item_complete() {
    let check2_source = load_source("coachmark_checklist");

    for marker in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "e2e/tests/docs_app_coachmark_contract.spec.mjs",
        "components/coachmark/scripts/check-ui-e2e-coachmark.sh",
        "components/coachmark/test/semantics.rs::coachmark_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/coachmark/test/semantics.rs::coachmark_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "components/coachmark/test/semantics.rs::coachmark_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths",
        "components/coachmark/test/semantics.rs::coachmark_e2e_check_script_covers_selector_and_settled_wait_contract",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "coachmark check2 should include e2e selector stability evidence marker `{marker}`."
        );
    }
}

#[test]
fn coachmark_check2_documents_repeatable_key_flow_e2e_regression_rules() {
    let check2_source = load_source("coachmark_checklist");

    for marker in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(marker),
            "coachmark check2 should keep repeatable key-flow e2e governance marker `{marker}`."
        );
    }
}

#[test]
fn coachmark_e2e_key_flow_regression_covers_overlay_focus_keyboard_with_semantic_breakpoints() {
    let e2e_source = load_source("coachmark_e2e_contract");

    for marker in [
        "const COACHMARK_CONTROLLED_CONTEXTUAL_HELP_SELECTOR =",
        "const COACHMARK_POPOVER_ROOT_OPEN_SELECTOR = '[data-slot=\"popover\"][data-state=\"open\"]';",
        "const COACHMARK_POPOVER_PANEL_SELECTOR = '[data-slot=\"popover-panel\"][data-state=\"panel\"]';",
        "const COACHMARK_DIALOG_PANEL_SELECTOR = '[data-slot=\"contextual-help-panel\"][role=\"dialog\"]';",
        "test(\"docs-app coachmark key flow is repeatable with overlay focus and keyboard dismissal\"",
        "await toggle.click();",
        "await popoverPanel.focus();",
        "await page.keyboard.press(\"Escape\");",
        "await expect(controlledContextualHelp).toHaveAttribute(\"data-open-interaction-source\", \"dismiss-press\");",
        "await expect(controlledContextualHelp).toHaveAttribute(\"data-ui-action\", \"dismiss\");",
        "await expect(page.locator(COACHMARK_POPOVER_ROOT_OPEN_SELECTOR)).toHaveCount(0);",
        "await page.keyboard.press(\"Enter\");",
        "await waitForCoachmarkSettled(reopenedContent);",
    ] {
        assert!(
            e2e_source.contains(marker),
            "coachmark repeatable key-flow e2e contract should include `{marker}`."
        );
    }

    for forbidden in ["waitForTimeout(", "toHaveScreenshot(", "toMatchSnapshot("] {
        assert!(
            !e2e_source.contains(forbidden),
            "coachmark repeatable key-flow e2e should avoid flaky assertion token `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_e2e_check_script_covers_repeatable_key_flow_regression_contract() {
    let script_source = load_source("coachmark_e2e_script");

    for marker in [
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_repeatable_key_flow_e2e_regression_rules",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_e2e_key_flow_regression_covers_overlay_focus_keyboard_with_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(marker),
            "coachmark e2e check script should include repeatable key-flow marker `{marker}`."
        );
    }
}

#[test]
fn coachmark_check2_marks_repeatable_key_flow_e2e_item_complete() {
    let check2_source = load_source("coachmark_checklist");

    for marker in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "e2e/tests/docs_app_coachmark_contract.spec.mjs",
        "components/coachmark/scripts/check-ui-e2e-coachmark.sh",
        "components/coachmark/test/semantics.rs::coachmark_check2_documents_repeatable_key_flow_e2e_regression_rules",
        "components/coachmark/test/semantics.rs::coachmark_e2e_key_flow_regression_covers_overlay_focus_keyboard_with_semantic_breakpoints",
        "components/coachmark/test/semantics.rs::coachmark_e2e_check_script_covers_repeatable_key_flow_regression_contract",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "coachmark check2 should include repeatable key-flow e2e evidence marker `{marker}`."
        );
    }
}

#[test]
fn coachmark_component_files_follow_single_responsibility_boundaries() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let motion = load_source("motion");

    for required in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::CoachmarkMotion;",
        "pub use view::Coachmark;",
    ] {
        assert!(
            module.contains(required),
            "Coachmark module boundary should include `{required}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub mod motion;",
        "#[component]",
        "view! {",
    ] {
        assert!(
            !module.contains(forbidden),
            "mod.rs should keep minimal exports and avoid implementation detail `{forbidden}`."
        );
    }

    for forbidden in ["web_sys::", "NodeRef<", "view! {", "style=", "on:click"] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should not contain DOM/render/style/event detail `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-coachmark[data-state=\"disabled\"]",
    ] {
        assert!(
            styles.contains(required),
            "styles.rs should stay static token-first css and include `{required}`."
        );
    }
    for forbidden in ["#[component]", "fn resolve_view_model(", "on:click"] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should not carry logic/view implementation `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "let view_model = logic::resolve_view_model(",
        "<ContextualHelp",
        "data-state=state.state_attr",
    ] {
        assert!(
            view.contains(required),
            "view.rs should render and mount headless contract via `{required}`."
        );
    }
    for forbidden in ["web_sys::", "wasm_bindgen"] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not expose platform-specific detail `{forbidden}`."
        );
    }

    for required in [
        "pub struct CoachmarkMotion {",
        "pub popover: PopoverMotion,",
        "pub fn sanitize_motion(motion: CoachmarkMotion) -> CoachmarkMotion {",
        "pub fn resolve_motion(motion: CoachmarkMotion) -> ContextualHelpMotion {",
        "ContextualHelpMotion {",
        "popover: motion.popover,",
    ] {
        assert!(
            motion.contains(required),
            "motion.rs should map component motion contract via `{required}`."
        );
    }
    for forbidden in ["web_sys::", "requestAnimationFrame", "spring", "keyframe"] {
        assert!(
            !motion.contains(forbidden),
            "motion.rs should not re-implement generic engine detail `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_does_not_introduce_spec_rs_for_simple_component_contract() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let motion = load_source("motion");

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "Spec::new(",
        "CoachmarkSpec",
        "spec_version",
        "schema_registry",
        "migrate_v",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !motion.contains(forbidden),
            "Coachmark should not expose `spec.rs` surface for simple component marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_component_directory_standard_files_follow_contract_and_na_paths() {
    let check2_source = load_source("coachmark_checklist");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let popover_motion = load_source("popover_motion");
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required_file).exists(),
            "coachmark component directory should include `{required_file}`."
        );
    }
    for absent_file in ["render.rs", "spec.rs"] {
        assert!(
            !src_dir.join(absent_file).exists(),
            "coachmark component directory should keep `{absent_file}` absent."
        );
    }

    for required in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::CoachmarkMotion;",
        "pub use view::Coachmark;",
    ] {
        assert!(
            mod_source.contains(required),
            "coachmark mod.rs should keep minimal stable export marker `{required}`."
        );
    }
    for forbidden in [
        "pub mod logic;",
        "pub mod motion;",
        "pub mod view;",
        "mod spec;",
        "mod render;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "coachmark mod.rs should not over-export or drift to `{forbidden}`."
        );
    }

    for required in [
        "pub struct CoachmarkViewModelInput",
        "pub struct CoachmarkViewModel",
        "pub fn resolve_view_model(",
        "pub fn resolve_default_open(",
        "pub fn resolve_on_open_change(",
    ] {
        assert!(
            logic_source.contains(required),
            "coachmark logic.rs should keep normalized state derivation marker `{required}`."
        );
    }
    for forbidden in ["view! {", "NodeRef<", "web_sys::", "window()", "document()"] {
        assert!(
            !logic_source.contains(forbidden),
            "coachmark logic.rs should stay free of DOM/platform token `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-coachmark[data-state=\"disabled\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "coachmark styles.rs should keep token-first CSS marker `{required}`."
        );
    }
    for forbidden in ["#[component]", "use ui_headless", "use leptos", "on:click="] {
        assert!(
            !styles_source.contains(forbidden),
            "coachmark styles.rs should avoid render/headless concern `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "let view_model = logic::resolve_view_model(",
        "let default_open = logic::resolve_default_open(default_open);",
        "let on_open_change = logic::resolve_on_open_change(on_open_change);",
        "let render_contextual_help = |open: Option<Signal<bool>>, has_footer: bool| -> AnyView {",
        "<ContextualHelp",
        "lang=lang.clone()",
        "dir=dir",
    ] {
        assert!(
            view_source.contains(required),
            "coachmark view.rs should keep render + headless mount marker `{required}`."
        );
    }
    for forbidden in ["mod render;", "include!(\"render.rs\")"] {
        assert!(
            !view_source.contains(forbidden),
            "coachmark view.rs should not drift to render split marker `{forbidden}`."
        );
    }

    for required in [
        "pub struct CoachmarkMotion",
        "pub fn sanitize_motion(motion: CoachmarkMotion) -> CoachmarkMotion",
        "pub fn resolve_motion(motion: CoachmarkMotion) -> ContextualHelpMotion",
    ] {
        assert!(
            motion_source.contains(required),
            "coachmark motion.rs should keep semantic->motion mapping marker `{required}`."
        );
    }
    for forbidden in ["view! {", "use_contextual_help(", "role=", "aria-"] {
        assert!(
            !motion_source.contains(forbidden),
            "coachmark motion.rs should avoid view/headless concern `{forbidden}`."
        );
    }
    assert!(
        popover_motion.contains("pub fn attach_motion("),
        "coachmark motion attach should stay delegated to popover motion contract."
    );

    let script_needle = "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "coachmark_component_directory_standard_files_follow_contract_and_na_paths",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark checklist should keep component-directory governance marker `{required}`."
        );
    }
}

#[test]
fn coachmark_file_placement_discipline_contract_is_explicit_for_interactive_component_scope() {
    let check2_source = load_source("coachmark_checklist");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");

    assert!(
        check2_source.contains("文件落点纪律"),
        "coachmark check2 should explicitly track file-placement discipline gate."
    );

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required_file).exists(),
            "file-placement discipline requires `{required_file}` in coachmark source directory."
        );
    }
    for forbidden_file in ["render.rs", "spec.rs"] {
        assert!(
            !src_dir.join(forbidden_file).exists(),
            "coachmark should keep `{forbidden_file}` absent in current scope."
        );
    }

    assert!(
        mod_source.contains("mod logic;")
            && mod_source.contains("mod motion;")
            && mod_source.contains("pub mod styles;")
            && mod_source.contains("mod view;"),
        "mod.rs should keep canonical module boundary for file-placement discipline."
    );

    assert!(
        logic_source.contains("pub fn resolve_view_model(")
            && styles_source.contains("pub const CSS: &str =")
            && view_source.contains("#[component]")
            && motion_source.contains("pub struct CoachmarkMotion"),
        "logic/styles/view/motion should keep canonical responsibility anchors."
    );

    let script_needle = "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_file_placement_discipline_contract_is_explicit_for_interactive_component_scope";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "coachmark_file_placement_discipline_contract_is_explicit_for_interactive_component_scope",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark check2 should keep file-placement-discipline marker `{required}`."
        );
    }
}

#[test]
fn coachmark_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component() {
    let check2_source = load_source("coachmark_checklist");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let motion_source = load_source("motion");
    let protocol_source = include_str!("../src/protocol.rs");
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");

    assert!(
        check2_source.contains("Hyper-Structure Builder（`spec.rs`）"),
        "coachmark checklist should explicitly track hyper-structure builder gate."
    );

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "coachmark is not a complex spec-builder component; spec.rs should remain N/A."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "CoachmarkSpec",
        "Spec::new(",
        ".render()",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "coachmark should not expose hyper-structure builder artifact `{forbidden}` in current scope."
        );
    }

    let script_needle = "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "coachmark_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark checklist should keep hyper-structure builder marker `{required}`."
        );
    }
}

#[test]
fn coachmark_context_compression_manifest_and_rbi_are_present_and_consistent() {
    let check2_source = load_source("coachmark_checklist");
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");
    let component_manifest = include_str!("../src/Component.toml");
    let component_rbi = include_str!("../src/coachmark.rbi");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["Component.toml", "coachmark.rbi"] {
        assert!(
            src_dir.join(required_file).exists(),
            "coachmark context-compression file should exist: `{required_file}`."
        );
    }

    for required in [
        "schema_version = \"1\"",
        "name = \"Coachmark\"",
        "crate = \"ui-coachmark\"",
        "name = \"open\"",
        "name = \"on_open_change\"",
        "name = \"default_open\"",
        "name = \"variant\"",
        "name = \"placement\"",
        "name = \"motion\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "coachmark Component.toml should keep context-compression marker `{required}`."
        );
    }

    for required in [
        "pub type CoachmarkVariant = ui::contextual_help::ContextualHelpVariant;",
        "pub type CoachmarkAssetVariant = ui::asset::AssetVariant;",
        "pub struct CoachmarkMotion",
        "pub fn Coachmark(",
        "open: Option<leptos::prelude::Signal<bool>>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
        "default_open: Option<bool>",
        "dir: Option<A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi.contains(required),
            "coachmark.rbi should keep signature-projection marker `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_context_compression_manifest_and_rbi_are_present_and_consistent";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "coachmark_context_compression_manifest_and_rbi_are_present_and_consistent",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark checklist should keep context-compression marker `{required}`."
        );
    }
}

#[test]
fn coachmark_token_first_static_style_contract_is_aggregated_and_injected() {
    let styles = load_source("styles");
    let view = load_source("view");
    let popover_view = load_source("popover_view");
    let popover_logic = load_source("popover_logic");
    let ui_components_css = load_source("ui_components_css");
    let ui_components_root = load_source("ui_components_root");

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-border, var(--ui-fallback-border))",
    ] {
        assert!(
            styles.contains(required) || load_source("contextual_help_styles").contains(required),
            "token-first styles should source visual values from ui-theme var `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-coachmark\")]",
        "out.push_str(crate::coachmark::styles::CSS);",
        "crate::css::push_components_css(&mut out);",
        "if inject_components_css.get_value() {",
    ] {
        assert!(
            ui_components_css.contains(required) || ui_components_root.contains(required),
            "coachmark styles should be aggregated/injected via `{required}`."
        );
    }

    for forbidden in ["style!(", "stylist", "emotion", "tailwind", "@apply "] {
        assert!(
            !styles.contains(forbidden) && !view.contains(forbidden),
            "component layer should not default to utility-first/css-in-rust marker `{forbidden}`."
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"text-",
        "class=\"bg-",
        "class=\"px-",
        "class=\"py-",
    ] {
        assert!(
            !view.contains(forbidden),
            "coachmark component view should not expose utility-first class marker `{forbidden}`."
        );
    }

    assert!(
        !view.contains("style="),
        "coachmark view should not carry runtime business style logic."
    );
    assert!(
        popover_view.contains("style=panel_vars")
            && popover_logic.contains("--ui-popover-top:")
            && popover_logic.contains("--ui-popover-left:")
            && popover_logic.contains("--ui-popover-anchor-width:"),
        "runtime style transport should stay constrained to required CSS custom properties."
    );
}

#[test]
fn coachmark_visual_desire_reuses_theme_baseline_and_heroui_alignment_gate() {
    let theme_visual_baseline_page = load_source("theme_visual_baseline_page");
    let theme_visual_baseline_e2e = load_source("theme_visual_baseline_e2e");
    let heroui_strategy_doc = load_source("heroui_strategy_doc");

    for required in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues.",
        "Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            theme_visual_baseline_page.contains(required),
            "docs-app baseline page should keep visual-desire marker `{required}`."
        );
    }

    for required in [
        "E2E_VISUAL_BASELINE",
        "theme visual baseline screenshots",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            theme_visual_baseline_e2e.contains(required),
            "e2e baseline should keep screenshot regression marker `{required}`."
        );
    }

    for required in [
        "# HeroUI 参数设计风格对齐策略",
        "在 `ui` 中建立一套接近 HeroUI 的参数设计规范",
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
        "Research Notes（HeroUI 风格提炼）",
        "参数分层明显：视觉（`variant/size/color/radius`）",
    ] {
        assert!(
            heroui_strategy_doc.contains(required),
            "HeroUI alignment strategy should keep quality-direction marker `{required}`."
        );
    }
}

#[test]
fn coachmark_tree_shaking_contract_is_component_feature_gated_and_budgeted_in_ci() {
    let ui_components_manifest = load_source("ui_components_manifest");
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");
    let web_demo_manifest = load_source("web_demo_manifest");
    let tree_shaking_spec = load_source("tree_shaking_spec");
    let tree_shaking_script = load_source("tree_shaking_script");
    let tree_shaking_budget = load_source("tree_shaking_budget");
    let platform_script = load_source("platform_script");
    let ci_workflow = load_source("ci_workflow");

    for required in [
        "[features]",
        "all-components = [",
        "\"component-coachmark\",",
        "component-coachmark = [",
        "\"component-asset\"",
        "\"component-button\"",
        "\"component-contextual_help\"",
    ] {
        assert!(
            ui_components_manifest.contains(required),
            "ui manifest should expose coachmark feature-gate marker `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-coachmark\")]",
        "#[path = \"../../../components/coachmark/src/mod.rs\"]",
        "pub mod coachmark;",
        "#[cfg(feature = \"all-components\")]",
        "pub use all_components::*;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib should keep conditional export marker `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-coachmark\")]",
        "out.push_str(crate::coachmark::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css should keep conditional aggregation marker `{required}`."
        );
    }

    for required in [
        "ui = { path = \"../../crates/ui\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
        "name = \"web-demo\"",
    ] {
        assert!(
            web_demo_manifest.contains(required),
            "web-demo should consume scoped bundle marker `{required}`."
        );
    }
    assert!(
        !web_demo_manifest.contains("all-components"),
        "web-demo should not pull all-components by default."
    );

    for required in [
        "Tree Shaking / 组件级裁剪规格",
        "禁止引入“全组件中央注册表”",
        "component-button,component-input,inject-css",
    ] {
        assert!(
            tree_shaking_spec.contains(required),
            "tree-shaking spec should keep contract marker `{required}`."
        );
    }

    for required in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script.contains(required),
            "tree-shaking script should keep CI gate marker `{required}`."
        );
    }

    for required in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            tree_shaking_budget.contains(required),
            "tree-shaking budget should define marker `{required}`."
        );
    }

    for required in [
        "component-coachmark,inject-css",
        "components/coachmark/src/mod.rs",
        "components/coachmark/src/view.rs",
    ] {
        assert!(
            platform_script.contains(required),
            "platform compile/source guard should include coachmark marker `{required}`."
        );
    }

    assert!(
        ci_workflow.contains("./scripts/check-ui-tree-shaking.sh"),
        "ci should run tree-shaking gate script."
    );
}

#[test]
fn coachmark_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let tree_shaking_script = load_source("tree_shaking_script");

    for required in [
        "COACHMARK_MIN_FEATURES=\"component-coachmark,inject-css\"",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_tree_shaking_contract_is_component_feature_gated_and_budgeted_in_ci",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "COACHMARK_TREE_OUTPUT=\"$(cargo tree -e features -i ui -p ui --no-default-features --features \"$COACHMARK_MIN_FEATURES\")\"",
        "if ! grep -q 'feature \"component-coachmark\" (command-line)' <<<\"$COACHMARK_TREE_OUTPUT\";",
        "if ! grep -q 'feature \"inject-css\" (command-line)' <<<\"$COACHMARK_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$COACHMARK_TREE_OUTPUT\";",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$COACHMARK_MIN_FEATURES\"",
    ] {
        assert!(
            tree_shaking_script.contains(required),
            "tree-shaking script should keep coachmark gate marker `{required}`."
        );
    }
}

#[test]
fn coachmark_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("coachmark_checklist");

    for required in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "coachmark_tree_shaking_contract_is_component_feature_gated_and_budgeted_in_ci",
        "coachmark_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "scripts/check-ui-tree-shaking.sh",
        "component-coachmark,inject-css",
        "cargo tree -e features -i ui -p ui --no-default-features --features component-coachmark,inject-css",
        "cargo tree -e features -i ui -p web-demo",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark check2 tree-shaking section should reference `{required}`."
        );
    }
}

#[test]
fn coachmark_type_system_and_semantic_markers_keep_state_machine_machine_readable() {
    let primitive = load_source("primitive");
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "pub enum CoachmarkCtaMode {",
        "pub enum CoachmarkAssetSource {",
        "pub struct CoachmarkStateInput {",
        "pub struct CoachmarkState {",
        "pub cta_mode: CoachmarkCtaMode,",
        "pub asset_source: CoachmarkAssetSource,",
        "pub fn resolve_state(input: CoachmarkStateInput) -> CoachmarkState {",
    ] {
        assert!(
            primitive.contains(required),
            "state primitive should keep typed state-axis marker `{required}`."
        );
    }

    for required in [
        "variant_attr: input.variant.as_attr(),",
        "placement_attr: input.placement.as_str(),",
        "cta_mode,",
        "asset_source,",
        "let state = resolve_state(CoachmarkStateInput {",
        "const _: Option<CoachmarkState> = None;",
    ] {
        assert!(
            logic.contains(required),
            "logic should keep normalized typed-input marker `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] variant: CoachmarkVariant,",
        "#[prop(optional)] placement: PopoverPlacement,",
        "#[prop(optional)] asset_variant: Option<CoachmarkAssetVariant>,",
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-placement=state.placement_attr",
        "data-open-mode=state.open_mode_attr",
        "data-asset-source=state.asset_source_attr",
    ] {
        assert!(
            view.contains(required),
            "view should expose typed + machine-readable marker `{required}`."
        );
    }

    for forbidden in [
        "variant: Option<String>",
        "placement: Option<String>",
        "asset_variant: Option<String>",
        "status: Option<String>",
        "mode: Option<String>",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "component should not introduce free-form state axis marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_overlay_focus_restoration_uses_global_focus_stack_not_component_noderef_cache() {
    let coachmark_view = load_source("view");
    let contextual_help_view = load_source("contextual_help_view");
    let popover_view = load_source("popover_view");
    let overlay_view = load_source("overlay_view");
    let focus_trap_source = load_source("focus_trap_source");

    for required in [
        "use_focus_trap(",
        "FocusTrapOptions::enabled(panel_ref)",
        ".with_scope_id(\"overlay\")",
        ".with_restore_policy(RestorePolicy::FallbackTo(",
        ".with_fallback_selector(",
    ] {
        assert!(
            overlay_view.contains(required),
            "overlay focus path should use headless focus-trap contract marker `{required}`."
        );
    }

    for required in [
        "let focus_trap = use_focus_trap(",
        "FocusTrapOptions::enabled(panel_ref)",
        "on:keydown=on_key_down",
    ] {
        assert!(
            popover_view.contains(required),
            "popover should delegate focus trapping via headless contract marker `{required}`."
        );
    }

    for required in [
        "thread_local! {",
        "static FOCUS_MANAGER_STACK:",
        "focus_manager_push_trap(",
        "focus_manager_pop_trap(",
        "focus_manager_peek_trap()",
        "restore_focus_chain(",
        "RestorePolicy::FallbackTo(",
        "if let Some(body) = document.body() {",
    ] {
        assert!(
            focus_trap_source.contains(required),
            "ui-headless focus manager should keep global stack restore marker `{required}`."
        );
    }

    for forbidden in [
        "restore_target_ref",
        "previous_focus_ref",
        "focus_restore_ref",
        "cached_focus_ref",
        "last_focus_ref",
        "document.body().focus(",
    ] {
        assert!(
            !coachmark_view.contains(forbidden)
                && !contextual_help_view.contains(forbidden)
                && !popover_view.contains(forbidden)
                && !overlay_view.contains(forbidden),
            "component layer should not cache focus target via NodeRef-like restore marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_escape_hatches_are_not_applicable_and_no_foreign_zone_surface_leaks() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let contextual_help_view = load_source("contextual_help_view");
    let popover_view = load_source("popover_view");

    for required in [
        "pub fn Coachmark(",
        "pub fn ContextualHelp(",
        "pub fn Popover(",
        "on_open_change",
        "on_primary",
        "on_secondary",
    ] {
        assert!(
            view.contains(required)
                || contextual_help_view.contains(required)
                || popover_view.contains(required),
            "coachmark composition should keep callback-driven contract marker `{required}`."
        );
    }

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "mapbox",
        "Leaflet",
        "leaflet",
        "YieldControl",
        "CleanupForeign",
        "ForeignZone",
        "foreign_zone",
        "foreign_instance",
        "chart_instance",
        "map_instance",
        "JsValue",
        "web_sys::HtmlCanvasElement",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !contextual_help_view.contains(forbidden)
                && !popover_view.contains(forbidden),
            "coachmark should not expose third-party imperative foreign-zone marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_ssr_hydration_ids_use_deterministic_id_provider_seed() {
    let ui_components_root = load_source("ui_components_root");
    let contextual_help_view = load_source("contextual_help_view");
    let headless_id_provider = load_source("headless_id_provider");

    for required in [
        "#[prop(optional, default = 1)] id_seed: u64",
        "provide_ui_id_provider(id_seed);",
    ] {
        assert!(
            ui_components_root.contains(required),
            "UiRoot should provide deterministic id seed marker `{required}`."
        );
    }

    for required in [
        "use_ui_id_provider",
        "provider.next_prefixed_id(\"ui-contextual-help\")",
        "logic::resolve_id(id, generated_id)",
    ] {
        assert!(
            contextual_help_view.contains(required),
            "ContextualHelp should consume deterministic id provider marker `{required}`."
        );
    }

    for required in [
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider>",
    ] {
        assert!(
            headless_id_provider.contains(required),
            "ui-headless id provider contract should include `{required}`."
        );
    }

    for forbidden in [
        "fn next_id() -> u64",
        "thread_local!",
        "SystemTime::now",
        "now()",
        "Uuid::new_v4",
        "rand::",
    ] {
        assert!(
            !contextual_help_view.contains(forbidden),
            "ContextualHelp id generation should avoid non-deterministic marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_platform_checks_cover_web_ssr_wasm_and_non_wasm_guards() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let motion = load_source("motion");
    let styles = load_source("styles");
    let view = load_source("view");
    let contextual_help_view = load_source("contextual_help_view");
    let popover_view = load_source("popover_view");
    let overlay_view = load_source("overlay_view");
    let platform_script = load_source("platform_script");

    for required in [
        "echo \"[platform] compile-only: default native path\"",
        "cargo check -p ui",
        "echo \"[platform] compile-only: ssr native path\"",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "echo \"[platform] compile-only: coachmark native path\"",
        "cargo check -p ui --no-default-features --features component-coachmark,inject-css",
        "echo \"[platform] compile-only: coachmark wasm path\"",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-coachmark,inject-css",
        "echo \"[platform] source guard: non-wasm coachmark files must not reference web_sys\"",
        "components/coachmark/src/mod.rs",
        "components/coachmark/src/logic.rs",
        "components/coachmark/src/motion.rs",
        "components/coachmark/src/styles.rs",
        "components/coachmark/src/view.rs",
    ] {
        assert!(
            platform_script.contains(required),
            "Platform script should include coachmark cross-platform guard marker `{required}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            popover_view.contains(required) && overlay_view.contains(required),
            "Overlay chain should keep explicit wasm/non-wasm cfg marker `{required}`."
        );
    }

    for forbidden in ["web_sys::", "wasm_bindgen", "window.", "document."] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !motion.contains(forbidden)
                && !styles.contains(forbidden)
                && !view.contains(forbidden)
                && !contextual_help_view.contains(forbidden),
            "Coachmark component layer should avoid non-wasm browser object marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_headless_web_ssr_feature_mutex_is_guarded_by_compile_error() {
    let headless_lib = load_source("headless_lib");
    let platform_script = load_source("platform_script");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            headless_lib.contains(required),
            "ui-headless lib should keep web/ssr mutex compile_error marker `{required}`."
        );
    }

    for required in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "if cargo check -p ui-headless --no-default-features --features web,ssr >\"$MUTEX_LOG\" 2>&1; then",
        "if ! rg -n \"mutually exclusive\" \"$MUTEX_LOG\" >/dev/null; then",
    ] {
        assert!(
            platform_script.contains(required),
            "platform script should enforce ui-headless web/ssr mutex marker `{required}`."
        );
    }
}

#[test]
fn coachmark_ui_motion_non_wasm_stub_keeps_ssr_tooling_compile_safe() {
    let ui_motion_lib = load_source("ui_motion_lib");
    let ui_motion_non_wasm_stub_test = load_source("ui_motion_non_wasm_stub_test");
    let popover_motion = load_source("popover_motion");
    let coachmark_motion = load_source("motion");
    let platform_script = load_source("platform_script");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should keep non-wasm no-op backend marker `{required}`."
        );
    }

    for required in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "assert!(web::prefers_reduced_motion());",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
        "web::animate(&(), &keyframes, MotionOptions::default());",
    ] {
        assert!(
            ui_motion_non_wasm_stub_test.contains(required),
            "ui-motion non-wasm stub test should keep marker `{required}`."
        );
    }

    for required in [
        "echo \"[platform] compile-only: ui-motion native path\"",
        "cargo check -p ui-motion",
        "echo \"[platform] compile-only: ui-motion wasm path\"",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "echo \"[platform] ui-motion non-wasm stub tests\"",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script.contains(required),
            "platform script should gate ui-motion non-wasm/wasm paths via `{required}`."
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "Effect::new(move |_| {",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            popover_motion.contains(required),
            "popover motion should keep predictable non-wasm degradation marker `{required}`."
        );
    }

    for required in [
        "pub struct CoachmarkMotion {",
        "pub popover: PopoverMotion,",
        "pub fn sanitize_motion(motion: CoachmarkMotion) -> CoachmarkMotion {",
        "pub fn resolve_motion(motion: CoachmarkMotion) -> ContextualHelpMotion {",
        "ContextualHelpMotion {",
        "popover: motion.popover,",
    ] {
        assert!(
            coachmark_motion.contains(required),
            "coachmark motion should keep contract-only mapping marker `{required}`."
        );
    }

    for forbidden in ["panic!(", ".unwrap()", ".expect("] {
        assert!(
            !popover_motion.contains(forbidden) && !coachmark_motion.contains(forbidden),
            "motion non-wasm path should avoid panic-prone marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let ui_motion_spring = load_source("ui_motion_spring");
    let popover_motion = load_source("popover_motion");
    let popover_view = load_source("popover_view");
    let contextual_help_view = load_source("contextual_help_view");
    let platform_script = load_source("platform_script");

    for required in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
        "on_rest();",
    ] {
        assert!(
            ui_motion_spring.contains(required),
            "ui-motion spring should keep reduced-motion immediate-settle marker `{required}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "Always initialize in the closed state so mounting while open animates in.",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            popover_motion.contains(required),
            "popover motion should keep wasm/non-wasm + hydration-safe marker `{required}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if logic::should_close_on_escape(",
    ] {
        assert!(
            popover_view.contains(required),
            "popover view should keep explicit wasm/non-wasm interaction branch marker `{required}`."
        );
    }

    for required in [
        "role=\"dialog\"",
        "aria-modal=\"false\"",
        "aria-labelledby=panel_aria_labelledby.get_value()",
        "aria-describedby=panel_aria_describedby.get_value()",
    ] {
        assert!(
            contextual_help_view.contains(required),
            "contextual-help semantic contract should remain stable across platform branches marker `{required}`."
        );
    }

    for required in [
        "cargo check -p ui --no-default-features --features component-coachmark,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-coachmark,inject-css",
        "echo \"[platform] coachmark reduced-motion/ssr/wasm contract\"",
        "cargo test -p ui --lib coachmark_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
    ] {
        assert!(
            platform_script.contains(required),
            "platform script should include coachmark reduced-motion/ssr/wasm gate marker `{required}`."
        );
    }
}

#[test]
fn coachmark_performance_governance_budget_is_defined_traceable_and_blocking() {
    let shell_source = load_source("docs_components_shell");
    let perf_probe_source = load_source("docs_perf_probe");
    let e2e_source = load_source("docs_components_coverage_e2e");
    let debug_overlay_source = load_source("docs_debug_overlay");
    let checklist_source = load_source("coachmark_checklist");
    let todo_source = load_source("todo_plan");
    let script_source = load_source("perf_script");
    let coachmark_view = load_source("view");
    let contextual_help_view = load_source("contextual_help_view");

    for required in [
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "\"coachmark\" => UiPerfBudget {",
        "max_mount_ms: 36.0,",
        "max_update_ms: Some(12.0),",
        "max_heap_kb: Some(640.0),",
    ] {
        assert!(
            shell_source.contains(required),
            "docs component shell should keep performance budget marker `{required}`."
        );
    }

    for required in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(required),
            "UiPerfProbe should expose perf observability marker `{required}`."
        );
    }

    for required in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            e2e_source.contains(required),
            "docs coverage e2e should keep perf regression guard marker `{required}`."
        );
    }

    for required in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(required),
            "debug overlay should keep trace-based attribution marker `{required}`."
        );
    }

    for required in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            checklist_source.contains(required),
            "coachmark checklist should retain performance governance marker `{required}`."
        );
    }

    for required in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(required),
            "performance follow-up plan should include marker `{required}`."
        );
    }

    for required in [
        "cargo test -p ui --lib coachmark_performance_governance_budget_is_defined_traceable_and_blocking",
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(required),
            "performance gate script should include blocking marker `{required}`."
        );
    }

    for required in [
        "let motion = motion::resolve_motion(motion);",
        "data-state=state.state_attr",
        "data-open-mode=state.open_mode_attr",
        "motion=motion",
        "data-motion-source=state.motion_source_attr",
    ] {
        assert!(
            coachmark_view.contains(required) || contextual_help_view.contains(required),
            "coachmark composition should keep state/style/motion attribution marker `{required}`."
        );
    }
}

#[test]
fn coachmark_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let coachmark_view = load_source("view");
    let contextual_help_view = load_source("contextual_help_view");
    let popover_view = load_source("popover_view");
    let overlay_view = load_source("overlay_view");
    let focus_trap_source = load_source("focus_trap_source");
    let checklist_source = load_source("coachmark_checklist");
    let todo_source = load_source("todo_plan");
    let perf_script = load_source("perf_script");
    let local_semantics = include_str!("../test/semantics.rs");
    let legacy_semantics = load_source("legacy_semantics");

    for required in [
        "role=\"dialog\"",
        "aria_expanded=open",
        "aria_haspopup=\"dialog\"",
        "aria-labelledby=panel_aria_labelledby.get_value()",
        "aria-describedby=panel_aria_describedby.get_value()",
        "data-state=state.state_attr",
        "data-open-mode=state.open_mode_attr",
        "data-open-source=state.open_source_attr",
        "data-default-open-source=state.default_open_source_attr",
        "data-open-change-source=state.open_change_source_attr",
    ] {
        assert!(
            coachmark_view.contains(required) || contextual_help_view.contains(required),
            "semantics/perf regression should keep aria+data marker `{required}`."
        );
    }

    for required in [
        "on:keydown=on_key_down",
        "if logic::should_close_on_escape(",
        "use_focus_trap(",
        "FocusTrapOptions::enabled(panel_ref)",
        "focus_manager_push_trap(",
        "focus_manager_pop_trap(",
    ] {
        assert!(
            popover_view.contains(required)
                || overlay_view.contains(required)
                || focus_trap_source.contains(required),
            "semantics/perf regression should keep focus-path marker `{required}`."
        );
    }

    let snapshot_macro = ["assert", "_snapshot"].concat();
    let insta_prefix = ["insta", "::"].concat();
    let to_match_macro = ["to_match", "_snapshot"].concat();
    let snapshot_bang = ["snapshot", "!"].concat();

    for forbidden in [
        &snapshot_macro,
        &insta_prefix,
        &to_match_macro,
        &snapshot_bang,
    ] {
        assert!(
            !local_semantics.contains(forbidden.as_str())
                && !legacy_semantics.contains(forbidden.as_str()),
            "coachmark regression should not degrade to snapshot-only assertion `{forbidden}`."
        );
    }

    for required in [
        "语义测试与性能回归",
        "aria-*`、`data-*` 与焦点流转",
        "render_count",
        "等价证据",
    ] {
        assert!(
            checklist_source.contains(required),
            "checklist should keep semantics/perf contract marker `{required}`."
        );
    }

    for required in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(required),
            "todo plan should keep render_count follow-up marker `{required}`."
        );
    }

    for required in [
        "cargo test -p ui --lib coachmark_performance_governance_budget_is_defined_traceable_and_blocking",
        "cargo test -p ui --lib coachmark_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            perf_script.contains(required),
            "performance gate script should include coachmark regression marker `{required}`."
        );
    }
}

#[test]
fn coachmark_check2_marks_semantics_and_performance_regression_complete() {
    let check2_source = load_source("coachmark_checklist");

    for required in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "coachmark_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "coachmark_performance_governance_budget_is_defined_traceable_and_blocking",
        "scripts/check-ui-performance.sh",
        "render_count",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark check2 semantics/perf section should reference `{required}`."
        );
    }
}

#[test]
fn coachmark_view_macro_complexity_is_split_into_semantic_subviews() {
    let view = load_source("view");

    for required in [
        "let footer_view = StoredValue::new(ViewFn::from(move || {",
        "let content_view = StoredValue::new(ViewFn::from(move || {",
        "let render_contextual_help = |open: Option<Signal<bool>>, has_footer: bool| -> AnyView {",
        "match (open, dir) {",
        "render_contextual_help(open, has_footer)",
    ] {
        assert!(
            view.contains(required),
            "coachmark view should keep macro split marker `{required}`."
        );
    }

    let contextual_help_mounts = view.matches("<ContextualHelp").count();
    assert!(
        contextual_help_mounts <= 8,
        "coachmark view should avoid repeated giant view! mounts; got {contextual_help_mounts}."
    );

    for forbidden in [
        "} else if has_footer {",
        "if let Some(open) = open {\n        if has_footer {",
    ] {
        assert!(
            !view.contains(forbidden),
            "coachmark view should avoid duplicated nested macro branch marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_view_prefers_functional_fragment_splitting() {
    let view = load_source("view");

    for required in [
        "fn render_footer_fragment(",
        "fn render_content_fragment(",
        "render_footer_fragment(",
        "render_content_fragment(",
    ] {
        assert!(
            view.contains(required),
            "coachmark view should keep functional fragment split marker `{required}`."
        );
    }

    for forbidden in [
        "#[component]\nfn CoachmarkFooter",
        "#[component]\nfn CoachmarkContent",
    ] {
        assert!(
            !view.contains(forbidden),
            "coachmark view should avoid local component noise marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_static_fragments_are_constantized_and_centralized() {
    let view = load_source("view");
    let logic = load_source("logic");

    for required in [
        "const COACHMARK_BUTTON_SECONDARY_CLASS: &str =",
        "const COACHMARK_BUTTON_PRIMARY_CLASS: &str =",
        "const COACHMARK_ASSET_CLASS: &str =",
        "const COACHMARK_STREAM_MODE_SNAPSHOT: &str =",
        "pub const COACHMARK_AGENT_SCHEMA: &str =",
        "class_name=COACHMARK_BUTTON_SECONDARY_CLASS.to_string()",
        "class_name=COACHMARK_BUTTON_PRIMARY_CLASS.to_string()",
        "class_name=COACHMARK_ASSET_CLASS.to_string()",
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-stream-mode=COACHMARK_STREAM_MODE_SNAPSHOT",
    ] {
        assert!(
            view.contains(required) || logic.contains(required),
            "coachmark view should keep static-fragment constantization marker `{required}`."
        );
    }

    for required in ["role=\"status\"", "aria-live=\"polite\""] {
        assert!(
            view.contains(required),
            "static-fragment refactor should preserve a11y semantic marker `{required}`."
        );
    }
}

#[test]
fn coachmark_agent_contract_is_schema_typed_and_machine_readable_locally() {
    let check2_source = load_source("coachmark_checklist");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let component_toml = include_str!("../src/Component.toml");

    for typed_source in [
        "pub const COACHMARK_AGENT_SCHEMA: &str = \"ui.coachmark.agent-contract.v1\";",
        "pub enum CoachmarkAgentSchemaVersion",
        "pub enum CoachmarkAgentIntent",
        "pub enum CoachmarkAgentAction",
        "pub enum CoachmarkAgentState",
        "pub enum CoachmarkAgentSource",
        "pub enum CoachmarkAgentOutputStatus",
        "pub struct CoachmarkAgentContract",
        "pub struct CoachmarkAgentContractInput",
        "pub fn resolve_agent_contract(input: CoachmarkAgentContractInput) -> CoachmarkAgentContract",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "Coachmark Agent Contract should stay type-derived via `{typed_source}`."
        );
    }

    for marker in [
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-schema-version=agent_contract.schema_version.as_str()",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-action=agent_contract.action.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-source=agent_contract.source.as_str()",
        "data-ui-state-source=agent_contract.state_source",
        "data-ui-action-source=agent_contract.action_source",
        "data-ui-render-path=agent_contract.render_path",
        "data-ui-output-status=agent_contract.output_status.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "Coachmark view should mount Agent Contract marker `{marker}`."
        );
    }

    for required in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_contract_schema_markers\"",
        "[[agent_contract]]",
        "schema = \"ui.coachmark.agent-contract.v1\"",
        "intent = \"guided-tour\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "attr = \"data-ui-state-source\"",
        "attr = \"data-ui-action-source\"",
    ] {
        assert!(
            component_toml.contains(required),
            "Coachmark manifest should keep Agent Contract marker `{required}`."
        );
    }

    for forbidden in [
        "data-ui-schema=\"",
        "format!(\"data-ui-schema",
        "format!(\"data-ui-intent",
        "format!(\"data-ui-state",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Coachmark Agent Contract should avoid free-form schema token `{forbidden}`."
        );
    }

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "coachmark_agent_contract_is_schema_typed_and_machine_readable_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark checklist should keep Agent Contract evidence `{required}`."
        );
    }
}

#[test]
fn coachmark_agent_contract_render_path_is_whitelist_safe_and_script_injection_free_locally() {
    let check2_source = load_source("coachmark_checklist");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let component_toml = include_str!("../src/Component.toml");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "allowed = [\"children()\", \"render_content_fragment()\", \"render_footer_fragment()\", \"actions.run()\"]",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\", \"eval(\"]",
    ] {
        assert!(
            component_toml.contains(required),
            "Coachmark manifest should keep whitelist-safe render path marker `{required}`."
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
            "Coachmark Agent Contract render path should forbid `{forbidden}`."
        );
    }

    for script_needle in [
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_agent_contract_is_schema_typed_and_machine_readable_locally",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_agent_contract_render_path_is_whitelist_safe_and_script_injection_free_locally",
    ] {
        assert!(
            script_source.contains(script_needle),
            "contract-hygiene gate script should include `{script_needle}`."
        );
    }

    for required in [
        "coachmark_agent_contract_render_path_is_whitelist_safe_and_script_injection_free_locally",
        "白名单能力边界",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark checklist should keep Agent Contract whitelist evidence `{required}`."
        );
    }
}

#[test]
fn coachmark_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("coachmark_checklist");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let mod_source = load_source("mod");
    let motion_source = load_source("motion");
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "N/A：`Coachmark` 不是 LLM 正文渲染组件",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark check2 should keep streaming-definition marker `{required}`."
        );
    }

    for required in [
        "const COACHMARK_UI_STREAM_SUPPORT: &str = \"optional\";",
        "const COACHMARK_STREAM_MODE_SNAPSHOT: &str = \"snapshot\";",
        "data-ui-stream-support=COACHMARK_UI_STREAM_SUPPORT",
        "data-ui-stream-fallback=COACHMARK_STREAM_MODE_SNAPSHOT",
        "data-ui-stream-mode=COACHMARK_STREAM_MODE_SNAPSHOT",
        "data-stream-mode=COACHMARK_STREAM_MODE_SNAPSHOT",
        "data-stream-fallback=COACHMARK_STREAM_MODE_SNAPSHOT",
    ] {
        assert!(
            view_source.contains(required),
            "coachmark runtime should keep stream/snapshot marker `{required}`."
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "COACHMARK_STREAM_MODE_STREAMING",
        "data-ui-stream-mode=\"streaming\"",
        "data-stream-mode=\"streaming\"",
        "project_streaming_",
        "use_ai_space_state",
    ] {
        assert!(
            !combined.contains(forbidden),
            "coachmark runtime path should not embed unsupported streaming protocol marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`."
    );
}

#[test]
fn coachmark_streaming_script_covers_two_mode_definition_contract() {
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");

    let needle = "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`."
    );
}

#[test]
fn coachmark_check2_marks_streaming_two_mode_definition_complete() {
    let source = load_source("coachmark_checklist");

    assert!(
        source.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
        "coachmark check2 should mark streaming two-mode definition gate complete."
    );

    for needle in [
        "coachmark_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "coachmark_streaming_script_covers_two_mode_definition_contract",
        "scripts/check-ui-streaming.sh",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
    ] {
        assert!(
            source.contains(needle),
            "coachmark check2 streaming section should reference `{needle}`."
        );
    }
}

#[test]
fn coachmark_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("coachmark_checklist");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "N/A：`Coachmark` 不直接渲染 LLM 正文",
        "coachmark_check2_documents_snapshot_as_default_baseline_capability",
        "scripts/check-ui-streaming.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "coachmark check2 should keep snapshot-baseline marker `{needle}`."
        );
    }
}

#[test]
fn coachmark_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let check2_source = load_source("coachmark_checklist");
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}");

    for needle in [
        "let view_model = logic::resolve_view_model(logic::CoachmarkViewModelInput {",
        "title,",
        "current_step,",
        "total_steps,",
        "primary_cta,",
        "secondary_cta,",
        "shortcut_key,",
        "modifier_keys,",
        "asset_variant,",
        "asset_label,",
        "asset_src,",
        "asset_alt,",
        "lang,",
        "data-state=state.state_attr",
        "data-open-mode=state.open_mode_attr",
        "data-footer=state.footer_attr",
        "data-asset=state.asset_attr",
        "data-cta=state.cta_attr",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "data-shortcut=state.shortcut_attr",
        "data-actions=state.actions_attr",
        "data-steps=state.steps_attr",
        "data-asset-source=state.asset_source_attr",
        "data-ui-stream-mode=COACHMARK_STREAM_MODE_SNAPSHOT",
        "data-stream-mode=COACHMARK_STREAM_MODE_SNAPSHOT",
    ] {
        assert!(
            view_source.contains(needle),
            "coachmark snapshot baseline should keep stable complete-result render marker `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_view_model(input: CoachmarkViewModelInput) -> CoachmarkViewModel",
        "pub fn resolve_default_open(default_open: Option<bool>) -> bool",
        "pub fn resolve_on_open_change(on_open_change: Option<Callback<bool>>) -> Callback<bool>",
    ] {
        assert!(
            logic_source.contains(needle),
            "coachmark logic should keep snapshot-baseline normalization marker `{needle}`."
        );
    }

    for forbidden in [
        "COACHMARK_STREAM_MODE_STREAMING",
        "streaming_chunk",
        "token_delta",
        "partial token",
        "data-ui-stream-mode=\"streaming\"",
        "data-stream-mode=\"streaming\"",
    ] {
        assert!(
            !combined.contains(forbidden),
            "coachmark snapshot baseline should avoid incremental streaming marker `{forbidden}`."
        );
    }

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "coachmark_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            check2_source.contains(needle),
            "coachmark check2 snapshot section should reference `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`."
    );
}

#[test]
fn coachmark_streaming_script_covers_snapshot_baseline_contract() {
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn coachmark_check2_marks_snapshot_baseline_capability_complete() {
    let source = load_source("coachmark_checklist");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "coachmark_check2_documents_snapshot_as_default_baseline_capability",
        "coachmark_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "scripts/check-ui-streaming.sh",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            source.contains(needle),
            "coachmark check2 snapshot section should reference `{needle}`."
        );
    }
}

#[test]
fn coachmark_check2_documents_streaming_required_optional_classification_rules() {
    let check2_source = load_source("coachmark_checklist");
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "`Coachmark` 归类为 `Streaming Optional`",
        "fallback=snapshot",
    ] {
        assert!(
            check2_source.contains(needle),
            "coachmark check2 should keep streaming required/optional rule `{needle}`."
        );
    }

    for script_needle in [
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming check script should enforce `{script_needle}`."
        );
    }
}

#[test]
fn coachmark_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("view");
    let logic_source = load_source("logic");

    for needle in [
        "role=\"status\"",
        "aria-live=\"polite\"",
        "data-state=state.state_attr",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-source=agent_contract.source.as_str()",
        "data-ui-output-status=agent_contract.output_status.as_str()",
        "data-output-status=agent_contract.output_status.as_str()",
        "data-ui-stream-fallback=COACHMARK_STREAM_MODE_SNAPSHOT",
        "data-ui-stream-mode=COACHMARK_STREAM_MODE_SNAPSHOT",
        "data-stream-fallback=COACHMARK_STREAM_MODE_SNAPSHOT",
        "data-stream-mode=COACHMARK_STREAM_MODE_SNAPSHOT",
    ] {
        assert!(
            view_source.contains(needle),
            "coachmark optional-streaming scope should keep semantic continuity marker `{needle}`."
        );
    }

    for needle in [
        "pub enum CoachmarkAgentOutputStatus",
        "CoachmarkAgentOutputStatus::Verified",
        "output_status: CoachmarkAgentOutputStatus",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "coachmark optional-streaming scope should expose explicit output-status marker `{needle}`."
        );
    }
}

#[test]
fn coachmark_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let mod_source = load_source("mod");
    let motion_source = load_source("motion");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "network_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "coachmark should keep validation/retry/resilience policy outside component layer; found `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_streaming_script_covers_required_optional_classification_contract() {
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn coachmark_check2_marks_streaming_required_optional_classification_complete() {
    let source = load_source("coachmark_checklist");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "coachmark_check2_documents_streaming_required_optional_classification_rules",
        "coachmark_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "coachmark_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
        "scripts/check-ui-streaming.sh",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            source.contains(needle),
            "coachmark check2 should keep required/optional classification evidence marker `{needle}`."
        );
    }
}

#[test]
fn coachmark_inner_html_usage_is_forbidden_without_trusted_constant_contract() {
    let view = load_source("view");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let contextual_help_view = load_source("contextual_help_view");
    let contextual_help_logic = load_source("contextual_help_logic");
    let popover_view = load_source("popover_view");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "insert_adjacent_html(",
        "create_contextual_fragment(",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !styles.contains(forbidden)
                && !contextual_help_view.contains(forbidden)
                && !contextual_help_logic.contains(forbidden)
                && !popover_view.contains(forbidden),
            "coachmark composition should not expose inner-html injection marker `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let check2_source = load_source("coachmark_checklist");
    let docs_app_source = include_str!("../../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("docs_debug_overlay");
    let trace_source = include_str!("../../../crates/ui-headless/src/trace.rs");
    let controllable_open_source = load_source("controllable_open");
    let contextual_help_view = load_source("contextual_help_view");
    let coachmark_view = load_source("view");
    let coachmark_logic = load_source("logic");
    let coachmark_readme = include_str!("../src/README.md");
    let coachmark_manifest = include_str!("../Cargo.toml");
    let ui_components_manifest = load_source("ui_components_manifest");
    let ui_components_lib = load_source("ui_components_lib");
    let wasm_debug_script = include_str!("../../../scripts/check-ui-wasm-debug.sh");

    for required in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(required),
            "docs-app should keep dev-only wasm debug visual entry marker `{required}`."
        );
    }

    for required in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "events.push(event);",
        "if events.len() > MAX_EVENTS",
    ] {
        assert!(
            trace_source.contains(required),
            "ui-headless trace should keep timestamped event stream marker `{required}`."
        );
    }

    for required in [
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
        "let events = events.get();",
        ".into_iter()",
        ".rev()",
        ".take(40)",
        "data-component=component",
        "data-kind=kind_attr",
    ] {
        assert!(
            debug_overlay_source.contains(required),
            "debug overlay should keep chronological replay/evidence marker `{required}`."
        );
    }

    for required in [
        "pub fn use_controllable_open_state_traced(",
        "trace.emit(component, UiTraceEventKind::OpenChange { open: next });",
        "let previous_open = RwSignal::new(open.get_untracked());",
        "ContextualHelpOpenInteractionIntent::TriggerPress",
        "ContextualHelpOpenInteractionIntent::DismissPress",
        "open_interaction_source.set(sync.next_source);",
        "open_interaction_source_for_trigger.set(intent.next_source);",
        "open_interaction_source_for_close.set(intent.next_source);",
    ] {
        assert!(
            controllable_open_source.contains(required) || contextual_help_view.contains(required),
            "coachmark open path should keep source/transition traceability marker `{required}`."
        );
    }

    for required in ["[features]", "default = []"] {
        assert!(
            coachmark_manifest.contains(required),
            "coachmark crate should keep minimal feature surface marker `{required}`."
        );
    }

    for forbidden in [
        "coachmark-wasm-debug",
        "coachmark_wasm_debug",
        "wasm-debug =",
    ] {
        assert!(
            !coachmark_manifest.contains(forbidden),
            "coachmark crate should not expose component-local wasm debug feature `{forbidden}`."
        );
    }

    for required in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_manifest.contains(required),
            "ui should keep shared wasm debug feature boundary marker `{required}`."
        );
    }

    for forbidden in [
        "coachmark-wasm-debug =",
        "coachmark_wasm_debug =",
        "component-coachmark\", \"dep:tracing",
    ] {
        assert!(
            !ui_components_manifest.contains(forbidden),
            "ui should not leak coachmark-local wasm debug feature `{forbidden}`."
        );
    }

    for required in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "shared wasm debug infra should stay isolated in ui root marker `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] debug",
        "UiDebugOverlay",
        "provide_ui_trace(",
        "use_ui_trace(",
        "trace.emit(",
        "request_replay",
        "replay",
    ] {
        assert!(
            !coachmark_view.contains(forbidden)
                && !coachmark_logic.contains(forbidden)
                && !coachmark_readme.contains(forbidden),
            "coachmark runtime/public contract should not leak wasm-debug internals `{forbidden}`."
        );
    }

    let script_marker = "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated";
    assert!(
        wasm_debug_script.contains(script_marker),
        "wasm-debug gate script should include `{script_marker}`."
    );

    for required in [
        "- [x] WASM 调试要求",
        "coachmark_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark checklist should keep wasm-debug governance marker `{required}`."
        );
    }
}

#[test]
fn coachmark_dx_playground_supports_hot_reload_context_and_isolated_workbench() {
    let check2_source = load_source("coachmark_checklist");
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let docs_page_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs",
    );
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "let (test_css, set_test_css) = signal(default_test_css.get_untracked());",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "on_press=on_reset_test_css",
        "data-playground-scope=scope_id.clone()",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "data-slot=\"playground-test\"",
        "Original CSS is loaded. Use :scope to target this playground only.",
        "Show code",
        "Show test",
    ] {
        assert!(
            playground_source.contains(required),
            "playground infrastructure should keep DX hot-reload/isolation marker `{required}`."
        );
    }

    for required in [
        "title=\"Config + Code + CSS Test Workbench\"",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"components/coachmark/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "let (workbench_open_raw, set_workbench_open_raw) = signal(true);",
        "open=workbench_open",
        "on_open_change=on_workbench_open_change",
        "Toggle open",
    ] {
        assert!(
            docs_page_source.contains(required),
            "coachmark docs page should keep workbench/context persistence marker `{required}`."
        );
    }

    let script_marker = "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_dx_playground_supports_hot_reload_context_and_isolated_workbench";
    assert!(
        script_source.contains(script_marker),
        "dx gate script should include `{script_marker}`."
    );

    for required in [
        "- [x] DX 要求",
        "coachmark_dx_playground_supports_hot_reload_context_and_isolated_workbench",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark checklist should keep dx governance marker `{required}`."
        );
    }
}

#[test]
fn coachmark_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_page_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs",
    );

    for required in [
        "pub(super) fn coachmark() -> AnyView",
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming Optional / Snapshot\"",
        "code_imports=COACHMARK_DOC_IMPORTS.to_string()",
        "code_imports=COACHMARK_CONTROLLED_IMPORTS.to_string()",
        "data-slot=\"coachmark-state-matrix\"",
        "data-slot=\"coachmark-controlled-vs-uncontrolled\"",
        "data-slot=\"coachmark-streaming-modes\"",
        "data-slot=\"coachmark-streaming-policy\"",
        "Streaming Optional; fallback=snapshot.",
    ] {
        assert!(
            docs_page_source.contains(required),
            "Coachmark docs copy-paste-ready matrix should include `{required}`."
        );
    }
}

#[test]
fn coachmark_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_page_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs",
    );
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let code_block_view = include_str!("../../../components/code-block/src/view.rs");

    for required in [
        "data-slot=\"coachmark-source-first\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "label=\"Copy coachmark starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-coachmark-source-copy\".to_string()",
        "use leptos::prelude::*;\\nuse ui::{Coachmark, CoachmarkAssetVariant};",
        "data-slot=\"coachmark-source-prerequisites\"",
        "component-coachmark",
        "UiRoot",
        "inject-css",
        "data-slot=\"coachmark-source-paths\"",
        "components/coachmark/src/mod.rs",
        "components/coachmark/src/logic.rs",
        "components/coachmark/src/view.rs",
        "components/coachmark/src/styles.rs",
        "components/coachmark/src/motion.rs",
        "components/coachmark/src/protocol.rs",
    ] {
        assert!(
            docs_page_source.contains(required),
            "Coachmark source-first copy contract should include `{required}`."
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "data-slot=\"playground-toggle-code\"",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground copy-ready pipeline should include `{required}`."
        );
    }

    for required in [
        "class_name=\"ui-code-block__copy-button\"",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view.contains(required),
            "CodeBlock one-click copy affordance should include `{required}`."
        );
    }
}

#[test]
fn coachmark_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("coachmark_checklist");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
        "coachmark_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "coachmark_dx_check_script_covers_source_first_copy_paste_ready_contract",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark source-first checklist rule should include `{required}`."
        );
    }
}

#[test]
fn coachmark_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_dx_check_script_covers_source_first_copy_paste_ready_contract",
    ] {
        assert!(
            script_source.contains(required),
            "coachmark dx check script should include `{required}`."
        );
    }
}

#[test]
fn coachmark_check2_marks_source_first_copy_paste_ready_item_complete() {
    let check2_source = load_source("coachmark_checklist");

    for marker in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "components/coachmark/test/semantics.rs::coachmark_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "components/coachmark/test/semantics.rs::coachmark_check2_documents_source_first_copy_paste_ready_rules",
        "components/coachmark/test/semantics.rs::coachmark_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "components/coachmark/test/semantics.rs::coachmark_check2_marks_source_first_copy_paste_ready_item_complete",
    ] {
        assert!(
            check2_source.contains(marker),
            "coachmark checklist completion marker should include `{marker}`."
        );
    }
}

#[test]
fn coachmark_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_source("coachmark_checklist");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "coachmark check2 heroui-benchmark docs-sync section should include `{needle}`.",
        );
    }
}

#[test]
fn coachmark_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("heroui_strategy_doc");
    let pages_source = load_source("docs_pages_catalog");
    let docs_source = load_source("docs_coachmark_page");
    let readme_source = load_source("coachmark_readme");

    for needle in [
        "### Coachmark 同步记录（2026-02-20）",
        "参数模型同步：`Coachmark` 参数主轴保持 `variant/open + on_open_change + default_open/is_disabled`",
        "component_doc!(\"Coachmark\", \"coachmark\", \"Overlays\", overlays_extra_coachmark::coachmark)",
        "`apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs::coachmark()`",
        "`components/coachmark/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui strategy doc should include coachmark synchronization marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"Coachmark\"",
        "\"coachmark\"",
        "overlays_extra_coachmark::coachmark",
    ] {
        assert!(
            pages_source.contains(needle),
            "component docs index should expose coachmark entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn coachmark() -> AnyView",
        "title=\"Coachmark\"",
        "slug=\"coachmark\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app coachmark page should stay indexable via marker `{needle}`.",
        );
    }

    for needle in ["# Coachmark", "## Hello World（最小可用）"] {
        assert!(
            readme_source.contains(needle),
            "coachmark README should remain an equivalent component doc entry via `{needle}`.",
        );
    }
}

#[test]
fn coachmark_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: coachmark heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce heroui-benchmark docs-sync contract `{needle}`.",
        );
    }
}

#[test]
fn coachmark_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("coachmark_checklist");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "components/coachmark/test/semantics.rs::coachmark_check2_documents_heroui_benchmark_docs_sync_rules",
        "components/coachmark/test/semantics.rs::coachmark_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "components/coachmark/test/semantics.rs::coachmark_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "components/coachmark/test/semantics.rs::coachmark_check2_marks_heroui_benchmark_docs_sync_contract_complete",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "coachmark check2 should keep heroui-benchmark docs-sync evidence marker `{needle}`.",
        );
    }
}

#[test]
fn coachmark_check2_documents_docs_product_copy_paste_ready_rules() {
    let check2_source = load_source("coachmark_checklist");

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "coachmark_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "coachmark_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "coachmark_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark check2 docs-product rule should include `{required}`."
        );
    }
}

#[test]
fn coachmark_dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_docs_product_copy_paste_ready_rules",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_dx_check_script_covers_docs_product_copy_paste_ready_contract",
    ] {
        assert!(
            script_source.contains(required),
            "coachmark dx check script should include `{required}`."
        );
    }
}

#[test]
fn coachmark_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_source("coachmark_checklist");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark check2 should keep docs-sync/state-matrix rule `{required}`."
        );
    }
}

#[test]
fn coachmark_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_page_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs",
    );
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let check2_source = load_source("coachmark_checklist");

    for marker in [
        "pub fn resolve_default_open(default_open: Option<bool>) -> bool {",
        "default_open.unwrap_or(false)",
        "pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool {",
        "is_disabled.or(disabled).unwrap_or(false)",
        "#[prop(optional)] variant: CoachmarkVariant,",
        "#[prop(optional)] open: Option<Signal<bool>>,",
        "#[prop(optional)] default_open: Option<bool>,",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>,",
        "#[prop(optional)] is_disabled: Option<bool>,",
        "#[prop(optional)] disabled: Option<bool>,",
    ] {
        assert!(
            logic_source.contains(marker) || view_source.contains(marker),
            "coachmark API/default contract should keep marker `{marker}` for docs sync."
        );
    }

    for marker in [
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"State Matrix\"",
        "data-slot=\"coachmark-controlled-vs-uncontrolled\"",
        "data-slot=\"coachmark-state-matrix\"",
        "default_open=true",
        "open=controlled_open",
        "on_open_change=on_controlled_open_change",
        "is_disabled=true",
        "variant=CoachmarkVariant::Info",
        "data-slot=\"coachmark-defaults-contract\"",
        "variant=CoachmarkVariant::Help",
        "default_open=false",
        "resolve_default_open(default_open.unwrap_or(false))",
        "resolve_is_disabled(is_disabled.or(disabled).unwrap_or(false))",
        "open + on_open_change",
    ] {
        assert!(
            docs_page_source.contains(marker),
            "coachmark docs should keep synced example/matrix/default marker `{marker}`."
        );
    }

    for marker in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs",
        "components/coachmark/test/semantics.rs::coachmark_check2_documents_docs_sync_and_state_matrix_rules",
        "components/coachmark/test/semantics.rs::coachmark_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "coachmark check2 should include docs-sync/state-matrix evidence marker `{marker}`."
        );
    }
}

#[test]
fn coachmark_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "echo \"[dx] contract: coachmark docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(required),
            "coachmark dx check script should include docs-sync/state-matrix marker `{required}`."
        );
    }
}

#[test]
fn coachmark_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let check2_source = load_source("coachmark_checklist");

    for marker in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs",
        "data-slot=\"coachmark-state-matrix\"",
        "data-slot=\"coachmark-controlled-vs-uncontrolled\"",
        "data-slot=\"coachmark-defaults-contract\"",
        "resolve_default_open(default_open.unwrap_or(false))",
        "resolve_is_disabled(is_disabled.or(disabled).unwrap_or(false))",
        "components/coachmark/test/semantics.rs::coachmark_check2_documents_docs_sync_and_state_matrix_rules",
        "components/coachmark/test/semantics.rs::coachmark_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "components/coachmark/test/semantics.rs::coachmark_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "coachmark check2 docs-sync/state-matrix section should reference `{marker}`."
        );
    }
}

#[test]
fn coachmark_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("coachmark_checklist");

    for marker in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(marker),
            "coachmark check2 documentation-as-product section should include `{marker}`."
        );
    }
}

#[test]
fn coachmark_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = load_source("coachmark_readme");
    let pages_source = load_source("docs_pages_catalog");
    let docs_page_source = load_source("docs_coachmark_page");

    for marker in [
        "# Coachmark",
        "## Hello World（最小可用）",
        "## 先用起来，再进阶",
        "## 常见用法",
        "默认路径（先用起来）",
        "### Controlled Example（高级入口）",
    ] {
        assert!(
            readme_source.contains(marker),
            "coachmark README should include beginner-first marker `{marker}`."
        );
    }

    let readme_hello = readme_source
        .find("## Hello World（最小可用）")
        .expect("coachmark README should include hello-world section");
    let readme_beginner = readme_source
        .find("## 先用起来，再进阶")
        .expect("coachmark README should include beginner-first progression section");
    let readme_common = readme_source
        .find("## 常见用法")
        .expect("coachmark README should include common-usage section");
    let readme_advanced = readme_source
        .find("### Controlled Example（高级入口）")
        .expect("coachmark README should include controlled advanced section");
    assert!(
        readme_hello < readme_beginner
            && readme_beginner < readme_common
            && readme_common < readme_advanced,
        "coachmark README should keep beginner-first order (hello -> beginner -> common -> advanced)."
    );

    for marker in [
        "component_doc!(",
        "\"Coachmark\"",
        "\"coachmark\"",
        "overlays_extra_coachmark::coachmark",
        "pub(super) fn coachmark() -> AnyView",
        "title=\"Coachmark\"",
        "slug=\"coachmark\"",
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Config + Code + CSS Test Workbench\"",
    ] {
        assert!(
            pages_source.contains(marker) || docs_page_source.contains(marker),
            "coachmark docs entry should include `{marker}`."
        );
    }

    let docs_hello = docs_page_source
        .find("title=\"Hello World\"")
        .expect("coachmark docs should include hello-world playground");
    let docs_common = docs_page_source
        .find("title=\"Controlled vs Uncontrolled\"")
        .expect("coachmark docs should include controlled/uncontrolled common usage playground");
    let docs_advanced = docs_page_source
        .find("title=\"Config + Code + CSS Test Workbench\"")
        .expect("coachmark docs should include advanced workbench section");
    let docs_state_matrix = docs_page_source
        .find("title=\"State Matrix\"")
        .expect("coachmark docs should include state matrix playground");
    assert!(
        docs_hello < docs_common && docs_common < docs_advanced && docs_hello < docs_state_matrix,
        "coachmark docs page should keep beginner/common/advanced progression order."
    );
}

#[test]
fn coachmark_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for marker in [
        "echo \"[dx] contract: coachmark documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_documentation_as_product_rules",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(marker),
            "coachmark DX script should include documentation-as-product marker `{marker}`."
        );
    }
}

#[test]
fn coachmark_check2_marks_documentation_as_product_item_complete() {
    let check2_source = load_source("coachmark_checklist");

    for marker in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "components/coachmark/src/README.md",
        "apps/docs-app/src/pages/components/pages.rs",
        "## Hello World（最小可用）",
        "## 先用起来，再进阶",
        "## 常见用法",
        "components/coachmark/test/semantics.rs::coachmark_check2_documents_documentation_as_product_rules",
        "components/coachmark/test/semantics.rs::coachmark_documentation_entry_exists_with_beginner_first_progression",
        "components/coachmark/test/semantics.rs::coachmark_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "coachmark check2 documentation-as-product section should retain marker `{marker}`."
        );
    }
}

#[test]
fn coachmark_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("coachmark_checklist");

    for marker in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(marker),
            "coachmark check2 interactive-playground section should include `{marker}`."
        );
    }
}

#[test]
fn coachmark_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_page_source = load_source("docs_coachmark_page");
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");

    for marker in [
        "data-slot=\"coachmark-interactive-playground\"",
        "title=\"Config + Code + CSS Test Workbench\"",
        "test_config_signal=workbench_actual_config",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"components/coachmark/src/styles.rs\".to_string()",
        "code_signal=workbench_code",
        "data-slot=\"coachmark-workbench-controls\"",
        "data-slot=\"coachmark-workbench-toggle-variant\"",
        "data-slot=\"coachmark-workbench-toggle-disabled\"",
        "data-slot=\"coachmark-workbench-toggle-steps\"",
        "data-slot=\"coachmark-workbench-toggle-cta\"",
        "data-slot=\"coachmark-workbench-toggle-asset\"",
        "data-slot=\"coachmark-workbench-toggle-class\"",
        "data-slot=\"coachmark-workbench-toggle-open\"",
        "data-slot=\"coachmark-workbench-preview\"",
        "data-slot=\"coachmark-interactive-spec-linkage\"",
        "AI Spec Input -> Preview Output Linkage",
        "CoachmarkWorkbenchConfig {",
        "test_config_signal=workbench_actual_config",
        "ui.coachmark.agent-contract.v1",
    ] {
        assert!(
            docs_page_source.contains(marker),
            "coachmark docs interactive playground should include `{marker}`."
        );
    }

    for marker in [
        "data-slot=\"playground-toggle-settings\"",
        "data-slot=\"playground-toggle-code\"",
        "data-slot=\"playground-toggle-test\"",
        "data-slot=\"playground-controls\"",
        "data-slot=\"playground-test\"",
        "<div data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "test_config_signal.map(|signal| {",
        "\"Actual config\"",
    ] {
        assert!(
            playground_source.contains(marker),
            "docs-app Playground should keep interactive preview marker `{marker}`."
        );
    }
}

#[test]
fn coachmark_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("coachmark_e2e_contract");
    let docs_page_source = load_source("docs_coachmark_page");

    for marker in [
        "docs-app coachmark key flow keeps controlled markers stable",
        "docs-app coachmark key flow is repeatable with overlay focus and keyboard dismissal",
        "docs-app coachmark key flow is repeatable after reload",
        "const COACHMARK_CONTROLLED_TOGGLE_SELECTOR = '[data-slot=\"coachmark-controlled-toggle\"]';",
        "await page.keyboard.press(\"Escape\");",
        "await page.reload();",
        "waitForCoachmarkReady(page)",
        "waitForCoachmarkSettled(root)",
    ] {
        assert!(
            e2e_source.contains(marker),
            "coachmark interactive replay e2e flow should include `{marker}`."
        );
    }

    for marker in [
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"coachmark-controlled-vs-uncontrolled\"",
        "data-slot=\"coachmark-controlled-toggle\"",
        "data-slot=\"coachmark-interactive-playground\"",
    ] {
        assert!(
            docs_page_source.contains(marker),
            "coachmark docs should expose stable interactive anchor `{marker}` for repeatable e2e replay.",
        );
    }
}

#[test]
fn coachmark_dx_check_script_covers_interactive_playground_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for marker in [
        "echo \"[dx] contract: coachmark interactive playground docs acceptance surface\"",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_interactive_playground_rules",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(marker),
            "coachmark DX script should include interactive-playground marker `{marker}`."
        );
    }
}

#[test]
fn coachmark_check2_marks_interactive_playground_item_complete() {
    let check2_source = load_source("coachmark_checklist");

    for marker in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "data-slot=\"coachmark-interactive-playground\"",
        "data-slot=\"coachmark-workbench-controls\"",
        "data-slot=\"coachmark-interactive-spec-linkage\"",
        "e2e/tests/docs_app_coachmark_contract.spec.mjs",
        "components/coachmark/test/semantics.rs::coachmark_check2_documents_interactive_playground_rules",
        "components/coachmark/test/semantics.rs::coachmark_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "components/coachmark/test/semantics.rs::coachmark_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "components/coachmark/test/semantics.rs::coachmark_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "coachmark check2 interactive-playground section should retain marker `{marker}`."
        );
    }
}

#[test]
fn coachmark_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let manifest_source = include_str!("../src/Component.toml");
    let rbi_source = include_str!("../src/coachmark.rbi");
    let protocol_source = include_str!("../src/protocol.rs");
    let check2_source = load_source("coachmark_checklist");
    let combined = [
        load_source("mod"),
        load_source("logic"),
        load_source("view"),
        load_source("styles"),
        load_source("motion"),
    ]
    .join("\n");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Coachmark\"",
        "crate = \"ui-coachmark\"",
        "schema = \"ui.coachmark.agent-contract.v1\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "coachmark manifest should keep stable v1 schema marker `{needle}`."
        );
    }

    for needle in [
        "pub enum CoachmarkComponentSchemaVersion {",
        "V1,",
        "pub struct CoachmarkComponentSpec {",
        "pub schema_version: CoachmarkComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "coachmark protocol should keep stable schema marker `{needle}`."
        );
    }

    for needle in [
        "pub fn Coachmark(",
        "open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "coachmark RBI should keep stable public API marker `{needle}`."
        );
    }

    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "contract.v2",
        "SchemaRegistry",
    ] {
        assert!(
            !manifest_source.contains(forbidden)
                && !protocol_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !combined.contains(forbidden),
            "coachmark should not introduce major-version migration marker `{forbidden}` in current scope."
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Coachmark` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "coachmark_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
        "scripts/check-ui-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "coachmark/check2.md should keep version-migration governance marker `{needle}`."
        );
    }
}

#[test]
fn coachmark_version_deprecation_migration_script_covers_engineering_gate() {
    let script_source = include_str!("../../../scripts/check-ui-engineering.sh");
    let marker = "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`."
    );
}

#[test]
fn coachmark_engineering_contract_uses_serde_protocol_and_structured_schema_defaults() {
    let check2_source = load_source("coachmark_checklist");
    let coachmark_protocol_source = include_str!("../src/protocol.rs");
    let coachmark_protocol_test_source = include_str!("../test/protocol.rs");
    let coachmark_manifest = include_str!("../Cargo.toml");

    for required in [
        "use serde::{Deserialize, Serialize};",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(rename_all = \"snake_case\")]",
        "pub enum CoachmarkComponentSchemaVersion {",
        "pub struct CoachmarkComponentSpec {",
        "#[serde(default)]",
        "pub schema_version: CoachmarkComponentSchemaVersion,",
    ] {
        assert!(
            coachmark_protocol_source.contains(required),
            "coachmark engineering contract should keep serde protocol marker `{required}`."
        );
    }

    for required in [
        "use serde::de::DeserializeOwned;",
        "T: Serialize + DeserializeOwned,",
        "assert_serde::<CoachmarkComponentSchemaVersion>();",
        "assert_serde::<CoachmarkComponentSpec>();",
    ] {
        assert!(
            coachmark_protocol_test_source.contains(required),
            "coachmark protocol test should keep serde-coverage marker `{required}`."
        );
    }

    assert!(
        coachmark_manifest.contains("serde = { version = \"1.0\", features = [\"derive\"] }"),
        "coachmark manifest should keep serde dependency wiring."
    );

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark checklist should keep engineering serde governance marker `{required}`."
        );
    }
}

#[test]
fn coachmark_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let docs_app_source = include_str!("../../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("docs_debug_overlay");
    let trace_source = include_str!("../../../crates/ui-headless/src/trace.rs");
    let contextual_help_view = load_source("contextual_help_view");
    let ui_components_manifest = load_source("ui_components_manifest");
    let combined = [
        load_source("mod"),
        load_source("logic"),
        load_source("view"),
        load_source("styles"),
        load_source("motion"),
    ]
    .join("\n");

    for required in [
        "provide_ui_trace(debug_overlay_enabled);",
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "pub enum UiTraceEventKind {",
        "pub struct UiTraceEvent {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "use_controllable_open_state_traced(",
    ] {
        assert!(
            docs_app_source.contains(required)
                || debug_overlay_source.contains(required)
                || trace_source.contains(required)
                || contextual_help_view.contains(required),
            "coachmark tracing semantics should reuse shared marker `{required}`."
        );
    }

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_manifest.contains(required),
            "engineering baseline should keep canonical tracing feature marker `{required}`."
        );
    }

    for forbidden_feature in [
        "coachmark-wasm-debug =",
        "coachmark_wasm_debug =",
        "component-coachmark\", \"dep:tracing",
        "component-coachmark-wasm-debug",
    ] {
        assert!(
            !ui_components_manifest.contains(forbidden_feature),
            "coachmark should not define component-local tracing feature `{forbidden_feature}`."
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "const COACHMARK_TRACE_TARGET",
        "target: \"ui::coachmark::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "coachmark should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let coachmark_lib = include_str!("../src/lib.rs");
    let coachmark_readme = include_str!("../src/README.md");
    let sources = [
        coachmark_lib,
        load_source("mod"),
        load_source("logic"),
        load_source("view"),
        load_source("styles"),
        load_source("motion"),
        coachmark_readme,
    ];

    for source in sources {
        for forbidden in [
            "tokio",
            "tokio::",
            "async_std",
            "async_std::",
            "async-std",
            "runtime::Handle",
            "tokio::runtime",
            "smol::",
            "JoinHandle<",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "coachmark engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !load_source("mod").contains("web_sys"),
        "coachmark public module boundary should not leak web_sys types."
    );
}

#[test]
fn coachmark_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = include_str!("../../../scripts/check-ui-engineering.sh");

    for needle in [
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn coachmark_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals() {
    let styles_source = load_source("styles");
    let theme_css_source =
        source_contract::source_from_file_relative(file!(), "../../../crates/ui-theme/src/css.rs");
    let check2_source = load_source("coachmark_checklist");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-coachmark-accent, var(--ui-fg, var(--ui-fallback-fg)))",
    ] {
        assert!(
            styles_source.contains(required),
            "coachmark styles should keep defensive fallback chain token `{required}`."
        );
    }

    for required in [
        "--ui-fallback-space-sm:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-font-size-150:",
        "--ui-fallback-line-height-150:",
        "--ui-fallback-font-size-100:",
        "--ui-fallback-line-height-100:",
        "--ui-fallback-component-height-100:",
        "--ui-fallback-fg:",
        "--ui-fallback-accent:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme should stay SSOT for coachmark fallback token `{required}`."
        );
    }

    for forbidden in [
        "var(--ui-space-sm);",
        "var(--ui-space-xs);",
        "color: var(--ui-fg-muted);",
        "font-size: var(--ui-font-size-150, 14px);",
        "line-height: var(--ui-line-height-150, 20px);",
        "font-size: var(--ui-font-size-100, 12px);",
        "line-height: var(--ui-line-height-100, 16px);",
        "min-inline-size: 5.25rem;",
        "rgb(",
        "hsl(",
    ] {
        assert!(
            !styles_source.to_ascii_lowercase().contains(forbidden),
            "coachmark styles should not keep raw terminal value marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "coachmark_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark checklist should keep defensive-variable governance marker `{required}`."
        );
    }
}

#[test]
fn coachmark_cascade_layer_and_runtime_style_contract_is_enforced() {
    let ui_components_css = load_source("ui_components_css");
    let coachmark_view = load_source("view");
    let contextual_help_view = load_source("contextual_help_view");
    let popover_view = load_source("popover_view");
    let popover_logic = load_source("popover_logic");
    let check2_source = load_source("coachmark_checklist");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-coachmark\")]",
        "out.push_str(crate::coachmark::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css aggregation should keep @layer ui marker `{required}`."
        );
    }

    assert!(
        popover_view.contains("style=panel_vars"),
        "coachmark overlay path should keep runtime style transport via CSS-variable payload."
    );

    for required in [
        "--ui-popover-top:",
        "--ui-popover-left:",
        "--ui-popover-anchor-width:",
    ] {
        assert!(
            popover_logic.contains(required),
            "popover runtime style payload should use CSS custom property marker `{required}`."
        );
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"transform:",
        "style:top=",
        "style:left=",
        "style:transform=",
    ] {
        assert!(
            !coachmark_view.contains(forbidden)
                && !contextual_help_view.contains(forbidden)
                && !popover_view.contains(forbidden),
            "coachmark composition should not expose plain inline style marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "coachmark_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark checklist should keep cascade-layer governance marker `{required}`."
        );
    }
}

#[test]
fn coachmark_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let coachmark_motion = load_source("motion");
    let coachmark_view = load_source("view");
    let contextual_help_view = load_source("contextual_help_view");
    let popover_motion = load_source("popover_motion");
    let ui_motion_spring = load_source("ui_motion_spring");
    let platform_script = load_source("platform_script");
    let check2_source = load_source("coachmark_checklist");

    for required in [
        "pub struct CoachmarkMotion {",
        "pub popover: PopoverMotion,",
        "pub fn sanitize_motion(motion: CoachmarkMotion) -> CoachmarkMotion {",
        "crate::popover::motion::sanitize_motion(motion.popover)",
        "pub fn resolve_motion(motion: CoachmarkMotion) -> ContextualHelpMotion {",
        "ContextualHelpMotion {",
        "popover: motion.popover,",
    ] {
        assert!(
            coachmark_motion.contains(required),
            "coachmark motion contract should include `{required}`."
        );
    }

    for required in [
        "let motion = motion::resolve_motion(motion);",
        "motion=motion",
        "pub fn ContextualHelp(",
    ] {
        assert!(
            coachmark_view.contains(required) || contextual_help_view.contains(required),
            "coachmark/contextual-help motion wiring should include `{required}`."
        );
    }

    for required in [
        "pub struct PopoverMotion {",
        "stiffness: 300.0,",
        "damping: 25.0,",
        "pub fn sanitize_motion(motion: PopoverMotion) -> PopoverMotion {",
        "#[cfg(target_arch = \"wasm32\")]",
        "pub fn attach_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            popover_motion.contains(required),
            "popover motion contract should include `{required}`."
        );
    }

    for required in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
    ] {
        assert!(
            ui_motion_spring.contains(required),
            "ui-motion spring should include reduced-motion contract marker `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        platform_script.contains(script_needle),
        "platform gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "coachmark_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark checklist should keep motion-contract governance marker `{required}`."
        );
    }
}

#[test]
fn coachmark_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let check2_source = load_source("coachmark_checklist");
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");
    let ui_components_root = load_source("ui_components_root");
    let active_highlight =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let controllable_state = include_str!("../../../crates/ui-headless/src/controllable_state.rs");
    let presence = include_str!("../../../crates/ui-headless/src/presence.rs");
    let a11y = load_source("headless_a11y");
    let script_source = include_str!("../../../scripts/check-ui-entrypoints.sh");

    for required in [
        "#[cfg(feature = \"component-coachmark\")]",
        "pub mod coachmark;",
        "pub mod root;",
        "pub use root::UiRoot;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib.rs should keep fixed entry marker `{required}`.",
        );
    }

    for forbidden in ["pub use web_sys", "web_sys::", "NodeRef<", "JsValue"] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui lib.rs should not leak platform detail `{forbidden}`.",
        );
    }

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-coachmark\")]",
        "out.push_str(crate::coachmark::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css.rs should keep fixed entry marker `{required}`.",
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
    ] {
        assert!(
            ui_components_root.contains(required),
            "ui root.rs should keep centralized injection marker `{required}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight shared primitive should contain `{required}`.",
        );
    }

    for forbidden in ["Coachmark", "aria-", "data-state"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight should stay generic and avoid component semantic token `{forbidden}`.",
        );
    }

    for required in [
        "pub struct ControllableState<T>",
        "pub fn use_controllable_state<T>(",
    ] {
        assert!(
            controllable_state.contains(required),
            "ui-headless controllable_state canonical path should contain `{required}`.",
        );
    }

    for required in [
        "pub struct Presence",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
    ] {
        assert!(
            presence.contains(required),
            "ui-headless presence canonical path should contain `{required}`.",
        );
    }

    for required in ["pub fn locale_attrs(", "pub fn aria_controls_when_open("] {
        assert!(
            a11y.contains(required),
            "ui-headless a11y canonical path should contain `{required}`.",
        );
    }

    let ui_components_src_dir =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui/src");
    for forbidden_file in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src_dir.join(forbidden_file).exists(),
            "ui/src/{forbidden_file} should be absent by fixed-entrypoint contract.",
        );
    }

    let script_needle = "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(script_needle),
        "entrypoints gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] `ui` 固定入口文件落点正确。",
        "coachmark_ui_components_fixed_entry_files_follow_layered_boundaries",
    ] {
        assert!(
            check2_source.contains(required),
            "coachmark checklist should keep fixed-entrypoint governance marker `{required}`.",
        );
    }
}

#[test]
fn coachmark_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in [".unwrap(", ".expect(", ".unwrap_err(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "coachmark non-test source should forbid rust-hygiene anti-pattern `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("logic");

    for required in [
        "use std::borrow::Cow;",
        "let normalized_alt: Cow<'_, str> = normalize_optional_text(asset_alt)",
        ".map(Cow::Owned)",
        ".unwrap_or_else(|| Cow::Borrowed(asset_label));",
        "normalized_alt.into_owned()",
    ] {
        assert!(
            logic_source.contains(required),
            "coachmark logic should keep Cow-based string hotspot mitigation marker `{required}`."
        );
    }

    for forbidden in ["asset_label.to_string()", "String::from(asset_label)"] {
        assert!(
            !logic_source.contains(forbidden),
            "coachmark logic should avoid string clone hotspot `{forbidden}`."
        );
    }
}

#[test]
fn coachmark_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let rust_hygiene_script = include_str!("../../../scripts/check-rust-hygiene.sh");
    let engineering_script = include_str!("../../../scripts/check-ui-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            rust_hygiene_script.contains(required),
            "rust-hygiene gate script should enforce `{required}`."
        );
    }

    for needle in [
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn coachmark_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("coachmark_checklist");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "coachmark_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "coachmark_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "coachmark_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "./scripts/check-rust-hygiene.sh",
        "Cow<'static, str>",
    ] {
        assert!(
            check2_source.contains(needle),
            "coachmark check2 rust-hygiene section should reference `{needle}`."
        );
    }
}
