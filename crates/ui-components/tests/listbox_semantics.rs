use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn listbox_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/listbox/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ListBox internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn listbox_supports_accessible_name_resolution() {
    let view_source = load_source("src/listbox/view.rs");
    let logic_source = load_source("src/listbox/logic.rs");

    for needle in [
        "aria_label: Option<String>",
        "aria_labelledby: Option<String>",
        "resolve_accessible_name",
        "aria-label=aria_label.get_value()",
        "aria-labelledby=aria_labelledby.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "ListBox should wire `{needle}` for Spectrum-style accessible naming."
        );
    }

    assert!(
        logic_source.contains("aria_label: Some(\"Listbox\".to_string())"),
        "ListBox logic should provide a default accessible label when none is supplied."
    );
}

#[test]
fn listbox_exposes_state_and_slot_data_attributes() {
    let source = load_source("src/listbox/view.rs");

    for needle in [
        "data-slot=\"listbox\"",
        "data-disabled=disabled.then_some(\"true\")",
        "data-empty=is_empty.then_some(\"true\")",
        "data-slot=\"listbox-options\"",
        "data-slot=\"listbox-highlight\"",
        "data-slot=\"listbox-option\"",
    ] {
        assert!(
            source.contains(needle),
            "ListBox should expose `{needle}` for Spectrum-style styling and regression tests."
        );
    }
}

#[test]
fn listbox_options_expose_selection_focus_and_disabled_states() {
    let source = load_source("src/listbox/view.rs");

    for needle in [
        "data-selected=move ||",
        "data-focused=move ||",
        "aria.active_index.get() == index",
        "data-disabled=if is_disabled { Some(\"true\") } else { None }",
    ] {
        assert!(
            source.contains(needle),
            "ListBox options should expose `{needle}` for selection/focus/disabled state."
        );
    }
}

#[test]
fn listbox_attaches_focus_ring_and_active_highlight_motion() {
    let source = load_source("src/listbox/view.rs");

    for needle in [
        "use_focus_ring",
        "class:ui-listbox--focus-visible",
        "attach_active_highlight_motion(",
        "node_ref=options_ref",
        "node_ref=highlight_ref",
    ] {
        assert!(
            source.contains(needle),
            "ListBox should keep `{needle}` for focus-visible and HeroUI-like active highlight motion."
        );
    }
}
