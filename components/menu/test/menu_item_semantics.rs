use std::fs;
use std::path::Path;

fn workspace_dir() -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"))
        .to_path_buf()
}

fn load_source(rel_path: &str) -> String {
    if let Some(component_rel_path) = rel_path.strip_prefix("src/menu/") {
        let path = workspace_dir()
            .join("components/menu/src")
            .join(component_rel_path);
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn menu_item_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/menu/item/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "MenuItem internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn menu_item_uses_logic_state_model() {
    let logic_source = load_source("src/menu/item/logic.rs");
    let view_source = load_source("src/menu/item/view.rs");

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
fn menu_item_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/menu/item/view.rs");

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
            "MenuItem should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn menu_item_styles_include_kind_and_state_markers() {
    let source = load_source("src/menu/item/styles.rs");

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
    let source = load_source("src/menu/item/view.rs");

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

#[test]
fn menu_item_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "pub(super) fn menu_item() -> AnyView",
        "title=\"MenuItem\"",
        "slug=\"menu-item\"",
        "description=\"baseline-style menu row primitive with centralized kind/checked/focus/source normalization and stable `slot` + `data-*` contracts.\"",
        "<Playground title=\"Action + Checkbox\" code_signal=code>",
        "<Playground title=\"Radio + Submenu + Disabled\" code_signal=states_code>",
        "<MenuItem",
        "kind=MenuItemKind::Action",
        "kind=checkbox_kind",
        "kind=radio_kind",
        "has_submenu=true",
        "disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "collections-extra docs page should include `{needle}` for menu-item coverage.",
        );
    }
}

#[test]
fn menu_item_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "let (checkbox_checked, set_checkbox_checked) = signal(true);",
        "let (radio_selected, set_radio_selected) = signal(true);",
        "set_checkbox_checked.update(|value| *value = !*value);",
        "set_radio_selected.update(|value| *value = !*value);",
        "aria_label=\"Open profile\".to_string()",
        "id=\"docs-menu-item-radio\".to_string()",
        "class_name=\"docs-menu-item-custom\".to_string()",
        "\"Open profile\"",
        "\"Pin to favorites\"",
        "\"Set as primary workspace\"",
        "\"Disabled destructive action\"",
        "\"checkbox checked: \"",
        "\"radio selected: \"",
    ] {
        assert!(
            source.contains(needle),
            "menu-item docs playgrounds should contain `{needle}`.",
        );
    }
}
