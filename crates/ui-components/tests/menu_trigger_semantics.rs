use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn menu_trigger_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/menu_trigger/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "MenuTrigger's internal modules should stay private; found `{needle}`."
        );
    }
}

#[test]
fn menu_trigger_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/menu_trigger/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            source.contains(needle),
            "MenuTrigger should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn menu_trigger_is_labeled_and_owns_a_menu() {
    let source = load_source("src/menu_trigger/view.rs");

    for needle in [
        "aria_label: Option<String>",
        "aria_haspopup=\"menu\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "aria_labelledby=trigger_id.get_value()",
    ] {
        assert!(
            source.contains(needle),
            "MenuTrigger should wire `{needle}` for Spectrum-style menu trigger semantics."
        );
    }
}

#[test]
fn menu_trigger_uses_presence_to_allow_exit_motion() {
    let source = load_source("src/menu_trigger/view.rs");

    for needle in [
        "use_presence(open)",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "MenuTrigger should use presence for motion-friendly unmounting via `{needle}`."
        );
    }
}

#[test]
fn menu_trigger_emits_spectrum_style_root_data_attributes() {
    let source = load_source("src/menu_trigger/view.rs");

    for attr in [
        "data-slot=\"menu-trigger\"",
        "data-open=move || open.get().then_some(\"true\")",
        "data-disabled=trigger_disabled.get_value().then_some(\"true\")",
        "data-empty=(item_count.get_value() == 0).then_some(\"true\")",
        "data-has-items=(item_count.get_value() > 0).then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "MenuTrigger should set `{attr}` to support Spectrum-style styling and regression testing."
        );
    }
}

#[test]
fn menu_trigger_supports_arrow_key_opening() {
    let source = load_source("src/menu_trigger/view.rs");

    for needle in [
        "focus_strategy_for_open_key",
        "request_open_change.run(true)",
        "set_open_focus.set(strategy)",
    ] {
        assert!(
            source.contains(needle),
            "MenuTrigger should support ArrowUp/ArrowDown opening via `{needle}`."
        );
    }
}

#[test]
fn menu_trigger_uses_logic_for_empty_and_disabled_trigger_state() {
    let logic_source = load_source("src/menu_trigger/logic.rs");
    let view_source = load_source("src/menu_trigger/view.rs");

    assert!(
        logic_source.contains("resolve_trigger_disabled"),
        "MenuTrigger logic should centralize trigger-disabled semantics for disabled + empty states."
    );

    for needle in [
        "let trigger_disabled = StoredValue::new(logic::resolve_trigger_disabled(",
        "if trigger_disabled.get_value()",
        "disabled=trigger_disabled.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "MenuTrigger view should consume `{needle}` to keep trigger behavior/state attrs consistent."
        );
    }
}
