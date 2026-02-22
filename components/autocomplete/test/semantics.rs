const MOD_SOURCE: &str = include_str!("../src/mod.rs");
const LOGIC_SOURCE: &str = include_str!("../src/logic.rs");
const VIEW_SOURCE: &str = include_str!("../src/view.rs");
const STYLES_SOURCE: &str = include_str!("../src/styles.rs");
const MOTION_SOURCE: &str = include_str!("../src/motion.rs");
const PROTOCOL_SOURCE: &str = include_str!("../src/protocol.rs");
const COMPONENT_MANIFEST_SOURCE: &str = include_str!("../src/Component.toml");
const COMPONENT_RBI_SOURCE: &str = include_str!("../src/autocomplete.rbi");
const README_SOURCE: &str = include_str!("../src/README.md");
const BUTTON_SPEC_SOURCE: &str = include_str!("../../../components/button/src/spec.rs");
const COMPONENT_CARGO_SOURCE: &str = include_str!("../Cargo.toml");
const UI_COMPONENTS_CARGO_SOURCE: &str = include_str!("../../../crates/ui/Cargo.toml");
const UI_COMPONENTS_LIB_SOURCE: &str = include_str!("../../../crates/ui/src/lib.rs");
const UI_COMPONENTS_CSS_SOURCE: &str = include_str!("../../../crates/ui/src/css.rs");
const UI_COMPONENTS_ROOT_SOURCE: &str = include_str!("../../../crates/ui/src/root.rs");
const UI_VISUAL_PRIMITIVE_ACTIVE_HIGHLIGHT_SOURCE: &str =
    include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
const UI_THEME_CSS_SOURCE: &str = include_str!("../../../crates/ui-theme/src/css.rs");
const HEADLESS_A11Y_SOURCE: &str = include_str!("../../../crates/ui-headless/src/a11y.rs");
const HEADLESS_LIB_SOURCE: &str = include_str!("../../../crates/ui-headless/src/lib.rs");
const HEADLESS_CONTROLLABLE_STATE_SOURCE: &str =
    include_str!("../../../crates/ui-headless/src/controllable_state.rs");
const HEADLESS_PRESENCE_SOURCE: &str = include_str!("../../../crates/ui-headless/src/presence.rs");
const HEADLESS_ID_PROVIDER_SOURCE: &str =
    include_str!("../../../crates/ui-headless/src/id_provider.rs");
const MOTION_LIB_SOURCE: &str = include_str!("../../../crates/ui-motion/src/lib.rs");
const HEADLESS_POPOVER_SOURCE: &str =
    include_str!("../../../crates/ui-headless/src/popover_position.rs");
const HEADLESS_POPOVER_TEST_SOURCE: &str =
    include_str!("../../../crates/ui-headless/src/test/popover_position.rs");
const DOCS_COMPONENT_PAGES_SOURCE: &str =
    include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
const DOCS_COLLECTIONS_SOURCE: &str =
    include_str!("../../../apps/docs-app/src/pages/components/pages/collections.rs");
const DOCS_PLAYGROUND_SOURCE: &str = include_str!("../../../apps/docs-app/src/playground.rs");
const DOCS_COMPONENT_SHELL_SOURCE: &str =
    include_str!("../../../apps/docs-app/src/pages/components/shell.rs");
const DOCS_APP_LIB_SOURCE: &str = include_str!("../../../apps/docs-app/src/lib.rs");
const DOCS_PERF_PROBE_SOURCE: &str = include_str!("../../../apps/docs-app/src/perf_probe.rs");
const DOCS_DEBUG_OVERLAY_SOURCE: &str = include_str!("../../../apps/docs-app/src/debug_overlay.rs");
const DOCS_PLAN_TODO_SOURCE: &str = include_str!("../../../docs/plan/TODO.md");
const DOCS_THEME_VISUAL_BASELINE_SOURCE: &str =
    include_str!("../../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
const HEROUI_PARAMETER_STRATEGY_SOURCE: &str =
    include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");
const E2E_COMPONENTS_COVERAGE_SOURCE: &str =
    include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs");
const E2E_AUTOCOMPLETE_CONTRACT_SOURCE: &str =
    include_str!("../../../e2e/tests/docs_app_autocomplete_contract.spec.mjs");
const E2E_THEME_VISUAL_BASELINE_SOURCE: &str =
    include_str!("../../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
const AUTOCOMPLETE_E2E_SCRIPT_SOURCE: &str =
    include_str!("../../../components/autocomplete/scripts/check-ui-e2e-autocomplete.sh");
const TREE_SHAKING_SCRIPT_SOURCE: &str = include_str!("../../../scripts/check-ui-tree-shaking.sh");
const VIEW_MACRO_SCRIPT_SOURCE: &str = include_str!("../../../scripts/check-ui-view-macro.sh");
const PLATFORM_SCRIPT_SOURCE: &str = include_str!("../../../scripts/check-ui-platforms.sh");
const PERFORMANCE_SCRIPT_SOURCE: &str = include_str!("../../../scripts/check-ui-performance.sh");
const COMPONENT_FILES_SCRIPT_SOURCE: &str =
    include_str!("../../../scripts/check-ui-component-files.sh");
const DX_SCRIPT_SOURCE: &str = include_str!("../../../scripts/check-ui-dx.sh");
const WASM_DEBUG_SCRIPT_SOURCE: &str = include_str!("../../../scripts/check-ui-wasm-debug.sh");
const INNER_HTML_SCRIPT_SOURCE: &str = include_str!("../../../scripts/check-ui-inner-html.sh");
const CONTRACT_HYGIENE_SCRIPT_SOURCE: &str =
    include_str!("../../../scripts/check-ui-contract-hygiene.sh");
const ENGINEERING_SCRIPT_SOURCE: &str = include_str!("../../../scripts/check-ui-engineering.sh");
const TREE_SHAKING_BUDGET_SOURCE: &str = include_str!("../../../scripts/tree_shaking_budget.env");
const STATE_PRIMITIVES_AUTOCOMPLETE_SOURCE: &str =
    include_str!("../../../crates/ui-state-primitives/src/autocomplete.rs");
const UI_COMPONENTS_AUTOCOMPLETE_SEMANTICS_SOURCE: &str =
    include_str!("../../../components/autocomplete/test/autocomplete_semantics.rs");

#[test]
fn module_contract_keeps_public_api_stable_and_dom_free() {
    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::AutocompleteMotion;",
        "pub use view::Autocomplete;",
        "#[path = \"../test/semantics.rs\"]",
    ] {
        assert!(
            MOD_SOURCE.contains(needle),
            "Autocomplete module should include `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "web_sys",
        "HtmlElement",
        "NodeRef",
    ] {
        assert!(
            !MOD_SOURCE.contains(forbidden),
            "Autocomplete module should not expose `{forbidden}` in public API."
        );
    }
}

#[test]
fn layer_boundaries_keep_logic_view_styles_motion_separated() {
    assert!(
        LOGIC_SOURCE.contains("pub use ui_state_primitives::autocomplete::{"),
        "logic.rs should consume state primitives instead of re-implementing them."
    );
    assert!(
        !LOGIC_SOURCE.contains("view! {"),
        "logic.rs should not render Leptos view tree."
    );
    assert!(
        !LOGIC_SOURCE.contains("use_combo_box("),
        "logic.rs should not host ui-headless interaction contracts."
    );

    for needle in [
        "let accessibility_state =",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
        "let root_state = logic::normalize_root_state(logic::RootStateInput {",
        "let aria = use_combo_box(ComboBoxOptions {",
        "let text_field = use_text_field(TextFieldOptions {",
        "let motion = crate::motion::sanitize_motion(motion);",
        "crate::motion::attach_popover_motion(",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "view.rs should assemble layer contract via `{needle}`."
        );
    }

    assert!(
        STYLES_SOURCE.contains("var(--ui-"),
        "styles.rs should stay token-first and consume `--ui-*` variables."
    );
    for forbidden in ["on:click", "on:keydown", "use_combo_box(", "logic::"] {
        assert!(
            !STYLES_SOURCE.contains(forbidden),
            "styles.rs should not include runtime logic token `{forbidden}`."
        );
    }

    assert!(
        MOTION_SOURCE.contains("pub fn attach_popover_motion("),
        "motion.rs should own attach mapping for motion contracts."
    );
    for forbidden in ["role=", "aria-", "use_combo_box(", "view! {"] {
        assert!(
            !MOTION_SOURCE.contains(forbidden),
            "motion.rs should not encode view/a11y contract token `{forbidden}`."
        );
    }
}

#[test]
fn ui_components_root_reexports_autocomplete_contract() {
    assert!(
        UI_COMPONENTS_LIB_SOURCE
            .contains("pub use autocomplete::{Autocomplete, AutocompleteMotion};"),
        "ui root should re-export Autocomplete public API."
    );
}

#[test]
fn api_naming_contract_prefers_is_on_default_and_keeps_migration_aliases() {
    for needle in [
        "selected_index: Option<Signal<Option<usize>>>",
        "default_selected_index: Option<usize>",
        "on_selected_index_change: Option<Callback<Option<usize>>>",
        "set_selected_index: Option<WriteSignal<Option<usize>>>",
        "is_disabled: Option<bool>",
        "disabled: bool",
        "is_required: Option<Signal<bool>>",
        "required: Option<Signal<bool>>",
        "is_invalid: Option<Signal<bool>>",
        "invalid: Option<Signal<bool>>",
        "is_open: Option<Signal<bool>>",
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete API should expose `{needle}` for naming compatibility and migration."
        );
    }

    for needle in [
        "pub struct SelectionChangeInput",
        "pub struct SelectionChange",
        "pub fn normalize_selection_change(",
        "pub selected_index: Option<Signal<Option<usize>>>",
        "pub default_selected_index: Option<usize>",
        "pub is_controlled: bool",
        "pub selected_source: SelectedSource",
        "pub change_source: SelectedChangeSource",
        "pub enum SelectedSource",
        "pub enum SelectedChangeSource",
        "pub const fn as_attr(self) -> &'static str",
        "Self::SelectedIndex => \"selected_index\"",
        "Self::DefaultSelectedIndex => \"default_selected_index\"",
        "Self::OnSelectedIndexChange => \"on_selected_index_change\"",
        "Self::SetSelectedIndex => \"set_selected_index\"",
        "Self::None => \"none\"",
        "default_selected_index: Option<usize>",
        "item_count: usize",
        "let selected_source = if is_controlled {",
        "SelectedSource::SelectedIndex",
        "SelectedSource::DefaultSelectedIndex",
        "SelectedChangeSource::OnSelectedIndexChange",
        "SelectedChangeSource::SetSelectedIndex",
        "SelectedChangeSource::None",
    ] {
        assert!(
            LOGIC_SOURCE.contains(needle),
            "logic.rs should normalize selection callback naming via `{needle}`."
        );
    }

    for needle in [
        "let selection_change = logic::normalize_selection_change(logic::SelectionChangeInput {",
        "selected_index,",
        "default_selected_index,",
        "let selected_source_attr = selection_change.selected_source.as_attr();",
        "let selected_change_source_attr = selection_change.change_source.as_attr();",
        "let selected_state = overlay_open::use_controllable_state(",
        "Some(selection_change.default_selected_index),",
        "selection_change.on_selected_index_change,",
        "let selected_index = selected_state.value;",
        "let request_selected_index_change = selected_state.request_change;",
        "request_selected_index_change.run(Some(original_index));",
        "data-selected-source=selected_source_attr",
        "data-selected-controlled=is_selected_controlled.then_some(\"true\")",
        "data-selected-uncontrolled=(!is_selected_controlled).then_some(\"true\")",
        "data-selected-change-source=selected_change_source_attr",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "view.rs should consume normalized naming contract via `{needle}`."
        );
    }

    for needle in [
        "selection 受控/非受控轴：`selected_index` + `on_selected_index_change` + `default_selected_index`",
        "兼容别名（迁移期）：`open`、`disabled`、`required`、`invalid`、`set_selected_index`",
        "迁移建议：优先使用 `is_*` / `on_*` / `default_*` 命名；`set_selected_index` 仅作为兼容桥接",
    ] {
        assert!(
            README_SOURCE.contains(needle),
            "README should document API naming migration contract `{needle}`."
        );
    }
}

#[test]
fn default_value_priority_is_centralized_in_logic_only() {
    assert!(
        LOGIC_SOURCE.contains("pub i18n_empty_message: Option<String>"),
        "logic.rs should type empty-message fallback inputs explicitly."
    );
    assert!(
        LOGIC_SOURCE
            .contains("resolve_empty_message(input.empty_message.or(input.i18n_empty_message))"),
        "logic.rs should own empty-message fallback priority."
    );

    for forbidden in [
        "empty_message: empty_message",
        ".or_else(|| Some(common.autocomplete_empty_message.to_string()))",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "view.rs should not inline default fallback token `{forbidden}`."
        );
    }

    assert!(
        VIEW_SOURCE
            .contains("i18n_empty_message: Some(common.autocomplete_empty_message.to_string()),"),
        "view.rs should only pass i18n slot through logic input."
    );
}

#[test]
fn a11y_i18n_locale_contracts_are_headless_backed_and_not_component_reimplemented() {
    for needle in [
        "use_combo_box(ComboBoxOptions {",
        "use_text_field(TextFieldOptions {",
        "use_ui_i18n",
        "CommonStrings",
        "let i18n = use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "i18n_empty_message: Some(common.autocomplete_empty_message.to_string()),",
        "role=aria.listbox.role",
        "role=aria.input.role",
        "aria-autocomplete=aria.input.aria_autocomplete",
        "aria-controls=move || aria.input.aria_controls.get()",
        "lang=aria.input.lang.clone()",
        "dir=aria.input.dir",
        "lang=aria.listbox.lang.clone()",
        "dir=aria.listbox.dir",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete should keep a11y/i18n/locale contract marker `{needle}`."
        );
    }

    assert!(
        !VIEW_SOURCE.contains("\"No matches\""),
        "Autocomplete view.rs should not hardcode user-visible empty-state copy."
    );

    for needle in ["pub enum A11yDirection", "pub fn locale_attrs("] {
        assert!(
            HEADLESS_A11Y_SOURCE.contains(needle),
            "ui-headless a11y source should include shared marker `{needle}`."
        );
    }

    for forbidden in ["mod a11y;", "fn locale_attrs("] {
        assert!(
            !MOD_SOURCE.contains(forbidden)
                && !VIEW_SOURCE.contains(forbidden)
                && !LOGIC_SOURCE.contains(forbidden),
            "Autocomplete component layer should avoid reimplementing shared a11y helper `{forbidden}`."
        );
    }
}

#[test]
fn state_normalization_is_centralized_in_logic_before_view_consumes_it() {
    for needle in [
        "pub struct InputStateSource",
        "pub fn reduce_sync_from_selection(",
        "pub fn reduce_after_option_commit(",
        "pub fn reduce_after_input_blur(",
        "pub fn reduce_after_input_change(",
    ] {
        assert!(
            LOGIC_SOURCE.contains(needle),
            "logic.rs should centralize input-event state reduction via `{needle}`."
        );
    }

    for needle in [
        "logic::reduce_sync_from_selection(",
        "logic::reduce_after_option_commit(",
        "logic::reduce_after_input_blur(",
        "logic::reduce_after_input_change(",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "view.rs should consume centralized logic reducer `{needle}`."
        );
    }

    for forbidden in [
        "logic::reduce_input_state(",
        "logic::AutocompleteInputEvent::",
        "logic::AutocompleteInputState {",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "view.rs should not reconstruct state-machine internals `{forbidden}`."
        );
    }
}

