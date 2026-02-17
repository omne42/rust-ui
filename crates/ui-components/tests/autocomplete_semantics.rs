use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn autocomplete_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/autocomplete/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Autocomplete internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn autocomplete_uses_logic_state_model() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/autocomplete.rs");

    for needle in [
        "pub use ui_state_primitives::autocomplete::{",
        "AutocompleteStateInput",
        "AutocompleteState",
        "RootDataState",
        "resolve_root_data_state",
        "normalize_optional_text",
        "normalize_id_base",
        "normalize_disabled_indices",
        "resolve_state",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Autocomplete logic should include `{needle}` while consuming centralized ui-state-primitives."
        );
    }

    for needle in [
        "pub struct AutocompleteStateInput",
        "pub struct AutocompleteState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_disabled_indices(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Autocomplete primitive source should define `{needle}`."
        );
    }

    for needle in [
        "let root_state = logic::normalize_root_state(logic::RootStateInput {",
        "logic::resolve_root_data_state(is_open.get(), state.is_disabled).as_attr()",
        "let state = root_state.state;",
        "let class = root_state.class_name;",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn autocomplete_logic_does_not_reimplement_reusable_state_primitives() {
    let logic_source = load_source("src/autocomplete/logic.rs");

    for forbidden in [
        "pub fn normalize_disabled_indices(",
        "pub fn filter_indices(",
        "pub fn map_selected_to_filtered(",
        "pub fn map_filtered_to_original(",
        "pub fn resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Autocomplete logic should not reimplement reusable primitive `{forbidden}`; it must consume ui-state-primitives instead.",
        );
    }
}

#[test]
fn autocomplete_component_has_store_adapter_boundary() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");

    for forbidden in ["GlobalState", "AppState", "Store<", "SignalStore", "apps::"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Autocomplete should not bind app/business store type `{forbidden}` directly.",
        );
    }
}

#[test]
fn autocomplete_discrete_data_state_is_enum_backed() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");

    for needle in [
        "pub enum RootDataState",
        "Open,",
        "Disabled,",
        "Closed,",
        "pub fn resolve_root_data_state(",
        "pub const fn as_attr(self) -> &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "Autocomplete logic should model discrete data-state with `{needle}`."
        );
    }

    assert!(
        view_source
            .contains("logic::resolve_root_data_state(is_open.get(), state.is_disabled).as_attr()"),
        "Autocomplete view should map data-state via typed enum contract instead of inline string branching."
    );
}

#[test]
fn autocomplete_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");

    for needle in [
        "is_open: Option<Signal<bool>>",
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "pub struct OpenStateInput",
        "pub struct OpenState",
        "pub fn normalize_open_state(",
    ] {
        assert!(
            source.contains(needle) || logic_source.contains(needle),
            "Autocomplete should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn autocomplete_wires_open_value_change_default_triplet_into_headless_state() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in [
        "let normalized_open_state = logic::normalize_open_state(logic::OpenStateInput {",
        "let open_state = overlay_open::use_controllable_open_state_traced(",
        "\"autocomplete\",",
        "open,",
        "default_open,",
        "on_open_change,",
        "let is_open = open_state.open;",
        "let set_open = open_state.request_open_change;",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete open axis should wire `{needle}` for stable controlled/uncontrolled semantics.",
        );
    }
}

#[test]
fn autocomplete_supports_is_prefixed_boolean_props_with_legacy_aliases() {
    let source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");

    for needle in [
        "is_disabled: Option<bool>",
        "disabled: bool",
        "is_required: Option<Signal<bool>>",
        "required: Option<Signal<bool>>",
        "is_invalid: Option<Signal<bool>>",
        "invalid: Option<Signal<bool>>",
        "pub struct AccessibilityStateInput",
        "pub struct AccessibilityState",
        "pub fn normalize_accessibility_state(",
        "is_disabled: input.is_disabled.unwrap_or(input.disabled)",
        "let required = input",
        ".is_required",
        ".or(input.required)",
        "let invalid = input",
        ".is_invalid",
        ".or(input.invalid)",
    ] {
        assert!(
            source.contains(needle) || logic_source.contains(needle),
            "Autocomplete API naming contract should include `{needle}`."
        );
    }

    for needle in [
        "let accessibility_state =",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
        "let is_disabled = accessibility_state.is_disabled;",
        "let required = accessibility_state.required;",
        "let invalid = accessibility_state.invalid;",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete view should consume normalized accessibility state via `{needle}`."
        );
    }
}

