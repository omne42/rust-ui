use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn menu_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/menu/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Menu internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn menu_supports_accessible_name_resolution() {
    let view_source = load_source("src/menu/view.rs");
    let logic_source = load_source("src/menu/logic.rs");

    for needle in [
        "aria_label: Option<String>",
        "aria_labelledby: Option<String>",
        "resolve_accessible_name",
        "aria-label=aria_label.get_value()",
        "aria-labelledby=aria_labelledby.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "Menu should wire `{needle}` for Spectrum-style accessible naming."
        );
    }

    assert!(
        logic_source.contains("aria_label: Some(\"Menu\".to_string())"),
        "Menu logic should provide a safe default accessible label when no label props are supplied."
    );
}

#[test]
fn menu_exposes_state_and_slot_data_attributes() {
    let source = load_source("src/menu/view.rs");

    for needle in [
        "data-slot=\"menu\"",
        "data-disabled=disabled.then_some(\"true\")",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-has-checked-items=move || state.get().has_checked_items.then_some(\"true\")",
        "data-checked-empty=move || (!state.get().has_checked_items).then_some(\"true\")",
        "data-has-disabled-items=move || state.get().has_disabled_items.then_some(\"true\")",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
        "data-slot=\"menu-items\"",
        "data-slot=\"menu-highlight\"",
        "data-slot=\"menu-item\"",
    ] {
        assert!(
            source.contains(needle),
            "Menu should expose `{needle}` for Spectrum-style styling and regression tests."
        );
    }
}

#[test]
fn menu_items_expose_kind_checked_and_focus_state() {
    let source = load_source("src/menu/view.rs");

    for needle in [
        "data-index=index",
        "data-kind=item.attrs.role",
        "data-checked=move ||",
        "aria_checked",
        "data-focused=move ||",
        "aria.active_index.get() == index",
    ] {
        assert!(
            source.contains(needle),
            "Menu items should wire `{needle}` to expose kind/checked/focus state."
        );
    }
}

#[test]
fn menu_uses_logic_state_model() {
    let view_source = load_source("src/menu/view.rs");
    let logic_source = load_source("src/menu/logic.rs");

    for needle in [
        "pub struct MenuState",
        "pub fn resolve_state(",
        "pub has_checked_items: bool",
        "pub has_disabled_items: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "Menu logic should include `{needle}` for centralized root-state derivation."
        );
    }

    assert!(
        view_source.contains("logic::resolve_state("),
        "Menu view should derive root state through resolve_state."
    );
    assert!(
        view_source.contains("has_disabled || disabled"),
        "Menu view should include component-disabled state when deriving has_disabled_items."
    );
}

#[test]
fn menu_attaches_active_highlight_motion_driver() {
    let source = load_source("src/menu/view.rs");

    for needle in [
        "attach_active_highlight_motion(",
        "node_ref=items_ref",
        "node_ref=highlight_ref",
    ] {
        assert!(
            source.contains(needle),
            "Menu should keep active-highlight motion wiring via `{needle}`."
        );
    }
}

#[test]
fn menu_styles_include_motion_marker_contracts() {
    let source = load_source("src/menu/styles.rs");

    for selector in [
        ".ui-menu[data-motion-source=\"custom\"]",
        ".ui-menu[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Menu styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}
