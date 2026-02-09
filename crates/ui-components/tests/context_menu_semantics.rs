use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn context_menu_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/context_menu/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::ContextMenu;"),
        "context_menu module should export `ContextMenu`."
    );
    assert!(
        module_source.contains("ContextMenuMotion"),
        "context_menu module should expose a motion alias."
    );
    assert!(
        crate_source.contains("pub use context_menu::{ContextMenu, ContextMenuMotion};"),
        "crate root should re-export context_menu contracts."
    );
}

#[test]
fn context_menu_uses_logic_state_model() {
    let view_source = load_source("src/context_menu/view.rs");
    let logic_source = load_source("src/context_menu/logic.rs");

    for needle in [
        "pub struct ContextMenuStateInput",
        "pub struct ContextMenuState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_disabled_indices(",
        "pub fn resolve_trigger_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn focus_strategy_for_open_key(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ContextMenu logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let id_base = logic::normalize_id_base(id_base);",
        "let disabled_indices = logic::normalize_disabled_indices(disabled_indices, item_count);",
        "let (aria_label, has_custom_aria_label) = logic::resolve_trigger_aria_label(aria_label);",
        "let state = logic::resolve_state(logic::ContextMenuStateInput {",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "ContextMenu view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn context_menu_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/context_menu/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            source.contains(needle),
            "ContextMenu should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn context_menu_trigger_wires_context_and_keyboard_open_contract() {
    let source = load_source("src/context_menu/view.rs");

    for needle in [
        "on:contextmenu=on_context_menu",
        "on:keydown=on_key_down",
        "if let Some(strategy) = logic::focus_strategy_for_open_key(&key, ev.shift_key())",
        "aria-haspopup=\"menu\"",
        "aria-expanded=move || if open.get() { \"true\" } else { \"false\" }",
        "aria-controls=aria_controls",
        "aria_labelledby=trigger_id.get_value()",
    ] {
        assert!(
            source.contains(needle),
            "ContextMenu should wire `{needle}` to match context-trigger + keyboard-open semantics."
        );
    }
}

#[test]
fn context_menu_emits_spectrum_root_state_data_attributes() {
    let source = load_source("src/context_menu/view.rs");

    for needle in [
        "data-slot=\"context-menu\"",
        "data-slot=\"context-menu-trigger\"",
        "data-state=move ||",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-disabled=state.is_trigger_disabled.then_some(\"true\")",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-placement=state.placement_attr",
        "data-controlled=state.is_controlled.then_some(\"true\")",
        "data-uncontrolled=state.is_uncontrolled.then_some(\"true\")",
        "data-close-on-action=state.close_on_action.then_some(\"true\")",
        "data-keep-open-on-action=state.keep_open_on_action.then_some(\"true\")",
        "data-custom-label=state.has_custom_aria_label.then_some(\"true\")",
        "data-has-disabled-items=state.has_disabled_items.then_some(\"true\")",
        "data-has-item-kinds=state.has_item_kinds.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ContextMenu should set `{needle}` so it can be styled/tested with Spectrum-compatible selectors."
        );
    }
}

#[test]
fn context_menu_renders_menu_inside_popover_with_presence() {
    let source = load_source("src/context_menu/view.rs");

    for needle in [
        "use_presence(open)",
        "<Popover",
        "placement=state.placement",
        "motion=motion.popover",
        "<Menu",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "ContextMenu should compose popover/presence/menu via `{needle}` for motion-safe unmounting."
        );
    }
}

#[test]
fn context_menu_styles_include_disabled_and_persistent_markers() {
    let source = load_source("src/context_menu/styles.rs");

    for needle in [
        ".ui-context-menu {",
        ".ui-context-menu__trigger {",
        ".ui-context-menu--persistent",
        ".ui-context-menu--disabled",
        ".ui-context-menu--empty",
    ] {
        assert!(
            source.contains(needle),
            "ContextMenu styles should include `{needle}` for stable visual state contracts."
        );
    }
}