#[test]
fn non_composite_api_uses_single_items_axis_instead_of_parallel_slot_convention() {
    for needle in ["#[component]\npub fn Autocomplete(", "items: Vec<String>"] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete should keep a simple non-composite API marker `{needle}`."
        );
    }

    for forbidden in [
        "labels:",
        "titles:",
        "panels:",
        "children: Children",
        "AutocompleteItem",
        "ItemSpec",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden) && !README_SOURCE.contains(forbidden),
            "Autocomplete should not expose composite-slot convention token `{forbidden}`."
        );
    }
}

#[test]
fn non_drag_component_does_not_introduce_drag_macro_micro_state_machine() {
    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "on:drag",
        "ondrag",
        "draggable=",
    ] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden)
                && !VIEW_SOURCE.contains(forbidden)
                && !MOTION_SOURCE.contains(forbidden),
            "Autocomplete should not introduce drag state-machine token `{forbidden}`."
        );
    }
}

#[test]
fn two_pass_geometry_pipeline_uses_measure_and_guarded_rectification() {
    for needle in [
        "use_popover_position(PopoverPositionOptions {",
        "position.top_px.get()",
        "position.left_px.get()",
        "position.anchor_width_px.get()",
        "data-placement=move || position.placement.get().as_str()",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete view should keep measure-stage marker `{needle}`."
        );
    }

    for needle in [
        "fn compute_popover_position(",
        "const POSITION_EPSILON_PX: f64 = 0.01;",
        "fn should_update_scalar(current: f64, next: f64) -> bool",
        "if should_update_scalar(top_px.get_untracked(), computed.top) {",
        "if should_update_scalar(left_px.get_untracked(), computed.left) {",
        "if placement.get_untracked() != computed.placement {",
        "ResizeObserver",
    ] {
        assert!(
            HEADLESS_POPOVER_SOURCE.contains(needle),
            "ui-headless popover position should keep guarded rectification marker `{needle}`."
        );
    }

    for needle in [
        "fn scalar_update_guard_ignores_sub_epsilon_noise()",
        "fn scalar_update_guard_accepts_meaningful_delta()",
    ] {
        assert!(
            HEADLESS_POPOVER_TEST_SOURCE.contains(needle),
            "ui-headless popover tests should keep convergence regression `{needle}`."
        );
    }
}

#[test]
fn non_registered_collection_flow_uses_filtered_vec_order_not_hashset_iteration() {
    for needle in [
        "let filtered_indices = Memo::new",
        "logic::filter_indices(",
        "logic::map_selected_to_filtered(selected_index.get(), &filtered_indices.get())",
        "logic::map_filtered_to_original(filtered_index, &indices)",
        "disabled_indices.contains(&original)",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete should keep deterministic collection-order marker `{needle}`."
        );
    }

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "for index in disabled_indices",
    ] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden) && !VIEW_SOURCE.contains(forbidden),
            "Autocomplete should avoid registration-protocol token `{forbidden}` for non-composite collection flow.",
        );
    }
}

#[test]
fn non_container_component_does_not_expose_slot_projection_modes() {
    for needle in [
        "use_presence(is_open)",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete should keep predictable presence lifecycle marker `{needle}`."
        );
    }

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
    ] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden)
                && !VIEW_SOURCE.contains(forbidden)
                && !MOTION_SOURCE.contains(forbidden),
            "Autocomplete should avoid container slot-projection token `{forbidden}`."
        );
    }
}

#[test]
fn env_streams_are_delegated_to_headless_without_raw_event_flood_in_component_layer() {
    for needle in [
        "use_popover_position(PopoverPositionOptions {",
        "position.top_px.get()",
        "position.left_px.get()",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete view should consume headless geometry/env stream output `{needle}`."
        );
    }

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "match_media",
        "on:resize",
        "on:scroll",
        "BreakpointChanged",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden) && !LOGIC_SOURCE.contains(forbidden),
            "Autocomplete component layer should not own raw env stream token `{forbidden}`."
        );
    }

    for needle in [
        "web_sys::ResizeObserver",
        "add_event_listener_with_callback(\"resize\",",
        "add_event_listener_with_callback_and_bool(",
        "\"scroll\"",
        "if should_update_scalar(top_px.get_untracked(), computed.top) {",
    ] {
        assert!(
            HEADLESS_POPOVER_SOURCE.contains(needle),
            "ui-headless popover position should keep env stream sampling/guard marker `{needle}`."
        );
    }
}

#[test]
fn event_light_cone_contract_stays_out_for_non_grid_autocomplete() {
    for needle in [
        "items: Vec<String>",
        "let filtered_indices = Memo::new",
        "logic::filter_indices(",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete should keep single-list filtering flow marker `{needle}`."
        );
    }

    for forbidden in [
        "SelectionState::All",
        "ContextBus",
        "context_bus",
        "provide_context(",
        "use_context(",
        "selector(",
        "Selector<",
        "prop_drilling",
    ] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden)
                && !VIEW_SOURCE.contains(forbidden)
                && !MOTION_SOURCE.contains(forbidden),
            "Autocomplete should avoid event-light-cone token `{forbidden}` for non-grid scope."
        );
    }
}

#[test]
fn causality_bus_contract_is_not_introduced_for_local_component_interaction() {
    for needle in [
        "let on_option_click = aria.handlers.on_option_click;",
        "request_selected_index_change.run(Some(original_index));",
        "set_open,",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete should keep direct local interaction flow marker `{needle}`."
        );
    }

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "causality_bus",
        "broadcast(",
        "subscribe(",
        "subscriber",
    ] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden)
                && !VIEW_SOURCE.contains(forbidden)
                && !MOTION_SOURCE.contains(forbidden),
            "Autocomplete should avoid causality-bus token `{forbidden}` for local interaction scope."
        );
    }
}

#[test]
fn styles_depend_on_semantic_markers_and_runtime_inline_style_is_css_var_only() {
    for required in [
        ".ui-autocomplete[data-empty=\"true\"] .ui-autocomplete__input",
        ".ui-autocomplete[data-controlled=\"true\"] .ui-autocomplete__control",
        ".ui-autocomplete[data-has-disabled-options=\"true\"] .ui-autocomplete__listbox",
        ".ui-autocomplete__panel[data-placement=\"bottom-start\"]",
        ".ui-autocomplete__option[data-selected=\\\"true\\\"]",
        ".ui-autocomplete__option[data-focused=\\\"true\\\"]",
    ] {
        assert!(
            STYLES_SOURCE.contains(required),
            "Autocomplete styles should keep semantic selector `{required}`."
        );
    }

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        " > * > * > ",
        " + div + div",
    ] {
        assert!(
            !STYLES_SOURCE.contains(forbidden),
            "Autocomplete styles should avoid brittle structural selector `{forbidden}`."
        );
    }

    for needle in [
        "let panel_vars = move || {",
        "\"--ui-popover-top: {}px; --ui-popover-left: {}px; --ui-popover-anchor-width: {}px;\"",
        "style=panel_vars",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete view should keep runtime style variable marker `{needle}`."
        );
    }

    let inline_style_count = VIEW_SOURCE.matches("style=").count();
    assert_eq!(
        inline_style_count, 1,
        "Autocomplete view should keep exactly one inline style binding for required css variables."
    );
    assert!(
        !VIEW_SOURCE.contains("style=\""),
        "Autocomplete view should avoid hardcoded inline style literals."
    );
}

#[test]
fn styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals() {
    for needle in [
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
        "var(--ui-shadow-md, var(--ui-fallback-shadow-md))",
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))",
        "var(--ui-overlay-viewport-inset, var(--ui-fallback-overlay-viewport-inset))",
        "var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index))",
        "var(--ui-overlay-enter-offset-y, var(--ui-fallback-overlay-enter-offset-y))",
        "var(--ui-overlay-enter-scale, var(--ui-fallback-overlay-enter-scale))",
    ] {
        assert!(
            STYLES_SOURCE.contains(needle),
            "Autocomplete styles should keep defensive fallback chain marker `{needle}`."
        );
    }

    for needle in [
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-space-md:",
        "--ui-fallback-font-size-150:",
        "--ui-fallback-line-height-150:",
        "--ui-fallback-font-size-100:",
        "--ui-fallback-line-height-100:",
        "--ui-fallback-border-width:",
        "--ui-fallback-border:",
        "--ui-fallback-radius-md:",
        "--ui-fallback-radius-lg:",
        "--ui-fallback-bg:",
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-focus-ring:",
        "--ui-fallback-danger:",
        "--ui-fallback-accent-soft:",
        "--ui-fallback-shadow-md:",
        "--ui-fallback-overlay-panel-min-width:",
        "--ui-fallback-overlay-viewport-inset:",
        "--ui-fallback-overlay-z-index:",
        "--ui-fallback-overlay-enter-offset-y:",
        "--ui-fallback-overlay-enter-scale:",
    ] {
        assert!(
            UI_THEME_CSS_SOURCE.contains(needle),
            "ui-theme css should provide fallback terminal `{needle}`."
        );
    }

    for forbidden in [
        "14px", "20px", "12px", "16px", "240px", "0px", "1px", "2px", "3px",
    ] {
        assert!(
            !STYLES_SOURCE.contains(forbidden),
            "Autocomplete styles should avoid raw terminal token `{forbidden}`."
        );
    }
}

#[test]
fn defensive_variables_check_script_covers_style_fallback_contract() {
    let needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_styles_use_defensive_variable_fallback_chain";
    assert!(
        CONTRACT_HYGIENE_SCRIPT_SOURCE.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn check2_marks_defensive_variables_contract_complete() {
    assert!(
        include_str!("../check2.md").contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "Autocomplete check2 should mark defensive-variables gate complete."
    );

    for needle in [
        "autocomplete_styles_use_defensive_variable_fallback_chain",
        "defensive_variables_check_script_covers_style_fallback_contract",
        "scripts/check-ui-contract-hygiene.sh",
        "components/autocomplete/src/styles.rs",
        "crates/ui-theme/src/css.rs",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "Autocomplete check2 defensive-variables section should reference `{needle}`."
        );
    }
}

#[test]
fn cascade_layer_and_runtime_style_contract_is_enforced() {
    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-autocomplete\")]",
        "out.push_str(crate::autocomplete::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            UI_COMPONENTS_CSS_SOURCE.contains(needle),
            "ui css entry should enforce cascade-layer contract `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            UI_COMPONENTS_ROOT_SOURCE.contains(needle),
            "UiRoot should keep centralized css injection contract `{needle}`."
        );
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
        "style=\"position:",
        "style:top=",
        "style:left=",
        "style:right=",
        "style:bottom=",
        "style:width=",
        "style:height=",
        "style:position=",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "autocomplete view should not include plain inline style token `{forbidden}`."
        );
    }

    let style_lines: Vec<&str> = VIEW_SOURCE
        .lines()
        .filter(|line| line.contains("style="))
        .collect();
    assert_eq!(
        style_lines.len(),
        1,
        "autocomplete view should keep a single runtime style binding for css vars."
    );
    assert!(
        style_lines[0].contains("style=panel_vars"),
        "autocomplete runtime style binding should route through `panel_vars`."
    );

    for needle in [
        "let panel_vars = move || {",
        "--ui-popover-top: {}px;",
        "--ui-popover-left: {}px;",
        "--ui-popover-anchor-width: {}px;",
        "position.top_px.get()",
        "position.left_px.get()",
        "position.anchor_width_px.get()",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "autocomplete runtime style payload should stay css-custom-property-only via `{needle}`."
        );
    }
}

#[test]
fn cascade_layer_check_script_covers_contract() {
    let needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        CONTRACT_HYGIENE_SCRIPT_SOURCE.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn check2_marks_cascade_layer_contract_complete() {
    assert!(
        include_str!("../check2.md").contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "autocomplete check2 should mark cascade-layer gate complete."
    );

    for needle in [
        "cascade_layer_and_runtime_style_contract_is_enforced",
        "autocomplete_cascade_layer_and_runtime_style_contract_is_enforced",
        "scripts/check-ui-contract-hygiene.sh",
        "crates/ui/src/css.rs",
        "crates/ui/src/root.rs",
        "components/autocomplete/src/view.rs",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "autocomplete check2 cascade-layer section should reference `{needle}`."
        );
    }
}

#[test]
fn view_macro_complexity_is_split_into_semantic_subblocks() {
    for needle in [
        "struct AutocompleteOptionViewCtx {",
        "fn render_autocomplete_option(ctx: AutocompleteOptionViewCtx) -> impl IntoView",
        "fn render_autocomplete_description(",
        "fn render_autocomplete_error(",
        "render_autocomplete_option(AutocompleteOptionViewCtx {",
        "let description_view =",
        "render_autocomplete_description(description, text_field.description.id.clone())",
        "let error_view = render_autocomplete_error(error, text_field.error.id.clone(), invalid);",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete view should keep macro-complexity split marker `{needle}`."
        );
    }

    for forbidden in [
        "let description_id = text_field.description.id.clone();\n                view! {",
        "let error_id = text_field.error.id.clone();\n                let error_id = StoredValue::new(error_id);",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "Autocomplete view should not regress to inlined optional-section macro token `{forbidden}`."
        );
    }

    let view_macro_count = VIEW_SOURCE.matches("view! {").count();
    assert!(
        view_macro_count <= 5,
        "Autocomplete view macro complexity regression: expected <= 5 `view!` blocks, found {view_macro_count}."
    );

    assert!(
        VIEW_MACRO_SCRIPT_SOURCE.contains(
            "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_view_macro_complexity_is_split_into_semantic_subrenders",
        ),
        "view-macro gate script should include autocomplete complexity test target.",
    );

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "view_macro_complexity_is_split_into_semantic_subblocks",
        "autocomplete_view_macro_complexity_is_split_into_semantic_subrenders",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "Autocomplete check2 should keep view-macro complexity marker `{needle}`.",
        );
    }
}

#[test]
fn view_functional_split_prefers_plain_functions_over_local_components() {
    for needle in [
        "fn render_autocomplete_option(ctx: AutocompleteOptionViewCtx) -> impl IntoView",
        "fn render_autocomplete_description(",
        "fn render_autocomplete_error(",
        "render_autocomplete_option(AutocompleteOptionViewCtx {",
        "let description_view =",
        "let error_view = render_autocomplete_error(error, text_field.error.id.clone(), invalid);",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete view should keep function-first split marker `{needle}`."
        );
    }

    let component_count = VIEW_SOURCE.matches("#[component]").count();
    assert_eq!(
        component_count, 2,
        "Autocomplete should keep exactly two component boundaries (root + panel); found {component_count}.",
    );

    for needle in [
        "#[component]\nfn AutocompletePanel(",
        "#[component]\npub fn Autocomplete(",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete view should keep only required component boundary marker `{needle}`."
        );
    }

    assert!(
        VIEW_MACRO_SCRIPT_SOURCE.contains(
            "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_view_functional_split_prefers_plain_functions_over_local_components",
        ),
        "view-macro gate script should include autocomplete function-first test target.",
    );

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "view_functional_split_prefers_plain_functions_over_local_components",
        "autocomplete_view_functional_split_prefers_plain_functions_over_local_components",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "Autocomplete check2 should keep function-first split marker `{needle}`.",
        );
    }
}

