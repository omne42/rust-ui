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

    for needle in [
        "pub struct AutocompleteStateInput",
        "pub struct AutocompleteState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_disabled_indices(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Autocomplete logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let id_base = logic::normalize_id_base(id_base);",
        "let disabled_indices = logic::normalize_disabled_indices(disabled_indices, item_count);",
        "let state = logic::resolve_state(logic::AutocompleteStateInput {",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn autocomplete_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn autocomplete_normalizes_label_placeholder_and_id_base() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");

    for needle in [
        "logic::normalize_label",
        "logic::resolve_placeholder",
        "logic::normalize_id_base",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should use `{needle}` to keep text and id semantics stable."
        );
    }

    for needle in ["\"Options\".to_string()", "\"autocomplete\".to_string()"] {
        assert!(
            logic_source.contains(needle),
            "Autocomplete logic should provide fallback semantics via `{needle}`."
        );
    }
}

#[test]
fn autocomplete_escape_stops_propagation_when_open() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in ["stop_propagation()", "key == \"Escape\"", "was_open"] {
        assert!(
            source.contains(needle),
            "Autocomplete should handle Escape bubbling with `{needle}` to avoid closing parent overlays unexpectedly."
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
            "Autocomplete panel should include `{needle}` for Spectrum-style popover behavior."
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
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete panel should expose `{needle}` for Spectrum-style state styling and deterministic tests."
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
fn autocomplete_emits_spectrum_style_state_data_attributes() {
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
        "data-motion-source=if motion == AutocompleteMotion::default()",
        "data-custom-motion=(motion != AutocompleteMotion::default()).then_some(\"true\")",
        "data-typed=move || has_typed.get().then_some(\"true\")",
        "data-count=state.item_count.to_string()",
        "data-filtered-count=move || filtered_count.get().to_string()",
        "data-disabled-option-count=state.disabled_option_count.to_string()",
    ] {
        assert!(
            source.contains(attr),
            "Autocomplete should set `{attr}` to support Spectrum-style styling and state inspection."
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
        ".ui-autocomplete--has-disabled-options",
        ".ui-autocomplete--empty",
        ".ui-autocomplete[data-motion-source=\"custom\"]",
        ".ui-autocomplete[data-custom-motion=\"true\"]",
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
            "Autocomplete motion contract should include `{needle}` for HeroUI-style spring customization."
        );
    }
}