#[test]
fn autocomplete_view_does_not_inline_default_fallback_rules() {
    let source = load_source("src/autocomplete/view.rs");

    for forbidden in [
        "is_disabled.unwrap_or(disabled)",
        "is_required.or(required)",
        "is_invalid.or(invalid)",
        "is_open.or(open)",
        "unwrap_or_else(|| Signal::derive(|| false))",
        "logic::normalize_id_base(",
        "logic::normalize_label(",
        "logic::resolve_placeholder(",
        "logic::resolve_state(",
    ] {
        assert!(
            !source.contains(forbidden),
            "Autocomplete view.rs should not own fallback/priority rule `{forbidden}`; keep it in logic.rs.",
        );
    }
}

#[test]
fn autocomplete_normalizes_label_placeholder_and_id_base() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/autocomplete.rs");

    for needle in [
        "normalize_label(",
        "resolve_placeholder(",
        "resolve_empty_message(",
        "normalize_id_base(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Autocomplete logic should use `{needle}` to keep text and id semantics stable."
        );
    }

    assert!(
        view_source.contains("logic::normalize_root_state(logic::RootStateInput {"),
        "Autocomplete view should delegate normalization to logic::normalize_root_state."
    );

    for needle in [
        "pub const DEFAULT_LABEL: &str = \"Options\"",
        "pub const DEFAULT_ID_BASE: &str = \"autocomplete\"",
        "pub const DEFAULT_PLACEHOLDER: &str = \"Type…\"",
        "pub const DEFAULT_EMPTY_MESSAGE: &str = \"No matches\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Autocomplete primitives should provide fallback semantics via `{needle}`."
        );
    }
}
#[test]
fn autocomplete_escape_stops_propagation_when_open() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in [
        "let key_result = aria.handlers.on_input_key_down.run(key);",
        "if key_result.handled {",
        "if key_result.stop_propagation {",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete should consume typed headless keydown outcomes via `{needle}` to keep keyboard semantics out of view.rs."
        );
    }
}

#[test]
fn autocomplete_passes_lang_dir_and_headless_aria_controls_contract() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in [
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
        "lang,",
        "dir,",
        "aria-controls=move || aria.input.aria_controls.get()",
        "lang=aria.input.lang.clone()",
        "dir=aria.input.dir",
        "lang=aria.listbox.lang.clone()",
        "dir=aria.listbox.dir",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete should wire `{needle}` so locale + aria-controls semantics come from ui-headless contract."
        );
    }
}

#[test]
fn autocomplete_panel_is_portaled_and_uses_popover_positioning() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in [
        "<Portal>",
        "use_popover_position",
        "data-ui-overlay-portal",
        "--ui-popover-top",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete panel should include `{needle}` for baseline-style popover behavior."
        );
    }
}

#[test]
fn autocomplete_panel_exposes_option_and_empty_state_slots() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in [
        "data-slot=\"autocomplete-listbox\"",
        "data-empty=move || filtered_indices.get().is_empty().then_some(\"true\")",
        "data-slot=\"autocomplete-option\"",
        "data-focused=move || (active_index.get() == filtered_index).then_some(\"true\")",
        "data-slot=\"autocomplete-empty\"",
        "{move || empty_message.get_value()}",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete panel should expose `{needle}` for baseline-style state styling and deterministic tests."
        );
    }
}

#[test]
fn autocomplete_has_a11y_i18n_and_locale_entrypoints_via_headless_contracts() {
    let view_source = load_source("src/autocomplete/view.rs");
    let common_i18n_source = load_source("../ui-headless/src/i18n/common.rs");

    for needle in [
        "use ui_headless::{",
        "CommonStrings",
        "use_ui_i18n",
        "let i18n = use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "empty_message: empty_message",
        ".or_else(|| Some(common.autocomplete_empty_message.to_string()))",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete should include a11y/i18n/locale contract token `{needle}`."
        );
    }

    assert!(
        common_i18n_source.contains("pub autocomplete_empty_message: Arc<str>,"),
        "CommonStrings should expose autocomplete empty-message i18n slot."
    );
    assert!(
        common_i18n_source.contains("autocomplete_empty_message: \"No matches\".into(),"),
        "CommonStrings default should provide autocomplete empty-message fallback."
    );
    assert!(
        !view_source.contains("\"No matches\""),
        "Autocomplete view.rs should not hardcode user-visible empty-state copy."
    );
}