#[test]
fn static_fragments_are_constantized_or_absent_for_simple_combobox_layout() {
    for needle in [
        "class=\"ui-active-highlight\"",
        "data-slot=\"autocomplete-highlight\"",
        "class=\"ui-autocomplete__empty\" data-slot=\"autocomplete-empty\"",
        "empty_message.get_value()",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete view should keep stable static-fragment anchor `{needle}`."
        );
    }

    for forbidden in [
        "<svg",
        "</svg>",
        "<footer",
        "</footer>",
        "include_str!(",
        "markdown_to_html",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "Autocomplete should avoid heavy static fragment token `{forbidden}` in view layer."
        );
    }

    assert!(
        VIEW_MACRO_SCRIPT_SOURCE.contains(
            "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_static_fragments_are_constantized_or_absent_for_simple_combobox_layout",
        ),
        "view-macro gate script should include autocomplete static-fragment test target.",
    );

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "static_fragments_are_constantized_or_absent_for_simple_combobox_layout",
        "autocomplete_static_fragments_are_constantized_or_absent_for_simple_combobox_layout",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "Autocomplete check2 should keep static-fragment governance marker `{needle}`.",
        );
    }
}

#[test]
fn inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let component_combined = format!(
        "{}\n{}\n{}\n{}\n{}",
        MOD_SOURCE, LOGIC_SOURCE, VIEW_SOURCE, STYLES_SOURCE, MOTION_SOURCE
    );

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
    ] {
        assert!(
            !component_combined.contains(forbidden),
            "Autocomplete component layer must not include raw html injection token `{forbidden}`."
        );
    }

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !README_SOURCE.contains(forbidden),
            "Autocomplete README examples must not contain raw html injection token `{forbidden}`."
        );
    }

    assert!(
        INNER_HTML_SCRIPT_SOURCE.contains(
            "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_inner_html_usage_is_forbidden_in_component_and_docs_examples",
        ),
        "inner-html gate script should include autocomplete raw-html contract test target.",
    );

    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A：`Autocomplete` 当前无 `inner_html` 使用点",
        "inner_html_usage_is_forbidden_in_component_and_docs_examples",
        "autocomplete_inner_html_usage_is_forbidden_in_component_and_docs_examples",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "Autocomplete check2 should keep inner_html safety marker `{needle}`.",
        );
    }
}

#[test]
fn wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    assert!(
        UI_COMPONENTS_CARGO_SOURCE
            .contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "shared wasm debug capability should remain feature-gated via `button-wasm-debug`."
    );

    let all_components_start = UI_COMPONENTS_CARGO_SOURCE
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = UI_COMPONENTS_CARGO_SOURCE[all_components_start..]
        .find("\n\ncomponent-accordion")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before component feature declarations");
    let all_components_block =
        &UI_COMPONENTS_CARGO_SOURCE[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "wasm debug feature must not be pulled into all-components production path."
    );
    assert!(
        !UI_COMPONENTS_CARGO_SOURCE.contains("autocomplete-wasm-debug"),
        "Autocomplete should not define component-local wasm debug feature when shared trace overlay is sufficient."
    );

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            DOCS_APP_LIB_SOURCE.contains(needle),
            "docs app should keep wasm debug visual entry marker `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "let ts_ms = event.ts_ms;",
        "UiTraceEventKind::OpenChange",
        "UiTraceEventKind::Inspect",
        "UiTraceEventKind::Note",
    ] {
        assert!(
            DOCS_DEBUG_OVERLAY_SOURCE.contains(needle),
            "global trace timeline should keep marker `{needle}`."
        );
    }

    for needle in [
        "let open_state = overlay_open::use_controllable_open_state_traced(",
        "\"autocomplete\",",
        "let key_result = aria.handlers.on_input_key_down.run(key);",
        "data-state=move || {",
        "data-selected-source=selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete should keep traceable state/interaction marker `{needle}`."
        );
    }

    for needle in [
        "docs-app autocomplete key flow is repeatable with semantic contract breakpoints",
        "await controlledInput.fill(\"Shen\")",
        "await option.click()",
        "await page.reload()",
    ] {
        assert!(
            E2E_AUTOCOMPLETE_CONTRACT_SOURCE.contains(needle),
            "Autocomplete e2e should keep replayable key flow marker `{needle}`."
        );
    }

    for forbidden in [
        "autocomplete-wasm-debug",
        "wasm_debug",
        "data-debug-source=",
        "request_replay.run(",
    ] {
        assert!(
            !MOD_SOURCE.contains(forbidden)
                && !LOGIC_SOURCE.contains(forbidden)
                && !VIEW_SOURCE.contains(forbidden)
                && !MOTION_SOURCE.contains(forbidden),
            "Autocomplete should not ship component-local wasm debug runtime token `{forbidden}`."
        );
    }

    assert!(
        WASM_DEBUG_SCRIPT_SOURCE.contains(
            "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
        ),
        "wasm-debug gate script should include autocomplete contract test target."
    );

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
        "autocomplete_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "Autocomplete check2 should keep wasm-debug marker `{needle}`.",
        );
    }
}

#[test]
fn dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Show settings\"",
        "\"Show code\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            DOCS_PLAYGROUND_SOURCE.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn autocomplete() -> AnyView {",
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "test_css_source=workbench_test_css",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/autocomplete/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"autocomplete-workbench\"",
        "data-slot=\"autocomplete-workbench-canvas\"",
        "ui::autocomplete::styles::CSS",
    ] {
        assert!(
            DOCS_COLLECTIONS_SOURCE.contains(needle),
            "Autocomplete docs should keep DX CSS hot-reload marker `{needle}`."
        );
    }
}

#[test]
fn dx_workbench_supports_optional_state_persistence_and_isolated_canvas() {
    for needle in [
        "AUTOCOMPLETE_WORKBENCH_STORAGE_KEY",
        "fn load_autocomplete_workbench_selected() -> Option<usize>",
        "fn save_autocomplete_workbench_selected(selected_index: usize)",
        "fn clear_autocomplete_workbench_selected()",
        "let persisted_autocomplete_workbench_selected = load_autocomplete_workbench_selected();",
        "let (workbench_persist_state, set_workbench_persist_state) =",
        "save_autocomplete_workbench_selected(selected_index);",
        "clear_autocomplete_workbench_selected();",
        "\" Persist selected index (optional)\"",
        "\" · persist selected: \"",
        "data-slot=\"autocomplete-workbench-controls\"",
        "data-slot=\"autocomplete-workbench\"",
        "data-slot=\"autocomplete-workbench-canvas\"",
    ] {
        assert!(
            DOCS_COLLECTIONS_SOURCE.contains(needle),
            "Autocomplete workbench should keep optional-persistence marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            DOCS_COLLECTIONS_SOURCE.contains(needle),
            "Autocomplete workbench persistence should keep platform guard `{needle}`."
        );
    }

    for needle in [
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            DX_SCRIPT_SOURCE.contains(needle),
            "dx check script should enforce `{needle}`."
        );
    }

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "autocomplete_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "autocomplete_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "Autocomplete check2 should keep DX marker `{needle}`.",
        );
    }
}

#[test]
fn docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    for needle in [
        "pub(super) fn autocomplete() -> AnyView {",
        "title=\"Hello World\"",
        "code_imports=autocomplete_code_imports.clone()",
        "title=\"Controlled Open State\"",
        "title=\"Disabled + Empty\"",
        "data-slot=\"autocomplete-state-matrix\"",
        "状态矩阵 State Matrix（受控 / 非受控）",
        "data-slot=\"autocomplete-state-rows\"",
        "title=\"Streaming/Snapshot Display\"",
        "data-slot=\"autocomplete-streaming-snapshot\"",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "data-ui-output-state=\"snapshot\"",
        "data-ui-output-state=\"streaming\"",
        "id_base=\"docs-autocomplete-snapshot\".to_string()",
        "id_base=\"docs-autocomplete-streaming\".to_string()",
        "label=\"Snapshot mode\".to_string()",
        "label=\"Streaming preview\".to_string()",
        "data-slot=\"autocomplete-source-first\"",
        "<Snippet",
        "label=\"Copy starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-autocomplete-source-copy\".to_string()",
        "use leptos::prelude::*;\\nuse ui::Autocomplete;",
        "data-slot=\"autocomplete-source-paths\"",
        "data-slot=\"autocomplete-source-prerequisites\"",
        "<code>\"component-autocomplete\"</code>",
        "<code>\"inject-css\"</code>",
    ] {
        assert!(
            DOCS_COLLECTIONS_SOURCE.contains(needle),
            "Autocomplete docs should keep docs-product marker `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let missing_imports = missing_import_lines(&raw, &imports);",
        "return compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value());",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            DOCS_PLAYGROUND_SOURCE.contains(needle),
            "Playground copy-ready pipeline should keep `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        DX_SCRIPT_SOURCE.contains(script_needle),
        "dx check script should enforce `{script_needle}`."
    );

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "autocomplete_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "apps/docs-app/src/pages/components/pages/collections.rs",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "Autocomplete check2 should keep docs-product marker `{needle}`."
        );
    }
}

#[test]
fn check2_documents_docs_sync_and_state_matrix_rules() {
    let check2 = include_str!("../check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "已同步补齐示例与双矩阵",
        "状态矩阵为 `section[data-slot=\"autocomplete-state-matrix\"]`；参数矩阵为 `section[data-slot=\"autocomplete-parameter-matrix\"]`",
        "文档参数矩阵默认值与实现一致",
    ] {
        assert!(
            check2.contains(required),
            "Autocomplete check2 should keep docs-sync/state-matrix rule `{required}`."
        );
    }
}

#[test]
fn docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>,",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional, into)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_selected_index: Option<usize>",
        "#[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>",
        "#[prop(optional)] set_selected_index: Option<WriteSignal<Option<usize>>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "pub fn normalize_open_state(input: OpenStateInput) -> OpenState",
        "pub fn normalize_selection_change(input: SelectionChangeInput) -> SelectionChange",
        "pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState",
        "let default_selected_index = input",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle) || LOGIC_SOURCE.contains(needle),
            "Autocomplete API/default contract should keep marker `{needle}` for docs sync."
        );
    }

    for needle in [
        "pub const DEFAULT_LABEL: &str = \"Options\";",
        "pub const DEFAULT_ID_BASE: &str = \"autocomplete\";",
        "pub const DEFAULT_PLACEHOLDER: &str = \"Type…\";",
        "pub const DEFAULT_EMPTY_MESSAGE: &str = \"No matches\";",
        "pub fn resolve_placeholder(placeholder: Option<String>) -> String",
        "pub fn resolve_empty_message(value: Option<String>) -> String",
    ] {
        assert!(
            STATE_PRIMITIVES_AUTOCOMPLETE_SOURCE.contains(needle) || LOGIC_SOURCE.contains(needle),
            "Autocomplete primitive/default source should keep marker `{needle}`."
        );
    }

    for needle in [
        "title=\"Hello World\"",
        "title=\"Selection + Validation\"",
        "title=\"Controlled Open State\"",
        "title=\"Disabled + Empty\"",
        "data-slot=\"autocomplete-state-matrix\"",
        "data-slot=\"autocomplete-state-rows\"",
        "data-slot=\"autocomplete-parameter-matrix\"",
        "data-slot=\"autocomplete-parameter-rows\"",
        "<code>\"is_open + on_open_change + default_open\"</code>",
        "<code>\"selected_index + on_selected_index_change + default_selected_index\"</code>",
        "<code>\"set_selected_index\"</code>",
        "<code>\"is_disabled / is_required / is_invalid\"</code>",
        "<code>\"label / id_base / placeholder / empty_message\"</code>",
        "is_open=controlled_open",
        "on_open_change=on_open_change",
        "selected_index=controlled_selected",
        "set_selected_index=set_controlled_selected",
        "is_disabled=true",
        "selected_index=empty_selected",
        "set_selected_index=set_empty_selected",
    ] {
        assert!(
            DOCS_COLLECTIONS_SOURCE.contains(needle),
            "Autocomplete docs should keep synced example/matrix marker `{needle}`."
        );
    }

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "apps/docs-app/src/pages/components/pages/collections.rs::autocomplete",
        "check2_documents_docs_sync_and_state_matrix_rules",
        "docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "Autocomplete check2 should keep docs-sync evidence marker `{needle}`."
        );
    }
}

#[test]
fn dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    for needle in [
        "echo \"[dx] contract: autocomplete docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            DX_SCRIPT_SOURCE.contains(needle),
            "DX check script should include docs-sync/state-matrix marker `{needle}`."
        );
    }
}

#[test]
fn check2_marks_docs_sync_and_state_matrix_item_complete() {
    let check2 = include_str!("../check2.md");

    assert!(
        check2.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "Autocomplete check2 should mark docs-sync/state-matrix checklist item complete."
    );

    for needle in [
        "apps/docs-app/src/pages/components/pages/collections.rs::autocomplete",
        "data-slot=\"autocomplete-state-matrix\"",
        "data-slot=\"autocomplete-parameter-matrix\"",
        "DEFAULT_LABEL",
        "DEFAULT_ID_BASE",
        "DEFAULT_PLACEHOLDER",
        "DEFAULT_EMPTY_MESSAGE",
        "check2_documents_docs_sync_and_state_matrix_rules",
        "docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(needle),
            "Autocomplete check2 docs-sync/state-matrix section should reference `{needle}`."
        );
    }
}

#[test]
fn check2_documents_documentation_as_product_rules() {
    let check2 = include_str!("../check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "已形成新手优先路径：`Hello World` 零门槛示例 + `常见用法`（默认 API）在前，`受控 open 示例`（进阶控制）在后",
        "`apps/docs-app/src/pages/components/pages.rs` 继续保留 `component_doc!(\"Autocomplete\", \"autocomplete\", \"Collections\", collections::autocomplete)` 文档入口",
        "验证记录：`bash -n scripts/check-ui-dx.sh` 通过",
    ] {
        assert!(
            check2.contains(required),
            "Autocomplete check2 should keep documentation-as-product rule `{required}`."
        );
    }
}

#[test]
fn documentation_entry_exists_with_beginner_first_progression() {
    assert!(
        README_SOURCE.contains("# Autocomplete"),
        "Autocomplete should keep a discoverable README entry."
    );

    for needle in [
        "## Hello World",
        "先跑默认路径，不需要先理解分层细节。",
        "## 常见用法",
        "非受控 open：仅传 `default_open`",
        "受控 open：传 `is_open + on_open_change`",
        "## 受控 open 示例",
        "进阶控制路径：当你需要把开合状态与上层流程同步时，再使用受控 open。",
        "## Architecture Layers",
        "## Source-first",
    ] {
        assert!(
            README_SOURCE.contains(needle),
            "Autocomplete README should keep beginner-first marker `{needle}`."
        );
    }

    let hello_pos = README_SOURCE.find("## Hello World");
    let common_pos = README_SOURCE.find("## 常见用法");
    let advanced_pos = README_SOURCE.find("## 受控 open 示例");
    assert!(
        hello_pos.is_some() && common_pos.is_some() && advanced_pos.is_some(),
        "Autocomplete README should keep hello/common/advanced sections."
    );
    assert!(
        hello_pos < common_pos && common_pos < advanced_pos,
        "Autocomplete README should keep beginner path before advanced control."
    );

    for needle in [
        "component_doc!(",
        "\"Autocomplete\",",
        "\"autocomplete\",",
        "\"Collections\",",
        "collections::autocomplete",
    ] {
        assert!(
            DOCS_COMPONENT_PAGES_SOURCE.contains(needle),
            "docs-app catalog should keep Autocomplete entrypoint marker `{needle}`."
        );
    }
}

