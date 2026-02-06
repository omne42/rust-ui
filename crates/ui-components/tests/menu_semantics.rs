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
        "data-empty=is_empty.then_some(\"true\")",
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