#[test]
fn autocomplete_has_no_async_loading_protocol_and_keeps_sync_input_contract() {
    let view_source = load_source("src/autocomplete/view.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "on_retry",
        "retry",
        "use_async_action",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Autocomplete should not define per-component async protocol token `{forbidden}` when interaction is sync-only."
        );
    }

    for required in [
        "let on_action = Callback::new(move |filtered_index: usize| {",
        "set_selected_index.set(Some(original_index));",
        "set_query.set(label);",
        "set_has_typed.set(false);",
    ] {
        assert!(
            view_source.contains(required),
            "Autocomplete should keep synchronous selection-action flow `{required}`."
        );
    }
}

#[test]
fn autocomplete_uses_presence_for_motion_safe_unmounting() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in [
        "use_presence(is_open)",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete should use `{needle}` so popover exit motion can finish before unmount."
        );
    }
}

#[test]
fn autocomplete_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/autocomplete/view.rs");

    for attr in [
        "data-slot=\"autocomplete\"",
        "data-state=move ||",
        "data-open=move || is_open.get().then_some(\"true\")",
        "data-closed=move || (!is_open.get()).then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-empty=move || (filtered_count.get() == 0).then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-has-filtered-items=move || (filtered_count.get() > 0).then_some(\"true\")",
        "data-selection-empty=move || selected_index.get().is_none().then_some(\"true\")",
        "data-has-selection=move || selected_index.get().is_some().then_some(\"true\")",
        "data-invalid=move || invalid.get().then_some(\"true\")",
        "data-valid=move || (!invalid.get()).then_some(\"true\")",
        "data-required=move || required.get().then_some(\"true\")",
        "data-optional=move || (!required.get()).then_some(\"true\")",
        "data-has-description=state.has_description.then_some(\"true\")",
        "data-has-error=state.has_error.then_some(\"true\")",
        "data-has-disabled-options=state.has_disabled_options.then_some(\"true\")",
        "data-controlled=state.is_controlled.then_some(\"true\")",
        "data-uncontrolled=state.is_uncontrolled.then_some(\"true\")",
        "data-label-source=state.label_source_attr",
        "data-description-source=state.description_source_attr",
        "data-error-source=state.error_source_attr",
        "data-placeholder-source=state.placeholder_source_attr",
        "data-id-source=state.id_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-custom-label=state.has_custom_label.then_some(\"true\")",
        "data-custom-description=state.has_custom_description.then_some(\"true\")",
        "data-custom-error=state.has_custom_error.then_some(\"true\")",
        "data-custom-placeholder=state.has_custom_placeholder.then_some(\"true\")",
        "data-custom-id=state.has_custom_id_base.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
        "data-typed=move || has_typed.get().then_some(\"true\")",
        "data-count=state.item_count.to_string()",
        "data-filtered-count=move || filtered_count.get().to_string()",
        "data-disabled-option-count=state.disabled_option_count.to_string()",
    ] {
        assert!(
            source.contains(attr),
            "Autocomplete should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}
#[test]
fn autocomplete_panel_styles_use_fixed_positioning_and_transform_origin_by_placement() {
    let source = load_source("src/autocomplete/styles.rs");

    for needle in [
        "position: fixed;",
        "var(--ui-popover-top",
        "data-placement=\"bottom-start\"",
        ".ui-autocomplete__empty",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete styles should include `{needle}` for popover layout and empty-state rendering."
        );
    }
}

#[test]
fn autocomplete_styles_include_controlled_and_disabled_option_markers() {
    let source = load_source("src/autocomplete/styles.rs");

    for needle in [
        ".ui-autocomplete--controlled",
        ".ui-autocomplete[data-controlled=\"true\"]",
        ".ui-autocomplete--has-disabled-options",
        ".ui-autocomplete[data-has-disabled-options=\"true\"]",
        ".ui-autocomplete--empty",
        ".ui-autocomplete[data-empty=\"true\"]",
        ".ui-autocomplete[data-label-source=\"custom\"]",
        ".ui-autocomplete[data-custom-label=\"true\"]",
        ".ui-autocomplete--custom-label",
        ".ui-autocomplete[data-description-source=\"custom\"]",
        ".ui-autocomplete[data-custom-description=\"true\"]",
        ".ui-autocomplete--custom-description",
        ".ui-autocomplete[data-error-source=\"custom\"]",
        ".ui-autocomplete[data-custom-error=\"true\"]",
        ".ui-autocomplete--custom-error",
        ".ui-autocomplete[data-placeholder-source=\"custom\"]",
        ".ui-autocomplete[data-custom-placeholder=\"true\"]",
        ".ui-autocomplete--custom-placeholder",
        ".ui-autocomplete[data-id-source=\"custom\"]",
        ".ui-autocomplete[data-custom-id=\"true\"]",
        ".ui-autocomplete--custom-id",
        ".ui-autocomplete[data-class-source=\"custom\"]",
        ".ui-autocomplete[data-custom-class=\"true\"]",
        ".ui-autocomplete--custom-class",
        ".ui-autocomplete[data-motion-source=\"custom\"]",
        ".ui-autocomplete[data-custom-motion=\"true\"]",
        ".ui-autocomplete--custom-motion",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete styles should include `{needle}` for stable state-marker contracts."
        );
    }
}
#[test]
fn autocomplete_motion_contract_exposes_popover_and_highlight_customization() {
    let mod_source = load_source("src/autocomplete/mod.rs");
    let motion_source = load_source("src/autocomplete/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::AutocompleteMotion;",
        "pub struct AutocompleteMotion",
        "pub popover: PopoverMotion",
        "pub highlight: ActiveHighlightMotion",
        "fn default_motion_uses_default_popover_and_highlight_motion()",
        "fn supports_custom_popover_and_highlight_motion_contracts()",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "Autocomplete motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
fn autocomplete_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/autocomplete/motion.rs");
    let view_source = load_source("src/autocomplete/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: AutocompleteMotion) -> AutocompleteMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "highlight: sanitize_highlight(motion.highlight)",
        "fn sanitize_motion_falls_back_for_invalid_nested_values()",
    ] {
        assert!(
            motion_source.contains(needle),
            "Autocomplete motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::autocomplete::motion::sanitize_motion(motion);"),
        "Autocomplete view should sanitize motion before attaching popover and active-highlight motion.",
    );
}

#[test]
fn autocomplete_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "pub(super) fn autocomplete() -> AnyView",
        "title=\"Autocomplete\"",
        "slug=\"autocomplete\"",
        "description=\"Combobox-like autocomplete with baseline-style root attrs, controlled/uncontrolled open state, and baseline-level active highlight motion.\"",
        "<Playground title=\"Selection + Validation\" code_signal=code>",
        "<Playground title=\"Controlled Open State\" code_signal=controlled_code>",
        "<Playground title=\"Disabled + Empty\" code_signal=states_code>",
        "<Autocomplete",
        "is_open=controlled_open",
        "is_disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "collections docs page should include `{needle}` for autocomplete coverage.",
        );
    }
}

#[test]
fn autocomplete_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "id_base=\"docs-autocomplete\".to_string()",
        "label=\"City\".to_string()",
        "disabled_indices=vec![3]",
        "description=\"Search and pick one city\".to_string()",
        "error=\"City is required\".to_string()",
        "placeholder=\"Type…\".to_string()",
        "on_press=Callback::new(move |_| set_invalid.update(|value| *value = !*value))",
        "\"selected: \"",
        "id_base=\"docs-autocomplete-controlled\".to_string()",
        "on_open_change=on_open_change",
        "description=\"Open state is externally controlled\".to_string()",
        "\"open: \"",
        "id_base=\"docs-autocomplete-disabled\".to_string()",
        "id_base=\"docs-autocomplete-empty\".to_string()",
        "placeholder=\"No options\".to_string()",
        "\"disabled selected: \"",
        "\"empty selected: \"",
    ] {
        assert!(
            source.contains(needle),
            "autocomplete docs playgrounds should contain `{needle}`.",
        );
    }
}