#[test]
fn dx_check_script_covers_documentation_as_product_contract() {
    for needle in [
        "echo \"[dx] contract: autocomplete documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_documentation_as_product_rules",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            DX_SCRIPT_SOURCE.contains(needle),
            "DX check script should include documentation-as-product marker `{needle}`."
        );
    }
}

#[test]
fn check2_marks_documentation_as_product_item_complete() {
    let check2 = include_str!("../check2.md");

    assert!(
        check2.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "Autocomplete check2 should mark documentation-as-product item complete."
    );

    for needle in [
        "components/autocomplete/src/README.md",
        "apps/docs-app/src/pages/components/pages.rs",
        "check2_documents_documentation_as_product_rules",
        "documentation_entry_exists_with_beginner_first_progression",
        "dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(needle),
            "Autocomplete check2 documentation-as-product section should reference `{needle}`."
        );
    }
}

#[test]
fn check2_documents_interactive_playground_rules() {
    let check2 = include_str!("../check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
        "AI Spec 子条款对 `Autocomplete` 为 N/A（组件不承载 Spec 输入协议，交互验收以 props/state Workbench + Streaming/Snapshot 契约为准）",
        "check2_documents_interactive_playground_rules",
        "docs_app_provides_interactive_playground_for_props_state_and_preview",
        "interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            check2.contains(required),
            "Autocomplete check2 should keep interactive-playground rule `{required}`."
        );
    }
}

#[test]
fn docs_app_provides_interactive_playground_for_props_state_and_preview() {
    for marker in [
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "test_config_signal=workbench_actual_config",
        "test_css_source=workbench_test_css",
        "controls=move || view! {",
        "data-slot=\"autocomplete-workbench-controls\"",
        "data-slot=\"autocomplete-workbench-canvas\"",
        "Persist selected index (optional)",
        "\"Toggle open\"",
        "title=\"Streaming/Snapshot Display\"",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "<AiSpace mode=snapshot_mode output_status=verified_output>",
        "<AiSpace mode=streaming_mode output_status=draft_output>",
    ] {
        assert!(
            DOCS_COLLECTIONS_SOURCE.contains(marker),
            "Autocomplete docs interactive playground should include `{marker}`."
        );
    }

    for marker in [
        "<div data-playground-scope=scope_id.clone()>",
        "<Card class_name=\"playground__preview\".to_string()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<div data-slot=\"playground-controls\">",
        "Card class_name=\"playground__panel playground__controls\".to_string()",
    ] {
        assert!(
            DOCS_PLAYGROUND_SOURCE.contains(marker),
            "docs-app Playground should keep interactive preview marker `{marker}`."
        );
    }
}

#[test]
fn interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    for marker in [
        "docs-app autocomplete key flow is repeatable with semantic contract breakpoints",
        "docs-app autocomplete high-risk overlay/focus/keyboard path is replayable with semantic breakpoints",
        "#docs-autocomplete-controlled-input",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"data-closed\", \"true\")",
        "toHaveText(\"selected: 3\")",
        "await page.reload();",
    ] {
        assert!(
            E2E_AUTOCOMPLETE_CONTRACT_SOURCE.contains(marker),
            "Autocomplete interactive-playground e2e flow should include `{marker}`."
        );
    }

    for marker in [
        "data-slot=\"autocomplete-controlled-playground\"",
        "data-slot=\"autocomplete-controlled-open\"",
        "data-slot=\"autocomplete-controlled-selected\"",
        "data-slot=\"autocomplete-workbench-controls\"",
        "data-slot=\"autocomplete-workbench-canvas\"",
    ] {
        assert!(
            DOCS_COLLECTIONS_SOURCE.contains(marker),
            "Autocomplete docs should expose stable interactive anchor `{marker}` for repeatable e2e replay."
        );
    }
}

#[test]
fn dx_check_script_covers_interactive_playground_contract() {
    for marker in [
        "echo \"[dx] contract: autocomplete interactive playground docs acceptance surface\"",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_interactive_playground_rules",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            DX_SCRIPT_SOURCE.contains(marker),
            "DX check script should include interactive-playground marker `{marker}`."
        );
    }
}

#[test]
fn check2_marks_interactive_playground_item_complete() {
    let check2 = include_str!("../check2.md");

    assert!(
        check2.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "Autocomplete check2 should mark interactive-playground item complete."
    );

    for marker in [
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"autocomplete-workbench-controls\"",
        "docs-app autocomplete key flow is repeatable with semantic contract breakpoints",
        "AI Spec 子条款对 `Autocomplete` 为 N/A（组件不承载 Spec 输入协议，交互验收以 props/state Workbench + Streaming/Snapshot 契约为准）",
        "check2_documents_interactive_playground_rules",
        "docs_app_provides_interactive_playground_for_props_state_and_preview",
        "interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-dx.sh",
        "e2e/tests/docs_app_autocomplete_contract.spec.mjs",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(marker),
            "Autocomplete check2 interactive-playground section should include `{marker}`."
        );
    }
}

#[test]
fn check2_documents_source_first_copy_paste_ready_rules() {
    let check2 = include_str!("../check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
        "check2_documents_source_first_copy_paste_ready_rules",
        "docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            check2.contains(required),
            "Autocomplete check2 source-first section should include `{required}`."
        );
    }
}

#[test]
fn docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot();

    for marker in [
        "data-slot=\"autocomplete-source-first\"",
        "label=\"Copy starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-autocomplete-source-copy\".to_string()",
        "code_imports=autocomplete_code_imports.clone()",
        "use leptos::prelude::*;\\nuse ui::Autocomplete;",
        "data-slot=\"autocomplete-source-paths\"",
        "components/autocomplete/src/mod.rs",
        "components/autocomplete/src/logic.rs",
        "components/autocomplete/src/view.rs",
        "components/autocomplete/src/styles.rs",
        "components/autocomplete/src/motion.rs",
        "data-slot=\"autocomplete-source-prerequisites\"",
        "<code>\"component-autocomplete\"</code>",
        "<code>\"inject-css\"</code>",
    ] {
        assert!(
            DOCS_COLLECTIONS_SOURCE.contains(marker),
            "Autocomplete source-first docs should include `{marker}`."
        );
    }

    for marker in [
        "code_imports: Option<String>",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines(&raw, &imports)",
        "<CodeBlock code=resolved_code.get() />",
        "data-slot=\"playground-code\"",
    ] {
        assert!(
            DOCS_PLAYGROUND_SOURCE.contains(marker),
            "playground source-first copy path should include `{marker}`."
        );
    }
}

#[test]
fn dx_check_script_covers_source_first_copy_paste_ready_contract() {
    for marker in [
        "echo \"[dx] contract: autocomplete source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            DX_SCRIPT_SOURCE.contains(marker),
            "DX check script should include source-first marker `{marker}`."
        );
    }
}

#[test]
fn check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2 = include_str!("../check2.md");

    assert!(
        check2.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "Autocomplete check2 should mark source-first copy-paste-ready item complete."
    );

    for marker in [
        "data-slot=\"autocomplete-source-first\"",
        "Snippet(label=\"Copy starter\", copyable=true)",
        "code_imports=autocomplete_code_imports",
        "components/autocomplete/src/mod.rs",
        "components/autocomplete/src/logic.rs",
        "components/autocomplete/src/view.rs",
        "components/autocomplete/src/styles.rs",
        "components/autocomplete/src/motion.rs",
        "check2_documents_source_first_copy_paste_ready_rules",
        "docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(marker),
            "Autocomplete check2 source-first section should include `{marker}`."
        );
    }
}

#[test]
fn check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2 = include_str!("../check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2.contains(required),
            "Autocomplete check2 heroui-benchmark docs-sync section should include `{required}`."
        );
    }
}

#[test]
fn heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    for marker in [
        "### Autocomplete 同步记录（2026-02-18）",
        "参数模型同步：`Autocomplete` 保持 `is_open/open + on_open_change + default_open`、`is_disabled/disabled`、`is_required/required`、`is_invalid/invalid` 轴",
        "`apps/docs-app/src/pages/components/pages.rs` 继续通过 `component_doc!(\"Autocomplete\", \"autocomplete\", \"Collections\", collections::autocomplete)` 暴露入口",
        "`apps/docs-app/src/pages/components/pages/collections.rs::autocomplete()`",
        "Source-first / Copy-Paste Ready",
        "参数语义变更必须先同步本策略文档与 docs 页面",
    ] {
        assert!(
            HEROUI_PARAMETER_STRATEGY_SOURCE.contains(marker),
            "heroui strategy doc should include autocomplete sync marker `{marker}`."
        );
    }

    for marker in [
        "component_doc!(",
        "\"Autocomplete\",",
        "\"autocomplete\",",
        "\"Collections\",",
        "collections::autocomplete",
    ] {
        assert!(
            DOCS_COMPONENT_PAGES_SOURCE.contains(marker),
            "component docs index should expose autocomplete entry marker `{marker}`."
        );
    }

    for marker in [
        "pub(super) fn autocomplete() -> AnyView {",
        "title=\"Autocomplete\"",
        "slug=\"autocomplete\"",
    ] {
        assert!(
            DOCS_COLLECTIONS_SOURCE.contains(marker),
            "autocomplete docs-app page should stay indexable via marker `{marker}`."
        );
    }
}

#[test]
fn dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    for marker in [
        "echo \"[dx] contract: autocomplete heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            DX_SCRIPT_SOURCE.contains(marker),
            "DX check script should include heroui-benchmark docs-sync marker `{marker}`."
        );
    }
}

#[test]
fn check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2 = include_str!("../check2.md");

    assert!(
        check2.contains("- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。"),
        "Autocomplete check2 should mark heroui-benchmark docs-sync item complete."
    );

    for marker in [
        "docs/spec/heroui-parameter-design-strategy.md",
        "apps/docs-app/src/pages/components/pages.rs",
        "apps/docs-app/src/pages/components/pages/collections.rs",
        "component_doc!(\"Autocomplete\", \"autocomplete\", \"Collections\", collections::autocomplete)",
        "check2_documents_heroui_benchmark_docs_sync_rules",
        "heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(marker),
            "Autocomplete check2 heroui-benchmark docs-sync section should include `{marker}`."
        );
    }
}

#[test]
fn engineering_contract_uses_serde_protocol_and_structured_schema_defaults() {
    assert!(
        COMPONENT_CARGO_SOURCE.contains("serde = { version = \"1.0\", features = [\"derive\"] }"),
        "Autocomplete crate should keep serde derive dependency for structured protocol schema."
    );
    assert!(
        !COMPONENT_CARGO_SOURCE.contains("serde_json"),
        "Autocomplete should avoid serde_json fan-out when protocol only requires typed serde schema defaults."
    );

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "pub enum AutocompleteComponentSchemaVersion",
        "#[serde(rename_all = \"snake_case\")]",
        "pub struct AutocompleteComponentSpec",
        "#[serde(default)]",
        "pub schema_version: AutocompleteComponentSchemaVersion,",
    ] {
        assert!(
            PROTOCOL_SOURCE.contains(needle),
            "Autocomplete protocol should keep structured serde schema marker `{needle}`."
        );
    }

    for forbidden in [
        "serde_json::",
        "SchemaError",
        "from_json(",
        "to_json_result(",
    ] {
        assert!(
            !PROTOCOL_SOURCE.contains(forbidden),
            "Autocomplete protocol should avoid unsupported migration/error token `{forbidden}` in current v1 scope."
        );
    }

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "autocomplete_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "autocomplete_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "autocomplete_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "Autocomplete check2 should keep engineering marker `{needle}`."
        );
    }
}

#[test]
fn engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    assert!(
        UI_COMPONENTS_CARGO_SOURCE
            .contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "engineering baseline should keep canonical tracing feature gate."
    );
    assert!(
        !UI_COMPONENTS_CARGO_SOURCE.contains("autocomplete-wasm-debug")
            && !UI_COMPONENTS_CARGO_SOURCE.contains("component-autocomplete-wasm-debug"),
        "Autocomplete should not define component-local tracing debug feature in cargo feature graph."
    );

    let combined = format!(
        "{}\n{}\n{}\n{}",
        MOD_SOURCE, LOGIC_SOURCE, VIEW_SOURCE, MOTION_SOURCE
    );
    assert!(
        combined.contains("overlay_open::use_controllable_open_state_traced(")
            && combined.contains("\"autocomplete\""),
        "Autocomplete should reuse shared traced controllable-state hook."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::autocomplete::",
        "const AUTOCOMPLETE_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Autocomplete should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let combined = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}",
        MOD_SOURCE,
        LOGIC_SOURCE,
        VIEW_SOURCE,
        STYLES_SOURCE,
        MOTION_SOURCE,
        PROTOCOL_SOURCE,
        README_SOURCE
    );

    for forbidden in [
        "tokio",
        "tokio::",
        "async_std",
        "async_std::",
        "async-std",
        "runtime::Handle",
        "smol::",
        "spawn_blocking(",
        "futures::executor",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Autocomplete engineering contract should not leak runtime marker `{forbidden}`."
        );
    }

    assert!(
        !MOD_SOURCE.contains("web_sys"),
        "Autocomplete public module boundary should not leak web_sys types."
    );
}

