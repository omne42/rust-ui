use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn menu_item_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/menu_item/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "MenuItem internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn menu_item_uses_logic_state_model() {
    let logic_source = load_source("src/menu_item/logic.rs");
    let view_source = load_source("src/menu_item/view.rs");

    for needle in [
        "pub enum MenuItemSelectionIndicator",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_selection_indicator(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn resolve_checked(",
        "pub fn resolve_aria_checked(",
        "data_state_attr",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "MenuItem logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(MenuItemStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "MenuItem view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn menu_item_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/menu_item/view.rs");

    for attr in [
        "data-slot=\"menu-item\"",
        "data-slot=\"menu-item-indicator\"",
        "data-slot=\"menu-item-label\"",
        "data-slot=\"menu-item-submenu-indicator\"",
        "data-index=index_text",
        "data-kind=move || state.get().kind_attr",
        "data-state=move || state.get().data_state_attr",
        "data-checkable=move || state.get().is_checkable.then_some(\"true\")",
        "data-checked=move || state.get().is_checked.then_some(\"true\")",
        "data-unchecked=move || (!state.get().is_checked && state.get().is_checkable).then_some(\"true\")",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-submenu=move || state.get().has_submenu.then_some(\"true\")",
        "data-selection-indicator=selection_indicator.as_attr()",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "MenuItem should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn menu_item_styles_include_kind_and_state_markers() {
    let source = load_source("src/menu_item/styles.rs");

    for selector in [
        ".ui-menu-item--kind-action",
        ".ui-menu-item[data-kind=\"checkbox\"]",
        ".ui-menu-item--checkable",
        ".ui-menu-item[data-checkable=\"true\"]",
        ".ui-menu-item--checked",
        ".ui-menu-item[data-checked=\"true\"]",
        ".ui-menu-item--focused",
        ".ui-menu-item[data-focused=\"true\"]",
        ".ui-menu-item--disabled",
        ".ui-menu-item[data-disabled=\"true\"]",
        ".ui-menu-item--submenu",
        ".ui-menu-item[data-has-submenu=\"true\"]",
        ".ui-menu-item--custom-class",
        ".ui-menu-item[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "MenuItem styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn menu_item_supports_kind_based_aria_roles_and_checked_states() {
    let source = load_source("src/menu_item/view.rs");

    for needle in [
        "role=move || state.get().role_attr",
        "aria-checked=move || logic::resolve_aria_checked(kind)",
        "aria-disabled=disabled.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "MenuItem should include `{needle}` for ARIA role and checked-state semantics."
        );
    }
}
