use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn dropdown_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/dropdown/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Dropdown internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn dropdown_uses_logic_state_model() {
    let view_source = load_source("src/dropdown/view.rs");
    let logic_source = load_source("src/dropdown/logic.rs");

    for needle in [
        "pub enum DropdownOpenFocusStrategy",
        "pub fn focus_strategy_for_open_key(",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_disabled_indices(",
        "pub fn resolve_trigger_disabled(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Dropdown logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_id_base(id_base)",
        "logic::normalize_disabled_indices(disabled_indices, item_count)",
        "let open_state = overlay_open::use_controllable_open_state(open, default_open, on_open_change);",
        "logic::resolve_state(crate::dropdown::DropdownStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Dropdown view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn dropdown_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/dropdown/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            source.contains(needle),
            "Dropdown should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn dropdown_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/dropdown/view.rs");

    for attr in [
        "data-slot=\"dropdown\"",
        "data-state=move ||",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-close-on-action=state.close_on_action.then_some(\"true\")",
        "data-keep-open-on-action=state.keep_open_on_action.then_some(\"true\")",
        "data-controlled=state.is_controlled.then_some(\"true\")",
        "data-uncontrolled=state.is_uncontrolled.then_some(\"true\")",
        "data-custom-label=state.has_custom_aria_label.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-has-disabled-items=state.has_disabled_items.then_some(\"true\")",
        "data-has-item-kinds=state.has_item_kinds.then_some(\"true\")",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Dropdown should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn dropdown_composes_button_popover_and_menu() {
    let source = load_source("src/dropdown/view.rs");

    for needle in [
        "<Button",
        "aria_haspopup=\"menu\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "<Popover",
        "<Menu",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "Dropdown should compose overlay/menu flow through `{needle}`."
        );
    }
}

#[test]
fn dropdown_styles_include_persistent_and_disabled_markers() {
    let source = load_source("src/dropdown/styles.rs");

    for selector in [
        ".ui-dropdown--disabled",
        ".ui-dropdown[data-disabled=\"true\"]",
        ".ui-dropdown--persistent",
        ".ui-dropdown[data-keep-open-on-action=\"true\"]",
        ".ui-dropdown--custom-class",
        ".ui-dropdown[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Dropdown styles should include `{selector}` as stable visual-state contracts."
        );
    }
}