#[test]
fn engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    for needle in [
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            ENGINEERING_SCRIPT_SOURCE.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade() {
    for needle in [
        "pub enum AutocompleteComponentSchemaVersion {",
        "V1,",
        "pub struct AutocompleteComponentSpec {",
        "pub schema_version: AutocompleteComponentSchemaVersion,",
    ] {
        assert!(
            PROTOCOL_SOURCE.contains(needle),
            "Autocomplete protocol should keep v1 schema marker `{needle}` in non-breaking scope."
        );
    }

    for needle in [
        "schema_version = \"1\"",
        "schema = \"ui.autocomplete.agent-contract.v1\"",
        "values = [\"v1\"]",
    ] {
        assert!(
            COMPONENT_MANIFEST_SOURCE.contains(needle),
            "Autocomplete Component.toml should keep v1 registration marker `{needle}` in non-breaking scope."
        );
    }

    for needle in ["pub enum AutocompleteAgentSchemaVersion {", "V1,"] {
        assert!(
            COMPONENT_RBI_SOURCE.contains(needle),
            "Autocomplete RBI should keep v1 marker `{needle}` in non-breaking scope."
        );
    }

    let combined = [
        MOD_SOURCE,
        LOGIC_SOURCE,
        VIEW_SOURCE,
        STYLES_SOURCE,
        MOTION_SOURCE,
        PROTOCOL_SOURCE,
        COMPONENT_MANIFEST_SOURCE,
        COMPONENT_RBI_SOURCE,
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
            "without major breaking upgrade, autocomplete should not introduce migration token `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        ENGINEERING_SCRIPT_SOURCE.contains(script_needle),
        "engineering check script should enforce `{script_needle}`."
    );

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Autocomplete` 改动未引入跨大版本 API 破坏升级",
        "AutocompleteComponentSchemaVersion::V1",
        "schema_version = \"1\"",
        "ui.autocomplete.agent-contract.v1",
        "AutocompleteAgentSchemaVersion::V1",
        "version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
        "autocomplete_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
        "scripts/check-ui-engineering.sh",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "Autocomplete check2 should keep version-deprecation marker `{needle}`."
        );
    }
}

#[test]
fn semantic_contract_matrix_covers_state_a11y_input_paths_and_platform_guards() {
    for needle in [
        "role=aria.input.role",
        "aria-controls=move || aria.input.aria_controls.get()",
        "data-state=move ||",
        "data-selected-source=selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
        "data-selected-controlled=is_selected_controlled.then_some(\"true\")",
        "data-selected-uncontrolled=(!is_selected_controlled).then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-focus-visible=move || focus_ring.is_focus_visible.get().then_some(\"true\")",
        "let key_result = aria.handlers.on_input_key_down.run(key);",
        "if key_result.handled {",
        "if key_result.stop_propagation {",
        "on:pointermove=move |_| on_option_pointer_move.run(filtered_index)",
        "on:click=move |_| on_option_click.run(filtered_index)",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete semantic matrix should include `{needle}`."
        );
    }

    for needle in [
        "feature = \"web\"",
        "feature = \"ssr\"",
        "compile_error!(",
        "features `web` and `ssr` are mutually exclusive; enable exactly one",
    ] {
        assert!(
            HEADLESS_LIB_SOURCE.contains(needle),
            "ui-headless should keep platform-guard marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
    ] {
        assert!(
            MOTION_LIB_SOURCE.contains(needle),
            "ui-motion should keep non-wasm fallback marker `{needle}`."
        );
    }
}

#[test]
fn token_first_static_style_contract_is_aggregated_and_injected_via_ui_root() {
    assert!(
        STYLES_SOURCE.contains("pub const CSS: &str"),
        "Autocomplete styles should stay as static CSS contract constant."
    );
    assert!(
        STYLES_SOURCE.contains("var(--ui-"),
        "Autocomplete styles should consume ui-theme css variables."
    );
    assert!(
        UI_COMPONENTS_CSS_SOURCE.contains("out.push_str(crate::autocomplete::styles::CSS);"),
        "ui css aggregator should include autocomplete styles contract."
    );

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            UI_COMPONENTS_ROOT_SOURCE.contains(needle),
            "UiRoot should inject component CSS through `{needle}`."
        );
    }

    let combined = format!(
        "{}\n{}\n{}\n{}",
        LOGIC_SOURCE, VIEW_SOURCE, STYLES_SOURCE, MOTION_SOURCE
    );
    for forbidden in [
        "tailwind",
        "utility-first",
        "styled_components",
        "emotion",
        "css!(",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Autocomplete component sources should avoid css-framework default token `{forbidden}`."
        );
    }
}

#[test]
fn visual_desire_default_theme_baseline_is_guarded_by_docs_and_screenshot_specs() {
    for needle in [
        "component_doc!(",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
    ] {
        assert!(
            DOCS_COMPONENT_PAGES_SOURCE.contains(needle),
            "docs component registry should include theme visual baseline marker `{needle}`."
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "Default theme should feel trustworthy at first glance",
    ] {
        assert!(
            DOCS_THEME_VISUAL_BASELINE_SOURCE.contains(needle),
            "theme visual baseline docs page should contain `{needle}`."
        );
    }

    for needle in [
        "docs-app: theme visual baseline renders button/input/overlay",
        "docs-app: theme visual baseline screenshots",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            E2E_THEME_VISUAL_BASELINE_SOURCE.contains(needle),
            "theme visual baseline e2e spec should include `{needle}`."
        );
    }
}

#[test]
fn tree_shaking_contract_keeps_feature_gates_and_ci_budget_pipeline_explicit() {
    for needle in [
        "component-autocomplete = [\"component-active_highlight\", \"component-popover\", \"dep:ui-autocomplete\"]",
        "#[cfg(feature = \"component-autocomplete\")]\npub use ui_autocomplete as autocomplete;",
        "#[cfg(feature = \"component-autocomplete\")]\n    out.push_str(crate::autocomplete::styles::CSS);",
    ] {
        assert!(
            UI_COMPONENTS_CARGO_SOURCE.contains(needle)
                || UI_COMPONENTS_LIB_SOURCE.contains(needle)
                || UI_COMPONENTS_CSS_SOURCE.contains(needle),
            "Autocomplete tree-shaking gate should include `{needle}`."
        );
    }

    for needle in [
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "cargo tree -e features -i ui -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            TREE_SHAKING_SCRIPT_SOURCE.contains(needle)
                || TREE_SHAKING_BUDGET_SOURCE.contains(needle),
            "Tree-shaking verification pipeline should include `{needle}`."
        );
    }
}

#[test]
fn tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    for needle in [
        "AUTOCOMPLETE_MIN_FEATURES=\"component-autocomplete,inject-css\"",
        "autocomplete_tree_shaking_feature_gates_are_explicit",
        "autocomplete_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "autocomplete_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "AUTOCOMPLETE_TREE_OUTPUT=\"$(cargo tree -e features -i ui -p ui --no-default-features --features \"$AUTOCOMPLETE_MIN_FEATURES\")\"",
        "missing command-line feature: component-autocomplete",
        "missing command-line feature: inject-css for autocomplete minimal tree",
        "autocomplete minimal feature tree should not pull all-components",
        "[tree-shaking] autocomplete minimal wasm check",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$AUTOCOMPLETE_MIN_FEATURES\"",
    ] {
        assert!(
            TREE_SHAKING_SCRIPT_SOURCE.contains(needle),
            "autocomplete tree-shaking script should include `{needle}`."
        );
    }
}

#[test]
fn check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2 = include_str!("../check2.md");
    assert!(
        check2.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
        "autocomplete check2 should mark tree-shaking feature-pruning item complete."
    );

    for needle in [
        "component-autocomplete = [\"component-active_highlight\", \"component-popover\", \"dep:ui-autocomplete\"]",
        "#[cfg(feature = \"component-autocomplete\")] pub use ui_autocomplete as autocomplete;",
        "#[cfg(feature = \"component-autocomplete\")] out.push_str(crate::autocomplete::styles::CSS);",
        "tree_shaking_contract_keeps_feature_gates_and_ci_budget_pipeline_explicit",
        "tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "autocomplete_tree_shaking_feature_gates_are_explicit",
        "autocomplete_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "autocomplete_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "scripts/check-ui-tree-shaking.sh",
        "cargo tree -e features -p ui --no-default-features --features component-autocomplete,inject-css",
        "cargo tree -e features -i ui -p web-demo",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-autocomplete,inject-css",
    ] {
        assert!(
            check2.contains(needle),
            "autocomplete tree-shaking check2 section should reference `{needle}`."
        );
    }
}

#[test]
fn platform_compile_only_contract_covers_default_ssr_wasm_and_non_wasm_source_guard() {
    for needle in [
        "[platform] compile-only: default native path",
        "cargo check -p ui",
        "[platform] compile-only: ssr native path",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "[platform] compile-only: web wasm path (ui-headless)",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "[platform] compile-only: autocomplete native path",
        "cargo check -p ui --no-default-features --features component-autocomplete,inject-css",
        "[platform] compile-only: autocomplete wasm path",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-autocomplete,inject-css",
        "[platform] source guard: non-wasm autocomplete files must not reference web_sys",
        "components/autocomplete/src/view.rs",
    ] {
        assert!(
            PLATFORM_SCRIPT_SOURCE.contains(needle),
            "platform compile-only/source-guard pipeline should include `{needle}`."
        );
    }

    for needle in [
        "feature = \"web\"",
        "feature = \"ssr\"",
        "compile_error!(",
        "features `web` and `ssr` are mutually exclusive; enable exactly one",
    ] {
        assert!(
            HEADLESS_LIB_SOURCE.contains(needle),
            "ui-headless should keep explicit web/ssr feature mutex marker `{needle}`."
        );
    }

    let combined = format!(
        "{}\n{}\n{}\n{}\n{}",
        MOD_SOURCE, LOGIC_SOURCE, VIEW_SOURCE, STYLES_SOURCE, MOTION_SOURCE
    );
    for forbidden in ["web_sys::window", "window()", "document()"] {
        assert!(
            !combined.contains(forbidden),
            "Autocomplete non-wasm component paths should avoid browser-only token `{forbidden}`.",
        );
    }
}

#[test]
fn headless_web_ssr_mutex_contract_is_guarded_by_compile_error_and_failure_probe() {
    for needle in [
        "[platform] compile guard: ui-headless web+ssr must fail",
        "if cargo check -p ui-headless --no-default-features --features web,ssr >\"$MUTEX_LOG\" 2>&1; then",
        "expected ui-headless web+ssr to fail, but command succeeded",
        "if ! rg -n \"mutually exclusive\" \"$MUTEX_LOG\" >/dev/null; then",
        "ui-headless web+ssr failed for an unexpected reason",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            PLATFORM_SCRIPT_SOURCE.contains(needle),
            "platform mutex guard pipeline should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            HEADLESS_LIB_SOURCE.contains(needle),
            "ui-headless should keep explicit feature mutex guard `{needle}`."
        );
    }
}

#[test]
fn motion_non_wasm_stub_contract_is_predictable_and_toolchain_safe() {
    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            MOTION_LIB_SOURCE.contains(needle),
            "ui-motion should keep non-wasm stub marker `{needle}`."
        );
    }

    let non_wasm_stub_test_source =
        include_str!("../../../crates/ui-motion/tests/non_wasm_stub.rs");
    for needle in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
    ] {
        assert!(
            non_wasm_stub_test_source.contains(needle),
            "ui-motion non-wasm stub regression should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_popover_motion(",
        "if !is_open.get() {",
        "on_exit_complete.run(())",
    ] {
        assert!(
            MOTION_SOURCE.contains(needle),
            "Autocomplete motion should keep predictable non-wasm fallback marker `{needle}`."
        );
    }

    for forbidden in ["panic!(", "unreachable!(", "todo!(", "unimplemented!("] {
        assert!(
            !MOTION_SOURCE.contains(forbidden),
            "Autocomplete motion fallback should avoid crash-only placeholder `{forbidden}`."
        );
    }

    for needle in [
        "echo \"[platform] compile-only: ui-motion native path\"",
        "cargo check -p ui-motion",
        "echo \"[platform] compile-only: ui-motion wasm path\"",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "echo \"[platform] ui-motion non-wasm stub tests\"",
        "cargo test -p ui-motion --test non_wasm_stub",
        "echo \"[platform] source guard: autocomplete motion must keep explicit wasm/non-wasm branches\"",
        "echo \"[platform] source guard: autocomplete non-wasm motion fallback must remain predictable\"",
    ] {
        assert!(
            PLATFORM_SCRIPT_SOURCE.contains(needle),
            "platform pipeline should keep ui-motion/toolchain fallback marker `{needle}`."
        );
    }
}

#[test]
fn reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
        "on_rest();",
    ] {
        assert!(
            include_str!("../../../crates/ui-motion/src/spring.rs").contains(needle),
            "ui-motion spring reduced-motion contract should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if !is_open.get() {",
        "on_exit_complete.run(())",
    ] {
        assert!(
            MOTION_SOURCE.contains(needle),
            "Autocomplete motion should keep wasm/non-wasm semantic branch marker `{needle}`."
        );
    }

    for needle in [
        "let presence = use_presence(is_open);",
        "<Show when=move || presence.is_present.get()>",
        "on_exit_complete=presence.finish_exit",
        "let open_now = is_open.get_untracked();",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle) || MOTION_SOURCE.contains(needle),
            "Autocomplete should keep SSR/hydration-safe presence sequencing marker `{needle}`."
        );
    }

    for needle in [
        "echo \"[platform] autocomplete reduced-motion/ssr/wasm contract\"",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
    ] {
        assert!(
            PLATFORM_SCRIPT_SOURCE.contains(needle),
            "platform pipeline should include autocomplete reduced-motion/ssr/wasm contract marker `{needle}`."
        );
    }
}

#[test]
fn motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop() {
    for needle in [
        "pub struct PopoverMotion",
        "pub spring: ui_motion::spring::SpringConfig,",
        "stiffness: 300.0,",
        "damping: 25.0,",
        "mass: 1.0,",
        "pub fn sanitize_popover_motion(motion: PopoverMotion) -> PopoverMotion",
        "pub fn sanitize_motion(motion: AutocompleteMotion) -> AutocompleteMotion",
        "pub fn attach_popover_motion(",
    ] {
        assert!(
            MOTION_SOURCE.contains(needle),
            "Autocomplete motion contract should include `{needle}`."
        );
    }

    for needle in [
        "let motion = crate::motion::sanitize_motion(motion);",
        "crate::motion::attach_popover_motion(",
        "popover_motion=motion.popover",
        "motion=motion.highlight",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete view should mount motion contract via `{needle}`."
        );
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "on_rest();",
    ] {
        assert!(
            include_str!("../../../crates/ui-motion/src/spring.rs").contains(needle),
            "ui-motion spring should keep reduced-motion contract marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if !is_open.get() {",
        "on_exit_complete.run(())",
    ] {
        assert!(
            MOTION_SOURCE.contains(needle),
            "Autocomplete motion should keep non-wasm no-op branch marker `{needle}`."
        );
    }

    for forbidden in ["panic!(", "todo!(", "unimplemented!("] {
        assert!(
            !MOTION_SOURCE.contains(forbidden),
            "Autocomplete motion fallback should avoid crash-only token `{forbidden}`."
        );
    }
}

#[test]
fn motion_contract_check_script_covers_contractualization_guard() {
    let needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop";
    assert!(
        CONTRACT_HYGIENE_SCRIPT_SOURCE.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn check2_marks_motion_contractualization_complete() {
    assert!(
        include_str!("../check2.md").contains("- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。"),
        "autocomplete check2 should mark motion-contract gate complete."
    );

    for needle in [
        "motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop",
        "autocomplete_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop",
        "scripts/check-ui-contract-hygiene.sh",
        "components/autocomplete/src/motion.rs",
        "crates/ui-motion/src/spring.rs",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "autocomplete check2 motion-contract section should reference `{needle}`."
        );
    }
}

#[test]
fn ui_components_fixed_entry_files_are_correctly_located_and_scoped() {
    for needle in [
        "mod css;",
        "#[cfg(feature = \"component-autocomplete\")]",
        "pub use ui_autocomplete as autocomplete;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "pub use autocomplete::{Autocomplete, AutocompleteMotion};",
    ] {
        assert!(
            UI_COMPONENTS_LIB_SOURCE.contains(needle),
            "ui lib entry should keep `{needle}`."
        );
    }

    for forbidden in [
        "pub use leptos::web_sys",
        "pub type HtmlElement",
        "pub type NodeRef",
    ] {
        assert!(
            !UI_COMPONENTS_LIB_SOURCE.contains(forbidden),
            "ui lib entry should not expose platform detail `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-autocomplete\")]",
        "out.push_str(crate::autocomplete::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            UI_COMPONENTS_CSS_SOURCE.contains(needle),
            "ui css entry should keep `{needle}`."
        );
    }

    for needle in [
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
            UI_COMPONENTS_ROOT_SOURCE.contains(needle),
            "UiRoot entry should centralize css/theme/i18n contract `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "pub spring: ui_motion::spring::SpringConfig,",
        "measure_layout: impl FnMut() -> Option<HighlightLayout> + 'static,",
    ] {
        assert!(
            UI_VISUAL_PRIMITIVE_ACTIVE_HIGHLIGHT_SOURCE.contains(needle),
            "active-highlight visual primitive should keep `{needle}`."
        );
    }

    for forbidden in [
        "Autocomplete",
        "autocomplete_empty_message",
        "use_combo_box(",
        "aria-autocomplete",
    ] {
        assert!(
            !UI_VISUAL_PRIMITIVE_ACTIVE_HIGHLIGHT_SOURCE.contains(forbidden),
            "active-highlight primitive should avoid component business semantic `{forbidden}`."
        );
    }

    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root should be two levels above components/autocomplete");
    for absent in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        let path = workspace_root.join("crates/ui/src").join(absent);
        assert!(
            !path.exists(),
            "ui fixed-entry contract forbids `{}`.",
            path.display()
        );
    }

    for needle in [
        "pub mod controllable_state;",
        "pub mod presence;",
        "pub mod a11y;",
        "pub use controllable_state::{",
        "pub use presence::{Presence, use_presence};",
        "use_controllable_open_state_traced,",
    ] {
        assert!(
            HEADLESS_LIB_SOURCE.contains(needle),
            "ui-headless should host fixed primitive entry `{needle}`."
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_controllable_state<T>(",
    ] {
        assert!(
            HEADLESS_CONTROLLABLE_STATE_SOURCE.contains(needle),
            "headless controllable-state primitive should keep `{needle}`."
        );
    }

    assert!(
        HEADLESS_PRESENCE_SOURCE.contains("pub fn use_presence(is_open: Signal<bool>) -> Presence"),
        "headless presence primitive should provide `use_presence` entry."
    );
}

#[test]
fn ui_components_fixed_entry_check_script_covers_contract() {
    let needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_ui_components_fixed_entry_files_are_correctly_located_and_scoped";
    assert!(
        CONTRACT_HYGIENE_SCRIPT_SOURCE.contains(needle),
        "contract-hygiene script should cover fixed-entry file contract `{needle}`."
    );
}

#[test]
fn check2_marks_ui_components_fixed_entry_files_contract_complete() {
    let check2 = include_str!("../check2.md");
    assert!(
        check2.contains("- [x] `ui` 固定入口文件落点正确。"),
        "autocomplete check2 should mark ui fixed-entry file item complete."
    );

    for needle in [
        "ui_components_fixed_entry_files_are_correctly_located_and_scoped",
        "autocomplete_ui_components_fixed_entry_files_are_correctly_located_and_scoped",
        "ui_components_fixed_entry_check_script_covers_contract",
        "crates/ui/src/lib.rs",
        "crates/ui/src/css.rs",
        "crates/ui/src/root.rs",
        "crates/ui-visual-primitive/src/active_highlight.rs",
        "crates/ui-headless/src/controllable_state.rs",
        "crates/ui-headless/src/presence.rs",
        "crates/ui-headless/src/a11y.rs",
        "crates/ui/src/overlay_open.rs",
        "crates/ui/src/presence.rs",
        "crates/ui/src/a11y.rs",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            check2.contains(needle),
            "autocomplete check2 fixed-entry evidence should reference `{needle}`."
        );
    }
}

#[test]
fn component_directory_standard_files_follow_contract_and_na_spec() {
    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::AutocompleteMotion;",
        "pub use view::Autocomplete;",
    ] {
        assert!(
            MOD_SOURCE.contains(needle),
            "mod.rs should keep minimal stable export `{needle}`."
        );
    }

    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !MOD_SOURCE.contains(forbidden),
            "mod.rs should not over-export internals `{forbidden}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::autocomplete::{",
        "pub fn normalize_root_state(",
        "pub fn resolve_root_data_state(",
    ] {
        assert!(
            LOGIC_SOURCE.contains(needle),
            "logic.rs should keep normalization/derivation marker `{needle}`."
        );
    }
    for forbidden in ["view! {", "NodeRef<", "web_sys::"] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden),
            "logic.rs should not host view/platform detail `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str", "var(--ui-"] {
        assert!(
            STYLES_SOURCE.contains(needle),
            "styles.rs should keep token-first static style marker `{needle}`."
        );
    }
    for forbidden in ["on:click", "on:keydown", "style=\"", "style:top="] {
        assert!(
            !STYLES_SOURCE.contains(forbidden),
            "styles.rs should not include runtime view/event token `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "pub fn Autocomplete(",
        "use_combo_box(ComboBoxOptions {",
        "use_text_field(TextFieldOptions {",
        "let root_state = logic::normalize_root_state(logic::RootStateInput {",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "view.rs should keep structure/headless assembly marker `{needle}`."
        );
    }
    assert!(
        !VIEW_SOURCE.contains("mod render;"),
        "view.rs should not drift into render.rs module indirection."
    );

    for needle in [
        "pub struct AutocompleteMotion",
        "pub fn attach_popover_motion(",
        "pub fn sanitize_motion(motion: AutocompleteMotion) -> AutocompleteMotion",
    ] {
        assert!(
            MOTION_SOURCE.contains(needle),
            "motion.rs should keep motion-contract mapping marker `{needle}`."
        );
    }
    for forbidden in ["role=", "aria-", "use_combo_box(", "view! {"] {
        assert!(
            !MOTION_SOURCE.contains(forbidden),
            "motion.rs should not include semantic/view token `{forbidden}`."
        );
    }

    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root should be two levels above components/autocomplete");
    for missing in ["spec.rs", "render.rs"] {
        let path = workspace_root
            .join("components/autocomplete/src")
            .join(missing);
        assert!(
            !path.exists(),
            "Autocomplete simple component scope should not include `{}`.",
            path.display()
        );
    }
}

#[test]
fn component_directory_check_script_covers_contract() {
    let needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_component_directory_standard_files_follow_contract_and_na_spec";
    assert!(
        COMPONENT_FILES_SCRIPT_SOURCE.contains(needle),
        "component-files script should cover autocomplete directory contract `{needle}`."
    );
}

#[test]
fn check2_marks_component_directory_standard_files_contract_complete() {
    let check2 = include_str!("../check2.md");
    assert!(
        check2.contains("- [x] 组件目录标准文件落点正确。"),
        "autocomplete check2 should mark component-directory item complete."
    );
    for needle in [
        "component_directory_standard_files_follow_contract_and_na_spec",
        "autocomplete_component_directory_standard_files_follow_contract_and_na_spec",
        "component_directory_check_script_covers_contract",
        "components/autocomplete/src/mod.rs",
        "components/autocomplete/src/logic.rs",
        "components/autocomplete/src/styles.rs",
        "components/autocomplete/src/view.rs",
        "components/autocomplete/src/motion.rs",
        "components/autocomplete/src/spec.rs",
        "components/autocomplete/src/render.rs",
        "scripts/check-ui-component-files.sh",
    ] {
        assert!(
            check2.contains(needle),
            "check2 component-directory evidence should reference `{needle}`."
        );
    }
}

#[test]
fn file_placement_discipline_is_strict_for_component_scope() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root should be two levels above components/autocomplete");
    let component_src_dir = workspace_root.join("components/autocomplete/src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        let path = component_src_dir.join(required);
        assert!(
            path.exists(),
            "autocomplete file-placement discipline requires `{}`.",
            path.display()
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        let path = component_src_dir.join(forbidden);
        assert!(
            !path.exists(),
            "autocomplete should not introduce forbidden file `{}`.",
            path.display()
        );
    }

    let protocol = component_src_dir.join("protocol.rs");
    assert!(
        protocol.exists(),
        "autocomplete keeps protocol.rs as schema/projection sidecar."
    );
    for needle in [
        "pub enum AutocompleteComponentSchemaVersion",
        "pub struct AutocompleteComponentSpec",
        "#[serde(default)]",
    ] {
        assert!(
            PROTOCOL_SOURCE.contains(needle),
            "protocol.rs should stay schema-only via `{needle}`."
        );
    }

    let combined =
        format!("{MOD_SOURCE}\n{LOGIC_SOURCE}\n{STYLES_SOURCE}\n{VIEW_SOURCE}\n{MOTION_SOURCE}");
    for needle in [
        "pub use view::Autocomplete;",
        "pub fn normalize_root_state(",
        "pub const CSS: &str",
        "view! {",
        "pub struct AutocompleteMotion",
    ] {
        assert!(
            combined.contains(needle),
            "autocomplete file-placement discipline should keep marker `{needle}`."
        );
    }
}

#[test]
fn file_placement_check_script_covers_contract() {
    let needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_file_placement_discipline_is_strict_for_component_scope";
    assert!(
        COMPONENT_FILES_SCRIPT_SOURCE.contains(needle),
        "component-files script should enforce `{needle}`."
    );
}

#[test]
fn check2_marks_file_placement_discipline_contract_complete() {
    let check2 = include_str!("../check2.md");
    assert!(
        check2.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
        "autocomplete check2 should mark file-placement-discipline item complete."
    );

    for needle in [
        "components/autocomplete/src/mod.rs",
        "components/autocomplete/src/logic.rs",
        "components/autocomplete/src/styles.rs",
        "components/autocomplete/src/view.rs",
        "components/autocomplete/src/motion.rs",
        "components/autocomplete/src/protocol.rs",
        "render.rs",
        "spec.rs",
        "components/autocomplete/test/semantics.rs::file_placement_discipline_is_strict_for_component_scope",
        "components/autocomplete/test/semantics.rs::file_placement_check_script_covers_contract",
        "components/autocomplete/test/autocomplete_semantics.rs::autocomplete_file_placement_discipline_is_strict_for_component_scope",
        "components/autocomplete/test/autocomplete_semantics.rs::autocomplete_file_placement_check_script_covers_contract",
        "components/autocomplete/test/autocomplete_semantics.rs::autocomplete_check2_marks_file_placement_discipline_contract_complete",
        "scripts/check-ui-component-files.sh",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_file_placement_discipline_is_strict_for_component_scope",
    ] {
        assert!(
            check2.contains(needle),
            "autocomplete check2 file-placement section should reference `{needle}`."
        );
    }
}

#[test]
fn hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root should be two levels above components/autocomplete");
    let spec_path = workspace_root.join("components/autocomplete/src/spec.rs");

    assert!(
        !spec_path.exists(),
        "autocomplete should keep spec.rs absent for non-complex component scope: `{}`",
        spec_path.display()
    );

    for source in [
        MOD_SOURCE,
        LOGIC_SOURCE,
        VIEW_SOURCE,
        MOTION_SOURCE,
        PROTOCOL_SOURCE,
        README_SOURCE,
    ] {
        for forbidden in ["AutocompleteSpec", "spec::", "Spec::new()", ".render()"] {
            assert!(
                !source.contains(forbidden),
                "autocomplete simple component scope should not expose builder token `{forbidden}`."
            );
        }
    }

    for needle in [
        "pub struct ButtonSpec",
        "impl ButtonSpec",
        "pub fn new() -> Self",
        "pub fn render(self) -> impl IntoView",
    ] {
        assert!(
            BUTTON_SPEC_SOURCE.contains(needle),
            "complex-component spec baseline should remain in button spec via `{needle}`."
        );
    }
}

#[test]
fn hyper_structure_builder_check_script_covers_contract() {
    let needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        COMPONENT_FILES_SCRIPT_SOURCE.contains(needle),
        "component-files script should enforce `{needle}`."
    );
}

#[test]
fn check2_marks_hyper_structure_builder_item_complete() {
    let check2 = include_str!("../check2.md");
    assert!(
        check2.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
        "autocomplete check2 should mark hyper-structure-builder item complete."
    );

    for needle in [
        "components/autocomplete/src/spec.rs",
        "components/button/src/spec.rs",
        "components/autocomplete/test/semantics.rs::{hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "hyper_structure_builder_check_script_covers_contract",
        "check2_marks_hyper_structure_builder_item_complete",
        "components/autocomplete/test/autocomplete_semantics.rs::{autocomplete_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "autocomplete_hyper_structure_builder_check_script_covers_contract",
        "autocomplete_check2_marks_hyper_structure_builder_item_complete",
        "scripts/check-ui-component-files.sh",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
    ] {
        assert!(
            check2.contains(needle),
            "autocomplete check2 hyper-structure-builder section should reference `{needle}`."
        );
    }
}

#[test]
fn context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root should be two levels above components/autocomplete");
    let src_dir = workspace_root.join("components/autocomplete/src");

    assert!(
        src_dir.join("Component.toml").exists(),
        "autocomplete context-compression contract requires `src/Component.toml`."
    );
    assert!(
        src_dir.join("autocomplete.rbi").exists(),
        "autocomplete context-compression contract requires `src/autocomplete.rbi`."
    );

    for needle in [
        "schema_version = \"1\"",
        "name = \"Autocomplete\"",
        "crate = \"ui-autocomplete\"",
        "rbi = \"autocomplete.rbi\"",
        "name = \"id_base\"",
        "name = \"label\"",
        "name = \"items\"",
        "name = \"selected_index\"",
        "name = \"default_selected_index\"",
        "name = \"on_selected_index_change\"",
        "name = \"set_selected_index\"",
        "name = \"is_open\"",
        "name = \"open\"",
        "name = \"default_open\"",
        "name = \"on_open_change\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            COMPONENT_MANIFEST_SOURCE.contains(needle),
            "autocomplete Component.toml should include marker `{needle}`."
        );
    }

    for needle in [
        "pub use crate::motion::AutocompleteMotion;",
        "pub fn sanitize_motion(",
        "pub fn sanitize_popover_motion(",
        "pub fn Autocomplete(",
        "id_base: String",
        "label: String",
        "items: Vec<String>",
        "selected_index: Option<leptos::prelude::Signal<Option<usize>>>",
        "default_selected_index: Option<usize>",
        "on_selected_index_change: Option<leptos::prelude::Callback<Option<usize>>>",
        "set_selected_index: Option<leptos::prelude::WriteSignal<Option<usize>>>",
        "is_open: Option<leptos::prelude::Signal<bool>>",
        "open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
        "motion: crate::motion::AutocompleteMotion",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            COMPONENT_RBI_SOURCE.contains(needle),
            "autocomplete RBI projection should keep marker `{needle}`."
        );
    }

    for needle in [
        "pub fn Autocomplete(",
        "id_base: String,",
        "label: String,",
        "items: Vec<String>,",
        "selected_index: Option<Signal<Option<usize>>>",
        "default_selected_index: Option<usize>",
        "on_selected_index_change: Option<Callback<Option<usize>>>",
        "set_selected_index: Option<WriteSignal<Option<usize>>>",
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] motion: AutocompleteMotion",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "autocomplete view signature should include `{needle}` for manifest/rbi drift detection."
        );
    }
}

#[test]
fn component_files_check_script_covers_context_compression_manifest_contract() {
    let needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        COMPONENT_FILES_SCRIPT_SOURCE.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn check2_marks_context_compression_manifest_and_rbi_contract_complete() {
    let source = include_str!("../check2.md");

    assert!(
        source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "autocomplete check2 should mark context-compression manifest/rbi gate complete."
    );

    for needle in [
        "components/autocomplete/src/Component.toml",
        "components/autocomplete/src/autocomplete.rbi",
        "context_compression_manifest_and_rbi_projection_are_present_and_current",
        "autocomplete_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "component_files_check_script_covers_context_compression_manifest_contract",
        "scripts/check-ui-component-files.sh",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            source.contains(needle),
            "autocomplete check2 context-compression section should reference `{needle}`."
        );
    }
}

#[test]
fn performance_governance_budget_is_defined_traceable_and_blocking() {
    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"autocomplete\" => UiPerfBudget {",
        "max_mount_ms: 38.0,",
        "max_update_ms: Some(13.0),",
        "max_heap_kb: Some(768.0),",
    ] {
        assert!(
            DOCS_COMPONENT_SHELL_SOURCE.contains(needle),
            "docs shell should keep autocomplete performance budget token `{needle}`."
        );
    }

    for needle in [
        "component_doc!(",
        "\"Autocomplete\"",
        "\"autocomplete\"",
        "collections::autocomplete",
    ] {
        assert!(
            DOCS_COMPONENT_PAGES_SOURCE.contains(needle),
            "Autocomplete docs page should remain in component traversal via `{needle}`."
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
            DOCS_PERF_PROBE_SOURCE.contains(needle),
            "UiPerfProbe should expose performance observability marker `{needle}`."
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            E2E_COMPONENTS_COVERAGE_SOURCE.contains(needle),
            "docs coverage e2e should keep perf regression guard `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            DOCS_DEBUG_OVERLAY_SOURCE.contains(needle),
            "debug overlay should keep trace-based perf attribution marker `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            DOCS_PLAN_TODO_SOURCE.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || {",
        "data-label-source=state.label_source_attr",
        "data-placeholder-source=state.placeholder_source_attr",
        "data-id-source=state.id_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-selected-source=selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete view should expose perf triage attribution marker `{needle}`."
        );
    }

    for needle in [
        "echo \"[perf] contract: autocomplete performance governance\"",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_performance_governance_budget_is_defined_and_blocking",
    ] {
        assert!(
            PERFORMANCE_SCRIPT_SOURCE.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
        "performance_governance_budget_is_defined_traceable_and_blocking",
        "autocomplete_performance_governance_budget_is_defined_and_blocking",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "Autocomplete check2 should keep performance-governance marker `{needle}`.",
        );
    }
}

#[test]
fn semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement() {
    for needle in [
        "role=aria.input.role",
        "aria-controls=move || aria.input.aria_controls.get()",
        "aria-expanded=move || aria.input.aria_expanded.get()",
        "aria-selected=move || option_attrs.get().aria_selected",
        "aria-disabled=move || option_attrs.get().aria_disabled",
        "data-state=move ||",
        "data-selected-source=selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
        "data-focused=move || focus_ring.is_focused.get().then_some(\"true\")",
        "data-focus-visible=move || focus_ring.is_focus_visible.get().then_some(\"true\")",
        "on:focus=on_focus",
        "on:blur=on_blur",
        "let key_result = aria.handlers.on_input_key_down.run(key);",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete view should keep semantics+focus marker `{needle}`."
        );
    }

    let perf_gate_needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_performance_governance_budget_is_defined_and_blocking";
    assert!(
        PERFORMANCE_SCRIPT_SOURCE.contains(perf_gate_needle),
        "performance gate script should include `{perf_gate_needle}`."
    );

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            DOCS_PLAN_TODO_SOURCE.contains(needle),
            "render_count follow-up tracking should include `{needle}`."
        );
    }

    for needle in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "semantic_contract_matrix_covers_state_a11y_input_paths_and_platform_guards",
        "performance_governance_budget_is_defined_traceable_and_blocking",
        "semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "autocomplete_semantic_contract_matrix_covers_core_branches_and_platform_paths",
        "autocomplete_performance_governance_budget_is_defined_and_blocking",
        "autocomplete_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "`render_count` 状态：当前测试框架对该组件仍采用可重复 mount/perf trace 等价证据",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "Autocomplete check2 should keep semantics+performance marker `{needle}`."
        );
    }
}

#[test]
fn semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks() {
    for needle in [
        "role=aria.input.role",
        "aria-controls=move || aria.input.aria_controls.get()",
        "data-state=move ||",
        "data-label-source=state.label_source_attr",
        "data-description-source=state.description_source_attr",
        "data-error-source=state.error_source_attr",
        "data-placeholder-source=state.placeholder_source_attr",
        "data-id-source=state.id_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete semantic-priority contract should keep `{needle}`."
        );
    }

    for needle in [
        "fn semantic_contract_matrix_covers_state_a11y_input_paths_and_platform_guards()",
        "fn performance_governance_budget_is_defined_traceable_and_blocking()",
        "fn semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            include_str!("../test/semantics.rs").contains(needle),
            "local semantics suite should keep contract-focused assertion `{needle}`."
        );
    }

    for needle in [
        "fn autocomplete_semantics_suite_is_contract_first_not_snapshot_only()",
        "fn autocomplete_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
        "fn autocomplete_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()",
    ] {
        assert!(
            UI_COMPONENTS_AUTOCOMPLETE_SEMANTICS_SOURCE.contains(needle),
            "workspace semantics suite should keep contract-focused assertion `{needle}`."
        );
    }

    let local_has_snapshot_assertion = include_str!("../test/semantics.rs").lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("assert_snapshot!(") || trimmed.starts_with("insta::assert")
    });
    assert!(
        !local_has_snapshot_assertion,
        "semantic-priority contract should avoid snapshot-only assertion calls in local semantics suite."
    );
    let workspace_has_snapshot_assertion =
        UI_COMPONENTS_AUTOCOMPLETE_SEMANTICS_SOURCE
            .lines()
            .any(|line| {
                let trimmed = line.trim_start();
                trimmed.starts_with("assert_snapshot!(") || trimmed.starts_with("insta::assert")
            });
    assert!(
        !workspace_has_snapshot_assertion,
        "semantic-priority contract should avoid snapshot-only assertion calls in workspace semantics suite."
    );

    let script_needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        PERFORMANCE_SCRIPT_SOURCE.contains(script_needle),
        "performance gate script should include semantic-priority command `{script_needle}`."
    );

    for needle in [
        "语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "components/autocomplete/test/semantics.rs",
        "semantic_contract_matrix_covers_state_a11y_input_paths_and_platform_guards",
        "semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "autocomplete_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "scripts/check-ui-performance.sh",
    ] {
        assert!(
            include_str!("../check2.md").contains(needle),
            "Autocomplete check2 should keep semantic-test-priority evidence marker `{needle}`."
        );
    }
}

#[test]
fn focus_stack_gc_contract_is_na_for_non_modal_combobox_and_component_does_not_restore_focus_itself()
 {
    for needle in [
        "on:focus=on_focus",
        "on:blur=on_blur",
        "aria.handlers.open.run(())",
        "aria.handlers.close.run(())",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete should keep input-centric focus lifecycle marker `{needle}`."
        );
    }

    for forbidden in [
        "use_focus_trap(",
        "FocusTrap",
        "RestorePolicy",
        "FallbackTo(",
        "Selector(",
        "document.body",
        "body.focus(",
        "previous_focus",
    ] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden)
                && !VIEW_SOURCE.contains(forbidden)
                && !MOTION_SOURCE.contains(forbidden),
            "Autocomplete component layer should not implement overlay focus-restore internals `{forbidden}`.",
        );
    }

    for needle in [
        "pub enum RestorePolicy",
        "FallbackTo(String)",
        "Selector(String)",
        "FOCUS_MANAGER_STACK",
        "fn restore_focus_chain(",
    ] {
        assert!(
            include_str!("../../../crates/ui-headless/src/focus_trap.rs").contains(needle),
            "ui-headless focus manager should provide shared overlay focus-restore primitive `{needle}`."
        );
    }
}

#[test]
fn hydration_discontinuity_contract_uses_seeded_id_provider_and_avoids_entropy_sources() {
    for needle in [
        "use_text_field, use_ui_i18n, use_ui_id_provider,",
        "let generated_id_base = use_ui_id_provider()",
        "next_prefixed_id(ui_state_primitives::autocomplete::DEFAULT_ID_BASE)",
        "let id_base = logic::resolve_id_base(id_base, generated_id_base);",
        "has_custom_id_base,",
    ] {
        assert!(
            VIEW_SOURCE.contains(needle),
            "Autocomplete view should keep hydration-stable id bootstrap marker `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_id_base(id_base: String, generated_id_base: String) -> String",
        "normalize_optional_text(Some(id_base)).unwrap_or(generated_id_base)",
        "pub has_custom_id_base: bool,",
        "has_custom_id_base: input.has_custom_id_base,",
    ] {
        assert!(
            LOGIC_SOURCE.contains(needle),
            "Autocomplete logic should keep deterministic id normalization marker `{needle}`."
        );
    }

    let combined = format!("{}\n{}\n{}", MOD_SOURCE, LOGIC_SOURCE, VIEW_SOURCE);
    for forbidden in [
        "now()",
        "Instant::now",
        "SystemTime::now",
        "Date::now",
        "Uuid",
        "uuid::",
        "new_v4",
        "rand::",
        "thread_rng",
        "random::<",
        "getrandom",
        "nanoid",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Autocomplete should avoid hydration-unstable entropy token `{forbidden}`."
        );
    }

    assert!(
        UI_COMPONENTS_ROOT_SOURCE.contains("provide_ui_id_provider(id_seed);"),
        "UiRoot should provide deterministic id seed context."
    );

    for needle in [
        "pub struct UiIdProvider",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider>",
        "pub fn next_prefixed_id(self, prefix: &str) -> String",
    ] {
        assert!(
            HEADLESS_ID_PROVIDER_SOURCE.contains(needle),
            "ui-headless id provider should keep deterministic seed marker `{needle}`."
        );
    }
}

#[test]
fn escape_hatch_foreign_zone_contract_is_na_and_third_party_imperative_instances_are_absent() {
    let combined = format!(
        "{}\n{}\n{}\n{}\n{}",
        MOD_SOURCE, LOGIC_SOURCE, VIEW_SOURCE, MOTION_SOURCE, README_SOURCE
    );

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "OpenLayers",
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Autocomplete should not introduce third-party imperative integration token `{forbidden}`.",
        );
    }

    for forbidden in [
        "pub struct EChart",
        "pub struct Mapbox",
        "pub enum ForeignZone",
        "pub type ForeignHandle",
        "pub third_party_instance:",
        "pub map_instance:",
        "pub chart_instance:",
    ] {
        assert!(
            !MOD_SOURCE.contains(forbidden)
                && !LOGIC_SOURCE.contains(forbidden)
                && !VIEW_SOURCE.contains(forbidden)
                && !MOTION_SOURCE.contains(forbidden),
            "Autocomplete public/component API should not expose third-party instance handle `{forbidden}`.",
        );
    }
}

#[test]
fn check2_documents_agent_contract_schema_governance_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "agent_contract_is_schema_typed_and_machine_readable",
        "agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should keep autocomplete Agent Contract marker `{required}`."
        );
    }
}

#[test]
fn agent_contract_is_schema_typed_and_machine_readable() {
    for typed_source in [
        "pub const AUTOCOMPLETE_AGENT_SCHEMA: &str = \"ui.autocomplete.agent-contract\";",
        "pub enum AutocompleteAgentSchemaVersion",
        "pub enum AutocompleteAgentIntent",
        "pub enum AutocompleteAgentAction",
        "pub enum AutocompleteAgentState",
        "pub enum AutocompleteAgentSource",
        "pub struct AutocompleteAgentContract",
        "pub struct AutocompleteAgentContractInput",
        "pub fn resolve_agent_contract(input: AutocompleteAgentContractInput) -> AutocompleteAgentContract",
    ] {
        assert!(
            LOGIC_SOURCE.contains(typed_source),
            "Autocomplete Agent Contract should remain type-derived via `{typed_source}`."
        );
    }

    for marker in [
        "let agent_contract = Signal::derive(move || {",
        "logic::resolve_agent_contract(logic::AutocompleteAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-ui-state-source=move || agent_contract.get().state_source",
        "data-ui-motion-source=move || agent_contract.get().motion_source",
        "data-ui-selected-source=move || agent_contract.get().selected_source",
        "data-ui-selected-change-source=move || agent_contract.get().selected_change_source",
        "data-ui-open-value-source=move || agent_contract.get().open_value_source",
        "data-ui-config-policy=move || agent_contract.get().config_policy",
    ] {
        assert!(
            VIEW_SOURCE.contains(marker),
            "Autocomplete view should mount Agent Contract marker `{marker}`."
        );
    }

    for required in [
        "name = \"agent_contract_schema_markers\"",
        "name = \"agent_contract_whitelist_boundary\"",
        "[[agent_contract]]",
        "schema = \"ui.autocomplete.agent-contract.v1\"",
        "intent = \"autocomplete.suggest-and-select\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "AUTOCOMPLETE_AGENT_SCHEMA",
        "AutocompleteAgentContract",
        "resolve_agent_contract(",
    ] {
        assert!(
            COMPONENT_MANIFEST_SOURCE.contains(required) || COMPONENT_RBI_SOURCE.contains(required),
            "Autocomplete context-compression assets should keep Agent Contract marker `{required}`."
        );
    }
}

#[test]
fn agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    for marker in [
        "Self::V1 => \"v1\"",
        "Self::SuggestAndSelect => \"autocomplete.suggest-and-select\"",
        "Self::Idle => \"idle\"",
        "Self::Query => \"query\"",
        "Self::CommitSelection => \"commit-selection\"",
        "Self::Open => \"open\"",
        "Self::Disabled => \"disabled\"",
        "Self::StatePrimitives => \"state-primitives\"",
    ] {
        assert!(
            LOGIC_SOURCE.contains(marker),
            "Autocomplete Agent Contract should keep closed typed mapping marker `{marker}`."
        );
    }

    for forbidden in [
        "data-ui-schema=\"",
        "data-ui-intent=\"",
        "data-ui-action=\"",
        "data-ui-state=\"",
        "data-ui-source=\"",
        "format!(\"data-ui-",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "Autocomplete view should not splice free-form Agent Contract marker `{forbidden}`."
        );
    }
}

#[test]
fn agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "\"typed_state_from_ui_state_primitives::autocomplete::resolve_state\"",
        "\"typed_semantics_from_logic::resolve_agent_contract\"",
        "\"typed_render_mount_from_view\"",
        "\"<script\"",
        "\"javascript:\"",
        "\"eval(\"",
        "name = \"agent_contract_whitelist_boundary\"",
    ] {
        assert!(
            COMPONENT_MANIFEST_SOURCE.contains(required),
            "Autocomplete Component.toml should keep whitelist boundary marker `{required}`."
        );
    }

    for forbidden in [
        "inner_html=",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "eval(",
    ] {
        assert!(
            !VIEW_SOURCE.contains(forbidden),
            "Autocomplete render path should remain injection-safe without `{forbidden}`."
        );
    }
}

#[test]
fn contract_hygiene_script_covers_agent_contract_schema_guards() {
    for needle in [
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            CONTRACT_HYGIENE_SCRIPT_SOURCE.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn streaming_term_is_limited_to_llm_output_render_modes() {
    let check2_source = include_str!("../check2.md");

    for marker in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "streaming_term_is_limited_to_llm_output_render_modes",
        "autocomplete_streaming_term_is_limited_to_llm_output_render_modes",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "check2 should keep streaming-term governance marker `{marker}`."
        );
    }

    for marker in [
        "pub enum AutocompleteAgentStreamMode",
        "Self::Streaming => \"streaming\"",
        "Self::Snapshot => \"snapshot\"",
        "stream_support: AutocompleteAgentStreamSupport::Unsupported,",
        "stream_fallback: AutocompleteAgentStreamFallback::Snapshot,",
        "stream_mode: AutocompleteAgentStreamMode::Snapshot,",
    ] {
        assert!(
            LOGIC_SOURCE.contains(marker),
            "logic.rs should keep LLM render mode marker `{marker}`."
        );
    }

    for marker in [
        "[streaming_policy]",
        "term_scope = \"llm-output-rendering\"",
        "defined_modes = [\"streaming\", \"snapshot\"]",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
    ] {
        assert!(
            COMPONENT_MANIFEST_SOURCE.contains(marker),
            "Component.toml should keep streaming-term scope marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
    ] {
        assert!(
            VIEW_SOURCE.contains(marker),
            "view.rs should expose machine-readable stream marker `{marker}`."
        );
    }
}

#[test]
fn contract_hygiene_script_covers_streaming_term_guard() {
    let needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_streaming_term_is_limited_to_llm_output_render_modes";
    assert!(
        CONTRACT_HYGIENE_SCRIPT_SOURCE.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn snapshot_is_foundational_and_complete_config_renders_stably() {
    let check2_source = include_str!("../check2.md");

    for marker in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "snapshot_is_foundational_and_complete_config_renders_stably",
        "autocomplete_snapshot_is_foundational_and_complete_config_renders_stably",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "check2 should keep snapshot-foundation marker `{marker}`."
        );
    }

    for marker in [
        "pub fn Autocomplete(",
        "id_base: String,",
        "label: String,",
        "items: Vec<String>,",
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool,",
        "#[prop(optional)] motion: AutocompleteMotion,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
    ] {
        assert!(
            VIEW_SOURCE.contains(marker),
            "view.rs should keep complete-config snapshot render marker `{marker}`."
        );
    }

    for marker in [
        "Self::Verified => \"verified\"",
        "output_status: AutocompleteAgentOutputStatus::Verified,",
        "Self::Snapshot => \"snapshot\"",
        "stream_mode: AutocompleteAgentStreamMode::Snapshot,",
        "stream_fallback: AutocompleteAgentStreamFallback::Snapshot,",
    ] {
        assert!(
            LOGIC_SOURCE.contains(marker),
            "logic.rs should keep snapshot/output-status marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
    ] {
        assert!(
            VIEW_SOURCE.contains(marker),
            "view.rs should expose snapshot output marker `{marker}`."
        );
    }

    for marker in [
        "[streaming_policy]",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
        "attr = \"data-ui-stream-fallback\"",
        "attr = \"data-ui-stream-mode\"",
        "values = [\"snapshot\"]",
        "attr = \"data-ui-output-status\"",
        "values = [\"verified\"]",
    ] {
        assert!(
            COMPONENT_MANIFEST_SOURCE.contains(marker),
            "Component.toml should keep snapshot-foundation marker `{marker}`."
        );
    }
}

#[test]
fn contract_hygiene_script_covers_snapshot_foundation_guard() {
    let needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_snapshot_is_foundational_and_complete_config_renders_stably";
    assert!(
        CONTRACT_HYGIENE_SCRIPT_SOURCE.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status() {
    let check2_source = include_str!("../check2.md");

    for marker in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status",
        "autocomplete_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "check2 should keep streaming-requirement marker `{marker}`."
        );
    }

    for marker in [
        "## Streaming 策略",
        "`Snapshot`：默认路径，组件稳定消费完整配置并渲染。",
        "`Streaming Optional`：`Autocomplete` 不是 LLM 正文阅读面；若上层为流式容器，本组件按 `fallback=snapshot` 方式消费稳定配置。",
    ] {
        assert!(
            README_SOURCE.contains(marker),
            "README should keep streaming optional policy marker `{marker}`."
        );
    }

    for marker in [
        "required = false",
        "owner = \"upstream\"",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
    ] {
        assert!(
            COMPONENT_MANIFEST_SOURCE.contains(marker),
            "Component.toml should keep streaming optional policy marker `{marker}`."
        );
    }

    for marker in [
        "output_status: AutocompleteAgentOutputStatus::Verified,",
        "stream_support: AutocompleteAgentStreamSupport::Unsupported,",
        "stream_fallback: AutocompleteAgentStreamFallback::Snapshot,",
        "stream_mode: AutocompleteAgentStreamMode::Snapshot,",
    ] {
        assert!(
            LOGIC_SOURCE.contains(marker),
            "logic.rs should keep explicit output status marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
    ] {
        assert!(
            VIEW_SOURCE.contains(marker),
            "view.rs should keep explicit output marker `{marker}`."
        );
    }
}

#[test]
fn contract_hygiene_script_covers_streaming_requirement_guard() {
    let needle = "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status";
    assert!(
        CONTRACT_HYGIENE_SCRIPT_SOURCE.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn rust_hygiene_forbids_unwrap_expect_and_let_result_swallowing_in_non_test_sources() {
    for (name, source) in [
        ("mod.rs", MOD_SOURCE),
        ("logic.rs", LOGIC_SOURCE),
        ("view.rs", VIEW_SOURCE),
        ("styles.rs", STYLES_SOURCE),
        ("motion.rs", MOTION_SOURCE),
        ("protocol.rs", PROTOCOL_SOURCE),
    ] {
        assert!(
            !source.contains("unwrap("),
            "{name} should not contain `unwrap(` in non-test code."
        );
        assert!(
            !source.contains("expect("),
            "{name} should not contain `expect(` in non-test code."
        );
        assert!(
            !source.contains("let _ ="),
            "{name} should not swallow side-effect results via `let _ = ...`."
        );
    }
}

#[test]
fn string_clone_hotspots_converge_to_cow_static_str_for_class_tokens() {
    for marker in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(\"ui-autocomplete\")];",
        "classes.push(Cow::Borrowed(\"ui-autocomplete--disabled\"));",
        "classes.push(Cow::Borrowed(\"ui-autocomplete--custom-class\"));",
        "classes.push(Cow::Owned(base_class_name));",
        "let mut composed = first.into_owned();",
        "composed.push_str(class.as_ref());",
    ] {
        assert!(
            LOGIC_SOURCE.contains(marker),
            "logic.rs should keep Cow-based class token composition marker `{marker}`."
        );
    }

    for forbidden in [
        "vec![\"ui-autocomplete\".to_string()]",
        "classes.push(\"ui-autocomplete--disabled\".to_string())",
        "classes.push(\"ui-autocomplete--custom-class\".to_string())",
    ] {
        assert!(
            !LOGIC_SOURCE.contains(forbidden),
            "logic.rs should remove String clone hotspot `{forbidden}`."
        );
    }
}

#[test]
fn check2_marks_rust_hygiene_item_complete_with_component_scope() {
    let check2_source = include_str!("../check2.md");
    for marker in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "rust_hygiene_forbids_unwrap_expect_and_let_result_swallowing_in_non_test_sources",
        "string_clone_hotspots_converge_to_cow_static_str_for_class_tokens",
        "autocomplete_rust_hygiene_forbids_unwrap_expect_and_let_result_swallowing_in_non_test_sources",
        "autocomplete_string_clone_hotspots_converge_to_cow_static_str_for_class_tokens",
        "scripts/check-ui-contract-hygiene.sh",
        "`./scripts/check-rust-hygiene.sh`",
    ] {
        assert!(
            check2_source.contains(marker),
            "check2 should keep rust-hygiene marker `{marker}`."
        );
    }
}

#[test]
fn contract_hygiene_script_covers_rust_hygiene_guards() {
    for needle in [
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_rust_hygiene_forbids_unwrap_expect_and_let_result_swallowing_in_non_test_sources",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_string_clone_hotspots_converge_to_cow_static_str_for_class_tokens",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_marks_rust_hygiene_item_complete_with_component_scope",
    ] {
        assert!(
            CONTRACT_HYGIENE_SCRIPT_SOURCE.contains(needle),
            "contract-hygiene script should include rust-hygiene guard `{needle}`."
        );
    }
}

#[test]
fn check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2 = include_str!("../check2.md");

    for marker in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "已统一使用语义选择器与稳定等待",
        "并移除文本定位依赖（不再使用 `hasText` 过滤）",
        "关键路径显式断言 ready/settled 断点",
    ] {
        assert!(
            check2.contains(marker),
            "Autocomplete check2 should keep e2e selector stability rule `{marker}`."
        );
    }
}

#[test]
fn e2e_selector_contract_uses_semantic_markers_and_wasm_stable_waits() {
    for needle in [
        "page.goto(\"/#/components/autocomplete\")",
        "body:not(:has(#boot))",
        "#docs-autocomplete-controlled-input",
        "locator('xpath=ancestor::*[@data-slot=\"autocomplete\"][1]')",
        "[data-slot=\"autocomplete-controlled-open\"]",
        "[data-slot=\"autocomplete-controlled-selected\"]",
        "[data-slot=\"autocomplete-option\"]",
        "toHaveAttribute(\"data-controlled\", \"true\")",
    ] {
        assert!(
            E2E_AUTOCOMPLETE_CONTRACT_SOURCE.contains(needle),
            "Autocomplete e2e selector/stable-wait contract should include `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
        "hasText:",
    ] {
        assert!(
            !E2E_AUTOCOMPLETE_CONTRACT_SOURCE.contains(forbidden),
            "Autocomplete e2e selector contract should avoid flaky/text locator token `{forbidden}`."
        );
    }
}

#[test]
fn e2e_contract_covers_ready_and_settled_semantic_breakpoints() {
    for needle in [
        "toHaveText(\"open: false\")",
        "toHaveText(\"open: true\")",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"data-closed\", \"true\")",
        "toHaveText(\"selected: 3\")",
        "await page.reload();",
        "toHaveText(\"selected: 2\")",
    ] {
        assert!(
            E2E_AUTOCOMPLETE_CONTRACT_SOURCE.contains(needle),
            "Autocomplete e2e ready/settled contract should include `{needle}`."
        );
    }
}

#[test]
fn e2e_check_script_covers_selector_and_stable_wait_contract() {
    for needle in [
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_e2e_selectors_are_semantic_and_wasm_wait_strategy_is_stable",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_e2e_contract_covers_ready_and_settled_semantic_breakpoints",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_e2e_key_flow_is_repeatable_with_semantic_breakpoints",
    ] {
        assert!(
            AUTOCOMPLETE_E2E_SCRIPT_SOURCE.contains(needle),
            "Autocomplete e2e check script should include `{needle}`."
        );
    }
}

#[test]
fn check2_marks_e2e_selector_stability_item_complete() {
    let check2 = include_str!("../check2.md");

    assert!(
        check2.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "Autocomplete check2 should mark e2e selector stability item complete."
    );

    for needle in [
        "e2e/tests/docs_app_autocomplete_contract.spec.mjs",
        "components/autocomplete/scripts/check-ui-e2e-autocomplete.sh",
        "components/autocomplete/test/semantics.rs::{check2_documents_e2e_selector_and_stable_wait_rules",
        "e2e_selector_contract_uses_semantic_markers_and_wasm_stable_waits",
        "e2e_contract_covers_ready_and_settled_semantic_breakpoints",
        "e2e_check_script_covers_selector_and_stable_wait_contract",
        "components/autocomplete/test/autocomplete_semantics.rs::{autocomplete_check2_documents_e2e_selector_and_stable_wait_rules",
        "autocomplete_e2e_selectors_are_semantic_and_wasm_wait_strategy_is_stable",
        "autocomplete_e2e_contract_covers_ready_and_settled_semantic_breakpoints",
        "autocomplete_e2e_key_flow_is_repeatable_with_semantic_breakpoints",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(needle),
            "Autocomplete check2 should include e2e selector stability evidence marker `{needle}`."
        );
    }
}

#[test]
fn check2_documents_e2e_repeatable_key_flow_rules() {
    let check2 = include_str!("../check2.md");

    for marker in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "已纳入两条可重复关键流程并可回放",
        "失败可定位到具体语义断点而非笼统页面差异",
        "overlay/focus/keyboard 已进入回归集合",
    ] {
        assert!(
            check2.contains(marker),
            "Autocomplete check2 should keep repeatable-flow governance rule `{marker}`."
        );
    }
}

#[test]
fn e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    for needle in [
        "docs-app autocomplete key flow is repeatable with semantic contract breakpoints",
        "await controlledInput.fill(\"Shen\")",
        "await option.click();",
        "await expect(selectedMarker).toHaveText(\"selected: 3\")",
        "await page.reload();",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
    ] {
        assert!(
            E2E_AUTOCOMPLETE_CONTRACT_SOURCE.contains(needle),
            "Autocomplete repeatable key-flow contract should include `{needle}`."
        );
    }
}

#[test]
fn e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    for needle in [
        "docs-app autocomplete high-risk overlay/focus/keyboard path is replayable with semantic breakpoints",
        "await controlledInput.focus();",
        "await expect(controlledInput).toBeFocused();",
        "await controlledInput.press(\"Escape\");",
        "await controlledInput.press(\"ArrowDown\");",
        "await controlledInput.press(\"Enter\");",
        "const activeDescendant = await controlledInput.getAttribute(\"aria-activedescendant\");",
        "expect(activeDescendant).toBeTruthy();",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"data-closed\", \"true\")",
        "await page.reload();",
    ] {
        assert!(
            E2E_AUTOCOMPLETE_CONTRACT_SOURCE.contains(needle),
            "Autocomplete high-risk e2e flow should include `{needle}`."
        );
    }
}

#[test]
fn e2e_check_script_covers_repeatable_flow_and_high_risk_contract() {
    for needle in [
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_e2e_key_flow_is_repeatable_with_semantic_breakpoints",
        "cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            AUTOCOMPLETE_E2E_SCRIPT_SOURCE.contains(needle),
            "Autocomplete e2e script should include repeatable-flow/high-risk contract `{needle}`."
        );
    }
}

#[test]
fn check2_marks_e2e_repeatable_regression_item_complete() {
    let check2 = include_str!("../check2.md");

    assert!(
        check2.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "Autocomplete check2 should mark repeatable e2e regression item complete."
    );

    for needle in [
        "e2e/tests/docs_app_autocomplete_contract.spec.mjs",
        "components/autocomplete/scripts/check-ui-e2e-autocomplete.sh",
        "components/autocomplete/test/semantics.rs::{check2_documents_e2e_repeatable_key_flow_rules",
        "e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "e2e_check_script_covers_repeatable_flow_and_high_risk_contract",
        "components/autocomplete/test/autocomplete_semantics.rs::{autocomplete_check2_documents_e2e_repeatable_key_flow_rules",
        "autocomplete_e2e_key_flow_is_repeatable_with_semantic_breakpoints",
        "autocomplete_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "autocomplete_e2e_check_script_covers_repeatable_flow_and_high_risk_contract",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(needle),
            "Autocomplete check2 should include repeatable e2e evidence marker `{needle}`."
        );
    }
}
